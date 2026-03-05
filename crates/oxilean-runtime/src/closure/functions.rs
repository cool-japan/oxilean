//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;
use std::collections::HashMap;

use super::types::{
    BatchForceResult, CallConvention, CallInliner, CallStack, CaptureSet, Closure, ClosureArena,
    ClosureBuilder, ClosureOptReport, ClosureOptimizer, ClosureRegistry, ClosureSerializer,
    ClosureSizeEstimator, ClosureStats, EnvGraph, FlatClosure, FnPtr, FunctionEntry, FunctionTable,
    InlineCandidate, LambdaLifter, MutualRecGroup, Pap, PapQueue, PapResult, StackFrame,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fn_ptr() {
        let ptr = FnPtr::new(42);
        assert_eq!(ptr.index, 42);
        assert_eq!(ptr.module_id, 0);
        assert!(!ptr.is_null());
        let null = FnPtr::null();
        assert!(null.is_null());
    }
    #[test]
    fn test_closure_basic() {
        let closure = Closure::simple(FnPtr::new(0), 2);
        assert_eq!(closure.arity, 2);
        assert_eq!(closure.env_size(), 0);
    }
    #[test]
    fn test_closure_with_env() {
        let env = vec![RtObject::nat(42), RtObject::bool_val(true)];
        let closure = Closure::new(FnPtr::new(0), 3, env);
        assert_eq!(closure.env_size(), 2);
        assert_eq!(
            closure
                .get_env(0)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(42)
        );
    }
    #[test]
    fn test_closure_builder() {
        let closure = ClosureBuilder::new(FnPtr::new(5), 2)
            .name("my_func".to_string())
            .capture(RtObject::nat(10))
            .capture(RtObject::nat(20))
            .recursive()
            .build();
        assert_eq!(closure.arity, 2);
        assert_eq!(closure.env_size(), 2);
        assert!(closure.is_recursive);
        assert_eq!(closure.name, Some("my_func".to_string()));
    }
    #[test]
    fn test_pap_under_applied() {
        let closure = Closure::simple(FnPtr::new(0), 3);
        let pap = Pap::new(closure, vec![RtObject::nat(1)]);
        assert_eq!(pap.remaining_arity(), 2);
        assert_eq!(pap.num_applied(), 1);
        assert!(!pap.is_saturated());
    }
    #[test]
    fn test_pap_apply() {
        let closure = Closure::simple(FnPtr::new(0), 3);
        let pap = Pap::new(closure, vec![RtObject::nat(1)]);
        match pap.apply(&[RtObject::nat(2)]) {
            PapResult::UnderApplied(new_pap) => {
                assert_eq!(new_pap.remaining_arity(), 1);
                assert_eq!(new_pap.num_applied(), 2);
            }
            _ => panic!("Expected UnderApplied"),
        }
        match pap.apply(&[RtObject::nat(2), RtObject::nat(3)]) {
            PapResult::Saturated { args, .. } => {
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected Saturated"),
        }
        match pap.apply(&[RtObject::nat(2), RtObject::nat(3), RtObject::nat(4)]) {
            PapResult::OverApplied {
                args,
                remaining_args,
                ..
            } => {
                assert_eq!(args.len(), 3);
                assert_eq!(remaining_args.len(), 1);
            }
            _ => panic!("Expected OverApplied"),
        }
    }
    #[test]
    fn test_function_table() {
        let mut table = FunctionTable::new();
        let ptr = table.register(FunctionEntry::new("foo".to_string(), 2));
        assert_eq!(ptr.index, 0);
        let entry = table.get(ptr).expect("key should exist in map");
        assert_eq!(entry.name, "foo");
        assert_eq!(entry.arity, 2);
        let (found_ptr, found_entry) = table
            .get_by_name("foo")
            .expect("test operation should succeed");
        assert_eq!(found_ptr, ptr);
        assert_eq!(found_entry.name, "foo");
    }
    #[test]
    fn test_function_table_builtins() {
        let mut table = FunctionTable::new();
        table.register_builtins();
        assert!(table.get_by_name("Nat.add").is_some());
        assert!(table.get_by_name("IO.println").is_some());
    }
    #[test]
    fn test_call_stack() {
        let mut stack = CallStack::new();
        assert!(stack.is_empty());
        let frame = StackFrame::new(FnPtr::new(0), vec![RtObject::nat(1)], Vec::new(), 4);
        stack.push(frame).expect("test operation should succeed");
        assert_eq!(stack.depth(), 1);
        let frame = stack.current().expect("test operation should succeed");
        assert_eq!(frame.args.len(), 1);
        stack.pop();
        assert!(stack.is_empty());
    }
    #[test]
    fn test_call_stack_overflow() {
        let mut stack = CallStack::with_max_depth(2);
        stack
            .push(StackFrame::new(FnPtr::new(0), Vec::new(), Vec::new(), 0))
            .expect("test operation should succeed");
        stack
            .push(StackFrame::new(FnPtr::new(1), Vec::new(), Vec::new(), 0))
            .expect("test operation should succeed");
        let result = stack.push(StackFrame::new(FnPtr::new(2), Vec::new(), Vec::new(), 0));
        assert!(result.is_err());
    }
    #[test]
    fn test_stack_frame_locals() {
        let mut frame = StackFrame::new(
            FnPtr::new(0),
            vec![RtObject::nat(10)],
            vec![RtObject::nat(20)],
            3,
        );
        assert!(frame.set_local(0, RtObject::nat(42)));
        assert_eq!(
            frame
                .get_local(0)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(42)
        );
        assert_eq!(
            frame
                .get_arg(0)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(10)
        );
        assert_eq!(
            frame
                .get_env(0)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(20)
        );
    }
    #[test]
    fn test_call_convention() {
        assert!(CallConvention::TailCall.supports_tco());
        assert!(CallConvention::DirectCall.supports_tco());
        assert!(!CallConvention::ClosureCall.supports_tco());
        assert!(CallConvention::DirectCall.is_direct());
    }
    #[test]
    fn test_closure_stats() {
        let mut stats = ClosureStats::new();
        stats.record_exact_call();
        stats.record_exact_call();
        stats.record_tail_call();
        stats.record_pap_created();
        assert_eq!(stats.total_calls(), 3);
        assert_eq!(stats.paps_created, 1);
    }
    #[test]
    fn test_mutual_rec_group() {
        let c1 = Closure::named("even".to_string(), FnPtr::new(0), 1, Vec::new());
        let c2 = Closure::named("odd".to_string(), FnPtr::new(1), 1, Vec::new());
        let group = MutualRecGroup::new(vec![c1, c2], Vec::new());
        assert_eq!(group.size(), 2);
        assert_eq!(
            group
                .get_closure(0)
                .expect("string conversion should succeed")
                .name,
            Some("even".to_string())
        );
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_closure_arena_alloc_get() {
        let mut arena = ClosureArena::new();
        let c = Closure::new(FnPtr::new(0), 2, Vec::new());
        let h = arena.alloc(c);
        assert_eq!(arena.get(h).map(|c| c.arity), Some(2));
    }
    #[test]
    fn test_closure_arena_free_reuse() {
        let mut arena = ClosureArena::new();
        let h1 = arena.alloc(Closure::new(FnPtr::new(0), 1, Vec::new()));
        arena.free(h1);
        let h2 = arena.alloc(Closure::new(FnPtr::new(1), 2, Vec::new()));
        assert_eq!(h1, h2);
        assert_eq!(arena.live_count(), 1);
    }
    #[test]
    fn test_closure_arena_stats() {
        let mut arena = ClosureArena::new();
        let h = arena.alloc(Closure::new(FnPtr::new(0), 1, Vec::new()));
        arena.free(h);
        assert_eq!(arena.alloc_count(), 1);
        assert_eq!(arena.free_count(), 1);
    }
    #[test]
    fn test_closure_arena_iter() {
        let mut arena = ClosureArena::new();
        arena.alloc(Closure::new(FnPtr::new(0), 1, Vec::new()));
        arena.alloc(Closure::new(FnPtr::new(1), 2, Vec::new()));
        let arities: Vec<u16> = arena.iter().map(|(_, c)| c.arity).collect();
        assert_eq!(arities.len(), 2);
    }
    #[test]
    fn test_lambda_lifter_basic() {
        let mut lifter = LambdaLifter::new();
        let l = lifter.lift(vec!["x".to_string(), "y".to_string()], 2);
        assert_eq!(l.id, 0);
        assert_eq!(l.free_vars.len(), 2);
        assert_eq!(lifter.len(), 1);
    }
    #[test]
    fn test_lambda_lifter_ids() {
        let mut lifter = LambdaLifter::new();
        let l1 = lifter.lift(vec![], 0);
        let l2 = lifter.lift(vec![], 0);
        assert_ne!(l1.id, l2.id);
    }
    #[test]
    fn test_lambda_lifter_total_free_vars() {
        let mut lifter = LambdaLifter::new();
        lifter.lift(vec!["a".to_string()], 1);
        lifter.lift(vec!["b".to_string(), "c".to_string()], 1);
        assert_eq!(lifter.total_free_vars(), 3);
    }
    #[test]
    fn test_opt_report_has_changes() {
        let mut report = ClosureOptReport::default();
        assert!(!report.has_changes());
        report.inlined_paps = 1;
        assert!(report.has_changes());
    }
    #[test]
    fn test_opt_report_total() {
        let report = ClosureOptReport {
            inlined_paps: 2,
            specialized_arities: 3,
            env_reduced_vars: 1,
            removed_dead_closures: 0,
        };
        assert_eq!(report.total(), 6);
    }
    #[test]
    fn test_opt_report_display() {
        let report = ClosureOptReport {
            inlined_paps: 1,
            ..Default::default()
        };
        let s = format!("{}", report);
        assert!(s.contains("inlined_paps: 1"));
    }
    #[test]
    fn test_optimizer_saturated_pap() {
        let opt = ClosureOptimizer::new(8);
        let c = Closure::new(FnPtr::new(0), 2, Vec::new());
        let pap = Pap::new(c, vec![RtObject::nat(1), RtObject::nat(2)]);
        let result = opt.optimize_pap(&pap);
        assert!(result.is_some());
        let saturated = result.expect("test operation should succeed");
        assert_eq!(saturated.arity, 0);
        assert_eq!(saturated.env.len(), 2);
    }
    #[test]
    fn test_optimizer_partial_pap() {
        let opt = ClosureOptimizer::new(8);
        let c = Closure::new(FnPtr::new(0), 3, Vec::new());
        let pap = Pap::new(c, vec![RtObject::nat(1)]);
        assert!(opt.optimize_pap(&pap).is_none());
    }
    #[test]
    fn test_env_graph_basic() {
        let mut g = EnvGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let trans = g.transitive_captures(a);
        assert!(trans.contains(&b));
    }
    #[test]
    fn test_env_graph_no_cycle() {
        let mut g = EnvGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        assert!(!g.has_cycle());
    }
    #[test]
    fn test_env_graph_cycle() {
        let mut g = EnvGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        assert!(g.has_cycle());
    }
    #[test]
    fn test_env_graph_counts() {
        let mut g = EnvGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let _c = g.add_node();
        g.add_edge(a, b);
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 1);
    }
    #[test]
    fn test_call_inliner_register_top() {
        let mut inliner = CallInliner::new();
        inliner.register(InlineCandidate {
            fn_id: 1,
            call_site_count: 10,
            env_size: 2,
            arity: 1,
        });
        inliner.register(InlineCandidate {
            fn_id: 2,
            call_site_count: 2,
            env_size: 8,
            arity: 3,
        });
        let top = inliner.top_candidates(1);
        assert_eq!(top.len(), 1);
    }
    #[test]
    fn test_call_inliner_record_inline() {
        let mut inliner = CallInliner::new();
        inliner.register(InlineCandidate {
            fn_id: 5,
            call_site_count: 1,
            env_size: 0,
            arity: 1,
        });
        inliner.record_inline(5);
        assert_eq!(inliner.inlined_count(), 1);
        assert_eq!(inliner.candidate_count(), 0);
    }
    #[test]
    fn test_inline_candidate_score() {
        let c = InlineCandidate {
            fn_id: 0,
            call_site_count: 3,
            env_size: 4,
            arity: 2,
        };
        assert!(c.inline_score() > 0.0);
    }
    #[test]
    fn test_inline_candidate_is_leaf() {
        let c = InlineCandidate {
            fn_id: 0,
            call_site_count: 1,
            env_size: 2,
            arity: 2,
        };
        assert!(c.is_leaf_candidate());
        let c2 = InlineCandidate {
            fn_id: 1,
            call_site_count: 1,
            env_size: 10,
            arity: 2,
        };
        assert!(!c2.is_leaf_candidate());
    }
}
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_flat_closure_serialize_roundtrip() {
        let c = FlatClosure::new(7, 2, vec![10, 20, 30]);
        let bytes = c.serialize();
        let c2 = FlatClosure::deserialize(&bytes).expect("test operation should succeed");
        assert_eq!(c2.fn_index, 7);
        assert_eq!(c2.arity, 2);
        assert_eq!(c2.env, vec![10, 20, 30]);
    }
    #[test]
    fn test_flat_closure_is_thunk() {
        let c = FlatClosure::new(0, 0, vec![]);
        assert!(c.is_thunk());
        let c2 = FlatClosure::new(0, 1, vec![]);
        assert!(!c2.is_thunk());
    }
    #[test]
    fn test_flat_closure_display() {
        let c = FlatClosure::new(3, 2, vec![1, 2]);
        let s = format!("{}", c);
        assert!(s.contains("fn=3"));
    }
    #[test]
    fn test_flat_closure_deserialize_too_short() {
        assert!(FlatClosure::deserialize(&[0, 1, 2]).is_none());
    }
    #[test]
    fn test_capture_set_basic() {
        let mut cs = CaptureSet::new();
        cs.capture("x");
        cs.capture("y");
        assert!(cs.is_captured("x"));
        assert!(!cs.is_captured("z"));
        assert_eq!(cs.len(), 2);
    }
    #[test]
    fn test_capture_set_bind_removes() {
        let mut cs = CaptureSet::new();
        cs.capture("a");
        cs.bind("a");
        assert!(!cs.is_captured("a"));
    }
    #[test]
    fn test_capture_set_union() {
        let mut cs1 = CaptureSet::new();
        cs1.capture("x");
        let mut cs2 = CaptureSet::new();
        cs2.capture("y");
        cs1.union(&cs2);
        assert!(cs1.is_captured("x"));
        assert!(cs1.is_captured("y"));
    }
    #[test]
    fn test_capture_set_difference() {
        let mut cs1 = CaptureSet::new();
        cs1.capture("x");
        cs1.capture("y");
        let mut cs2 = CaptureSet::new();
        cs2.capture("y");
        cs1.difference(&cs2);
        assert!(cs1.is_captured("x"));
        assert!(!cs1.is_captured("y"));
    }
    #[test]
    fn test_capture_set_display() {
        let mut cs = CaptureSet::new();
        cs.capture("a");
        let s = format!("{}", cs);
        assert!(s.contains("a"));
    }
    #[test]
    fn test_pap_queue_enqueue_dequeue() {
        let mut q = PapQueue::new(10);
        let c = Closure::new(FnPtr::new(0), 2, Vec::new());
        let pap = Pap::new(c, vec![RtObject::nat(1)]);
        assert!(q.enqueue(pap));
        assert_eq!(q.len(), 1);
        let out = q.dequeue();
        assert!(out.is_some());
        assert!(q.is_empty());
    }
    #[test]
    fn test_pap_queue_full() {
        let mut q = PapQueue::new(1);
        let c = Closure::new(FnPtr::new(0), 2, Vec::new());
        let pap1 = Pap::new(c.clone(), vec![]);
        let pap2 = Pap::new(c, vec![]);
        assert!(q.enqueue(pap1));
        assert!(!q.enqueue(pap2));
    }
    #[test]
    fn test_pap_queue_stats() {
        let mut q = PapQueue::new(10);
        let c = Closure::new(FnPtr::new(0), 1, Vec::new());
        let pap = Pap::new(c, vec![]);
        q.enqueue(pap);
        q.apply_front(vec![RtObject::nat(42)]);
        assert_eq!(q.enqueue_count(), 1);
        assert_eq!(q.saturated_count(), 1);
    }
    #[test]
    fn test_closure_serializer_roundtrip() {
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10u64);
        env.insert("y".to_string(), 20u64);
        let bytes = ClosureSerializer::serialize_env(&env);
        let restored =
            ClosureSerializer::deserialize_env(&bytes).expect("test operation should succeed");
        assert_eq!(restored.get("x"), Some(&10u64));
        assert_eq!(restored.get("y"), Some(&20u64));
    }
    #[test]
    fn test_closure_serializer_empty() {
        let env: HashMap<String, u64> = HashMap::new();
        let bytes = ClosureSerializer::serialize_env(&env);
        let restored =
            ClosureSerializer::deserialize_env(&bytes).expect("test operation should succeed");
        assert!(restored.is_empty());
    }
    #[test]
    fn test_closure_serializer_too_short() {
        assert!(ClosureSerializer::deserialize_env(&[0, 1]).is_none());
    }
}
/// Batch-force a list of flat closures with arity 0 (thunks).
#[allow(dead_code)]
pub fn batch_force_thunks(thunks: &mut Vec<FlatClosure>) -> BatchForceResult {
    let mut forced = 0;
    let mut already = 0;
    for thunk in thunks.iter_mut() {
        if thunk.arity == 0 {
            if thunk.env.is_empty() {
                already += 1;
            } else {
                thunk.env.clear();
                forced += 1;
            }
        }
    }
    BatchForceResult {
        forced,
        failed: 0,
        already_evaluated: already,
    }
}
#[cfg(test)]
mod tests_extended3 {
    use super::*;
    #[test]
    fn test_closure_registry_basic() {
        let mut reg = ClosureRegistry::new();
        reg.register("foo", Closure::new(FnPtr::new(0), 1, Vec::new()));
        assert!(reg.lookup("foo").is_some());
        assert!(reg.lookup("bar").is_none());
        assert_eq!(reg.access_count("foo"), 1);
    }
    #[test]
    fn test_closure_registry_unregister() {
        let mut reg = ClosureRegistry::new();
        reg.register("x", Closure::new(FnPtr::new(0), 0, Vec::new()));
        let removed = reg.unregister("x");
        assert!(removed.is_some());
        assert!(reg.is_empty());
    }
    #[test]
    fn test_closure_registry_most_accessed() {
        let mut reg = ClosureRegistry::new();
        reg.register("a", Closure::new(FnPtr::new(0), 0, Vec::new()));
        reg.register("b", Closure::new(FnPtr::new(1), 0, Vec::new()));
        reg.lookup("a");
        reg.lookup("a");
        reg.lookup("a");
        reg.lookup("b");
        assert_eq!(reg.most_accessed(), Some("a"));
    }
    #[test]
    fn test_batch_force_thunks() {
        let mut thunks = vec![
            FlatClosure::new(0, 0, vec![1, 2]),
            FlatClosure::new(1, 0, vec![]),
            FlatClosure::new(2, 1, vec![]),
        ];
        let result = batch_force_thunks(&mut thunks);
        assert_eq!(result.forced, 1);
        assert_eq!(result.already_evaluated, 1);
        assert_eq!(result.failed, 0);
    }
}
/// Summarize the closure size distribution in a function table.
#[allow(dead_code)]
pub fn closure_size_summary(table: &FunctionTable) -> (usize, usize, f64) {
    let estimator = ClosureSizeEstimator::default();
    let sizes: Vec<usize> = table
        .iter()
        .map(|(_ptr, entry)| {
            estimator.object_overhead + estimator.ptr_size * (entry.env_size as usize + 1)
        })
        .collect();
    if sizes.is_empty() {
        return (0, 0, 0.0);
    }
    let min = *sizes.iter().min().expect("test operation should succeed");
    let max = *sizes.iter().max().expect("test operation should succeed");
    let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
    (min, max, avg)
}
#[cfg(test)]
mod tests_estimator {
    use super::*;
    #[test]
    fn test_closure_size_estimator() {
        let est = ClosureSizeEstimator::default();
        let c = Closure::new(FnPtr::new(0), 2, vec![RtObject::nat(1), RtObject::nat(2)]);
        let size = est.estimate_closure(&c);
        assert!(size > 0);
    }
    #[test]
    fn test_pap_size_estimate() {
        let est = ClosureSizeEstimator::default();
        let c = Closure::new(FnPtr::new(0), 3, Vec::new());
        let pap = Pap::new(c, vec![RtObject::nat(1)]);
        let size = est.estimate_pap(&pap);
        assert!(size > 0);
    }
    #[test]
    fn test_flat_closure_size_estimate() {
        let est = ClosureSizeEstimator::default();
        let fc = FlatClosure::new(0, 1, vec![42, 43]);
        let size = est.estimate_flat(&fc);
        assert!(size > 0);
    }
}
/// Convert a multi-argument function into a chain of single-argument closures.
/// Returns the chain as a Vec of FlatClosures with decreasing arities.
#[allow(dead_code)]
pub fn curry_closure(fn_index: u32, arity: u32) -> Vec<FlatClosure> {
    (0..arity)
        .map(|i| FlatClosure::new(fn_index, arity - i, vec![]))
        .collect()
}
/// Uncurry: given a vector of single-arg PAP results, collapse to a single call.
#[allow(dead_code)]
pub fn uncurry_apply(fn_index: u32, args: Vec<u64>) -> FlatClosure {
    FlatClosure::new(fn_index, 0, args)
}
#[cfg(test)]
mod tests_curry {
    use super::*;
    #[test]
    fn test_curry_closure() {
        let chain = curry_closure(3, 4);
        assert_eq!(chain.len(), 4);
        assert_eq!(chain[0].arity, 4);
        assert_eq!(chain[3].arity, 1);
    }
    #[test]
    fn test_uncurry_apply() {
        let fc = uncurry_apply(5, vec![1, 2, 3]);
        assert_eq!(fc.fn_index, 5);
        assert_eq!(fc.arity, 0);
        assert_eq!(fc.env, vec![1, 2, 3]);
    }
}
