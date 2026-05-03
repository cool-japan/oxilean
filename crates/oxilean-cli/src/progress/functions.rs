//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::time::{Duration, Instant};

use super::types::{
    BatchItemStatus, BatchProgress, ColorMode, EtaEstimator, LogLevel, MultiProgressTracker,
    MultiTaskProgress, NodeStatus, PhaseTimeline, PhaseTransition, ProgressBar, ProgressBatch,
    ProgressCheckpointLog, ProgressLog, ProgressNode, ProgressReporter, Spinner, SpinnerStyle,
    StepProgress, ThroughputDisplay,
};

/// Attempt to read the terminal width. Falls back to 80 if unavailable.
pub fn terminal_width() -> usize {
    if let Ok(s) = std::env::var("COLUMNS") {
        if let Ok(n) = s.trim().parse::<usize>() {
            if n > 0 {
                return n;
            }
        }
    }
    80
}
pub fn color_green(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[32;1m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_yellow(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[33;1m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_red(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[31;1m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_cyan(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[36m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_dim(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[2m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_bold(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[1m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_magenta(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[35m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
pub fn color_blue(text: &str, mode: ColorMode) -> String {
    if mode.enabled() {
        format!("\x1b[34m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}
/// Render a progress bar segment of the given width.
///
/// Returns a string like `"========--------"`.
pub fn render_bar(current: usize, total: usize, width: usize, fill: char, empty: char) -> String {
    let filled = (current * width).checked_div(total).unwrap_or(width);
    let empty_count = width.saturating_sub(filled);
    format!(
        "{}{}",
        fill.to_string().repeat(filled),
        empty.to_string().repeat(empty_count)
    )
}
/// Format a byte count for human display (B, KB, MB, GB).
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
/// Format a count with thousands separators (commas).
pub fn format_count(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let digits: Vec<char> = s.chars().collect();
    let len = digits.len();
    for (i, ch) in digits.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}
/// Format a duration as `"Xh Ym Zs"` or `"Xm Ys"` or `"Xs"` or `"Xms"`.
pub fn format_duration(d: Duration) -> String {
    let total_ms = d.as_millis();
    if total_ms < 1000 {
        return format!("{}ms", total_ms);
    }
    let total_secs = d.as_secs();
    if total_secs < 60 {
        return format!("{}s", total_secs);
    }
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    if mins < 60 {
        return format!("{}m {}s", mins, secs);
    }
    let hours = mins / 60;
    let mins_rem = mins % 60;
    format!("{}h {}m {}s", hours, mins_rem, secs)
}
/// Format a rate (items per second).
pub fn format_rate(items_per_sec: f64) -> String {
    if items_per_sec >= 1_000_000.0 {
        format!("{:.1}M/s", items_per_sec / 1_000_000.0)
    } else if items_per_sec >= 1000.0 {
        format!("{:.1}K/s", items_per_sec / 1000.0)
    } else if items_per_sec >= 1.0 {
        format!("{:.1}/s", items_per_sec)
    } else {
        format!("{:.2}/s", items_per_sec)
    }
}
/// Serialize a ProgressBar state to JSON.
pub fn progress_to_json(pb: &ProgressBar) -> String {
    format!(
        r#"{{"current":{},"total":{},"percentage":{:.2},"label":"{}","complete":{}}}"#,
        pb.current,
        pb.total,
        pb.percentage(),
        pb.label.replace('"', "\\\""),
        pb.is_complete()
    )
}
/// Serialize a ProgressReporter state to JSON.
pub fn reporter_to_json(pr: &ProgressReporter) -> String {
    let mut phase_parts = Vec::new();
    for phase in &pr.phases {
        if let Some((cur, tot)) = pr.phase_progress.get(phase) {
            let pct = if *tot == 0 {
                100.0
            } else {
                (*cur as f64 / *tot as f64) * 100.0
            };
            phase_parts.push(format!(
                r#"{{"name":"{}","current":{},"total":{},"percentage":{:.2}}}"#,
                phase.replace('"', "\\\""),
                cur,
                tot,
                pct
            ));
        }
    }
    format!(
        r#"{{"phases":[{}],"overall_percentage":{:.2},"elapsed_ms":{}}}"#,
        phase_parts.join(","),
        pr.overall_percentage(),
        pr.started_at.elapsed().as_millis()
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn progress_bar_new() {
        let pb = ProgressBar::new(10, "parsing");
        assert_eq!(pb.total, 10);
        assert_eq!(pb.current, 0);
        assert_eq!(pb.label, "parsing");
    }
    #[test]
    fn progress_bar_increment() {
        let mut pb = ProgressBar::new(5, "test");
        pb.increment();
        pb.increment();
        assert_eq!(pb.current, 2);
    }
    #[test]
    fn progress_bar_no_overflow() {
        let mut pb = ProgressBar::new(2, "t");
        pb.increment();
        pb.increment();
        pb.increment();
        assert_eq!(pb.current, 2);
    }
    #[test]
    fn progress_bar_is_complete() {
        let mut pb = ProgressBar::new(3, "t");
        assert!(!pb.is_complete());
        pb.increment();
        pb.increment();
        pb.increment();
        assert!(pb.is_complete());
    }
    #[test]
    fn progress_bar_reset() {
        let mut pb = ProgressBar::new(5, "t");
        pb.increment();
        pb.increment();
        pb.reset();
        assert_eq!(pb.current, 0);
        assert!(!pb.is_complete());
    }
    #[test]
    fn progress_bar_render_format() {
        let mut pb = ProgressBar::new(20, "checking");
        for _ in 0..10 {
            pb.increment();
        }
        let s = pb.render();
        assert!(s.starts_with('['));
        assert!(s.contains("10/20"));
        assert!(s.contains("checking"));
    }
    #[test]
    fn progress_bar_percentage() {
        let mut pb = ProgressBar::new(4, "t");
        pb.increment();
        pb.increment();
        assert!((pb.percentage() - 50.0).abs() < 0.1);
    }
    #[test]
    fn progress_bar_with_chars() {
        let mut pb = ProgressBar::with_chars(10, "t", '#', '.');
        pb.set_current(5);
        let s = pb.render();
        assert!(s.contains('#'));
        assert!(s.contains('.'));
    }
    #[test]
    fn progress_bar_advance() {
        let mut pb = ProgressBar::new(10, "t");
        pb.advance(3);
        assert_eq!(pb.current(), 3);
    }
    #[test]
    fn progress_bar_set_current() {
        let mut pb = ProgressBar::new(10, "t");
        pb.set_current(7);
        assert_eq!(pb.current(), 7);
    }
    #[test]
    fn progress_bar_set_label() {
        let mut pb = ProgressBar::new(10, "old");
        pb.set_label("new");
        assert!(pb.render().contains("new"));
    }
    #[test]
    fn progress_bar_zero_total() {
        let pb = ProgressBar::new(0, "empty");
        assert!(pb.is_complete());
        assert!((pb.percentage() - 100.0).abs() < 0.01);
    }
    #[test]
    fn progress_reporter_start_and_advance() {
        let mut pr = ProgressReporter::new(vec!["parse".to_string(), "elab".to_string()]);
        pr.start_phase("parse", 4);
        assert!(!pr.advance_phase("parse"));
        assert!(!pr.advance_phase("parse"));
        assert!(!pr.advance_phase("parse"));
        assert!(pr.advance_phase("parse"));
    }
    #[test]
    fn progress_reporter_overall_percentage() {
        let mut pr = ProgressReporter::new(vec!["a".to_string(), "b".to_string()]);
        pr.start_phase("a", 2);
        pr.start_phase("b", 2);
        pr.advance_phase("a");
        pr.advance_phase("b");
        let pct = pr.overall_percentage();
        assert!((pct - 50.0).abs() < 1e-9);
    }
    #[test]
    fn progress_reporter_summary() {
        let mut pr = ProgressReporter::new(vec!["lex".to_string()]);
        pr.start_phase("lex", 10);
        for _ in 0..5 {
            pr.advance_phase("lex");
        }
        let s = pr.summary();
        assert!(s.contains("lex"));
        assert!(s.contains("5/10"));
    }
    #[test]
    fn progress_reporter_transitions() {
        let mut pr = ProgressReporter::new(vec!["a".to_string(), "b".to_string()]);
        pr.start_phase("a", 1);
        pr.start_phase("b", 1);
        assert_eq!(pr.transitions().len(), 2);
    }
    #[test]
    fn progress_reporter_advance_by() {
        let mut pr = ProgressReporter::new(vec!["p".to_string()]);
        pr.start_phase("p", 5);
        let done = pr.advance_phase_by("p", 5);
        assert!(done);
    }
    #[test]
    fn progress_reporter_current_phase_name() {
        let mut pr = ProgressReporter::new(vec!["a".to_string(), "b".to_string()]);
        pr.start_phase("b", 10);
        assert_eq!(pr.current_phase_name(), Some("b"));
    }
    #[test]
    fn progress_reporter_json() {
        let mut pr = ProgressReporter::new(vec!["x".to_string()]);
        pr.start_phase("x", 10);
        let j = pr.to_json();
        assert!(j.contains("\"name\":\"x\""));
        assert!(j.contains("\"total\":10"));
    }
    #[test]
    fn render_bar_empty() {
        let s = render_bar(0, 10, 10, '=', '-');
        assert_eq!(s, "----------");
    }
    #[test]
    fn render_bar_full() {
        let s = render_bar(10, 10, 10, '=', '-');
        assert_eq!(s, "==========");
    }
    #[test]
    fn render_bar_half() {
        let s = render_bar(5, 10, 10, '=', '-');
        assert_eq!(s, "=====-----");
    }
    #[test]
    fn render_bar_zero_total() {
        let s = render_bar(0, 0, 10, '=', '-');
        assert_eq!(s, "==========");
    }
    #[test]
    fn format_bytes_b() {
        assert_eq!(format_bytes(512), "512 B");
    }
    #[test]
    fn format_bytes_kb() {
        let s = format_bytes(2048);
        assert!(s.contains("KB"), "got: {}", s);
    }
    #[test]
    fn format_bytes_mb() {
        let s = format_bytes(3 * 1024 * 1024);
        assert!(s.contains("MB"), "got: {}", s);
    }
    #[test]
    fn format_bytes_gb() {
        let s = format_bytes(2 * 1024 * 1024 * 1024);
        assert!(s.contains("GB"), "got: {}", s);
    }
    #[test]
    fn format_count_small() {
        assert_eq!(format_count(42), "42");
    }
    #[test]
    fn format_count_thousands() {
        let s = format_count(1000);
        assert_eq!(s, "1,000");
    }
    #[test]
    fn format_count_millions() {
        let s = format_count(1_234_567);
        assert_eq!(s, "1,234,567");
    }
    #[test]
    fn format_duration_ms() {
        let d = Duration::from_millis(500);
        assert!(format_duration(d).contains("ms"));
    }
    #[test]
    fn format_duration_secs() {
        let d = Duration::from_secs(30);
        assert!(format_duration(d).contains("30s"));
    }
    #[test]
    fn format_duration_mins() {
        let d = Duration::from_secs(90);
        let s = format_duration(d);
        assert!(s.contains('m'));
    }
    #[test]
    fn format_duration_hours() {
        let d = Duration::from_secs(7200);
        let s = format_duration(d);
        assert!(s.contains('h'));
    }
    #[test]
    fn format_rate_per_sec() {
        let s = format_rate(42.5);
        assert!(s.contains("/s"), "got: {}", s);
    }
    #[test]
    fn format_rate_kilo() {
        let s = format_rate(1500.0);
        assert!(s.contains('K'));
    }
    #[test]
    fn format_rate_mega() {
        let s = format_rate(2_000_000.0);
        assert!(s.contains('M'));
    }
    #[test]
    fn spinner_tick_changes_frame() {
        let mut s = Spinner::new("loading", SpinnerStyle::Ascii);
        let r1 = s.render();
        s.tick();
        let r2 = s.render();
        assert_ne!(r1, r2);
    }
    #[test]
    fn spinner_render_contains_label() {
        let s = Spinner::new("compiling", SpinnerStyle::Dots);
        let r = s.render();
        assert!(r.contains("compiling"), "got: {}", r);
    }
    #[test]
    fn spinner_all_styles() {
        for style in [
            SpinnerStyle::Braille,
            SpinnerStyle::Ascii,
            SpinnerStyle::Dots,
            SpinnerStyle::Arrows,
            SpinnerStyle::Clock,
        ] {
            let mut s = Spinner::new("test", style);
            s.tick();
            let r = s.render();
            assert!(!r.is_empty());
        }
    }
    #[test]
    fn spinner_set_label() {
        let mut s = Spinner::new("old", SpinnerStyle::Ascii);
        s.set_label("new");
        assert!(s.render().contains("new"));
    }
    #[test]
    fn spinner_frame_wraps() {
        let mut s = Spinner::new("t", SpinnerStyle::Ascii);
        for _ in 0..100 {
            s.tick();
        }
        let _ = s.render();
    }
    #[test]
    fn eta_estimator_no_rate_no_eta() {
        let e = EtaEstimator::new(100);
        assert!(e.eta(0).is_none());
    }
    #[test]
    fn eta_estimator_rate_after_record() {
        let mut e = EtaEstimator::new(100);
        e.record(10);
        e.record(20);
        let _ = e.eta(20);
    }
    #[test]
    fn eta_estimator_reset() {
        let mut e = EtaEstimator::new(100);
        e.record(50);
        e.reset();
        assert_eq!(e.rate(), 0.0);
        assert!(e.eta(50).is_none());
    }
    #[test]
    fn throughput_empty() {
        let t = ThroughputDisplay::new(5.0);
        assert_eq!(t.throughput(), 0.0);
    }
    #[test]
    fn throughput_reset() {
        let mut t = ThroughputDisplay::new(5.0);
        t.record(100.0);
        t.reset();
        assert_eq!(t.throughput(), 0.0);
    }
    #[test]
    fn progress_node_leaf() {
        let mut node = ProgressNode::leaf("parse", 10);
        assert_eq!(node.total, 10);
        assert_eq!(node.current, 0);
        node.increment();
        assert_eq!(node.current, 1);
    }
    #[test]
    fn progress_node_complete() {
        let mut node = ProgressNode::leaf("elab", 5);
        node.complete();
        assert!(matches!(node.status, NodeStatus::Complete));
        assert_eq!(node.current, 5);
    }
    #[test]
    fn progress_node_group_with_children() {
        let mut root = ProgressNode::group("root");
        root.add_child(ProgressNode::leaf("child1", 5));
        root.add_child(ProgressNode::leaf("child2", 3));
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.subtree_size(), 3);
    }
    #[test]
    fn progress_node_render_tree() {
        let mut root = ProgressNode::group("root");
        root.start();
        let mut c = ProgressNode::leaf("child", 10);
        c.current = 5;
        root.add_child(c);
        let s = root.render_tree(0);
        assert!(s.contains("root"));
        assert!(s.contains("child"));
    }
    #[test]
    fn progress_node_fail() {
        let mut node = ProgressNode::leaf("test", 10);
        node.fail();
        assert!(matches!(node.status, NodeStatus::Failed));
    }
    #[test]
    fn multi_task_basic() {
        let mut mp = MultiTaskProgress::new();
        mp.add_task("compile", 10);
        mp.add_task("link", 5);
        assert_eq!(mp.task_count(), 2);
        assert!(!mp.all_complete());
    }
    #[test]
    fn multi_task_advance_by_name() {
        let mut mp = MultiTaskProgress::new();
        mp.add_task("a", 3);
        let done = mp.advance_task("a");
        assert!(!done);
        mp.advance_task("a");
        let done = mp.advance_task("a");
        assert!(done);
    }
    #[test]
    fn multi_task_overall_pct() {
        let mut mp = MultiTaskProgress::new();
        mp.add_task("x", 4);
        mp.add_task("y", 4);
        mp.advance_task("x");
        mp.advance_task("y");
        let pct = mp.overall_percentage();
        assert!((pct - 25.0).abs() < 0.1);
    }
    #[test]
    fn multi_task_json() {
        let mut mp = MultiTaskProgress::new();
        mp.add_task("build", 10);
        let j = mp.to_json();
        assert!(j.contains("\"build\""));
    }
    #[test]
    fn phase_timeline_begin_end() {
        let mut t = PhaseTimeline::new();
        t.begin_phase("lex");
        t.end_phase();
        t.begin_phase("parse");
        t.end_phase();
        assert_eq!(t.phase_count(), 2);
    }
    #[test]
    fn phase_timeline_render() {
        let mut t = PhaseTimeline::new();
        t.begin_phase("lex");
        t.end_phase();
        let s = t.render();
        assert!(s.contains("lex"));
    }
    #[test]
    fn step_progress_basic() {
        let mut sp = StepProgress::new(vec![
            "step1".to_string(),
            "step2".to_string(),
            "step3".to_string(),
        ]);
        assert!(!sp.is_complete());
        sp.begin_current();
        sp.complete_current();
        sp.begin_current();
        sp.complete_current();
        sp.begin_current();
        sp.complete_current();
        assert!(sp.is_complete());
        assert_eq!(sp.done_count(), 3);
    }
    #[test]
    fn step_progress_fail() {
        let mut sp = StepProgress::new(vec!["a".to_string(), "b".to_string()]);
        sp.fail_current();
        assert_eq!(sp.done_count(), 0);
    }
    #[test]
    fn step_progress_skip() {
        let mut sp = StepProgress::new(vec!["a".to_string()]);
        sp.skip_current();
        assert!(sp.is_complete());
    }
    #[test]
    fn step_progress_render() {
        let mut sp = StepProgress::new(vec!["setup".to_string(), "build".to_string()]);
        sp.begin_current();
        let s = sp.render();
        assert!(s.contains("setup"));
        assert!(s.contains("build"));
    }
    #[test]
    fn batch_progress_counts() {
        let mut bp = BatchProgress::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        bp.set_status(0, BatchItemStatus::Success);
        bp.set_status(1, BatchItemStatus::Failure);
        assert_eq!(bp.success_count(), 1);
        assert_eq!(bp.failure_count(), 1);
        assert_eq!(bp.queued_count(), 1);
    }
    #[test]
    fn batch_progress_is_done() {
        let mut bp = BatchProgress::new(vec!["a".to_string(), "b".to_string()]);
        bp.set_status(0, BatchItemStatus::Success);
        bp.set_status(1, BatchItemStatus::Failure);
        assert!(bp.is_done());
    }
    #[test]
    fn batch_progress_set_by_name() {
        let mut bp = BatchProgress::new(vec!["myfile.lean".to_string()]);
        bp.set_status_by_name("myfile.lean", BatchItemStatus::Success);
        assert_eq!(bp.success_count(), 1);
    }
    #[test]
    fn batch_progress_render() {
        let mut bp = BatchProgress::new(vec!["x".to_string()]);
        bp.set_status(0, BatchItemStatus::Warning);
        let s = bp.render_all();
        assert!(s.contains('x'));
    }
    #[test]
    fn phase_transition_format() {
        let t = PhaseTransition::new(Some("lex"), "parse");
        let s = t.format_line();
        assert!(s.contains("lex"));
        assert!(s.contains("parse"));
        assert!(s.contains("→"));
    }
    #[test]
    fn phase_transition_with_note() {
        let t = PhaseTransition::new(None, "check").with_note("100 files");
        let s = t.format_line();
        assert!(s.contains("100 files"));
    }
    #[test]
    fn color_mode_never() {
        let s = color_green("hello", ColorMode::Never);
        assert_eq!(s, "hello");
    }
    #[test]
    fn color_mode_always() {
        let s = color_red("err", ColorMode::Always);
        assert!(s.contains("err"));
        assert!(s.contains("\x1b["));
    }
    #[test]
    fn progress_log_basic() {
        let mut log = ProgressLog::new();
        log.info("started");
        log.warn("slow");
        log.error("oops");
        assert_eq!(log.len(), 3);
        assert_eq!(log.error_count(), 1);
        assert_eq!(log.warn_count(), 1);
    }
    #[test]
    fn progress_log_level_filter() {
        let mut log = ProgressLog::new();
        log.set_min_level(LogLevel::Warn);
        log.info("ignored");
        log.warn("kept");
        assert_eq!(log.len(), 1);
    }
    #[test]
    fn progress_log_phase_entries() {
        let mut log = ProgressLog::new();
        log.set_min_level(LogLevel::Trace);
        log.log_in_phase(LogLevel::Info, "parse", "parsed 10 files");
        log.log_in_phase(LogLevel::Info, "elab", "elaborated 5 defs");
        let parse_entries = log.entries_for_phase("parse");
        assert_eq!(parse_entries.len(), 1);
    }
    #[test]
    fn progress_log_clear() {
        let mut log = ProgressLog::new();
        log.info("a");
        log.info("b");
        log.clear();
        assert!(log.is_empty());
    }
    #[test]
    fn progress_log_format() {
        let mut log = ProgressLog::new();
        log.info("hello world");
        let formatted = log.format();
        assert!(formatted.contains("hello world"));
    }
    #[test]
    fn progress_to_json_basic() {
        let mut pb = ProgressBar::new(10, "test");
        pb.set_current(5);
        let j = progress_to_json(&pb);
        assert!(j.contains("\"current\":5"));
        assert!(j.contains("\"total\":10"));
        assert!(j.contains("\"complete\":false"));
    }
    #[test]
    fn reporter_to_json_basic() {
        let mut pr = ProgressReporter::new(vec!["p".to_string()]);
        pr.start_phase("p", 10);
        let j = reporter_to_json(&pr);
        assert!(j.contains("\"name\":\"p\""));
    }
    #[test]
    fn terminal_width_fallback() {
        std::env::remove_var("COLUMNS");
        let w = terminal_width();
        assert!(w >= 40, "expected reasonable width, got {}", w);
    }
    #[test]
    fn terminal_width_from_env() {
        std::env::set_var("COLUMNS", "120");
        let w = terminal_width();
        assert_eq!(w, 120);
        std::env::remove_var("COLUMNS");
    }
}
#[allow(dead_code)]
pub fn format_eta(ms: Option<u64>) -> String {
    match ms {
        None => "(estimating...)".to_string(),
        Some(0) => "(almost done)".to_string(),
        Some(ms) if ms < 1000 => "< 1s".to_string(),
        Some(ms) if ms < 60_000 => format!("~{}s", ms / 1000),
        Some(ms) => format!("~{}m {}s", ms / 60_000, (ms % 60_000) / 1000),
    }
}
#[allow(dead_code)]
pub fn render_progress_bar(fraction: f64, width: usize, fill: char, empty: char) -> String {
    let filled = ((fraction.clamp(0.0, 1.0) * width as f64) as usize).min(width);
    let empty_count = width - filled;
    format!(
        "[{}{}]",
        fill.to_string().repeat(filled),
        empty.to_string().repeat(empty_count)
    )
}
#[cfg(test)]
mod progress_extra_tests {
    use super::*;
    #[test]
    fn test_batch_pending() {
        let mut b = ProgressBatch::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        b.mark_done(0);
        assert_eq!(b.pending(), 2);
        assert_eq!(b.completed, 1);
    }
    #[test]
    fn test_render_progress_bar() {
        let bar = render_progress_bar(0.5, 10, '#', '-');
        assert_eq!(bar, "[#####-----]");
    }
    #[test]
    fn test_format_eta() {
        assert_eq!(format_eta(None), "(estimating...)");
        assert_eq!(format_eta(Some(0)), "(almost done)");
        assert!(format_eta(Some(5000)).contains("5s"));
    }
    #[test]
    fn test_multi_progress_overall() {
        let mut mp = MultiProgressTracker::new();
        mp.add_task("A");
        mp.add_task("B");
        mp.set_progress(0, 1.0);
        mp.set_progress(1, 0.0);
        assert!((mp.overall_progress() - 0.5).abs() < 1e-9);
    }
}
#[cfg(test)]
mod progress_log_tests {
    use super::*;
    #[test]
    fn test_progress_log_rate() {
        let mut log = ProgressCheckpointLog::new();
        log.record("start", 0, 0.0);
        log.record("mid", 500, 0.5);
        log.record("end", 1000, 1.0);
        let rate = log
            .average_rate_per_ms()
            .expect("test operation should succeed");
        assert!((rate - 0.001).abs() < 1e-9);
    }
    #[test]
    fn test_elapsed_between() {
        let mut log = ProgressCheckpointLog::new();
        log.record("a", 100, 0.0);
        log.record("b", 300, 0.5);
        assert_eq!(log.elapsed_between(0, 1), Some(200));
    }
}
