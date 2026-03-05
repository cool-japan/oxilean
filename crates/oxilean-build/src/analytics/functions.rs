//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    AnalyticsExporter, BuildAnalytics, BuildEvent, BuildProfiler, BuildReport, BuildSession,
    BuildTimeline, BuildTrendAnalyzer, CompilerFlagAnalytics, CriticalPathNode, DependencyStats,
    ErrorRateTracker, ExportFormat, FileComplexityMetrics, GraphBuildAnalytics,
    IncrementalSavingsEstimator, MemoryUsageTracker, MetricsAggregator, ParallelismSnapshot,
    PhaseBreakdown, ProfilingSpan, TimelineEvent, TimelineEventState, WatchCycleStat,
    WatchModeAnalytics, WorkerStats,
};

/// Compute dependency statistics from a file-to-imports map.
///
/// `deps` maps each file to the list of files it directly imports.
pub fn compute_dependency_stats(deps: &HashMap<String, Vec<String>>) -> DependencyStats {
    let file_count = deps.len();
    let edge_count: usize = deps.values().map(|v| v.len()).sum();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for file in deps.keys() {
        in_degree.entry(file.clone()).or_insert(0);
    }
    for imports in deps.values() {
        for dep in imports {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }
    let mut most_depended: Vec<(String, usize)> = in_degree.into_iter().collect();
    most_depended.sort_by(|a, b| b.1.cmp(&a.1));
    most_depended.truncate(10);
    let max_depth = compute_max_depth(deps);
    let strongly_connected = count_sccs(deps);
    DependencyStats {
        file_count,
        edge_count,
        max_depth,
        strongly_connected,
        most_depended_on: most_depended,
    }
}
/// Compute the maximum chain depth using BFS/DFS from sources.
fn compute_max_depth(deps: &HashMap<String, Vec<String>>) -> usize {
    if deps.is_empty() {
        return 0;
    }
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for k in deps.keys() {
        in_degree.entry(k.as_str()).or_insert(0);
    }
    for imports in deps.values() {
        for dep in imports {
            if deps.contains_key(dep) {
                *in_degree.entry(dep.as_str()).or_insert(0) += 1;
            }
        }
    }
    let mut depth: HashMap<&str, usize> = HashMap::new();
    use std::collections::VecDeque;
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (node, &deg) in &in_degree {
        if deg == 0 {
            depth.insert(node, 0);
            queue.push_back(node);
        }
    }
    let mut max_d = 0;
    while let Some(node) = queue.pop_front() {
        let cur_depth = depth[node];
        if cur_depth > max_d {
            max_d = cur_depth;
        }
        if let Some(imports) = deps.get(node) {
            for dep in imports {
                if deps.contains_key(dep.as_str()) {
                    let new_d = cur_depth + 1;
                    let entry = depth.entry(dep.as_str()).or_insert(0);
                    if new_d > *entry {
                        *entry = new_d;
                        queue.push_back(dep.as_str());
                    }
                }
            }
        }
    }
    max_d
}
/// Count strongly connected components using Kosaraju's algorithm (stub).
fn count_sccs(deps: &HashMap<String, Vec<String>>) -> usize {
    if deps.is_empty() {
        return 0;
    }
    let nodes: Vec<&str> = deps.keys().map(|s| s.as_str()).collect();
    let node_idx: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let n = nodes.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (src, imports) in deps {
        if let Some(&si) = node_idx.get(src.as_str()) {
            for dep in imports {
                if let Some(&di) = node_idx.get(dep.as_str()) {
                    adj[si].push(di);
                    radj[di].push(si);
                }
            }
        }
    }
    let mut visited = vec![false; n];
    let mut finish_order: Vec<usize> = Vec::new();
    for start in 0..n {
        if !visited[start] {
            dfs_finish(&adj, start, &mut visited, &mut finish_order);
        }
    }
    let mut visited2 = vec![false; n];
    let mut scc_count = 0;
    for &node in finish_order.iter().rev() {
        if !visited2[node] {
            dfs_mark(&radj, node, &mut visited2);
            scc_count += 1;
        }
    }
    scc_count
}
fn dfs_finish(adj: &[Vec<usize>], start: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
    visited[start] = true;
    while let Some((node, idx)) = stack.last_mut() {
        let node = *node;
        let i = *idx;
        if i < adj[node].len() {
            // Safety: stack is guaranteed non-empty here (inside while let Some on stack.last_mut())
            *stack
                .last_mut()
                .expect("stack is non-empty inside while let Some loop") = (node, i + 1);
            let next = adj[node][i];
            if !visited[next] {
                visited[next] = true;
                stack.push((next, 0));
            }
        } else {
            stack.pop();
            order.push(node);
        }
    }
}
fn dfs_mark(radj: &[Vec<usize>], start: usize, visited: &mut [bool]) {
    let mut stack = vec![start];
    visited[start] = true;
    while let Some(node) = stack.pop() {
        for &next in &radj[node] {
            if !visited[next] {
                visited[next] = true;
                stack.push(next);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analytics_new() {
        let a = BuildAnalytics::new();
        assert_eq!(a.total_files, 0);
        assert_eq!(a.cached_files, 0);
        assert_eq!(a.events.len(), 0);
        assert_eq!(a.total_duration_ms(), 0);
    }
    #[test]
    fn test_record_events() {
        let mut a = BuildAnalytics::new();
        a.record(BuildEvent::FileStart {
            path: "foo.lean".into(),
            size_bytes: 1024,
        });
        a.record(BuildEvent::FileEnd {
            path: "foo.lean".into(),
            duration_ms: 50,
            declarations: 3,
        });
        a.record(BuildEvent::CacheHit("bar.lean".into()));
        assert_eq!(a.total_files, 1);
        assert_eq!(a.cached_files, 1);
        assert_eq!(a.total_declarations(), 3);
        assert_eq!(a.total_duration_ms(), 50);
    }
    #[test]
    fn test_cache_hit_rate() {
        let mut a = BuildAnalytics::new();
        a.record(BuildEvent::CacheHit("a.lean".into()));
        a.record(BuildEvent::CacheHit("b.lean".into()));
        a.record(BuildEvent::FileEnd {
            path: "c.lean".into(),
            duration_ms: 10,
            declarations: 1,
        });
        a.record(BuildEvent::FileEnd {
            path: "d.lean".into(),
            duration_ms: 10,
            declarations: 1,
        });
        let rate = a.cache_hit_rate();
        assert!((rate - 0.5).abs() < 1e-9, "rate={}", rate);
    }
    #[test]
    fn test_generate_report() {
        let mut a = BuildAnalytics::new();
        a.record(BuildEvent::FileEnd {
            path: "x.lean".into(),
            duration_ms: 100,
            declarations: 5,
        });
        a.record(BuildEvent::Error {
            path: "y.lean".into(),
            message: "oops".into(),
        });
        let r = a.generate_report();
        assert_eq!(r.total_files, 1);
        assert_eq!(r.total_declarations, 5);
        assert_eq!(r.errors, 1);
        assert_eq!(r.total_duration_ms, 100);
    }
    #[test]
    fn test_report_to_text() {
        let r = BuildReport {
            total_files: 10,
            cached_files: 4,
            total_declarations: 42,
            total_duration_ms: 500,
            cache_hit_rate: 0.286,
            errors: 0,
            slowest_files: vec![("slow.lean".into(), 200)],
        };
        let text = r.to_text();
        assert!(text.contains("10"));
        assert!(text.contains("slow.lean"));
        let json = r.to_json();
        assert!(json.contains("\"total_files\": 10"));
        assert!(json.contains("slow.lean"));
    }
    #[test]
    fn test_dependency_stats() {
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        deps.insert("a.lean".into(), vec!["b.lean".into(), "c.lean".into()]);
        deps.insert("b.lean".into(), vec!["c.lean".into()]);
        deps.insert("c.lean".into(), vec![]);
        let stats = compute_dependency_stats(&deps);
        assert_eq!(stats.file_count, 3);
        assert_eq!(stats.edge_count, 3);
        assert!(stats.max_depth >= 2);
        assert!(!stats.most_depended_on.is_empty());
        assert_eq!(stats.most_depended_on[0].0, "c.lean");
    }
    #[test]
    fn test_slowest_files() {
        let mut a = BuildAnalytics::new();
        a.record(BuildEvent::FileEnd {
            path: "fast.lean".into(),
            duration_ms: 10,
            declarations: 1,
        });
        a.record(BuildEvent::FileEnd {
            path: "slow.lean".into(),
            duration_ms: 200,
            declarations: 2,
        });
        a.record(BuildEvent::FileEnd {
            path: "medium.lean".into(),
            duration_ms: 50,
            declarations: 1,
        });
        let top2 = a.slowest_files(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].0, "slow.lean");
        assert_eq!(top2[0].1, 200);
    }
}
/// Computes an overall "health score" for a build (0.0 = terrible, 100.0 = perfect).
#[allow(dead_code)]
pub fn compute_build_health_score(report: &BuildReport) -> f64 {
    let mut score = 100.0_f64;
    let error_penalty = (report.errors as f64) * 10.0;
    score -= error_penalty;
    let cache_bonus = report.cache_hit_rate * 10.0;
    score += cache_bonus;
    if report.total_files == 0 {
        score -= 20.0;
    }
    if report.total_files > 0 {
        let avg_ms = report.total_duration_ms as f64 / report.total_files as f64;
        if avg_ms > 500.0 {
            score -= ((avg_ms - 500.0) / 100.0).min(20.0);
        }
    }
    score.clamp(0.0, 100.0)
}
/// Computes the critical path through a dependency DAG.
///
/// `deps` maps each node to its direct dependencies.
/// `durations` maps each node to its duration in ms.
#[allow(dead_code)]
pub fn compute_critical_path(
    deps: &HashMap<String, Vec<String>>,
    durations: &HashMap<String, u64>,
) -> Vec<CriticalPathNode> {
    if deps.is_empty() {
        return Vec::new();
    }
    let mut in_degree: HashMap<&str, usize> = deps.keys().map(|k| (k.as_str(), 0)).collect();
    for children in deps.values() {
        for c in children {
            *in_degree.entry(c.as_str()).or_insert(0) += 1;
        }
    }
    let mut queue: std::collections::VecDeque<&str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&k, _)| k)
        .collect();
    let mut topo: Vec<&str> = Vec::new();
    while let Some(node) = queue.pop_front() {
        topo.push(node);
        if let Some(children) = deps.get(node) {
            for child in children {
                let d = in_degree.entry(child.as_str()).or_insert(0);
                *d = d.saturating_sub(1);
                if *d == 0 {
                    queue.push_back(child.as_str());
                }
            }
        }
    }
    let mut earliest_finish: HashMap<&str, u64> = HashMap::new();
    for &node in &topo {
        let dur = durations.get(node).cloned().unwrap_or(0);
        let es = if let Some(children) = deps.get(node) {
            children
                .iter()
                .map(|c| earliest_finish.get(c.as_str()).cloned().unwrap_or(0))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        earliest_finish.insert(node, es + dur);
    }
    let project_duration = earliest_finish.values().cloned().max().unwrap_or(0);
    let mut latest_start: HashMap<&str, u64> = HashMap::new();
    for &node in topo.iter().rev() {
        let dur = durations.get(node).cloned().unwrap_or(0);
        let ls = if let Some(ls_val) = latest_start.get(node) {
            *ls_val
        } else {
            let ef = earliest_finish.get(node).cloned().unwrap_or(0);
            project_duration.saturating_sub(project_duration - ef + dur)
        };
        latest_start.insert(node, ls);
        let _ = dur;
    }
    topo.iter()
        .map(|&node| {
            let dur = durations.get(node).cloned().unwrap_or(0);
            let ef = earliest_finish.get(node).cloned().unwrap_or(0);
            let es = ef.saturating_sub(dur);
            let lf = earliest_finish.get(node).cloned().unwrap_or(0);
            let ls_val = lf.saturating_sub(dur);
            let float = ls_val.saturating_sub(es);
            CriticalPathNode {
                name: node.to_string(),
                duration_ms: dur,
                earliest_start: es,
                earliest_finish: ef,
                latest_start: ls_val,
                latest_finish: lf,
                float,
            }
        })
        .collect()
}
/// Analyzes build parallelism from a set of timeline events.
#[allow(dead_code)]
pub fn analyze_parallelism(events: &[TimelineEvent]) -> Vec<ParallelismSnapshot> {
    let mut changes: Vec<(u64, i64)> = Vec::new();
    for ev in events {
        if ev.end_ms > 0 {
            changes.push((ev.start_ms, 1));
            changes.push((ev.end_ms, -1));
        }
    }
    changes.sort_by_key(|&(ts, _)| ts);
    let mut snapshots = Vec::new();
    let mut running: i64 = 0;
    for (ts, delta) in changes {
        running += delta;
        snapshots.push(ParallelismSnapshot {
            ts_ms: ts,
            concurrent_tasks: running.max(0) as usize,
        });
    }
    snapshots
}
/// Compute the peak concurrency from a set of snapshots.
#[allow(dead_code)]
pub fn peak_parallelism(snapshots: &[ParallelismSnapshot]) -> usize {
    snapshots
        .iter()
        .map(|s| s.concurrent_tasks)
        .max()
        .unwrap_or(0)
}
/// Compute the average concurrency from snapshots.
#[allow(dead_code)]
pub fn average_parallelism(snapshots: &[ParallelismSnapshot]) -> f64 {
    if snapshots.is_empty() {
        return 0.0;
    }
    let sum: usize = snapshots.iter().map(|s| s.concurrent_tasks).sum();
    sum as f64 / snapshots.len() as f64
}
/// Aggregate complexity metrics for a set of files.
#[allow(dead_code)]
pub fn aggregate_complexity(files: &[FileComplexityMetrics]) -> FileComplexityMetrics {
    let mut agg = FileComplexityMetrics::new("<aggregate>");
    for f in files {
        agg.line_count += f.line_count;
        agg.declaration_count += f.declaration_count;
        agg.import_count += f.import_count;
        agg.cyclomatic_complexity += f.cyclomatic_complexity;
        agg.comment_lines += f.comment_lines;
    }
    agg
}
/// Aggregate per-worker stats from a set of timeline events.
#[allow(dead_code)]
pub fn aggregate_worker_stats(events: &[TimelineEvent]) -> Vec<WorkerStats> {
    let mut map: HashMap<u32, WorkerStats> = HashMap::new();
    for ev in events {
        let stats = map
            .entry(ev.worker_id)
            .or_insert_with(|| WorkerStats::new(ev.worker_id));
        match ev.state {
            TimelineEventState::Done => {
                stats.total_ms += ev.duration_ms();
                stats.unit_count += 1;
            }
            TimelineEventState::Skipped => {
                stats.cache_hits += 1;
            }
            _ => {}
        }
    }
    let mut v: Vec<WorkerStats> = map.into_values().collect();
    v.sort_by_key(|s| s.worker_id);
    v
}
#[cfg(test)]
mod extra_analytics_tests {
    use super::*;
    #[test]
    fn test_profiling_span_open_close() {
        let mut s = ProfilingSpan::open("parse", 0);
        assert!(s.is_open());
        s.close(50);
        assert!(!s.is_open());
        assert_eq!(s.duration(), 50);
    }
    #[test]
    fn test_profiling_span_with_meta() {
        let s = ProfilingSpan::open("elab", 0).with_meta("file", "foo.lean");
        assert_eq!(s.metadata.get("file").map(|v| v.as_str()), Some("foo.lean"));
    }
    #[test]
    fn test_profiler_begin_end() {
        let mut p = BuildProfiler::new();
        let i = p.begin("parse");
        p.end(i);
        assert_eq!(p.span_count(), 1);
        assert_eq!(p.completed_spans().len(), 1);
    }
    #[test]
    fn test_profiler_child_span() {
        let mut p = BuildProfiler::new();
        let parent = p.begin("compile");
        let child = p.begin_child("codegen", parent);
        p.end(child);
        p.end(parent);
        assert_eq!(p.span_count(), 2);
    }
    #[test]
    fn test_profiler_slowest_span() {
        let mut p = BuildProfiler::new();
        let i1 = p.begin("fast");
        p.end(i1);
        let i2 = p.begin("slow");
        p.end(i2);
        let s = p.slowest_span();
        assert!(s.is_some());
    }
    #[test]
    fn test_profiler_average_duration_for() {
        let mut p = BuildProfiler::new();
        let i1 = p.begin("parse");
        p.end(i1);
        let i2 = p.begin("parse");
        p.end(i2);
        let avg = p.average_duration_for("parse");
        assert!(avg.is_some());
    }
    #[test]
    fn test_profiler_to_csv() {
        let mut p = BuildProfiler::new();
        let i = p.begin("link");
        p.end(i);
        let csv = p.to_csv();
        assert!(csv.contains("link"));
        assert!(csv.contains("name,start_ts"));
    }
    #[test]
    fn test_memory_tracker_allocate_deallocate() {
        let mut m = MemoryUsageTracker::new();
        m.allocate(1024);
        m.allocate(2048);
        assert_eq!(m.current_bytes(), 3072);
        assert_eq!(m.peak_bytes(), 3072);
        m.deallocate(1024);
        assert_eq!(m.current_bytes(), 2048);
        assert_eq!(m.peak_bytes(), 3072);
    }
    #[test]
    fn test_memory_tracker_live_allocations() {
        let mut m = MemoryUsageTracker::new();
        m.allocate(100);
        m.allocate(200);
        m.deallocate(100);
        assert_eq!(m.live_allocations(), 1);
    }
    #[test]
    fn test_memory_tracker_reset() {
        let mut m = MemoryUsageTracker::new();
        m.allocate(500);
        m.reset();
        assert_eq!(m.current_bytes(), 0);
        assert_eq!(m.peak_bytes(), 0);
    }
    #[test]
    fn test_metrics_aggregator_mean() {
        let mut agg = MetricsAggregator::new();
        agg.push("fps", 0, 10.0);
        agg.push("fps", 1, 20.0);
        agg.push("fps", 2, 30.0);
        let mean = agg.mean("fps").expect("test operation should succeed");
        assert!((mean - 20.0).abs() < 1e-9);
    }
    #[test]
    fn test_metrics_aggregator_min_max() {
        let mut agg = MetricsAggregator::new();
        agg.push("latency", 0, 5.0);
        agg.push("latency", 1, 100.0);
        agg.push("latency", 2, 50.0);
        assert!((agg.min("latency").expect("test operation should succeed") - 5.0).abs() < 1e-9);
        assert!((agg.max("latency").expect("test operation should succeed") - 100.0).abs() < 1e-9);
    }
    #[test]
    fn test_metrics_aggregator_std_dev() {
        let mut agg = MetricsAggregator::new();
        agg.push("x", 0, 2.0);
        agg.push("x", 1, 4.0);
        agg.push("x", 2, 4.0);
        agg.push("x", 3, 4.0);
        agg.push("x", 4, 5.0);
        agg.push("x", 5, 5.0);
        agg.push("x", 6, 7.0);
        agg.push("x", 7, 9.0);
        let sd = agg.std_dev("x").expect("test operation should succeed");
        assert!(sd > 0.0);
    }
    #[test]
    fn test_metrics_aggregator_percentile_95() {
        let mut agg = MetricsAggregator::new();
        for i in 0..100u64 {
            agg.push("val", i, i as f64);
        }
        let p95 = agg
            .percentile_95("val")
            .expect("test operation should succeed");
        assert!(p95 >= 94.0 && p95 <= 99.0, "p95={}", p95);
    }
    #[test]
    fn test_metrics_aggregator_window() {
        let mut agg = MetricsAggregator::new();
        agg.push("cpu", 100, 0.5);
        agg.push("cpu", 200, 0.8);
        agg.push("cpu", 300, 0.3);
        let w = agg.window("cpu", 150, 250);
        assert_eq!(w.len(), 1);
        assert!((w[0].value - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_metrics_aggregator_missing_series() {
        let agg = MetricsAggregator::new();
        assert!(agg.mean("nonexistent").is_none());
        assert!(agg.min("nonexistent").is_none());
        assert!(agg.max("nonexistent").is_none());
    }
    #[test]
    fn test_timeline_push_and_count() {
        let mut tl = BuildTimeline::new();
        let mut ev = TimelineEvent::start("foo.lean", 0, 0);
        ev.finish(100);
        tl.push(ev);
        assert_eq!(tl.events().len(), 1);
    }
    #[test]
    fn test_timeline_count_in_state() {
        let mut tl = BuildTimeline::new();
        let mut done = TimelineEvent::start("a.lean", 0, 0);
        done.finish(10);
        let mut fail = TimelineEvent::start("b.lean", 0, 0);
        fail.fail(5);
        tl.push(done);
        tl.push(fail);
        assert_eq!(tl.count_in_state(&TimelineEventState::Done), 1);
        assert_eq!(tl.count_in_state(&TimelineEventState::Failed), 1);
    }
    #[test]
    fn test_timeline_total_span() {
        let mut tl = BuildTimeline::new();
        let mut ev = TimelineEvent::start("x", 100, 0);
        ev.finish(300);
        tl.push(ev);
        assert_eq!(tl.total_span_ms(), 200);
    }
    #[test]
    fn test_timeline_gantt_text() {
        let mut tl = BuildTimeline::new();
        let mut ev = TimelineEvent::start("mod", 0, 0);
        ev.finish(5);
        tl.push(ev);
        let g = tl.to_gantt_text(1);
        assert!(g.contains('W'));
    }
    #[test]
    fn test_incremental_savings() {
        let mut est = IncrementalSavingsEstimator::new(100.0);
        est.record_cache_hit();
        est.record_cache_hit();
        est.record_compilation(80);
        assert!((est.estimated_savings_ms() - 200.0).abs() < 1e-9);
        assert_eq!(est.actual_compile_ms(), 80);
        assert_eq!(est.total_files_hypothetical(), 3);
        let eff = est.cache_efficiency();
        assert!((eff - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_incremental_savings_speedup() {
        let mut est = IncrementalSavingsEstimator::new(50.0);
        est.record_cache_hit();
        est.record_compilation(100);
        let sf = est.speedup_factor();
        assert!(sf > 1.0);
    }
    #[test]
    fn test_build_health_perfect() {
        let r = BuildReport {
            total_files: 10,
            cached_files: 5,
            total_declarations: 100,
            total_duration_ms: 1000,
            cache_hit_rate: 0.5,
            errors: 0,
            slowest_files: vec![],
        };
        let score = compute_build_health_score(&r);
        assert!(score > 50.0, "score={}", score);
    }
    #[test]
    fn test_build_health_with_errors() {
        let r = BuildReport {
            total_files: 5,
            cached_files: 0,
            total_declarations: 20,
            total_duration_ms: 500,
            cache_hit_rate: 0.0,
            errors: 8,
            slowest_files: vec![],
        };
        let score = compute_build_health_score(&r);
        assert!(score < 50.0, "score={}", score);
    }
    #[test]
    fn test_error_rate_tracker_overall() {
        let mut t = ErrorRateTracker::new();
        t.record(0, false);
        t.record(1, true);
        t.record(2, false);
        t.record(3, true);
        assert!((t.overall_error_rate() - 0.5).abs() < 1e-9);
        assert_eq!(t.total_errors(), 2);
    }
    #[test]
    fn test_error_rate_tracker_window() {
        let mut t = ErrorRateTracker::new();
        t.record(100, true);
        t.record(200, false);
        t.record(300, true);
        let rate = t.error_rate_in_window(150, 350);
        assert!((rate - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_file_complexity_doc_density() {
        let mut m = FileComplexityMetrics::new("foo.lean");
        m.line_count = 100;
        m.comment_lines = 25;
        assert!((m.doc_density() - 0.25).abs() < 1e-9);
    }
    #[test]
    fn test_file_complexity_is_complex() {
        let mut m = FileComplexityMetrics::new("big.lean");
        m.line_count = 600;
        assert!(m.is_complex());
        let mut m2 = FileComplexityMetrics::new("small.lean");
        m2.line_count = 10;
        m2.cyclomatic_complexity = 2;
        assert!(!m2.is_complex());
    }
    #[test]
    fn test_aggregate_complexity() {
        let files = vec![
            {
                let mut f = FileComplexityMetrics::new("a.lean");
                f.line_count = 100;
                f.declaration_count = 5;
                f
            },
            {
                let mut f = FileComplexityMetrics::new("b.lean");
                f.line_count = 200;
                f.declaration_count = 10;
                f
            },
        ];
        let agg = aggregate_complexity(&files);
        assert_eq!(agg.line_count, 300);
        assert_eq!(agg.declaration_count, 15);
    }
    #[test]
    fn test_phase_breakdown_from_analytics() {
        let mut a = BuildAnalytics::new();
        a.record(BuildEvent::ParseEnd {
            path: "a.lean".into(),
            duration_ms: 30,
        });
        a.record(BuildEvent::ElabEnd {
            path: "a.lean".into(),
            duration_ms: 70,
        });
        let bd = PhaseBreakdown::from_analytics(&a);
        assert_eq!(bd.parse_ms, 30);
        assert_eq!(bd.elab_ms, 70);
        assert_eq!(bd.total_ms(), 100);
        assert!((bd.parse_fraction() - 0.3).abs() < 1e-9);
        assert!((bd.elab_fraction() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_worker_stats_avg() {
        let mut ws = WorkerStats::new(0);
        ws.total_ms = 300;
        ws.unit_count = 3;
        assert!((ws.avg_ms_per_unit() - 100.0).abs() < 1e-9);
    }
    #[test]
    fn test_aggregate_worker_stats() {
        let mut events = Vec::new();
        let mut e1 = TimelineEvent::start("a.lean", 0, 1);
        e1.finish(100);
        let mut e2 = TimelineEvent::start("b.lean", 0, 2);
        e2.finish(200);
        let mut e3 = TimelineEvent::start("c.lean", 0, 1);
        e3.skip();
        events.push(e1);
        events.push(e2);
        events.push(e3);
        let stats = aggregate_worker_stats(&events);
        assert_eq!(stats.len(), 2);
        let w1 = stats
            .iter()
            .find(|s| s.worker_id == 1)
            .expect("test operation should succeed");
        assert_eq!(w1.unit_count, 1);
        assert_eq!(w1.cache_hits, 1);
    }
    #[test]
    fn test_analytics_exporter_csv() {
        let a = BuildAnalytics::new();
        let exp = AnalyticsExporter::new(&a);
        let csv = exp.export(ExportFormat::Csv);
        assert!(csv.contains("metric,value"));
    }
    #[test]
    fn test_analytics_exporter_markdown() {
        let a = BuildAnalytics::new();
        let exp = AnalyticsExporter::new(&a);
        let md = exp.export(ExportFormat::Markdown);
        assert!(md.contains("| Metric | Value |"));
    }
    #[test]
    fn test_analytics_exporter_json() {
        let a = BuildAnalytics::new();
        let exp = AnalyticsExporter::new(&a);
        let json = exp.export(ExportFormat::Json);
        assert!(json.contains("total_files"));
    }
    #[test]
    fn test_watch_mode_analytics_avg_rebuild() {
        let mut w = WatchModeAnalytics::new();
        w.push_cycle(WatchCycleStat {
            cycle: 0,
            files_changed: 1,
            files_recompiled: 3,
            duration_ms: 100,
            success: true,
        });
        w.push_cycle(WatchCycleStat {
            cycle: 1,
            files_changed: 2,
            files_recompiled: 5,
            duration_ms: 200,
            success: true,
        });
        assert!((w.avg_rebuild_ms() - 150.0).abs() < 1e-9);
        assert!((w.success_rate() - 1.0).abs() < 1e-9);
        assert!((w.avg_files_recompiled() - 4.0).abs() < 1e-9);
    }
    #[test]
    fn test_watch_mode_analytics_fastest_slowest() {
        let mut w = WatchModeAnalytics::new();
        w.push_cycle(WatchCycleStat {
            cycle: 0,
            files_changed: 1,
            files_recompiled: 1,
            duration_ms: 50,
            success: true,
        });
        w.push_cycle(WatchCycleStat {
            cycle: 1,
            files_changed: 3,
            files_recompiled: 10,
            duration_ms: 500,
            success: false,
        });
        assert_eq!(
            w.fastest_cycle()
                .expect("timing should succeed")
                .duration_ms,
            50
        );
        assert_eq!(
            w.slowest_cycle()
                .expect("timing should succeed")
                .duration_ms,
            500
        );
        assert!((w.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_analyze_parallelism_basic() {
        let mut events = Vec::new();
        let mut e1 = TimelineEvent::start("a", 0, 0);
        e1.finish(100);
        let mut e2 = TimelineEvent::start("b", 50, 1);
        e2.finish(150);
        events.push(e1);
        events.push(e2);
        let snaps = analyze_parallelism(&events);
        let peak = peak_parallelism(&snaps);
        assert!(peak >= 2, "peak={}", peak);
    }
    #[test]
    fn test_average_parallelism_empty() {
        assert!((average_parallelism(&[]) - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_critical_path_basic() {
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        deps.insert("A".into(), vec!["B".into()]);
        deps.insert("B".into(), vec!["C".into()]);
        deps.insert("C".into(), vec![]);
        let mut durations: HashMap<String, u64> = HashMap::new();
        durations.insert("A".into(), 10);
        durations.insert("B".into(), 20);
        durations.insert("C".into(), 30);
        let path = compute_critical_path(&deps, &durations);
        assert!(!path.is_empty());
        let names: Vec<&str> = path.iter().map(|n| n.name.as_str()).collect();
        assert!(names.contains(&"A"));
        assert!(names.contains(&"B"));
        assert!(names.contains(&"C"));
    }
    #[test]
    fn test_critical_path_empty() {
        let deps: HashMap<String, Vec<String>> = HashMap::new();
        let durations: HashMap<String, u64> = HashMap::new();
        let path = compute_critical_path(&deps, &durations);
        assert!(path.is_empty());
    }
    #[test]
    fn test_timeline_event_state_display() {
        assert_eq!(format!("{}", TimelineEventState::Done), "done");
        assert_eq!(format!("{}", TimelineEventState::Failed), "failed");
        assert_eq!(format!("{}", TimelineEventState::Pending), "pending");
        assert_eq!(format!("{}", TimelineEventState::Running), "running");
        assert_eq!(format!("{}", TimelineEventState::Skipped), "skipped");
    }
}
/// Compute the graph diameter using BFS from each node.
#[allow(dead_code)]
pub(super) fn compute_graph_diameter(deps: &HashMap<String, Vec<String>>) -> usize {
    let nodes: Vec<&str> = deps.keys().map(|s| s.as_str()).collect();
    let mut max_dist = 0;
    for start in &nodes {
        let dist = bfs_max_dist(deps, start);
        if dist > max_dist {
            max_dist = dist;
        }
    }
    max_dist
}
/// BFS from `start` and return the maximum distance to any reachable node.
#[allow(dead_code)]
fn bfs_max_dist(deps: &HashMap<String, Vec<String>>, start: &str) -> usize {
    let mut visited: HashMap<&str, usize> = HashMap::new();
    let mut queue: std::collections::VecDeque<(&str, usize)> = std::collections::VecDeque::new();
    queue.push_back((start, 0));
    visited.insert(start, 0);
    let mut max_d = 0;
    while let Some((node, d)) = queue.pop_front() {
        if d > max_d {
            max_d = d;
        }
        if let Some(neighbors) = deps.get(node) {
            for nb in neighbors {
                if !visited.contains_key(nb.as_str()) {
                    visited.insert(nb.as_str(), d + 1);
                    queue.push_back((nb.as_str(), d + 1));
                }
            }
        }
    }
    max_d
}
#[cfg(test)]
mod final_analytics_tests {
    use super::*;
    #[test]
    fn test_build_trend_avg_duration() {
        let mut trend = BuildTrendAnalyzer::new();
        let mut s1 = BuildSession::new("abc", 1000);
        s1.duration_ms = 200;
        let mut s2 = BuildSession::new("def", 2000);
        s2.duration_ms = 400;
        trend.push_session(s1);
        trend.push_session(s2);
        assert!((trend.avg_duration_ms() - 300.0).abs() < 1e-9);
    }
    #[test]
    fn test_build_trend_slope_increasing() {
        let mut trend = BuildTrendAnalyzer::new();
        for i in 0u64..5 {
            let mut s = BuildSession::new(&format!("s{}", i), i * 1000);
            s.duration_ms = i * 100;
            trend.push_session(s);
        }
        let slope = trend.duration_trend_slope();
        assert!(slope > 0.0, "slope={}", slope);
    }
    #[test]
    fn test_build_trend_failing_sessions() {
        let mut trend = BuildTrendAnalyzer::new();
        let mut s1 = BuildSession::new("ok", 0);
        s1.errors = 0;
        let mut s2 = BuildSession::new("bad", 1000);
        s2.errors = 3;
        trend.push_session(s1);
        trend.push_session(s2);
        assert_eq!(trend.failing_sessions().len(), 1);
    }
    #[test]
    fn test_build_trend_avg_cache_hit_rate() {
        let mut trend = BuildTrendAnalyzer::new();
        let mut s1 = BuildSession::new("a", 0);
        s1.cache_hit_rate = 0.2;
        let mut s2 = BuildSession::new("b", 1);
        s2.cache_hit_rate = 0.8;
        trend.push_session(s1);
        trend.push_session(s2);
        assert!((trend.avg_cache_hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_graph_build_analytics_compute() {
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        deps.insert("a".into(), vec!["b".into()]);
        deps.insert("b".into(), vec!["c".into()]);
        deps.insert("c".into(), vec![]);
        let ga = GraphBuildAnalytics::compute(&deps);
        assert_eq!(ga.node_count, 3);
        assert_eq!(ga.edge_count, 2);
        assert!(ga.leaf_fraction > 0.0);
        assert_eq!(ga.isolated_count, 0);
    }
    #[test]
    fn test_graph_build_analytics_empty() {
        let deps: HashMap<String, Vec<String>> = HashMap::new();
        let ga = GraphBuildAnalytics::compute(&deps);
        assert_eq!(ga.node_count, 0);
        assert_eq!(ga.edge_count, 0);
        assert!((ga.avg_out_degree - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_compiler_flag_analytics_top_flags() {
        let mut cfa = CompilerFlagAnalytics::new();
        cfa.record_flags(&["-O2", "-g", "-O2", "-O2", "-g"]);
        let top = cfa.top_flags(2);
        assert_eq!(top[0].0, "-O2");
        assert_eq!(top[0].1, 3);
        assert_eq!(top[1].0, "-g");
        assert_eq!(top[1].1, 2);
    }
    #[test]
    fn test_compiler_flag_analytics_totals() {
        let mut cfa = CompilerFlagAnalytics::new();
        cfa.record_flags(&["-a", "-b", "-c"]);
        cfa.record_flags(&["-a"]);
        assert_eq!(cfa.distinct_flag_count(), 3);
        assert_eq!(cfa.total_flag_uses(), 4);
    }
}
