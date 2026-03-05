//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    NnfTree, PolyF64, PolyrithTactic, PosProofStep, PositivityCheckerExt, PositivityExtConfig100,
    PositivityExtConfig101, PositivityExtConfigVal100, PositivityExtConfigVal101,
    PositivityExtDiag100, PositivityExtDiag101, PositivityExtDiff100, PositivityExtDiff101,
    PositivityExtPass100, PositivityExtPass101, PositivityExtPipeline100, PositivityExtPipeline101,
    PositivityExtResult100, PositivityExtResult101, PositivitySystem, PositivityTactic,
    PosstellensatzCert, Sign, SignContext, SignInfo, SignInterval, SosCertificate,
    TacticPositivityAnalysisPass, TacticPositivityConfig, TacticPositivityConfigValue,
    TacticPositivityDiagnostics, TacticPositivityDiff, TacticPositivityPipeline,
    TacticPositivityResult,
};

/// Check whether a string represents a square term: `x^2`, `(...)^2`, or numeric squares.
pub(super) fn is_square_term(s: &str) -> bool {
    let s = s.trim();
    if s.ends_with("^2") {
        return true;
    }
    if let Ok(n) = s.parse::<u64>() {
        let sq = (n as f64).sqrt() as u64;
        return sq * sq == n;
    }
    false
}
/// Find the first occurrence of `op` in `s` at parenthesis depth 0.
pub(super) fn find_op_at_depth0(s: &str, op: char) -> Option<usize> {
    let mut depth: i32 = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ch if ch == op && depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_sign_of_nat() {
        assert_eq!(PositivityTactic::sign_of_nat(0), Sign::Zero);
        assert_eq!(PositivityTactic::sign_of_nat(5), Sign::Pos);
    }
    #[test]
    fn test_sign_of_abs_always_nonneg() {
        assert_eq!(PositivityTactic::sign_of_abs(Sign::Neg), Sign::Nonneg);
        assert_eq!(PositivityTactic::sign_of_abs(Sign::Pos), Sign::Nonneg);
        assert_eq!(PositivityTactic::sign_of_abs(Sign::Zero), Sign::Zero);
    }
    #[test]
    fn test_sign_of_product_pos_pos() {
        assert_eq!(
            PositivityTactic::sign_of_product(Sign::Pos, Sign::Pos),
            Sign::Pos
        );
    }
    #[test]
    fn test_sign_of_product_neg_neg() {
        assert_eq!(
            PositivityTactic::sign_of_product(Sign::Neg, Sign::Neg),
            Sign::Pos
        );
    }
    #[test]
    fn test_sign_of_power_even() {
        assert_eq!(PositivityTactic::sign_of_power(Sign::Neg, 2), Sign::Pos);
        assert_eq!(PositivityTactic::sign_of_power(Sign::Zero, 2), Sign::Zero);
    }
    #[test]
    fn test_sign_of_power_zero_exp() {
        assert_eq!(PositivityTactic::sign_of_power(Sign::Neg, 0), Sign::Pos);
        assert_eq!(PositivityTactic::sign_of_power(Sign::Unknown, 0), Sign::Pos);
    }
    #[test]
    fn test_analyze_expr_numeric() {
        let t = PositivityTactic::new();
        assert_eq!(t.analyze_expr("5"), Sign::Pos);
        assert_eq!(t.analyze_expr("0"), Sign::Zero);
    }
    #[test]
    fn test_analyze_expr_abs() {
        let t = PositivityTactic::new();
        assert!(t.analyze_expr("abs(x)").is_nonneg());
    }
    #[test]
    fn test_can_prove_pos_literal() {
        let t = PositivityTactic::new();
        assert!(t.can_prove_pos("3"));
        assert!(!t.can_prove_pos("0"));
    }
    #[test]
    fn test_can_prove_nonneg_literal() {
        let t = PositivityTactic::new();
        assert!(t.can_prove_nonneg("0"));
        assert!(t.can_prove_nonneg("5"));
    }
    #[test]
    fn test_polyrith_sum_of_squares() {
        let p = PolyrithTactic::new();
        assert!(p.verify_sum_of_squares("x^2"));
        assert!(p.verify_sum_of_squares("x^2 + y^2"));
        assert!(!p.verify_sum_of_squares("x^3"));
        assert!(!p.verify_sum_of_squares("x + y"));
    }
    #[test]
    fn test_sign_accessors() {
        assert!(Sign::Pos.is_positive());
        assert!(Sign::Pos.is_nonneg());
        assert!(!Sign::Pos.is_nonpos());
        assert!(Sign::Zero.is_nonneg());
        assert!(Sign::Zero.is_nonpos());
        assert!(Sign::Neg.is_nonpos());
        assert!(!Sign::Unknown.is_known());
        assert!(Sign::Pos.is_known());
    }
}
/// Sign of sum.
#[allow(dead_code)]
pub fn sign_of_sum(a: &Sign, b: &Sign) -> Sign {
    match (a, b) {
        (Sign::Pos, Sign::Pos) => Sign::Pos,
        (Sign::Pos, Sign::Zero) | (Sign::Zero, Sign::Pos) => Sign::Pos,
        (Sign::Pos, Sign::Nonneg) | (Sign::Nonneg, Sign::Pos) => Sign::Pos,
        (Sign::Neg, Sign::Neg) => Sign::Neg,
        (Sign::Neg, Sign::Zero) | (Sign::Zero, Sign::Neg) => Sign::Neg,
        (Sign::Neg, Sign::Nonpos) | (Sign::Nonpos, Sign::Neg) => Sign::Neg,
        (Sign::Zero, Sign::Zero) => Sign::Zero,
        (Sign::Nonneg, Sign::Nonneg) => Sign::Nonneg,
        (Sign::Nonpos, Sign::Nonpos) => Sign::Nonpos,
        _ => Sign::Unknown,
    }
}
/// Sign of product.
#[allow(dead_code)]
pub fn sign_of_product(a: &Sign, b: &Sign) -> Sign {
    match (a, b) {
        (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
        (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => Sign::Pos,
        (Sign::Pos, Sign::Nonneg) | (Sign::Nonneg, Sign::Pos) => Sign::Nonneg,
        (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Sign::Neg,
        (Sign::Neg, Sign::Nonneg) | (Sign::Nonneg, Sign::Neg) => Sign::Nonpos,
        (Sign::Nonneg, Sign::Nonneg) => Sign::Nonneg,
        (Sign::Nonpos, Sign::Nonpos) => Sign::Nonneg,
        (Sign::Nonneg, Sign::Nonpos) | (Sign::Nonpos, Sign::Nonneg) => Sign::Nonpos,
        _ => Sign::Unknown,
    }
}
/// Sign of negation.
#[allow(dead_code)]
pub fn sign_of_negation(s: &Sign) -> Sign {
    match s {
        Sign::Pos => Sign::Neg,
        Sign::Neg => Sign::Pos,
        Sign::Zero => Sign::Zero,
        Sign::Nonneg => Sign::Nonpos,
        Sign::Nonpos => Sign::Nonneg,
        Sign::Unknown => Sign::Unknown,
    }
}
/// Sign of power (non-negative exponent).
#[allow(dead_code)]
pub fn sign_of_power(base: &Sign, exp: u32) -> Sign {
    if exp == 0 {
        return Sign::Pos;
    }
    match base {
        Sign::Zero => Sign::Zero,
        Sign::Pos => Sign::Pos,
        Sign::Nonneg => Sign::Nonneg,
        Sign::Neg => {
            if exp % 2 == 0 {
                Sign::Pos
            } else {
                Sign::Neg
            }
        }
        Sign::Nonpos => {
            if exp % 2 == 0 {
                Sign::Nonneg
            } else {
                Sign::Nonpos
            }
        }
        Sign::Unknown => Sign::Unknown,
    }
}
/// Sign of sqrt (only defined for nonneg).
#[allow(dead_code)]
pub fn sign_of_sqrt(s: &Sign) -> Sign {
    match s {
        Sign::Pos => Sign::Pos,
        Sign::Nonneg => Sign::Nonneg,
        Sign::Zero => Sign::Zero,
        _ => Sign::Unknown,
    }
}
/// Sign of exp (always positive).
#[allow(dead_code)]
pub fn sign_of_exp(_s: &Sign) -> Sign {
    Sign::Pos
}
/// Sign of log (defined for positive).
#[allow(dead_code)]
pub fn sign_of_log(s: &Sign) -> Sign {
    match s {
        Sign::Pos => Sign::Unknown,
        _ => Sign::Unknown,
    }
}
/// Sign of absolute value.
#[allow(dead_code)]
pub fn sign_of_abs(s: &Sign) -> Sign {
    match s {
        Sign::Zero => Sign::Zero,
        Sign::Pos | Sign::Neg => Sign::Pos,
        Sign::Nonneg | Sign::Nonpos => Sign::Nonneg,
        Sign::Unknown => Sign::Nonneg,
    }
}
#[cfg(test)]
mod positivity_extended_tests {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_interval_is_pos() {
        let iv = SignInterval::pos();
        assert!(iv.is_pos());
        assert!(iv.is_nonneg());
        assert!(!iv.contains_zero());
    }
    #[test]
    fn test_interval_is_nonneg() {
        let iv = SignInterval::nonneg();
        assert!(iv.is_nonneg());
        assert!(!iv.is_pos());
        assert!(iv.contains_zero());
    }
    #[test]
    fn test_interval_add_pos_pos() {
        let a = SignInterval::pos();
        let b = SignInterval::pos();
        let s = a.add(&b);
        assert!(s.is_pos());
    }
    #[test]
    fn test_interval_mul_pos_pos() {
        let a = SignInterval::new(1.0, 2.0);
        let b = SignInterval::new(3.0, 4.0);
        let p = a.mul(&b);
        assert!(p.is_pos());
    }
    #[test]
    fn test_sign_of_sum_pos_pos() {
        assert_eq!(sign_of_sum(&Sign::Pos, &Sign::Pos), Sign::Pos);
    }
    #[test]
    fn test_sign_of_product_neg_neg() {
        assert_eq!(sign_of_product(&Sign::Neg, &Sign::Neg), Sign::Pos);
    }
    #[test]
    fn test_sign_of_power_even() {
        assert_eq!(sign_of_power(&Sign::Neg, 2), Sign::Pos);
    }
    #[test]
    fn test_sign_of_power_odd() {
        assert_eq!(sign_of_power(&Sign::Neg, 3), Sign::Neg);
    }
    #[test]
    fn test_sign_of_sqrt() {
        assert_eq!(sign_of_sqrt(&Sign::Pos), Sign::Pos);
        assert_eq!(sign_of_sqrt(&Sign::Neg), Sign::Unknown);
    }
    #[test]
    fn test_sign_of_exp() {
        assert_eq!(sign_of_exp(&Sign::Unknown), Sign::Pos);
    }
    #[test]
    fn test_sign_of_abs() {
        assert_eq!(sign_of_abs(&Sign::Neg), Sign::Pos);
        assert_eq!(sign_of_abs(&Sign::Zero), Sign::Zero);
    }
    #[test]
    fn test_sign_of_negation() {
        assert_eq!(sign_of_negation(&Sign::Pos), Sign::Neg);
        assert_eq!(sign_of_negation(&Sign::Nonneg), Sign::Nonpos);
    }
    #[test]
    fn test_sign_context_lookup() {
        let mut ctx = SignContext::new();
        ctx.known_pos("x");
        assert!(ctx.lookup("x").is_some());
        assert!(ctx.lookup("x").expect("value should be present").is_pos());
    }
    #[test]
    fn test_pos_checker_ext() {
        let mut ctx = SignContext::new();
        ctx.known_pos("a");
        ctx.known_nonneg("b");
        let checker = PositivityCheckerExt::with_context(ctx);
        let sum_sign = checker.check_sum_pos(&["a", "b"]);
        assert_eq!(sum_sign, Sign::Pos);
    }
    #[test]
    fn test_pos_proof_step_depth() {
        let p = PosProofStep::AddPos(
            Box::new(PosProofStep::PosLit(1)),
            Box::new(PosProofStep::PosLit(2)),
        );
        assert_eq!(p.depth(), 1);
        assert_eq!(p.size(), 3);
    }
    #[test]
    fn test_pos_proof_step_sorry() {
        let p = PosProofStep::Sorry;
        assert!(p.is_sorry());
    }
    #[test]
    fn test_interval_abs() {
        let iv = SignInterval::new(-3.0, 2.0);
        let abs = iv.abs_interval();
        assert!(abs.is_nonneg());
    }
    #[test]
    fn test_interval_meet() {
        let a = SignInterval::new(0.0, 5.0);
        let b = SignInterval::new(2.0, 8.0);
        let m = a.meet(&b);
        assert_eq!(m.lo, 2.0);
        assert_eq!(m.hi, 5.0);
    }
    #[test]
    fn test_interval_join() {
        let a = SignInterval::new(0.0, 3.0);
        let b = SignInterval::new(-1.0, 2.0);
        let j = a.join(&b);
        assert_eq!(j.lo, -1.0);
        assert_eq!(j.hi, 3.0);
    }
}
/// Attempt to find an SOS decomposition for a polynomial.
#[allow(dead_code)]
pub fn find_sos_decomposition(poly: &PolyF64) -> Option<SosCertificate> {
    if poly.poly_degree() == 0 {
        let c = poly.coeffs[0];
        if c >= 0.0 {
            return Some(SosCertificate::new(vec![PolyF64::poly_constant(c.sqrt())]));
        }
        return None;
    }
    if poly.poly_degree() == 2 {
        let a = poly.coeffs.get(2).copied().unwrap_or(0.0);
        let b = poly.coeffs.get(1).copied().unwrap_or(0.0);
        let c = poly.coeffs.first().copied().unwrap_or(0.0);
        if a >= 0.0 && c >= 0.0 && b.abs() < 1e-10 {
            let p1 = PolyF64::new(vec![0.0, a.sqrt()]);
            let p2 = PolyF64::poly_constant(c.sqrt());
            return Some(SosCertificate::new(vec![p1, p2]));
        }
    }
    None
}
/// Look up the sign of a known function.
#[allow(dead_code)]
pub fn sign_of_known(name: &str) -> SignInfo {
    match name {
        "exp" | "cosh" => SignInfo::Positive,
        "sq" | "sq_abs" => SignInfo::NonNegative,
        _ => SignInfo::SignUnknown,
    }
}
#[cfg(test)]
mod positivity_ext_tests {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_poly_constant_nonneg() {
        let p = PolyF64::poly_constant(5.0);
        assert_eq!(p.is_nonneg_on_reals(), Some(true));
    }
    #[test]
    fn test_poly_constant_neg() {
        let p = PolyF64::poly_constant(-1.0);
        assert_eq!(p.is_nonneg_on_reals(), Some(false));
    }
    #[test]
    fn test_poly_even_degree_pos() {
        let p = PolyF64::new(vec![1.0, 0.0, 1.0]);
        assert_eq!(p.is_nonneg_on_reals(), Some(true));
    }
    #[test]
    fn test_poly_odd_degree() {
        let p = PolyF64::new(vec![0.0, 0.0, 0.0, 1.0]);
        assert_eq!(p.is_nonneg_on_reals(), Some(false));
    }
    #[test]
    fn test_poly_eval() {
        let p = PolyF64::new(vec![1.0, 2.0, 1.0]);
        assert!((p.poly_eval(0.0) - 1.0).abs() < 1e-10);
        assert!((p.poly_eval(1.0) - 4.0).abs() < 1e-10);
        assert!((p.poly_eval(-1.0) - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_poly_degree() {
        let p = PolyF64::new(vec![1.0, 2.0, 0.0]);
        assert_eq!(p.poly_degree(), 1);
    }
    #[test]
    fn test_sos_constant() {
        let p = PolyF64::poly_constant(4.0);
        let cert = find_sos_decomposition(&p);
        assert!(cert.is_some());
    }
    #[test]
    fn test_sos_quadratic() {
        let p = PolyF64::new(vec![4.0, 0.0, 1.0]);
        let cert = find_sos_decomposition(&p);
        assert!(cert.is_some());
    }
    #[test]
    fn test_sos_neg_constant() {
        let p = PolyF64::poly_constant(-1.0);
        let cert = find_sos_decomposition(&p);
        assert!(cert.is_none());
    }
    #[test]
    fn test_nnf_tree_depth() {
        let t = NnfTree::nnf_square(NnfTree::atom("x", "hyp"));
        assert_eq!(t.nnf_depth(), 1);
    }
    #[test]
    fn test_nnf_tree_leaves() {
        let t = NnfTree::NnfSum {
            children: vec![
                NnfTree::nnf_const(1.0),
                NnfTree::nnf_const(2.0),
                NnfTree::atom("x", "x>=0"),
            ],
        };
        assert_eq!(t.num_leaves(), 3);
    }
    #[test]
    fn test_positivity_system_success() {
        let mut sys = PositivitySystem::new();
        let p = PolyF64::poly_constant(9.0);
        let cert = sys.prove_nonneg_poly(&p);
        assert!(cert.is_some());
        assert_eq!(sys.direct_proofs, 1);
    }
    #[test]
    fn test_sign_info_nonneg() {
        assert!(SignInfo::Positive.is_nonneg());
        assert!(SignInfo::NonNegative.is_nonneg());
        assert!(SignInfo::SignZero.is_nonneg());
        assert!(!SignInfo::Negative.is_nonneg());
    }
    #[test]
    fn test_sign_info_negate() {
        assert_eq!(SignInfo::Positive.sign_negate(), SignInfo::Negative);
        assert_eq!(SignInfo::SignZero.sign_negate(), SignInfo::SignZero);
    }
    #[test]
    fn test_sign_info_add() {
        assert_eq!(
            SignInfo::Positive.sign_add(&SignInfo::Positive),
            SignInfo::Positive
        );
        assert_eq!(
            SignInfo::SignZero.sign_add(&SignInfo::Positive),
            SignInfo::Positive
        );
    }
    #[test]
    fn test_sign_info_mul() {
        assert_eq!(
            SignInfo::Positive.sign_mul(&SignInfo::Positive),
            SignInfo::Positive
        );
        assert_eq!(
            SignInfo::Negative.sign_mul(&SignInfo::Negative),
            SignInfo::Positive
        );
        assert_eq!(
            SignInfo::Positive.sign_mul(&SignInfo::Negative),
            SignInfo::Negative
        );
        assert_eq!(
            SignInfo::SignZero.sign_mul(&SignInfo::Positive),
            SignInfo::SignZero
        );
    }
    #[test]
    fn test_sign_of_known() {
        assert_eq!(sign_of_known("exp"), SignInfo::Positive);
        assert_eq!(sign_of_known("sq"), SignInfo::NonNegative);
    }
    #[test]
    fn test_sign_context_basic() {
        let mut ctx = SignContext::new();
        ctx.bind("x", SignInterval::new(0.0, 1.0));
        assert!(ctx.lookup("x").is_some());
    }
    #[test]
    fn test_positivity_checker_ext_basic() {
        let checker = PositivityCheckerExt::new();
        assert!(checker.ctx.bindings.is_empty());
    }
    #[test]
    fn test_posstellensatz_cert() {
        let mut cert = PosstellensatzCert::new("p(x)", "sos");
        cert.add_inequality("x >= 0");
        assert!(cert.is_sos_cert());
        assert_eq!(cert.inequalities.len(), 1);
    }
}
#[cfg(test)]
mod tacticpositivity_analysis_tests {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_tacticpositivity_result_ok() {
        let r = TacticPositivityResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpositivity_result_err() {
        let r = TacticPositivityResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpositivity_result_partial() {
        let r = TacticPositivityResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticpositivity_result_skipped() {
        let r = TacticPositivityResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticpositivity_analysis_pass_run() {
        let mut p = TacticPositivityAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticpositivity_analysis_pass_empty_input() {
        let mut p = TacticPositivityAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticpositivity_analysis_pass_success_rate() {
        let mut p = TacticPositivityAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticpositivity_analysis_pass_disable() {
        let mut p = TacticPositivityAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticpositivity_pipeline_basic() {
        let mut pipeline = TacticPositivityPipeline::new("main_pipeline");
        pipeline.add_pass(TacticPositivityAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticPositivityAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticpositivity_pipeline_disabled_pass() {
        let mut pipeline = TacticPositivityPipeline::new("partial");
        let mut p = TacticPositivityAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticPositivityAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticpositivity_diff_basic() {
        let mut d = TacticPositivityDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticpositivity_diff_summary() {
        let mut d = TacticPositivityDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticpositivity_config_set_get() {
        let mut cfg = TacticPositivityConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticpositivity_config_read_only() {
        let mut cfg = TacticPositivityConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticpositivity_config_remove() {
        let mut cfg = TacticPositivityConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticpositivity_diagnostics_basic() {
        let mut diag = TacticPositivityDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticpositivity_diagnostics_max_errors() {
        let mut diag = TacticPositivityDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticpositivity_diagnostics_clear() {
        let mut diag = TacticPositivityDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticpositivity_config_value_types() {
        let b = TacticPositivityConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticPositivityConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticPositivityConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticPositivityConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticPositivityConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod positivity_ext_tests_100 {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_positivity_ext_result_ok_100() {
        let r = PositivityExtResult100::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_err_100() {
        let r = PositivityExtResult100::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_partial_100() {
        let r = PositivityExtResult100::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_skipped_100() {
        let r = PositivityExtResult100::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_positivity_ext_pass_run_100() {
        let mut p = PositivityExtPass100::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_positivity_ext_pass_empty_100() {
        let mut p = PositivityExtPass100::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_positivity_ext_pass_rate_100() {
        let mut p = PositivityExtPass100::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_positivity_ext_pass_disable_100() {
        let mut p = PositivityExtPass100::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_positivity_ext_pipeline_basic_100() {
        let mut pipeline = PositivityExtPipeline100::new("main_pipeline");
        pipeline.add_pass(PositivityExtPass100::new("pass1"));
        pipeline.add_pass(PositivityExtPass100::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_positivity_ext_pipeline_disabled_100() {
        let mut pipeline = PositivityExtPipeline100::new("partial");
        let mut p = PositivityExtPass100::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PositivityExtPass100::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_positivity_ext_diff_basic_100() {
        let mut d = PositivityExtDiff100::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_positivity_ext_config_set_get_100() {
        let mut cfg = PositivityExtConfig100::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_positivity_ext_config_read_only_100() {
        let mut cfg = PositivityExtConfig100::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_positivity_ext_config_remove_100() {
        let mut cfg = PositivityExtConfig100::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_positivity_ext_diagnostics_basic_100() {
        let mut diag = PositivityExtDiag100::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_positivity_ext_diagnostics_max_errors_100() {
        let mut diag = PositivityExtDiag100::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_positivity_ext_diagnostics_clear_100() {
        let mut diag = PositivityExtDiag100::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_positivity_ext_config_value_types_100() {
        let b = PositivityExtConfigVal100::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PositivityExtConfigVal100::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PositivityExtConfigVal100::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PositivityExtConfigVal100::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PositivityExtConfigVal100::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod positivity_ext_tests_101 {
    use super::*;
    use crate::tactic::positivity::*;
    #[test]
    fn test_positivity_ext_result_ok_100() {
        let r = PositivityExtResult101::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_err_100() {
        let r = PositivityExtResult101::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_partial_100() {
        let r = PositivityExtResult101::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_positivity_ext_result_skipped_100() {
        let r = PositivityExtResult101::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_positivity_ext_pass_run_100() {
        let mut p = PositivityExtPass101::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_positivity_ext_pass_empty_100() {
        let mut p = PositivityExtPass101::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_positivity_ext_pass_rate_100() {
        let mut p = PositivityExtPass101::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_positivity_ext_pass_disable_100() {
        let mut p = PositivityExtPass101::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_positivity_ext_pipeline_basic_100() {
        let mut pipeline = PositivityExtPipeline101::new("main_pipeline");
        pipeline.add_pass(PositivityExtPass101::new("pass1"));
        pipeline.add_pass(PositivityExtPass101::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_positivity_ext_pipeline_disabled_100() {
        let mut pipeline = PositivityExtPipeline101::new("partial");
        let mut p = PositivityExtPass101::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PositivityExtPass101::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_positivity_ext_diff_basic_100() {
        let mut d = PositivityExtDiff101::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_positivity_ext_config_set_get_100() {
        let mut cfg = PositivityExtConfig101::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_positivity_ext_config_read_only_100() {
        let mut cfg = PositivityExtConfig101::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_positivity_ext_config_remove_100() {
        let mut cfg = PositivityExtConfig101::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_positivity_ext_diagnostics_basic_100() {
        let mut diag = PositivityExtDiag101::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_positivity_ext_diagnostics_max_errors_100() {
        let mut diag = PositivityExtDiag101::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_positivity_ext_diagnostics_clear_100() {
        let mut diag = PositivityExtDiag101::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_positivity_ext_config_value_types_100() {
        let b = PositivityExtConfigVal101::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PositivityExtConfigVal101::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PositivityExtConfigVal101::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PositivityExtConfigVal101::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PositivityExtConfigVal101::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
