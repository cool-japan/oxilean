//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    FarkasCert, LinCombExpr, LinCombMap, LinCombTerm, LinearCombCache, LinearCombLogger,
    LinearCombPriorityQueue, LinearCombRegistry, LinearCombStats, LinearCombUtil0,
    LinearCombination, LinearCombinationExtConfig2900, LinearCombinationExtConfigVal2900,
    LinearCombinationExtDiag2900, LinearCombinationExtDiff2900, LinearCombinationExtPass2900,
    LinearCombinationExtPipeline2900, LinearCombinationExtResult2900, LinearCombinationTactic,
    LpSolveResult, Rat, RatMatrix, SimpleLp, TacticLinearCombinationAnalysisPass,
    TacticLinearCombinationConfig, TacticLinearCombinationConfigValue,
    TacticLinearCombinationDiagnostics, TacticLinearCombinationDiff,
    TacticLinearCombinationPipeline, TacticLinearCombinationResult, UniPoly,
};
use std::collections::HashMap;

/// Find the best integer coefficient for `hyp` to reduce `residual`.
///
/// Returns the coefficient `c` such that `c * hyp` cancels the most
/// variable terms in `residual`.
pub(super) fn find_best_coeff(residual: &LinCombExpr, hyp: &LinCombExpr) -> i64 {
    if hyp.terms.is_empty() {
        if hyp.constant != 0 && residual.constant != 0 && residual.constant % hyp.constant == 0 {
            return residual.constant / hyp.constant;
        }
        return 0;
    }
    for r_term in &residual.terms {
        if let Some(h_term) = hyp.terms.iter().find(|t| t.variable == r_term.variable) {
            if h_term.coefficient != 0 && r_term.coefficient % h_term.coefficient == 0 {
                return r_term.coefficient / h_term.coefficient;
            }
        }
    }
    0
}
/// Parse a simple linear expression string.
///
/// Supports forms like:
/// - `"3"` → constant 3
/// - `"x"` → 1*x
/// - `"2 * x"` → 2*x
/// - `"2 * x + 3 * y + 1"` → 2*x + 3*y + 1
/// - `"x + y"` → 1*x + 1*y
pub(super) fn parse_linear_expr(s: &str) -> Option<LinCombExpr> {
    let mut expr = LinCombExpr::new();
    for token in s.split('+') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some(star_pos) = token.find('*') {
            let coeff_str = token[..star_pos].trim();
            let var_str = token[star_pos + 1..].trim();
            let coeff: i64 = coeff_str.parse().ok()?;
            expr.add_term(coeff, var_str);
        } else if let Ok(c) = token.parse::<i64>() {
            expr.constant += c;
        } else {
            expr.add_term(1, token);
        }
    }
    Some(expr)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::linear_combination::*;
    fn make_expr(constant: i64, terms: &[(&str, i64)]) -> LinCombExpr {
        let mut e = LinCombExpr::new();
        e.constant = constant;
        for (var, coeff) in terms {
            e.add_term(*coeff, var);
        }
        e
    }
    #[test]
    fn test_lin_comb_expr_new_is_zero() {
        let e = LinCombExpr::new();
        assert!(e.is_zero());
    }
    #[test]
    fn test_lin_comb_expr_evaluate() {
        let mut e = LinCombExpr::new();
        e.add_term(2, "x");
        e.add_term(3, "y");
        e.set_constant(5);
        let mut map = HashMap::new();
        map.insert("x".to_string(), 1);
        map.insert("y".to_string(), 2);
        assert_eq!(e.evaluate(&map), 13);
    }
    #[test]
    fn test_lin_comb_expr_simplify() {
        let mut e = LinCombExpr::new();
        e.add_term(2, "x");
        e.add_term(3, "x");
        let simplified = e.simplify();
        assert_eq!(simplified.terms.len(), 1);
        assert_eq!(simplified.terms[0].coefficient, 5);
    }
    #[test]
    fn test_lin_comb_expr_negate() {
        let e = make_expr(3, &[("x", 2)]);
        let neg = e.negate();
        assert_eq!(neg.constant, -3);
        assert_eq!(neg.terms[0].coefficient, -2);
    }
    #[test]
    fn test_lin_comb_expr_add() {
        let a = make_expr(1, &[("x", 2)]);
        let b = make_expr(2, &[("x", 3)]);
        let sum = a.add(&b);
        assert_eq!(sum.constant, 3);
        let x_term = sum
            .terms
            .iter()
            .find(|t| t.variable == "x")
            .expect("x_term should be present");
        assert_eq!(x_term.coefficient, 5);
    }
    #[test]
    fn test_lin_comb_expr_scale() {
        let e = make_expr(2, &[("x", 3)]);
        let scaled = e.scale(4);
        assert_eq!(scaled.constant, 8);
        assert_eq!(scaled.terms[0].coefficient, 12);
    }
    #[test]
    fn test_linear_combination_verify_simple() {
        let mut combo = LinearCombination::new();
        let h1 = make_expr(-3, &[("x", 1)]);
        combo.add_hypothesis("h1", h1.clone());
        combo.set_goal(h1);
        let coeffs = vec![("h1".to_string(), 1i64)];
        assert!(combo.verify(&coeffs));
    }
    #[test]
    fn test_linear_combination_tactic_run() {
        let mut combo = LinearCombination::new();
        let h1 = make_expr(-2, &[("x", 1)]);
        let h2 = make_expr(-3, &[("y", 1)]);
        let goal = make_expr(-5, &[("x", 1), ("y", 1)]);
        combo.add_hypothesis("h1", h1);
        combo.add_hypothesis("h2", h2);
        combo.set_goal(goal);
        let tactic = LinearCombinationTactic::new();
        assert!(tactic.run(&combo));
    }
    #[test]
    fn test_parse_linear_expr_constant() {
        let e = parse_linear_expr("5").expect("e should be present");
        assert_eq!(e.constant, 5);
        assert!(e.terms.is_empty());
    }
    #[test]
    fn test_parse_linear_expr_with_vars() {
        let e = parse_linear_expr("2 * x + 3 * y + 1").expect("e should be present");
        assert_eq!(e.constant, 1);
        let x = e
            .terms
            .iter()
            .find(|t| t.variable == "x")
            .expect("x should be present");
        assert_eq!(x.coefficient, 2);
        let y = e
            .terms
            .iter()
            .find(|t| t.variable == "y")
            .expect("y should be present");
        assert_eq!(y.coefficient, 3);
    }
    #[test]
    fn test_lin_comb_term_new() {
        let t = LinCombTerm::new(5, "x");
        assert_eq!(t.coefficient, 5);
        assert_eq!(t.variable, "x");
    }
}
#[allow(dead_code)]
pub(super) fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// GCD of two integers.
#[allow(dead_code)]
pub fn int_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// LCM of two integers.
#[allow(dead_code)]
pub fn int_lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = int_gcd(a.abs(), b.abs());
    (a.abs() / g) * b.abs()
}
/// LCM of a list.
#[allow(dead_code)]
pub fn lcm_list(nums: &[i64]) -> i64 {
    nums.iter().copied().fold(1, int_lcm)
}
/// GCD of a list.
#[allow(dead_code)]
pub fn gcd_list(nums: &[i64]) -> i64 {
    nums.iter().copied().fold(0, int_gcd)
}
#[cfg(test)]
mod lc_extended_tests {
    use super::*;
    use crate::tactic::linear_combination::*;
    #[test]
    fn test_rat_new_basic() {
        let r = Rat::new(1, 2);
        assert_eq!(r.numer, 1);
        assert_eq!(r.denom, 2);
    }
    #[test]
    fn test_rat_reduction() {
        let r = Rat::new(4, 6);
        assert_eq!(r.numer, 2);
        assert_eq!(r.denom, 3);
    }
    #[test]
    fn test_rat_add() {
        let a = Rat::new(1, 3);
        let b = Rat::new(1, 6);
        let s = a.add(&b);
        assert_eq!(s, Rat::new(1, 2));
    }
    #[test]
    fn test_rat_mul() {
        let a = Rat::new(2, 3);
        let b = Rat::new(3, 4);
        let p = a.mul(&b);
        assert_eq!(p, Rat::new(1, 2));
    }
    #[test]
    fn test_rat_div() {
        let a = Rat::new(1, 2);
        let b = Rat::new(1, 4);
        assert_eq!(a.div(&b), Rat::new(2, 1));
    }
    #[test]
    fn test_rat_matrix_rref() {
        let mut m = RatMatrix::new(2, 3);
        m.set(0, 0, Rat::new(1, 1));
        m.set(0, 1, Rat::new(2, 1));
        m.set(0, 2, Rat::new(3, 1));
        m.set(1, 0, Rat::new(4, 1));
        m.set(1, 1, Rat::new(5, 1));
        m.set(1, 2, Rat::new(6, 1));
        let rank = m.rref();
        assert_eq!(rank, 2);
    }
    #[test]
    fn test_lin_comb_map_add_term() {
        let mut e = LinCombMap::new();
        e.add_term(3, "x");
        e.add_term(2, "y");
        assert_eq!(e.num_terms(), 2);
    }
    #[test]
    fn test_lin_comb_map_is_zero() {
        let e = LinCombMap::new();
        assert!(e.is_zero());
    }
    #[test]
    fn test_lin_comb_map_eval() {
        let mut e = LinCombMap::new();
        e.add_term(2, "x");
        e.add_constant(1);
        let mut env = std::collections::HashMap::new();
        env.insert("x".to_string(), 3i64);
        assert_eq!(e.eval(&env), 7);
    }
    #[test]
    fn test_lin_comb_map_scale() {
        let mut e = LinCombMap::new();
        e.add_term(3, "x");
        let s = e.scale(2);
        assert_eq!(*s.terms.get("x").expect("element at \'x\' should exist"), 6);
    }
    #[test]
    fn test_farkas_cert_valid() {
        let cert = FarkasCert::new(vec![Rat::new(1, 1), Rat::new(1, 1)], Rat::new(-1, 1));
        assert!(cert.is_valid_refutation());
    }
    #[test]
    fn test_simple_lp_feasible() {
        let mut lp = SimpleLp::new([1, 1], 10);
        lp.add_constraint([1, 0], 5);
        lp.add_constraint([0, 1], 5);
        let result = lp.solve();
        matches!(result, LpSolveResult::Optimal(_, _));
    }
    #[test]
    fn test_simple_lp_infeasible() {
        let mut lp = SimpleLp::new([1, 0], 1);
        lp.add_constraint([1, 0], -100);
        lp.add_constraint([-1, 0], -100);
        let result = lp.solve();
        assert_eq!(result, LpSolveResult::Infeasible);
    }
    #[test]
    fn test_int_gcd() {
        assert_eq!(int_gcd(12, 8), 4);
        assert_eq!(int_gcd(7, 5), 1);
    }
    #[test]
    fn test_int_lcm() {
        assert_eq!(int_lcm(4, 6), 12);
    }
    #[test]
    fn test_lcm_list() {
        assert_eq!(lcm_list(&[2, 3, 4]), 12);
    }
    #[test]
    fn test_gcd_list() {
        assert_eq!(gcd_list(&[12, 8, 4]), 4);
    }
    #[test]
    fn test_uni_poly_eval() {
        let p = UniPoly {
            coeffs: vec![1, 2, 1],
        };
        assert_eq!(p.eval(3), 16);
    }
    #[test]
    fn test_uni_poly_mul() {
        let p = UniPoly { coeffs: vec![1, 1] };
        let q = p.mul(&p);
        assert_eq!(q.coeffs, vec![1, 2, 1]);
    }
    #[test]
    fn test_uni_poly_add() {
        let p = UniPoly { coeffs: vec![1, 2] };
        let q = UniPoly { coeffs: vec![3, 1] };
        let s = p.add(&q);
        assert_eq!(s.coeffs, vec![4, 3]);
    }
    #[test]
    fn test_rat_is_zero() {
        assert!(Rat::zero().is_zero());
        assert!(!Rat::one().is_zero());
    }
    #[test]
    fn test_rat_neg() {
        let r = Rat::new(3, 2);
        assert_eq!(r.neg(), Rat::new(-3, 2));
    }
}
/// Compute a simple hash of a LinearComb name.
#[allow(dead_code)]
pub fn linearcomb_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a LinearComb name is valid.
#[allow(dead_code)]
pub fn linearcomb_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a LinearComb string.
#[allow(dead_code)]
pub fn linearcomb_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a LinearComb string to a maximum length.
#[allow(dead_code)]
pub fn linearcomb_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join LinearComb strings with a separator.
#[allow(dead_code)]
pub fn linearcomb_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod linearcomb_ext_tests {
    use super::*;
    use crate::tactic::linear_combination::*;
    #[test]
    fn test_linearcomb_util_new() {
        let u = LinearCombUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_linearcomb_util_tag() {
        let u = LinearCombUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_linearcomb_util_disable() {
        let u = LinearCombUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_linearcomb_registry_register() {
        let mut reg = LinearCombRegistry::new(10);
        let u = LinearCombUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_linearcomb_registry_lookup() {
        let mut reg = LinearCombRegistry::new(10);
        reg.register(LinearCombUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_linearcomb_registry_capacity() {
        let mut reg = LinearCombRegistry::new(2);
        reg.register(LinearCombUtil0::new(1, "a", 1));
        reg.register(LinearCombUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(LinearCombUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_linearcomb_registry_score() {
        let mut reg = LinearCombRegistry::new(10);
        reg.register(LinearCombUtil0::new(1, "a", 10));
        reg.register(LinearCombUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_linearcomb_cache_hit_miss() {
        let mut cache = LinearCombCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_linearcomb_cache_hit_rate() {
        let mut cache = LinearCombCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_linearcomb_cache_clear() {
        let mut cache = LinearCombCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_linearcomb_logger_basic() {
        let mut logger = LinearCombLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_linearcomb_logger_capacity() {
        let mut logger = LinearCombLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_linearcomb_stats_success() {
        let mut stats = LinearCombStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_linearcomb_stats_failure() {
        let mut stats = LinearCombStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_linearcomb_stats_merge() {
        let mut a = LinearCombStats::new();
        let mut b = LinearCombStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_linearcomb_priority_queue() {
        let mut pq = LinearCombPriorityQueue::new();
        pq.push(LinearCombUtil0::new(1, "low", 1), 1);
        pq.push(LinearCombUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_linearcomb_hash() {
        let h1 = linearcomb_hash("foo");
        let h2 = linearcomb_hash("foo");
        assert_eq!(h1, h2);
        let h3 = linearcomb_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_linearcomb_valid_name() {
        assert!(linearcomb_is_valid_name("foo_bar"));
        assert!(!linearcomb_is_valid_name("foo-bar"));
        assert!(!linearcomb_is_valid_name(""));
    }
    #[test]
    fn test_linearcomb_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(linearcomb_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod tacticlinearcombination_analysis_tests {
    use super::*;
    use crate::tactic::linear_combination::*;
    #[test]
    fn test_tacticlinearcombination_result_ok() {
        let r = TacticLinearCombinationResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticlinearcombination_result_err() {
        let r = TacticLinearCombinationResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticlinearcombination_result_partial() {
        let r = TacticLinearCombinationResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticlinearcombination_result_skipped() {
        let r = TacticLinearCombinationResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticlinearcombination_analysis_pass_run() {
        let mut p = TacticLinearCombinationAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticlinearcombination_analysis_pass_empty_input() {
        let mut p = TacticLinearCombinationAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticlinearcombination_analysis_pass_success_rate() {
        let mut p = TacticLinearCombinationAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticlinearcombination_analysis_pass_disable() {
        let mut p = TacticLinearCombinationAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticlinearcombination_pipeline_basic() {
        let mut pipeline = TacticLinearCombinationPipeline::new("main_pipeline");
        pipeline.add_pass(TacticLinearCombinationAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticLinearCombinationAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticlinearcombination_pipeline_disabled_pass() {
        let mut pipeline = TacticLinearCombinationPipeline::new("partial");
        let mut p = TacticLinearCombinationAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticLinearCombinationAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticlinearcombination_diff_basic() {
        let mut d = TacticLinearCombinationDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticlinearcombination_diff_summary() {
        let mut d = TacticLinearCombinationDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticlinearcombination_config_set_get() {
        let mut cfg = TacticLinearCombinationConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticlinearcombination_config_read_only() {
        let mut cfg = TacticLinearCombinationConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticlinearcombination_config_remove() {
        let mut cfg = TacticLinearCombinationConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticlinearcombination_diagnostics_basic() {
        let mut diag = TacticLinearCombinationDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticlinearcombination_diagnostics_max_errors() {
        let mut diag = TacticLinearCombinationDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticlinearcombination_diagnostics_clear() {
        let mut diag = TacticLinearCombinationDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticlinearcombination_config_value_types() {
        let b = TacticLinearCombinationConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticLinearCombinationConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticLinearCombinationConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticLinearCombinationConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticLinearCombinationConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod linear_combination_ext_tests_2900 {
    use super::*;
    use crate::tactic::linear_combination::*;
    #[test]
    fn test_linear_combination_ext_result_ok_2900() {
        let r = LinearCombinationExtResult2900::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_linear_combination_ext_result_err_2900() {
        let r = LinearCombinationExtResult2900::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_linear_combination_ext_result_partial_2900() {
        let r = LinearCombinationExtResult2900::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_linear_combination_ext_result_skipped_2900() {
        let r = LinearCombinationExtResult2900::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_linear_combination_ext_pass_run_2900() {
        let mut p = LinearCombinationExtPass2900::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_linear_combination_ext_pass_empty_2900() {
        let mut p = LinearCombinationExtPass2900::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_linear_combination_ext_pass_rate_2900() {
        let mut p = LinearCombinationExtPass2900::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_linear_combination_ext_pass_disable_2900() {
        let mut p = LinearCombinationExtPass2900::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_linear_combination_ext_pipeline_basic_2900() {
        let mut pipeline = LinearCombinationExtPipeline2900::new("main_pipeline");
        pipeline.add_pass(LinearCombinationExtPass2900::new("pass1"));
        pipeline.add_pass(LinearCombinationExtPass2900::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_linear_combination_ext_pipeline_disabled_2900() {
        let mut pipeline = LinearCombinationExtPipeline2900::new("partial");
        let mut p = LinearCombinationExtPass2900::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LinearCombinationExtPass2900::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_linear_combination_ext_diff_basic_2900() {
        let mut d = LinearCombinationExtDiff2900::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_linear_combination_ext_config_set_get_2900() {
        let mut cfg = LinearCombinationExtConfig2900::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_linear_combination_ext_config_read_only_2900() {
        let mut cfg = LinearCombinationExtConfig2900::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_linear_combination_ext_config_remove_2900() {
        let mut cfg = LinearCombinationExtConfig2900::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_linear_combination_ext_diagnostics_basic_2900() {
        let mut diag = LinearCombinationExtDiag2900::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_linear_combination_ext_diagnostics_max_errors_2900() {
        let mut diag = LinearCombinationExtDiag2900::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_linear_combination_ext_diagnostics_clear_2900() {
        let mut diag = LinearCombinationExtDiag2900::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_linear_combination_ext_config_value_types_2900() {
        let b = LinearCombinationExtConfigVal2900::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = LinearCombinationExtConfigVal2900::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = LinearCombinationExtConfigVal2900::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = LinearCombinationExtConfigVal2900::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = LinearCombinationExtConfigVal2900::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
