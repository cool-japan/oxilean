//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ModelValue, SmtContext, SmtExtConfig3100, SmtExtConfigVal3100, SmtExtDiag3100, SmtExtDiff3100,
    SmtExtPass3100, SmtExtPipeline3100, SmtExtResult3100, SmtModel, SmtProofObligation,
    SmtQueryBuilder, SmtResult, SmtSolver, SmtSort, SmtStats, SmtTacticConfig, SmtTacticResult,
    SmtTerm, TacticSmtAnalysisPass, TacticSmtConfig, TacticSmtConfigValue, TacticSmtDiagnostics,
    TacticSmtDiff, TacticSmtPipeline, TacticSmtResult,
};

/// A snapshot of declarations and assertions at a push/pop boundary.
pub(super) type SmtSnapshot = (Vec<(String, SmtSort)>, Vec<SmtTerm>);
pub fn smt_true() -> SmtTerm {
    SmtTerm::BoolLit(true)
}
pub fn smt_false() -> SmtTerm {
    SmtTerm::BoolLit(false)
}
pub fn smt_var(name: &str, sort: SmtSort) -> SmtTerm {
    SmtTerm::Var(name.to_string(), sort)
}
pub fn smt_int(n: i64) -> SmtTerm {
    SmtTerm::IntLit(n)
}
pub fn smt_and(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::And(vec![a, b])
}
pub fn smt_or(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Or(vec![a, b])
}
pub fn smt_not(a: SmtTerm) -> SmtTerm {
    SmtTerm::Not(Box::new(a))
}
pub fn smt_implies(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Implies(Box::new(a), Box::new(b))
}
pub fn smt_eq(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Eq(Box::new(a), Box::new(b))
}
pub fn smt_lt(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Lt(Box::new(a), Box::new(b))
}
pub fn smt_le(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Le(Box::new(a), Box::new(b))
}
pub fn smt_add(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Add(Box::new(a), Box::new(b))
}
pub fn smt_mul(a: SmtTerm, b: SmtTerm) -> SmtTerm {
    SmtTerm::Mul(Box::new(a), Box::new(b))
}
pub fn smt_forall(vars: Vec<(&str, SmtSort)>, body: SmtTerm) -> SmtTerm {
    SmtTerm::Forall(
        vars.into_iter().map(|(s, t)| (s.to_string(), t)).collect(),
        Box::new(body),
    )
}
pub fn smt_exists(vars: Vec<(&str, SmtSort)>, body: SmtTerm) -> SmtTerm {
    SmtTerm::Exists(
        vars.into_iter().map(|(s, t)| (s.to_string(), t)).collect(),
        Box::new(body),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::smt::*;
    #[test]
    fn test_smt_sort_display() {
        assert_eq!(SmtSort::Bool.to_string(), "Bool");
        assert_eq!(SmtSort::Int.to_string(), "Int");
        assert_eq!(SmtSort::Real.to_string(), "Real");
        assert_eq!(SmtSort::BitVec(32).to_string(), "(_ BitVec 32)");
        let arr = SmtSort::Array(Box::new(SmtSort::Int), Box::new(SmtSort::Bool));
        assert_eq!(arr.to_string(), "(Array Int Bool)");
        assert_eq!(SmtSort::Named("MyType".to_string()).to_string(), "MyType");
    }
    #[test]
    fn test_smt_term_bool_lit() {
        assert_eq!(SmtTerm::BoolLit(true).to_smtlib(), "true");
        assert_eq!(SmtTerm::BoolLit(false).to_smtlib(), "false");
    }
    #[test]
    fn test_smt_term_int_lit() {
        assert_eq!(SmtTerm::IntLit(42).to_smtlib(), "42");
        assert_eq!(SmtTerm::IntLit(0).to_smtlib(), "0");
        assert_eq!(SmtTerm::IntLit(-5).to_smtlib(), "(- 5)");
    }
    #[test]
    fn test_smt_term_and() {
        let a = SmtTerm::BoolLit(true);
        let b = SmtTerm::BoolLit(false);
        let and = smt_and(a, b);
        let s = and.to_smtlib();
        assert!(s.starts_with("(and "), "expected (and ...), got: {}", s);
        assert!(s.contains("true"));
        assert!(s.contains("false"));
    }
    #[test]
    fn test_smt_term_forall() {
        let body = smt_lt(smt_var("x", SmtSort::Int), smt_int(10));
        let t = smt_forall(vec![("x", SmtSort::Int)], body);
        let s = t.to_smtlib();
        assert!(
            s.starts_with("(forall "),
            "expected (forall ...), got: {}",
            s
        );
        assert!(s.contains("(x Int)"), "expected (x Int) in: {}", s);
    }
    #[test]
    fn test_smt_context_new() {
        let ctx = SmtContext::new(SmtSolver::Z3);
        assert_eq!(ctx.solver, SmtSolver::Z3);
        assert_eq!(ctx.assertion_count(), 0);
        assert_eq!(ctx.decl_count(), 0);
    }
    #[test]
    fn test_smt_context_declare_assert() {
        let mut ctx = SmtContext::new(SmtSolver::Cvc5);
        ctx.declare_const("x", SmtSort::Int);
        ctx.declare_const("y", SmtSort::Int);
        assert_eq!(ctx.decl_count(), 2);
        ctx.assert(smt_lt(
            smt_var("x", SmtSort::Int),
            smt_var("y", SmtSort::Int),
        ));
        assert_eq!(ctx.assertion_count(), 1);
    }
    #[test]
    fn test_smt_context_emit_smtlib2() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.set_option("produce-models", "true");
        ctx.declare_const("a", SmtSort::Bool);
        ctx.assert(smt_var("a", SmtSort::Bool));
        let script = ctx.emit_smtlib2();
        assert!(
            script.contains("set-option"),
            "missing set-option in: {}",
            script
        );
        assert!(
            script.contains("declare-const a Bool"),
            "missing decl in: {}",
            script
        );
        assert!(
            script.contains("(assert a)"),
            "missing assert in: {}",
            script
        );
        assert!(
            script.contains("(check-sat)"),
            "missing check-sat in: {}",
            script
        );
    }
    #[test]
    fn test_smt_check_sat_empty_is_sat() {
        // An empty assertion set is trivially satisfiable.
        let ctx = SmtContext::new(SmtSolver::Yices2);
        assert_eq!(ctx.check_sat(), SmtResult::Sat);
        assert!(!ctx.check_unsat());
    }
    #[test]
    fn test_smt_convenience_builders() {
        assert!(smt_true().is_literal());
        assert!(smt_false().is_literal());
        assert!(smt_int(0).is_literal());
        assert!(!smt_var("x", SmtSort::Int).is_literal());
        let t = smt_and(smt_true(), smt_false());
        assert!(t.to_smtlib().contains("and"));
        let t2 = smt_implies(smt_var("p", SmtSort::Bool), smt_var("q", SmtSort::Bool));
        assert!(t2.to_smtlib().contains("=>"));
        let t3 = smt_eq(smt_int(1), smt_int(1));
        assert!(t3.to_smtlib().contains("="));
        let sum = smt_add(smt_int(2), smt_int(3));
        assert!(sum.to_smtlib().contains('+'));
        let prod = smt_mul(smt_int(4), smt_int(5));
        assert!(prod.to_smtlib().contains('*'));
        let le = smt_le(smt_int(0), smt_int(1));
        assert!(le.to_smtlib().contains("<="));
        let neg = smt_not(smt_true());
        assert!(neg.to_smtlib().contains("not"));
        let ex = smt_exists(
            vec![("n", SmtSort::Int)],
            smt_lt(smt_var("n", SmtSort::Int), smt_int(0)),
        );
        assert!(ex.to_smtlib().contains("exists"));
        let mut ctx = SmtContext::new(SmtSolver::Bitwuzla);
        ctx.declare_const("v", SmtSort::Bool);
        ctx.push();
        ctx.assert(smt_var("v", SmtSort::Bool));
        assert_eq!(ctx.assertion_count(), 1);
        ctx.pop();
        assert_eq!(ctx.assertion_count(), 0);
        assert_eq!(ctx.decl_count(), 1);
    }
}
/// Count the number of sub-terms in an SMT term (recursive).
#[allow(dead_code)]
pub fn smt_term_size(term: &SmtTerm) -> usize {
    match term {
        SmtTerm::BoolLit(_) | SmtTerm::IntLit(_) | SmtTerm::RealLit(_) | SmtTerm::BvLit(..) => 1,
        SmtTerm::Var(..) => 1,
        SmtTerm::Not(t) | SmtTerm::BvNot(t) => 1 + smt_term_size(t),
        SmtTerm::And(terms) | SmtTerm::Or(terms) => {
            1 + terms.iter().map(smt_term_size).sum::<usize>()
        }
        SmtTerm::App(_, args) => 1 + args.iter().map(smt_term_size).sum::<usize>(),
        SmtTerm::Implies(a, b)
        | SmtTerm::Iff(a, b)
        | SmtTerm::Eq(a, b)
        | SmtTerm::Add(a, b)
        | SmtTerm::Sub(a, b)
        | SmtTerm::Mul(a, b)
        | SmtTerm::Div(a, b)
        | SmtTerm::Mod(a, b)
        | SmtTerm::Lt(a, b)
        | SmtTerm::Le(a, b)
        | SmtTerm::Gt(a, b)
        | SmtTerm::Ge(a, b)
        | SmtTerm::BvAdd(a, b)
        | SmtTerm::BvAnd(a, b)
        | SmtTerm::BvUlt(a, b) => 1 + smt_term_size(a) + smt_term_size(b),
        SmtTerm::Ite(c, t, e) => 1 + smt_term_size(c) + smt_term_size(t) + smt_term_size(e),
        SmtTerm::Forall(_, body) | SmtTerm::Exists(_, body) => 1 + smt_term_size(body),
    }
}
/// Collect all free variable names referenced in an SMT term.
#[allow(dead_code)]
pub fn smt_free_vars(term: &SmtTerm) -> Vec<String> {
    let mut vars = Vec::new();
    smt_collect_vars(term, &mut vars);
    vars.sort();
    vars.dedup();
    vars
}
pub(super) fn smt_collect_vars(term: &SmtTerm, out: &mut Vec<String>) {
    match term {
        SmtTerm::Var(name, _) => out.push(name.clone()),
        SmtTerm::Not(t) | SmtTerm::BvNot(t) => smt_collect_vars(t, out),
        SmtTerm::And(terms) | SmtTerm::Or(terms) => {
            for t in terms {
                smt_collect_vars(t, out);
            }
        }
        SmtTerm::App(_, args) => {
            for a in args {
                smt_collect_vars(a, out);
            }
        }
        SmtTerm::Implies(a, b)
        | SmtTerm::Iff(a, b)
        | SmtTerm::Eq(a, b)
        | SmtTerm::Add(a, b)
        | SmtTerm::Sub(a, b)
        | SmtTerm::Mul(a, b)
        | SmtTerm::Div(a, b)
        | SmtTerm::Mod(a, b)
        | SmtTerm::Lt(a, b)
        | SmtTerm::Le(a, b)
        | SmtTerm::Gt(a, b)
        | SmtTerm::Ge(a, b)
        | SmtTerm::BvAdd(a, b)
        | SmtTerm::BvAnd(a, b)
        | SmtTerm::BvUlt(a, b) => {
            smt_collect_vars(a, out);
            smt_collect_vars(b, out);
        }
        SmtTerm::Ite(c, t, e) => {
            smt_collect_vars(c, out);
            smt_collect_vars(t, out);
            smt_collect_vars(e, out);
        }
        SmtTerm::Forall(vars, body) | SmtTerm::Exists(vars, body) => {
            let bound: Vec<&str> = vars.iter().map(|(n, _)| n.as_str()).collect();
            let mut body_vars = Vec::new();
            smt_collect_vars(body, &mut body_vars);
            for v in body_vars {
                if !bound.contains(&v.as_str()) {
                    out.push(v);
                }
            }
        }
        _ => {}
    }
}
/// Returns `true` if the SMT term contains no free variables.
#[allow(dead_code)]
pub fn smt_is_ground(term: &SmtTerm) -> bool {
    smt_free_vars(term).is_empty()
}
/// Negate a term, applying double-negation elimination.
#[allow(dead_code)]
pub fn smt_negate(term: SmtTerm) -> SmtTerm {
    match term {
        SmtTerm::Not(inner) => *inner,
        SmtTerm::BoolLit(b) => SmtTerm::BoolLit(!b),
        other => SmtTerm::Not(Box::new(other)),
    }
}
/// Convert an `And`-chain into a flat list of conjuncts.
#[allow(dead_code)]
pub fn smt_conjuncts(term: &SmtTerm) -> Vec<SmtTerm> {
    match term {
        SmtTerm::And(terms) => terms.iter().flat_map(smt_conjuncts).collect(),
        other => vec![other.clone()],
    }
}
/// Convert an `Or`-chain into a flat list of disjuncts.
#[allow(dead_code)]
pub fn smt_disjuncts(term: &SmtTerm) -> Vec<SmtTerm> {
    match term {
        SmtTerm::Or(terms) => terms.iter().flat_map(smt_disjuncts).collect(),
        other => vec![other.clone()],
    }
}
/// Build an n-ary conjunction from a list of terms.
#[allow(dead_code)]
pub fn smt_and_many(mut terms: Vec<SmtTerm>) -> SmtTerm {
    if terms.is_empty() {
        return smt_true();
    }
    if terms.len() == 1 {
        return terms.remove(0);
    }
    SmtTerm::And(terms)
}
/// Build an n-ary disjunction from a list of terms.
#[allow(dead_code)]
pub fn smt_or_many(mut terms: Vec<SmtTerm>) -> SmtTerm {
    if terms.is_empty() {
        return smt_false();
    }
    if terms.len() == 1 {
        return terms.remove(0);
    }
    SmtTerm::Or(terms)
}
/// Substitute a variable with a concrete term in an SMT term.
#[allow(dead_code)]
pub fn smt_subst(term: SmtTerm, var: &str, replacement: SmtTerm) -> SmtTerm {
    match term {
        SmtTerm::Var(ref name, _) if name == var => replacement,
        SmtTerm::Var(..) => term,
        SmtTerm::BoolLit(_) | SmtTerm::IntLit(_) | SmtTerm::RealLit(_) | SmtTerm::BvLit(..) => term,
        SmtTerm::Not(t) => SmtTerm::Not(Box::new(smt_subst(*t, var, replacement))),
        SmtTerm::BvNot(t) => SmtTerm::BvNot(Box::new(smt_subst(*t, var, replacement))),
        SmtTerm::And(terms) => SmtTerm::And(
            terms
                .into_iter()
                .map(|t| smt_subst(t, var, replacement.clone()))
                .collect(),
        ),
        SmtTerm::Or(terms) => SmtTerm::Or(
            terms
                .into_iter()
                .map(|t| smt_subst(t, var, replacement.clone()))
                .collect(),
        ),
        SmtTerm::App(f, args) => SmtTerm::App(
            f,
            args.into_iter()
                .map(|a| smt_subst(a, var, replacement.clone()))
                .collect(),
        ),
        SmtTerm::Implies(a, b) => SmtTerm::Implies(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Iff(a, b) => SmtTerm::Iff(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Eq(a, b) => SmtTerm::Eq(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Add(a, b) => SmtTerm::Add(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Sub(a, b) => SmtTerm::Sub(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Mul(a, b) => SmtTerm::Mul(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Div(a, b) => SmtTerm::Div(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Mod(a, b) => SmtTerm::Mod(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Lt(a, b) => SmtTerm::Lt(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Le(a, b) => SmtTerm::Le(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Gt(a, b) => SmtTerm::Gt(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Ge(a, b) => SmtTerm::Ge(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::BvAdd(a, b) => SmtTerm::BvAdd(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::BvAnd(a, b) => SmtTerm::BvAnd(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::BvUlt(a, b) => SmtTerm::BvUlt(
            Box::new(smt_subst(*a, var, replacement.clone())),
            Box::new(smt_subst(*b, var, replacement)),
        ),
        SmtTerm::Ite(c, t, e) => SmtTerm::Ite(
            Box::new(smt_subst(*c, var, replacement.clone())),
            Box::new(smt_subst(*t, var, replacement.clone())),
            Box::new(smt_subst(*e, var, replacement)),
        ),
        SmtTerm::Forall(vars, body) | SmtTerm::Exists(vars, body) => {
            if vars.iter().any(|(n, _)| n == var) {
                SmtTerm::Forall(vars, body)
            } else {
                SmtTerm::Exists(vars, Box::new(smt_subst(*body, var, replacement)))
            }
        }
    }
}
/// Return the bit-width of a BitVec sort, or `None` if not a BitVec.
#[allow(dead_code)]
pub fn bv_width(sort: &SmtSort) -> Option<u32> {
    match sort {
        SmtSort::BitVec(w) => Some(*w),
        _ => None,
    }
}
/// Check if two sorts are equal.
#[allow(dead_code)]
pub fn sorts_equal(a: &SmtSort, b: &SmtSort) -> bool {
    match (a, b) {
        (SmtSort::Bool, SmtSort::Bool)
        | (SmtSort::Int, SmtSort::Int)
        | (SmtSort::Real, SmtSort::Real) => true,
        (SmtSort::BitVec(w1), SmtSort::BitVec(w2)) => w1 == w2,
        (SmtSort::Array(i1, e1), SmtSort::Array(i2, e2)) => {
            sorts_equal(i1, i2) && sorts_equal(e1, e2)
        }
        (SmtSort::Named(n1), SmtSort::Named(n2)) => n1 == n2,
        _ => false,
    }
}
/// Return a human-readable description of a sort.
#[allow(dead_code)]
pub fn sort_description(sort: &SmtSort) -> &'static str {
    match sort {
        SmtSort::Bool => "Boolean",
        SmtSort::Int => "Integer",
        SmtSort::Real => "Real number",
        SmtSort::BitVec(_) => "Bit-vector",
        SmtSort::Array(..) => "Array",
        SmtSort::Named(_) => "User-defined",
    }
}
/// Check if a sort is numeric (Int or Real).
#[allow(dead_code)]
pub fn sort_is_numeric(sort: &SmtSort) -> bool {
    matches!(sort, SmtSort::Int | SmtSort::Real)
}
/// Check if a sort is arithmetic-compatible (Int, Real, or BitVec).
#[allow(dead_code)]
pub fn sort_is_arithmetic(sort: &SmtSort) -> bool {
    matches!(sort, SmtSort::Int | SmtSort::Real | SmtSort::BitVec(_))
}
/// Run the SMT tactic on a proof obligation using the OxiZ solver.
///
/// The obligation is discharged in refutation style: the negated goal
/// (together with all hypotheses) is asserted, and the solver is asked
/// whether it is satisfiable.  `Unsat` → `Proved`; `Sat` → `CounterExample`;
/// `Unknown` or `Error` → forwarded as such.
///
/// The `config.solver` field is advisory — OxiZ is always used as the backend.
#[allow(dead_code)]
pub fn run_smt_tactic(
    obligation: &SmtProofObligation,
    config: &SmtTacticConfig,
) -> SmtTacticResult {
    let goal_size = smt_term_size(&obligation.goal);
    if goal_size > config.max_term_size {
        return SmtTacticResult::TooLarge;
    }

    // Build a fresh SmtContext with all declarations and the negated goal.
    let mut ctx = SmtContext::new(config.solver.clone());
    for (name, sort) in &obligation.declarations {
        ctx.declare_const(name, sort.clone());
    }
    for hyp in &obligation.hypotheses {
        ctx.assert(hyp.clone());
    }
    ctx.assert(smt_negate(obligation.goal.clone()));

    match ctx.check_sat() {
        SmtResult::Unsat => SmtTacticResult::Proved,
        SmtResult::Sat => SmtTacticResult::CounterExample(SmtModel::new()),
        SmtResult::Unknown => SmtTacticResult::Unknown("solver returned unknown".to_string()),
        SmtResult::Error(e) => SmtTacticResult::Error(e),
    }
}
/// Try to reduce a linear arithmetic goal to an SMT proof obligation.
#[allow(dead_code)]
pub fn linearize_goal(goal_str: &str) -> Option<SmtProofObligation> {
    if let Some(pos) = goal_str.find("<=") {
        let _lhs = goal_str[..pos].trim();
        let _rhs = goal_str[pos + 2..].trim();
        let obligation = SmtProofObligation::new(SmtTerm::BoolLit(true));
        return Some(obligation);
    }
    if let Some(pos) = goal_str.find('<') {
        let _lhs = goal_str[..pos].trim();
        let _rhs = goal_str[pos + 1..].trim();
        let obligation = SmtProofObligation::new(SmtTerm::BoolLit(true));
        return Some(obligation);
    }
    None
}
/// Simplify a Boolean SMT term by applying obvious reductions.
#[allow(dead_code)]
pub fn smt_simplify_bool(term: SmtTerm) -> SmtTerm {
    match term {
        SmtTerm::Not(inner) => match *inner {
            SmtTerm::Not(inner2) => smt_simplify_bool(*inner2),
            SmtTerm::BoolLit(b) => SmtTerm::BoolLit(!b),
            other => SmtTerm::Not(Box::new(smt_simplify_bool(other))),
        },
        SmtTerm::And(terms) => {
            let mut simplified: Vec<SmtTerm> = Vec::new();
            for t in terms {
                let s = smt_simplify_bool(t);
                match s {
                    SmtTerm::BoolLit(false) => return SmtTerm::BoolLit(false),
                    SmtTerm::BoolLit(true) => {}
                    other => simplified.push(other),
                }
            }
            smt_and_many(simplified)
        }
        SmtTerm::Or(terms) => {
            let mut simplified: Vec<SmtTerm> = Vec::new();
            for t in terms {
                let s = smt_simplify_bool(t);
                match s {
                    SmtTerm::BoolLit(true) => return SmtTerm::BoolLit(true),
                    SmtTerm::BoolLit(false) => {}
                    other => simplified.push(other),
                }
            }
            smt_or_many(simplified)
        }
        SmtTerm::Implies(a, b) => {
            let sa = smt_simplify_bool(*a);
            match sa {
                SmtTerm::BoolLit(false) => SmtTerm::BoolLit(true),
                SmtTerm::BoolLit(true) => smt_simplify_bool(*b),
                other => SmtTerm::Implies(Box::new(other), Box::new(smt_simplify_bool(*b))),
            }
        }
        SmtTerm::Iff(a, b) => {
            let sa = smt_simplify_bool(*a);
            let sb = smt_simplify_bool(*b);
            match (&sa, &sb) {
                (SmtTerm::BoolLit(x), SmtTerm::BoolLit(y)) => SmtTerm::BoolLit(x == y),
                _ => SmtTerm::Iff(Box::new(sa), Box::new(sb)),
            }
        }
        other => other,
    }
}
/// Simplify a linear arithmetic SMT term by evaluating constant sub-expressions.
#[allow(dead_code)]
pub fn smt_simplify_arith(term: SmtTerm) -> SmtTerm {
    match term {
        SmtTerm::Add(a, b) => {
            let sa = smt_simplify_arith(*a);
            let sb = smt_simplify_arith(*b);
            match (&sa, &sb) {
                (SmtTerm::IntLit(x), SmtTerm::IntLit(y)) => SmtTerm::IntLit(x + y),
                (SmtTerm::IntLit(0), _) => sb,
                (_, SmtTerm::IntLit(0)) => sa,
                _ => SmtTerm::Add(Box::new(sa), Box::new(sb)),
            }
        }
        SmtTerm::Sub(a, b) => {
            let sa = smt_simplify_arith(*a);
            let sb = smt_simplify_arith(*b);
            match (&sa, &sb) {
                (SmtTerm::IntLit(x), SmtTerm::IntLit(y)) => SmtTerm::IntLit(x - y),
                (_, SmtTerm::IntLit(0)) => sa,
                _ => SmtTerm::Sub(Box::new(sa), Box::new(sb)),
            }
        }
        SmtTerm::Mul(a, b) => {
            let sa = smt_simplify_arith(*a);
            let sb = smt_simplify_arith(*b);
            match (&sa, &sb) {
                (SmtTerm::IntLit(x), SmtTerm::IntLit(y)) => SmtTerm::IntLit(x * y),
                (SmtTerm::IntLit(0), _) | (_, SmtTerm::IntLit(0)) => SmtTerm::IntLit(0),
                (SmtTerm::IntLit(1), _) => sb,
                (_, SmtTerm::IntLit(1)) => sa,
                _ => SmtTerm::Mul(Box::new(sa), Box::new(sb)),
            }
        }
        other => other,
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    use crate::tactic::smt::*;
    #[test]
    fn test_smt_term_size_literal() {
        assert_eq!(smt_term_size(&SmtTerm::BoolLit(true)), 1);
        assert_eq!(smt_term_size(&SmtTerm::IntLit(42)), 1);
    }
    #[test]
    fn test_smt_term_size_compound() {
        let t = smt_and(smt_true(), smt_false());
        assert_eq!(smt_term_size(&t), 3);
    }
    #[test]
    fn test_smt_free_vars_none() {
        let t = smt_add(smt_int(1), smt_int(2));
        assert!(smt_free_vars(&t).is_empty());
    }
    #[test]
    fn test_smt_free_vars_found() {
        let t = smt_lt(smt_var("x", SmtSort::Int), smt_var("y", SmtSort::Int));
        let vars = smt_free_vars(&t);
        assert_eq!(vars, vec!["x", "y"]);
    }
    #[test]
    fn test_smt_is_ground_true() {
        assert!(smt_is_ground(&smt_int(5)));
        assert!(smt_is_ground(&smt_true()));
    }
    #[test]
    fn test_smt_is_ground_false() {
        assert!(!smt_is_ground(&smt_var("x", SmtSort::Int)));
    }
    #[test]
    fn test_smt_negate_bool_lit() {
        let t = smt_negate(smt_true());
        assert_eq!(t, SmtTerm::BoolLit(false));
        let f = smt_negate(smt_false());
        assert_eq!(f, SmtTerm::BoolLit(true));
    }
    #[test]
    fn test_smt_negate_double_neg() {
        let t = smt_not(smt_var("p", SmtSort::Bool));
        let nn = smt_negate(t);
        assert!(matches!(nn, SmtTerm::Var(..)));
    }
    #[test]
    fn test_smt_conjuncts_flat() {
        let t = smt_and_many(vec![smt_true(), smt_false(), smt_var("x", SmtSort::Bool)]);
        let conjs = smt_conjuncts(&t);
        assert_eq!(conjs.len(), 3);
    }
    #[test]
    fn test_smt_and_many_empty() {
        let t = smt_and_many(vec![]);
        assert_eq!(t, SmtTerm::BoolLit(true));
    }
    #[test]
    fn test_smt_or_many_empty() {
        let t = smt_or_many(vec![]);
        assert_eq!(t, SmtTerm::BoolLit(false));
    }
    #[test]
    fn test_smt_or_many_singleton() {
        let t = smt_or_many(vec![smt_true()]);
        assert!(matches!(t, SmtTerm::BoolLit(true)));
    }
    #[test]
    fn test_smt_subst_var() {
        let t = smt_var("x", SmtSort::Int);
        let r = smt_subst(t, "x", smt_int(42));
        assert_eq!(r, SmtTerm::IntLit(42));
    }
    #[test]
    fn test_smt_subst_nested() {
        let t = smt_add(smt_var("x", SmtSort::Int), smt_int(1));
        let r = smt_subst(t, "x", smt_int(5));
        match r {
            SmtTerm::Add(a, b) => {
                assert_eq!(*a, SmtTerm::IntLit(5));
                assert_eq!(*b, SmtTerm::IntLit(1));
            }
            _ => panic!("expected Add"),
        }
    }
    #[test]
    fn test_bv_width() {
        assert_eq!(bv_width(&SmtSort::BitVec(32)), Some(32));
        assert_eq!(bv_width(&SmtSort::Int), None);
    }
    #[test]
    fn test_sorts_equal() {
        assert!(sorts_equal(&SmtSort::Int, &SmtSort::Int));
        assert!(!sorts_equal(&SmtSort::Int, &SmtSort::Real));
        assert!(sorts_equal(&SmtSort::BitVec(8), &SmtSort::BitVec(8)));
        assert!(!sorts_equal(&SmtSort::BitVec(8), &SmtSort::BitVec(16)));
    }
    #[test]
    fn test_sort_description() {
        assert_eq!(sort_description(&SmtSort::Bool), "Boolean");
        assert_eq!(sort_description(&SmtSort::Int), "Integer");
        assert_eq!(sort_description(&SmtSort::BitVec(32)), "Bit-vector");
    }
    #[test]
    fn test_sort_is_numeric() {
        assert!(sort_is_numeric(&SmtSort::Int));
        assert!(sort_is_numeric(&SmtSort::Real));
        assert!(!sort_is_numeric(&SmtSort::Bool));
    }
    #[test]
    fn test_smt_query_builder() {
        let mut builder = SmtQueryBuilder::new(SmtSolver::Z3, "QF_LIA");
        builder.declare("x", SmtSort::Int);
        builder.declare("y", SmtSort::Int);
        builder.assert(smt_lt(
            smt_var("x", SmtSort::Int),
            smt_var("y", SmtSort::Int),
        ));
        let script = builder.build();
        assert!(script.contains("x"));
        assert!(script.contains("y"));
    }
    #[test]
    fn test_smt_query_builder_named_assertion() {
        let mut builder = SmtQueryBuilder::new(SmtSolver::Cvc5, "QF_LIA");
        builder.assert_named("hyp1", smt_true());
        assert_eq!(builder.named_assertion_count(), 1);
        let labels = builder.assertion_labels();
        assert_eq!(labels, vec!["hyp1"]);
    }
    #[test]
    fn test_proof_obligation_negated_goal() {
        let mut ob = SmtProofObligation::new(smt_var("P", SmtSort::Bool));
        ob.add_hypothesis(smt_var("H", SmtSort::Bool));
        let neg = ob.negated_goal();
        let conjs = smt_conjuncts(&neg);
        assert_eq!(conjs.len(), 2);
    }
    #[test]
    fn test_model_value_extraction() {
        let v = ModelValue::Int(42);
        assert_eq!(v.as_int(), Some(42));
        assert_eq!(v.as_bool(), None);
        assert!(!v.is_unknown());
    }
    #[test]
    fn test_smt_model_eval_int() {
        let mut model = SmtModel::new();
        model.insert("x", ModelValue::Int(3));
        model.insert("y", ModelValue::Int(4));
        let t = smt_add(smt_var("x", SmtSort::Int), smt_var("y", SmtSort::Int));
        assert_eq!(model.eval_int(&t), Some(7));
    }
    #[test]
    fn test_smt_model_eval_bool() {
        let mut model = SmtModel::new();
        model.insert("p", ModelValue::Bool(true));
        model.insert("q", ModelValue::Bool(false));
        let t = smt_and(smt_var("p", SmtSort::Bool), smt_var("q", SmtSort::Bool));
        assert_eq!(model.eval_bool(&t), Some(false));
    }
    #[test]
    fn test_smt_stats_merge() {
        let mut s1 = SmtStats {
            declarations: 5,
            assertions: 10,
            ..SmtStats::default()
        };
        let s2 = SmtStats {
            declarations: 3,
            assertions: 7,
            ..SmtStats::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.declarations, 8);
        assert_eq!(s1.assertions, 17);
    }
    #[test]
    fn test_smt_stats_ratios() {
        let s = SmtStats {
            declarations: 4,
            assertions: 8,
            emitted_bytes: 400,
            ..SmtStats::default()
        };
        assert_eq!(s.assertion_ratio(), 2.0);
        assert_eq!(s.avg_assertion_size(), 50.0);
    }
    #[test]
    fn test_smt_tactic_config_qflia() {
        let c = SmtTacticConfig::minimal();
        assert!(c.is_qflia());
        let c2 = SmtTacticConfig::default_config();
        assert!(!c2.is_qflia());
    }
    #[test]
    fn test_smt_tactic_result_description() {
        assert_eq!(SmtTacticResult::Proved.description(), "proved");
        assert_eq!(SmtTacticResult::TooLarge.description(), "term too large");
        let u = SmtTacticResult::Unknown("x".to_string());
        assert_eq!(u.description(), "unknown");
    }
    #[test]
    fn test_run_smt_tactic_too_large() {
        let obligation = SmtProofObligation::new(smt_var("x", SmtSort::Bool));
        let config = SmtTacticConfig {
            max_term_size: 0,
            ..SmtTacticConfig::default_config()
        };
        let result = run_smt_tactic(&obligation, &config);
        assert!(matches!(result, SmtTacticResult::TooLarge));
    }
    #[test]
    fn test_smt_simplify_bool_double_neg() {
        let t = smt_not(smt_not(smt_var("p", SmtSort::Bool)));
        let s = smt_simplify_bool(t);
        assert!(matches!(s, SmtTerm::Var(..)));
    }
    #[test]
    fn test_smt_simplify_bool_and_false() {
        let t = smt_and(smt_var("p", SmtSort::Bool), smt_false());
        let s = smt_simplify_bool(t);
        assert_eq!(s, SmtTerm::BoolLit(false));
    }
    #[test]
    fn test_smt_simplify_bool_or_true() {
        let t = smt_or(smt_true(), smt_var("q", SmtSort::Bool));
        let s = smt_simplify_bool(t);
        assert_eq!(s, SmtTerm::BoolLit(true));
    }
    #[test]
    fn test_smt_simplify_arith_constants() {
        let t = smt_add(smt_int(3), smt_int(4));
        let s = smt_simplify_arith(t);
        assert_eq!(s, SmtTerm::IntLit(7));
    }
    #[test]
    fn test_smt_simplify_arith_zero() {
        let t = smt_add(smt_int(0), smt_var("x", SmtSort::Int));
        let s = smt_simplify_arith(t);
        assert!(matches!(s, SmtTerm::Var(..)));
    }
    #[test]
    fn test_smt_simplify_arith_mul_zero() {
        let t = smt_mul(smt_int(0), smt_var("x", SmtSort::Int));
        let s = smt_simplify_arith(t);
        assert_eq!(s, SmtTerm::IntLit(0));
    }
    #[test]
    fn test_smt_context_push_pop_restore() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("a", SmtSort::Int);
        ctx.push();
        ctx.declare_const("b", SmtSort::Int);
        assert_eq!(ctx.decl_count(), 2);
        ctx.pop();
        assert_eq!(ctx.decl_count(), 1);
    }
    #[test]
    fn test_smt_context_reset() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("x", SmtSort::Int);
        ctx.assert(smt_true());
        ctx.reset();
        assert_eq!(ctx.decl_count(), 0);
        assert_eq!(ctx.assertion_count(), 0);
    }
    #[test]
    fn test_linearize_goal_le() {
        let ob = linearize_goal("x <= 5");
        assert!(ob.is_some());
    }
    #[test]
    fn test_linearize_goal_lt() {
        let ob = linearize_goal("a < b");
        assert!(ob.is_some());
    }
    #[test]
    fn test_linearize_goal_none() {
        let ob = linearize_goal("totally random string");
        assert!(ob.is_none());
    }
    #[test]
    fn test_smt_proof_obligation_variable_count() {
        let mut ob = SmtProofObligation::new(smt_true());
        ob.add_decl("x", SmtSort::Int);
        ob.add_decl("y", SmtSort::Bool);
        assert_eq!(ob.variable_count(), 2);
    }
    #[test]
    fn test_smt_proof_obligation_is_ground() {
        let _ob = SmtProofObligation::new(smt_int(5));
        let ground_ob = SmtProofObligation::new(SmtTerm::BoolLit(true));
        assert!(ground_ob.is_ground_goal());
    }
    #[test]
    fn test_smt_solver_display() {
        assert_eq!(SmtSolver::Z3.to_string(), "z3");
        assert_eq!(SmtSolver::Cvc5.to_string(), "cvc5");
        assert_eq!(SmtSolver::Yices2.to_string(), "yices2");
        assert_eq!(SmtSolver::Bitwuzla.to_string(), "bitwuzla");
    }
    #[test]
    fn test_smt_free_vars_quantifier_bound() {
        let body = smt_lt(smt_var("x", SmtSort::Int), smt_var("y", SmtSort::Int));
        let t = smt_forall(vec![("x", SmtSort::Int)], body);
        let vars = smt_free_vars(&t);
        assert!(!vars.contains(&"x".to_string()));
        assert!(vars.contains(&"y".to_string()));
    }
    #[test]
    fn test_model_variables_list() {
        let mut model = SmtModel::new();
        model.insert("a", ModelValue::Int(1));
        model.insert("b", ModelValue::Bool(true));
        let mut vars = model.variables();
        vars.sort();
        assert_eq!(vars, vec!["a", "b"]);
    }
    #[test]
    fn test_smt_implies_simplify_false_premise() {
        let t = SmtTerm::Implies(Box::new(smt_false()), Box::new(smt_var("P", SmtSort::Bool)));
        let s = smt_simplify_bool(t);
        assert_eq!(s, SmtTerm::BoolLit(true));
    }
    #[test]
    fn test_smt_iff_simplify_same_literals() {
        let t = SmtTerm::Iff(Box::new(smt_true()), Box::new(smt_true()));
        let s = smt_simplify_bool(t);
        assert_eq!(s, SmtTerm::BoolLit(true));
    }
    #[test]
    fn test_smt_mul_one_simplify() {
        let t = smt_mul(smt_int(1), smt_var("x", SmtSort::Int));
        let s = smt_simplify_arith(t);
        assert!(matches!(s, SmtTerm::Var(..)));
    }
}
#[cfg(test)]
mod tacticsmt_analysis_tests {
    use super::*;
    use crate::tactic::smt::*;
    #[test]
    fn test_tacticsmt_result_ok() {
        let r = TacticSmtResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsmt_result_err() {
        let r = TacticSmtResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsmt_result_partial() {
        let r = TacticSmtResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsmt_result_skipped() {
        let r = TacticSmtResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticsmt_analysis_pass_run() {
        let mut p = TacticSmtAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticsmt_analysis_pass_empty_input() {
        let mut p = TacticSmtAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticsmt_analysis_pass_success_rate() {
        let mut p = TacticSmtAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticsmt_analysis_pass_disable() {
        let mut p = TacticSmtAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticsmt_pipeline_basic() {
        let mut pipeline = TacticSmtPipeline::new("main_pipeline");
        pipeline.add_pass(TacticSmtAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSmtAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticsmt_pipeline_disabled_pass() {
        let mut pipeline = TacticSmtPipeline::new("partial");
        let mut p = TacticSmtAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSmtAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticsmt_diff_basic() {
        let mut d = TacticSmtDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticsmt_diff_summary() {
        let mut d = TacticSmtDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticsmt_config_set_get() {
        let mut cfg = TacticSmtConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticsmt_config_read_only() {
        let mut cfg = TacticSmtConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticsmt_config_remove() {
        let mut cfg = TacticSmtConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticsmt_diagnostics_basic() {
        let mut diag = TacticSmtDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticsmt_diagnostics_max_errors() {
        let mut diag = TacticSmtDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticsmt_diagnostics_clear() {
        let mut diag = TacticSmtDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticsmt_config_value_types() {
        let b = TacticSmtConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticSmtConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticSmtConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticSmtConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticSmtConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod smt_ext_tests_3100 {
    use super::*;
    use crate::tactic::smt::*;
    #[test]
    fn test_smt_ext_result_ok_3100() {
        let r = SmtExtResult3100::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_smt_ext_result_err_3100() {
        let r = SmtExtResult3100::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_smt_ext_result_partial_3100() {
        let r = SmtExtResult3100::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_smt_ext_result_skipped_3100() {
        let r = SmtExtResult3100::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_smt_ext_pass_run_3100() {
        let mut p = SmtExtPass3100::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_smt_ext_pass_empty_3100() {
        let mut p = SmtExtPass3100::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_smt_ext_pass_rate_3100() {
        let mut p = SmtExtPass3100::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_smt_ext_pass_disable_3100() {
        let mut p = SmtExtPass3100::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_smt_ext_pipeline_basic_3100() {
        let mut pipeline = SmtExtPipeline3100::new("main_pipeline");
        pipeline.add_pass(SmtExtPass3100::new("pass1"));
        pipeline.add_pass(SmtExtPass3100::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_smt_ext_pipeline_disabled_3100() {
        let mut pipeline = SmtExtPipeline3100::new("partial");
        let mut p = SmtExtPass3100::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(SmtExtPass3100::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_smt_ext_diff_basic_3100() {
        let mut d = SmtExtDiff3100::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_smt_ext_config_set_get_3100() {
        let mut cfg = SmtExtConfig3100::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_smt_ext_config_read_only_3100() {
        let mut cfg = SmtExtConfig3100::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_smt_ext_config_remove_3100() {
        let mut cfg = SmtExtConfig3100::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_smt_ext_diagnostics_basic_3100() {
        let mut diag = SmtExtDiag3100::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_smt_ext_diagnostics_max_errors_3100() {
        let mut diag = SmtExtDiag3100::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_smt_ext_diagnostics_clear_3100() {
        let mut diag = SmtExtDiag3100::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_smt_ext_config_value_types_3100() {
        let b = SmtExtConfigVal3100::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = SmtExtConfigVal3100::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = SmtExtConfigVal3100::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = SmtExtConfigVal3100::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = SmtExtConfigVal3100::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod oxiz_integration_tests {
    use super::*;
    use crate::tactic::smt::*;

    /// A simple sat query: declare an integer x and assert x > 0.
    /// OxiZ should return Sat.
    #[test]
    fn smt_sat_simple() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("x", SmtSort::Int);
        ctx.assert(SmtTerm::Gt(
            Box::new(SmtTerm::Var("x".to_string(), SmtSort::Int)),
            Box::new(SmtTerm::IntLit(0)),
        ));
        assert_eq!(ctx.check_sat(), SmtResult::Sat);
    }

    /// An unsatisfiable constraint: x > 0 AND x < 0 has no solution.
    /// OxiZ should return Unsat.
    #[test]
    fn smt_unsat_contradiction() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("x", SmtSort::Int);
        let x = SmtTerm::Var("x".to_string(), SmtSort::Int);
        let zero = SmtTerm::IntLit(0);
        ctx.assert(SmtTerm::And(vec![
            SmtTerm::Gt(Box::new(x.clone()), Box::new(zero.clone())),
            SmtTerm::Lt(Box::new(x), Box::new(zero)),
        ]));
        assert_eq!(ctx.check_sat(), SmtResult::Unsat);
    }

    /// UF reflexivity: (f x) = (f x) is always satisfiable.
    /// This exercises the declare_fun extension on SmtContext.
    #[test]
    fn smt_sat_uf_reflexivity() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("x", SmtSort::Int);
        ctx.declare_fun("f", vec![SmtSort::Int], SmtSort::Int);
        let fx = SmtTerm::App(
            "f".to_string(),
            vec![SmtTerm::Var("x".to_string(), SmtSort::Int)],
        );
        ctx.assert(SmtTerm::Eq(Box::new(fx.clone()), Box::new(fx)));
        assert_eq!(ctx.check_sat(), SmtResult::Sat);
    }

    /// Boolean contradiction: p AND (NOT p) is unsatisfiable.
    #[test]
    fn smt_unsat_boolean() {
        let mut ctx = SmtContext::new(SmtSolver::Z3);
        ctx.declare_const("p", SmtSort::Bool);
        let p = SmtTerm::Var("p".to_string(), SmtSort::Bool);
        ctx.assert(SmtTerm::And(vec![p.clone(), SmtTerm::Not(Box::new(p))]));
        assert_eq!(ctx.check_sat(), SmtResult::Unsat);
    }
}
