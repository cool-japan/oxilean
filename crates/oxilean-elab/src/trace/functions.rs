//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use super::types::{
    CountingTraceSink, DeduplicateStep, ElabTracer, LevelFilterStep, NullTraceSink, PrefixEnricher,
    TraceAggregator, TraceAnnotation, TraceAnnotationKind, TraceBuffer, TraceCategory,
    TraceContext, TraceDispatcher, TraceElabNode, TraceEvent, TraceEventCounter, TraceExportFormat,
    TraceExporter, TraceFilter, TraceLevel, TraceMask, TraceMetricsSnapshot, TracePipeline,
    TraceProfiler, TraceQueryEngine, TraceReport, TraceSessionStats, TraceSpan, TraceSpanCollector,
    TruncateStep, VecTraceSink,
};
use oxilean_kernel::*;

static GLOBAL_TRACER: OnceLock<Mutex<Option<ElabTracer>>> = OnceLock::new();
fn get_global_tracer() -> &'static Mutex<Option<ElabTracer>> {
    GLOBAL_TRACER.get_or_init(|| Mutex::new(None))
}
/// Initializes the global tracer with the given verbosity level.
pub fn init_tracer(level: TraceLevel) {
    let mut tracer = get_global_tracer()
        .lock()
        .expect("global tracer mutex should not be poisoned");
    *tracer = Some({
        let mut t = ElabTracer::new();
        t.set_level(level);
        t
    });
}
/// Closes the global tracer and discards all recorded events.
pub fn close_tracer() {
    let mut tracer = get_global_tracer()
        .lock()
        .expect("global tracer mutex should not be poisoned");
    *tracer = None;
}
/// Records a trace event in the global tracer.
pub fn trace_global(event: TraceEvent) {
    if let Ok(mut tracer) = get_global_tracer().lock() {
        if let Some(ref mut t) = *tracer {
            t.trace(event);
        }
    }
}
/// Returns a formatted string of all events in the global tracer.
pub fn format_global_events() -> String {
    if let Ok(tracer) = get_global_tracer().lock() {
        if let Some(ref t) = *tracer {
            return t.format_events();
        }
    }
    String::new()
}
/// Exports all events from the global tracer to a file.
pub fn export_global_trace(path: &str) -> std::io::Result<()> {
    if let Ok(tracer) = get_global_tracer().lock() {
        if let Some(ref t) = *tracer {
            return t.export_to_file(path);
        }
    }
    Ok(())
}
/// Clears all events from the global tracer.
pub fn clear_global_trace() {
    if let Ok(mut tracer) = get_global_tracer().lock() {
        if let Some(ref mut t) = *tracer {
            t.clear();
        }
    }
}
/// Formats trace events as a tree with nested indentation showing hierarchy.
pub fn format_trace_tree(events: &[TraceEvent]) -> String {
    let mut result = String::from("Elaboration Trace Tree:\n");
    let mut indent_level = 0;
    for event in events {
        let indent = indent_lines(&event.message, indent_level);
        result.push_str(&format!(
            "{} [{}] {}\n",
            " ".repeat(indent_level * 2),
            event.level.as_str(),
            indent
        ));
        if matches!(
            event.category,
            TraceCategory::Elaboration
                | TraceCategory::TypeInference
                | TraceCategory::InstanceSynthesis
        ) {
            indent_level = (indent_level + 1).min(5);
        }
    }
    result
}
/// Formats trace events as a timeline with relative timestamps.
pub fn format_trace_timeline(events: &[TraceEvent]) -> String {
    let mut result = String::from("Elaboration Trace Timeline:\n");
    let mut prev_timestamp = 0u64;
    for event in events {
        let delta = if prev_timestamp == 0 {
            0u64
        } else {
            event.timestamp.saturating_sub(prev_timestamp)
        };
        result.push_str(&format!(
            "T+{}ms [{:6}] {}: {}\n",
            delta,
            event.level.as_str(),
            event.category.as_str(),
            event.message.lines().next().unwrap_or("")
        ));
        prev_timestamp = event.timestamp;
    }
    result
}
/// Formats trace events as a summary with statistics grouped by level and category.
pub fn format_trace_summary(events: &[TraceEvent]) -> String {
    let mut result = String::from("Elaboration Trace Summary:\n");
    result.push_str(&format!("Total Events: {}\n\n", events.len()));
    result.push_str("By Level:\n");
    let mut level_counts = HashMap::new();
    for event in events {
        *level_counts.entry(event.level).or_insert(0) += 1;
    }
    for level in &[
        TraceLevel::Error,
        TraceLevel::Warn,
        TraceLevel::Info,
        TraceLevel::Debug,
        TraceLevel::Trace,
    ] {
        let count = level_counts.get(level).unwrap_or(&0);
        result.push_str(&format!("  {}: {}\n", level.as_str(), count));
    }
    result.push_str("\nBy Category:\n");
    let mut category_counts = HashMap::new();
    for event in events {
        *category_counts.entry(event.category).or_insert(0) += 1;
    }
    let mut categories: Vec<_> = category_counts.iter().collect();
    categories.sort_by_key(|&(cat, _)| cat.as_str());
    for (category, count) in categories {
        result.push_str(&format!("  {}: {}\n", category.as_str(), count));
    }
    result
}
/// Returns the current Unix timestamp in milliseconds.
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
/// Formats a Unix timestamp in milliseconds as a human-readable string.
pub fn format_timestamp(ts: u64) -> String {
    let secs = ts / 1000;
    let millis = ts % 1000;
    format!("{:010}.{:03}", secs, millis)
}
/// Indents all lines in a string by the given number of levels (2 spaces per level).
pub fn indent_lines(text: &str, level: usize) -> String {
    let indent = " ".repeat(level * 2);
    text.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Escapes special characters in a string for safe display.
pub fn escape_special_chars(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('"', "\\\"")
}
/// Macro for elaboration tracing
#[macro_export]
macro_rules! trace_elab {
    ($msg:expr) => {
        let event = $crate::trace::TraceEvent::new(
            $crate::trace::TraceLevel::Trace,
            $crate::trace::TraceCategory::Elaboration,
            $msg.to_string(),
        );
        $crate::trace::trace_global(event);
    };
}
/// Macro for type inference tracing
#[macro_export]
macro_rules! trace_infer {
    ($msg:expr) => {
        let event = $crate::trace::TraceEvent::new(
            $crate::trace::TraceLevel::Debug,
            $crate::trace::TraceCategory::TypeInference,
            $msg.to_string(),
        );
        $crate::trace::trace_global(event);
    };
}
/// Macro for unification tracing
#[macro_export]
macro_rules! trace_unify {
    ($msg:expr) => {
        let event = $crate::trace::TraceEvent::new(
            $crate::trace::TraceLevel::Debug,
            $crate::trace::TraceCategory::Unification,
            $msg.to_string(),
        );
        $crate::trace::trace_global(event);
    };
}
/// Macro for instance synthesis tracing
#[macro_export]
macro_rules! trace_instance {
    ($msg:expr) => {
        let event = $crate::trace::TraceEvent::new(
            $crate::trace::TraceLevel::Debug,
            $crate::trace::TraceCategory::InstanceSynthesis,
            $msg.to_string(),
        );
        $crate::trace::trace_global(event);
    };
}
/// Macro for tactic tracing
#[macro_export]
macro_rules! trace_tactic {
    ($msg:expr) => {
        let event = $crate::trace::TraceEvent::new(
            $crate::trace::TraceLevel::Debug,
            $crate::trace::TraceCategory::Tactic,
            $msg.to_string(),
        );
        $crate::trace::trace_global(event);
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::*;
    #[test]
    fn test_trace_level_ordering() {
        assert!(TraceLevel::Off.level() < TraceLevel::Error.level());
        assert!(TraceLevel::Error.level() < TraceLevel::Warn.level());
        assert!(TraceLevel::Warn.level() < TraceLevel::Info.level());
        assert!(TraceLevel::Info.level() < TraceLevel::Debug.level());
        assert!(TraceLevel::Debug.level() < TraceLevel::Trace.level());
    }
    #[test]
    fn test_trace_level_display() {
        assert_eq!(TraceLevel::Error.as_str(), "ERROR");
        assert_eq!(TraceLevel::Warn.as_str(), "WARN");
        assert_eq!(TraceLevel::Debug.as_str(), "DEBUG");
    }
    #[test]
    fn test_trace_category_display() {
        assert_eq!(TraceCategory::Elaboration.as_str(), "Elaboration");
        assert_eq!(TraceCategory::TypeInference.as_str(), "TypeInference");
        assert_eq!(TraceCategory::Tactic.as_str(), "Tactic");
    }
    #[test]
    fn test_trace_event_new() {
        let event = TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test message".to_string(),
        );
        assert_eq!(event.level, TraceLevel::Info);
        assert_eq!(event.category, TraceCategory::Elaboration);
    }
    #[test]
    fn test_trace_event_span() {
        let event = TraceEvent::new(
            TraceLevel::Debug,
            TraceCategory::TypeInference,
            "Inferring type".to_string(),
        )
        .with_span("main.ox:42:5".to_string());
        assert_eq!(event.span, Some("main.ox:42:5".to_string()));
    }
    #[test]
    fn test_trace_event_context() {
        let event = TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Elaborating".to_string(),
        )
        .with_context("name".to_string(), "foo".to_string())
        .with_context("type".to_string(), "Nat".to_string());
        assert_eq!(event.context.len(), 2);
    }
    #[test]
    fn test_elab_tracer_new() {
        let tracer = ElabTracer::default();
        assert_eq!(tracer.level, TraceLevel::Off);
        assert_eq!(tracer.event_count(), 0);
    }
    #[test]
    fn test_set_level() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Debug);
        assert_eq!(tracer.level(), TraceLevel::Debug);
    }
    #[test]
    fn test_trace_records_event() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Trace,
            TraceCategory::Elaboration,
            "Test".to_string(),
        ));
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_level_filtering() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Info);
        tracer.trace(TraceEvent::new(
            TraceLevel::Debug,
            TraceCategory::Elaboration,
            "Debug".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Info".to_string(),
        ));
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_category_enable_disable() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.disable_category(TraceCategory::Elaboration);
        tracer.trace(TraceEvent::new(
            TraceLevel::Trace,
            TraceCategory::Elaboration,
            "Test".to_string(),
        ));
        assert_eq!(tracer.event_count(), 0);
        tracer.enable_category(TraceCategory::Elaboration);
        tracer.trace(TraceEvent::new(
            TraceLevel::Trace,
            TraceCategory::Elaboration,
            "Test2".to_string(),
        ));
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_filter_level() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Error,
            TraceCategory::Elaboration,
            "Error".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Warn,
            TraceCategory::Elaboration,
            "Warn".to_string(),
        ));
        assert_eq!(tracer.filter_by_level(TraceLevel::Error).len(), 1);
    }
    #[test]
    fn test_filter_category() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Elab".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::TypeInference,
            "Infer".to_string(),
        ));
        assert_eq!(
            tracer.filter_by_category(TraceCategory::Elaboration).len(),
            1
        );
    }
    #[test]
    fn test_filter_both() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Debug,
            TraceCategory::Elaboration,
            "Debug Elab".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Info Elab".to_string(),
        ));
        let filtered = tracer.filter_events(TraceLevel::Debug, TraceCategory::Elaboration);
        assert_eq!(filtered.len(), 1);
    }
    #[test]
    fn test_count_at_level() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Error,
            TraceCategory::Elaboration,
            "E1".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Error,
            TraceCategory::Elaboration,
            "E2".to_string(),
        ));
        assert_eq!(tracer.event_count_at_level(TraceLevel::Error), 2);
    }
    #[test]
    fn test_count_in_category() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "E1".to_string(),
        ));
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "E2".to_string(),
        ));
        assert_eq!(
            tracer.event_count_in_category(TraceCategory::Elaboration),
            2
        );
    }
    #[test]
    fn test_clear() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Trace);
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test".to_string(),
        ));
        assert_eq!(tracer.event_count(), 1);
        tracer.clear();
        assert_eq!(tracer.event_count(), 0);
    }
    #[test]
    fn test_format_event() {
        let event = TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test message".to_string(),
        );
        let formatted = event.format();
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("Elaboration"));
    }
    #[test]
    fn test_timestamp() {
        let ts = 1234567890123u64;
        let formatted = format_timestamp(ts);
        assert!(formatted.contains("1234567890"));
    }
    #[test]
    fn test_escape_chars() {
        let text = "hello\nworld";
        let escaped = escape_special_chars(text);
        assert!(escaped.contains("\\n"));
    }
    #[test]
    fn test_summary() {
        let events = vec![
            TraceEvent::new(
                TraceLevel::Error,
                TraceCategory::Elaboration,
                "E1".to_string(),
            ),
            TraceEvent::new(
                TraceLevel::Info,
                TraceCategory::TypeInference,
                "I1".to_string(),
            ),
        ];
        let summary = format_trace_summary(&events);
        assert!(summary.contains("Total Events: 2"));
    }
    #[test]
    fn test_timeline() {
        let events = vec![
            TraceEvent::new(
                TraceLevel::Info,
                TraceCategory::Elaboration,
                "Event 1".to_string(),
            ),
            TraceEvent::new(
                TraceLevel::Debug,
                TraceCategory::TypeInference,
                "Event 2".to_string(),
            ),
        ];
        let timeline = format_trace_timeline(&events);
        assert!(timeline.contains("Timeline"));
    }
    #[test]
    fn test_indent() {
        let text = "line1\nline2";
        let indented = indent_lines(text, 1);
        assert!(indented.contains("  line1"));
    }
    #[test]
    fn test_timestamp_order() {
        let ts1 = current_timestamp();
        let ts2 = current_timestamp();
        assert!(ts2 >= ts1);
    }
    #[test]
    fn test_debug_log() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Debug);
        tracer.debug(TraceCategory::TypeInference, "Debug message".to_string());
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_info_log() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Info);
        tracer.info(TraceCategory::Elaboration, "Info message".to_string());
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_warn_log() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Warn);
        tracer.warn(TraceCategory::Elaboration, "Warn message".to_string());
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_error_log() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Error);
        tracer.error(TraceCategory::Elaboration, "Error message".to_string());
        assert_eq!(tracer.event_count(), 1);
    }
    #[test]
    fn test_tree() {
        let events = vec![
            TraceEvent::new(
                TraceLevel::Info,
                TraceCategory::Elaboration,
                "Event 1".to_string(),
            ),
            TraceEvent::new(
                TraceLevel::Info,
                TraceCategory::TypeInference,
                "Event 2".to_string(),
            ),
        ];
        let tree = format_trace_tree(&events);
        assert!(tree.contains("Tree"));
    }
    #[test]
    fn test_contexts_batch() {
        let contexts = [
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let event = TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test".to_string(),
        )
        .with_contexts(contexts);
        assert_eq!(event.context.len(), 2);
    }
    #[test]
    fn test_export() {
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Info);
        tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test export".to_string(),
        ));
        let path = "/tmp/trace_test.log";
        let result = tracer.export_to_file(path);
        assert!(result.is_ok());
    }
    #[test]
    fn test_global_clear_first() {
        close_tracer();
        init_tracer(TraceLevel::Info);
        trace_global(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Test".to_string(),
        ));
        clear_global_trace();
        let _formatted = format_global_events();
        close_tracer();
    }
    #[test]
    fn test_global_init_only() {
        let mut local_tracer = ElabTracer::default();
        local_tracer.set_level(TraceLevel::Info);
        local_tracer.trace(TraceEvent::new(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "Global test".to_string(),
        ));
        let formatted = local_tracer.format_events();
        assert!(formatted.contains("Global test"));
    }
}
#[allow(dead_code)]
pub trait TraceSink: Send + Sync {
    fn sink_name(&self) -> &'static str;
    fn write_event(&mut self, event: &TraceEvent);
    fn flush(&mut self) {}
}
#[cfg(test)]
mod extended_trace_tests {
    use super::*;
    use crate::trace::*;
    fn make_event(level: TraceLevel, cat: TraceCategory, msg: &str) -> TraceEvent {
        TraceEvent::new(level, cat, msg.to_string())
    }
    #[test]
    fn test_trace_buffer_ring() {
        let mut buf = TraceBuffer::new(3);
        for i in 0..5 {
            buf.push(make_event(
                TraceLevel::Info,
                TraceCategory::Elaboration,
                &format!("msg{}", i),
            ));
        }
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.dropped(), 2);
    }
    #[test]
    fn test_trace_buffer_latest() {
        let mut buf = TraceBuffer::new(10);
        for i in 0..5 {
            buf.push(make_event(
                TraceLevel::Debug,
                TraceCategory::Unification,
                &format!("ev{}", i),
            ));
        }
        let latest = buf.latest(2);
        assert_eq!(latest.len(), 2);
        assert!(latest[1].message.contains("ev4"));
    }
    #[test]
    fn test_trace_filter_accepts() {
        let filter = TraceFilter::new(TraceLevel::Warn).enable_category(TraceCategory::Elaboration);
        let good = make_event(TraceLevel::Error, TraceCategory::Elaboration, "err");
        let bad_level = make_event(TraceLevel::Debug, TraceCategory::Elaboration, "dbg");
        let bad_cat = make_event(TraceLevel::Error, TraceCategory::Unification, "err2");
        assert!(filter.accepts(&good));
        assert!(!filter.accepts(&bad_level));
        assert!(!filter.accepts(&bad_cat));
    }
    #[test]
    fn test_trace_filter_blocked_message() {
        let filter = TraceFilter::new(TraceLevel::Debug).block_message("secret");
        let ev = make_event(TraceLevel::Debug, TraceCategory::Elaboration, "secret data");
        assert!(!filter.accepts(&ev));
    }
    #[test]
    fn test_vec_trace_sink() {
        let mut sink = VecTraceSink::new();
        sink.write_event(&make_event(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "hello",
        ));
        sink.write_event(&make_event(
            TraceLevel::Error,
            TraceCategory::Tactic,
            "oops",
        ));
        assert_eq!(sink.events().len(), 2);
    }
    #[test]
    fn test_counting_trace_sink() {
        let mut sink = CountingTraceSink::new();
        sink.write_event(&make_event(
            TraceLevel::Error,
            TraceCategory::Elaboration,
            "e1",
        ));
        sink.write_event(&make_event(
            TraceLevel::Error,
            TraceCategory::Elaboration,
            "e2",
        ));
        sink.write_event(&make_event(
            TraceLevel::Warn,
            TraceCategory::Elaboration,
            "w1",
        ));
        assert_eq!(sink.count(TraceLevel::Error), 2);
        assert_eq!(sink.count(TraceLevel::Warn), 1);
        assert_eq!(sink.total(), 3);
    }
    #[test]
    fn test_trace_dispatcher() {
        let mut dispatcher = TraceDispatcher::new().add_sink(NullTraceSink);
        let ev = make_event(TraceLevel::Debug, TraceCategory::Elaboration, "hi");
        dispatcher.dispatch(&ev);
        assert_eq!(dispatcher.sink_count(), 1);
    }
    #[test]
    fn test_trace_span_duration() {
        let span = TraceSpan::new("test_op", TraceCategory::Unification, 100).close(250);
        assert_eq!(span.duration_us(), Some(150));
        assert!(span.is_closed());
    }
    #[test]
    fn test_trace_span_collector_slowest() {
        let mut col = TraceSpanCollector::new();
        col.begin("fast", TraceCategory::Elaboration, 0);
        col.end(10);
        col.begin("slow", TraceCategory::Elaboration, 20);
        col.end(200);
        let slowest = col.slowest_span().expect("test operation should succeed");
        assert_eq!(slowest.name, "slow");
    }
    #[test]
    fn test_trace_exporter_json() {
        let exporter = TraceExporter::new(TraceExportFormat::Json);
        let events = vec![make_event(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "hello",
        )];
        let out = exporter.export(&events);
        assert!(out.starts_with('['));
        assert!(out.ends_with(']'));
        assert!(out.contains("hello"));
    }
    #[test]
    fn test_trace_exporter_csv() {
        let exporter = TraceExporter::new(TraceExportFormat::Csv);
        let events = vec![make_event(
            TraceLevel::Warn,
            TraceCategory::Tactic,
            "warn msg",
        )];
        let out = exporter.export(&events);
        assert!(out.starts_with("level,category,message\n"));
        assert!(out.contains("warn msg"));
    }
    #[test]
    fn test_trace_aggregator() {
        let mut agg = TraceAggregator::new();
        agg.ingest(make_event(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "a",
        ));
        agg.ingest(make_event(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "b",
        ));
        agg.ingest(make_event(
            TraceLevel::Error,
            TraceCategory::Unification,
            "c",
        ));
        assert_eq!(agg.count("Info", "Elaboration"), 2);
        assert_eq!(agg.count("Error", "Unification"), 1);
        assert_eq!(agg.total(), 3);
    }
    #[test]
    fn test_trace_profiler_violations() {
        let mut profiler = TraceProfiler::new().set_threshold("unify", 1000);
        profiler.record("unify", 500);
        profiler.record("unify", 2000);
        let violations = profiler.violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].1, 2000);
    }
    #[test]
    fn test_trace_context_baggage() {
        let ctx = TraceContext::new("trace-001")
            .with_parent("trace-000")
            .with_baggage("user", "alice");
        assert!(ctx.is_sampled());
        assert_eq!(ctx.get_baggage("user"), Some(&"alice".to_string()));
    }
    #[test]
    fn test_trace_report_summary() {
        let mut tracer = ElabTracer::default();
        tracer.set_level(TraceLevel::Debug);
        tracer.trace(make_event(
            TraceLevel::Info,
            TraceCategory::Elaboration,
            "x",
        ));
        let report = TraceReport::from_tracer(&tracer);
        assert_eq!(report.total_events, 1);
        let summary = report.summary();
        assert!(summary.contains("TraceReport"));
    }
}
#[allow(dead_code)]
pub trait TraceEventEnricher: Send + Sync {
    fn enrich(&self, event: &mut TraceEvent);
}
#[allow(dead_code)]
pub trait TracePipelineStep: Send + Sync {
    fn step_name(&self) -> &'static str;
    fn process(&self, events: Vec<TraceEvent>) -> Vec<TraceEvent>;
}
#[cfg(test)]
mod extended_trace_tests2 {
    use super::*;
    use crate::trace::*;
    fn make_event(level: TraceLevel, cat: TraceCategory, msg: &str) -> TraceEvent {
        TraceEvent::new(level, cat, msg.to_string())
    }
    #[test]
    fn test_trace_annotation() {
        let ann =
            TraceAnnotation::new(TraceAnnotationKind::Success).with_note("unified in 3 steps");
        assert!(ann.is_success());
        assert!(!ann.is_failure());
        assert!(ann.note.is_some());
    }
    #[test]
    fn test_prefix_enricher() {
        let enricher = PrefixEnricher::new("MODULE");
        let mut ev = make_event(TraceLevel::Info, TraceCategory::Elaboration, "hello");
        enricher.enrich(&mut ev);
        assert!(ev.message.starts_with("[MODULE]"));
    }
    #[test]
    fn test_trace_pipeline_steps() {
        let pipeline = TracePipeline::new()
            .add_step(DeduplicateStep)
            .add_step(LevelFilterStep::new(TraceLevel::Warn))
            .add_step(TruncateStep::new(5));
        assert_eq!(
            pipeline.step_names(),
            vec!["deduplicate", "level_filter", "truncate"]
        );
    }
    #[test]
    fn test_trace_pipeline_dedup() {
        let pipeline = TracePipeline::new().add_step(DeduplicateStep);
        let events = vec![
            make_event(TraceLevel::Info, TraceCategory::Elaboration, "hello"),
            make_event(TraceLevel::Info, TraceCategory::Elaboration, "hello"),
            make_event(TraceLevel::Info, TraceCategory::Elaboration, "world"),
        ];
        let result = pipeline.run(events);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_trace_pipeline_level_filter() {
        let pipeline = TracePipeline::new().add_step(LevelFilterStep::new(TraceLevel::Error));
        let events = vec![
            make_event(TraceLevel::Debug, TraceCategory::Elaboration, "debug"),
            make_event(TraceLevel::Error, TraceCategory::Elaboration, "error"),
        ];
        let result = pipeline.run(events);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].level, TraceLevel::Error);
    }
    #[test]
    fn test_trace_session_stats() {
        let mut stats = TraceSessionStats::new("sess-001");
        stats.record_event(TraceLevel::Error);
        stats.record_event(TraceLevel::Warn);
        stats.record_event(TraceLevel::Info);
        stats.record_span(500);
        stats.record_span(1500);
        assert_eq!(stats.total_events, 3);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.warn_count, 1);
        assert!(stats.has_errors());
        assert_eq!(stats.slowest_us, 1500);
        assert!((stats.mean_span_us() - 1000.0).abs() < 1.0);
    }
    #[test]
    fn test_trace_mask_operations() {
        let mut mask = TraceMask::NONE;
        assert!(mask.is_none());
        mask.set(3);
        mask.set(7);
        assert!(mask.is_set(3));
        assert!(mask.is_set(7));
        assert!(!mask.is_set(1));
        mask.clear(3);
        assert!(!mask.is_set(3));
        let m2 = TraceMask::from_categories(&[1, 2, 3]);
        assert!(m2.is_set(1));
        assert!(m2.is_set(2));
        assert!(!m2.is_set(4));
    }
    #[test]
    fn test_trace_mask_union_intersection() {
        let a = TraceMask::from_categories(&[0, 1]);
        let b = TraceMask::from_categories(&[1, 2]);
        let u = a.union(b);
        assert!(u.is_set(0) && u.is_set(1) && u.is_set(2));
        let i = a.intersection(b);
        assert!(!i.is_set(0) && i.is_set(1) && !i.is_set(2));
    }
    #[test]
    fn test_trace_elab_node_depth() {
        let child = TraceElabNode::new("child", TraceLevel::Debug, TraceCategory::Elaboration);
        let root = TraceElabNode::new("root", TraceLevel::Info, TraceCategory::Elaboration)
            .with_child(child.clone())
            .with_child(
                TraceElabNode::new("mid", TraceLevel::Info, TraceCategory::Elaboration)
                    .with_child(child),
            );
        assert_eq!(root.depth(), 2);
        assert_eq!(root.total_nodes(), 4);
    }
    #[test]
    fn test_trace_elab_node_render() {
        let node = TraceElabNode::new(
            "unify ?a = Nat",
            TraceLevel::Debug,
            TraceCategory::Unification,
        )
        .with_child(TraceElabNode::new(
            "check Nat : Sort 0",
            TraceLevel::Debug,
            TraceCategory::Elaboration,
        ));
        let rendered = node.render(0);
        assert!(rendered.contains("unify ?a = Nat"));
        assert!(rendered.contains("check Nat"));
    }
}
#[allow(dead_code)]
pub fn trace_group_by_category(events: &[TraceEvent]) -> HashMap<String, Vec<&TraceEvent>> {
    let mut map: HashMap<String, Vec<&TraceEvent>> = HashMap::new();
    for e in events {
        map.entry(format!("{:?}", e.category)).or_default().push(e);
    }
    map
}
#[allow(dead_code)]
pub fn trace_group_by_level(events: &[TraceEvent]) -> HashMap<String, Vec<&TraceEvent>> {
    let mut map: HashMap<String, Vec<&TraceEvent>> = HashMap::new();
    for e in events {
        map.entry(format!("{:?}", e.level)).or_default().push(e);
    }
    map
}
#[cfg(test)]
mod trace_query_tests {
    use super::*;
    use crate::trace::*;
    fn ev(level: TraceLevel, cat: TraceCategory, msg: &str) -> TraceEvent {
        TraceEvent::new(level, cat, msg.to_string())
    }
    #[test]
    fn test_query_by_level() {
        let events = vec![
            ev(TraceLevel::Error, TraceCategory::Elaboration, "err"),
            ev(TraceLevel::Info, TraceCategory::Elaboration, "info"),
            ev(TraceLevel::Error, TraceCategory::Unification, "err2"),
        ];
        let q = TraceQueryEngine::new(&events);
        assert_eq!(q.by_level(TraceLevel::Error).len(), 2);
        assert_eq!(q.by_level(TraceLevel::Info).len(), 1);
        assert!(q.any_error());
    }
    #[test]
    fn test_query_containing() {
        let events = vec![
            ev(
                TraceLevel::Debug,
                TraceCategory::Elaboration,
                "found metavariable",
            ),
            ev(
                TraceLevel::Debug,
                TraceCategory::Elaboration,
                "checking term",
            ),
        ];
        let q = TraceQueryEngine::new(&events);
        assert_eq!(q.containing("metavariable").len(), 1);
        assert_eq!(q.containing("checking").len(), 1);
        assert_eq!(q.containing("nothing").len(), 0);
    }
    #[test]
    fn test_trace_group_by() {
        let events = vec![
            ev(TraceLevel::Info, TraceCategory::Elaboration, "a"),
            ev(TraceLevel::Info, TraceCategory::Unification, "b"),
            ev(TraceLevel::Error, TraceCategory::Elaboration, "c"),
        ];
        let grouped = trace_group_by_category(&events);
        assert_eq!(grouped["Elaboration"].len(), 2);
        assert_eq!(grouped["Unification"].len(), 1);
    }
    #[test]
    fn test_event_counter() {
        let events = vec![
            ev(TraceLevel::Error, TraceCategory::Elaboration, "e1"),
            ev(TraceLevel::Error, TraceCategory::Elaboration, "e2"),
            ev(TraceLevel::Info, TraceCategory::Elaboration, "i1"),
        ];
        let mut counter = TraceEventCounter::new()
            .add_counter("errors", |e| e.level == TraceLevel::Error)
            .add_counter("elab_events", |e| e.category == TraceCategory::Elaboration);
        counter.process_all(&events);
        assert_eq!(counter.count("errors"), 2);
        assert_eq!(counter.count("elab_events"), 3);
    }
}
#[cfg(test)]
mod trace_metrics_tests {
    use super::*;
    use crate::trace::*;
    #[test]
    fn test_metrics_snapshot_health() {
        let mut snap = TraceMetricsSnapshot::new(1000);
        snap.error_rate = 0.01;
        snap.buffer_utilization = 0.5;
        assert!(snap.is_healthy());
        assert_eq!(snap.health_status(), "healthy");
        snap.error_rate = 0.12;
        assert!(!snap.is_healthy());
        assert_eq!(snap.health_status(), "critical");
    }
}
