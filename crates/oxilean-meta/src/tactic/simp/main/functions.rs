//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    MainExtConfig1100, MainExtConfigVal1100, MainExtDiag1100, MainExtDiff1100, MainExtPass1100,
    MainExtPipeline1100, MainExtResult1100, PrioritizedSimpSet, SimpLemmaFilter, SimpLemmaSelector,
    SimpLemmaSet, SimpRewriteLog, SimpRunSummary, SimpStats, TacticSimpMainAnalysisPass,
    TacticSimpMainConfig, TacticSimpMainConfigValue, TacticSimpMainDiagnostics, TacticSimpMainDiff,
    TacticSimpMainPipeline, TacticSimpMainResult,
};
use crate::basic::MetaContext;
use crate::tactic::simp::types::{SimpConfig, SimpLemma, SimpResult, SimpTheorems};
use oxilean_kernel::{beta_normalize, Expr, Name};

/// Apply the simp algorithm to an expression.
///
/// Traverses the expression bottom-up, applying matching lemmas
/// from the database, beta/eta/iota reductions, and congruence
/// lemmas at each node.
pub fn simp(
    expr: &Expr,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> SimpResult {
    let mut current = expr.clone();
    let mut changed = false;
    let mut steps = 0;
    loop {
        if steps >= config.max_steps {
            break;
        }
        steps += 1;
        let sub_result = simp_subexprs(&current, theorems, config, ctx);
        if let SimpResult::Simplified { new_expr, .. } = &sub_result {
            current = new_expr.clone();
            changed = true;
        }
        let reduced = apply_reductions(&current, config);
        if let Some(new_expr) = reduced {
            if new_expr != current {
                current = new_expr;
                changed = true;
                continue;
            }
        }
        let lemma_result = try_simp_lemmas(&current, theorems, ctx);
        match lemma_result {
            SimpResult::Simplified { new_expr, proof: _ } => {
                current = new_expr;
                changed = true;
                continue;
            }
            SimpResult::Proved(proof) => {
                return SimpResult::Proved(proof);
            }
            SimpResult::Unchanged => {}
        }
        break;
    }
    if is_true_expr(&current) {
        let proof = Expr::Const(Name::str("True.intro"), vec![]);
        return SimpResult::Proved(proof);
    }
    if changed {
        SimpResult::Simplified {
            new_expr: current,
            proof: None,
        }
    } else {
        SimpResult::Unchanged
    }
}
/// Simplify subexpressions recursively.
pub(super) fn simp_subexprs(
    expr: &Expr,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> SimpResult {
    match expr {
        Expr::App(f, a) => {
            let f_result = simp(f, theorems, config, ctx);
            let a_result = simp(a, theorems, config, ctx);
            let new_f = match &f_result {
                SimpResult::Simplified { new_expr, .. } => new_expr.clone(),
                _ => *f.clone(),
            };
            let new_a = match &a_result {
                SimpResult::Simplified { new_expr, .. } => new_expr.clone(),
                _ => *a.clone(),
            };
            if f_result.is_simplified() || a_result.is_simplified() {
                SimpResult::Simplified {
                    new_expr: Expr::App(Box::new(new_f), Box::new(new_a)),
                    proof: None,
                }
            } else {
                SimpResult::Unchanged
            }
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_result = simp(ty, theorems, config, ctx);
            let body_result = simp(body, theorems, config, ctx);
            let new_ty = match &ty_result {
                SimpResult::Simplified { new_expr, .. } => new_expr.clone(),
                _ => *ty.clone(),
            };
            let new_body = match &body_result {
                SimpResult::Simplified { new_expr, .. } => new_expr.clone(),
                _ => *body.clone(),
            };
            if ty_result.is_simplified() || body_result.is_simplified() {
                SimpResult::Simplified {
                    new_expr: Expr::Lam(*bi, name.clone(), Box::new(new_ty), Box::new(new_body)),
                    proof: None,
                }
            } else {
                SimpResult::Unchanged
            }
        }
        _ => SimpResult::Unchanged,
    }
}
/// Try to apply simp lemmas at the head of an expression.
pub(super) fn try_simp_lemmas(
    expr: &Expr,
    theorems: &SimpTheorems,
    _ctx: &mut MetaContext,
) -> SimpResult {
    let candidates = theorems.find_lemmas(expr);
    for lemma in candidates {
        if let Some(result) = try_apply_lemma(expr, lemma) {
            return result;
        }
    }
    SimpResult::Unchanged
}
/// Try to apply a single simp lemma.
///
/// Attempts to match `expr` against `lemma.lhs`, capturing free BVars
/// as wildcards. If the match succeeds, substitutes captured expressions
/// into `lemma.rhs` and returns the result.
pub(super) fn try_apply_lemma(expr: &Expr, lemma: &SimpLemma) -> Option<SimpResult> {
    let mut captures: Vec<Option<Expr>> = Vec::new();
    if match_pattern(expr, &lemma.lhs, &mut captures, 0) {
        let result_rhs = subst_captures(&lemma.rhs, &captures, 0);
        return Some(SimpResult::Simplified {
            new_expr: result_rhs,
            proof: Some(lemma.proof.clone()),
        });
    }
    None
}
/// Match `expr` against `pattern`, filling `captures[i]` for BVar(i) wildcards.
///
/// `depth` tracks how many binders we've entered (so that BVars that are free
/// in the pattern, i.e., with index >= depth, act as wildcards).
pub(super) fn match_pattern(
    expr: &Expr,
    pattern: &Expr,
    captures: &mut Vec<Option<Expr>>,
    depth: u32,
) -> bool {
    match pattern {
        Expr::BVar(idx) if *idx >= depth => {
            let slot = *idx as usize;
            while captures.len() <= slot {
                captures.push(None);
            }
            if let Some(ref existing) = captures[slot] {
                existing == expr
            } else {
                captures[slot] = Some(expr.clone());
                true
            }
        }
        Expr::BVar(idx) => matches!(expr, Expr::BVar(i) if i == idx),
        Expr::App(pf, pa) => {
            if let Expr::App(ef, ea) = expr {
                match_pattern(ef, pf, captures, depth) && match_pattern(ea, pa, captures, depth)
            } else {
                false
            }
        }
        Expr::Lam(pbi, pname, pty, pbody) => {
            if let Expr::Lam(ebi, ename, ety, ebody) = expr {
                ebi == pbi
                    && ename == pname
                    && match_pattern(ety, pty, captures, depth)
                    && match_pattern(ebody, pbody, captures, depth + 1)
            } else {
                false
            }
        }
        Expr::Pi(pbi, pname, pty, pbody) => {
            if let Expr::Pi(ebi, ename, ety, ebody) = expr {
                ebi == pbi
                    && ename == pname
                    && match_pattern(ety, pty, captures, depth)
                    && match_pattern(ebody, pbody, captures, depth + 1)
            } else {
                false
            }
        }
        _ => expr == pattern,
    }
}
/// Substitute captured wildcard expressions into a pattern RHS.
///
/// Replaces `BVar(i)` (free in the RHS, i.e. index >= `depth`) with the
/// captured expression at slot `i`.
pub(super) fn subst_captures(rhs: &Expr, captures: &[Option<Expr>], depth: u32) -> Expr {
    match rhs {
        Expr::BVar(idx) if *idx >= depth => {
            let slot = *idx as usize;
            if slot < captures.len() {
                if let Some(ref captured) = captures[slot] {
                    return captured.clone();
                }
            }
            rhs.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_captures(f, captures, depth)),
            Box::new(subst_captures(a, captures, depth)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(subst_captures(ty, captures, depth)),
            Box::new(subst_captures(body, captures, depth + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(subst_captures(ty, captures, depth)),
            Box::new(subst_captures(body, captures, depth + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(subst_captures(ty, captures, depth)),
            Box::new(subst_captures(val, captures, depth)),
            Box::new(subst_captures(body, captures, depth + 1)),
        ),
        _ => rhs.clone(),
    }
}
/// Check for syntactic match (simplified, no capture).
///
/// Kept for external callers; internally uses match_pattern with capture.
pub(super) fn syntactic_match(expr: &Expr, pattern: &Expr) -> bool {
    let mut captures = Vec::new();
    match_pattern(expr, pattern, &mut captures, 0)
}
/// Apply built-in reductions.
pub(super) fn apply_reductions(expr: &Expr, config: &SimpConfig) -> Option<Expr> {
    if config.beta {
        if let Expr::App(f, _a) = expr {
            if matches!(f.as_ref(), Expr::Lam(..)) {
                let reduced = beta_normalize(expr);
                if &reduced != expr {
                    return Some(reduced);
                }
            }
        }
        let reduced = beta_normalize(expr);
        if &reduced != expr {
            return Some(reduced);
        }
    }
    if config.eta {
        if let Expr::Lam(_, _, _, body) = expr {
            if let Expr::App(f, arg) = body.as_ref() {
                if matches!(arg.as_ref(), Expr::BVar(0)) && !contains_bvar0(f) {
                    let reduced = shift_bvars(f, -1, 0);
                    return Some(reduced);
                }
            }
        }
    }
    None
}
/// Check if expression contains BVar(0).
pub(super) fn contains_bvar0(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(0) => true,
        Expr::BVar(_) => false,
        Expr::App(f, a) => contains_bvar0(f) || contains_bvar0(a),
        Expr::Lam(_, _, ty, body) => contains_bvar0(ty) || contains_bvar0(body),
        Expr::Pi(_, _, ty, body) => contains_bvar0(ty) || contains_bvar0(body),
        Expr::Let(_, ty, val, body) => {
            contains_bvar0(ty) || contains_bvar0(val) || contains_bvar0(body)
        }
        _ => false,
    }
}
/// Shift all free BVars in an expression by `delta` (must be ≥ 0 if positive shift).
/// Only shifts BVars with index ≥ `cutoff` (depth of binders entered so far).
pub(super) fn shift_bvars(expr: &Expr, delta: i32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                let new_i = (*i as i32 + delta) as u32;
                Expr::BVar(new_i)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(shift_bvars(f, delta, cutoff)),
            Box::new(shift_bvars(a, delta, cutoff)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, delta, cutoff)),
            Box::new(shift_bvars(body, delta, cutoff + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, delta, cutoff)),
            Box::new(shift_bvars(body, delta, cutoff + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(shift_bvars(ty, delta, cutoff)),
            Box::new(shift_bvars(val, delta, cutoff)),
            Box::new(shift_bvars(body, delta, cutoff + 1)),
        ),
        _ => expr.clone(),
    }
}
/// Check if an expression is `True`.
pub(super) fn is_true_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if * name == Name::str("True"))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::simp::main::*;
    use crate::tactic::simp::types::default_simp_lemmas;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_lemma(name: &str, lhs: Expr, rhs: Expr) -> SimpLemma {
        SimpLemma {
            name: Name::str(name),
            lhs: lhs.clone(),
            rhs,
            proof: Expr::Const(Name::str(name), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        }
    }
    #[test]
    fn test_simp_unchanged() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = simp(&expr, &theorems, &config, &mut ctx);
        assert!(!result.is_simplified());
    }
    #[test]
    fn test_simp_with_lemma() {
        let mut ctx = mk_ctx();
        let mut theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        theorems.add_lemma(mk_lemma("ab", a.clone(), b.clone()));
        let result = simp(&a, &theorems, &config, &mut ctx);
        assert!(result.is_simplified());
        assert_eq!(*result.new_expr().expect("new_expr should succeed"), b);
    }
    #[test]
    fn test_simp_to_true() {
        let mut ctx = mk_ctx();
        let mut theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let p = Expr::Const(Name::str("P"), vec![]);
        let true_expr = Expr::Const(Name::str("True"), vec![]);
        theorems.add_lemma(mk_lemma("p_true", p.clone(), true_expr));
        let result = simp(&p, &theorems, &config, &mut ctx);
        assert!(result.is_proved());
    }
    #[test]
    fn test_simp_subexpr() {
        let mut ctx = mk_ctx();
        let mut theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        theorems.add_lemma(mk_lemma("ab", a.clone(), b.clone()));
        let f = Expr::Const(Name::str("f"), vec![]);
        let fa = Expr::App(Box::new(f.clone()), Box::new(a));
        let _fb = Expr::App(Box::new(f), Box::new(b));
        let result = simp(&fa, &theorems, &config, &mut ctx);
        assert!(result.is_simplified());
    }
    #[test]
    fn test_is_true_expr() {
        assert!(is_true_expr(&Expr::Const(Name::str("True"), vec![])));
        assert!(!is_true_expr(&Expr::Const(Name::str("False"), vec![])));
    }
    #[test]
    fn test_max_steps() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig {
            max_steps: 0,
            ..SimpConfig::default()
        };
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = simp(&expr, &theorems, &config, &mut ctx);
        assert!(!result.is_simplified());
    }
    #[test]
    fn test_beta_reduction() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let a = Expr::Const(Name::str("a"), vec![]);
        let id_fn = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id_fn), Box::new(a.clone()));
        let result = simp(&app, &theorems, &config, &mut ctx);
        assert!(result.is_simplified());
        assert_eq!(*result.new_expr().expect("new_expr should succeed"), a);
    }
    #[test]
    fn test_eta_reduction() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let f = Expr::Const(Name::str("f"), vec![]);
        let eta_f = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)))),
        );
        let result = simp(&eta_f, &theorems, &config, &mut ctx);
        assert!(result.is_simplified());
        assert_eq!(*result.new_expr().expect("new_expr should succeed"), f);
    }
    #[test]
    fn test_syntactic_match_bvar_wildcard() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let pattern = Expr::BVar(0);
        assert!(syntactic_match(&a, &pattern));
        assert!(syntactic_match(&b, &pattern));
    }
    #[test]
    fn test_syntactic_match_app_with_wildcard() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let pattern = Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)));
        let expr1 = Expr::App(Box::new(f.clone()), Box::new(a));
        let expr2 = Expr::App(Box::new(f), Box::new(b));
        assert!(syntactic_match(&expr1, &pattern));
        assert!(syntactic_match(&expr2, &pattern));
    }
    #[test]
    fn test_default_simp_lemmas_count() {
        use crate::tactic::simp::types::default_simp_lemmas;
        let db = default_simp_lemmas();
        assert!(
            db.num_lemmas() >= 20,
            "expected >= 20 default lemmas, got {}",
            db.num_lemmas()
        );
    }
    #[test]
    fn test_default_simp_has_nat_rules() {
        let db = default_simp_lemmas();
        let names: Vec<String> = db.all_lemmas().iter().map(|l| l.name.to_string()).collect();
        assert!(names.contains(&"Nat.add_zero".to_string()));
        assert!(names.contains(&"Nat.zero_add".to_string()));
        assert!(names.contains(&"Nat.mul_one".to_string()));
        assert!(names.contains(&"Nat.mul_zero".to_string()));
    }
    #[test]
    fn test_default_simp_has_list_rules() {
        let db = default_simp_lemmas();
        let names: Vec<String> = db.all_lemmas().iter().map(|l| l.name.to_string()).collect();
        assert!(names.contains(&"List.nil_append".to_string()));
        assert!(names.contains(&"List.append_nil".to_string()));
    }
    #[test]
    fn test_default_simp_has_logic_rules() {
        let db = default_simp_lemmas();
        let names: Vec<String> = db.all_lemmas().iter().map(|l| l.name.to_string()).collect();
        assert!(names.contains(&"true_and".to_string()));
        assert!(names.contains(&"and_false".to_string()));
        assert!(names.contains(&"not_false".to_string()));
    }
    #[test]
    fn test_simp_nat_add_zero() {
        let mut ctx = mk_ctx();
        let config = SimpConfig::default();
        let theorems = default_simp_lemmas();
        let n = Expr::Const(Name::str("myN"), vec![]);
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let add = Expr::Const(Name::str("Nat.add"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(add), Box::new(n.clone()))),
            Box::new(zero),
        );
        let result = simp(&expr, &theorems, &config, &mut ctx);
        assert!(result.is_simplified(), "n + 0 should simplify to n");
        assert_eq!(*result.new_expr().expect("new_expr should succeed"), n);
    }
}
/// Get the outermost constant name from an expression head.
pub(super) fn expr_head_name(expr: &Expr) -> Option<Name> {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    if let Expr::Const(name, _) = e {
        Some(name.clone())
    } else {
        None
    }
}
/// Simp normal form: repeatedly apply simp until no more progress.
pub fn simp_nf(
    expr: &Expr,
    theorems: &crate::tactic::simp::types::SimpTheorems,
    config: &crate::tactic::simp::types::SimpConfig,
    ctx: &mut MetaContext,
) -> Expr {
    match simp(expr, theorems, config, ctx) {
        crate::tactic::simp::types::SimpResult::Simplified { new_expr, .. } => new_expr,
        crate::tactic::simp::types::SimpResult::Proved(_) => Expr::Const(Name::str("True"), vec![]),
        crate::tactic::simp::types::SimpResult::Unchanged => expr.clone(),
    }
}
/// Simp a list of expressions, returning all results.
pub fn simp_many(
    exprs: &[Expr],
    theorems: &crate::tactic::simp::types::SimpTheorems,
    config: &crate::tactic::simp::types::SimpConfig,
    ctx: &mut MetaContext,
) -> Vec<crate::tactic::simp::types::SimpResult> {
    exprs
        .iter()
        .map(|e| simp(e, theorems, config, ctx))
        .collect()
}
/// Build a simp set from a list of equation pairs (name, lhs, rhs).
pub fn build_simp_set(
    equations: &[(Name, Expr, Expr)],
) -> crate::tactic::simp::types::SimpTheorems {
    use crate::tactic::simp::types::SimpLemma;
    let mut theorems = crate::tactic::simp::types::SimpTheorems::new();
    for (name, lhs, rhs) in equations {
        theorems.add_lemma(SimpLemma {
            name: name.clone(),
            lhs: lhs.clone(),
            rhs: rhs.clone(),
            proof: Expr::Const(name.clone(), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        });
    }
    theorems
}
/// Display a SimpResult for debugging.
pub fn display_simp_result(result: &crate::tactic::simp::types::SimpResult) -> String {
    match result {
        crate::tactic::simp::types::SimpResult::Unchanged => "unchanged".to_string(),
        crate::tactic::simp::types::SimpResult::Simplified { new_expr, proof } => {
            format!(
                "simplified to {:?} (proof: {})",
                new_expr,
                if proof.is_some() { "yes" } else { "no" }
            )
        }
        crate::tactic::simp::types::SimpResult::Proved(p) => format!("proved: {:?}", p),
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::tactic::simp::main::*;
    use crate::tactic::simp::types::{SimpConfig, SimpLemma, SimpTheorems};
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_lemma(name: &str, lhs: Expr, rhs: Expr) -> SimpLemma {
        SimpLemma {
            name: Name::str(name),
            lhs,
            rhs,
            proof: Expr::Const(Name::str(name), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        }
    }
    #[test]
    fn test_simp_stats_merge() {
        let mut s1 = SimpStats {
            rewrites_applied: 3,
            total_steps: 5,
            ..Default::default()
        };
        let s2 = SimpStats {
            rewrites_applied: 2,
            total_steps: 3,
            ..Default::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.rewrites_applied, 5);
        assert_eq!(s1.total_steps, 8);
    }
    #[test]
    fn test_simp_nf_returns_expr() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::Const(Name::str("x"), vec![]);
        let nf = simp_nf(&expr, &theorems, &config, &mut ctx);
        assert_eq!(nf, expr);
    }
    #[test]
    fn test_simp_many() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let exprs = vec![
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        ];
        let results = simp_many(&exprs, &theorems, &config, &mut ctx);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_build_simp_set() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let equations = vec![(Name::str("ab"), a.clone(), b)];
        let theorems = build_simp_set(&equations);
        let candidates = theorems.find_lemmas(&a);
        assert!(!candidates.is_empty());
    }
    #[test]
    fn test_simp_lemma_filter_min_priority() {
        let low = SimpLemma {
            name: Name::str("low"),
            lhs: Expr::Const(Name::str("x"), vec![]),
            rhs: Expr::Const(Name::str("y"), vec![]),
            proof: Expr::Const(Name::str("low"), vec![]),
            priority: 100,
            is_conditional: false,
            is_forward: true,
        };
        let filter = SimpLemmaFilter::new().min_priority(500);
        assert!(!filter.accepts(&low));
        let filter2 = SimpLemmaFilter::new().min_priority(50);
        assert!(filter2.accepts(&low));
    }
    #[test]
    fn test_expr_head_name_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(expr_head_name(&e), Some(Name::str("Nat")));
    }
    #[test]
    fn test_expr_head_name_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(expr_head_name(&app), Some(Name::str("f")));
    }
    #[test]
    fn test_display_simp_result_unchanged() {
        let r = crate::tactic::simp::types::SimpResult::Unchanged;
        assert_eq!(display_simp_result(&r), "unchanged");
    }
    #[test]
    fn test_prioritized_insert_order() {
        let mut set = PrioritizedSimpSet::new();
        set.insert(mk_lemma(
            "low",
            Expr::Const(Name::str("low"), vec![]),
            Expr::BVar(0),
        ));
        set.insert(SimpLemma {
            name: Name::str("high"),
            lhs: Expr::Const(Name::str("high"), vec![]),
            rhs: Expr::BVar(0),
            proof: Expr::Const(Name::str("high"), vec![]),
            priority: 2000,
            is_conditional: false,
            is_forward: true,
        });
        assert_eq!(set.len(), 2);
        let first = set
            .iter()
            .next()
            .expect("iterator should have next element");
        assert_eq!(first.priority, 2000);
    }
    #[test]
    fn test_prioritized_empty() {
        let set = PrioritizedSimpSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
}
/// Repeated simp until fixpoint with a maximum iteration limit.
///
/// Returns the final simplified expression and how many iterations were taken.
pub fn simp_to_fixpoint(
    expr: &Expr,
    theorems: &crate::tactic::simp::types::SimpTheorems,
    config: &crate::tactic::simp::types::SimpConfig,
    ctx: &mut MetaContext,
    max_iters: usize,
) -> (Expr, usize) {
    let mut current = expr.clone();
    let mut iters = 0;
    loop {
        if iters >= max_iters {
            break;
        }
        iters += 1;
        match simp(&current, theorems, config, ctx) {
            crate::tactic::simp::types::SimpResult::Simplified { new_expr, .. } => {
                if new_expr == current {
                    break;
                }
                current = new_expr;
            }
            _ => break,
        }
    }
    (current, iters)
}
/// Check if a simp set is confluent on a given expression.
///
/// Two simp sets are confluent if they produce the same result
/// regardless of the order lemmas are applied. This is a heuristic check.
pub fn is_confluent(
    expr: &Expr,
    theorems: &crate::tactic::simp::types::SimpTheorems,
    ctx: &mut MetaContext,
) -> bool {
    let config = crate::tactic::simp::types::SimpConfig::default();
    let (result1, _) = simp_to_fixpoint(expr, theorems, &config, ctx, 100);
    let (result2, _) = simp_to_fixpoint(expr, theorems, &config, ctx, 100);
    result1 == result2
}
#[cfg(test)]
mod fixpoint_tests {
    use super::*;
    use crate::tactic::simp::main::*;
    use crate::tactic::simp::types::{SimpConfig, SimpTheorems};
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_simp_to_fixpoint_unchanged() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::Const(Name::str("x"), vec![]);
        let (result, iters) = simp_to_fixpoint(&expr, &theorems, &config, &mut ctx, 10);
        assert_eq!(result, expr);
        assert_eq!(iters, 1);
    }
    #[test]
    fn test_is_confluent_no_lemmas() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let expr = Expr::Const(Name::str("a"), vec![]);
        assert!(is_confluent(&expr, &theorems, &mut ctx));
    }
    #[test]
    fn test_simp_to_fixpoint_max_iters() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::BVar(0);
        let (_, iters) = simp_to_fixpoint(&expr, &theorems, &config, &mut ctx, 5);
        assert!(iters <= 5);
    }
    #[test]
    fn test_simp_nf_to_true() {
        let mut ctx = mk_ctx();
        let mut theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let p = Expr::Const(Name::str("P"), vec![]);
        let true_expr = Expr::Const(Name::str("True"), vec![]);
        theorems.add_lemma(SimpLemma {
            name: Name::str("p_true"),
            lhs: p.clone(),
            rhs: true_expr.clone(),
            proof: Expr::Const(Name::str("p_true"), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        });
        let nf = simp_nf(&p, &theorems, &config, &mut ctx);
        assert_eq!(nf, true_expr);
    }
    #[test]
    fn test_simp_many_empty() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let results = simp_many(&[], &theorems, &config, &mut ctx);
        assert!(results.is_empty());
    }
}
/// Simp with a custom rewrite function applied after each step.
///
/// The hook function is called with the expression before and after each step.
pub fn simp_with_hook<F>(
    expr: &Expr,
    theorems: &crate::tactic::simp::types::SimpTheorems,
    config: &crate::tactic::simp::types::SimpConfig,
    ctx: &mut MetaContext,
    mut hook: F,
) -> crate::tactic::simp::types::SimpResult
where
    F: FnMut(&Expr, &Expr),
{
    let result = simp(expr, theorems, config, ctx);
    if let crate::tactic::simp::types::SimpResult::Simplified { ref new_expr, .. } = result {
        hook(expr, new_expr);
    }
    result
}
/// Count lemmas in a theorem set that could potentially match an expression.
pub fn count_potential_matches(
    expr: &Expr,
    theorems: &crate::tactic::simp::types::SimpTheorems,
) -> usize {
    theorems.find_lemmas(expr).len()
}
#[cfg(test)]
mod hook_tests {
    use super::*;
    use crate::tactic::simp::main::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_simp_with_hook_no_change() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::Const(Name::str("x"), vec![]);
        let mut hook_called = false;
        simp_with_hook(&expr, &theorems, &config, &mut ctx, |_, _| {
            hook_called = true;
        });
        assert!(!hook_called);
    }
    #[test]
    fn test_simp_with_hook_changed() {
        let mut ctx = mk_ctx();
        let mut theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        theorems.add_lemma(SimpLemma {
            name: Name::str("ab"),
            lhs: a.clone(),
            rhs: b.clone(),
            proof: Expr::Const(Name::str("ab"), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        });
        let mut hook_called = false;
        simp_with_hook(&a, &theorems, &config, &mut ctx, |_, _| {
            hook_called = true;
        });
        assert!(hook_called);
    }
    #[test]
    fn test_count_potential_matches_empty() {
        let theorems = SimpTheorems::new();
        let expr = Expr::BVar(0);
        assert_eq!(count_potential_matches(&expr, &theorems), 0);
    }
}
#[cfg(test)]
mod simp_main_ext_tests {
    use super::*;
    use crate::tactic::simp::main::*;
    use oxilean_kernel::{Environment, Level, Name};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_lemma(name: &str, priority: u32, cond: bool) -> SimpLemma {
        SimpLemma {
            name: Name::str(name),
            lhs: Expr::BVar(0),
            rhs: Expr::BVar(1),
            proof: Expr::BVar(0),
            is_conditional: cond,
            is_forward: true,
            priority,
        }
    }
    #[test]
    fn test_simp_rewrite_log_record() {
        let mut log = SimpRewriteLog::new();
        let e = Expr::Const(Name::str("a"), vec![]);
        log.record(Name::str("rule"), e.clone(), e);
        assert_eq!(log.len(), 1);
    }
    #[test]
    fn test_simp_rewrite_log_bounded() {
        let mut log = SimpRewriteLog::bounded(2);
        let e = Expr::Const(Name::str("a"), vec![]);
        log.record(Name::str("r1"), e.clone(), e.clone());
        log.record(Name::str("r2"), e.clone(), e.clone());
        log.record(Name::str("r3"), e.clone(), e.clone());
        assert_eq!(log.len(), 2);
    }
    #[test]
    fn test_simp_rewrite_log_distinct_lemma_count() {
        let mut log = SimpRewriteLog::new();
        let e = Expr::Sort(Level::zero());
        log.record(Name::str("add_comm"), e.clone(), e.clone());
        log.record(Name::str("add_comm"), e.clone(), e.clone());
        log.record(Name::str("mul_one"), e.clone(), e);
        assert_eq!(log.distinct_lemma_count(), 2);
    }
    #[test]
    fn test_simp_rewrite_log_get_step() {
        let mut log = SimpRewriteLog::new();
        let e = Expr::BVar(0);
        log.record(Name::str("r"), e.clone(), e);
        let step = log.get_step(0);
        assert!(step.is_some());
    }
    #[test]
    fn test_simp_rewrite_log_clear() {
        let mut log = SimpRewriteLog::new();
        log.record(Name::str("r"), Expr::BVar(0), Expr::BVar(1));
        log.clear();
        assert!(log.is_empty());
    }
    #[test]
    fn test_simp_run_summary_record() {
        let mut s = SimpRunSummary::new();
        s.record(&SimpResult::Unchanged);
        s.record(&SimpResult::Proved(Expr::BVar(0)));
        assert_eq!(s.num_runs, 2);
        assert_eq!(s.proved_runs, 1);
        assert_eq!(s.unchanged_runs, 1);
    }
    #[test]
    fn test_simp_run_summary_progress_rate() {
        let mut s = SimpRunSummary::new();
        s.record(&SimpResult::Unchanged);
        s.record(&SimpResult::Proved(Expr::BVar(0)));
        assert!((s.progress_rate() - 0.5).abs() < 1e-5);
    }
    #[test]
    fn test_simp_run_summary_display() {
        let s = SimpRunSummary::new();
        let d = format!("{}", s);
        assert!(d.contains("SimpRunSummary"));
    }
    #[test]
    fn test_simp_lemma_selector_all() {
        let sel = SimpLemmaSelector::all();
        let l = mk_lemma("test", 1000, true);
        assert!(sel.accepts(&l));
    }
    #[test]
    fn test_simp_lemma_selector_unconditional() {
        let sel = SimpLemmaSelector::unconditional_only();
        let cond = mk_lemma("c", 1000, true);
        let uncond = mk_lemma("u", 1000, false);
        assert!(!sel.accepts(&cond));
        assert!(sel.accepts(&uncond));
    }
    #[test]
    fn test_simp_lemma_selector_min_priority() {
        let sel = SimpLemmaSelector::with_min_priority(500);
        assert!(!sel.accepts(&mk_lemma("low", 100, false)));
        assert!(sel.accepts(&mk_lemma("high", 1000, false)));
    }
    #[test]
    fn test_simp_lemma_set_add_remove() {
        let mut set = SimpLemmaSet::new("test");
        set.add(mk_lemma("r1", 1000, false));
        set.add(mk_lemma("r2", 500, false));
        assert_eq!(set.len(), 2);
        set.remove(&Name::str("r1"));
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_simp_lemma_set_select() {
        let mut set = SimpLemmaSet::new("test");
        set.add(mk_lemma("r1", 1000, false));
        set.add(mk_lemma("r2", 100, true));
        let sel = SimpLemmaSelector::with_min_priority(500);
        let selected = set.select(&sel);
        assert_eq!(selected.len(), 1);
    }
    #[test]
    fn test_simp_lemma_set_sort_by_priority() {
        let mut set = SimpLemmaSet::new("test");
        set.add(mk_lemma("low", 100, false));
        set.add(mk_lemma("high", 1000, false));
        set.sort_by_priority();
        assert_eq!(set.lemmas[0].priority, 1000);
    }
    #[test]
    fn test_simp_lemma_set_merge() {
        let mut s1 = SimpLemmaSet::new("s1");
        s1.add(mk_lemma("r1", 1000, false));
        let mut s2 = SimpLemmaSet::new("s2");
        s2.add(mk_lemma("r2", 500, false));
        s1.merge(&s2);
        assert_eq!(s1.len(), 2);
    }
    #[test]
    fn test_simp_to_fixpoint_unchanged() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let expr = Expr::Const(Name::str("x"), vec![]);
        let (result, iters) = simp_to_fixpoint(&expr, &theorems, &config, &mut ctx, 10);
        assert_eq!(result, expr);
        assert_eq!(iters, 1);
    }
    #[test]
    fn test_simp_many_multiple() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let exprs = vec![
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        ];
        let results = simp_many(&exprs, &theorems, &config, &mut ctx);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_simp_nf_identity() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let nf = simp_nf(&e, &theorems, &config, &mut ctx);
        assert_eq!(nf, e);
    }
    #[test]
    fn test_count_potential_matches_zero() {
        let theorems = SimpTheorems::new();
        let e = Expr::Sort(Level::zero());
        assert_eq!(count_potential_matches(&e, &theorems), 0);
    }
}
#[cfg(test)]
mod tacticsimpmain_analysis_tests {
    use super::*;
    use crate::tactic::simp::main::*;
    #[test]
    fn test_tacticsimpmain_result_ok() {
        let r = TacticSimpMainResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmain_result_err() {
        let r = TacticSimpMainResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmain_result_partial() {
        let r = TacticSimpMainResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpmain_result_skipped() {
        let r = TacticSimpMainResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticsimpmain_analysis_pass_run() {
        let mut p = TacticSimpMainAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticsimpmain_analysis_pass_empty_input() {
        let mut p = TacticSimpMainAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticsimpmain_analysis_pass_success_rate() {
        let mut p = TacticSimpMainAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticsimpmain_analysis_pass_disable() {
        let mut p = TacticSimpMainAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticsimpmain_pipeline_basic() {
        let mut pipeline = TacticSimpMainPipeline::new("main_pipeline");
        pipeline.add_pass(TacticSimpMainAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSimpMainAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticsimpmain_pipeline_disabled_pass() {
        let mut pipeline = TacticSimpMainPipeline::new("partial");
        let mut p = TacticSimpMainAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSimpMainAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticsimpmain_diff_basic() {
        let mut d = TacticSimpMainDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticsimpmain_diff_summary() {
        let mut d = TacticSimpMainDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticsimpmain_config_set_get() {
        let mut cfg = TacticSimpMainConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticsimpmain_config_read_only() {
        let mut cfg = TacticSimpMainConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticsimpmain_config_remove() {
        let mut cfg = TacticSimpMainConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticsimpmain_diagnostics_basic() {
        let mut diag = TacticSimpMainDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticsimpmain_diagnostics_max_errors() {
        let mut diag = TacticSimpMainDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticsimpmain_diagnostics_clear() {
        let mut diag = TacticSimpMainDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticsimpmain_config_value_types() {
        let b = TacticSimpMainConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticSimpMainConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticSimpMainConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticSimpMainConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticSimpMainConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod main_ext_tests_1100 {
    use super::*;
    use crate::tactic::simp::main::*;
    #[test]
    fn test_main_ext_result_ok_1100() {
        let r = MainExtResult1100::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_main_ext_result_err_1100() {
        let r = MainExtResult1100::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_main_ext_result_partial_1100() {
        let r = MainExtResult1100::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_main_ext_result_skipped_1100() {
        let r = MainExtResult1100::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_main_ext_pass_run_1100() {
        let mut p = MainExtPass1100::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_main_ext_pass_empty_1100() {
        let mut p = MainExtPass1100::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_main_ext_pass_rate_1100() {
        let mut p = MainExtPass1100::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_main_ext_pass_disable_1100() {
        let mut p = MainExtPass1100::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_main_ext_pipeline_basic_1100() {
        let mut pipeline = MainExtPipeline1100::new("main_pipeline");
        pipeline.add_pass(MainExtPass1100::new("pass1"));
        pipeline.add_pass(MainExtPass1100::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_main_ext_pipeline_disabled_1100() {
        let mut pipeline = MainExtPipeline1100::new("partial");
        let mut p = MainExtPass1100::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MainExtPass1100::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_main_ext_diff_basic_1100() {
        let mut d = MainExtDiff1100::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_main_ext_config_set_get_1100() {
        let mut cfg = MainExtConfig1100::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_main_ext_config_read_only_1100() {
        let mut cfg = MainExtConfig1100::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_main_ext_config_remove_1100() {
        let mut cfg = MainExtConfig1100::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_main_ext_diagnostics_basic_1100() {
        let mut diag = MainExtDiag1100::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_main_ext_diagnostics_max_errors_1100() {
        let mut diag = MainExtDiag1100::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_main_ext_diagnostics_clear_1100() {
        let mut diag = MainExtDiag1100::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_main_ext_config_value_types_1100() {
        let b = MainExtConfigVal1100::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MainExtConfigVal1100::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MainExtConfigVal1100::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MainExtConfigVal1100::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MainExtConfigVal1100::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
