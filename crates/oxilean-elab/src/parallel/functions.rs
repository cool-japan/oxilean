//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;
use oxilean_parse::Decl;
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    BatchExecutionResult, DependencyAnalyzer, DependencyConfig, ElabOutput, ElabResult, ElabTask,
    ExecutionStats, ParallelBatchResult, ParallelElabConfig, ParallelElabStats, ParallelScheduler,
    ParallelTaskOutcome, ParallelTaskQueue, PrioritizedTask, ProgressTracker, SchedulerConfig,
    SchedulerSummary, TaskDependencyInfo, TaskError, TaskGraph, TaskId, TaskPriority,
    TaskPriorityQueue, TaskStatus, WorkStealDeque,
};

pub fn num_cpus_fallback() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
/// Collect all `Var` names referenced in a declaration's type/body.
pub fn collect_decl_refs(decl: &Decl, out: &mut HashSet<String>) {
    use oxilean_parse::Decl as D;
    use oxilean_parse::SurfaceExpr;
    fn collect_expr(expr: &SurfaceExpr, out: &mut HashSet<String>) {
        match expr {
            SurfaceExpr::Var(name) => {
                out.insert(name.clone());
            }
            SurfaceExpr::App(f, a) => {
                collect_expr(&f.value, out);
                collect_expr(&a.value, out);
            }
            SurfaceExpr::Lam(binders, body) | SurfaceExpr::Pi(binders, body) => {
                for b in binders {
                    if let Some(ty) = &b.ty {
                        collect_expr(&ty.value, out);
                    }
                }
                collect_expr(&body.value, out);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(t) = ty {
                    collect_expr(&t.value, out);
                }
                collect_expr(&val.value, out);
                collect_expr(&body.value, out);
            }
            SurfaceExpr::Ann(e, t) => {
                collect_expr(&e.value, out);
                collect_expr(&t.value, out);
            }
            SurfaceExpr::Proj(e, _) => collect_expr(&e.value, out),
            SurfaceExpr::If(c, t, e) => {
                collect_expr(&c.value, out);
                collect_expr(&t.value, out);
                collect_expr(&e.value, out);
            }
            SurfaceExpr::Match(scrut, arms) => {
                collect_expr(&scrut.value, out);
                for arm in arms {
                    if let Some(g) = &arm.guard {
                        collect_expr(&g.value, out);
                    }
                    collect_expr(&arm.rhs.value, out);
                }
            }
            SurfaceExpr::Have(_, ty, val, body) => {
                collect_expr(&ty.value, out);
                collect_expr(&val.value, out);
                collect_expr(&body.value, out);
            }
            SurfaceExpr::Suffices(_, ty, body) => {
                collect_expr(&ty.value, out);
                collect_expr(&body.value, out);
            }
            SurfaceExpr::Show(ty, e) => {
                collect_expr(&ty.value, out);
                collect_expr(&e.value, out);
            }
            SurfaceExpr::NamedArg(f, _, arg) => {
                collect_expr(&f.value, out);
                collect_expr(&arg.value, out);
            }
            SurfaceExpr::AnonymousCtor(elems)
            | SurfaceExpr::ListLit(elems)
            | SurfaceExpr::Tuple(elems) => {
                for e in elems {
                    collect_expr(&e.value, out);
                }
            }
            SurfaceExpr::Return(e) => collect_expr(&e.value, out),
            SurfaceExpr::Do(actions) => {
                for action in actions {
                    match action {
                        oxilean_parse::DoAction::Bind(_, e) => collect_expr(&e.value, out),
                        oxilean_parse::DoAction::Expr(e) => collect_expr(&e.value, out),
                        oxilean_parse::DoAction::Return(e) => collect_expr(&e.value, out),
                        oxilean_parse::DoAction::Let(_, e) => collect_expr(&e.value, out),
                        oxilean_parse::DoAction::LetTyped(_, e, ty) => {
                            collect_expr(&e.value, out);
                            collect_expr(&ty.value, out);
                        }
                    }
                }
            }
            SurfaceExpr::StringInterp(_parts) => {}
            _ => {}
        }
    }
    fn collect_located(e: &oxilean_parse::Located<SurfaceExpr>, out: &mut HashSet<String>) {
        collect_expr(&e.value, out);
    }
    match decl {
        D::Axiom { ty, .. } => collect_located(ty, out),
        D::Definition {
            ty,
            val,
            where_clauses,
            ..
        } => {
            if let Some(t) = ty {
                collect_located(t, out);
            }
            collect_located(val, out);
            for w in where_clauses {
                if let Some(t) = &w.ty {
                    collect_located(t, out);
                }
                collect_located(&w.val, out);
            }
        }
        D::Theorem {
            ty,
            proof,
            where_clauses,
            ..
        } => {
            collect_located(ty, out);
            collect_located(proof, out);
            for w in where_clauses {
                if let Some(t) = &w.ty {
                    collect_located(t, out);
                }
                collect_located(&w.val, out);
            }
        }
        D::Inductive { ty, ctors, .. } => {
            collect_located(ty, out);
            for ctor in ctors {
                collect_located(&ctor.ty, out);
            }
        }
        D::Structure {
            extends, fields, ..
        } => {
            for e in extends {
                out.insert(e.clone());
            }
            for f in fields {
                collect_located(&f.ty, out);
                if let Some(d) = &f.default {
                    collect_located(d, out);
                }
            }
        }
        D::ClassDecl {
            extends, fields, ..
        } => {
            for e in extends {
                out.insert(e.clone());
            }
            for f in fields {
                collect_located(&f.ty, out);
                if let Some(d) = &f.default {
                    collect_located(d, out);
                }
            }
        }
        D::InstanceDecl {
            ty,
            defs,
            class_name,
            ..
        } => {
            out.insert(class_name.clone());
            collect_located(ty, out);
            for (_, e) in defs {
                collect_located(e, out);
            }
        }
        D::Namespace { decls, .. } | D::SectionDecl { decls, .. } => {
            for d in decls {
                collect_decl_refs(&d.value, out);
            }
        }
        D::Mutual { decls } => {
            for d in decls {
                collect_decl_refs(&d.value, out);
            }
        }
        D::Attribute { decl, .. } => collect_decl_refs(&decl.value, out),
        D::HashCmd { arg, .. } => collect_located(arg, out),
        _ => {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parallel::*;
    #[test]
    fn test_task_id_creation() {
        let id1 = TaskId::new(0);
        let id2 = TaskId::new(1);
        assert_eq!(id1.inner(), 0);
        assert_eq!(id2.inner(), 1);
        assert_ne!(id1, id2);
    }
    #[test]
    fn test_task_id_display() {
        let id = TaskId::new(42);
        assert_eq!(format!("{}", id), "Task#42");
    }
    #[test]
    fn test_task_status_display() {
        assert_eq!(format!("{}", TaskStatus::Pending), "Pending");
        assert_eq!(format!("{}", TaskStatus::Running), "Running");
        assert_eq!(format!("{}", TaskStatus::Completed), "Completed");
        assert_eq!(format!("{}", TaskStatus::Failed), "Failed");
        assert_eq!(format!("{}", TaskStatus::Cancelled), "Cancelled");
    }
    #[test]
    fn test_elab_task_creation() {
        let id = TaskId::new(0);
        let task = ElabTask::new(id, "test".to_string(), Name::str("test_decl"));
        assert_eq!(task.id, id);
        assert_eq!(task.name, "test");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.deps.is_empty());
    }
    #[test]
    fn test_elab_task_add_dependency() {
        let id1 = TaskId::new(0);
        let id2 = TaskId::new(1);
        let mut task = ElabTask::new(id1, "test".to_string(), Name::str("test_decl"));
        task.add_dependency(id2);
        assert!(task.deps.contains(&id2));
        task.add_dependency(id2);
        assert_eq!(task.deps.len(), 1);
    }
    #[test]
    fn test_elab_task_is_ready() {
        let id0 = TaskId::new(0);
        let id1 = TaskId::new(1);
        let id2 = TaskId::new(2);
        let mut task = ElabTask::new(id2, "test".to_string(), Name::str("test_decl"));
        task.add_dependency(id0);
        task.add_dependency(id1);
        let mut completed = HashSet::new();
        assert!(!task.is_ready(&completed));
        completed.insert(id0);
        assert!(!task.is_ready(&completed));
        completed.insert(id1);
        assert!(task.is_ready(&completed));
    }
    #[test]
    fn test_task_graph_creation() {
        let graph = TaskGraph::new();
        assert_eq!(graph.total_count(), 0);
        assert_eq!(graph.completed_count(), 0);
        assert_eq!(graph.failed_count(), 0);
    }
    #[test]
    fn test_task_graph_add_task() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        assert_eq!(graph.total_count(), 2);
        assert_ne!(id1, id2);
    }
    #[test]
    fn test_task_graph_add_dependency() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        assert!(graph.add_dependency(id2, id1).is_ok());
        let task2 = graph.get_task(id2).expect("test operation should succeed");
        assert!(task2.deps.contains(&id1));
    }
    #[test]
    fn test_task_graph_add_dependency_nonexistent() {
        let mut graph = TaskGraph::new();
        let id_a = TaskId::new(999);
        let id_b = TaskId::new(998);
        assert!(graph.add_dependency(id_a, id_b).is_err());
    }
    #[test]
    fn test_task_graph_ready_tasks() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        let ready = graph.ready_tasks();
        assert_eq!(ready.len(), 1);
        assert!(ready.contains(&id1));
        graph.complete_task(id1).ok();
        let ready = graph.ready_tasks();
        assert_eq!(ready.len(), 1);
        assert!(ready.contains(&id2));
    }
    #[test]
    fn test_task_graph_complete_task() {
        let mut graph = TaskGraph::new();
        let id = graph.add_task("task".to_string(), Name::str("decl"));
        assert!(graph.complete_task(id).is_ok());
        assert_eq!(graph.completed_count(), 1);
        let task = graph.get_task(id).expect("test operation should succeed");
        assert_eq!(task.status, TaskStatus::Completed);
    }
    #[test]
    fn test_task_graph_fail_task() {
        let mut graph = TaskGraph::new();
        let id = graph.add_task("task".to_string(), Name::str("decl"));
        assert!(graph.fail_task(id, "test error".to_string()).is_ok());
        assert_eq!(graph.failed_count(), 1);
        let task = graph.get_task(id).expect("test operation should succeed");
        assert_eq!(task.status, TaskStatus::Failed);
    }
    #[test]
    fn test_task_graph_start_task() {
        let mut graph = TaskGraph::new();
        let id = graph.add_task("task".to_string(), Name::str("decl"));
        assert!(graph.start_task(id).is_ok());
        let task = graph.get_task(id).expect("test operation should succeed");
        assert_eq!(task.status, TaskStatus::Running);
    }
    #[test]
    fn test_task_graph_no_cycles() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        assert!(graph.detect_cycles().is_none());
    }
    #[test]
    fn test_task_graph_detects_cycles() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        graph.add_dependency(id1, id3).ok();
        assert!(graph.detect_cycles().is_some());
    }
    #[test]
    fn test_task_graph_topological_order_no_deps() {
        let mut graph = TaskGraph::new();
        let _id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let _id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let order = graph
            .topological_order()
            .expect("test operation should succeed");
        assert_eq!(order.len(), 2);
    }
    #[test]
    fn test_task_graph_topological_order_with_deps() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        let order = graph
            .topological_order()
            .expect("test operation should succeed");
        assert_eq!(order.len(), 3);
        let pos1 = order
            .iter()
            .position(|&id| id == id1)
            .expect("test operation should succeed");
        let pos2 = order
            .iter()
            .position(|&id| id == id2)
            .expect("test operation should succeed");
        let pos3 = order
            .iter()
            .position(|&id| id == id3)
            .expect("test operation should succeed");
        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
    }
    #[test]
    fn test_task_graph_topological_order_with_cycle() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        graph.add_dependency(id1, id2).ok();
        graph.add_dependency(id2, id1).ok();
        assert!(graph.topological_order().is_err());
    }
    #[test]
    fn test_task_graph_critical_path() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        let path = graph.critical_path_length(id3);
        assert_eq!(path, 2);
    }
    #[test]
    fn test_elab_output_creation() {
        let output = ElabOutput::new(Name::str("test"), "output".to_string());
        assert_eq!(output.decl_name, Name::str("test"));
        assert_eq!(output.decl, "output");
    }
    #[test]
    fn test_elab_result_creation() {
        let task_id = TaskId::new(0);
        let output = ElabOutput::new(Name::str("test"), "output".to_string());
        let result = ElabResult::new(task_id, Name::str("test"), 100, output);
        assert_eq!(result.task_id, task_id);
        assert_eq!(result.elapsed_ms, 100);
    }
    #[test]
    fn test_scheduler_config_defaults() {
        let config = SchedulerConfig::new();
        assert!(config.max_concurrent > 0);
        assert!(config.report_progress);
        assert_eq!(config.task_timeout_ms, 30000);
        assert_eq!(config.max_retries, 0);
    }
    #[test]
    fn test_scheduler_config_builder() {
        let config = SchedulerConfig::new()
            .with_max_concurrent(4)
            .with_progress_reporting(false)
            .with_timeout(60000)
            .with_max_retries(3);
        assert_eq!(config.max_concurrent, 4);
        assert!(!config.report_progress);
        assert_eq!(config.task_timeout_ms, 60000);
        assert_eq!(config.max_retries, 3);
    }
    #[test]
    fn test_parallel_scheduler_creation() {
        let scheduler = ParallelScheduler::new();
        assert_eq!(scheduler.graph.total_count(), 0);
        assert!(scheduler.results.is_empty());
    }
    #[test]
    fn test_parallel_scheduler_wavefront() {
        let mut scheduler = ParallelScheduler::new();
        let _id1 = scheduler
            .graph
            .add_task("task1".to_string(), Name::str("decl1"));
        let _id2 = scheduler
            .graph
            .add_task("task2".to_string(), Name::str("decl2"));
        let wavefront = scheduler.wavefront();
        assert_eq!(wavefront.len(), 2);
    }
    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(10);
        assert_eq!(tracker.total, 10);
        assert_eq!(tracker.completed, 0);
        assert_eq!(tracker.failed, 0);
        assert_eq!(tracker.progress_pct(), 0);
    }
    #[test]
    fn test_progress_tracker_mark_completed() {
        let mut tracker = ProgressTracker::new(10);
        tracker.mark_completed();
        assert_eq!(tracker.completed, 1);
        assert_eq!(tracker.progress_pct(), 10);
    }
    #[test]
    fn test_progress_tracker_mark_failed() {
        let mut tracker = ProgressTracker::new(10);
        tracker.mark_failed();
        assert_eq!(tracker.failed, 1);
    }
    #[test]
    fn test_progress_tracker_progress_pct() {
        let mut tracker = ProgressTracker::new(100);
        assert_eq!(tracker.progress_pct(), 0);
        for _ in 0..50 {
            tracker.mark_completed();
        }
        assert_eq!(tracker.progress_pct(), 50);
        for _ in 0..50 {
            tracker.mark_completed();
        }
        assert_eq!(tracker.progress_pct(), 100);
    }
    #[test]
    fn test_progress_tracker_format_progress() {
        let mut tracker = ProgressTracker::new(10);
        for _ in 0..5 {
            tracker.mark_completed();
        }
        for _ in 0..2 {
            tracker.mark_failed();
        }
        let formatted = tracker.format_progress();
        assert!(formatted.contains("5/10"));
        assert!(formatted.contains("50%"));
        assert!(formatted.contains("2 failed"));
    }
    #[test]
    fn test_dependency_analyzer_creation() {
        let analyzer = DependencyAnalyzer::new();
        assert!(analyzer.declarations.is_empty());
        assert!(analyzer.dependencies.is_empty());
    }
    #[test]
    fn test_dependency_analyzer_analyze_empty() {
        let mut analyzer = DependencyAnalyzer::new();
        assert!(analyzer.analyze_declarations(&[]).is_ok());
    }
    #[test]
    fn test_task_error_display() {
        let err = TaskError::TaskNotFound(TaskId::new(0));
        assert!(format!("{}", err).contains("not found"));
        let err = TaskError::CyclicDependency(vec![]);
        assert!(format!("{}", err).contains("cyclic"));
    }
    #[test]
    fn test_scheduler_summary_creation() {
        let summary = SchedulerSummary {
            total: 10,
            completed: 8,
            failed: 2,
            total_time_ms: 1000,
        };
        assert_eq!(summary.total, 10);
        assert_eq!(summary.completed, 8);
        assert_eq!(summary.failed, 2);
    }
    #[test]
    fn test_scheduler_summary_display() {
        let summary = SchedulerSummary {
            total: 10,
            completed: 8,
            failed: 2,
            total_time_ms: 1000,
        };
        let formatted = format!("{}", summary);
        assert!(formatted.contains("8/10"));
        assert!(formatted.contains("2 failed"));
    }
    #[test]
    fn test_execution_stats_from_empty_results() {
        let stats = ExecutionStats::from_results(&[]);
        assert!(stats.is_none());
    }
    #[test]
    fn test_execution_stats_from_results() {
        let results = vec![
            ElabResult::new(
                TaskId::new(0),
                Name::str("test1"),
                100,
                ElabOutput::new(Name::str("test1"), "out1".to_string()),
            ),
            ElabResult::new(
                TaskId::new(1),
                Name::str("test2"),
                200,
                ElabOutput::new(Name::str("test2"), "out2".to_string()),
            ),
        ];
        let stats = ExecutionStats::from_results(&results).expect("test operation should succeed");
        assert_eq!(stats.avg_time_ms, 150);
        assert_eq!(stats.min_time_ms, 100);
        assert_eq!(stats.max_time_ms, 200);
    }
    #[test]
    fn test_task_dependency_info_analyze() {
        let mut graph = TaskGraph::new();
        let id1 = graph.add_task("task1".to_string(), Name::str("decl1"));
        let id2 = graph.add_task("task2".to_string(), Name::str("decl2"));
        let id3 = graph.add_task("task3".to_string(), Name::str("decl3"));
        graph.add_dependency(id2, id1).ok();
        graph.add_dependency(id3, id2).ok();
        let info = TaskDependencyInfo::analyze(&graph, id2).expect("test operation should succeed");
        assert_eq!(info.task_id, id2);
        assert!(info.dependencies.contains(&id1));
        assert!(info.dependents.contains(&id3));
    }
    #[test]
    fn test_dependency_config_defaults() {
        let config = DependencyConfig::new();
        assert_eq!(config.max_depth, 100);
        assert!(config.transitive);
        assert!(config.detect_cycles);
    }
    #[test]
    fn test_batch_execution_result_success() {
        let result = ElabResult::new(
            TaskId::new(0),
            Name::str("test"),
            100,
            ElabOutput::new(Name::str("test"), "output".to_string()),
        );
        let batch = BatchExecutionResult {
            batch_id: 1,
            results: vec![result],
            errors: vec![],
            batch_time_ms: 100,
        };
        assert!(batch.is_successful());
        assert_eq!(batch.success_rate(), 1.0);
    }
    #[test]
    fn test_batch_execution_result_partial_failure() {
        let result = ElabResult::new(
            TaskId::new(0),
            Name::str("test"),
            100,
            ElabOutput::new(Name::str("test"), "output".to_string()),
        );
        let batch = BatchExecutionResult {
            batch_id: 1,
            results: vec![result],
            errors: vec![(TaskId::new(1), "error".to_string())],
            batch_time_ms: 100,
        };
        assert!(!batch.is_successful());
        assert!(batch.success_rate() < 1.0 && batch.success_rate() > 0.0);
    }
    #[test]
    fn test_batch_execution_result_display() {
        let batch = BatchExecutionResult {
            batch_id: 1,
            results: vec![],
            errors: vec![(TaskId::new(1), "error".to_string())],
            batch_time_ms: 100,
        };
        let formatted = format!("{}", batch);
        assert!(formatted.contains("Batch 1"));
        assert!(formatted.contains("1 errors"));
    }
}
#[cfg(test)]
mod parallel_ext_tests {
    use super::*;
    use crate::parallel::*;
    #[test]
    fn test_work_steal_deque_push_pop() {
        let mut d: WorkStealDeque<u32> = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        assert_eq!(d.len(), 2);
        assert_eq!(d.pop(), Some(2));
        assert_eq!(d.pop(), Some(1));
        assert!(d.is_empty());
    }
    #[test]
    fn test_work_steal_deque_steal() {
        let mut d: WorkStealDeque<u32> = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.push(3);
        assert_eq!(d.steal(), Some(1));
        assert_eq!(d.len(), 2);
    }
    #[test]
    fn test_parallel_task_queue_enqueue_dequeue() {
        let mut q = ParallelTaskQueue::new();
        q.enqueue(TaskId::new(1));
        q.enqueue(TaskId::new(2));
        assert_eq!(q.pending(), 2);
        assert_eq!(q.dequeue(), Some(TaskId::new(1)));
        assert_eq!(q.total_dequeued(), 1);
    }
    #[test]
    fn test_parallel_task_queue_throughput() {
        let mut q = ParallelTaskQueue::new();
        q.enqueue(TaskId::new(1));
        q.enqueue(TaskId::new(2));
        q.dequeue();
        assert!((q.throughput() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_parallel_elab_config_effective_threads() {
        let cfg = ParallelElabConfig::new();
        assert_eq!(cfg.effective_threads(), 4);
        let cfg2 = ParallelElabConfig::new().with_threads(8);
        assert_eq!(cfg2.effective_threads(), 8);
    }
    #[test]
    fn test_parallel_elab_config_builder() {
        let cfg = ParallelElabConfig::new()
            .with_threads(2)
            .no_work_stealing()
            .with_timing();
        assert_eq!(cfg.num_threads, 2);
        assert!(!cfg.work_stealing);
        assert!(cfg.collect_timing);
    }
    #[test]
    fn test_parallel_elab_stats_speedup() {
        let s = ParallelElabStats {
            tasks_executed: 100,
            tasks_stolen: 10,
            wall_time_ms: 50,
            cpu_time_ms: 200,
            batches: 5,
        };
        assert!((s.speedup() - 4.0).abs() < 1e-10);
        assert!((s.steal_rate() - 0.1).abs() < 1e-10);
    }
    #[test]
    fn test_parallel_elab_stats_merge() {
        let mut a = ParallelElabStats {
            tasks_executed: 10,
            ..Default::default()
        };
        let b = ParallelElabStats {
            tasks_executed: 20,
            batches: 3,
            ..Default::default()
        };
        a.merge(&b);
        assert_eq!(a.tasks_executed, 30);
        assert_eq!(a.batches, 3);
    }
    #[test]
    fn test_task_priority_order() {
        assert!(TaskPriority::Urgent > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert!(TaskPriority::Normal > TaskPriority::Low);
    }
    #[test]
    fn test_task_priority_display() {
        assert_eq!(format!("{}", TaskPriority::High), "high");
        assert_eq!(format!("{}", TaskPriority::Urgent), "urgent");
    }
    #[test]
    fn test_task_priority_queue_ordering() {
        let mut q = TaskPriorityQueue::new();
        q.enqueue(TaskId::new(1), TaskPriority::Low);
        q.enqueue(TaskId::new(2), TaskPriority::High);
        q.enqueue(TaskId::new(3), TaskPriority::Normal);
        let (id, prio) = q.dequeue().expect("test operation should succeed");
        assert_eq!(id, TaskId::new(2));
        assert_eq!(prio, TaskPriority::High);
        let (id2, prio2) = q.dequeue().expect("test operation should succeed");
        assert_eq!(id2, TaskId::new(3));
        assert_eq!(prio2, TaskPriority::Normal);
        let (id3, prio3) = q.dequeue().expect("test operation should succeed");
        assert_eq!(id3, TaskId::new(1));
        assert_eq!(prio3, TaskPriority::Low);
    }
    #[test]
    fn test_task_priority_queue_empty() {
        let mut q = TaskPriorityQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.dequeue(), None);
    }
    #[test]
    fn test_prioritized_task_constructors() {
        let t = PrioritizedTask::urgent(TaskId::new(99));
        assert_eq!(t.priority, TaskPriority::Urgent);
        let t2 = PrioritizedTask::normal(TaskId::new(1));
        assert_eq!(t2.priority, TaskPriority::Normal);
    }
}
#[cfg(test)]
mod parallel_batch_tests {
    use super::*;
    use crate::parallel::*;
    #[test]
    fn test_batch_result_counts() {
        let mut r = ParallelBatchResult::new();
        r.push(ParallelTaskOutcome::Success {
            task_id: TaskId::new(1),
            duration_ns: 1000,
        });
        r.push(ParallelTaskOutcome::Failure {
            task_id: TaskId::new(2),
            reason: "oops".to_string(),
        });
        r.push(ParallelTaskOutcome::Skipped {
            task_id: TaskId::new(3),
        });
        assert_eq!(r.success_count(), 1);
        assert_eq!(r.failure_count(), 1);
        assert_eq!(r.skipped_count(), 1);
    }
    #[test]
    fn test_batch_result_success_rate() {
        let mut r = ParallelBatchResult::new();
        r.push(ParallelTaskOutcome::Success {
            task_id: TaskId::new(1),
            duration_ns: 500,
        });
        r.push(ParallelTaskOutcome::Success {
            task_id: TaskId::new(2),
            duration_ns: 1500,
        });
        r.push(ParallelTaskOutcome::Failure {
            task_id: TaskId::new(3),
            reason: "err".to_string(),
        });
        assert!((r.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_batch_result_avg_ns() {
        let mut r = ParallelBatchResult::new();
        r.push(ParallelTaskOutcome::Success {
            task_id: TaskId::new(1),
            duration_ns: 1000,
        });
        r.push(ParallelTaskOutcome::Success {
            task_id: TaskId::new(2),
            duration_ns: 3000,
        });
        assert!((r.avg_success_ns() - 2000.0).abs() < 1e-6);
    }
    #[test]
    fn test_batch_result_empty() {
        let r = ParallelBatchResult::new();
        assert_eq!(r.success_count(), 0);
        assert!((r.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_task_outcome_task_id() {
        let o = ParallelTaskOutcome::Skipped {
            task_id: TaskId::new(42),
        };
        assert_eq!(o.task_id(), TaskId::new(42));
    }
}
