//! Tests for the plan_optimizer module.

#[cfg(test)]
mod test_impl {
    use crate::plan_optimizer::{
        critical_path, is_acyclic, leaf_nodes, minimum_makespan, root_nodes, schedule,
        sequential_cost, BuildNode, BuildPlan,
    };

    // --------------------------------------------------------
    // Helpers
    // --------------------------------------------------------

    fn node(id: usize, cost: u64, deps: Vec<usize>) -> BuildNode {
        BuildNode::new(id, format!("node_{}", id), cost, deps)
    }

    fn linear_plan(costs: &[u64]) -> BuildPlan {
        let nodes: Vec<BuildNode> = costs
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let deps = if i == 0 { vec![] } else { vec![i - 1] };
                node(i, c, deps)
            })
            .collect();
        BuildPlan::from_nodes(nodes)
    }

    // --------------------------------------------------------
    // 1. Empty plan
    // --------------------------------------------------------
    #[test]
    fn test_empty_plan_critical_path() {
        let plan = BuildPlan::from_nodes(vec![]);
        assert!(critical_path(&plan).is_empty());
    }

    #[test]
    fn test_empty_plan_schedule() {
        let plan = BuildPlan::from_nodes(vec![]);
        let sched = schedule(&plan, 4);
        assert_eq!(sched.num_workers(), 4);
        assert_eq!(sched.total_nodes(), 0);
        assert_eq!(sched.estimated_makespan_ms, 0);
    }

    // --------------------------------------------------------
    // 2. Single-node plan
    // --------------------------------------------------------
    #[test]
    fn test_single_node_critical_path() {
        let plan = BuildPlan::from_nodes(vec![node(0, 42, vec![])]);
        let cp = critical_path(&plan);
        assert_eq!(cp, vec![0]);
    }

    #[test]
    fn test_single_node_schedule_one_worker() {
        let plan = BuildPlan::from_nodes(vec![node(0, 42, vec![])]);
        let sched = schedule(&plan, 1);
        assert_eq!(sched.total_nodes(), 1);
        assert_eq!(sched.estimated_makespan_ms, 42);
    }

    // --------------------------------------------------------
    // 3. Linear chain
    // --------------------------------------------------------
    #[test]
    fn test_linear_chain_critical_path() {
        // 0 → 1 → 2 → 3, costs [10, 20, 30, 40]
        let plan = linear_plan(&[10, 20, 30, 40]);
        let cp = critical_path(&plan);
        // Full chain is the only path.
        assert_eq!(cp, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_linear_chain_makespan_equals_sum() {
        let plan = linear_plan(&[10, 20, 30, 40]);
        let sched = schedule(&plan, 4);
        // Chain is the bottleneck; adding workers doesn't help.
        assert_eq!(sched.estimated_makespan_ms, 100);
    }

    // --------------------------------------------------------
    // 4. Parallel branches — critical path follows heaviest branch
    // --------------------------------------------------------
    #[test]
    fn test_parallel_branches_critical_path() {
        // Root 0 (cost 10) → branch A: 1 (cost 5), branch B: 2 (cost 100)
        let nodes = vec![
            node(0, 10, vec![]),
            node(1, 5, vec![0]),
            node(2, 100, vec![0]),
        ];
        let plan = BuildPlan::from_nodes(nodes);
        let cp = critical_path(&plan);
        // Should follow 0 → 2 (10 + 100 = 110 vs 10 + 5 = 15).
        assert_eq!(cp, vec![0, 2]);
    }

    // --------------------------------------------------------
    // 5. Diamond DAG
    // --------------------------------------------------------
    #[test]
    fn test_diamond_dag_critical_path() {
        //        0 (10)
        //       / \
        //    1(5)  2(50)
        //       \ /
        //        3 (10)
        let nodes = vec![
            node(0, 10, vec![]),
            node(1, 5, vec![0]),
            node(2, 50, vec![0]),
            node(3, 10, vec![1, 2]),
        ];
        let plan = BuildPlan::from_nodes(nodes);
        let cp = critical_path(&plan);
        // 0→2→3: 10+50+10=70 vs 0→1→3: 10+5+10=25
        assert_eq!(cp, vec![0, 2, 3]);
    }

    // --------------------------------------------------------
    // 6. is_acyclic
    // --------------------------------------------------------
    #[test]
    fn test_is_acyclic_true_for_dag() {
        let plan = linear_plan(&[1, 2, 3]);
        assert!(is_acyclic(&plan));
    }

    #[test]
    fn test_is_acyclic_true_for_empty() {
        assert!(is_acyclic(&BuildPlan::from_nodes(vec![])));
    }

    // --------------------------------------------------------
    // 7. root_nodes and leaf_nodes
    // --------------------------------------------------------
    #[test]
    fn test_root_and_leaf_nodes_linear() {
        let plan = linear_plan(&[10, 20, 30]);
        assert_eq!(root_nodes(&plan), vec![0]);
        assert_eq!(leaf_nodes(&plan), vec![2]);
    }

    #[test]
    fn test_root_nodes_multiple() {
        let nodes = vec![
            node(0, 10, vec![]),
            node(1, 20, vec![]),
            node(2, 30, vec![0, 1]),
        ];
        let plan = BuildPlan::from_nodes(nodes);
        let mut roots = root_nodes(&plan);
        roots.sort();
        assert_eq!(roots, vec![0, 1]);
        assert_eq!(leaf_nodes(&plan), vec![2]);
    }

    // --------------------------------------------------------
    // 8. sequential_cost
    // --------------------------------------------------------
    #[test]
    fn test_sequential_cost() {
        let plan = linear_plan(&[10, 20, 30]);
        assert_eq!(sequential_cost(&plan), 60);
    }

    // --------------------------------------------------------
    // 9. minimum_makespan
    // --------------------------------------------------------
    #[test]
    fn test_minimum_makespan_linear() {
        let plan = linear_plan(&[10, 20, 30]);
        assert_eq!(minimum_makespan(&plan), 60); // whole chain
    }

    #[test]
    fn test_minimum_makespan_parallel() {
        // Two independent nodes.
        let nodes = vec![node(0, 10, vec![]), node(1, 90, vec![])];
        let plan = BuildPlan::from_nodes(nodes);
        // Critical path is just node 1.
        assert_eq!(minimum_makespan(&plan), 90);
    }

    // --------------------------------------------------------
    // 10. Schedule: all nodes assigned
    // --------------------------------------------------------
    #[test]
    fn test_schedule_assigns_all_nodes() {
        let plan = linear_plan(&[10, 20, 30, 40]);
        let sched = schedule(&plan, 2);
        assert_eq!(sched.total_nodes(), 4);
    }

    // --------------------------------------------------------
    // 11. Schedule: no worker has more than one lane
    // --------------------------------------------------------
    #[test]
    fn test_schedule_correct_number_of_lanes() {
        let plan = linear_plan(&[10, 20]);
        let sched = schedule(&plan, 3);
        assert_eq!(sched.num_workers(), 3);
    }

    // --------------------------------------------------------
    // 12. Schedule: dependency ordering preserved within lane
    // --------------------------------------------------------
    #[test]
    fn test_schedule_dependency_order_respected() {
        // A linear chain must be scheduled in order 0,1,2.
        let plan = linear_plan(&[10, 20, 30]);
        let sched = schedule(&plan, 1);
        // With one worker the lane must follow topo order.
        assert_eq!(sched.lanes[0], vec![0, 1, 2]);
    }

    // --------------------------------------------------------
    // 13. Schedule: two independent nodes go to separate workers
    // --------------------------------------------------------
    #[test]
    fn test_schedule_independent_nodes_parallel() {
        let nodes = vec![node(0, 50, vec![]), node(1, 50, vec![])];
        let plan = BuildPlan::from_nodes(nodes);
        let sched = schedule(&plan, 2);
        // With 2 workers and 2 independent equal-cost nodes, makespan = 50.
        assert_eq!(sched.estimated_makespan_ms, 50);
    }

    // --------------------------------------------------------
    // 14. Schedule: makespan ≤ sequential_cost
    // --------------------------------------------------------
    #[test]
    fn test_schedule_makespan_le_sequential_cost() {
        let plan = linear_plan(&[10, 20, 30, 40, 50]);
        let sched_par = schedule(&plan, 4);
        let seq = sequential_cost(&plan);
        assert!(sched_par.estimated_makespan_ms <= seq);
    }

    // --------------------------------------------------------
    // 15. Critical path is subset of all node IDs
    // --------------------------------------------------------
    #[test]
    fn test_critical_path_ids_are_valid() {
        let plan = linear_plan(&[5, 15, 10, 20]);
        let all_ids: std::collections::HashSet<usize> = plan.nodes.iter().map(|n| n.id).collect();
        let cp = critical_path(&plan);
        for id in &cp {
            assert!(all_ids.contains(id), "ID {} not in plan", id);
        }
    }

    // --------------------------------------------------------
    // 16. Schedule critical_path field matches standalone function
    // --------------------------------------------------------
    #[test]
    fn test_schedule_critical_path_matches_function() {
        let plan = linear_plan(&[10, 20, 30]);
        let cp_standalone = critical_path(&plan);
        let sched = schedule(&plan, 2);
        assert_eq!(sched.critical_path, cp_standalone);
    }

    // --------------------------------------------------------
    // 17. Explicit edges via BuildPlan::new
    // --------------------------------------------------------
    #[test]
    fn test_explicit_edges_respected() {
        // Nodes have no deps in .dependencies; we supply edges separately.
        let nodes = vec![
            BuildNode::new(0, "a", 10, vec![]),
            BuildNode::new(1, "b", 20, vec![]),
            BuildNode::new(2, "c", 30, vec![]),
        ];
        // 0 → 1 → 2 via explicit edges.
        let edges = vec![(0, 1), (1, 2)];
        let plan = BuildPlan::new(nodes, edges);
        assert!(is_acyclic(&plan));
        let cp = critical_path(&plan);
        assert_eq!(cp, vec![0, 1, 2]);
    }

    // --------------------------------------------------------
    // 18. Wide fan-out: one root many leaves
    // --------------------------------------------------------
    #[test]
    fn test_fan_out_schedule_utilizes_workers() {
        // Node 0 is the root; nodes 1–8 each depend only on 0.
        let mut nodes = vec![node(0, 100, vec![])];
        for i in 1..=8 {
            nodes.push(node(i, 10, vec![0]));
        }
        let plan = BuildPlan::from_nodes(nodes);
        let sched = schedule(&plan, 4);
        // Makespan = cost(0) + cost of heaviest leaf lane ≥ 100 + 10 = 110.
        assert!(sched.estimated_makespan_ms >= 110);
        // All 9 nodes must be scheduled.
        assert_eq!(sched.total_nodes(), 9);
    }

    // --------------------------------------------------------
    // 19. Wide fan-in: many roots one sink
    // --------------------------------------------------------
    #[test]
    fn test_fan_in_critical_path() {
        // Nodes 0–3 are independent; node 4 depends on all of them.
        let mut nodes: Vec<BuildNode> = (0..4)
            .map(|i| node(i, (i as u64 + 1) * 10, vec![]))
            .collect();
        nodes.push(node(4, 5, vec![0, 1, 2, 3])); // sink
        let plan = BuildPlan::from_nodes(nodes);
        let cp = critical_path(&plan);
        // Longest path: node 3 (cost 40) → node 4 (cost 5) = 45.
        assert_eq!(cp, vec![3, 4]);
    }

    // --------------------------------------------------------
    // 20. BuildNode::new constructor
    // --------------------------------------------------------
    #[test]
    fn test_build_node_new() {
        let n = BuildNode::new(7, "my_crate", 500, vec![1, 2, 3]);
        assert_eq!(n.id, 7);
        assert_eq!(n.name, "my_crate");
        assert_eq!(n.estimated_cost_ms, 500);
        assert_eq!(n.dependencies, vec![1, 2, 3]);
    }

    // --------------------------------------------------------
    // 21. BuildPlan::node_count
    // --------------------------------------------------------
    #[test]
    fn test_build_plan_node_count() {
        let plan = linear_plan(&[1, 2, 3, 4, 5]);
        assert_eq!(plan.node_count(), 5);
    }

    // --------------------------------------------------------
    // 22. Schedule with num_workers=0 coerces to 1
    // --------------------------------------------------------
    #[test]
    fn test_schedule_zero_workers_coerces_to_one() {
        let plan = linear_plan(&[10, 20]);
        let sched = schedule(&plan, 0);
        assert_eq!(sched.num_workers(), 1);
        assert_eq!(sched.total_nodes(), 2);
    }

    // --------------------------------------------------------
    // 23. More workers than nodes — extra lanes are empty
    // --------------------------------------------------------
    #[test]
    fn test_schedule_more_workers_than_nodes() {
        let plan = BuildPlan::from_nodes(vec![node(0, 10, vec![])]);
        let sched = schedule(&plan, 8);
        assert_eq!(sched.num_workers(), 8);
        assert_eq!(sched.total_nodes(), 1);
        // At most one lane has a node.
        let non_empty: usize = sched.lanes.iter().filter(|l| !l.is_empty()).count();
        assert_eq!(non_empty, 1);
    }

    // --------------------------------------------------------
    // 24. BuildSchedule::total_nodes aggregates all lanes
    // --------------------------------------------------------
    #[test]
    fn test_schedule_total_nodes_correct() {
        let plan = BuildPlan::from_nodes(vec![
            node(0, 10, vec![]),
            node(1, 20, vec![]),
            node(2, 30, vec![0]),
            node(3, 40, vec![1]),
        ]);
        let sched = schedule(&plan, 2);
        assert_eq!(sched.total_nodes(), 4);
    }

    // --------------------------------------------------------
    // 25. Every node appears exactly once in the schedule
    // --------------------------------------------------------
    #[test]
    fn test_schedule_each_node_appears_exactly_once() {
        let plan = linear_plan(&[10, 20, 30, 40, 50]);
        let sched = schedule(&plan, 3);
        let mut all_ids: Vec<usize> = sched.lanes.iter().flatten().copied().collect();
        all_ids.sort();
        let expected: Vec<usize> = (0..5).collect();
        assert_eq!(all_ids, expected);
    }
}
