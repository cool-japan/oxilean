//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CollectingPanicHandler, ContextSnapshot, DiagCode, ErrorAccumulator, ErrorFilter, ErrorPolicy,
    ErrorRenderer, ErrorSeverity, ErrorSource, ErrorTemplates, EvalError, EvalErrorBuilder,
    EvalErrorChain, EvalErrorContext, EvalErrorKind, EvalErrorStats, EvalFrame, EvalQuota,
    RecoveryStrategy, RenderStyle, RuntimeError, SilentPanicHandler, SourceSpan, SourcedError,
    StackTrace,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_source_span_display_with_file() {
        let s = SourceSpan::new(10, 25, Some("foo.lean".to_string()));
        assert_eq!(s.to_string(), "foo.lean:10..25");
    }
    #[test]
    fn test_source_span_display_no_file() {
        let s = SourceSpan::new(0, 5, None);
        assert_eq!(s.to_string(), "0..5");
    }
    #[test]
    fn test_source_span_len() {
        let s = SourceSpan::new(3, 10, None);
        assert_eq!(s.len(), 7);
    }
    #[test]
    fn test_source_span_empty() {
        let s = SourceSpan::synthetic();
        assert!(s.is_empty());
    }
    #[test]
    fn test_eval_frame_display() {
        let frame = EvalFrame::new("myFn", SourceSpan::new(0, 5, Some("a.lean".to_string())));
        let s = frame.to_string();
        assert!(s.contains("myFn"));
        assert!(s.contains("a.lean"));
    }
    #[test]
    fn test_eval_frame_tail_call() {
        let frame = EvalFrame::new("g", SourceSpan::synthetic()).tail_call();
        assert!(frame.is_tail_call);
        assert!(frame.to_string().contains("[tail]"));
    }
    #[test]
    fn test_eval_error_display_div_zero() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero);
        assert!(err.to_string().contains("division by zero"));
    }
    #[test]
    fn test_eval_error_with_span() {
        let span = SourceSpan::new(5, 10, Some("test.lean".to_string()));
        let err = EvalError::new(EvalErrorKind::DivisionByZero).with_span(span);
        let s = err.to_string();
        assert!(s.contains("test.lean"));
    }
    #[test]
    fn test_eval_error_with_frame() {
        let span = SourceSpan::synthetic();
        let frame = EvalFrame::new("foo", span.clone());
        let err = EvalError::new(EvalErrorKind::DivisionByZero).with_frame(frame);
        assert!(err.has_context());
        assert!(err.to_string().contains("foo"));
    }
    #[test]
    fn test_eval_error_with_hint() {
        let err = EvalErrorBuilder::div_by_zero();
        assert!(!err.hints.is_empty());
        let s = err.to_string();
        assert!(s.contains("hint:"));
    }
    #[test]
    fn test_eval_error_with_note() {
        let err = EvalErrorBuilder::sorry("myAxiom");
        assert!(err.note.is_some());
        assert!(err.to_string().contains("note:"));
    }
    #[test]
    fn test_eval_error_type_mismatch_display() {
        let err = EvalErrorBuilder::type_mismatch("Nat", "Bool");
        assert!(err.to_string().contains("Nat"));
        assert!(err.to_string().contains("Bool"));
    }
    #[test]
    fn test_eval_error_to_runtime_error_div() {
        let err = EvalErrorBuilder::div_by_zero();
        let re = err.to_runtime_error();
        assert_eq!(re, RuntimeError::DivisionByZero);
    }
    #[test]
    fn test_eval_error_to_runtime_error_sorry() {
        let err = EvalErrorBuilder::sorry("mySorry");
        let re = err.to_runtime_error();
        assert!(matches!(re, RuntimeError::SorryReached(_)));
    }
    #[test]
    fn test_eval_error_builder_stack_overflow() {
        let err = EvalErrorBuilder::stack_overflow(1000);
        assert!(err.to_string().contains("1000"));
        assert!(!err.hints.is_empty());
    }
    #[test]
    fn test_eval_error_builder_index_oob() {
        let err = EvalErrorBuilder::index_out_of_bounds(5, 3);
        assert!(err.to_string().contains("5"));
        assert!(err.to_string().contains("3"));
    }
    #[test]
    fn test_eval_error_builder_fuel_exhausted() {
        let err = EvalErrorBuilder::fuel_exhausted(500);
        assert!(err.to_string().contains("500"));
    }
    #[test]
    fn test_eval_error_builder_undefined_var() {
        let err = EvalErrorBuilder::undefined_var("x");
        assert!(err.to_string().contains("`x`"));
    }
    #[test]
    fn test_eval_error_builder_undefined_global() {
        let err = EvalErrorBuilder::undefined_global("Nat.add");
        assert!(err.to_string().contains("Nat.add"));
    }
    #[test]
    fn test_eval_error_builder_black_hole() {
        let err = EvalErrorBuilder::black_hole("fibThunk");
        assert!(err.to_string().contains("fibThunk"));
        assert!(err.note.is_some());
    }
    #[test]
    fn test_eval_error_multiple_frames() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero).with_frames(vec![
            EvalFrame::new("a", SourceSpan::synthetic()),
            EvalFrame::new("b", SourceSpan::synthetic()),
        ]);
        assert_eq!(err.frames.len(), 2);
    }
}
/// A hook called when a runtime panic occurs.
pub trait PanicHandler: Send + Sync {
    /// Called when a panic error is encountered.
    fn on_panic(&self, err: &EvalError);
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_recovery_strategy_continuation() {
        assert!(RecoveryStrategy::LogAndContinue.allows_continuation());
        assert!(RecoveryStrategy::ReturnDefault.allows_continuation());
        assert!(!RecoveryStrategy::Abort.allows_continuation());
    }
    #[test]
    fn test_recovery_strategy_retry() {
        let r = RecoveryStrategy::Retry { max_attempts: 3 };
        assert!(r.is_retry());
        assert!(r.allows_continuation());
        assert_eq!(format!("{}", r), "retry(max=3)");
    }
    #[test]
    fn test_error_policy_strict() {
        let p = ErrorPolicy::strict();
        let err = EvalErrorBuilder::div_by_zero();
        assert_eq!(*p.strategy_for(&err), RecoveryStrategy::Abort);
        assert!(!p.allows_continuation(&err));
    }
    #[test]
    fn test_error_policy_lenient() {
        let p = ErrorPolicy::lenient();
        let err = EvalErrorBuilder::div_by_zero();
        assert!(p.allows_continuation(&err));
    }
    #[test]
    fn test_error_policy_custom() {
        let p = ErrorPolicy::strict().with(
            EvalErrorKind::DivisionByZero,
            RecoveryStrategy::ReturnDefault,
        );
        let err = EvalErrorBuilder::div_by_zero();
        assert!(p.allows_continuation(&err));
    }
    #[test]
    fn test_eval_error_context_push_pop() {
        let mut ctx = EvalErrorContext::new();
        ctx.push("fn_a", SourceSpan::synthetic());
        ctx.push("fn_b", SourceSpan::synthetic());
        assert_eq!(ctx.depth(), 2);
        ctx.pop();
        assert_eq!(ctx.depth(), 1);
    }
    #[test]
    fn test_eval_error_context_annotate() {
        let mut ctx = EvalErrorContext::new();
        ctx.push("helper", SourceSpan::synthetic());
        let err = EvalError::new(EvalErrorKind::DivisionByZero);
        let annotated = ctx.annotate(err);
        assert!(annotated.has_context());
    }
    #[test]
    fn test_eval_error_context_disabled() {
        let mut ctx = EvalErrorContext::disabled();
        ctx.push("f", SourceSpan::synthetic());
        assert_eq!(ctx.depth(), 0);
    }
    #[test]
    fn test_collecting_panic_handler() {
        let handler = CollectingPanicHandler::new();
        let err = EvalErrorBuilder::panic_msg("oops");
        handler.on_panic(&err);
        handler.on_panic(&err);
        assert_eq!(handler.count(), 2);
        let collected = handler.collected();
        assert!(collected[0].contains("oops"));
    }
    #[test]
    fn test_error_accumulator_basic() {
        let mut acc = ErrorAccumulator::new();
        assert!(acc.is_empty());
        acc.push(EvalErrorBuilder::div_by_zero());
        acc.push(EvalErrorBuilder::undefined_var("x"));
        assert_eq!(acc.len(), 2);
    }
    #[test]
    fn test_error_accumulator_limit() {
        let mut acc = ErrorAccumulator::with_limit(2);
        assert!(!acc.push(EvalErrorBuilder::div_by_zero()));
        assert!(acc.push(EvalErrorBuilder::div_by_zero()));
        assert!(acc.at_limit());
    }
    #[test]
    fn test_error_accumulator_format_all() {
        let mut acc = ErrorAccumulator::new();
        acc.push(EvalErrorBuilder::div_by_zero());
        let fmt = acc.format_all();
        assert!(fmt.contains("[1]"));
        assert!(fmt.contains("division by zero"));
    }
    #[test]
    fn test_error_kind_predicates_fatal() {
        assert!(EvalErrorKind::StackOverflow { max_depth: 1 }.is_fatal());
        assert!(EvalErrorKind::FuelExhausted { limit: 0 }.is_fatal());
        assert!(!EvalErrorKind::DivisionByZero.is_fatal());
    }
    #[test]
    fn test_error_kind_predicates_logic() {
        assert!(EvalErrorKind::Panic {
            message: "x".into()
        }
        .is_logic_error());
        assert!(EvalErrorKind::SorryReached { name: "s".into() }.is_logic_error());
        assert!(!EvalErrorKind::DivisionByZero.is_logic_error());
    }
    #[test]
    fn test_error_kind_predicates_type_error() {
        assert!(EvalErrorKind::TypeMismatch {
            expected: "Nat".into(),
            got: "Bool".into()
        }
        .is_type_error());
        assert!(!EvalErrorKind::DivisionByZero.is_type_error());
    }
    #[test]
    fn test_error_kind_name() {
        assert_eq!(
            EvalErrorKind::DivisionByZero.kind_name(),
            "division_by_zero"
        );
        assert_eq!(
            EvalErrorKind::TypeMismatch {
                expected: "a".into(),
                got: "b".into()
            }
            .kind_name(),
            "type_mismatch"
        );
    }
    #[test]
    fn test_eval_error_stats() {
        let mut stats = EvalErrorStats::new();
        assert!(!stats.has_errors());
        stats.record(&EvalErrorBuilder::div_by_zero());
        stats.record(&EvalErrorBuilder::div_by_zero());
        stats.record(&EvalErrorBuilder::undefined_var("x"));
        assert_eq!(stats.total(), 3);
        assert_eq!(stats.count("division_by_zero"), 2);
        assert_eq!(stats.count("undefined_variable"), 1);
    }
    #[test]
    fn test_eval_error_stats_most_frequent() {
        let mut stats = EvalErrorStats::new();
        stats.record(&EvalErrorBuilder::div_by_zero());
        stats.record(&EvalErrorBuilder::div_by_zero());
        stats.record(&EvalErrorBuilder::undefined_var("x"));
        let (kind, count) = stats
            .most_frequent()
            .expect("test operation should succeed");
        assert_eq!(kind, "division_by_zero");
        assert_eq!(count, 2);
    }
    #[test]
    fn test_eval_error_stats_reset() {
        let mut stats = EvalErrorStats::new();
        stats.record(&EvalErrorBuilder::div_by_zero());
        stats.reset();
        assert_eq!(stats.total(), 0);
        assert!(!stats.has_errors());
    }
    #[test]
    fn test_stack_trace_from_error() {
        let span = SourceSpan::new(0, 5, Some("test.lean".to_string()));
        let err =
            EvalError::new(EvalErrorKind::DivisionByZero).with_frame(EvalFrame::new("myFn", span));
        let trace = StackTrace::from_error(&err);
        assert_eq!(trace.depth(), 1);
        assert!(trace.format().contains("division by zero"));
        assert!(trace.format().contains("myFn"));
    }
    #[test]
    fn test_stack_trace_empty() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero);
        let trace = StackTrace::from_error(&err);
        assert_eq!(trace.depth(), 0);
        let fmt = trace.format();
        assert!(!fmt.contains("call stack:"));
    }
    #[test]
    fn test_stack_trace_display() {
        let err = EvalErrorBuilder::undefined_var("foo");
        let trace = StackTrace::from_error(&err);
        let s = format!("{}", trace);
        assert!(s.contains("undefined variable"));
    }
    #[test]
    fn test_error_context_clear() {
        let mut ctx = EvalErrorContext::new();
        ctx.push("f", SourceSpan::synthetic());
        ctx.push("g", SourceSpan::synthetic());
        ctx.clear();
        assert!(ctx.is_empty());
    }
    #[test]
    fn test_error_context_max_frames() {
        let mut ctx = EvalErrorContext::new();
        ctx.max_frames = 2;
        ctx.push("a", SourceSpan::synthetic());
        ctx.push("b", SourceSpan::synthetic());
        ctx.push("c", SourceSpan::synthetic());
        assert_eq!(ctx.depth(), 2);
    }
    #[test]
    fn test_eval_error_kind_resource() {
        assert!(EvalErrorKind::StackOverflow { max_depth: 10 }.is_resource_error());
        assert!(EvalErrorKind::FuelExhausted { limit: 100 }.is_resource_error());
        assert!(!EvalErrorKind::DivisionByZero.is_resource_error());
    }
    #[test]
    fn test_silent_panic_handler() {
        let h = SilentPanicHandler;
        h.on_panic(&EvalErrorBuilder::panic_msg("test"));
    }
    #[test]
    fn test_recovery_strategy_display() {
        assert_eq!(format!("{}", RecoveryStrategy::Abort), "abort");
        assert_eq!(
            format!("{}", RecoveryStrategy::ReturnDefault),
            "return-default"
        );
        assert_eq!(
            format!("{}", RecoveryStrategy::FallbackToSorry),
            "fallback-to-sorry"
        );
        assert_eq!(
            format!("{}", RecoveryStrategy::LogAndContinue),
            "log-and-continue"
        );
    }
}
#[cfg(test)]
mod tests_renderer {
    use super::*;
    #[test]
    fn test_renderer_plain() {
        let err = EvalErrorBuilder::div_by_zero();
        let rendered = ErrorRenderer::plain().render(&err);
        assert!(rendered.contains("division by zero"));
    }
    #[test]
    fn test_renderer_compact() {
        let err = EvalErrorBuilder::div_by_zero();
        let rendered = ErrorRenderer::compact().render(&err);
        assert!(rendered.contains("division by zero"));
        assert!(!rendered.contains('\n'));
    }
    #[test]
    fn test_renderer_compact_with_span() {
        let span = SourceSpan::new(0, 5, Some("f.lean".to_string()));
        let err = EvalError::new(EvalErrorKind::DivisionByZero).with_span(span);
        let rendered = ErrorRenderer::compact().render(&err);
        assert!(rendered.contains("f.lean"));
    }
    #[test]
    fn test_renderer_ansi() {
        let err = EvalErrorBuilder::div_by_zero();
        let rendered = ErrorRenderer::new(RenderStyle::Ansi).render(&err);
        assert!(rendered.contains("division by zero"));
        assert!(rendered.contains('\x1b'));
    }
    #[test]
    fn test_renderer_structured() {
        let err = EvalErrorBuilder::div_by_zero();
        let rendered = ErrorRenderer::new(RenderStyle::Structured).render(&err);
        assert!(rendered.starts_with('{'));
        assert!(rendered.contains("division_by_zero"));
    }
    #[test]
    fn test_renderer_max_frames() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero).with_frames(vec![
            EvalFrame::new("a", SourceSpan::synthetic()),
            EvalFrame::new("b", SourceSpan::synthetic()),
            EvalFrame::new("c", SourceSpan::synthetic()),
        ]);
        let rendered = ErrorRenderer::plain().with_max_frames(2).render(&err);
        assert!(rendered.contains("1 more frames"));
    }
    #[test]
    fn test_renderer_no_hints() {
        let err = EvalErrorBuilder::div_by_zero();
        let rendered = ErrorRenderer::plain().with_hints(false).render(&err);
        assert!(!rendered.contains("hint:"));
    }
    #[test]
    fn test_renderer_no_note() {
        let err = EvalErrorBuilder::sorry("x");
        let rendered = ErrorRenderer::plain().with_note(false).render(&err);
        assert!(!rendered.contains("note:"));
    }
    #[test]
    fn test_error_chain_basic() {
        let chain = EvalErrorChain::new()
            .push(EvalErrorBuilder::undefined_var("x"))
            .push(EvalErrorBuilder::div_by_zero());
        assert_eq!(chain.len(), 2);
        assert!(chain
            .root_cause()
            .expect("test operation should succeed")
            .to_string()
            .contains("undefined variable"));
        assert!(chain
            .last_error()
            .expect("test operation should succeed")
            .to_string()
            .contains("division by zero"));
    }
    #[test]
    fn test_error_chain_format() {
        let chain = EvalErrorChain::new()
            .push(EvalErrorBuilder::div_by_zero())
            .push(EvalErrorBuilder::panic_msg("cascaded"));
        let s = chain.format();
        assert!(s.contains("root cause"));
        assert!(s.contains("caused"));
    }
    #[test]
    fn test_error_chain_display() {
        let chain = EvalErrorChain::new().push(EvalErrorBuilder::div_by_zero());
        assert!(!chain.to_string().is_empty());
    }
    #[test]
    fn test_error_filter_keep_fatal() {
        let errors = vec![
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::stack_overflow(100),
            EvalErrorBuilder::undefined_var("x"),
            EvalErrorBuilder::fuel_exhausted(500),
        ];
        let fatal = ErrorFilter::keep_fatal(errors);
        assert_eq!(fatal.len(), 2);
    }
    #[test]
    fn test_error_filter_keep_kind() {
        let errors = vec![
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::undefined_var("x"),
        ];
        let filtered = ErrorFilter::keep_kind(errors, "division_by_zero");
        assert_eq!(filtered.len(), 2);
    }
    #[test]
    fn test_error_filter_dedup_by_kind() {
        let errors = vec![
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::undefined_var("x"),
        ];
        let deduped = ErrorFilter::dedup_by_kind(errors);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_eval_error_has_span() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero);
        assert!(!err.has_span());
        let with_span = err.with_span(SourceSpan::synthetic());
        assert!(with_span.has_span());
    }
    #[test]
    fn test_eval_error_has_hints() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero);
        assert!(!err.has_hints());
        let with_hint = err.with_hint("fix it");
        assert!(with_hint.has_hints());
    }
    #[test]
    fn test_eval_error_compact() {
        let err = EvalErrorBuilder::div_by_zero();
        let s = err.compact();
        assert!(s.contains("division by zero"));
    }
    #[test]
    fn test_eval_error_matches() {
        let err = EvalErrorBuilder::div_by_zero();
        assert!(err.matches(|k| matches!(k, EvalErrorKind::DivisionByZero)));
        assert!(!err.matches(|k| matches!(k, EvalErrorKind::StackOverflow { .. })));
    }
    #[test]
    fn test_eval_error_context_max_frame_boundary() {
        let mut ctx = EvalErrorContext::new();
        ctx.max_frames = 3;
        for i in 0..5 {
            ctx.push(format!("f{}", i), SourceSpan::synthetic());
        }
        assert_eq!(ctx.depth(), 3);
    }
    #[test]
    fn test_accumulator_into_errors() {
        let mut acc = ErrorAccumulator::new();
        acc.push(EvalErrorBuilder::div_by_zero());
        let errors = acc.into_errors();
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_eval_error_stats_black_hole() {
        let mut stats = EvalErrorStats::new();
        stats.record(&EvalErrorBuilder::black_hole("th"));
        assert_eq!(stats.count("black_hole"), 1);
    }
    #[test]
    fn test_error_kind_io() {
        let kind = EvalErrorKind::Io {
            message: "read failed".into(),
        };
        assert_eq!(kind.kind_name(), "io");
        assert!(!kind.is_fatal());
        assert!(!kind.is_type_error());
    }
    #[test]
    fn test_render_style_default() {
        assert_eq!(RenderStyle::default(), RenderStyle::Plain);
    }
}
/// Standard diagnostic codes for eval errors.
pub mod diag_codes {
    use super::DiagCode;
    /// Division by zero.
    pub const DIV_BY_ZERO: DiagCode = DiagCode {
        prefix: 'E',
        number: 1001,
    };
    /// Stack overflow.
    pub const STACK_OVERFLOW: DiagCode = DiagCode {
        prefix: 'E',
        number: 1002,
    };
    /// Type mismatch.
    pub const TYPE_MISMATCH: DiagCode = DiagCode {
        prefix: 'E',
        number: 1003,
    };
    /// Index out of bounds.
    pub const INDEX_OOB: DiagCode = DiagCode {
        prefix: 'E',
        number: 1004,
    };
    /// Sorry reached.
    pub const SORRY_REACHED: DiagCode = DiagCode {
        prefix: 'W',
        number: 2001,
    };
    /// Fuel exhausted.
    pub const FUEL_EXHAUSTED: DiagCode = DiagCode {
        prefix: 'E',
        number: 1005,
    };
    /// Undefined variable.
    pub const UNDEFINED_VAR: DiagCode = DiagCode {
        prefix: 'E',
        number: 1006,
    };
    /// Undefined global.
    pub const UNDEFINED_GLOBAL: DiagCode = DiagCode {
        prefix: 'E',
        number: 1007,
    };
    /// Arithmetic overflow.
    pub const ARITH_OVERFLOW: DiagCode = DiagCode {
        prefix: 'E',
        number: 1008,
    };
    /// Non-exhaustive match.
    pub const NON_EXHAUSTIVE: DiagCode = DiagCode {
        prefix: 'E',
        number: 1009,
    };
    /// Panic.
    pub const PANIC: DiagCode = DiagCode {
        prefix: 'E',
        number: 1010,
    };
    /// Unimplemented.
    pub const UNIMPLEMENTED: DiagCode = DiagCode {
        prefix: 'W',
        number: 2002,
    };
    /// I/O error.
    pub const IO_ERROR: DiagCode = DiagCode {
        prefix: 'E',
        number: 1011,
    };
    /// Black hole.
    pub const BLACK_HOLE: DiagCode = DiagCode {
        prefix: 'E',
        number: 1012,
    };
}
#[cfg(test)]
mod tests_severity_quota {
    use super::*;
    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Info < ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning < ErrorSeverity::Error);
        assert!(ErrorSeverity::Error < ErrorSeverity::Critical);
    }
    #[test]
    fn test_error_severity_display() {
        assert_eq!(format!("{}", ErrorSeverity::Info), "info");
        assert_eq!(format!("{}", ErrorSeverity::Critical), "critical");
    }
    #[test]
    fn test_kind_default_severity() {
        assert_eq!(
            EvalErrorKind::DivisionByZero.default_severity(),
            ErrorSeverity::Error
        );
        assert_eq!(
            EvalErrorKind::StackOverflow { max_depth: 1 }.default_severity(),
            ErrorSeverity::Critical
        );
        assert_eq!(
            EvalErrorKind::SorryReached { name: "x".into() }.default_severity(),
            ErrorSeverity::Warning
        );
    }
    #[test]
    fn test_eval_error_severity() {
        let err = EvalErrorBuilder::stack_overflow(100);
        assert!(err.is_critical());
        let err2 = EvalErrorBuilder::div_by_zero();
        assert!(!err2.is_critical());
    }
    #[test]
    fn test_eval_error_is_at_least_warning() {
        let err = EvalErrorBuilder::sorry("x");
        assert!(err.is_at_least_warning());
    }
    #[test]
    fn test_eval_quota_unlimited() {
        let mut q = EvalQuota::unlimited();
        assert!(q.tick().is_ok());
        assert!(q.tick().is_ok());
        assert!(!q.is_exhausted());
        assert_eq!(q.remaining(), None);
        assert_eq!(q.steps_taken(), 2);
    }
    #[test]
    fn test_eval_quota_limited() {
        let mut q = EvalQuota::limited(3);
        assert!(q.tick().is_ok());
        assert!(q.tick().is_ok());
        assert!(q.tick().is_ok());
        assert!(q.tick().is_err());
        assert!(q.is_exhausted());
    }
    #[test]
    fn test_eval_quota_consume_multiple() {
        let mut q = EvalQuota::limited(10);
        assert!(q.consume(5).is_ok());
        assert_eq!(q.remaining(), Some(5));
        assert!(q.consume(6).is_err());
    }
    #[test]
    fn test_eval_quota_reset() {
        let mut q = EvalQuota::limited(5);
        q.consume(3).expect("test operation should succeed");
        q.reset();
        assert_eq!(q.remaining(), Some(5));
        assert_eq!(q.steps_taken(), 0);
    }
    #[test]
    fn test_eval_quota_default() {
        let q = EvalQuota::default();
        assert_eq!(q.remaining(), None);
    }
    #[test]
    fn test_diag_code_display() {
        let code = DiagCode::error(1001);
        assert_eq!(format!("{}", code), "E1001");
        let warn = DiagCode::warning(2001);
        assert_eq!(format!("{}", warn), "W2001");
    }
    #[test]
    fn test_diag_code_for_div_zero() {
        let code = EvalErrorKind::DivisionByZero.diag_code();
        assert_eq!(code.prefix, 'E');
        assert_eq!(code.number, 1001);
    }
    #[test]
    fn test_diag_code_for_sorry() {
        let code = EvalErrorKind::SorryReached { name: "x".into() }.diag_code();
        assert_eq!(code.prefix, 'W');
    }
    #[test]
    fn test_error_source_display() {
        assert_eq!(format!("{}", ErrorSource::Kernel), "kernel");
        assert_eq!(
            format!(
                "{}",
                ErrorSource::UserCode {
                    decl_name: "foo".into()
                }
            ),
            "user-code(foo)"
        );
        assert_eq!(
            format!(
                "{}",
                ErrorSource::BytecodeInterp {
                    chunk_name: "main".into(),
                    ip: 42
                }
            ),
            "bytecode-interp(main@42)"
        );
    }
    #[test]
    fn test_sourced_error_display() {
        let se = SourcedError::unknown(EvalErrorBuilder::div_by_zero());
        let s = format!("{}", se);
        assert!(s.contains("[unknown]"));
        assert!(s.contains("division by zero"));
    }
    #[test]
    fn test_sourced_error_new() {
        let se = SourcedError::new(
            EvalErrorBuilder::undefined_var("x"),
            ErrorSource::Elaborator,
        );
        assert_eq!(se.source, ErrorSource::Elaborator);
    }
    #[test]
    fn test_error_filter_remove() {
        let errors = vec![
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::stack_overflow(10),
        ];
        let remaining = ErrorFilter::remove(errors, |e| e.kind.is_fatal());
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].kind, EvalErrorKind::DivisionByZero);
    }
    #[test]
    fn test_error_chain_empty() {
        let chain = EvalErrorChain::new();
        assert!(chain.is_empty());
        assert!(chain.root_cause().is_none());
    }
    #[test]
    fn test_diag_code_equality() {
        let a = DiagCode::error(1001);
        let b = DiagCode::error(1001);
        let c = DiagCode::error(1002);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
    #[test]
    fn test_eval_quota_fuel_error_kind() {
        let mut q = EvalQuota::limited(1);
        q.consume(1).expect("test operation should succeed");
        let err = q.tick().unwrap_err();
        assert!(matches!(err.kind, EvalErrorKind::FuelExhausted { .. }));
    }
}
/// Groups a list of errors by their kind name.
pub fn group_by_kind(errors: &[EvalError]) -> std::collections::HashMap<String, Vec<&EvalError>> {
    let mut groups: std::collections::HashMap<String, Vec<&EvalError>> = Default::default();
    for err in errors {
        groups
            .entry(err.kind.kind_name().to_string())
            .or_default()
            .push(err);
    }
    groups
}
#[cfg(test)]
mod tests_templates {
    use super::*;
    #[test]
    fn test_wrong_num_args() {
        let err = ErrorTemplates::wrong_num_args(2, 3);
        assert!(err.to_string().contains("2 arguments"));
        assert!(!err.hints.is_empty());
    }
    #[test]
    fn test_add_overflow() {
        let err = ErrorTemplates::add_overflow(i64::MAX, 1);
        assert!(err.to_string().contains("+"));
    }
    #[test]
    fn test_mul_overflow() {
        let err = ErrorTemplates::mul_overflow(1000, 1000);
        assert!(err.to_string().contains("*"));
    }
    #[test]
    fn test_empty_list_head() {
        let err = ErrorTemplates::empty_list_head();
        assert!(err.to_string().contains("head"));
        assert!(err.note.is_some());
    }
    #[test]
    fn test_assertion_failed() {
        let err = ErrorTemplates::assertion_failed("x > 0");
        assert!(err.to_string().contains("x > 0"));
    }
    #[test]
    fn test_cast_failed() {
        let err = ErrorTemplates::cast_failed("Bool", "Nat");
        assert!(err.to_string().contains("Nat"));
        assert!(!err.hints.is_empty());
    }
    #[test]
    fn test_group_by_kind() {
        let errors = vec![
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::div_by_zero(),
            EvalErrorBuilder::undefined_var("x"),
        ];
        let groups = group_by_kind(&errors);
        assert_eq!(groups["division_by_zero"].len(), 2);
        assert_eq!(groups["undefined_variable"].len(), 1);
    }
    #[test]
    fn test_negative_index() {
        let err = ErrorTemplates::negative_index(-5);
        assert!(err.to_string().contains("-5"));
    }
    #[test]
    fn test_empty_list_tail() {
        let err = ErrorTemplates::empty_list_tail();
        assert!(err.to_string().contains("tail"));
    }
}
#[cfg(test)]
mod tests_snapshot {
    use super::*;
    #[test]
    fn test_context_snapshot_take() {
        let mut ctx = EvalErrorContext::new();
        ctx.push("fn_a", SourceSpan::synthetic());
        ctx.push("fn_b", SourceSpan::synthetic());
        let snap = ContextSnapshot::take(&ctx, 42);
        assert_eq!(snap.frames.len(), 2);
        assert_eq!(snap.step, 42);
    }
    #[test]
    fn test_context_snapshot_into_error() {
        let mut ctx = EvalErrorContext::new();
        ctx.push("fn_x", SourceSpan::synthetic());
        let snap = ContextSnapshot::take(&ctx, 0);
        let err = snap.into_error(EvalErrorKind::DivisionByZero);
        assert!(err.has_context());
    }
}
