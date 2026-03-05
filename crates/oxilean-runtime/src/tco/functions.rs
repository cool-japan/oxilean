//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BinopKind, BoundedTrampoline, ContinuationEvaluator, ContinuationFrame, EvaluationContext,
    ExplicitCallStack, ExtendedTailCallDetector, FunctionTcoMetrics, InliningDecision,
    InliningThreshold, LoopDetector, MultiStep, MutualTailCall, PartialValue, PeepholeOptimizer,
    PeepholeRule, RecursiveStep, RewriteRule, StackFrame, StateMachineState, StepResult, TailCall,
    TailCallAnalysisReport, TailCallBenchmarkResult, TailCallChain, TailCallCounter,
    TailCallDetector, TailCallOptimizationPass, TailCallOptimizer, TailCallProof,
    TailCallScheduler, TailCallSchedulerConfig, TailPositionKind, TcoStatistics,
    TrampolineMetricsRegistry, UnrollConfig, UnrollResult,
};

/// Drive a trampoline loop until `Done`, returning the final value.
///
/// This function is the main entry point for TCO-ed computations.
/// It runs in *O(1)* stack space regardless of how many `Call` steps are
/// returned.
pub fn trampoline<T>(mut step: TailCall<T>) -> T {
    loop {
        match step {
            TailCall::Done(v) => return v,
            TailCall::Call(f) => step = f(),
        }
    }
}
/// Drive a trampoline loop and record statistics.
///
/// Returns `(result, counter)`.
pub fn trampoline_instrumented<T>(mut step: TailCall<T>) -> (T, TailCallCounter) {
    let mut counter = TailCallCounter::new();
    let mut depth = 0u64;
    loop {
        match step {
            TailCall::Done(v) => return (v, counter),
            TailCall::Call(f) => {
                depth += 1;
                counter.record(depth);
                step = f();
            }
        }
    }
}
/// Run a step-function interpreter with tail call optimization.
///
/// `initial_state` is the starting state. `step` maps the current state to
/// either a new state (continue), a final output, or an error. The loop runs
/// in O(1) stack depth.
pub fn run_tco_interpreter<State, Output, F>(
    initial_state: State,
    step: F,
) -> Result<Output, String>
where
    F: Fn(State) -> StepResult<State, Output>,
{
    let mut state = initial_state;
    loop {
        match step(state) {
            StepResult::Continue(next) => state = next,
            StepResult::Finished(out) => return Ok(out),
            StepResult::Error(e) => return Err(e),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fact_step(n: u64, acc: u64) -> TailCall<u64> {
        if n == 0 {
            TailCall::Done(acc)
        } else {
            TailCall::Call(Box::new(move || fact_step(n - 1, n * acc)))
        }
    }
    #[test]
    fn test_trampoline_factorial() {
        assert_eq!(trampoline(fact_step(0, 1)), 1);
        assert_eq!(trampoline(fact_step(1, 1)), 1);
        assert_eq!(trampoline(fact_step(5, 1)), 120);
        assert_eq!(trampoline(fact_step(10, 1)), 3628800);
    }
    #[test]
    fn test_trampoline_done_immediately() {
        let result = trampoline(TailCall::Done(42u64));
        assert_eq!(result, 42);
    }
    #[test]
    fn test_trampoline_instrumented_counts_steps() {
        let (result, stats) = trampoline_instrumented(fact_step(5, 1));
        assert_eq!(result, 120);
        assert_eq!(stats.optimized, 5);
        assert_eq!(stats.max_depth, 5);
    }
    #[test]
    fn test_trampoline_instrumented_no_calls() {
        let (result, stats) = trampoline_instrumented(TailCall::Done(7u64));
        assert_eq!(result, 7);
        assert_eq!(stats.optimized, 0);
    }
    #[test]
    fn test_counter_record() {
        let mut c = TailCallCounter::new();
        c.record(3);
        c.record(5);
        c.record(2);
        assert_eq!(c.optimized, 3);
        assert_eq!(c.max_depth, 5);
    }
    #[test]
    fn test_detector_identifies_tail_calls() {
        let opcodes = ["Push", "Call", "Return", "Halt"];
        let mut det = TailCallDetector::new();
        det.analyse(&opcodes);
        assert!(det.is_tail(1));
        assert_eq!(det.count(), 1);
    }
    #[test]
    fn test_detector_non_tail_call() {
        let opcodes = ["Push", "Call", "Add", "Return", "Halt"];
        let mut det = TailCallDetector::new();
        det.analyse(&opcodes);
        assert!(!det.is_tail(1));
        assert_eq!(det.count(), 0);
    }
    #[test]
    fn test_detector_no_calls() {
        let opcodes = ["Push", "Push", "Add", "Return"];
        let mut det = TailCallDetector::new();
        det.analyse(&opcodes);
        assert_eq!(det.count(), 0);
    }
    #[test]
    fn test_recursive_step_factorial() {
        let result = RecursiveStep::run(10u64, 1u64, |n, acc| {
            if n == 0 {
                None
            } else {
                Some((n - 1, n * acc))
            }
        });
        assert_eq!(result, 3628800);
    }
    #[test]
    fn test_recursive_step_sum() {
        let result = RecursiveStep::run(100u64, 0u64, |n, acc| {
            if n == 0 {
                None
            } else {
                Some((n - 1, acc + n))
            }
        });
        assert_eq!(result, 5050);
    }
    #[test]
    fn test_recursive_step_zero() {
        let result = RecursiveStep::run(0u64, 42u64, |n, acc| {
            if n == 0 {
                None
            } else {
                Some((n - 1, acc + 1))
            }
        });
        assert_eq!(result, 42);
    }
    #[test]
    fn test_run_tco_interpreter_countdown() {
        let result = run_tco_interpreter(10u64, |n| {
            if n == 0 {
                StepResult::Finished(0u64)
            } else {
                StepResult::Continue(n - 1)
            }
        });
        assert_eq!(result, Ok(0));
    }
    #[test]
    fn test_run_tco_interpreter_error() {
        let result: Result<u64, String> = run_tco_interpreter(5u64, |n| {
            if n == 3 {
                StepResult::Error("hit 3".to_string())
            } else {
                StepResult::Continue(n - 1)
            }
        });
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "hit 3");
    }
}
/// A continuation: a function that accepts a value and produces a result.
#[allow(dead_code)]
pub type Cont<A, R> = Box<dyn FnOnce(A) -> R>;
/// CPS-transformed computation: given a continuation, produce the final result.
#[allow(dead_code)]
pub type Cps<A, R> = Box<dyn FnOnce(Cont<A, R>) -> R>;
/// Build a CPS computation that immediately applies its continuation to `value`.
#[allow(dead_code)]
pub fn cps_pure<A: 'static, R: 'static>(value: A) -> Cps<A, R> {
    Box::new(move |k: Cont<A, R>| k(value))
}
/// Sequence two CPS computations: run `ma`, pass the result to `f`, run the result.
#[allow(dead_code)]
pub fn cps_bind<A: 'static, B: 'static, R: 'static>(
    ma: Cps<A, R>,
    f: impl FnOnce(A) -> Cps<B, R> + 'static,
) -> Cps<B, R> {
    Box::new(move |k: Cont<B, R>| {
        ma(Box::new(move |a: A| {
            let mb = f(a);
            mb(k)
        }))
    })
}
/// Drive a mutually-recursive trampoline to completion.
#[allow(dead_code)]
pub fn mutual_trampoline<A, B, R>(mut step: MutualTailCall<A, B, R>) -> R {
    loop {
        match step {
            MutualTailCall::Done(r) => return r,
            MutualTailCall::GoA(a, f) => step = f(a),
            MutualTailCall::GoB(b, g) => step = g(b),
        }
    }
}
/// Compute the sum of `0..n` via TCO trampoline.
#[allow(dead_code)]
pub fn tco_sum(n: u64) -> u64 {
    RecursiveStep::run(
        n,
        0u64,
        |i, acc| {
            if i == 0 {
                None
            } else {
                Some((i - 1, acc + i))
            }
        },
    )
}
/// Compute `n!` via TCO trampoline.
#[allow(dead_code)]
pub fn tco_factorial(n: u64) -> u64 {
    RecursiveStep::run(
        n,
        1u64,
        |i, acc| {
            if i == 0 {
                None
            } else {
                Some((i - 1, i * acc))
            }
        },
    )
}
/// Compute the `n`-th Fibonacci number via TCO with two-accumulator technique.
#[allow(dead_code)]
pub fn tco_fibonacci(n: u64) -> u64 {
    fn fib_step(n: u64, a: u64, b: u64) -> TailCall<u64> {
        if n == 0 {
            TailCall::Done(a)
        } else {
            TailCall::Call(Box::new(move || fib_step(n - 1, b, a + b)))
        }
    }
    trampoline(fib_step(n, 0, 1))
}
/// Compute the n-th triangular number via TCO.
#[allow(dead_code)]
pub fn tco_triangular(n: u64) -> u64 {
    tco_sum(n)
}
/// Run a simple benchmark of `tco_factorial` for the given `n`.
#[allow(dead_code)]
pub fn bench_tco_factorial(n: u64, iterations: u64) -> TailCallBenchmarkResult {
    let mut _last = 0u64;
    for _ in 0..iterations {
        _last = tco_factorial(n);
    }
    TailCallBenchmarkResult {
        name: format!("tco_factorial({})", n),
        iterations,
        total_ns: 0,
        value: _last,
    }
}
/// Run a simple benchmark of `tco_fibonacci` for the given `n`.
#[allow(dead_code)]
pub fn bench_tco_fibonacci(n: u64, iterations: u64) -> TailCallBenchmarkResult {
    let mut _last = 0u64;
    for _ in 0..iterations {
        _last = tco_fibonacci(n);
    }
    TailCallBenchmarkResult {
        name: format!("tco_fibonacci({})", n),
        iterations,
        total_ns: 0,
        value: _last,
    }
}
/// Drive a two-function mutual trampoline.
///
/// `even_fn` handles `MultiStep::Even`, `odd_fn` handles `MultiStep::Odd`.
#[allow(dead_code)]
pub fn multi_trampoline<State, Output, EF, OF>(
    init: MultiStep<State, Output>,
    even_fn: EF,
    odd_fn: OF,
) -> Output
where
    EF: Fn(State) -> MultiStep<State, Output>,
    OF: Fn(State) -> MultiStep<State, Output>,
{
    let mut step = init;
    loop {
        match step {
            MultiStep::Done(o) => return o,
            MultiStep::Even(s) => step = even_fn(s),
            MultiStep::Odd(s) => step = odd_fn(s),
        }
    }
}
/// Example: determine parity of n using mutual tail recursion via `multi_trampoline`.
#[allow(dead_code)]
pub fn is_even_via_multi_trampoline(n: u64) -> bool {
    multi_trampoline(
        MultiStep::Even(n),
        |n| {
            if n == 0 {
                MultiStep::Done(true)
            } else {
                MultiStep::Odd(n - 1)
            }
        },
        |n| {
            if n == 0 {
                MultiStep::Done(false)
            } else {
                MultiStep::Even(n - 1)
            }
        },
    )
}
/// A transition function: given a state, return the next state or halt.
#[allow(dead_code)]
pub type TransitionFn = Box<dyn Fn(StateMachineState) -> Option<StateMachineState>>;
/// Run a state machine until the transition function returns `None`.
#[allow(dead_code)]
pub fn run_state_machine(
    mut state: StateMachineState,
    transition: &dyn Fn(StateMachineState) -> Option<StateMachineState>,
) -> StateMachineState {
    loop {
        match transition(state.clone()) {
            Some(next) => state = next,
            None => return state,
        }
    }
}
/// Decide whether to inline a callee.
#[allow(dead_code)]
pub fn decide_inlining(
    callee_size: usize,
    call_depth: usize,
    call_count: u64,
    force: bool,
    threshold: &InliningThreshold,
) -> InliningDecision {
    if force {
        return InliningDecision::ForceInline;
    }
    if callee_size > threshold.max_size || call_depth > threshold.max_depth {
        return InliningDecision::DoNotInline;
    }
    if call_count >= threshold.min_call_count {
        InliningDecision::Inline
    } else {
        InliningDecision::DoNotInline
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_tco_factorial_values() {
        assert_eq!(tco_factorial(0), 1);
        assert_eq!(tco_factorial(1), 1);
        assert_eq!(tco_factorial(5), 120);
        assert_eq!(tco_factorial(10), 3628800);
    }
    #[test]
    fn test_tco_sum_values() {
        assert_eq!(tco_sum(0), 0);
        assert_eq!(tco_sum(10), 55);
        assert_eq!(tco_sum(100), 5050);
    }
    #[test]
    fn test_tco_fibonacci_values() {
        assert_eq!(tco_fibonacci(0), 0);
        assert_eq!(tco_fibonacci(1), 1);
        assert_eq!(tco_fibonacci(7), 13);
        assert_eq!(tco_fibonacci(10), 55);
    }
    #[test]
    fn test_bounded_trampoline_success() {
        let bt = BoundedTrampoline::new(1_000_000);
        let result = bt.run(trampoline_step(100_000));
        assert_eq!(result, Ok(0));
    }
    fn trampoline_step(n: u64) -> TailCall<u64> {
        if n == 0 {
            TailCall::Done(0)
        } else {
            TailCall::Call(Box::new(move || trampoline_step(n - 1)))
        }
    }
    #[test]
    fn test_bounded_trampoline_limit_exceeded() {
        let bt = BoundedTrampoline::new(5);
        let result = bt.run(trampoline_step(100));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("step limit"));
    }
    #[test]
    fn test_explicit_call_stack() {
        let mut stack = ExplicitCallStack::new();
        assert_eq!(stack.depth(), 0);
        assert!(stack.top().is_none());
        let mut f1 = StackFrame::new("foo", 10);
        f1.bind("x", 42);
        stack.push(f1);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.max_depth, 1);
        let mut f2 = StackFrame::new("bar", 20);
        f2.bind("y", 99);
        stack.push(f2);
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.max_depth, 2);
        let top = stack.pop().expect("collection should not be empty");
        assert_eq!(top.function, "bar");
        assert_eq!(top.lookup("y"), Some(99));
        assert_eq!(top.lookup("z"), None);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.max_depth, 2);
    }
    #[test]
    fn test_stack_backtrace() {
        let mut stack = ExplicitCallStack::new();
        stack.push(StackFrame::new("main", 0));
        stack.push(StackFrame::new("foo", 5));
        stack.push(StackFrame::new("bar", 10));
        let bt = stack.backtrace();
        assert_eq!(bt, vec!["main", "foo", "bar"]);
        let fmt = stack.format_backtrace();
        assert!(fmt.contains("main"));
        assert!(fmt.contains("bar"));
    }
    #[test]
    fn test_tail_call_scheduler() {
        let cfg = TailCallSchedulerConfig {
            max_steps_per_batch: 10,
            step_limit: 1_000,
        };
        let sched = TailCallScheduler::with_config(trampoline_step(500), cfg);
        let result = sched.run_to_completion();
        assert_eq!(result, Ok(0));
    }
    #[test]
    fn test_tail_call_scheduler_step_limit() {
        let cfg = TailCallSchedulerConfig {
            max_steps_per_batch: 5,
            step_limit: 10,
        };
        let sched = TailCallScheduler::with_config(trampoline_step(1000), cfg);
        let result = sched.run_to_completion();
        assert!(result.is_err());
    }
    #[test]
    fn test_analysis_report() {
        let opcodes = ["Push", "Call", "Return", "Push", "Call", "Add", "Return"];
        let mut det = TailCallDetector::new();
        det.analyse(&opcodes);
        let report = TailCallAnalysisReport::build(&det, &opcodes);
        assert_eq!(report.total_calls, 2);
        assert_eq!(report.tail_positions.len(), 1);
        assert!((report.tail_ratio - 0.5).abs() < 1e-9);
        assert!(report.summary.contains("1/2"));
    }
    #[test]
    fn test_tail_call_optimizer() {
        let mut opt = TailCallOptimizer::new();
        let opcodes = ["Push", "Call", "Return", "Push", "Call", "Add", "Return"];
        let report = opt.analyse_chunk(&opcodes);
        assert_eq!(report.tail_positions.len(), 1);
        assert_eq!(opt.stats.optimized, 1);
    }
    #[test]
    fn test_is_even_via_multi_trampoline() {
        assert!(is_even_via_multi_trampoline(0));
        assert!(!is_even_via_multi_trampoline(1));
        assert!(is_even_via_multi_trampoline(100));
        assert!(!is_even_via_multi_trampoline(99));
    }
    #[test]
    fn test_extended_tail_call_detector() {
        let ops: Vec<(&str, Option<&str>)> = vec![
            ("Push", None),
            ("Call", Some("foo")),
            ("Return", None),
            ("Push", None),
            ("Call", Some("bar")),
            ("Return", None),
            ("Push", None),
            ("Call", Some("foo")),
            ("Add", None),
            ("Return", None),
        ];
        let mut det = ExtendedTailCallDetector::new("foo");
        det.analyse_with_callees(&ops);
        assert_eq!(det.count_kind(TailPositionKind::SelfTailCall), 1);
        assert_eq!(det.count_kind(TailPositionKind::MutualTailCall), 1);
        assert_eq!(det.count_kind(TailPositionKind::NonTail), 1);
    }
    #[test]
    fn test_peephole_optimizer() {
        let mut opt = PeepholeOptimizer::new();
        opt.add_rule(PeepholeRule::new(
            vec!["Dup", "Pop"],
            vec![],
            "eliminate dup-pop pair",
        ));
        let opcodes = vec!["Push", "Dup", "Pop", "Return"];
        let result = opt.optimize(&opcodes);
        assert_eq!(result, vec!["Push", "Return"]);
        assert_eq!(opt.rewrites, 1);
    }
    #[test]
    fn test_peephole_no_match() {
        let mut opt = PeepholeOptimizer::new();
        opt.add_rule(PeepholeRule::new(
            vec!["Dup", "Pop"],
            vec![],
            "eliminate dup-pop pair",
        ));
        let opcodes = vec!["Push", "Add", "Return"];
        let result = opt.optimize(&opcodes);
        assert_eq!(result, vec!["Push", "Add", "Return"]);
        assert_eq!(opt.rewrites, 0);
    }
    #[test]
    fn test_inlining_decision_inline() {
        let thresh = InliningThreshold::default();
        let decision = decide_inlining(10, 4, 5, false, &thresh);
        assert_eq!(decision, InliningDecision::Inline);
    }
    #[test]
    fn test_inlining_decision_too_large() {
        let thresh = InliningThreshold::default();
        let decision = decide_inlining(100, 4, 10, false, &thresh);
        assert_eq!(decision, InliningDecision::DoNotInline);
    }
    #[test]
    fn test_inlining_decision_force() {
        let thresh = InliningThreshold::default();
        let decision = decide_inlining(100, 100, 0, true, &thresh);
        assert_eq!(decision, InliningDecision::ForceInline);
    }
    #[test]
    fn test_unroll_result_full() {
        let cfg = UnrollConfig {
            factor: 4,
            full_unroll_limit: 16,
        };
        let res = UnrollResult::compute(8, &cfg);
        assert!(res.fully_unrolled);
        assert_eq!(res.factor, 8);
    }
    #[test]
    fn test_unroll_result_partial() {
        let cfg = UnrollConfig {
            factor: 4,
            full_unroll_limit: 16,
        };
        let res = UnrollResult::compute(100, &cfg);
        assert!(!res.fully_unrolled);
        assert_eq!(res.factor, 4);
    }
    #[test]
    fn test_tco_statistics() {
        let mut stats = TcoStatistics::new();
        let c1 = TailCallCounter {
            optimized: 10,
            max_depth: 10,
        };
        let c2 = TailCallCounter {
            optimized: 20,
            max_depth: 15,
        };
        stats.record_run(&c1, false);
        stats.record_run(&c2, true);
        assert_eq!(stats.total_runs, 2);
        assert_eq!(stats.total_steps, 30);
        assert_eq!(stats.global_max_depth, 15);
        assert_eq!(stats.step_limit_hits, 1);
        assert!((stats.avg_steps() - 15.0).abs() < 1e-9);
        assert!(stats.summary().contains("runs=2"));
    }
    #[test]
    fn test_run_state_machine() {
        let final_state = run_state_machine(StateMachineState::new(0), &|s| {
            let id = s.id;
            if id < 3 {
                Some(s.emit(&format!("step {}", id)).tap_id(id + 1))
            } else {
                None
            }
        });
        assert_eq!(final_state.id, 3);
        assert_eq!(final_state.output.len(), 3);
    }
    #[test]
    fn test_bench_tco_factorial() {
        let result = bench_tco_factorial(5, 10);
        assert_eq!(result.value, 120);
        assert_eq!(result.iterations, 10);
        assert!(result.report().contains("tco_factorial"));
    }
    #[test]
    fn test_bench_tco_fibonacci() {
        let result = bench_tco_fibonacci(7, 5);
        assert_eq!(result.value, 13);
        assert_eq!(result.iterations, 5);
    }
}
pub(super) trait TapId {
    fn tap_id(self, id: u32) -> Self;
}
/// Unroll a computation N times at compile time.
#[allow(dead_code)]
pub fn unroll_n<const N: usize, T, F>(mut state: T, f: F) -> T
where
    F: Fn(T) -> T,
{
    for _ in 0..N {
        state = f(state);
    }
    state
}
/// Compute n*2^K by unrolling K doublings.
#[allow(dead_code)]
pub fn double_n<const K: usize>(n: u64) -> u64 {
    unroll_n::<K, u64, _>(n, |x| x.wrapping_mul(2))
}
#[cfg(test)]
mod extended_tests_2 {
    use super::*;
    #[test]
    fn test_continuation_evaluator_add() {
        let mut eval = ContinuationEvaluator::new();
        eval.push_value(10);
        eval.push_cont(ContinuationFrame::ApplyBinop {
            op: BinopKind::Add,
            operand: 32,
        });
        eval.run();
        assert_eq!(eval.pop_value(), Some(42));
    }
    #[test]
    fn test_continuation_evaluator_store() {
        let mut eval = ContinuationEvaluator::new();
        eval.push_value(99);
        eval.push_cont(ContinuationFrame::StoreResult {
            var: "x".to_string(),
        });
        eval.run();
        assert_eq!(eval.env.get("x"), Some(&99));
    }
    #[test]
    fn test_binop_eval() {
        assert_eq!(BinopKind::Add.eval(3, 4), Some(7));
        assert_eq!(BinopKind::Sub.eval(10, 3), Some(7));
        assert_eq!(BinopKind::Mul.eval(3, 4), Some(12));
        assert_eq!(BinopKind::Div.eval(12, 4), Some(3));
        assert_eq!(BinopKind::Div.eval(12, 0), None);
    }
    #[test]
    fn test_function_tco_metrics() {
        let mut m = FunctionTcoMetrics::new("my_fn");
        m.record(5);
        m.record(10);
        m.record(7);
        assert_eq!(m.call_count, 3);
        assert_eq!(m.max_depth_eliminated, 10);
        assert_eq!(m.total_steps, 22);
        assert!((m.avg_depth() - 22.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metrics_registry() {
        let mut reg = TrampolineMetricsRegistry::new();
        reg.record("fact", 5);
        reg.record("fact", 10);
        reg.record("fib", 7);
        assert_eq!(reg.total_calls(), 3);
        let top = reg.top_by_steps(1);
        assert_eq!(top[0].name, "fact");
    }
    #[test]
    fn test_loop_detector() {
        let mut det = LoopDetector::new();
        assert!(!det.check(0xABCD));
        assert!(!det.check(0x1234));
        assert!(det.check(0xABCD));
        assert_eq!(det.unique_states(), 2);
        det.reset();
        assert_eq!(det.unique_states(), 0);
        assert!(!det.check(0xABCD));
    }
    #[test]
    fn test_partial_value_arithmetic() {
        let a = PartialValue::Known(3);
        let b = PartialValue::Known(4);
        assert_eq!(a.add(&b), PartialValue::Known(7));
        assert_eq!(a.mul(&b), PartialValue::Known(12));
        assert!(a.is_known());
        let u = PartialValue::Unknown("x".to_string());
        let result = a.mul(&u);
        match result {
            PartialValue::Unknown(_) => {}
            _ => panic!("expected Unknown"),
        }
        let zero = PartialValue::Known(0);
        assert_eq!(u.mul(&zero), PartialValue::Known(0));
    }
    #[test]
    fn test_tail_call_chain() {
        let mut chain = TailCallChain::new();
        assert!(chain.is_empty());
        chain.push("foo");
        chain.push("bar");
        assert_eq!(chain.len(), 2);
        assert!(chain.can_fuse);
        chain.mark_non_fusable();
        assert!(!chain.can_fuse);
    }
    #[test]
    fn test_evaluation_context() {
        let mut ctx = EvaluationContext::new();
        ctx.bind("x", 42);
        ctx.bind("y", 99);
        assert_eq!(ctx.lookup("x"), Some(42));
        assert_eq!(ctx.lookup("z"), None);
        assert_eq!(ctx.size(), 2);
        let child = ctx.child();
        assert_eq!(child.depth, 1);
        assert_eq!(child.lookup("x"), Some(42));
    }
    #[test]
    fn test_double_n() {
        assert_eq!(double_n::<3>(1), 8);
        assert_eq!(double_n::<0>(5), 5);
        assert_eq!(double_n::<4>(1), 16);
    }
    #[test]
    fn test_tail_call_proof() {
        let proof = TailCallProof::new(
            "factorial",
            "n",
            "Nat.lt",
            "n decreases by 1 at each recursive call, bounded below by 0",
        );
        let formatted = proof.format();
        assert!(formatted.contains("factorial"));
        assert!(formatted.contains("Nat.lt"));
        assert!(formatted.contains("decreases"));
    }
    #[test]
    fn test_unroll_n() {
        let result = unroll_n::<5, u64, _>(1, |x| x + 1);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_rewrite_rule() {
        let rule = RewriteRule::new("id x", "x");
        assert!(rule.unconditional);
        assert_eq!(rule.lhs, "id x");
        assert_eq!(rule.rhs, "x");
        let cond = RewriteRule::conditional("if true then x else y", "x");
        assert!(!cond.unconditional);
    }
}
#[cfg(test)]
mod pass_tests {
    use super::*;
    #[test]
    fn test_pass_processes_function() {
        let mut pass = TailCallOptimizationPass::new();
        let opcodes = ["Push", "Call", "Return"];
        let report = pass.process_function("my_fn", &opcodes, false);
        assert!(report.is_some());
        assert_eq!(pass.processed.len(), 1);
        assert_eq!(pass.skipped.len(), 0);
    }
    #[test]
    fn test_pass_skips_function() {
        let mut pass = TailCallOptimizationPass::new();
        let opcodes = ["Push", "Call", "Return"];
        let report = pass.process_function("inlined_fn", &opcodes, true);
        assert!(report.is_none());
        assert_eq!(pass.processed.len(), 0);
        assert_eq!(pass.skipped.len(), 1);
    }
    #[test]
    fn test_pass_summary() {
        let mut pass = TailCallOptimizationPass::new();
        pass.process_function("f1", &["Push", "Call", "Return"], false);
        pass.process_function("f2", &["Push", "Call", "Add", "Return"], false);
        pass.process_function("f3", &["Return"], true);
        let summary = pass.summary();
        assert!(summary.contains("processed"));
        assert!(summary.contains("skipped"));
    }
}
