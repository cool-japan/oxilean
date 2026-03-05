//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Level, Name};

use super::types::{
    AnnotatedError, ConfigNode, DecisionNode, Diagnostic, DiagnosticCollection, Either2,
    ErrorAccumulator, ErrorCategory, ErrorContext, ErrorNote, ErrorReport, Fixture,
    FlatSubstitution, FocusStack, KernelError, KernelPhase, LabelSet, MinHeap, NonEmptyVec,
    PathBuf, PhasedError, PrefixCounter, RewriteRule, RewriteRuleSet, Severity, SimpleDag,
    SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket,
    TransformStat, TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Kernel result type.
pub type KernelResult<T> = Result<T, KernelError>;
/// Format a `KernelError` for display in an IDE panel.
pub fn format_error_panel(err: &KernelError, width: usize) -> String {
    let separator = "-".repeat(width);
    let mut lines = vec![
        separator.clone(),
        format!("  Error [{:?}]", err.category()),
        format!("  {}", err.short_description()),
    ];
    match err {
        KernelError::TypeMismatch {
            expected,
            got,
            context,
        } => {
            lines.push(format!("  Context  : {}", context));
            lines.push(format!("  Expected : {}", expected));
            lines.push(format!("  Got      : {}", got));
        }
        KernelError::UniverseInconsistency { lhs, rhs } => {
            lines.push(format!("  LHS : {}", lhs));
            lines.push(format!("  RHS : {}", rhs));
        }
        _ => {}
    }
    lines.push(separator);
    lines.join("\n")
}
/// Return a one-line summary of the error, truncated.
pub fn summarise_error(err: &KernelError, max_len: usize) -> String {
    let full = err.to_string();
    if full.len() <= max_len {
        full
    } else {
        format!("{}...", &full[..max_len.saturating_sub(3)])
    }
}
/// Collect all unique unknown constants from a list of errors.
pub fn collect_unknown_constants(errors: &[KernelError]) -> Vec<&Name> {
    let mut out = vec![];
    for err in errors {
        if let KernelError::UnknownConstant(name) = err {
            if !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}
/// Check whether a list of errors contains any universe inconsistencies.
pub fn has_universe_errors(errors: &[KernelError]) -> bool {
    errors
        .iter()
        .any(|e| matches!(e, KernelError::UniverseInconsistency { .. }))
}
/// Extension trait for `Result<T, KernelError>` that adds context.
#[allow(clippy::result_large_err)]
pub trait KernelResultExt<T> {
    /// Attach a context string to any contained `KernelError`.
    fn context(self, ctx: impl Into<String>) -> Result<T, KernelError>;
}
impl<T> KernelResultExt<T> for Result<T, KernelError> {
    fn context(self, ctx: impl Into<String>) -> Result<T, KernelError> {
        self.map_err(|e| e.with_context(ctx))
    }
}
/// Wrap a closure into a `KernelResult`.
#[allow(clippy::result_large_err)]
pub fn try_kernel<T, E: std::error::Error>(f: impl FnOnce() -> Result<T, E>) -> KernelResult<T> {
    f().map_err(|e| KernelError::Other(e.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn mk_sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_type_mismatch_display() {
        let err = KernelError::TypeMismatch {
            expected: mk_sort(),
            got: mk_sort(),
            context: "test".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("type mismatch"));
        assert!(s.contains("test"));
    }
    #[test]
    fn test_unbound_variable_display() {
        let err = KernelError::UnboundVariable(7);
        assert_eq!(err.to_string(), "unbound variable: #7");
    }
    #[test]
    fn test_unknown_constant_display() {
        let name = Name::str("Nat");
        let err = KernelError::UnknownConstant(name);
        assert!(err.to_string().contains("Nat"));
    }
    #[test]
    fn test_category_assignment() {
        assert_eq!(
            KernelError::UnboundVariable(0).category(),
            ErrorCategory::Binding
        );
        assert_eq!(
            KernelError::UnknownConstant(Name::str("X")).category(),
            ErrorCategory::Resolution
        );
        assert_eq!(
            KernelError::Other("x".to_string()).category(),
            ErrorCategory::Other
        );
    }
    #[test]
    fn test_recoverability() {
        assert!(KernelError::UnknownConstant(Name::str("X")).is_recoverable());
        assert!(KernelError::Other("x".to_string()).is_recoverable());
        assert!(KernelError::UnboundVariable(0).is_fatal());
    }
    #[test]
    fn test_error_accumulator_basic() {
        let mut acc = ErrorAccumulator::new();
        assert!(acc.is_empty());
        acc.push(KernelError::UnboundVariable(1));
        acc.push(KernelError::Other("oops".to_string()));
        assert_eq!(acc.len(), 2);
    }
    #[test]
    fn test_error_accumulator_into_result() {
        let mut acc = ErrorAccumulator::new();
        acc.push(KernelError::UnboundVariable(0));
        assert!(acc.into_result().is_err());
        assert!(ErrorAccumulator::new().into_result().is_ok());
    }
    #[test]
    fn test_error_accumulator_drain() {
        let mut acc = ErrorAccumulator::new();
        acc.push(KernelError::UnboundVariable(0));
        acc.push(KernelError::UnboundVariable(1));
        let drained = acc.drain();
        assert_eq!(drained.len(), 2);
        assert!(acc.is_empty());
    }
    #[test]
    fn test_error_report_display() {
        let err = KernelError::Other("bad".to_string());
        let report = ErrorReport::new(err)
            .with_location(Name::str("myTheorem"))
            .add_note("try again");
        let s = report.to_string();
        assert!(s.contains("myTheorem"));
        assert!(s.contains("bad"));
        assert!(s.contains("try again"));
    }
    #[test]
    fn test_diagnostic_collection_has_errors() {
        let mut coll = DiagnosticCollection::new();
        assert!(!coll.has_errors());
        coll.add(Diagnostic::warning("some warning"));
        assert!(!coll.has_errors());
        coll.add(Diagnostic::error("some error"));
        assert!(coll.has_errors());
    }
    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }
    #[test]
    fn test_diagnostic_display() {
        let d = Diagnostic::error("bad type").at(Name::str("foo"));
        let s = d.to_string();
        assert!(s.contains("error"));
        assert!(s.contains("foo"));
        assert!(s.contains("bad type"));
    }
    #[test]
    fn test_error_context_format() {
        let ctx = ErrorContext::new()
            .push("checking Nat.add")
            .push("checking body");
        let s = ctx.format();
        assert!(s.contains("checking Nat.add"));
    }
    #[test]
    fn test_error_context_into_error() {
        let ctx = ErrorContext::new().push("step 1");
        let err = ctx.into_error("something went wrong");
        assert!(matches!(err, KernelError::Other(_)));
    }
    #[test]
    fn test_format_error_panel() {
        let err = KernelError::UnboundVariable(3);
        let panel = format_error_panel(&err, 40);
        assert!(panel.contains("unbound variable"));
    }
    #[test]
    fn test_summarise_error_long() {
        let err = KernelError::Other("a".repeat(200));
        let s = summarise_error(&err, 50);
        assert!(s.len() <= 50);
        assert!(s.ends_with("..."));
    }
    #[test]
    fn test_summarise_error_short() {
        let err = KernelError::Other("short".to_string());
        let s = summarise_error(&err, 100);
        assert_eq!(s, "short");
    }
    #[test]
    fn test_collect_unknown_constants() {
        let errs = vec![
            KernelError::UnknownConstant(Name::str("Foo")),
            KernelError::UnboundVariable(1),
            KernelError::UnknownConstant(Name::str("Bar")),
            KernelError::UnknownConstant(Name::str("Foo")),
        ];
        let names = collect_unknown_constants(&errs);
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_has_universe_errors() {
        let no_univ = vec![KernelError::UnboundVariable(0)];
        assert!(!has_universe_errors(&no_univ));
        let with_univ = vec![KernelError::UniverseInconsistency {
            lhs: Level::zero(),
            rhs: Level::succ(Level::zero()),
        }];
        assert!(has_universe_errors(&with_univ));
    }
    #[test]
    fn test_kernel_result_ext_context() {
        let r: Result<(), KernelError> = Err(KernelError::UnboundVariable(5));
        let r2 = r.context("while checking foo");
        let s = r2.unwrap_err().to_string();
        assert!(s.contains("while checking foo"));
    }
    #[test]
    fn test_from_string_conversion() {
        let err: KernelError = "hello".into();
        assert!(matches!(err, KernelError::Other(_)));
        let err2: KernelError = String::from("world").into();
        assert!(matches!(err2, KernelError::Other(_)));
    }
    #[test]
    fn test_count_by_category() {
        let mut acc = ErrorAccumulator::new();
        acc.push(KernelError::UnboundVariable(0));
        acc.push(KernelError::UnboundVariable(1));
        acc.push(KernelError::UnknownConstant(Name::str("X")));
        let counts = acc.count_by_category();
        assert_eq!(counts[&ErrorCategory::Binding], 2);
        assert_eq!(counts[&ErrorCategory::Resolution], 1);
    }
    #[test]
    fn test_fatal_recoverable_split() {
        let mut acc = ErrorAccumulator::new();
        acc.push(KernelError::UnboundVariable(0));
        acc.push(KernelError::Other("x".to_string()));
        assert_eq!(acc.fatal_errors().len(), 1);
        assert_eq!(acc.recoverable_errors().len(), 1);
    }
    #[test]
    fn test_short_description() {
        let err = KernelError::NotASort(mk_sort());
        assert!(err.short_description().contains("sort"));
        let err2 = KernelError::NotAFunction(mk_sort());
        assert!(err2.short_description().contains("function"));
    }
    #[test]
    fn test_with_context_type_mismatch() {
        let err = KernelError::TypeMismatch {
            expected: mk_sort(),
            got: mk_sort(),
            context: "inner".to_string(),
        };
        let err2 = err.with_context("outer");
        if let KernelError::TypeMismatch { context, .. } = err2 {
            assert!(context.contains("outer"));
            assert!(context.contains("inner"));
        } else {
            panic!("expected TypeMismatch");
        }
    }
    #[test]
    fn test_try_kernel_ok() {
        let r: KernelResult<i32> = try_kernel(|| Ok::<i32, std::num::ParseIntError>(42));
        assert_eq!(r.expect("r should be valid"), 42);
    }
    #[test]
    fn test_try_kernel_err() {
        let r: KernelResult<i32> = try_kernel(|| "abc".parse::<i32>());
        assert!(r.is_err());
    }
    #[test]
    fn test_diagnostic_collection_by_severity() {
        let mut coll = DiagnosticCollection::new();
        coll.add(Diagnostic::error("e1"));
        coll.add(Diagnostic::warning("w1"));
        coll.add(Diagnostic::info("i1"));
        coll.add(Diagnostic::error("e2"));
        assert_eq!(coll.errors().len(), 2);
        assert_eq!(coll.warnings().len(), 1);
        assert_eq!(coll.by_severity(Severity::Info).len(), 1);
    }
}
#[cfg(test)]
mod extra_error_tests {
    use super::*;
    #[test]
    fn test_error_note_display_no_location() {
        let note = ErrorNote::new("try something else");
        let s = note.to_string();
        assert!(s.contains("try something else"));
        assert!(s.contains("note:"));
    }
    #[test]
    fn test_error_note_display_with_location() {
        let note = ErrorNote::new("see definition").at(Name::str("Foo.bar"));
        let s = note.to_string();
        assert!(s.contains("Foo.bar"));
    }
    #[test]
    fn test_annotated_error_no_notes() {
        let ae = AnnotatedError::new(KernelError::UnboundVariable(3));
        assert_eq!(ae.num_notes(), 0);
    }
    #[test]
    fn test_annotated_error_with_note() {
        let ae = AnnotatedError::new(KernelError::UnboundVariable(3))
            .with_note(ErrorNote::new("check binder count"));
        assert_eq!(ae.num_notes(), 1);
    }
    #[test]
    fn test_annotated_error_display() {
        let ae =
            AnnotatedError::new(KernelError::Other("bad".into())).with_note(ErrorNote::new("hint"));
        let s = ae.to_string();
        assert!(s.contains("bad"));
        assert!(s.contains("hint"));
    }
    #[test]
    fn test_annotated_error_is_fatal() {
        let ae = AnnotatedError::new(KernelError::UnboundVariable(0));
        assert!(ae.is_fatal());
        let ae2 = AnnotatedError::new(KernelError::Other("x".into()));
        assert!(!ae2.is_fatal());
    }
    #[test]
    fn test_kernel_phase_display() {
        assert_eq!(format!("{}", KernelPhase::Parse), "parse");
        assert_eq!(format!("{}", KernelPhase::TypeCheck), "type-check");
    }
    #[test]
    fn test_phased_error_display() {
        let pe = PhasedError::new(KernelPhase::Elab, KernelError::UnboundVariable(2));
        let s = pe.to_string();
        assert!(s.contains("elab"));
        assert!(s.contains("unbound"));
    }
    #[test]
    fn test_phased_error_new() {
        let pe = PhasedError::new(KernelPhase::Reduction, KernelError::Other("x".into()));
        assert_eq!(pe.phase, KernelPhase::Reduction);
    }
    #[test]
    fn test_error_note_at() {
        let note = ErrorNote::new("see here").at(Name::str("Nat.add"));
        assert!(note.location.is_some());
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
