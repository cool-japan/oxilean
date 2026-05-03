//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ArithDecide, CachedDecideTactic, DecideCache, DecideConjunction, DecideEngine,
    DecideExtConfig900, DecideExtConfigVal900, DecideExtDiag900, DecideExtDiff900,
    DecideExtPass900, DecideExtPipeline900, DecideExtResult900, DecideLogic, DecideResult,
    DecideStats, DecideTactic, IntDecide, ListDecide, NativeDecide, SetDecide,
    TacticDecideAnalysisPass, TacticDecideConfig, TacticDecideConfigValue, TacticDecideDiagnostics,
    TacticDecideDiff, TacticDecidePipeline, TacticDecideResult,
};
#[allow(unused_imports)]
use crate::basic::{MVarId, MetaContext};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

/// Find the first occurrence of `op` in `s` at parenthesis depth 0.
pub(super) fn find_binary_op(s: &str, op: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    let bytes = s.as_bytes();
    let op_bytes = op.as_bytes();
    let op_len = op_bytes.len();
    if op_len > s.len() {
        return None;
    }
    for i in 0..=(s.len() - op_len) {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && &bytes[i..i + op_len] == op_bytes {
            return Some(i);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::decide::*;
    #[test]
    fn test_decide_nat_eq_true() {
        assert_eq!(DecideTactic::decide_nat_eq(5, 5), DecideResult::True);
    }
    #[test]
    fn test_decide_nat_eq_false() {
        assert_eq!(DecideTactic::decide_nat_eq(3, 7), DecideResult::False);
    }
    #[test]
    fn test_decide_nat_lt_true() {
        assert_eq!(DecideTactic::decide_nat_lt(2, 5), DecideResult::True);
    }
    #[test]
    fn test_decide_nat_lt_false() {
        assert_eq!(DecideTactic::decide_nat_lt(5, 2), DecideResult::False);
    }
    #[test]
    fn test_decide_bool() {
        assert_eq!(DecideTactic::decide_bool(true), DecideResult::True);
        assert_eq!(DecideTactic::decide_bool(false), DecideResult::False);
    }
    #[test]
    fn test_decide_and() {
        assert_eq!(
            DecideTactic::decide_and(DecideResult::True, DecideResult::True),
            DecideResult::True
        );
        assert_eq!(
            DecideTactic::decide_and(DecideResult::True, DecideResult::False),
            DecideResult::False
        );
        assert_eq!(
            DecideTactic::decide_and(DecideResult::False, DecideResult::Unknown),
            DecideResult::False
        );
    }
    #[test]
    fn test_decide_or() {
        assert_eq!(
            DecideTactic::decide_or(DecideResult::False, DecideResult::True),
            DecideResult::True
        );
        assert_eq!(
            DecideTactic::decide_or(DecideResult::False, DecideResult::False),
            DecideResult::False
        );
    }
    #[test]
    fn test_decide_not() {
        assert_eq!(
            DecideTactic::decide_not(DecideResult::True),
            DecideResult::False
        );
        assert_eq!(
            DecideTactic::decide_not(DecideResult::False),
            DecideResult::True
        );
        assert_eq!(
            DecideTactic::decide_not(DecideResult::Unknown),
            DecideResult::Unknown
        );
    }
    #[test]
    fn test_evaluate_literals() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("true"), DecideResult::True);
        assert_eq!(t.evaluate("false"), DecideResult::False);
    }
    #[test]
    fn test_evaluate_nat_eq() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("3 = 3"), DecideResult::True);
        assert_eq!(t.evaluate("3 = 4"), DecideResult::False);
    }
    #[test]
    fn test_evaluate_nat_lt() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("1 < 2"), DecideResult::True);
        assert_eq!(t.evaluate("2 < 1"), DecideResult::False);
    }
    #[test]
    fn test_native_decide_run() {
        let nd = NativeDecide::new();
        assert_eq!(nd.run("true"), DecideResult::True);
        assert_eq!(nd.run("false"), DecideResult::False);
        assert_eq!(nd.run("5 = 5"), DecideResult::True);
    }
    #[test]
    fn test_evaluate_unknown() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("Decidable P"), DecideResult::Unknown);
    }
    #[test]
    fn test_decide_result_accessors() {
        assert!(DecideResult::True.is_true());
        assert!(!DecideResult::True.is_false());
        assert!(!DecideResult::True.is_unknown());
        assert!(DecideResult::False.is_false());
        assert!(DecideResult::Unknown.is_unknown());
    }
}
/// Compute GCD of two unsigned 64-bit integers.
#[allow(dead_code)]
pub(super) fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// Primality test by trial division (for small numbers).
#[allow(dead_code)]
pub(super) fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
/// Decide bounded universal quantification over a range.
#[allow(dead_code)]
pub fn decide_forall_range<F: Fn(u64) -> DecideResult>(lo: u64, hi: u64, pred: F) -> DecideResult {
    for i in lo..=hi {
        let r = pred(i);
        if r == DecideResult::False {
            return DecideResult::False;
        }
        if r == DecideResult::Unknown {
            return DecideResult::Unknown;
        }
    }
    DecideResult::True
}
/// Decide bounded existential quantification over a range.
#[allow(dead_code)]
pub fn decide_exists_range<F: Fn(u64) -> DecideResult>(lo: u64, hi: u64, pred: F) -> DecideResult {
    for i in lo..=hi {
        let r = pred(i);
        if r == DecideResult::True {
            return DecideResult::True;
        }
    }
    DecideResult::False
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::tactic::decide::*;
    #[test]
    fn test_decide_cache_basic() {
        let mut c = DecideCache::new(10);
        assert!(c.get("true").is_none());
        c.store("true".to_string(), DecideResult::True);
        assert_eq!(c.get("true"), Some(DecideResult::True));
        assert_eq!(c.len(), 1);
    }
    #[test]
    fn test_decide_cache_hit_rate() {
        let mut c = DecideCache::new(10);
        c.store("true".to_string(), DecideResult::True);
        c.get("true");
        c.get("false");
        assert!((c.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_decide_cache_clear() {
        let mut c = DecideCache::new(10);
        c.store("true".to_string(), DecideResult::True);
        c.clear();
        assert!(c.is_empty());
    }
    #[test]
    fn test_cached_decide_tactic() {
        let mut tac = CachedDecideTactic::new(100);
        assert_eq!(tac.evaluate("true"), DecideResult::True);
        assert_eq!(tac.evaluate("true"), DecideResult::True);
        assert!(tac.hit_rate() > 0.0);
    }
    #[test]
    fn test_decide_conjunction_all_true() {
        let mut c = DecideConjunction::new();
        c.add_goal("true");
        c.add_goal("3 = 3");
        c.add_goal("1 < 2");
        assert_eq!(c.decide_all(), DecideResult::True);
    }
    #[test]
    fn test_decide_conjunction_any_true() {
        let mut c = DecideConjunction::new();
        c.add_goal("false");
        c.add_goal("3 = 3");
        assert_eq!(c.decide_any(), DecideResult::True);
    }
    #[test]
    fn test_decide_each() {
        let mut c = DecideConjunction::new();
        c.add_goal("true");
        c.add_goal("false");
        let results = c.decide_each();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], DecideResult::True);
        assert_eq!(results[1], DecideResult::False);
    }
    #[test]
    fn test_decide_xor() {
        assert_eq!(
            DecideLogic::decide_xor(DecideResult::True, DecideResult::False),
            DecideResult::True
        );
        assert_eq!(
            DecideLogic::decide_xor(DecideResult::True, DecideResult::True),
            DecideResult::False
        );
    }
    #[test]
    fn test_decide_implies() {
        assert_eq!(
            DecideLogic::decide_implies(DecideResult::True, DecideResult::False),
            DecideResult::False
        );
        assert_eq!(
            DecideLogic::decide_implies(DecideResult::False, DecideResult::False),
            DecideResult::True
        );
    }
    #[test]
    fn test_decide_iff() {
        assert_eq!(
            DecideLogic::decide_iff(DecideResult::True, DecideResult::True),
            DecideResult::True
        );
        assert_eq!(
            DecideLogic::decide_iff(DecideResult::True, DecideResult::False),
            DecideResult::False
        );
    }
    #[test]
    fn test_decide_nand_nor() {
        assert_eq!(
            DecideLogic::decide_nand(DecideResult::True, DecideResult::True),
            DecideResult::False
        );
        assert_eq!(
            DecideLogic::decide_nor(DecideResult::False, DecideResult::False),
            DecideResult::True
        );
    }
    #[test]
    fn test_int_decide_eq() {
        assert_eq!(IntDecide::decide_int_eq(5, 5), DecideResult::True);
        assert_eq!(IntDecide::decide_int_eq(5, 6), DecideResult::False);
    }
    #[test]
    fn test_int_decide_lt_le() {
        assert_eq!(IntDecide::decide_int_lt(-3, 0), DecideResult::True);
        assert_eq!(IntDecide::decide_int_le(5, 5), DecideResult::True);
    }
    #[test]
    fn test_int_decide_dvd() {
        assert_eq!(IntDecide::decide_int_dvd(3, 12), DecideResult::True);
        assert_eq!(IntDecide::decide_int_dvd(5, 12), DecideResult::False);
        assert_eq!(IntDecide::decide_int_dvd(0, 0), DecideResult::True);
        assert_eq!(IntDecide::decide_int_dvd(0, 5), DecideResult::False);
    }
    #[test]
    fn test_int_decide_even_odd() {
        assert_eq!(IntDecide::decide_even(4), DecideResult::True);
        assert_eq!(IntDecide::decide_odd(7), DecideResult::True);
        assert_eq!(IntDecide::decide_even(3), DecideResult::False);
    }
    #[test]
    fn test_int_decide_coprime() {
        assert_eq!(IntDecide::decide_coprime(3, 5), DecideResult::True);
        assert_eq!(IntDecide::decide_coprime(6, 9), DecideResult::False);
    }
    #[test]
    fn test_int_decide_prime() {
        assert_eq!(IntDecide::decide_prime(2), DecideResult::True);
        assert_eq!(IntDecide::decide_prime(17), DecideResult::True);
        assert_eq!(IntDecide::decide_prime(15), DecideResult::False);
        assert_eq!(IntDecide::decide_prime(1), DecideResult::False);
    }
    #[test]
    fn test_list_decide_mem() {
        assert_eq!(ListDecide::decide_mem(&[1, 2, 3], 2), DecideResult::True);
        assert_eq!(ListDecide::decide_mem(&[1, 2, 3], 5), DecideResult::False);
    }
    #[test]
    fn test_list_decide_sorted() {
        assert_eq!(ListDecide::decide_sorted(&[1, 2, 3, 4]), DecideResult::True);
        assert_eq!(ListDecide::decide_sorted(&[1, 3, 2]), DecideResult::False);
    }
    #[test]
    fn test_list_decide_palindrome() {
        assert_eq!(
            ListDecide::decide_palindrome(&[1, 2, 1]),
            DecideResult::True
        );
        assert_eq!(
            ListDecide::decide_palindrome(&[1, 2, 3]),
            DecideResult::False
        );
    }
    #[test]
    fn test_list_decide_distinct() {
        assert_eq!(ListDecide::decide_distinct(&[1, 2, 3]), DecideResult::True);
        assert_eq!(ListDecide::decide_distinct(&[1, 2, 2]), DecideResult::False);
    }
    #[test]
    fn test_set_decide_subset() {
        assert_eq!(
            SetDecide::decide_subset(&[1, 2], &[1, 2, 3]),
            DecideResult::True
        );
        assert_eq!(
            SetDecide::decide_subset(&[1, 4], &[1, 2, 3]),
            DecideResult::False
        );
    }
    #[test]
    fn test_set_decide_disjoint() {
        assert_eq!(
            SetDecide::decide_disjoint(&[1, 2], &[3, 4]),
            DecideResult::True
        );
        assert_eq!(
            SetDecide::decide_disjoint(&[1, 2], &[2, 3]),
            DecideResult::False
        );
    }
    #[test]
    fn test_set_decide_eq() {
        assert_eq!(
            SetDecide::decide_set_eq(&[1, 2, 3], &[3, 2, 1]),
            DecideResult::True
        );
    }
    #[test]
    fn test_arith_sum_eq() {
        assert_eq!(ArithDecide::decide_sum_eq(3, 4, 7), DecideResult::True);
        assert_eq!(ArithDecide::decide_sum_eq(3, 4, 8), DecideResult::False);
    }
    #[test]
    fn test_arith_product_eq() {
        assert_eq!(ArithDecide::decide_product_eq(3, 4, 12), DecideResult::True);
    }
    #[test]
    fn test_arith_power_eq() {
        assert_eq!(
            ArithDecide::decide_power_eq(2, 10, 1024),
            DecideResult::True
        );
    }
    #[test]
    fn test_arith_congruent() {
        assert_eq!(ArithDecide::decide_congruent(17, 5, 12), DecideResult::True);
        assert_eq!(
            ArithDecide::decide_congruent(17, 6, 12),
            DecideResult::False
        );
    }
    #[test]
    fn test_arith_perfect_square() {
        assert_eq!(ArithDecide::decide_perfect_square(25), DecideResult::True);
        assert_eq!(ArithDecide::decide_perfect_square(26), DecideResult::False);
    }
    #[test]
    fn test_decide_stats_record() {
        let mut s = DecideStats::new();
        s.record(&DecideResult::True);
        s.record(&DecideResult::False);
        s.record(&DecideResult::Unknown);
        assert_eq!(s.total, 3);
        assert_eq!(s.decided_true, 1);
        assert_eq!(s.decided_false, 1);
        assert_eq!(s.unknown, 1);
    }
    #[test]
    fn test_decide_stats_success_rate() {
        let mut s = DecideStats::new();
        s.record(&DecideResult::True);
        s.record(&DecideResult::False);
        s.record(&DecideResult::Unknown);
        assert!((s.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_decide_stats_merge() {
        let mut s1 = DecideStats {
            decided_true: 3,
            ..Default::default()
        };
        let s2 = DecideStats {
            decided_true: 2,
            unknown: 1,
            ..Default::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.decided_true, 5);
        assert_eq!(s1.unknown, 1);
    }
    #[test]
    fn test_decide_stats_reset() {
        let mut s = DecideStats {
            decided_true: 10,
            total: 10,
            ..Default::default()
        };
        s.reset();
        assert_eq!(s.total, 0);
    }
    #[test]
    fn test_decide_engine_basic() {
        let mut e = DecideEngine::new();
        assert_eq!(e.decide("true"), DecideResult::True);
        assert_eq!(e.decide("false"), DecideResult::False);
        assert_eq!(e.stats().total, 2);
    }
    #[test]
    fn test_decide_engine_all() {
        let mut e = DecideEngine::new();
        let result = e.decide_all(&["true", "3 = 3", "1 < 2"]);
        assert_eq!(result, DecideResult::True);
    }
    #[test]
    fn test_decide_engine_any() {
        let mut e = DecideEngine::new();
        let result = e.decide_any(&["false", "3 = 4", "true"]);
        assert_eq!(result, DecideResult::True);
    }
    #[test]
    fn test_decide_engine_reset_stats() {
        let mut e = DecideEngine::new();
        e.decide("true");
        e.reset_stats();
        assert_eq!(e.stats().total, 0);
    }
    #[test]
    fn test_forall_range() {
        let result = decide_forall_range(1, 10, |i| DecideTactic::decide_bool(i > 0));
        assert_eq!(result, DecideResult::True);
        let result2 = decide_forall_range(0, 10, |i| DecideTactic::decide_bool(i > 0));
        assert_eq!(result2, DecideResult::False);
    }
    #[test]
    fn test_exists_range() {
        let result = decide_exists_range(1, 10, |i| DecideTactic::decide_bool(i == 7));
        assert_eq!(result, DecideResult::True);
        let result2 = decide_exists_range(1, 5, |i| DecideTactic::decide_bool(i == 10));
        assert_eq!(result2, DecideResult::False);
    }
    #[test]
    fn test_gcd_helper() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(0, 5), 5);
    }
    #[test]
    fn test_is_prime_helper() {
        assert!(is_prime(2));
        assert!(is_prime(97));
        assert!(!is_prime(1));
        assert!(!is_prime(100));
    }
    #[test]
    fn test_evaluate_le() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("3 <= 5"), DecideResult::True);
        assert_eq!(t.evaluate("5 <= 3"), DecideResult::False);
        assert_eq!(t.evaluate("5 <= 5"), DecideResult::True);
    }
    #[test]
    fn test_evaluate_negation() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("not true"), DecideResult::False);
        assert_eq!(t.evaluate("not false"), DecideResult::True);
    }
    #[test]
    fn test_evaluate_conjunction() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("true and true"), DecideResult::True);
        assert_eq!(t.evaluate("true and false"), DecideResult::False);
        assert_eq!(t.evaluate("true && false"), DecideResult::False);
    }
    #[test]
    fn test_evaluate_disjunction() {
        let t = DecideTactic::new();
        assert_eq!(t.evaluate("false or true"), DecideResult::True);
        assert_eq!(t.evaluate("false || false"), DecideResult::False);
    }
    #[test]
    fn test_list_decide_all_equal() {
        assert_eq!(ListDecide::decide_all_equal(&[3, 3, 3]), DecideResult::True);
        assert_eq!(
            ListDecide::decide_all_equal(&[3, 3, 4]),
            DecideResult::False
        );
        assert_eq!(ListDecide::decide_all_equal(&[]), DecideResult::True);
    }
    #[test]
    fn test_arith_common_divisor() {
        assert_eq!(
            ArithDecide::decide_common_divisor(4, 12, 8),
            DecideResult::True
        );
        assert_eq!(
            ArithDecide::decide_common_divisor(5, 12, 8),
            DecideResult::False
        );
    }
    #[test]
    fn test_arith_mod_eq() {
        assert_eq!(ArithDecide::decide_mod_eq(17, 5, 2), DecideResult::True);
        assert_eq!(ArithDecide::decide_mod_eq(17, 5, 3), DecideResult::False);
        assert_eq!(ArithDecide::decide_mod_eq(17, 0, 2), DecideResult::Unknown);
    }
}
#[cfg(test)]
mod tacticdecide_analysis_tests {
    use super::*;
    use crate::tactic::decide::*;
    #[test]
    fn test_tacticdecide_result_ok() {
        let r = TacticDecideResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticdecide_result_err() {
        let r = TacticDecideResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticdecide_result_partial() {
        let r = TacticDecideResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticdecide_result_skipped() {
        let r = TacticDecideResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticdecide_analysis_pass_run() {
        let mut p = TacticDecideAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticdecide_analysis_pass_empty_input() {
        let mut p = TacticDecideAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticdecide_analysis_pass_success_rate() {
        let mut p = TacticDecideAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticdecide_analysis_pass_disable() {
        let mut p = TacticDecideAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticdecide_pipeline_basic() {
        let mut pipeline = TacticDecidePipeline::new("main_pipeline");
        pipeline.add_pass(TacticDecideAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticDecideAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticdecide_pipeline_disabled_pass() {
        let mut pipeline = TacticDecidePipeline::new("partial");
        let mut p = TacticDecideAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticDecideAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticdecide_diff_basic() {
        let mut d = TacticDecideDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticdecide_diff_summary() {
        let mut d = TacticDecideDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticdecide_config_set_get() {
        let mut cfg = TacticDecideConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticdecide_config_read_only() {
        let mut cfg = TacticDecideConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticdecide_config_remove() {
        let mut cfg = TacticDecideConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticdecide_diagnostics_basic() {
        let mut diag = TacticDecideDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticdecide_diagnostics_max_errors() {
        let mut diag = TacticDecideDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticdecide_diagnostics_clear() {
        let mut diag = TacticDecideDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticdecide_config_value_types() {
        let b = TacticDecideConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticDecideConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticDecideConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticDecideConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticDecideConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod decide_ext_tests_900 {
    use super::*;
    use crate::tactic::decide::*;
    #[test]
    fn test_decide_ext_result_ok_900() {
        let r = DecideExtResult900::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_decide_ext_result_err_900() {
        let r = DecideExtResult900::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_decide_ext_result_partial_900() {
        let r = DecideExtResult900::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_decide_ext_result_skipped_900() {
        let r = DecideExtResult900::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_decide_ext_pass_run_900() {
        let mut p = DecideExtPass900::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_decide_ext_pass_empty_900() {
        let mut p = DecideExtPass900::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_decide_ext_pass_rate_900() {
        let mut p = DecideExtPass900::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_decide_ext_pass_disable_900() {
        let mut p = DecideExtPass900::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_decide_ext_pipeline_basic_900() {
        let mut pipeline = DecideExtPipeline900::new("main_pipeline");
        pipeline.add_pass(DecideExtPass900::new("pass1"));
        pipeline.add_pass(DecideExtPass900::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_decide_ext_pipeline_disabled_900() {
        let mut pipeline = DecideExtPipeline900::new("partial");
        let mut p = DecideExtPass900::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(DecideExtPass900::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_decide_ext_diff_basic_900() {
        let mut d = DecideExtDiff900::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_decide_ext_config_set_get_900() {
        let mut cfg = DecideExtConfig900::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_decide_ext_config_read_only_900() {
        let mut cfg = DecideExtConfig900::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_decide_ext_config_remove_900() {
        let mut cfg = DecideExtConfig900::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_decide_ext_diagnostics_basic_900() {
        let mut diag = DecideExtDiag900::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_decide_ext_diagnostics_max_errors_900() {
        let mut diag = DecideExtDiag900::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_decide_ext_diagnostics_clear_900() {
        let mut diag = DecideExtDiag900::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_decide_ext_config_value_types_900() {
        let b = DecideExtConfigVal900::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = DecideExtConfigVal900::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = DecideExtConfigVal900::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = DecideExtConfigVal900::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = DecideExtConfigVal900::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

/// `decide` — evaluate decidable propositions by computation.
///
/// Converts the current goal's target to a string and runs it through
/// [`DecideTactic::evaluate`], which handles `Nat` comparisons (`=`, `<`, `≤`),
/// boolean operations (`&&`, `||`, `!`), and the literals `true` / `false`.
/// If the proposition evaluates to `true`, the goal is closed with a synthetic
/// proof constant; if it evaluates to `false` the tactic fails; if the decision
/// procedure returns `Unknown` it also fails, as the proposition is not in the
/// decidable fragment recognised by this kernel.
pub fn tac_decide(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("decide: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let target_str = target.to_string();

    let tac = DecideTactic::new();
    match tac.evaluate(&target_str) {
        DecideResult::True => {
            let proof = Expr::Const(Name::str("decide.isTrue"), vec![]);
            state.close_goal(proof, ctx)?;
            Ok(())
        }
        DecideResult::False => Err(TacticError::Failed(format!(
            "decide: proposition `{}` evaluates to false",
            target_str
        ))),
        DecideResult::Unknown => Err(TacticError::Failed(format!(
            "decide: proposition `{}` is not in the decidable fragment",
            target_str
        ))),
    }
}
