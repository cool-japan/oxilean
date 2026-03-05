//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AbstractAlarm, AbstractCmp, AbstractInterpreter, AbstractState, AbstractTrace, AlarmCollector,
    AlarmSeverity, AnalysisConfig, AnalysisResults, BlockReachability, CallGraph, CallGraphNode,
    ChaoticIterator, CongruenceDomain, CostBound, DepthDomain, FixpointEngine, FunctionSummary,
    InterpretationSummary, Interval, IntervalEnv, IntervalParityProduct, NullnessDomain,
    ParityDomain, PowersetDomain, ReachabilityAnalysis, SignDomain, SimpleAbstractInterpreter,
    SizeDomain, SummaryDatabase, TerminationEvidence, TransferEffect, TransferFunction,
    TrileanDomain,
};

/// A lattice-based abstract domain for static analysis.
pub trait AbstractDomain: Sized {
    /// Least upper bound (join) of two elements.
    fn join(&self, other: &Self) -> Self;
    /// Greatest lower bound (meet) of two elements.
    fn meet(&self, other: &Self) -> Self;
    /// True if this element is bottom (least element).
    fn is_bottom(&self) -> bool;
    /// True if this element is top (greatest element).
    fn is_top(&self) -> bool;
    /// True if `self ⊑ other`.
    fn leq(&self, other: &Self) -> bool;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sign_domain_join() {
        assert_eq!(SignDomain::Pos.join(&SignDomain::Zero), SignDomain::NonNeg);
        assert_eq!(SignDomain::Neg.join(&SignDomain::Pos), SignDomain::Nonzero);
        assert_eq!(SignDomain::Bottom.join(&SignDomain::Pos), SignDomain::Pos);
        assert_eq!(SignDomain::Top.join(&SignDomain::Neg), SignDomain::Top);
    }
    #[test]
    fn test_sign_domain_leq() {
        assert!(SignDomain::Bottom.leq(&SignDomain::Pos));
        assert!(SignDomain::Pos.leq(&SignDomain::NonNeg));
        assert!(SignDomain::NonNeg.leq(&SignDomain::Top));
        assert!(!SignDomain::Pos.leq(&SignDomain::Neg));
    }
    #[test]
    fn test_sign_negate() {
        assert_eq!(SignDomain::Pos.negate(), SignDomain::Neg);
        assert_eq!(SignDomain::Neg.negate(), SignDomain::Pos);
        assert_eq!(SignDomain::Zero.negate(), SignDomain::Zero);
        assert_eq!(SignDomain::NonNeg.negate(), SignDomain::NonPos);
        assert_eq!(SignDomain::Top.negate(), SignDomain::Top);
        assert_eq!(SignDomain::Bottom.negate(), SignDomain::Bottom);
    }
    #[test]
    fn test_depth_domain_increase() {
        let d = DepthDomain::new(10);
        assert_eq!(d.current_depth, 0);
        let d2 = d.increase();
        assert_eq!(d2.current_depth, 1);
        assert!(d2.is_bounded());
    }
    #[test]
    fn test_size_domain_add() {
        let a = SizeDomain::Small(3);
        let b = SizeDomain::Small(4);
        assert_eq!(SizeDomain::add(a, b), SizeDomain::Small(7));
        let c = SizeDomain::Zero;
        assert_eq!(
            SizeDomain::add(c, SizeDomain::Small(5)),
            SizeDomain::Small(5)
        );
        assert_eq!(
            SizeDomain::add(SizeDomain::Bottom, SizeDomain::Large),
            SizeDomain::Bottom
        );
    }
    #[test]
    fn test_abstract_state_join() {
        let s1 = AbstractState {
            sign: SignDomain::Pos,
            depth: DepthDomain::new(10),
            size: SizeDomain::Small(5),
        };
        let s2 = AbstractState {
            sign: SignDomain::Neg,
            depth: DepthDomain {
                max_depth: 10,
                current_depth: 3,
            },
            size: SizeDomain::Small(8),
        };
        let joined = s1.join(&s2);
        assert_eq!(joined.sign, SignDomain::Nonzero);
        assert_eq!(joined.depth.current_depth, 3);
        assert_eq!(joined.size, SizeDomain::Small(8));
    }
    #[test]
    fn test_interpreter_analyze_depth() {
        let interp = AbstractInterpreter::new(100);
        let d = interp.analyze_depth("((a b) (c (d e)))");
        assert_eq!(d.current_depth, 3);
        assert!(d.is_bounded());
    }
    #[test]
    fn test_fixed_point_convergence() {
        let interp = AbstractInterpreter::new(100);
        let result = interp.fixed_point(42u64, |x| *x);
        assert_eq!(result, 42);
        let result2 = interp.fixed_point(0u64, |x| (*x + 1).min(10));
        assert_eq!(result2, 10);
    }
}
/// Widening operator for interval environments.
#[allow(dead_code)]
pub fn widen_env(old: &IntervalEnv, new: &IntervalEnv) -> IntervalEnv {
    let mut result = IntervalEnv::new();
    for (name, old_iv) in &old.bindings {
        let new_iv = new.get(name);
        let lo = if new_iv.lo < old_iv.lo {
            i64::MIN
        } else {
            old_iv.lo
        };
        let hi = if new_iv.hi > old_iv.hi {
            i64::MAX
        } else {
            old_iv.hi
        };
        result.set(name, Interval { lo, hi });
    }
    result
}
/// Narrowing operator for interval environments.
#[allow(dead_code)]
pub fn narrow_env(wide: &IntervalEnv, precise: &IntervalEnv) -> IntervalEnv {
    let mut result = IntervalEnv::new();
    for (name, wide_iv) in &wide.bindings {
        let prec_iv = precise.get(name);
        let lo = if wide_iv.lo == i64::MIN {
            prec_iv.lo
        } else {
            wide_iv.lo
        };
        let hi = if wide_iv.hi == i64::MAX {
            prec_iv.hi
        } else {
            wide_iv.hi
        };
        result.set(name, Interval { lo, hi });
    }
    result
}
#[cfg(test)]
mod tests_abstract_extended {
    use super::*;
    #[test]
    fn test_interval_basics() {
        let iv = Interval::new(3, 7);
        assert!(iv.contains(5));
        assert!(!iv.contains(2));
        assert_eq!(iv.width(), 5);
        let bottom = Interval::bottom();
        assert!(bottom.is_bottom());
        let top = Interval::top();
        assert!(top.is_top());
    }
    #[test]
    fn test_interval_join_meet() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 10);
        let joined = a.join(&b);
        assert_eq!(joined.lo, 1);
        assert_eq!(joined.hi, 10);
        let met = a.meet(&b);
        assert_eq!(met.lo, 3);
        assert_eq!(met.hi, 5);
    }
    #[test]
    fn test_interval_arithmetic() {
        let a = Interval::new(1, 5);
        let b = Interval::new(2, 3);
        let sum = a.add(&b);
        assert_eq!(sum.lo, 3);
        assert_eq!(sum.hi, 8);
        let diff = a.sub(&b);
        assert_eq!(diff.lo, -2);
        assert_eq!(diff.hi, 3);
    }
    #[test]
    fn test_parity_domain_join() {
        use ParityDomain::*;
        assert_eq!(Even.join(&Even), Even);
        assert_eq!(Even.join(&Odd), Top);
        assert_eq!(Bottom.join(&Odd), Odd);
    }
    #[test]
    fn test_parity_domain_arithmetic() {
        use ParityDomain::*;
        assert_eq!(Even.add(&Odd), Odd);
        assert_eq!(Odd.add(&Odd), Even);
        assert_eq!(Even.mul(&Odd), Even);
        assert_eq!(Odd.mul(&Odd), Odd);
    }
    #[test]
    fn test_nullness_domain() {
        use NullnessDomain::*;
        assert!(Null.may_be_null());
        assert!(Top.may_be_null());
        assert!(!NonNull.may_be_null());
        assert_eq!(Null.join(&NonNull), Top);
        assert!(NonNull.is_definitely_non_null());
    }
    #[test]
    fn test_interval_env() {
        let mut env = IntervalEnv::new();
        env.set("x", Interval::new(0, 10));
        env.set("y", Interval::new(5, 15));
        let x = env.get("x");
        assert_eq!(x.lo, 0);
        assert_eq!(x.hi, 10);
        let mut env2 = IntervalEnv::new();
        env2.set("x", Interval::new(3, 20));
        env2.set("z", Interval::new(1, 1));
        let joined = env.join(&env2);
        let xj = joined.get("x");
        assert_eq!(xj.lo, 0);
        assert_eq!(xj.hi, 20);
    }
    #[test]
    fn test_fixpoint_engine() {
        let mut engine = FixpointEngine::new(10);
        assert!(!engine.is_exhausted());
        for _ in 0..10 {
            engine.step();
        }
        assert!(engine.is_exhausted());
        engine.reset();
        assert_eq!(engine.iterations(), 0);
    }
    #[test]
    fn test_fixpoint_detection() {
        let mut env1 = IntervalEnv::new();
        env1.set("x", Interval::new(0, 10));
        let mut env2 = IntervalEnv::new();
        env2.set("x", Interval::new(0, 10));
        assert!(FixpointEngine::is_fixpoint(&env1, &env2));
        env2.set("x", Interval::new(0, 11));
        assert!(!FixpointEngine::is_fixpoint(&env1, &env2));
    }
    #[test]
    fn test_widen_env() {
        let mut old = IntervalEnv::new();
        old.set("x", Interval::new(0, 5));
        let mut new = IntervalEnv::new();
        new.set("x", Interval::new(0, 10));
        let widened = widen_env(&old, &new);
        let xw = widened.get("x");
        assert_eq!(xw.lo, 0);
        assert_eq!(xw.hi, i64::MAX);
    }
    #[test]
    fn test_narrow_env() {
        let mut wide = IntervalEnv::new();
        wide.set(
            "x",
            Interval {
                lo: i64::MIN,
                hi: i64::MAX,
            },
        );
        let mut precise = IntervalEnv::new();
        precise.set("x", Interval::new(-100, 100));
        let narrowed = narrow_env(&wide, &precise);
        let xn = narrowed.get("x");
        assert_eq!(xn.lo, -100);
        assert_eq!(xn.hi, 100);
    }
}
#[cfg(test)]
mod tests_abstract_extended2 {
    use super::*;
    #[test]
    fn test_congruence_domain_satisfies() {
        let even = CongruenceDomain::congruent(2, 0);
        assert!(even.satisfies(4));
        assert!(!even.satisfies(3));
        let top = CongruenceDomain::top();
        assert!(top.satisfies(999));
        let bottom = CongruenceDomain::bottom();
        assert!(!bottom.satisfies(0));
    }
    #[test]
    fn test_congruence_domain_join() {
        let a = CongruenceDomain::congruent(4, 0);
        let b = CongruenceDomain::congruent(4, 0);
        let joined = a.join(&b);
        assert!(!joined.is_top());
        let c = CongruenceDomain::congruent(4, 1);
        let joined2 = a.join(&c);
        assert!(joined2.is_top());
    }
    #[test]
    fn test_powerset_domain() {
        let mut s = PowersetDomain::bottom(8);
        assert!(s.is_bottom());
        s.add(0);
        s.add(3);
        s.add(7);
        assert_eq!(s.count(), 3);
        assert!(s.contains(3));
        assert!(!s.contains(2));
        s.remove(3);
        assert!(!s.contains(3));
        let t = PowersetDomain::top(8);
        assert!(t.is_top());
        assert_eq!(t.count(), 8);
    }
    #[test]
    fn test_powerset_join_meet() {
        let mut a = PowersetDomain::bottom(4);
        a.add(0);
        a.add(1);
        let mut b = PowersetDomain::bottom(4);
        b.add(1);
        b.add(2);
        let joined = a.join(&b);
        assert!(joined.contains(0) && joined.contains(1) && joined.contains(2));
        let met = a.meet(&b);
        assert!(met.contains(1));
        assert!(!met.contains(0));
        assert!(!met.contains(2));
    }
    #[test]
    fn test_chaotic_iterator() {
        let mut iter = ChaoticIterator::new(5);
        for _ in 0..3 {
            assert!(iter.advance());
        }
        iter.mark_converged();
        assert!(iter.is_converged());
        assert!(!iter.is_limit_exceeded());
        assert_eq!(iter.steps(), 3);
    }
    #[test]
    fn test_chaotic_iterator_limit() {
        let mut iter = ChaoticIterator::new(3);
        for _ in 0..3 {
            iter.advance();
        }
        assert!(!iter.advance());
        assert!(iter.is_limit_exceeded());
    }
    #[test]
    fn test_abstract_trace() {
        let mut trace = AbstractTrace::new();
        trace.record("loop_head", Interval::new(0, 10));
        trace.record("loop_body", Interval::new(1, 10));
        assert_eq!(trace.at("loop_head"), Some(Interval::new(0, 10)));
        assert_eq!(trace.at("exit"), None);
        let fmt = trace.format();
        assert!(fmt.contains("loop_head: [0, 10]"));
    }
    #[test]
    fn test_alarm_collector() {
        let mut ac = AlarmCollector::new();
        ac.add(AbstractAlarm::new(
            "L1",
            "possible null",
            AlarmSeverity::Warning,
        ));
        ac.add(AbstractAlarm::new(
            "L2",
            "division by zero",
            AlarmSeverity::Error,
        ));
        ac.add(AbstractAlarm::new("L3", "info msg", AlarmSeverity::Info));
        assert!(ac.has_errors());
        assert_eq!(ac.errors().len(), 1);
        let (info, warn, err) = ac.count_by_severity();
        assert_eq!((info, warn, err), (1, 1, 1));
    }
}
#[cfg(test)]
mod tests_abstract_extended3 {
    use super::*;
    #[test]
    fn test_call_graph() {
        let mut g = CallGraph::new();
        let mut f = CallGraphNode::new("fact");
        f.add_callee("fact");
        f.add_callee("Nat.sub");
        g.add_node(f);
        let mut h = CallGraphNode::new("helper");
        h.add_callee("fact");
        g.add_node(h);
        assert!(
            g.find("fact")
                .expect("value should be present")
                .is_recursive
        );
        let recursive = g.recursive_fns();
        assert!(recursive.contains(&"fact"));
        let callers = g.callers_of("fact");
        assert!(callers.contains(&"helper"));
        assert!(callers.contains(&"fact"));
    }
    #[test]
    fn test_reachability_analysis() {
        let mut ra = ReachabilityAnalysis::new();
        ra.mark_reachable("entry");
        ra.mark_reachable("loop");
        ra.mark_unreachable("dead_code");
        assert!(ra.is_reachable("entry"));
        assert!(ra.is_unreachable("dead_code"));
        assert!(!ra.is_reachable("unknown_label"));
        assert_eq!(ra.reachable_count(), 2);
        assert_eq!(ra.unreachable_count(), 1);
    }
    #[test]
    fn test_termination_evidence() {
        let e1 = TerminationEvidence::Structural { arg_index: 0 };
        assert!(e1.is_proven());
        assert!(e1.describe().contains("structural"));
        let e2 = TerminationEvidence::Lexicographic {
            measures: vec!["n".to_string(), "m".to_string()],
        };
        assert!(e2.describe().contains("lexicographic"));
        let e3 = TerminationEvidence::Unknown;
        assert!(!e3.is_proven());
    }
    #[test]
    fn test_cost_bound() {
        let exact = CostBound::exact(10);
        assert!(exact.is_bounded());
        assert_eq!(exact.width(), Some(0));
        let range = CostBound::range(5, 20);
        assert_eq!(range.width(), Some(15));
        let open = CostBound::at_least(3);
        assert!(!open.is_bounded());
        let sum = exact.add(&range);
        assert_eq!(sum.lower, 15);
        assert_eq!(sum.upper, Some(30));
        let sum2 = exact.add(&open);
        assert!(!sum2.is_bounded());
    }
}
#[cfg(test)]
mod tests_abstract_extended4 {
    use super::*;
    #[test]
    fn test_transfer_function_apply() {
        let mut env = IntervalEnv::new();
        env.set("x", Interval::new(0, 10));
        env.set("y", Interval::new(5, 15));
        let mut tf = TransferFunction::new("incr_x");
        tf.add_effect(TransferEffect::Assign {
            var: "x".to_string(),
            interval: Interval::new(1, 11),
        });
        tf.add_effect(TransferEffect::Invalidate {
            var: "y".to_string(),
        });
        let new_env = tf.apply(&env);
        let x = new_env.get("x");
        assert_eq!(x.lo, 1);
        assert_eq!(x.hi, 11);
        let y = new_env.get("y");
        assert!(y.is_top());
    }
    #[test]
    fn test_transfer_function_constrain() {
        let mut env = IntervalEnv::new();
        env.set("n", Interval::new(0, 100));
        let mut tf = TransferFunction::new("guard_n_gt_50");
        tf.add_effect(TransferEffect::Constrain {
            var: "n".to_string(),
            constraint: Interval::new(51, i64::MAX),
        });
        let new_env = tf.apply(&env);
        let n = new_env.get("n");
        assert_eq!(n.lo, 51);
        assert_eq!(n.hi, 100);
    }
    #[test]
    fn test_function_summary() {
        let mut summary = FunctionSummary::new("fact");
        summary.termination = TerminationEvidence::Structural { arg_index: 0 };
        summary.cost = CostBound::range(1, 1000);
        assert!(summary.terminates());
        let desc = summary.describe();
        assert!(desc.contains("fact"));
        assert!(desc.contains("terminates=true"));
    }
    #[test]
    fn test_summary_database() {
        let mut db = SummaryDatabase::new();
        let mut s1 = FunctionSummary::new("f");
        s1.termination = TerminationEvidence::Structural { arg_index: 0 };
        db.add(s1);
        let mut s2 = FunctionSummary::new("g");
        s2.termination = TerminationEvidence::Unknown;
        db.add(s2);
        let proven = db.proven_terminating();
        assert!(proven.contains(&"f"));
        assert!(!proven.contains(&"g"));
        assert_eq!(db.len(), 2);
    }
}
#[cfg(test)]
mod tests_abstract_extended5 {
    use super::*;
    #[test]
    fn test_trilean_and_or_not() {
        use TrileanDomain::*;
        assert_eq!(True.and(&False), False);
        assert_eq!(True.and(&Top), Top);
        assert_eq!(False.or(&True), True);
        assert_eq!(False.or(&Top), Top);
        assert_eq!(True.not(), False);
        assert_eq!(Top.not(), Top);
    }
    #[test]
    fn test_trilean_join() {
        use TrileanDomain::*;
        assert_eq!(True.join(&False), Top);
        assert_eq!(True.join(&True), True);
        assert_eq!(False.join(&Bottom), False);
    }
    #[test]
    fn test_block_reachability() {
        use BlockReachability::*;
        assert_eq!(Unreachable.join(&Reachable), Reachable);
        assert_eq!(Unreachable.join(&Unknown), Unknown);
        assert!(!Unreachable.may_be_reachable());
        assert!(Unknown.may_be_reachable());
        assert!(Reachable.may_be_reachable());
    }
    #[test]
    fn test_analysis_config() {
        let cfg = AnalysisConfig::default_config();
        assert_eq!(cfg.max_iterations, 100);
        assert!(cfg.use_widening);
        assert!(cfg.collect_alarms);
        let fast = AnalysisConfig::fast();
        assert_eq!(fast.max_iterations, 10);
        assert!(!fast.collect_alarms);
    }
}
#[cfg(test)]
mod tests_abstract_product {
    use super::*;
    #[test]
    fn test_interval_parity_product_basics() {
        let p = IntervalParityProduct::from_value(4);
        assert_eq!(p.interval, Interval::new(4, 4));
        assert_eq!(p.parity, ParityDomain::Even);
        let p2 = IntervalParityProduct::from_value(7);
        assert_eq!(p2.parity, ParityDomain::Odd);
        let joined = p.join(&p2);
        assert_eq!(joined.interval, Interval::new(4, 7));
        assert_eq!(joined.parity, ParityDomain::Top);
    }
    #[test]
    fn test_interval_parity_add() {
        let even = IntervalParityProduct::from_value(4);
        let odd = IntervalParityProduct::from_value(3);
        let sum = even.add(&odd);
        assert_eq!(sum.parity, ParityDomain::Odd);
        assert_eq!(sum.interval.lo, 7);
        assert_eq!(sum.interval.hi, 7);
    }
    #[test]
    fn test_analysis_results() {
        let mut results = AnalysisResults::new();
        results.set("x", IntervalParityProduct::from_value(4));
        results.set("y", IntervalParityProduct::from_value(3));
        results.set(
            "z",
            IntervalParityProduct::new(Interval::new(0, 100), ParityDomain::Even),
        );
        let non_neg = results.proven_non_negative();
        assert!(non_neg.contains(&"x") && non_neg.contains(&"y") && non_neg.contains(&"z"));
        let even_vars = results.proven_even();
        assert!(even_vars.contains(&"x"));
        assert!(even_vars.contains(&"z"));
        assert!(!even_vars.contains(&"y"));
    }
}
#[cfg(test)]
mod tests_abstract_interp_entry {
    use super::*;
    #[test]
    fn test_interpretation_summary() {
        let s = InterpretationSummary::new(5, true, 0);
        assert!(s.proven_safe);
        assert!(s.describe().contains("safe=true"));
        let s2 = InterpretationSummary::new(5, true, 3);
        assert!(!s2.proven_safe);
    }
    #[test]
    fn test_simple_abstract_interpreter() {
        let cfg = AnalysisConfig::fast();
        let mut interp = SimpleAbstractInterpreter::new(cfg);
        interp.init_var("n", IntervalParityProduct::from_value(10));
        let summary = interp.run_stub();
        assert!(summary.converged);
        let r = interp
            .results()
            .get("n")
            .expect("element at \'n\' should exist");
        assert_eq!(r.interval.lo, 10);
    }
}
#[cfg(test)]
mod tests_abstract_cmp {
    use super::*;
    #[test]
    fn test_abstract_lt() {
        let a = Interval::new(1, 5);
        let b = Interval::new(10, 20);
        assert_eq!(AbstractCmp::lt(&a, &b), AbstractCmp::DefinitelyTrue);
        let c = Interval::new(15, 25);
        assert_eq!(AbstractCmp::lt(&c, &b), AbstractCmp::Unknown);
        let d = Interval::new(20, 30);
        assert_eq!(AbstractCmp::lt(&d, &b), AbstractCmp::DefinitelyFalse);
    }
    #[test]
    fn test_abstract_eq() {
        let a = Interval::new(5, 5);
        let b = Interval::new(5, 5);
        assert_eq!(AbstractCmp::eq(&a, &b), AbstractCmp::DefinitelyTrue);
        let c = Interval::new(1, 3);
        let d = Interval::new(10, 20);
        assert_eq!(AbstractCmp::eq(&c, &d), AbstractCmp::DefinitelyFalse);
        let e = Interval::new(1, 10);
        assert_eq!(AbstractCmp::eq(&a, &e), AbstractCmp::Unknown);
    }
}
/// Compute the abstract division `a / b` for intervals.
/// Returns Top (full interval) if `b` contains 0.
#[allow(dead_code)]
pub fn abstract_div(a: &Interval, b: &Interval) -> Interval {
    if a.is_bottom() || b.is_bottom() {
        return Interval::bottom();
    }
    if b.contains(0) {
        return Interval::top();
    }
    let combos = [a.lo / b.lo, a.lo / b.hi, a.hi / b.lo, a.hi / b.hi];
    let lo = *combos
        .iter()
        .min()
        .expect("combos iterator must be non-empty");
    let hi = *combos
        .iter()
        .max()
        .expect("combos iterator must be non-empty");
    Interval::new(lo, hi)
}
#[cfg(test)]
mod tests_abstract_div {
    use super::*;
    #[test]
    fn test_abstract_div_basic() {
        let a = Interval::new(6, 12);
        let b = Interval::new(2, 3);
        let result = abstract_div(&a, &b);
        assert!(result.lo >= 2 && result.hi <= 6);
    }
    #[test]
    fn test_abstract_div_by_zero() {
        let a = Interval::new(1, 10);
        let b = Interval::new(-1, 1);
        let result = abstract_div(&a, &b);
        assert!(result.is_top());
    }
}
