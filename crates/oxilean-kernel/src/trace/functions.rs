//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Expr;

use super::types::{
    ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, LabelSet, NonEmptyVec,
    PathBuf, ReductionRule, RewriteRule, RewriteRuleSet, RingTracer, SimpleDag, SlidingSum,
    SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, StringSink, TokenBucket,
    TraceCategory, TraceEvent, TraceFilter, TraceLevel, Tracer, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Build a debug event for definitional equality checking.
pub fn def_eq_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Debug, msg.into()).with_category(TraceCategory::DefEq)
}
/// Build a trace event for reduction.
pub fn reduce_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Trace, msg.into()).with_category(TraceCategory::Reduce)
}
/// Build an info event for tactic execution.
pub fn tactic_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Info, msg.into()).with_category(TraceCategory::Tactic)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;
    #[test]
    fn test_tracer_create() {
        let tracer = Tracer::new(TraceLevel::Info);
        assert_eq!(tracer.level(), TraceLevel::Info);
        assert_eq!(tracer.events().len(), 0);
    }
    #[test]
    fn test_log_event() {
        let mut tracer = Tracer::new(TraceLevel::Debug);
        tracer.log(TraceEvent::new(
            TraceLevel::Info,
            "Test message".to_string(),
        ));
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_log_filtered() {
        let mut tracer = Tracer::new(TraceLevel::Warn);
        tracer.log(TraceEvent::new(
            TraceLevel::Debug,
            "Test message".to_string(),
        ));
        assert_eq!(tracer.event_count(), 0);
    }
    #[test]
    fn test_clear() {
        let mut tracer = Tracer::new(TraceLevel::Debug);
        tracer.info("hello");
        assert_eq!(tracer.event_count(), 1);
        tracer.clear();
        assert_eq!(tracer.event_count(), 0);
    }
    #[test]
    fn test_with_expr() {
        let event = TraceEvent::new(TraceLevel::Info, "Test".to_string())
            .with_expr(Expr::Lit(Literal::Nat(42)));
        assert!(event.expr.is_some());
    }
    #[test]
    fn test_level_ordering() {
        assert!(TraceLevel::Off < TraceLevel::Error);
        assert!(TraceLevel::Error < TraceLevel::Warn);
        assert!(TraceLevel::Warn < TraceLevel::Info);
        assert!(TraceLevel::Info < TraceLevel::Debug);
        assert!(TraceLevel::Debug < TraceLevel::Trace);
    }
    #[test]
    fn test_level_from_str() {
        assert_eq!(TraceLevel::from_str("info"), Some(TraceLevel::Info));
        assert_eq!(TraceLevel::from_str("DEBUG"), Some(TraceLevel::Debug));
        assert_eq!(TraceLevel::from_str("bogus"), None);
    }
    #[test]
    fn test_suppress_category() {
        let mut tracer = Tracer::new(TraceLevel::Trace);
        tracer.suppress(TraceCategory::Simp);
        let event = TraceEvent::new(TraceLevel::Trace, "simp step".to_string())
            .with_category(TraceCategory::Simp);
        tracer.log(event);
        assert_eq!(tracer.event_count(), 0);
    }
    #[test]
    fn test_record_reduction() {
        let mut tracer = Tracer::new(TraceLevel::Trace);
        let before = Expr::Lit(Literal::Nat(1));
        let after = Expr::Lit(Literal::Nat(2));
        tracer.record_reduction(ReductionRule::Beta, before, after);
        assert_eq!(tracer.reduction_steps().len(), 1);
        assert_eq!(tracer.reduction_steps()[0].rule, ReductionRule::Beta);
    }
    #[test]
    fn test_depth_indentation() {
        let mut tracer = Tracer::new(TraceLevel::Info);
        tracer.push();
        tracer.push();
        assert_eq!(tracer.current_depth(), 2);
        tracer.info("indented");
        let formatted = tracer.events()[0].format();
        assert!(
            formatted.starts_with("    "),
            "Expected 4-space indent, got: {formatted:?}"
        );
    }
    #[test]
    fn test_render() {
        let mut tracer = Tracer::new(TraceLevel::Info);
        tracer.info("line 1");
        tracer.info("line 2");
        let rendered = tracer.render();
        assert!(rendered.contains("line 1"));
        assert!(rendered.contains("line 2"));
    }
    #[test]
    fn test_convenience_builders() {
        let e1 = def_eq_event("checking");
        assert_eq!(e1.category, Some(TraceCategory::DefEq));
        let e2 = reduce_event("reducing");
        assert_eq!(e2.category, Some(TraceCategory::Reduce));
        let e3 = tactic_event("applying");
        assert_eq!(e3.category, Some(TraceCategory::Tactic));
    }
}
/// Build an info event for the simp category.
pub fn simp_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Info, msg.into()).with_category(TraceCategory::Simp)
}
/// Build a debug event for the unification category.
pub fn unify_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Debug, msg.into()).with_category(TraceCategory::Unify)
}
/// Build a trace event for elaboration.
pub fn elab_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Trace, msg.into()).with_category(TraceCategory::Elab)
}
/// Build an info event for typeclass resolution.
pub fn typeclass_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Info, msg.into()).with_category(TraceCategory::Typeclass)
}
/// Build a warning event with no category.
pub fn warn_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Warn, msg.into())
}
/// Build an error event with no category.
pub fn error_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Error, msg.into())
}
#[cfg(test)]
mod trace_extended_tests {
    use super::*;
    #[test]
    fn test_ring_tracer_basic() {
        let mut rt = RingTracer::new(3, TraceLevel::Info);
        rt.log_msg(TraceLevel::Info, "a");
        rt.log_msg(TraceLevel::Info, "b");
        rt.log_msg(TraceLevel::Info, "c");
        assert_eq!(rt.stored_count(), 3);
        assert_eq!(rt.total_count(), 3);
    }
    #[test]
    fn test_ring_tracer_overflow() {
        let mut rt = RingTracer::new(2, TraceLevel::Info);
        rt.log_msg(TraceLevel::Info, "a");
        rt.log_msg(TraceLevel::Info, "b");
        rt.log_msg(TraceLevel::Info, "c");
        assert_eq!(rt.stored_count(), 2);
        assert_eq!(rt.total_count(), 3);
    }
    #[test]
    fn test_ring_tracer_clear() {
        let mut rt = RingTracer::new(5, TraceLevel::Info);
        rt.log_msg(TraceLevel::Info, "x");
        rt.clear();
        assert_eq!(rt.stored_count(), 0);
        assert_eq!(rt.total_count(), 0);
    }
    #[test]
    fn test_ring_tracer_level_filter() {
        let mut rt = RingTracer::new(10, TraceLevel::Warn);
        rt.log_msg(TraceLevel::Debug, "debug msg");
        assert_eq!(rt.stored_count(), 0);
        rt.log_msg(TraceLevel::Warn, "warn msg");
        assert_eq!(rt.stored_count(), 1);
    }
    #[test]
    fn test_tracer_stats() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.error("err");
        t.warn("wrn");
        t.info("inf");
        let stats = t.stats();
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 1);
        assert_eq!(stats.infos, 1);
        assert!(stats.has_errors());
        assert!(stats.has_warnings());
    }
    #[test]
    fn test_tracer_last_event() {
        let mut t = Tracer::new(TraceLevel::Info);
        assert!(t.last_event().is_none());
        t.info("first");
        t.info("second");
        assert_eq!(
            t.last_event().expect("last_event should succeed").message,
            "second"
        );
    }
    #[test]
    fn test_tracer_last_error() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.info("not an error");
        assert!(t.last_error().is_none());
        t.error("problem");
        assert!(t.last_error().is_some());
    }
    #[test]
    fn test_tracer_log_with_category() {
        let mut t = Tracer::new(TraceLevel::Debug);
        t.log_with_category(TraceLevel::Debug, TraceCategory::Simp, "simp step");
        let simp = t.events_in_category(&TraceCategory::Simp);
        assert_eq!(simp.len(), 1);
    }
    #[test]
    fn test_simp_event() {
        let e = simp_event("normalizing");
        assert_eq!(e.category, Some(TraceCategory::Simp));
    }
    #[test]
    fn test_unify_event() {
        let e = unify_event("checking");
        assert_eq!(e.category, Some(TraceCategory::Unify));
    }
    #[test]
    fn test_warn_error_events() {
        let w = warn_event("careful");
        let e = error_event("oops");
        assert_eq!(w.level, TraceLevel::Warn);
        assert_eq!(e.level, TraceLevel::Error);
    }
    #[test]
    fn test_tracer_is_empty() {
        let t = Tracer::new(TraceLevel::Info);
        assert!(t.is_empty());
    }
    #[test]
    fn test_category_log() {
        let mut t = Tracer::new(TraceLevel::Debug);
        t.log_infer("inferring type");
        let log = t.category_log(&TraceCategory::Infer);
        assert!(!log.is_empty());
    }
}
/// Replay events from one tracer into a sink.
pub fn replay_into_sink(tracer: &Tracer, sink: &mut StringSink) {
    for event in tracer.events() {
        sink.record(event);
    }
}
/// Filter events by a predicate and return formatted strings.
pub fn filter_events<F: Fn(&TraceEvent) -> bool>(tracer: &Tracer, pred: F) -> Vec<String> {
    tracer
        .events()
        .iter()
        .filter(|e| pred(e))
        .map(|e| e.format())
        .collect()
}
/// Count events matching a predicate.
pub fn count_matching<F: Fn(&TraceEvent) -> bool>(tracer: &Tracer, pred: F) -> usize {
    tracer.events().iter().filter(|e| pred(e)).count()
}
/// Check whether the tracer has logged any event with a message containing `substr`.
pub fn has_message_containing(tracer: &Tracer, substr: &str) -> bool {
    tracer.events().iter().any(|e| e.message.contains(substr))
}
/// Build a trace event for kernel type inference.
pub fn infer_event(msg: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Debug, msg.into()).with_category(TraceCategory::Infer)
}
/// Build a trace event for definitional equality with location.
pub fn def_eq_event_at(msg: impl Into<String>, loc: impl Into<String>) -> TraceEvent {
    TraceEvent::new(TraceLevel::Debug, msg.into())
        .with_category(TraceCategory::DefEq)
        .with_location(loc)
}
/// Merge events from `src` into `dst` tracer (if below `dst`'s level).
pub fn merge_tracers(src: &Tracer, dst: &mut Tracer) {
    for event in src.events() {
        dst.log(event.clone());
    }
}
/// Summarize a tracer's reduction steps.
pub fn summarize_reductions(tracer: &Tracer) -> String {
    let steps = tracer.reduction_steps();
    if steps.is_empty() {
        "No reductions recorded.".to_string()
    } else {
        format!(
            "{} reduction(s): {}",
            steps.len(),
            steps
                .iter()
                .map(|s| s.rule.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
#[cfg(test)]
mod trace_further_tests {
    use super::*;
    use crate::Literal;
    #[test]
    fn test_string_sink_record() {
        let mut t = Tracer::new(TraceLevel::Info);
        t.info("hello");
        let mut sink = StringSink::new();
        replay_into_sink(&t, &mut sink);
        assert_eq!(sink.len(), 1);
        assert!(sink.render().contains("hello"));
    }
    #[test]
    fn test_filter_events() {
        let mut t = Tracer::new(TraceLevel::Debug);
        t.info("info msg");
        t.debug("debug msg");
        let infos = filter_events(&t, |e| e.level == TraceLevel::Info);
        assert_eq!(infos.len(), 1);
    }
    #[test]
    fn test_count_matching() {
        let mut t = Tracer::new(TraceLevel::Debug);
        t.error("e1");
        t.error("e2");
        t.info("i1");
        let n = count_matching(&t, |e| e.level == TraceLevel::Error);
        assert_eq!(n, 2);
    }
    #[test]
    fn test_has_message_containing() {
        let mut t = Tracer::new(TraceLevel::Info);
        t.info("type mismatch found");
        assert!(has_message_containing(&t, "mismatch"));
        assert!(!has_message_containing(&t, "universe"));
    }
    #[test]
    fn test_merge_tracers() {
        let mut src = Tracer::new(TraceLevel::Info);
        src.info("from source");
        let mut dst = Tracer::new(TraceLevel::Info);
        merge_tracers(&src, &mut dst);
        assert_eq!(dst.event_count(), 1);
    }
    #[test]
    fn test_summarize_reductions_empty() {
        let t = Tracer::new(TraceLevel::Trace);
        let s = summarize_reductions(&t);
        assert!(s.contains("No reductions"));
    }
    #[test]
    fn test_summarize_reductions_some() {
        let mut t = Tracer::new(TraceLevel::Trace);
        let e = Expr::Lit(Literal::Nat(1));
        t.record_reduction(ReductionRule::Beta, e.clone(), e.clone());
        let s = summarize_reductions(&t);
        assert!(s.contains("beta"));
    }
    #[test]
    fn test_infer_event_category() {
        let e = infer_event("checking sort");
        assert_eq!(e.category, Some(TraceCategory::Infer));
        assert_eq!(e.level, TraceLevel::Debug);
    }
    #[test]
    fn test_def_eq_event_at_location() {
        let e = def_eq_event_at("comparing", "line 42");
        assert_eq!(e.category, Some(TraceCategory::DefEq));
        assert_eq!(e.location, "line 42");
    }
    #[test]
    fn test_string_sink_clear() {
        let mut sink = StringSink::new();
        sink.lines.push("a".to_string());
        sink.clear();
        assert!(sink.is_empty());
    }
}
/// Filter events from a tracer using a filter.
pub fn filtered_events<'a>(tracer: &'a Tracer, filter: &'a TraceFilter) -> Vec<&'a TraceEvent> {
    tracer
        .events()
        .iter()
        .filter(|e| filter.accepts(e))
        .collect()
}
/// Count events in a tracer matching a filter.
pub fn count_filtered(tracer: &Tracer, filter: &TraceFilter) -> usize {
    filtered_events(tracer, filter).len()
}
/// Extract all unique categories present in a tracer's events.
pub fn unique_categories(tracer: &Tracer) -> Vec<TraceCategory> {
    let mut cats: Vec<TraceCategory> = tracer
        .events()
        .iter()
        .filter_map(|e| e.category.clone())
        .collect();
    cats.dedup();
    cats
}
#[cfg(test)]
mod trace_filter_tests {
    use super::*;
    #[test]
    fn test_trace_filter_level_excludes() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.debug("a debug message");
        t.info("an info message");
        let filter = TraceFilter::at_level(TraceLevel::Info);
        let events = filtered_events(&t, &filter);
        assert!(events.iter().all(|e| e.level >= TraceLevel::Info));
    }
    #[test]
    fn test_trace_filter_category() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.trace_infer("infer something");
        t.trace_reduce("reduce something");
        let filter =
            TraceFilter::at_level(TraceLevel::Trace).with_categories(vec![TraceCategory::Infer]);
        let events = filtered_events(&t, &filter);
        assert!(events
            .iter()
            .all(|e| e.category == Some(TraceCategory::Infer)));
    }
    #[test]
    fn test_trace_filter_exclude_text() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.info("universe check");
        t.info("type inference");
        let filter = TraceFilter::at_level(TraceLevel::Info).excluding("universe");
        let events = filtered_events(&t, &filter);
        assert!(events.iter().all(|e| !e.message.contains("universe")));
    }
    #[test]
    fn test_count_filtered() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.info("msg1");
        t.info("msg2");
        t.debug("msg3");
        let filter = TraceFilter::at_level(TraceLevel::Info);
        assert_eq!(count_filtered(&t, &filter), 2);
    }
    #[test]
    fn test_unique_categories_empty() {
        let t = Tracer::new(TraceLevel::Off);
        let cats = unique_categories(&t);
        assert!(cats.is_empty());
    }
    #[test]
    fn test_unique_categories_non_empty() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.trace_infer("x");
        t.trace_reduce("y");
        let cats = unique_categories(&t);
        assert!(!cats.is_empty());
    }
    #[test]
    fn test_trace_filter_default_accepts_nothing() {
        let mut t = Tracer::new(TraceLevel::Trace);
        t.info("hi");
        let filter = TraceFilter::default();
        let events = filtered_events(&t, &filter);
        let _ = events;
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
