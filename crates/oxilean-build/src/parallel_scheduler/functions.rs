//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CriticalPathScheduler, ParallelScheduler, Priority, WorkItem, WorkStealingQueue, Worker, WorkerState};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_work_item_creation() {
        let mut item = WorkItem::new(1, "compile_foo");
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "compile_foo");
        assert_eq!(item.priority, Priority::Normal);
        assert_eq!(item.estimated_cost, 1);
        assert!(item.deps.is_empty());
        item.add_dep(10);
        item.add_dep(20);
        item.add_dep(10);
        assert_eq!(item.deps.len(), 2);
    }
    #[test]
    fn test_work_item_builder() {
        let item = WorkItem::new(2, "link")
            .with_priority(Priority::Critical)
            .with_cost(500);
        assert_eq!(item.priority, Priority::Critical);
        assert_eq!(item.estimated_cost, 500);
    }
    #[test]
    fn test_worker_state() {
        let mut w = Worker::new(0);
        assert!(w.is_idle());
        assert_eq!(w.state, WorkerState::Idle);
        w.assign(42);
        assert!(! w.is_idle());
        assert_eq!(w.state, WorkerState::Busy(42));
        w.complete(100);
        assert!(w.is_idle());
        assert_eq!(w.completed, 1);
        assert_eq!(w.total_cost, 100);
    }
    #[test]
    fn test_parallel_scheduler_new() {
        let sched = ParallelScheduler::new(4);
        assert_eq!(sched.workers.len(), 4);
        assert!(sched.is_complete());
        let stats = sched.stats();
        assert_eq!(stats.total_items, 0);
        assert_eq!(stats.completed, 0);
    }
    #[test]
    fn test_submit_and_schedule() {
        let mut sched = ParallelScheduler::new(2);
        sched.submit(WorkItem::new(1, "a").with_priority(Priority::High));
        sched.submit(WorkItem::new(2, "b").with_priority(Priority::Low));
        let first = sched.schedule_next();
        assert!(first.is_some());
        let (wid, iid) = first.expect("test operation should succeed");
        assert_eq!(iid, 1);
        sched.complete_item(wid, iid);
        let second = sched.schedule_next();
        assert!(second.is_some());
        let (wid2, iid2) = second.expect("test operation should succeed");
        assert_eq!(iid2, 2);
        sched.complete_item(wid2, iid2);
        assert!(sched.is_complete());
    }
    #[test]
    fn test_ready_items() {
        let mut sched = ParallelScheduler::new(1);
        let mut item_b = WorkItem::new(2, "b");
        item_b.add_dep(1);
        sched.submit(WorkItem::new(1, "a"));
        sched.submit(item_b);
        let ready = sched.ready_items();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, 1);
        let (wid, iid) = sched.schedule_next().expect("test operation should succeed");
        sched.complete_item(wid, iid);
        let ready2 = sched.ready_items();
        assert_eq!(ready2.len(), 1);
        assert_eq!(ready2[0].id, 2);
    }
    #[test]
    fn test_work_stealing_queue() {
        let mut q = WorkStealingQueue::new();
        q.push(WorkItem::new(1, "x"));
        q.push(WorkItem::new(2, "y"));
        q.push(WorkItem::new(3, "z"));
        assert_eq!(q.len(), 3);
        let front = q.pop().expect("collection should not be empty");
        assert_eq!(front.id, 1);
        let stolen = q.steal().expect("test operation should succeed");
        assert_eq!(stolen.id, 3);
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_critical_path() {
        let sched = CriticalPathScheduler::new();
        let items = vec![
            WorkItem::new(1, "a").with_cost(10), { let mut i = WorkItem::new(2, "b")
            .with_cost(10); i.add_dep(1); i }, { let mut i = WorkItem::new(3, "c")
            .with_cost(10); i.add_dep(2); i },
        ];
        let path = sched.compute_critical_path(&items);
        assert_eq!(path, vec![1, 2, 3]);
    }
    #[test]
    fn test_scheduler_stats() {
        let mut sched = ParallelScheduler::new(2);
        sched.submit(WorkItem::new(1, "a"));
        sched.submit(WorkItem::new(2, "b"));
        let (w1, i1) = sched.schedule_next().expect("test operation should succeed");
        let (w2, i2) = sched.schedule_next().expect("test operation should succeed");
        let stats = sched.stats();
        assert_eq!(stats.total_items, 2);
        assert!((stats.parallelism_ratio - 1.0).abs() < 1e-9);
        sched.complete_item(w1, i1);
        sched.complete_item(w2, i2);
        let stats2 = sched.stats();
        assert_eq!(stats2.completed, 2);
    }
}
