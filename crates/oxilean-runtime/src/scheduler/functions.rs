//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;

use super::types::{
    ActorId, ActorMailbox, ActorMessage, BackpressureController, ExtSchedulerStats,
    LoadBalanceStrategy, LoadBalancer, ParallelEval, PreemptionSimulator, PriorityTaskQueue,
    RoundRobinToken, Scheduler, SchedulerConfig, SchedulerTestHarness, SharedState, Task,
    TaskAffinity, TaskId, TaskPriority, TaskProfile, TaskState, WorkStealingDeque, Worker,
    YieldPoint,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_id() {
        let id = TaskId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(format!("{}", id), "task#42");
    }
    #[test]
    fn test_task_priority() {
        assert!(TaskPriority::Critical > TaskPriority::Low);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert_eq!(TaskPriority::from_u8(99), TaskPriority::Critical);
    }
    #[test]
    fn test_task_state() {
        let state = TaskState::Created;
        assert!(state.is_runnable());
        assert!(!state.is_terminal());
        let state = TaskState::Completed {
            result: RtObject::nat(42),
        };
        assert!(state.is_terminal());
        assert!(!state.is_runnable());
    }
    #[test]
    fn test_work_stealing_deque() {
        let mut deque = WorkStealingDeque::new(10);
        assert!(deque.is_empty());
        deque.push(TaskId::new(1));
        deque.push(TaskId::new(2));
        deque.push(TaskId::new(3));
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.pop(), Some(TaskId::new(3)));
        assert_eq!(deque.steal(), Some(TaskId::new(1)));
        assert_eq!(deque.len(), 1);
    }
    #[test]
    fn test_deque_batch_steal() {
        let mut deque = WorkStealingDeque::new(10);
        for i in 0..6 {
            deque.push(TaskId::new(i));
        }
        let stolen = deque.steal_batch(3);
        assert_eq!(stolen.len(), 3);
        assert_eq!(deque.len(), 3);
    }
    #[test]
    fn test_worker() {
        let mut worker = Worker::new(0, 100);
        assert!(worker.idle);
        assert_eq!(worker.load(), 0);
        worker.push_task(TaskId::new(1));
        assert_eq!(worker.load(), 1);
        worker.start_task(TaskId::new(1));
        assert!(!worker.idle);
        assert_eq!(worker.current_task, Some(TaskId::new(1)));
        worker.finish_task();
        assert!(worker.idle);
        assert_eq!(worker.tasks_completed, 1);
    }
    #[test]
    fn test_scheduler_spawn() {
        let mut scheduler = Scheduler::single_threaded();
        let id1 = scheduler.spawn(RtObject::nat(1));
        let id2 = scheduler.spawn(RtObject::nat(2));
        assert_eq!(scheduler.active_task_count(), 2);
        assert_ne!(id1, id2);
    }
    #[test]
    fn test_scheduler_schedule_and_complete() {
        let mut scheduler = Scheduler::single_threaded();
        let id = scheduler.spawn(RtObject::nat(42));
        let (worker_id, task_id) = scheduler
            .schedule_step()
            .expect("scheduling should succeed");
        assert_eq!(task_id, id);
        assert_eq!(worker_id, 0);
        scheduler.complete_task(id, RtObject::nat(100));
        assert!(scheduler.is_complete(id));
        assert_eq!(
            scheduler
                .get_result(id)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(100)
        );
    }
    #[test]
    fn test_scheduler_dependencies() {
        let mut scheduler = Scheduler::single_threaded();
        let id1 = scheduler.spawn(RtObject::nat(1));
        let id2 = scheduler.spawn_with_deps(RtObject::nat(2), vec![id1]);
        assert!(scheduler
            .get_task(id2)
            .expect("scheduling should succeed")
            .state
            .is_suspended());
        let (_w, t) = scheduler
            .schedule_step()
            .expect("scheduling should succeed");
        assert_eq!(t, id1);
        scheduler.complete_task(id1, RtObject::nat(10));
        assert_eq!(
            scheduler
                .get_task(id2)
                .expect("scheduling should succeed")
                .state,
            TaskState::Queued
        );
        let (_w, t) = scheduler
            .schedule_step()
            .expect("scheduling should succeed");
        assert_eq!(t, id2);
    }
    #[test]
    fn test_scheduler_cancel() {
        let mut scheduler = Scheduler::single_threaded();
        let id = scheduler.spawn(RtObject::nat(1));
        assert!(scheduler.cancel(id));
        assert!(scheduler.is_complete(id));
    }
    #[test]
    fn test_scheduler_run_all() {
        let mut scheduler = Scheduler::single_threaded();
        scheduler.spawn(RtObject::nat(1));
        scheduler.spawn(RtObject::nat(2));
        scheduler.spawn(RtObject::nat(3));
        scheduler.run_all(|task| Ok(task.action.clone()));
        assert_eq!(scheduler.completed_count(), 3);
        assert_eq!(scheduler.stats().tasks_completed, 3);
    }
    #[test]
    fn test_scheduler_fail() {
        let mut scheduler = Scheduler::single_threaded();
        let id = scheduler.spawn(RtObject::nat(1));
        scheduler.run_all(|_task| Err("error".to_string()));
        assert!(scheduler.is_complete(id));
        assert_eq!(
            scheduler
                .get_task(id)
                .expect("scheduling should succeed")
                .error(),
            Some("error")
        );
        assert_eq!(scheduler.stats().tasks_failed, 1);
    }
    #[test]
    fn test_scheduler_config() {
        let config = SchedulerConfig::new()
            .with_workers(8)
            .with_deque_capacity(2048)
            .with_max_tasks(50_000);
        assert_eq!(config.num_workers, 8);
        assert_eq!(config.deque_capacity, 2048);
        assert_eq!(config.max_tasks, 50_000);
    }
    #[test]
    fn test_parallel_eval() {
        let mut scheduler = Scheduler::single_threaded();
        let ids = ParallelEval::par_map(
            &mut scheduler,
            vec![RtObject::nat(1), RtObject::nat(2), RtObject::nat(3)],
        );
        assert_eq!(ids.len(), 3);
        assert_eq!(scheduler.active_task_count(), 3);
    }
    #[test]
    fn test_parallel_pair() {
        let mut scheduler = Scheduler::single_threaded();
        let (a, b) = ParallelEval::par_pair(&mut scheduler, RtObject::nat(1), RtObject::nat(2));
        assert_ne!(a, b);
        assert_eq!(scheduler.active_task_count(), 2);
    }
    #[test]
    fn test_parallel_barrier() {
        let mut scheduler = Scheduler::single_threaded();
        let (dep_ids, barrier_id) = ParallelEval::barrier(
            &mut scheduler,
            vec![RtObject::nat(1), RtObject::nat(2)],
            RtObject::nat(3),
        );
        assert_eq!(dep_ids.len(), 2);
        assert!(scheduler
            .get_task(barrier_id)
            .expect("scheduling should succeed")
            .state
            .is_suspended());
    }
    #[test]
    fn test_load_balancer_round_robin() {
        let mut lb = LoadBalancer::new(LoadBalanceStrategy::RoundRobin, 4);
        let loads = vec![0, 0, 0, 0];
        assert_eq!(lb.select_worker(&loads), 0);
        assert_eq!(lb.select_worker(&loads), 1);
        assert_eq!(lb.select_worker(&loads), 2);
        assert_eq!(lb.select_worker(&loads), 3);
        assert_eq!(lb.select_worker(&loads), 0);
    }
    #[test]
    fn test_load_balancer_least_loaded() {
        let mut lb = LoadBalancer::new(LoadBalanceStrategy::LeastLoaded, 4);
        let loads = vec![5, 2, 8, 1];
        assert_eq!(lb.select_worker(&loads), 3);
    }
    #[test]
    fn test_shared_state() {
        let state = SharedState::new();
        assert!(!state.should_shutdown());
        let id = state.next_task_id();
        assert_eq!(id, TaskId::new(0));
        state.push_task(id);
        assert_eq!(state.pop_task(), Some(id));
        assert_eq!(state.pop_task(), None);
        state.store_result(id, RtObject::nat(42));
        assert_eq!(
            state
                .get_result(id)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(42)
        );
        state.request_shutdown();
        assert!(state.should_shutdown());
    }
    #[test]
    fn test_scheduler_reset() {
        let mut scheduler = Scheduler::single_threaded();
        scheduler.spawn(RtObject::nat(1));
        scheduler.spawn(RtObject::nat(2));
        scheduler.run_all(|task| Ok(task.action.clone()));
        scheduler.reset();
        assert_eq!(scheduler.active_task_count(), 0);
        assert_eq!(scheduler.completed_count(), 0);
        assert_eq!(scheduler.stats().tasks_created, 0);
    }
    #[test]
    fn test_scheduler_multi_worker() {
        let config = SchedulerConfig::new().with_workers(4);
        let mut scheduler = Scheduler::new(config);
        for i in 0..10 {
            scheduler.spawn(RtObject::nat(i));
        }
        scheduler.run_all(|task| Ok(task.action.clone()));
        assert_eq!(scheduler.completed_count(), 10);
    }
    #[test]
    fn test_task_display() {
        let task = Task::named(TaskId::new(1), "test_task".to_string(), RtObject::nat(42));
        let s = format!("{}", task);
        assert!(s.contains("test_task"));
        assert!(s.contains("task#1"));
    }
}
#[cfg(test)]
mod scheduler_ext_tests {
    use super::*;
    #[test]
    fn test_task_priority_ordering() {
        assert!(TaskPriority::Critical > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert!(TaskPriority::Normal > TaskPriority::Low);
        assert!(TaskPriority::Low > TaskPriority::Background);
    }
    #[test]
    fn test_task_priority_from_u8() {
        assert_eq!(TaskPriority::from_u8(0), TaskPriority::Background);
        assert_eq!(TaskPriority::from_u8(4), TaskPriority::Critical);
        assert_eq!(TaskPriority::from_u8(99), TaskPriority::Critical);
    }
    #[test]
    fn test_priority_queue_pop_order() {
        let mut q = PriorityTaskQueue::new();
        q.push(TaskId::new(1), TaskPriority::Low);
        q.push(TaskId::new(2), TaskPriority::Critical);
        q.push(TaskId::new(3), TaskPriority::Normal);
        let (id, prio) = q.pop().expect("collection should not be empty");
        assert_eq!(id, TaskId::new(2));
        assert_eq!(prio, TaskPriority::Critical);
    }
    #[test]
    fn test_priority_queue_empty() {
        let mut q = PriorityTaskQueue::new();
        assert!(q.pop().is_none());
        assert!(q.is_empty());
    }
    #[test]
    fn test_priority_queue_count_at() {
        let mut q = PriorityTaskQueue::new();
        q.push(TaskId::new(1), TaskPriority::Normal);
        q.push(TaskId::new(2), TaskPriority::Normal);
        q.push(TaskId::new(3), TaskPriority::High);
        assert_eq!(q.count_at(TaskPriority::Normal), 2);
        assert_eq!(q.count_at(TaskPriority::High), 1);
        assert_eq!(q.len(), 3);
    }
    #[test]
    fn test_task_affinity_allows() {
        assert!(TaskAffinity::Any.allows(0));
        assert!(TaskAffinity::Any.allows(99));
        assert!(TaskAffinity::Worker(2).allows(2));
        assert!(!TaskAffinity::Worker(2).allows(3));
        assert!(TaskAffinity::MainThread.allows(0));
        assert!(!TaskAffinity::MainThread.allows(1));
    }
    #[test]
    fn test_task_affinity_steal() {
        assert!(TaskAffinity::Any.allows_steal());
        assert!(!TaskAffinity::Worker(0).allows_steal());
        assert!(TaskAffinity::Prefer(0).allows_steal());
    }
    #[test]
    fn test_yield_point() {
        let mut yp = YieldPoint::new();
        assert!(!yp.should_yield());
        let handle = yp.handle();
        handle.request();
        assert!(handle.is_pending());
        assert!(yp.should_yield());
        assert_eq!(yp.yield_count, 1);
    }
    #[test]
    fn test_ext_scheduler_stats() {
        let mut stats = ExtSchedulerStats::new();
        stats.record_created();
        stats.record_created();
        stats.record_completed(100);
        stats.record_completed(200);
        stats.record_cancelled();
        stats.record_steal();
        assert_eq!(stats.tasks_created, 2);
        assert_eq!(stats.tasks_completed, 2);
        assert_eq!(stats.tasks_cancelled, 1);
        assert_eq!(stats.tasks_stolen, 1);
        assert_eq!(stats.avg_latency(), 150.0);
        assert_eq!(stats.max_latency_ticks, 200);
    }
    #[test]
    fn test_ext_stats_utilization() {
        let mut stats = ExtSchedulerStats::new();
        for _ in 0..75 {
            stats.record_sample(true);
        }
        for _ in 0..25 {
            stats.record_sample(false);
        }
        assert!((stats.utilization() - 0.75).abs() < 0.01);
    }
    #[test]
    fn test_task_profile() {
        let mut profile = TaskProfile::new(TaskId::new(42), 1000);
        profile.start(1100);
        profile.complete(1500, 2);
        assert_eq!(profile.queue_latency(), Some(100));
        assert_eq!(profile.execution_time(), Some(400));
        assert_eq!(profile.total_latency(), Some(500));
        assert_eq!(profile.completed_by, Some(2));
    }
    #[test]
    fn test_backpressure_basic() {
        let mut bp = BackpressureController::new(5, 2);
        for _ in 0..4 {
            bp.enqueue();
        }
        assert!(!bp.is_throttled());
        bp.enqueue();
        assert!(bp.is_throttled());
        assert_eq!(bp.throttle_events, 1);
        for _ in 0..3 {
            bp.dequeue();
        }
        bp.dequeue();
        assert!(!bp.is_throttled());
    }
    #[test]
    fn test_backpressure_fill_ratio() {
        let mut bp = BackpressureController::new(10, 3);
        for _ in 0..5 {
            bp.enqueue();
        }
        assert!((bp.fill_ratio() - 0.5).abs() < 0.01);
    }
    #[test]
    fn test_actor_mailbox() {
        let actor_a = ActorId::new(1);
        let actor_b = ActorId::new(2);
        let mut mailbox = ActorMailbox::new(actor_b);
        let msg = ActorMessage::new(actor_a, actor_b, RtObject::nat(42), 0);
        mailbox.send(msg);
        assert_eq!(mailbox.pending(), 1);
        let received = mailbox.receive().expect("test operation should succeed");
        assert_eq!(received.seq, 0);
        assert!(mailbox.is_empty());
        assert_eq!(mailbox.total_received, 1);
        assert_eq!(mailbox.total_processed, 1);
    }
    #[test]
    fn test_scheduler_test_harness() {
        let mut harness = SchedulerTestHarness::new();
        let id1 = harness.submit(RtObject::nat(10));
        let id2 = harness.submit(RtObject::nat(20));
        harness.run_all(|obj| obj.clone());
        assert_eq!(harness.completed(), 2);
        assert_eq!(harness.execution_order, vec![id1, id2]);
        assert_eq!(
            harness
                .get_result(id1)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(10)
        );
        assert_eq!(
            harness
                .get_result(id2)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(20)
        );
    }
    #[test]
    fn test_preemption_simulator() {
        let mut sim = PreemptionSimulator::new(3);
        sim.set_active(TaskId::new(1));
        assert!(!sim.tick());
        assert!(!sim.tick());
        assert!(sim.tick());
        assert_eq!(sim.preemptions, 1);
        assert!(sim.active_task.is_none());
    }
    #[test]
    fn test_priority_queue_clear() {
        let mut q = PriorityTaskQueue::new();
        q.push(TaskId::new(1), TaskPriority::High);
        q.push(TaskId::new(2), TaskPriority::Low);
        q.clear();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }
    #[test]
    fn test_ext_stats_display() {
        let stats = ExtSchedulerStats::new();
        let s = format!("{}", stats);
        assert!(s.contains("ExtSchedulerStats"));
    }
    #[test]
    fn test_task_priority_display() {
        assert_eq!(format!("{}", TaskPriority::High), "high");
        assert_eq!(format!("{}", TaskPriority::Background), "background");
    }
    #[test]
    fn test_actor_id_display() {
        let id = ActorId::new(7);
        assert_eq!(format!("{}", id), "actor#7");
    }
}
#[cfg(test)]
mod extra_sched_tests {
    use super::*;
    #[test]
    fn test_round_robin_wraps() {
        let mut rr = RoundRobinToken::new(3);
        assert_eq!(rr.next(), 0);
        assert_eq!(rr.next(), 1);
        assert_eq!(rr.next(), 2);
        assert_eq!(rr.next(), 0);
    }
    #[test]
    fn test_round_robin_peek() {
        let mut rr = RoundRobinToken::new(2);
        assert_eq!(rr.peek(), 0);
        rr.next();
        assert_eq!(rr.peek(), 1);
    }
    #[test]
    fn test_round_robin_reset() {
        let mut rr = RoundRobinToken::new(4);
        rr.next();
        rr.next();
        rr.reset();
        assert_eq!(rr.peek(), 0);
    }
}
