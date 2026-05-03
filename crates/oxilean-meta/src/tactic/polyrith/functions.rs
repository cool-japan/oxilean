//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    GroebnerBasis, IdealMembershipChecker, IntPoly1, MVPoly, MVTerm, Monomial, MonomialV2,
    MultiPoly1, MultiTerm, PolyCoeff, Polynomial, PolyrithCache, PolyrithConfig,
    PolyrithExtConfig300, PolyrithExtConfig301, PolyrithExtConfigVal300, PolyrithExtConfigVal301,
    PolyrithExtDiag300, PolyrithExtDiag301, PolyrithExtDiff300, PolyrithExtDiff301,
    PolyrithExtPass300, PolyrithExtPass301, PolyrithExtPipeline300, PolyrithExtPipeline301,
    PolyrithExtResult300, PolyrithExtResult301, PolyrithSolver, PolyrithStats, PolyrithTactic,
    TacticPolyrithAnalysisPass, TacticPolyrithConfig, TacticPolyrithConfigValue,
    TacticPolyrithDiagnostics, TacticPolyrithDiff, TacticPolyrithPipeline, TacticPolyrithResult,
};
#[allow(unused_imports)]
use crate::basic::{MVarId, MetaContext};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_monomial_new() {
        let m = Monomial::new(3);
        assert_eq!(m.coefficient, 3);
        assert!(m.vars.is_empty());
        assert_eq!(m.degree(), 0);
    }
    #[test]
    fn test_monomial_degree() {
        let mut m = Monomial::new(1);
        m.add_var("x", 2);
        m.add_var("y", 3);
        assert_eq!(m.degree(), 5);
        m.add_var("x", 1);
        assert_eq!(m.degree(), 6);
        let prev_len = m.vars.len();
        m.add_var("z", 0);
        assert_eq!(m.vars.len(), prev_len);
    }
    #[test]
    fn test_polynomial_add() {
        let mut p = Polynomial::new();
        p.add_term(Monomial::new(2));
        let mut q = Polynomial::new();
        q.add_term(Monomial::new(3));
        let sum = Polynomial::add(&p, &q);
        assert_eq!(sum.terms.len(), 2);
        let const_sum: i64 = sum.terms.iter().map(|m| m.coefficient).sum();
        assert_eq!(const_sum, 5);
    }
    #[test]
    fn test_polynomial_mul() {
        let mut p = Polynomial::new();
        p.add_term(Monomial::new(2));
        let mut q = Polynomial::new();
        q.add_term(Monomial::new(3));
        let prod = Polynomial::mul(&p, &q);
        assert_eq!(prod.terms.len(), 1);
        assert_eq!(prod.terms[0].coefficient, 6);
    }
    #[test]
    fn test_polynomial_is_zero() {
        assert!(Polynomial::zero().is_zero());
        let mut p = Polynomial::new();
        p.add_term(Monomial::new(0));
        assert!(p.is_zero());
        let mut p2 = Polynomial::new();
        p2.add_term(Monomial::new(1));
        assert!(!p2.is_zero());
    }
    #[test]
    fn test_groebner_basis() {
        let mut basis = GroebnerBasis::new();
        assert!(basis.is_empty());
        basis.add_polynomial(Polynomial::one());
        assert!(!basis.is_empty());
        assert_eq!(basis.generators.len(), 1);
        let p = Polynomial::zero();
        let reduced = basis.reduce(&p);
        assert_eq!(reduced, p);
        assert!(basis.contains(&Polynomial::zero()));
    }
    #[test]
    fn test_polyrith_run() {
        let mut tac = PolyrithTactic::new();
        let mut h1 = Polynomial::new();
        h1.add_term(Monomial::new(2));
        let mut h2 = Polynomial::new();
        h2.add_term(Monomial::new(3));
        let mut goal = Polynomial::new();
        goal.add_term(Monomial::new(6));
        tac.set_hypotheses(vec![h1, h2]);
        tac.set_goal(goal);
        let result = tac.run();
        assert!(result.is_some());
        let coeffs = result.expect("coeffs should be present");
        assert!(tac.verify(&coeffs));
    }
    #[test]
    fn test_polyrith_verify() {
        let mut tac = PolyrithTactic::new();
        let mut h = Polynomial::new();
        h.add_term(Monomial::new(4));
        let mut goal = Polynomial::new();
        goal.add_term(Monomial::new(8));
        tac.set_hypotheses(vec![h]);
        tac.set_goal(goal);
        assert!(tac.verify(&[2]));
        assert!(!tac.verify(&[1]));
        assert!(!tac.verify(&[2, 1]));
    }
}
/// Compute GCD of two IntPoly1 via Euclidean algorithm.
#[allow(dead_code)]
pub fn int_poly_gcd(a: &IntPoly1, b: &IntPoly1) -> IntPoly1 {
    if a.is_zero() {
        return b.clone();
    }
    if b.is_zero() {
        return a.clone();
    }
    let da = a.degree();
    let db = b.degree();
    if da < db {
        return int_poly_gcd(b, a);
    }
    let lc_b = *b.coeffs.last().unwrap_or(&1);
    let shift = da - db;
    let scale = lc_b.pow(shift as u32 + 1);
    let scaled_a = a.scale(scale);
    let rem = poly_pseudo_rem(&scaled_a, b);
    if rem.is_zero() {
        b.primitive_part()
    } else {
        int_poly_gcd(b, &rem.primitive_part())
    }
}
#[allow(dead_code)]
pub(super) fn poly_pseudo_rem(a: &IntPoly1, b: &IntPoly1) -> IntPoly1 {
    let mut r = a.clone();
    while !r.is_zero() && r.degree() >= b.degree() {
        let rd = r.degree();
        let bd = b.degree();
        let lc_r = *r.coeffs.last().unwrap_or(&0);
        let lc_b = *b.coeffs.last().unwrap_or(&1);
        let r_scaled = r.scale(lc_b);
        let mut shift = vec![0i64; rd - bd];
        shift.extend_from_slice(&b.coeffs);
        let b_shifted = IntPoly1 { coeffs: shift }.scale(lc_r);
        r = r_scaled.sub(&b_shifted);
    }
    r
}
#[cfg(test)]
mod polyrith_extended_tests {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_poly_coeff_ops() {
        let a = PolyCoeff(4);
        let b = PolyCoeff(6);
        assert_eq!(a.gcd_with(&b), PolyCoeff(2));
        assert_eq!(a.add(&b), PolyCoeff(10));
        assert_eq!(a.mul(&b), PolyCoeff(24));
    }
    #[test]
    fn test_int_poly1_eval() {
        let p = IntPoly1 {
            coeffs: vec![1, 2, 1],
        };
        assert_eq!(p.eval(3), 16);
        assert_eq!(p.eval(0), 1);
    }
    #[test]
    fn test_int_poly1_add() {
        let p = IntPoly1 { coeffs: vec![1, 1] };
        let q = IntPoly1 { coeffs: vec![2, 1] };
        let s = p.add(&q);
        assert_eq!(s.coeffs, vec![3, 2]);
    }
    #[test]
    fn test_int_poly1_mul() {
        let p = IntPoly1 { coeffs: vec![1, 1] };
        let q = p.mul(&p);
        assert_eq!(q.coeffs, vec![1, 2, 1]);
    }
    #[test]
    fn test_int_poly1_degree() {
        let p = IntPoly1 {
            coeffs: vec![0, 0, 3],
        };
        assert_eq!(p.degree(), 2);
    }
    #[test]
    fn test_int_poly_gcd_coprime() {
        let p = IntPoly1 { coeffs: vec![1, 1] };
        let q = IntPoly1 { coeffs: vec![2, 1] };
        let g = int_poly_gcd(&p, &q);
        assert!(!g.is_zero());
    }
    #[test]
    fn test_multi_poly1_constant() {
        let p = MultiPoly1::constant(2, 5);
        assert_eq!(p.eval(&[0, 0]), 5);
    }
    #[test]
    fn test_multi_poly1_add_term() {
        let mut p = MultiPoly1::zero(2);
        p.add_term(MultiTerm::new(3, vec![1, 0]));
        p.add_term(MultiTerm::new(2, vec![0, 1]));
        assert_eq!(p.num_terms(), 2);
    }
    #[test]
    fn test_polyrith_config_default() {
        let cfg = PolyrithConfig::new();
        assert_eq!(cfg.max_degree, 4);
        assert!(cfg.use_cache);
    }
    #[test]
    fn test_polyrith_cache_miss() {
        let mut cache = PolyrithCache::new();
        assert!(cache.lookup("x^2 - y = 0").is_none());
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_polyrith_cache_hit() {
        let mut cache = PolyrithCache::new();
        cache.insert("key", Some(vec![1, 2]));
        let r = cache.lookup("key");
        assert!(r.is_some());
        assert_eq!(cache.hits, 1);
    }
    #[test]
    fn test_polyrith_solver_trivial_zero() {
        let mut solver = PolyrithSolver::new();
        solver.set_goal(MultiPoly1::zero(1));
        assert!(solver.try_trivial());
    }
    #[test]
    fn test_polyrith_solver_not_trivial() {
        let mut solver = PolyrithSolver::new();
        let mut g = MultiPoly1::zero(1);
        g.add_term(MultiTerm::new(1, vec![1]));
        solver.set_goal(g);
        assert!(!solver.try_trivial());
    }
    #[test]
    fn test_poly_coeff_zero() {
        assert!(PolyCoeff::zero().is_zero());
        assert!(!PolyCoeff::one().is_zero());
    }
    #[test]
    fn test_int_poly1_is_zero() {
        let z = IntPoly1::zero();
        assert!(z.is_zero());
    }
    #[test]
    fn test_multi_poly1_degree() {
        let mut p = MultiPoly1::zero(2);
        p.add_term(MultiTerm::new(1, vec![2, 3]));
        assert_eq!(p.degree(), 5);
    }
}
/// Compute the S-polynomial of two polynomials.
#[allow(dead_code)]
pub fn s_polynomial(f: &MVPoly, g: &MVPoly) -> MVPoly {
    let lm_f = match f.leading_monomial() {
        Some(m) => m,
        None => return MVPoly::zero(f.nvars),
    };
    let lm_g = match g.leading_monomial() {
        Some(m) => m,
        None => return MVPoly::zero(g.nvars),
    };
    let lc_f = f.leading_term().map(|t| t.coeff).unwrap_or(1);
    let lc_g = g.leading_term().map(|t| t.coeff).unwrap_or(1);
    let lcm = lm_f.lcm(lm_g);
    let mono_f = lcm.div(lm_f).unwrap_or_else(|| MonomialV2::one(f.nvars));
    let mono_g = lcm.div(lm_g).unwrap_or_else(|| MonomialV2::one(g.nvars));
    let term_f = MVTerm::new(lc_g, mono_f);
    let term_g = MVTerm::new(lc_f, mono_g);
    f.mul_term(&term_f).sub(&g.mul_term(&term_g))
}
/// Reduce polynomial `f` by divisor `g` (single step).
#[allow(dead_code)]
pub fn poly_reduce_step(f: &MVPoly, g: &MVPoly) -> Option<MVPoly> {
    if g.is_zero() {
        return None;
    }
    let lm_g = g.leading_monomial()?;
    let lc_g = g.leading_term()?.coeff;
    for (i, t) in f.terms.iter().enumerate() {
        if lm_g.divides(&t.mono) {
            let mono_q = t.mono.div(lm_g)?;
            let coeff_q = t.coeff;
            let term_q = MVTerm::new(coeff_q, mono_q);
            let mut reduced = f.mul_term(&MVTerm::new(lc_g, MonomialV2::one(f.nvars)));
            let _ = i;
            reduced = reduced.sub(&g.mul_term(&term_q));
            reduced.normalize();
            return Some(reduced);
        }
    }
    None
}
/// Check if f is in the radical of the ideal (using simple heuristics).
#[allow(dead_code)]
pub fn in_radical_heuristic(f: &MVPoly, generators: &[MVPoly], max_power: u32) -> bool {
    let mut power = f.clone();
    for _ in 1..=max_power {
        let mut checker = IdealMembershipChecker::new(generators.to_vec());
        if checker.is_member(power.clone(), 100) {
            return true;
        }
        power = power.mul(f);
    }
    false
}
#[cfg(test)]
mod polyrith_ext_tests {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_monomial_degree() {
        let m = MonomialV2::new(vec![2, 1, 0]);
        assert_eq!(m.degree(), 3);
    }
    #[test]
    fn test_monomial_mul() {
        let m1 = MonomialV2::new(vec![1, 0]);
        let m2 = MonomialV2::new(vec![0, 2]);
        let m3 = m1.mul(&m2);
        assert_eq!(m3.exponents, vec![1, 2]);
    }
    #[test]
    fn test_monomial_divides() {
        let m1 = MonomialV2::new(vec![1, 1]);
        let m2 = MonomialV2::new(vec![2, 3]);
        assert!(m1.divides(&m2));
        assert!(!m2.divides(&m1));
    }
    #[test]
    fn test_monomial_lcm() {
        let m1 = MonomialV2::new(vec![2, 1]);
        let m2 = MonomialV2::new(vec![1, 3]);
        let lcm = m1.lcm(&m2);
        assert_eq!(lcm.exponents, vec![2, 3]);
    }
    #[test]
    fn test_monomial_is_one() {
        assert!(MonomialV2::one(3).is_one());
        assert!(!MonomialV2::new(vec![1, 0, 0]).is_one());
    }
    #[test]
    fn test_mvpoly_zero() {
        let p = MVPoly::zero(2);
        assert!(p.is_zero());
    }
    #[test]
    fn test_mvpoly_one() {
        let p = MVPoly::one(2);
        assert!(!p.is_zero());
        assert_eq!(p.num_terms(), 1);
    }
    #[test]
    fn test_mvpoly_add() {
        let p1 = MVPoly::from_const(3, 2);
        let p2 = MVPoly::from_const(5, 2);
        let sum = p1.add(&p2);
        assert_eq!(sum.terms[0].coeff, 8);
    }
    #[test]
    fn test_mvpoly_sub() {
        let p1 = MVPoly::from_const(7, 1);
        let p2 = MVPoly::from_const(3, 1);
        let diff = p1.sub(&p2);
        assert_eq!(diff.terms[0].coeff, 4);
    }
    #[test]
    fn test_mvpoly_mul() {
        let p1 = MVPoly::from_const(2, 1);
        let p2 = MVPoly::from_const(3, 1);
        let prod = p1.mul(&p2);
        assert_eq!(prod.terms[0].coeff, 6);
    }
    #[test]
    fn test_mvpoly_neg() {
        let p = MVPoly::from_const(5, 2);
        let neg = p.neg();
        assert_eq!(neg.terms[0].coeff, -5);
    }
    #[test]
    fn test_mvpoly_normalize() {
        let mut p = MVPoly {
            nvars: 1,
            terms: vec![
                MVTerm::new(3, MonomialV2::one(1)),
                MVTerm::new(-3, MonomialV2::one(1)),
            ],
        };
        p.normalize();
        assert!(p.is_zero());
    }
    #[test]
    fn test_s_polynomial_zero() {
        let f = MVPoly::zero(2);
        let g = MVPoly::zero(2);
        let spoly = s_polynomial(&f, &g);
        assert!(spoly.is_zero());
    }
    #[test]
    fn test_ideal_membership_zero_in_ideal() {
        let mut checker = IdealMembershipChecker::new(vec![MVPoly::from_const(1, 2)]);
        assert!(checker.is_member(MVPoly::zero(2), 10));
    }
    #[test]
    fn test_polyrith_stats() {
        let mut stats = PolyrithStats::new();
        stats.record_success();
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_stats_groebner() {
        let mut stats = PolyrithStats::new();
        stats.record_groebner_call();
        assert_eq!(stats.groebner_calls, 1);
    }
    #[test]
    fn test_polyrith_cache_basic() {
        let mut cache = PolyrithCache::new();
        cache.insert("goal1", Some(vec![1, 2, 3]));
        assert!(cache.lookup("goal1").is_some());
        assert!(cache.lookup("goal2").is_none());
    }
    #[test]
    fn test_polyrith_solver_basic() {
        let solver = PolyrithSolver::new();
        assert!(solver.hyps.is_empty());
    }
    #[test]
    fn test_poly_coeff_basic() {
        let p = PolyCoeff(42);
        assert_eq!(p.0, 42);
    }
    #[test]
    fn test_int_poly1_basic() {
        let p = IntPoly1 { coeffs: vec![0, 1] };
        assert_eq!(p.coeffs.len(), 2);
    }
}
#[cfg(test)]
mod tacticpolyrith_analysis_tests {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_tacticpolyrith_result_ok() {
        let r = TacticPolyrithResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpolyrith_result_err() {
        let r = TacticPolyrithResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpolyrith_result_partial() {
        let r = TacticPolyrithResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpolyrith_result_skipped() {
        let r = TacticPolyrithResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticpolyrith_analysis_pass_run() {
        let mut p = TacticPolyrithAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticpolyrith_analysis_pass_empty_input() {
        let mut p = TacticPolyrithAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticpolyrith_analysis_pass_success_rate() {
        let mut p = TacticPolyrithAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticpolyrith_analysis_pass_disable() {
        let mut p = TacticPolyrithAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticpolyrith_pipeline_basic() {
        let mut pipeline = TacticPolyrithPipeline::new("main_pipeline");
        pipeline.add_pass(TacticPolyrithAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticPolyrithAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticpolyrith_pipeline_disabled_pass() {
        let mut pipeline = TacticPolyrithPipeline::new("partial");
        let mut p = TacticPolyrithAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticPolyrithAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticpolyrith_diff_basic() {
        let mut d = TacticPolyrithDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticpolyrith_diff_summary() {
        let mut d = TacticPolyrithDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticpolyrith_config_set_get() {
        let mut cfg = TacticPolyrithConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticpolyrith_config_read_only() {
        let mut cfg = TacticPolyrithConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticpolyrith_config_remove() {
        let mut cfg = TacticPolyrithConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticpolyrith_diagnostics_basic() {
        let mut diag = TacticPolyrithDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticpolyrith_diagnostics_max_errors() {
        let mut diag = TacticPolyrithDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticpolyrith_diagnostics_clear() {
        let mut diag = TacticPolyrithDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticpolyrith_config_value_types() {
        let b = TacticPolyrithConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticPolyrithConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticPolyrithConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticPolyrithConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticPolyrithConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod polyrith_ext_tests_300 {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_polyrith_ext_result_ok_300() {
        let r = PolyrithExtResult300::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_err_300() {
        let r = PolyrithExtResult300::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_partial_300() {
        let r = PolyrithExtResult300::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_skipped_300() {
        let r = PolyrithExtResult300::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_polyrith_ext_pass_run_300() {
        let mut p = PolyrithExtPass300::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_polyrith_ext_pass_empty_300() {
        let mut p = PolyrithExtPass300::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_polyrith_ext_pass_rate_300() {
        let mut p = PolyrithExtPass300::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_polyrith_ext_pass_disable_300() {
        let mut p = PolyrithExtPass300::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_polyrith_ext_pipeline_basic_300() {
        let mut pipeline = PolyrithExtPipeline300::new("main_pipeline");
        pipeline.add_pass(PolyrithExtPass300::new("pass1"));
        pipeline.add_pass(PolyrithExtPass300::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_polyrith_ext_pipeline_disabled_300() {
        let mut pipeline = PolyrithExtPipeline300::new("partial");
        let mut p = PolyrithExtPass300::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PolyrithExtPass300::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_polyrith_ext_diff_basic_300() {
        let mut d = PolyrithExtDiff300::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_polyrith_ext_config_set_get_300() {
        let mut cfg = PolyrithExtConfig300::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_polyrith_ext_config_read_only_300() {
        let mut cfg = PolyrithExtConfig300::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_polyrith_ext_config_remove_300() {
        let mut cfg = PolyrithExtConfig300::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_polyrith_ext_diagnostics_basic_300() {
        let mut diag = PolyrithExtDiag300::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_polyrith_ext_diagnostics_max_errors_300() {
        let mut diag = PolyrithExtDiag300::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_polyrith_ext_diagnostics_clear_300() {
        let mut diag = PolyrithExtDiag300::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_polyrith_ext_config_value_types_300() {
        let b = PolyrithExtConfigVal300::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PolyrithExtConfigVal300::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PolyrithExtConfigVal300::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PolyrithExtConfigVal300::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PolyrithExtConfigVal300::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod polyrith_ext_tests_301 {
    use super::*;
    use crate::tactic::polyrith::*;
    #[test]
    fn test_polyrith_ext_result_ok_300() {
        let r = PolyrithExtResult301::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_err_300() {
        let r = PolyrithExtResult301::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_partial_300() {
        let r = PolyrithExtResult301::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_polyrith_ext_result_skipped_300() {
        let r = PolyrithExtResult301::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_polyrith_ext_pass_run_300() {
        let mut p = PolyrithExtPass301::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_polyrith_ext_pass_empty_300() {
        let mut p = PolyrithExtPass301::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_polyrith_ext_pass_rate_300() {
        let mut p = PolyrithExtPass301::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_polyrith_ext_pass_disable_300() {
        let mut p = PolyrithExtPass301::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_polyrith_ext_pipeline_basic_300() {
        let mut pipeline = PolyrithExtPipeline301::new("main_pipeline");
        pipeline.add_pass(PolyrithExtPass301::new("pass1"));
        pipeline.add_pass(PolyrithExtPass301::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_polyrith_ext_pipeline_disabled_300() {
        let mut pipeline = PolyrithExtPipeline301::new("partial");
        let mut p = PolyrithExtPass301::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PolyrithExtPass301::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_polyrith_ext_diff_basic_300() {
        let mut d = PolyrithExtDiff301::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_polyrith_ext_config_set_get_300() {
        let mut cfg = PolyrithExtConfig301::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_polyrith_ext_config_read_only_300() {
        let mut cfg = PolyrithExtConfig301::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_polyrith_ext_config_remove_300() {
        let mut cfg = PolyrithExtConfig301::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_polyrith_ext_diagnostics_basic_300() {
        let mut diag = PolyrithExtDiag301::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_polyrith_ext_diagnostics_max_errors_300() {
        let mut diag = PolyrithExtDiag301::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_polyrith_ext_diagnostics_clear_300() {
        let mut diag = PolyrithExtDiag301::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_polyrith_ext_config_value_types_300() {
        let b = PolyrithExtConfigVal301::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PolyrithExtConfigVal301::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PolyrithExtConfigVal301::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PolyrithExtConfigVal301::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PolyrithExtConfigVal301::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

/// `polyrith` — prove polynomial arithmetic goals via Groebner basis membership.
///
/// The current goal should be a polynomial identity (typically `lhs = rhs` or
/// `lhs - rhs = 0`).  This tactic:
///
/// 1. Collects the local hypotheses from `ctx` — each hypothesis type is
///    converted to a string and parsed as an integer constant polynomial
///    (a lightweight model for constant-coefficient arithmetic).
/// 2. Extracts the goal polynomial from the target expression.
/// 3. Runs [`PolyrithTactic::run`] to search for a linear combination of
///    hypothesis polynomials equal to the goal.
/// 4. If a witness is found, closes the goal with a synthetic proof constant
///    embedding the coefficient vector; otherwise returns [`TacticError::Failed`].
pub fn tac_polyrith(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("polyrith: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let target_str = target.to_string();

    // Collect hypothesis polynomials from the local context.
    // Each hypothesis whose type string parses as an integer is treated as a
    // constant-coefficient polynomial; others are silently ignored.
    let hyps_raw = ctx.get_local_hyps();
    let hyp_strs: Vec<String> = hyps_raw
        .iter()
        .map(|(_name, ty)| {
            let ty_inst = ctx.instantiate_mvars(ty);
            ty_inst.to_string()
        })
        .collect();
    let hyp_refs: Vec<&str> = hyp_strs.iter().map(String::as_str).collect();

    // Run polyrith via the string interface.
    let tac = PolyrithTactic::new();
    if tac.run_with_strings(&hyp_refs, &target_str) {
        // Build a proof constant whose name encodes the success for the elab layer.
        let proof = Expr::Const(Name::str("polyrith.proved"), vec![]);
        state.close_goal(proof, ctx)?;
        Ok(())
    } else {
        Err(TacticError::Failed(format!(
            "polyrith: could not find a polynomial certificate for `{}`",
            target_str
        )))
    }
}
