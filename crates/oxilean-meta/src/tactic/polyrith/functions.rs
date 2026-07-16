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
use crate::tactic::certificate::{PolyrithCert, PolyrithCertEntry, ProofCertificate};
use crate::tactic::linear_combination::Rat;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Literal, Name};

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

// ---------------------------------------------------------------------------
// Expr → Polynomial parsing (mirrors omega's expr_to_linear pattern)
// ---------------------------------------------------------------------------

/// Map the head constant name of a (possibly partially-applied) expression.
fn poly_get_app_head_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => poly_get_app_head_name(f),
        _ => None,
    }
}

/// Return a canonical binary-op tag ("add" | "sub" | "mul") for known arithmetic
/// constants, mirroring the set in omega/functions.rs.
fn poly_extract_binary_op(expr: &Expr) -> Option<&'static str> {
    let head = poly_get_app_head_name(expr)?;
    match head.to_string().as_str() {
        "HAdd.hAdd" | "Nat.add" | "Int.add" | "Add.add" => Some("add"),
        "HSub.hSub" | "Nat.sub" | "Int.sub" | "Sub.sub" => Some("sub"),
        "HMul.hMul" | "Nat.mul" | "Int.mul" | "Mul.mul" => Some("mul"),
        _ => None,
    }
}

/// A map from `Expr` variable keys to canonical variable name strings.
///
/// Used so that `FVar(id)` and `Const(name)` consistently resolve to the
/// same polynomial variable names within a single `tac_polyrith` call.
type VarMap = std::collections::BTreeMap<String, String>;

/// Parse an expression into a multivariate `Polynomial` with string variable
/// names.
///
/// Recognises:
/// - Integer / Nat literals   → constant polynomial
/// - `Const(name)` / `FVar`  → degree-1 variable monomial
/// - `Int.add` / `Int.sub`   → polynomial addition / subtraction
/// - `Int.mul`               → full polynomial multiplication (non-linear ok)
/// - `Int.neg` / `Neg.neg`  → polynomial negation
/// - `Int.ofNat`              → strip the cast, recurse
///
/// Returns `None` for any unrecognised pattern.
pub fn parse_expr_to_polynomial(expr: &Expr, _var_map: &mut VarMap) -> Option<Polynomial> {
    match expr {
        // Integer and natural number literals.
        Expr::Lit(Literal::Nat(n)) => {
            let v = n.to_u64()? as i64;
            let mut p = Polynomial::new();
            p.add_term(Monomial::new(v));
            Some(p)
        }
        // A free variable reference.
        Expr::FVar(id) => {
            let var_name = format!("fvar_{}", id.0);
            let mut m = Monomial::new(1);
            m.add_var(&var_name, 1);
            let mut p = Polynomial::new();
            p.add_term(m);
            Some(p)
        }
        // A bound variable reference.
        Expr::BVar(i) => {
            let var_name = format!("bvar_{i}");
            let mut m = Monomial::new(1);
            m.add_var(&var_name, 1);
            let mut p = Polynomial::new();
            p.add_term(m);
            Some(p)
        }
        // A named constant treated as a polynomial variable.
        Expr::Const(name, _) => {
            let name_str = name.to_string();
            // Reject clearly non-arithmetic constants.
            if name_str.contains('.') && !name_str.starts_with("polyrith") {
                // Could be a type former like `Int`, `Nat`; treat as unknown.
                return None;
            }
            let mut m = Monomial::new(1);
            m.add_var(&name_str, 1);
            let mut p = Polynomial::new();
            p.add_term(m);
            Some(p)
        }
        // Application: could be a unary or binary operation.
        Expr::App(func, arg) => {
            // Unary: negation and casts.
            if let Expr::Const(ref cname, _) = **func {
                match cname.to_string().as_str() {
                    "Neg.neg" | "Int.neg" => {
                        let inner = parse_expr_to_polynomial(arg, _var_map)?;
                        return Some(inner.negate());
                    }
                    "Int.ofNat" | "Nat.cast" | "Int.ofNatLit" => {
                        return parse_expr_to_polynomial(arg, _var_map);
                    }
                    _ => {}
                }
            }
            // Binary: App(App(op, lhs), rhs).
            if let Expr::App(inner_func, lhs_expr) = func.as_ref() {
                if let Some(op) = poly_extract_binary_op(inner_func) {
                    let lhs = parse_expr_to_polynomial(lhs_expr, _var_map)?;
                    let rhs = parse_expr_to_polynomial(arg, _var_map)?;
                    return match op {
                        "add" => Some(Polynomial::add(&lhs, &rhs)),
                        "sub" => {
                            let neg_rhs = rhs.negate();
                            Some(Polynomial::add(&lhs, &neg_rhs))
                        }
                        "mul" => Some(Polynomial::mul(&lhs, &rhs)),
                        _ => None,
                    };
                }
                // Handle 4-arg typeclass form: App(App(App(App(op, inst), ty), lhs), rhs).
                if let Expr::App(inner2, lhs2) = inner_func.as_ref() {
                    if let Some(op) = poly_extract_binary_op(inner2) {
                        let lhs = parse_expr_to_polynomial(lhs2, _var_map)?;
                        let rhs = parse_expr_to_polynomial(arg, _var_map)?;
                        return match op {
                            "add" => Some(Polynomial::add(&lhs, &rhs)),
                            "sub" => {
                                let neg_rhs = rhs.negate();
                                Some(Polynomial::add(&lhs, &neg_rhs))
                            }
                            "mul" => Some(Polynomial::mul(&lhs, &rhs)),
                            _ => None,
                        };
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// Try to parse an `Expr` representing an equality `lhs = rhs` into a
/// polynomial `lhs - rhs`.  Returns `None` if the expression is not an
/// equality or cannot be parsed as a polynomial.
pub fn parse_equality_to_poly(expr: &Expr, var_map: &mut VarMap) -> Option<Polynomial> {
    // Equalities arrive as App(App(App(Eq, ty), lhs), rhs)
    // or App(App(op, lhs), rhs) where op encodes `=`.
    if let Expr::App(func, rhs_expr) = expr {
        // Try the 3-arg Eq form.
        if let Expr::App(func2, lhs_expr) = func.as_ref() {
            // Check for App(App(Eq, _ty), lhs)
            if let Expr::App(eq_head, _ty) = func2.as_ref() {
                if let Some(head) = poly_get_app_head_name(eq_head) {
                    if head.to_string() == "Eq" {
                        let lhs = parse_expr_to_polynomial(lhs_expr, var_map)?;
                        let rhs = parse_expr_to_polynomial(rhs_expr, var_map)?;
                        let neg_rhs = rhs.negate();
                        return Some(Polynomial::add(&lhs, &neg_rhs));
                    }
                }
            }
            // Try 2-arg form (e.g. some backends emit App(App(eq, lhs), rhs)).
            if let Some(head) = poly_get_app_head_name(func2) {
                if head.to_string() == "Eq" {
                    let lhs = parse_expr_to_polynomial(lhs_expr, var_map)?;
                    let rhs = parse_expr_to_polynomial(rhs_expr, var_map)?;
                    let neg_rhs = rhs.negate();
                    return Some(Polynomial::add(&lhs, &neg_rhs));
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// OxiZ-math bridge: polynomial conversion and independent Gröbner validation
// ---------------------------------------------------------------------------

/// Build a shared variable-index map from all string variable names in a set
/// of in-house `Polynomial` values.  The in-house representation uses `String`
/// variable names; `oxiz_math` polynomials use `u32` indices.
fn build_var_map_for_oxiz(polys: &[&Polynomial]) -> std::collections::HashMap<String, u32> {
    let mut map: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut next_id: u32 = 0;
    for poly in polys {
        for term in &poly.terms {
            for (var_name, _exp) in &term.vars {
                if !map.contains_key(var_name.as_str()) {
                    map.insert(var_name.clone(), next_id);
                    next_id += 1;
                }
            }
        }
    }
    map
}

/// Convert an in-house `Polynomial` (integer coefficients, named string
/// variables) into an `oxiz_math::polynomial::Polynomial` (BigRational
/// coefficients, `u32` variable indices).
///
/// Returns `None` only if the variable map is missing a name that appears in
/// the polynomial (which should never happen if `build_var_map_for_oxiz` was
/// called on a collection that includes this polynomial).
fn to_oxiz_poly(
    p: &Polynomial,
    var_map: &std::collections::HashMap<String, u32>,
) -> Option<oxiz_math::polynomial::Polynomial> {
    use num_bigint::BigInt;
    use num_rational::BigRational;
    use oxiz_math::polynomial::{Monomial as OxizMonomial, Term as OxizTerm};

    let mut oxiz_terms: Vec<OxizTerm> = Vec::with_capacity(p.terms.len());

    for monomial in &p.terms {
        // Convert each (name, exponent) pair to (u32 index, exponent).
        let mut powers: Vec<(u32, u32)> = Vec::with_capacity(monomial.vars.len());
        for (var_name, exp) in &monomial.vars {
            let idx = var_map.get(var_name.as_str())?;
            powers.push((*idx, *exp));
        }

        let coeff = BigRational::from_integer(BigInt::from(monomial.coefficient));
        let oxiz_mono = OxizMonomial::from_powers(powers);
        oxiz_terms.push(OxizTerm::new(coeff, oxiz_mono));
    }

    Some(oxiz_math::polynomial::Polynomial::from_terms(
        oxiz_terms,
        oxiz_math::polynomial::MonomialOrder::default(),
    ))
}

/// Use `oxiz_math`'s Buchberger implementation to independently verify that
/// `goal` is in the polynomial ideal generated by `generators`.
///
/// Returns:
/// - `Some(true)` if OxiZ-math's Buchberger confirms ideal membership,
/// - `Some(false)` if it denies membership,
/// - `None` if any conversion step fails (caller should fall back to the
///   in-house check; returning `None` does NOT indicate an error that should
///   prevent the tactic from succeeding).
pub(crate) fn oxiz_validate_ideal_membership(
    goal: &Polynomial,
    generators: &[Polynomial],
) -> Option<bool> {
    // Gather all polynomials to build the variable index map in one pass.
    let all_polys: Vec<&Polynomial> = std::iter::once(goal).chain(generators.iter()).collect();
    let var_map = build_var_map_for_oxiz(&all_polys);

    let goal_oxiz = to_oxiz_poly(goal, &var_map)?;

    let gens_oxiz: Option<Vec<oxiz_math::polynomial::Polynomial>> = generators
        .iter()
        .map(|g| to_oxiz_poly(g, &var_map))
        .collect();
    let gens_oxiz = gens_oxiz?;

    // `ideal_membership` internally calls `grobner_basis` then reduces `goal`
    // with respect to the resulting basis.  Zero remainder ⟺ membership.
    Some(oxiz_math::grobner::ideal_membership(&goal_oxiz, &gens_oxiz))
}

/// `polyrith` — prove polynomial arithmetic goals via Gröbner basis membership.
///
/// Algorithm:
///
/// 1. Parse the goal `lhs = rhs` into the polynomial `goal_poly = lhs - rhs`.
/// 2. Parse each local hypothesis that is an equality `p = q` into
///    `p - q`, collecting a list of *generator polynomials* for the ideal.
///    Hypotheses that cannot be parsed are silently skipped.
/// 3. Build a `GroebnerBasis` from the generators and call `contains(&goal_poly)`.
///    This reduces `goal_poly` by the basis using the polynomial-division
///    algorithm (Cox–Little–O'Shea §2.3): if the remainder is zero, the goal
///    is in the ideal and is therefore provable.
/// 4. If membership holds, close the goal with a placeholder proof constant
///    (`polyrith.proved`).  Kernel-verified proof-term reconstruction is deferred
///    to a future cycle.
/// 5. If membership does not hold (or parsing fails for the goal), return
///    `TacticError::Failed`.
pub fn tac_polyrith(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("polyrith: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);

    let mut var_map: VarMap = VarMap::new();

    // Parse the goal expression into a polynomial.
    let goal_poly = parse_equality_to_poly(&target, &mut var_map).ok_or_else(|| {
        TacticError::Failed(format!(
            "polyrith: could not parse goal as a polynomial equality: `{}`",
            target
        ))
    })?;

    // Collect and parse hypothesis polynomials.  Each local hypothesis whose
    // type is a parseable polynomial equality contributes one generator to the
    // ideal.  Non-polynomial hypotheses (e.g. `n : Nat`, propositions, etc.)
    // are silently skipped — they don't contribute generators.
    let hyps_raw = ctx.get_local_hyps();
    let mut generators: Vec<Polynomial> = Vec::new();
    for (_name, ty) in &hyps_raw {
        let ty_inst = ctx.instantiate_mvars(ty);
        if let Some(p) = parse_equality_to_poly(&ty_inst, &mut var_map) {
            generators.push(p);
        }
    }

    // Build a Gröbner basis from the hypothesis generators and test membership.
    let mut basis = GroebnerBasis::new();
    for gen in generators {
        basis.add_polynomial(gen);
    }

    if basis.contains(&goal_poly) {
        // Build a PolyrithCert from the hypothesis generators that contributed.
        // We generate one entry per parsed hypothesis (constraint_index = position in hyps_raw).
        let entries: Vec<PolyrithCertEntry> = hyps_raw
            .iter()
            .enumerate()
            .filter_map(|(idx, (_name, ty))| {
                let ty_inst = ctx.instantiate_mvars(ty);
                let mut vm = VarMap::new();
                parse_equality_to_poly(&ty_inst, &mut vm).map(|_| PolyrithCertEntry {
                    constraint_index: idx,
                    coeff: Rat { numer: 1, denom: 1 },
                })
            })
            .collect();

        // --- OxiZ-math independent validation ---
        // Run Buchberger on the same generators and check ideal membership.
        // Collect the generators again (they were moved into `basis` above).
        let generators_for_oxiz: Vec<Polynomial> = hyps_raw
            .iter()
            .filter_map(|(_name, ty)| {
                let ty_inst = ctx.instantiate_mvars(ty);
                let mut vm = VarMap::new();
                parse_equality_to_poly(&ty_inst, &mut vm)
            })
            .collect();

        let oxiz_validated =
            oxiz_validate_ideal_membership(&goal_poly, &generators_for_oxiz).unwrap_or(false);

        let cert = PolyrithCert {
            goal: format!("{}", target),
            entries,
            validated: oxiz_validated,
        };

        // Store the certificate for elab-side proof reconstruction.
        ctx.last_polyrith_cert = Some(cert.clone());
        ctx.last_certificate = Some(ProofCertificate::Polyrith(cert));

        // Close the goal with a placeholder proof term.
        // Kernel proof reconstruction is attempted by the elaborator.
        let proof = Expr::Const(Name::str("polyrith.proved"), vec![]);
        state.close_goal(proof, ctx)?;
        Ok(())
    } else {
        Err(TacticError::Failed(format!(
            "polyrith: goal `{}` is not in the ideal generated by the hypotheses",
            target
        )))
    }
}

// ---------------------------------------------------------------------------
// Tests for the real Gröbner-basis algorithm
// ---------------------------------------------------------------------------

#[cfg(test)]
mod groebner_real_algorithm_tests {
    use super::*;
    use crate::tactic::polyrith::*;

    /// Build a polynomial with named variables for testing the real algorithm.
    ///
    /// `terms` is a list of `(coeff, &[(var_name, exponent)])` pairs.
    fn make_poly(terms: &[(i64, &[(&str, u32)])]) -> Polynomial {
        let mut p = Polynomial::new();
        for &(coeff, vars) in terms {
            let mut m = Monomial::new(coeff);
            for &(v, e) in vars {
                m.add_var(v, e);
            }
            p.add_term(m);
        }
        p
    }

    /// Test: x + y - 3 is in the ideal <x - 1, y - 2>.
    ///
    /// This is the primary real-world polyrith test case: the goal `x + y = 3`
    /// with hypotheses `x = 1` and `y = 2`.  The parse step produces constant-
    /// offset generators `{x-1, y-2}` and goal polynomial `x+y-3`.
    ///
    /// The real Gröbner reduction succeeds:
    ///   1. Leading term of `x+y-3` is `x`; generator `x-1` divides it
    ///      (quotient = const(1)); subtract `x-1`; remainder = `y-2`.
    ///   2. Leading term of `y-2` is `y`; generator `y-2` divides it
    ///      (quotient = const(1)); subtract `y-2`; remainder = 0.
    ///
    /// The old brute-force stub would attempt this only via `PolyrithTactic::run`
    /// (constant-only coefficient search) and `run_with_strings` (parsing each
    /// hypothesis as an i64 constant, losing variable structure entirely).
    #[test]
    fn test_polyrith_ideal_membership_simple() {
        // h1: x - 1 = 0  (represents "x = 1")
        let h1 = make_poly(&[(1, &[("x", 1)]), (-1, &[])]);
        // h2: y - 2 = 0  (represents "y = 2")
        let h2 = make_poly(&[(1, &[("y", 1)]), (-2, &[])]);
        // goal: x + y - 3 = 0  (represents "x + y = 3")
        let goal = make_poly(&[(1, &[("x", 1)]), (1, &[("y", 1)]), (-3, &[])]);

        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(h1);
        basis.add_polynomial(h2);

        assert!(
            basis.contains(&goal),
            "x+y-3 should be in the ideal <x-1, y-2>"
        );
    }

    /// Test: y - 1 is NOT in the ideal <x>.
    ///
    /// This models an unprovable goal `y = 1` given only the hypothesis `x = 0`.
    /// No polynomial multiple of x can produce `y - 1`.
    #[test]
    fn test_polyrith_not_member() {
        // Generator: x (represents hypothesis "x = 0")
        let h1 = make_poly(&[(1, &[("x", 1)])]);
        // Goal: y - 1 (represents "y = 1", unprovable from x=0)
        let goal = make_poly(&[(1, &[("y", 1)]), (-1, &[])]);

        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(h1);

        assert!(!basis.contains(&goal), "y-1 should NOT be in the ideal <x>");
    }

    /// Test: x² - 1 is in the ideal <x² - 1>.
    ///
    /// This models the goal `(x-1)*(x+1) = 0` given the hypothesis `x^2 = 1`
    /// (since `(x-1)*(x+1) = x²-1`).  The goal polynomial is degree-2 with a
    /// constant offset — the old brute-force stub treated every polynomial as a
    /// single i64 constant, losing ALL variable structure, so it could never
    /// recognise this as a valid ideal membership certificate.
    #[test]
    fn test_polyrith_quadratic_identity() {
        // h1: x² - 1 = 0  (represents "x² = 1")
        let h1 = make_poly(&[(1, &[("x", 2)]), (-1, &[])]);
        // goal: x² - 1 = 0  (same polynomial — trivially in the ideal)
        let goal = make_poly(&[(1, &[("x", 2)]), (-1, &[])]);

        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(h1);

        assert!(basis.contains(&goal), "x²-1 should be in the ideal <x²-1>");
    }

    /// Test: 2x + 3y - 8 is in the ideal <x - 1, y - 2>.
    ///
    /// The witness is `2*(x-1) + 3*(y-2) = 2x+3y-8`.  Coefficient 3 is outside
    /// the brute-force search range `{-2,-1,0,1,2}`, so the old stub would
    /// FAIL to find the witness even if it could handle variable terms.
    /// The real Gröbner reduction:
    ///   1. Lead `2x`; divisor `x-1`, quotient `const(2)`;
    ///      subtract `2*(x-1) = 2x-2`; rest = `3y-6`.
    ///   2. Lead `3y`; divisor `y-2`, quotient `const(3)`;
    ///      subtract `3*(y-2) = 3y-6`; rest = 0.
    #[test]
    fn test_polyrith_linear_combination_outside_brute_force_range() {
        let h1 = make_poly(&[(1, &[("x", 1)]), (-1, &[])]);
        let h2 = make_poly(&[(1, &[("y", 1)]), (-2, &[])]);
        // 2x + 3y - 8
        let goal = make_poly(&[(2, &[("x", 1)]), (3, &[("y", 1)]), (-8, &[])]);

        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(h1);
        basis.add_polynomial(h2);

        assert!(
            basis.contains(&goal),
            "2x+3y-8 should be in the ideal <x-1, y-2>"
        );
    }

    /// Test parse_expr_to_polynomial with a literal.
    #[test]
    fn test_parse_expr_literal() {
        let expr = Expr::Lit(Literal::nat(5));
        let mut var_map = VarMap::new();
        let p = parse_expr_to_polynomial(&expr, &mut var_map);
        assert!(p.is_some(), "literal 5 should parse");
        let p = p.unwrap();
        assert_eq!(p.terms.len(), 1);
        assert_eq!(p.terms[0].coefficient, 5);
        assert!(p.terms[0].vars.is_empty());
    }

    // -----------------------------------------------------------------------
    // OxiZ-math bridge tests
    // -----------------------------------------------------------------------

    /// `to_oxiz_poly` should convert a simple polynomial 2*x + 3 without loss.
    #[test]
    fn test_to_oxiz_poly_conversion() {
        // Build 2*x + 3 using the in-house Polynomial type.
        let mut p = Polynomial::new();
        let mut m_x = Monomial::new(2);
        m_x.add_var("x", 1);
        p.add_term(m_x);
        p.add_term(Monomial::new(3));

        let var_map = build_var_map_for_oxiz(&[&p]);
        let oxiz_p = to_oxiz_poly(&p, &var_map).expect("conversion should succeed for 2*x + 3");

        // The resulting polynomial must be non-zero.
        assert!(
            !oxiz_p.is_zero(),
            "oxiz polynomial for 2*x+3 must not be zero"
        );
        // It should have exactly 2 terms (2*x and 3).
        assert_eq!(
            oxiz_p.num_terms(),
            2,
            "oxiz polynomial for 2*x+3 should have 2 terms"
        );
    }

    /// A zero polynomial should convert to a zero polynomial.
    #[test]
    fn test_to_oxiz_poly_zero_conversion() {
        let p = Polynomial::zero();
        let var_map = build_var_map_for_oxiz(&[&p]);
        let oxiz_p = to_oxiz_poly(&p, &var_map).expect("zero polynomial conversion should succeed");
        assert!(oxiz_p.is_zero(), "zero polynomial should map to zero");
    }

    /// OxiZ must confirm that `x - 1` is in the ideal generated by `[x - 1]`.
    #[test]
    fn test_oxiz_validate_ideal_membership_true() {
        // goal = x - 1
        let mut poly_x_minus_1 = Polynomial::new();
        let mut m_x = Monomial::new(1);
        m_x.add_var("x", 1);
        poly_x_minus_1.add_term(m_x);
        poly_x_minus_1.add_term(Monomial::new(-1));

        let result = oxiz_validate_ideal_membership(&poly_x_minus_1, &[poly_x_minus_1.clone()]);
        assert_eq!(result, Some(true), "x-1 should be in the ideal <x-1>");
    }

    /// OxiZ must deny that `x² - 2` is in the ideal generated by `[x - 1]`.
    #[test]
    fn test_oxiz_validate_ideal_membership_false() {
        // goal = x² - 2
        let mut x_sq_minus_2 = Polynomial::new();
        let mut m_x2 = Monomial::new(1);
        m_x2.add_var("x", 2);
        x_sq_minus_2.add_term(m_x2);
        x_sq_minus_2.add_term(Monomial::new(-2));

        // generator = x - 1
        let mut x_minus_1 = Polynomial::new();
        let mut m_x = Monomial::new(1);
        m_x.add_var("x", 1);
        x_minus_1.add_term(m_x);
        x_minus_1.add_term(Monomial::new(-1));

        let result = oxiz_validate_ideal_membership(&x_sq_minus_2, &[x_minus_1]);
        assert_eq!(result, Some(false), "x²-2 should NOT be in the ideal <x-1>");
    }

    /// PolyrithCert::validated field should be accessible and default to false.
    #[test]
    fn test_polyrith_cert_has_validated_field() {
        use crate::tactic::certificate::{PolyrithCert, PolyrithCertEntry};
        use crate::tactic::linear_combination::Rat;
        let cert = PolyrithCert {
            goal: "test".to_string(),
            entries: vec![],
            validated: false,
        };
        assert!(!cert.validated, "validated should be false by default");

        let cert_validated = PolyrithCert {
            goal: "test".to_string(),
            entries: vec![PolyrithCertEntry {
                constraint_index: 0,
                coeff: Rat { numer: 1, denom: 1 },
            }],
            validated: true,
        };
        assert!(
            cert_validated.validated,
            "validated should be true when set"
        );
    }

    /// OxiZ must confirm that `2x + 3y` is in the ideal generated by `[x, y]`.
    ///
    /// The witness is: `2·x + 3·y = 2*(x) + 3*(y)`, a direct linear combination
    /// of the generators.  Buchberger should recognise this immediately because
    /// `x` and `y` reduce both leading terms to zero.
    #[test]
    fn test_oxiz_validates_linear_combination() {
        // goal = 2*x + 3*y
        let mut goal = Polynomial::new();
        let mut m_2x = Monomial::new(2);
        m_2x.add_var("x", 1);
        goal.add_term(m_2x);
        let mut m_3y = Monomial::new(3);
        m_3y.add_var("y", 1);
        goal.add_term(m_3y);

        // generator 1: x
        let mut gen_x = Polynomial::new();
        let mut m_x = Monomial::new(1);
        m_x.add_var("x", 1);
        gen_x.add_term(m_x);

        // generator 2: y
        let mut gen_y = Polynomial::new();
        let mut m_y = Monomial::new(1);
        m_y.add_var("y", 1);
        gen_y.add_term(m_y);

        let result = oxiz_validate_ideal_membership(&goal, &[gen_x, gen_y]);
        assert_eq!(result, Some(true), "2x+3y should be in the ideal <x, y>");
    }

    /// OxiZ must handle a multivariate quadratic: `x*y - 1` in `<x - 1, y - 1>`.
    ///
    /// Witness: `x*y - 1 = y*(x-1) + 1*(y-1)`.
    #[test]
    fn test_oxiz_validates_multivariate_quadratic() {
        // goal = x*y - 1
        let mut goal = Polynomial::new();
        let mut m_xy = Monomial::new(1);
        m_xy.add_var("x", 1);
        m_xy.add_var("y", 1);
        goal.add_term(m_xy);
        goal.add_term(Monomial::new(-1));

        // h1: x - 1
        let mut h1 = Polynomial::new();
        let mut m_x = Monomial::new(1);
        m_x.add_var("x", 1);
        h1.add_term(m_x);
        h1.add_term(Monomial::new(-1));

        // h2: y - 1
        let mut h2 = Polynomial::new();
        let mut m_y = Monomial::new(1);
        m_y.add_var("y", 1);
        h2.add_term(m_y);
        h2.add_term(Monomial::new(-1));

        let result = oxiz_validate_ideal_membership(&goal, &[h1, h2]);
        assert_eq!(
            result,
            Some(true),
            "x*y-1 should be in the ideal <x-1, y-1>"
        );
    }
}
