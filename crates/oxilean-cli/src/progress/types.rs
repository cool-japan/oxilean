//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::{Duration, Instant};

/// Tracks multiple concurrent tasks (e.g. parallel build).
pub struct MultiTaskProgress {
    tasks: Vec<(String, ProgressBar)>,
    color: ColorMode,
    pub started_at: Instant,
}
impl MultiTaskProgress {
    /// Create an empty multi-task tracker.
    pub fn new() -> Self {
        MultiTaskProgress {
            tasks: Vec::new(),
            color: ColorMode::Never,
            started_at: Instant::now(),
        }
    }
    /// Set color mode.
    pub fn with_color(mut self, color: ColorMode) -> Self {
        self.color = color;
        self
    }
    /// Add a new task.
    pub fn add_task(&mut self, name: &str, total: usize) {
        let mut pb = ProgressBar::new(total, name);
        pb.start();
        self.tasks.push((name.to_string(), pb));
    }
    /// Advance a task by name.
    pub fn advance_task(&mut self, name: &str) -> bool {
        for (n, pb) in &mut self.tasks {
            if n == name {
                pb.increment();
                return pb.is_complete();
            }
        }
        false
    }
    /// Advance a task by index.
    pub fn advance_task_idx(&mut self, idx: usize) -> bool {
        if let Some((_, pb)) = self.tasks.get_mut(idx) {
            pb.increment();
            return pb.is_complete();
        }
        false
    }
    /// Returns true if all tasks are complete.
    pub fn all_complete(&self) -> bool {
        self.tasks.iter().all(|(_, pb)| pb.is_complete())
    }
    /// Number of complete tasks.
    pub fn complete_count(&self) -> usize {
        self.tasks.iter().filter(|(_, pb)| pb.is_complete()).count()
    }
    /// Total number of tasks.
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
    /// Overall percentage.
    pub fn overall_percentage(&self) -> f64 {
        if self.tasks.is_empty() {
            return 100.0;
        }
        let total_work: usize = self.tasks.iter().map(|(_, pb)| pb.total).sum();
        let done_work: usize = self.tasks.iter().map(|(_, pb)| pb.current).sum();
        if total_work == 0 {
            100.0
        } else {
            (done_work as f64 / total_work as f64) * 100.0
        }
    }
    /// Render all task progress bars.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for (name, pb) in &self.tasks {
            let bar = render_bar(pb.current, pb.total, 20, '=', '-');
            let status_icon = if pb.is_complete() { "✓" } else { "●" };
            let _ = writeln!(
                out,
                "{} {} [{}] {}/{}",
                status_icon, name, bar, pb.current, pb.total
            );
        }
        let pct = self.overall_percentage();
        let _ = write!(
            out,
            "Overall: {:.1}% ({}/{} tasks)",
            pct,
            self.complete_count(),
            self.task_count()
        );
        out
    }
    /// Render a compact single-line summary.
    pub fn render_compact(&self) -> String {
        format!(
            "[{}/{}] {:.1}% elapsed:{}",
            self.complete_count(),
            self.task_count(),
            self.overall_percentage(),
            format_duration(self.started_at.elapsed())
        )
    }
    /// Serialize to JSON.
    pub fn to_json(&self) -> String {
        let task_parts: Vec<String> = self
            .tasks
            .iter()
            .map(|(name, pb)| {
                format!(
                    r#"{{"name":"{}","current":{},"total":{},"complete":{}}}"#,
                    name.replace('"', "\\\""),
                    pb.current,
                    pb.total,
                    pb.is_complete()
                )
            })
            .collect();
        format!(
            r#"{{"tasks":[{}],"overall_percentage":{:.2},"complete_count":{},"task_count":{}}}"#,
            task_parts.join(","),
            self.overall_percentage(),
            self.complete_count(),
            self.task_count()
        )
    }
}
struct PhaseTimelineEntry {
    name: String,
    start: Instant,
    end: Option<Instant>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProgressBatch {
    pub items: Vec<String>,
    pub completed: usize,
    pub failed: usize,
}
#[allow(dead_code)]
impl ProgressBatch {
    pub fn new(items: Vec<String>) -> Self {
        let _n = items.len();
        Self {
            items,
            completed: 0,
            failed: 0,
        }
    }
    pub fn mark_done(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.completed += 1;
        }
    }
    pub fn mark_failed(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.failed += 1;
        }
    }
    pub fn total(&self) -> usize {
        self.items.len()
    }
    pub fn pending(&self) -> usize {
        self.items
            .len()
            .saturating_sub(self.completed + self.failed)
    }
    pub fn success_rate(&self) -> f64 {
        let done = self.completed + self.failed;
        if done == 0 {
            0.0
        } else {
            self.completed as f64 / done as f64
        }
    }
}
/// A sliding-window ETA estimator using exponential moving average.
pub struct EtaEstimator {
    pub total: usize,
    history: Vec<(Instant, usize)>,
    window: usize,
    ema_rate: f64,
    alpha: f64,
}
impl EtaEstimator {
    /// Create a new ETA estimator with the given total.
    pub fn new(total: usize) -> Self {
        EtaEstimator {
            total,
            history: Vec::new(),
            window: 10,
            ema_rate: 0.0,
            alpha: 0.3,
        }
    }
    /// Create with a custom EMA smoothing factor (0 < alpha <= 1).
    pub fn with_alpha(total: usize, alpha: f64) -> Self {
        let mut e = Self::new(total);
        e.alpha = alpha.clamp(0.01, 1.0);
        e
    }
    /// Record a new observation: `current` items done at this moment.
    pub fn record(&mut self, current: usize) {
        let now = Instant::now();
        if self.history.len() >= self.window {
            self.history.remove(0);
        }
        self.history.push((now, current));
        if self.history.len() >= 2 {
            let (t0, c0) = self.history[0];
            let elapsed = now.duration_since(t0).as_secs_f64();
            if elapsed > 0.0 {
                let items = (current.saturating_sub(c0)) as f64;
                let inst_rate = items / elapsed;
                if self.ema_rate == 0.0 {
                    self.ema_rate = inst_rate;
                } else {
                    self.ema_rate = self.alpha * inst_rate + (1.0 - self.alpha) * self.ema_rate;
                }
            }
        }
    }
    /// Estimate remaining time given current progress.
    pub fn eta(&self, current: usize) -> Option<Duration> {
        if self.ema_rate <= 0.0 {
            return None;
        }
        let remaining = self.total.saturating_sub(current) as f64;
        let secs = remaining / self.ema_rate;
        Some(Duration::from_secs_f64(secs.min(999_999.0)))
    }
    /// Return the current estimated rate (items per second).
    pub fn rate(&self) -> f64 {
        self.ema_rate
    }
    /// Reset all history.
    pub fn reset(&mut self) {
        self.history.clear();
        self.ema_rate = 0.0;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProgressCheckpoint {
    pub label: String,
    pub timestamp_ms: u64,
    pub fraction: f64,
}
#[allow(dead_code)]
impl ProgressCheckpoint {
    pub fn new(label: &str, timestamp_ms: u64, fraction: f64) -> Self {
        Self {
            label: label.to_string(),
            timestamp_ms,
            fraction,
        }
    }
}
/// Tracks throughput (items/sec or bytes/sec) with a sliding time window.
pub struct ThroughputDisplay {
    window_secs: f64,
    observations: Vec<(Instant, f64)>,
}
impl ThroughputDisplay {
    /// Create with the given averaging window in seconds.
    pub fn new(window_secs: f64) -> Self {
        ThroughputDisplay {
            window_secs: window_secs.max(0.1),
            observations: Vec::new(),
        }
    }
    /// Record `amount` units processed at this moment.
    pub fn record(&mut self, amount: f64) {
        let now = Instant::now();
        self.observations.push((now, amount));
        let cutoff = now - Duration::from_secs_f64(self.window_secs);
        self.observations.retain(|(t, _)| *t >= cutoff);
    }
    /// Return current throughput in units/second.
    pub fn throughput(&self) -> f64 {
        if self.observations.is_empty() {
            return 0.0;
        }
        let total: f64 = self.observations.iter().map(|(_, a)| *a).sum();
        let window = self.window_secs;
        total / window
    }
    /// Format as a human-readable string.
    pub fn display(&self) -> String {
        format_rate(self.throughput())
    }
    /// Reset all observations.
    pub fn reset(&mut self) {
        self.observations.clear();
    }
}
/// A progress log: stores ordered log entries with filtering support.
pub struct ProgressLog {
    entries: Vec<ProgressLogEntry>,
    min_level: LogLevel,
    max_entries: usize,
    color: ColorMode,
}
impl ProgressLog {
    /// Create a new progress log.
    pub fn new() -> Self {
        ProgressLog {
            entries: Vec::new(),
            min_level: LogLevel::Info,
            max_entries: 1000,
            color: ColorMode::Never,
        }
    }
    /// Set the minimum log level filter.
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }
    /// Set maximum number of retained entries.
    pub fn set_max_entries(&mut self, n: usize) {
        self.max_entries = n;
    }
    /// Set color mode.
    pub fn set_color(&mut self, color: ColorMode) {
        self.color = color;
    }
    /// Append a log entry.
    pub fn log(&mut self, level: LogLevel, message: &str) {
        if level < self.min_level {
            return;
        }
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(ProgressLogEntry {
            level,
            message: message.to_string(),
            timestamp: std::time::SystemTime::now(),
            phase: None,
        });
    }
    /// Append a log entry with a phase name.
    pub fn log_in_phase(&mut self, level: LogLevel, phase: &str, message: &str) {
        if level < self.min_level {
            return;
        }
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(ProgressLogEntry {
            level,
            message: message.to_string(),
            timestamp: std::time::SystemTime::now(),
            phase: Some(phase.to_string()),
        });
    }
    /// Convenience methods.
    pub fn trace(&mut self, msg: &str) {
        self.log(LogLevel::Trace, msg);
    }
    pub fn debug(&mut self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }
    pub fn info(&mut self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }
    pub fn warn(&mut self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }
    pub fn error(&mut self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
    /// Return all entries at or above the given level.
    pub fn entries_at_level(&self, level: LogLevel) -> Vec<&ProgressLogEntry> {
        self.entries.iter().filter(|e| e.level >= level).collect()
    }
    /// Format all log entries as a string.
    pub fn format(&self) -> String {
        let mut out = String::new();
        for entry in &self.entries {
            let level_str = format!("[{}]", entry.level.name());
            let phase_str = entry
                .phase
                .as_deref()
                .map(|p| format!("[{}] ", p))
                .unwrap_or_default();
            let colored_level = match entry.level {
                LogLevel::Error => color_red(&level_str, self.color),
                LogLevel::Warn => color_yellow(&level_str, self.color),
                LogLevel::Info => color_green(&level_str, self.color),
                LogLevel::Debug => color_cyan(&level_str, self.color),
                LogLevel::Trace => color_dim(&level_str, self.color),
            };
            let _ = writeln!(out, "{} {}{}", colored_level, phase_str, entry.message);
        }
        out
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// True if no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return entries filtered by phase name.
    pub fn entries_for_phase(&self, phase: &str) -> Vec<&ProgressLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.phase.as_deref() == Some(phase))
            .collect()
    }
    /// Count errors.
    pub fn error_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Error)
            .count()
    }
    /// Count warnings.
    pub fn warn_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Warn)
            .count()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchItemStatus {
    Queued,
    Processing,
    Success,
    Failure,
    Warning,
}
impl BatchItemStatus {
    pub fn icon(self) -> &'static str {
        match self {
            BatchItemStatus::Queued => "·",
            BatchItemStatus::Processing => "◐",
            BatchItemStatus::Success => "✓",
            BatchItemStatus::Failure => "✗",
            BatchItemStatus::Warning => "⚠",
        }
    }
}
/// A node in a hierarchical progress tree (for nested tasks).
pub struct ProgressNode {
    pub name: String,
    pub current: usize,
    pub total: usize,
    pub children: Vec<ProgressNode>,
    pub status: NodeStatus,
}
impl ProgressNode {
    /// Create a leaf node.
    pub fn leaf(name: &str, total: usize) -> Self {
        ProgressNode {
            name: name.to_string(),
            current: 0,
            total,
            children: Vec::new(),
            status: NodeStatus::Pending,
        }
    }
    /// Create a group node (parent).
    pub fn group(name: &str) -> Self {
        ProgressNode {
            name: name.to_string(),
            current: 0,
            total: 0,
            children: Vec::new(),
            status: NodeStatus::Pending,
        }
    }
    /// Add a child node.
    pub fn add_child(&mut self, child: ProgressNode) {
        self.children.push(child);
    }
    /// Increment this node's progress.
    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
        if self.current >= self.total && self.total > 0 {
            self.status = NodeStatus::Complete;
        }
    }
    /// Mark as running.
    pub fn start(&mut self) {
        self.status = NodeStatus::Running;
    }
    /// Mark as complete.
    pub fn complete(&mut self) {
        self.current = self.total;
        self.status = NodeStatus::Complete;
    }
    /// Mark as failed.
    pub fn fail(&mut self) {
        self.status = NodeStatus::Failed;
    }
    /// Mark as skipped.
    pub fn skip(&mut self) {
        self.status = NodeStatus::Skipped;
    }
    /// Return percentage complete (0..=100).
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            match self.status {
                NodeStatus::Complete | NodeStatus::Skipped => 100.0,
                _ => 0.0,
            }
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }
    /// Return total nodes in the subtree (including self).
    pub fn subtree_size(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|c| c.subtree_size())
            .sum::<usize>()
    }
    /// Render as an ASCII tree.
    pub fn render_tree(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let icon = self.status.icon();
        let pct = self.percentage();
        let bar = render_bar(self.current, self.total.max(1), 10, '=', '-');
        let mut out = format!("{}{} {} [{}] {:.0}%\n", pad, icon, self.name, bar, pct);
        for child in &self.children {
            out.push_str(&child.render_tree(indent + 1));
        }
        out
    }
}
/// Log level for progress log entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}
impl LogLevel {
    pub fn name(self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}
/// A single progress log entry.
#[derive(Debug, Clone)]
pub struct ProgressLogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: std::time::SystemTime,
    pub phase: Option<String>,
}
/// Status of a progress node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Pending,
    Running,
    Complete,
    Failed,
    Skipped,
}
impl NodeStatus {
    pub fn icon(self) -> &'static str {
        match self {
            NodeStatus::Pending => "○",
            NodeStatus::Running => "●",
            NodeStatus::Complete => "✓",
            NodeStatus::Failed => "✗",
            NodeStatus::Skipped => "⊘",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProgressCheckpointLog {
    pub checkpoints: Vec<ProgressCheckpoint>,
}
#[allow(dead_code)]
impl ProgressCheckpointLog {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
        }
    }
    pub fn record(&mut self, label: &str, timestamp_ms: u64, fraction: f64) {
        self.checkpoints
            .push(ProgressCheckpoint::new(label, timestamp_ms, fraction));
    }
    pub fn elapsed_between(&self, idx_a: usize, idx_b: usize) -> Option<u64> {
        let a = self.checkpoints.get(idx_a)?;
        let b = self.checkpoints.get(idx_b)?;
        Some(b.timestamp_ms.saturating_sub(a.timestamp_ms))
    }
    pub fn average_rate_per_ms(&self) -> Option<f64> {
        if self.checkpoints.len() < 2 {
            return None;
        }
        let first = &self.checkpoints[0];
        let last = self
            .checkpoints
            .last()
            .expect("checkpoints has at least 2 elements: checked by early return");
        let elapsed = last.timestamp_ms.saturating_sub(first.timestamp_ms);
        if elapsed == 0 {
            return None;
        }
        Some((last.fraction - first.fraction) / elapsed as f64)
    }
}
/// Color support policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    Always,
    Never,
    Auto,
}
impl ColorMode {
    /// Return `true` if colors should be emitted.
    pub fn enabled(self) -> bool {
        match self {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => std::env::var("NO_COLOR").is_err(),
        }
    }
}
/// A step-based progress tracker with named steps (e.g. for wizard/pipeline).
pub struct StepProgress {
    steps: Vec<String>,
    current_step: usize,
    statuses: Vec<StepStatus>,
}
impl StepProgress {
    /// Create a step progress tracker from a list of step names.
    pub fn new(steps: Vec<String>) -> Self {
        let n = steps.len();
        StepProgress {
            steps,
            current_step: 0,
            statuses: vec![StepStatus::Pending; n],
        }
    }
    /// Mark the current step as in-progress.
    pub fn begin_current(&mut self) {
        if self.current_step < self.statuses.len() {
            self.statuses[self.current_step] = StepStatus::InProgress;
        }
    }
    /// Mark the current step as done and advance.
    pub fn complete_current(&mut self) {
        if self.current_step < self.statuses.len() {
            self.statuses[self.current_step] = StepStatus::Done;
            self.current_step += 1;
        }
    }
    /// Mark the current step as failed.
    pub fn fail_current(&mut self) {
        if self.current_step < self.statuses.len() {
            self.statuses[self.current_step] = StepStatus::Error;
            self.current_step += 1;
        }
    }
    /// Skip the current step.
    pub fn skip_current(&mut self) {
        if self.current_step < self.statuses.len() {
            self.statuses[self.current_step] = StepStatus::Skipped;
            self.current_step += 1;
        }
    }
    /// Render all steps as a list.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for (i, step) in self.steps.iter().enumerate() {
            let status = self.statuses.get(i).copied().unwrap_or(StepStatus::Pending);
            let _ = writeln!(out, "{} {}", status.icon(), step);
        }
        out
    }
    /// Return how many steps are done.
    pub fn done_count(&self) -> usize {
        self.statuses
            .iter()
            .filter(|&&s| s == StepStatus::Done)
            .count()
    }
    /// Total steps.
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }
    /// Is the pipeline complete (all done or all past current)?
    pub fn is_complete(&self) -> bool {
        self.current_step >= self.steps.len()
    }
}
/// An animated spinner for indeterminate-length tasks.
pub struct Spinner {
    pub label: String,
    style: SpinnerStyle,
    frame: usize,
    pub started_at: Instant,
    color: ColorMode,
}
impl Spinner {
    /// Create a new spinner with the given label and style.
    pub fn new(label: &str, style: SpinnerStyle) -> Self {
        Spinner {
            label: label.to_string(),
            style,
            frame: 0,
            started_at: Instant::now(),
            color: ColorMode::Never,
        }
    }
    /// Create a spinner with color support.
    pub fn with_color(label: &str, style: SpinnerStyle, color: ColorMode) -> Self {
        let mut s = Self::new(label, style);
        s.color = color;
        s
    }
    /// Advance to the next animation frame.
    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % self.style.frame_count();
    }
    /// Set the tick frame directly.
    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame % self.style.frame_count();
    }
    /// Update the label.
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
    /// Render current spinner frame.
    pub fn render(&self) -> String {
        let frames = self.style.frames();
        let frame_str = frames[self.frame % frames.len()];
        let elapsed = self.started_at.elapsed();
        format!(
            "{} {} [{}]",
            frame_str,
            self.label,
            format_duration(elapsed)
        )
    }
    /// Render with color.
    pub fn render_colored(&self) -> String {
        let raw = self.render();
        color_cyan(&raw, self.color)
    }
    /// Return elapsed time since spinner was created.
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }
}
/// Records a transition between named phases.
#[derive(Debug, Clone)]
pub struct PhaseTransition {
    pub from_phase: Option<String>,
    pub to_phase: String,
    pub at: std::time::SystemTime,
    pub note: Option<String>,
}
impl PhaseTransition {
    /// Create a new transition record.
    pub fn new(from: Option<&str>, to: &str) -> Self {
        PhaseTransition {
            from_phase: from.map(|s| s.to_string()),
            to_phase: to.to_string(),
            at: std::time::SystemTime::now(),
            note: None,
        }
    }
    /// Attach a note to this transition.
    pub fn with_note(mut self, note: &str) -> Self {
        self.note = Some(note.to_string());
        self
    }
    /// Format as a single line.
    pub fn format_line(&self) -> String {
        let from_str = self.from_phase.as_deref().unwrap_or("(start)");
        if let Some(note) = &self.note {
            format!("{} → {} [{}]", from_str, self.to_phase, note)
        } else {
            format!("{} → {}", from_str, self.to_phase)
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaEstimatorMs {
    start_ms: u64,
    pub total: usize,
    completed: usize,
}
#[allow(dead_code)]
impl EtaEstimatorMs {
    pub fn new(total: usize, start_ms: u64) -> Self {
        Self {
            start_ms,
            total,
            completed: 0,
        }
    }
    pub fn update(&mut self, completed: usize) {
        self.completed = completed;
    }
    pub fn eta_ms(&self, now_ms: u64) -> Option<u64> {
        if self.completed == 0 {
            return None;
        }
        let elapsed = now_ms.saturating_sub(self.start_ms);
        let per_item = elapsed / self.completed as u64;
        let remaining = self.total.saturating_sub(self.completed) as u64;
        Some(per_item * remaining)
    }
    pub fn fraction_done(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.completed as f64 / self.total as f64
        }
    }
}
/// A timeline of phases with start times and durations.
pub struct PhaseTimeline {
    entries: Vec<PhaseTimelineEntry>,
    pub started_at: Instant,
}
impl PhaseTimeline {
    /// Create a new timeline.
    pub fn new() -> Self {
        PhaseTimeline {
            entries: Vec::new(),
            started_at: Instant::now(),
        }
    }
    /// Start a new phase (closes the previous one if open).
    pub fn begin_phase(&mut self, name: &str) {
        let now = Instant::now();
        if let Some(last) = self.entries.last_mut() {
            if last.end.is_none() {
                last.end = Some(now);
            }
        }
        self.entries.push(PhaseTimelineEntry {
            name: name.to_string(),
            start: now,
            end: None,
        });
    }
    /// Close the current phase.
    pub fn end_phase(&mut self) {
        if let Some(last) = self.entries.last_mut() {
            if last.end.is_none() {
                last.end = Some(Instant::now());
            }
        }
    }
    /// Render the timeline as a text table.
    pub fn render(&self) -> String {
        let mut out = String::from("Phase Timeline:\n");
        for entry in &self.entries {
            let start_ms = entry.start.duration_since(self.started_at).as_millis();
            let dur_str = if let Some(end) = entry.end {
                format_duration(end.duration_since(entry.start))
            } else {
                format_duration(entry.start.elapsed())
            };
            let _ = writeln!(out, "  {:20} +{}ms  ({})", entry.name, start_ms, dur_str);
        }
        out
    }
    /// Number of phases recorded.
    pub fn phase_count(&self) -> usize {
        self.entries.len()
    }
    /// Total elapsed time.
    pub fn total_elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiProgressTracker {
    tasks: Vec<(String, f64)>,
}
#[allow(dead_code)]
impl MultiProgressTracker {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, name: &str) {
        self.tasks.push((name.to_string(), 0.0));
    }
    pub fn set_progress(&mut self, idx: usize, fraction: f64) {
        if idx < self.tasks.len() {
            self.tasks[idx].1 = fraction.clamp(0.0, 1.0);
        }
    }
    pub fn overall_progress(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        self.tasks.iter().map(|(_, f)| f).sum::<f64>() / self.tasks.len() as f64
    }
    pub fn render(&self, bar_width: usize) -> String {
        self.tasks
            .iter()
            .map(|(name, f)| {
                format!(
                    "{}: {} {:.0}%",
                    name,
                    render_progress_bar(*f, bar_width, '#', '-'),
                    f * 100.0
                )
            })
            .collect::<Vec<_>>()
            .join(
                "
",
            )
    }
}
/// A simple progress bar for tracking completion of a single task.
pub struct ProgressBar {
    pub total: usize,
    pub current: usize,
    pub label: String,
    bar_width: usize,
    fill_char: char,
    empty_char: char,
    color: ColorMode,
    pub started_at: Option<Instant>,
    ema_rate: f64,
}
impl ProgressBar {
    pub fn new(total: usize, label: &str) -> Self {
        ProgressBar {
            total,
            current: 0,
            label: label.to_string(),
            bar_width: 20,
            fill_char: '=',
            empty_char: '-',
            color: ColorMode::Never,
            started_at: None,
            ema_rate: 0.0,
        }
    }
    pub fn with_width(total: usize, label: &str, bar_width: usize) -> Self {
        let mut pb = Self::new(total, label);
        pb.bar_width = bar_width;
        pb
    }
    pub fn with_color(total: usize, label: &str, color: ColorMode) -> Self {
        let mut pb = Self::new(total, label);
        pb.color = color;
        pb
    }
    pub fn with_chars(total: usize, label: &str, fill: char, empty: char) -> Self {
        let mut pb = Self::new(total, label);
        pb.fill_char = fill;
        pb.empty_char = empty;
        pb
    }
    /// Start the internal timer (for ETA computation).
    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }
    /// Advance the progress by 1.
    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
        if let Some(t0) = self.started_at {
            let elapsed = t0.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let inst_rate = self.current as f64 / elapsed;
                if self.ema_rate == 0.0 {
                    self.ema_rate = inst_rate;
                } else {
                    self.ema_rate = 0.3 * inst_rate + 0.7 * self.ema_rate;
                }
            }
        }
    }
    /// Advance by `n` steps.
    pub fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.increment();
        }
    }
    /// Set current directly.
    pub fn set_current(&mut self, n: usize) {
        self.current = n.min(self.total);
    }
    /// Update the label string.
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
    /// Render the progress bar as a string of the form `[===---] 45/100 label`.
    ///
    /// The bar width is 20 characters. `=` characters represent completed work
    /// and `-` characters represent remaining work.
    pub fn render(&self) -> String {
        let bar = render_bar(
            self.current,
            self.total,
            self.bar_width,
            self.fill_char,
            self.empty_char,
        );
        format!("[{}] {}/{} {}", bar, self.current, self.total, self.label)
    }
    /// Render with percentage and ETA.
    pub fn render_full(&self) -> String {
        let bar = render_bar(
            self.current,
            self.total,
            self.bar_width,
            self.fill_char,
            self.empty_char,
        );
        let pct = if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        };
        let eta_str = if let Some(_t0) = self.started_at {
            if self.ema_rate > 0.0 {
                let remaining = self.total.saturating_sub(self.current) as f64;
                let eta_secs = remaining / self.ema_rate;
                format!(
                    " ETA:{}",
                    format_duration(Duration::from_secs_f64(eta_secs.min(99999.0)))
                )
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        format!(
            "[{}] {:.1}% {}/{}{}{}",
            bar,
            pct,
            self.current,
            self.total,
            if eta_str.is_empty() { "" } else { " " },
            eta_str
        )
    }
    /// Render to a fixed terminal width.
    pub fn render_fitted(&self, width: usize) -> String {
        let base = self.render_full();
        if base.len() <= width {
            base
        } else {
            base[..width].to_string()
        }
    }
    /// Returns `true` when `current >= total`.
    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }
    /// Reset `current` back to zero without changing `total` or `label`.
    pub fn reset(&mut self) {
        self.current = 0;
        self.ema_rate = 0.0;
        self.started_at = None;
    }
    /// Current value.
    pub fn current(&self) -> usize {
        self.current
    }
    /// Total value.
    pub fn total(&self) -> usize {
        self.total
    }
    /// Percentage as a float \[0,100\].
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }
}
/// Status of a single step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    InProgress,
    Done,
    Skipped,
    Error,
}
impl StepStatus {
    pub fn icon(self) -> &'static str {
        match self {
            StepStatus::Pending => "[ ]",
            StepStatus::InProgress => "[~]",
            StepStatus::Done => "[✓]",
            StepStatus::Skipped => "[-]",
            StepStatus::Error => "[✗]",
        }
    }
}
/// Spinner animation style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpinnerStyle {
    /// Braille dots rotating animation.
    Braille,
    /// Simple ASCII pipe/dash animation.
    Ascii,
    /// Growing/shrinking dots.
    Dots,
    /// Rotating arrows.
    Arrows,
    /// Clock hands.
    Clock,
}
impl SpinnerStyle {
    fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Braille => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Ascii => &["|", "/", "-", "\\"],
            SpinnerStyle::Dots => &[".", "..", "..."],
            SpinnerStyle::Arrows => &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            SpinnerStyle::Clock => &[
                "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
            ],
        }
    }
    /// Number of frames in this style.
    pub fn frame_count(self) -> usize {
        self.frames().len()
    }
}
/// Tracks progress of a batch of items with optional per-item status.
pub struct BatchProgress {
    items: Vec<BatchItem>,
    color: ColorMode,
}
impl BatchProgress {
    /// Create with a list of item names.
    pub fn new(items: Vec<String>) -> Self {
        BatchProgress {
            items: items
                .into_iter()
                .map(|name| BatchItem {
                    name,
                    status: BatchItemStatus::Queued,
                })
                .collect(),
            color: ColorMode::Never,
        }
    }
    /// Set color mode.
    pub fn set_color(&mut self, color: ColorMode) {
        self.color = color;
    }
    /// Set status of item at index.
    pub fn set_status(&mut self, idx: usize, status: BatchItemStatus) {
        if let Some(item) = self.items.get_mut(idx) {
            item.status = status;
        }
    }
    /// Set status by name (first match).
    pub fn set_status_by_name(&mut self, name: &str, status: BatchItemStatus) {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            item.status = status;
        }
    }
    /// Count items with a given status.
    pub fn count_with_status(&self, status: BatchItemStatus) -> usize {
        self.items.iter().filter(|i| i.status == status).count()
    }
    /// Overall counts.
    pub fn success_count(&self) -> usize {
        self.count_with_status(BatchItemStatus::Success)
    }
    pub fn failure_count(&self) -> usize {
        self.count_with_status(BatchItemStatus::Failure)
    }
    pub fn queued_count(&self) -> usize {
        self.count_with_status(BatchItemStatus::Queued)
    }
    pub fn processing_count(&self) -> usize {
        self.count_with_status(BatchItemStatus::Processing)
    }
    pub fn warning_count(&self) -> usize {
        self.count_with_status(BatchItemStatus::Warning)
    }
    /// Total items.
    pub fn total(&self) -> usize {
        self.items.len()
    }
    /// Render a compact summary line.
    pub fn render_summary(&self) -> String {
        format!(
            "Batch: {total} items | ✓{ok} ✗{fail} ⚠{warn} ·{queued}",
            total = self.total(),
            ok = self.success_count(),
            fail = self.failure_count(),
            warn = self.warning_count(),
            queued = self.queued_count()
        )
    }
    /// Render full item list.
    pub fn render_all(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            let _ = writeln!(out, "  {} {}", item.status.icon(), item.name);
        }
        out.push_str(&self.render_summary());
        out
    }
    /// Render only failures.
    pub fn render_failures(&self) -> String {
        let mut out = String::new();
        for item in self
            .items
            .iter()
            .filter(|i| i.status == BatchItemStatus::Failure)
        {
            let _ = writeln!(out, "  ✗ {}", item.name);
        }
        out
    }
    /// Returns true if all items are done (success or failure).
    pub fn is_done(&self) -> bool {
        self.items.iter().all(|i| {
            matches!(
                i.status,
                BatchItemStatus::Success | BatchItemStatus::Failure | BatchItemStatus::Warning
            )
        })
    }
}
/// A multi-phase progress reporter that tracks progress across named phases.
pub struct ProgressReporter {
    pub(super) phases: Vec<String>,
    current_phase: usize,
    /// Maps phase name → (current, total).
    pub(super) phase_progress: HashMap<String, (usize, usize)>,
    transitions: Vec<PhaseTransition>,
    pub started_at: Instant,
}
impl ProgressReporter {
    /// Create a new reporter with the given ordered list of phase names.
    pub fn new(phases: Vec<String>) -> Self {
        ProgressReporter {
            phases,
            current_phase: 0,
            phase_progress: HashMap::new(),
            transitions: Vec::new(),
            started_at: Instant::now(),
        }
    }
    /// Register a phase with its total work units and reset its current count.
    pub fn start_phase(&mut self, phase: &str, total: usize) {
        let prev_phase = if self.current_phase < self.phases.len() {
            Some(self.phases[self.current_phase].clone())
        } else {
            None
        };
        self.phase_progress.insert(phase.to_string(), (0, total));
        if let Some(idx) = self.phases.iter().position(|p| p == phase) {
            self.current_phase = idx;
        }
        let transition = PhaseTransition::new(prev_phase.as_deref(), phase);
        self.transitions.push(transition);
    }
    /// Advance the named phase by 1.  Returns `true` if that phase is now complete.
    pub fn advance_phase(&mut self, phase: &str) -> bool {
        if let Some(entry) = self.phase_progress.get_mut(phase) {
            if entry.0 < entry.1 {
                entry.0 += 1;
            }
            entry.0 >= entry.1
        } else {
            false
        }
    }
    /// Advance the named phase by n steps. Returns true if complete.
    pub fn advance_phase_by(&mut self, phase: &str, n: usize) -> bool {
        let mut complete = false;
        for _ in 0..n {
            complete = self.advance_phase(phase);
        }
        complete
    }
    /// Return a human-readable summary of all phases.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        for phase in &self.phases {
            if let Some(&(cur, tot)) = self.phase_progress.get(phase) {
                let pct = if tot == 0 {
                    100.0_f64
                } else {
                    (cur as f64 / tot as f64) * 100.0
                };
                lines.push(format!("{}: {}/{} ({:.1}%)", phase, cur, tot, pct));
            } else {
                lines.push(format!("{}: not started", phase));
            }
        }
        lines.join("\n")
    }
    /// Return overall completion percentage across all started phases.
    pub fn overall_percentage(&self) -> f64 {
        let total_work: usize = self.phase_progress.values().map(|&(_, t)| t).sum();
        let done_work: usize = self.phase_progress.values().map(|&(c, _)| c).sum();
        if total_work == 0 {
            100.0
        } else {
            (done_work as f64 / total_work as f64) * 100.0
        }
    }
    /// Return elapsed time since reporter was created.
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }
    /// Return phase transition history.
    pub fn transitions(&self) -> &[PhaseTransition] {
        &self.transitions
    }
    /// Get current phase name.
    pub fn current_phase_name(&self) -> Option<&str> {
        self.phases.get(self.current_phase).map(|s| s.as_str())
    }
    /// Get progress for a named phase: returns (current, total) or None.
    pub fn phase_progress(&self, phase: &str) -> Option<(usize, usize)> {
        self.phase_progress.get(phase).copied()
    }
    /// Return a JSON-formatted progress string.
    pub fn to_json(&self) -> String {
        reporter_to_json(self)
    }
    /// List all known phase names.
    pub fn phases(&self) -> &[String] {
        &self.phases
    }
}
struct BatchItem {
    name: String,
    status: BatchItemStatus,
}
