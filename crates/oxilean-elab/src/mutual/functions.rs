//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Expr, Name, ReducibilityHint};
use oxilean_parse::AttributeKind;
use std::collections::{HashMap, HashSet};

use super::types::{
    ArgRelation, CallGraph, DeclDependencyGraph, MutualBlock, MutualChecker,
    MutualDefCycleDetector, MutualElabBudget, MutualElabError, MutualElabProgress, MutualElabStage,
    MutualRecursionSummary, MutualSigCollection, PartialSig, StructuralRecursion, TarjanNode,
    TerminationKind, TerminationMeasure, WellFoundedOrder, WellFoundedRecursion,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutual::*;
    use oxilean_kernel::{Level, Literal};
    fn nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    #[test]
    fn test_mutual_block_create() {
        let block = MutualBlock::new();
        assert_eq!(block.size(), 0);
    }
    #[test]
    fn test_mutual_block_add() {
        let mut block = MutualBlock::new();
        let name = Name::str("f");
        let ty = nat_lit(0);
        let body = nat_lit(1);
        block.add(name.clone(), ty.clone(), body.clone());
        assert_eq!(block.size(), 1);
        assert!(block.contains(&name));
        assert_eq!(block.get_type(&name), Some(&ty));
        assert_eq!(block.get_body(&name), Some(&body));
    }
    #[test]
    fn test_mutual_block_add_with_attrs() {
        let mut block = MutualBlock::new();
        let name = Name::str("f");
        block.add_with_attrs(
            name.clone(),
            nat_lit(0),
            nat_lit(1),
            vec![AttributeKind::Simp],
            false,
        );
        assert_eq!(block.get_attrs(&name), &[AttributeKind::Simp]);
        assert!(!block.is_def_noncomputable(&name));
    }
    #[test]
    fn test_mutual_block_names_in_order() {
        let mut block = MutualBlock::new();
        block.add(Name::str("a"), nat_lit(0), nat_lit(0));
        block.add(Name::str("b"), nat_lit(0), nat_lit(0));
        block.add(Name::str("c"), nat_lit(0), nat_lit(0));
        assert_eq!(
            block.names_in_order(),
            &[Name::str("a"), Name::str("b"), Name::str("c")]
        );
    }
    #[test]
    fn test_mutual_block_get_all_bodies() {
        let mut block = MutualBlock::new();
        block.add(Name::str("a"), nat_lit(0), nat_lit(10));
        block.add(Name::str("b"), nat_lit(0), nat_lit(20));
        let bodies = block.get_all_bodies();
        assert_eq!(bodies.len(), 2);
    }
    #[test]
    fn test_mutual_block_validate_ok() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        assert!(block.validate().is_ok());
    }
    #[test]
    fn test_mutual_block_validate_empty() {
        let block = MutualBlock::new();
        assert!(block.validate().is_err());
    }
    #[test]
    fn test_mutual_block_validate_missing_body() {
        let mut block = MutualBlock::new();
        block.names.push(Name::str("f"));
        block.types.insert(Name::str("f"), nat_lit(0));
        assert!(block.validate().is_err());
    }
    #[test]
    fn test_mutual_block_validate_missing_type() {
        let mut block = MutualBlock::new();
        block.names.push(Name::str("f"));
        block.bodies.insert(Name::str("f"), nat_lit(0));
        assert!(block.validate().is_err());
    }
    #[test]
    fn test_mutual_block_set_univ_params() {
        let mut block = MutualBlock::new();
        block.set_univ_params(vec![Name::str("u"), Name::str("v")]);
        assert_eq!(block.univ_params.len(), 2);
    }
    #[test]
    fn test_mutual_block_noncomputable() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        block.set_noncomputable(&Name::str("f"), true);
        assert!(block.is_def_noncomputable(&Name::str("f")));
        assert!(!block.is_def_noncomputable(&Name::str("g")));
    }
    #[test]
    fn test_mutual_checker() {
        let mut checker = MutualChecker::new();
        assert!(checker.current_block().is_none());
        checker.start_block();
        assert!(checker.current_block().is_some());
        let name = Name::str("f");
        let ty = nat_lit(0);
        let body = nat_lit(1);
        assert!(checker.add_def(name.clone(), ty, body).is_ok());
        let block = checker
            .finish_block()
            .expect("test operation should succeed");
        assert_eq!(block.size(), 1);
        assert!(block.contains(&name));
    }
    #[test]
    fn test_add_without_block() {
        let mut checker = MutualChecker::new();
        let result = checker.add_def(Name::str("f"), nat_lit(0), nat_lit(1));
        assert!(result.is_err());
    }
    #[test]
    fn test_finish_without_block() {
        let mut checker = MutualChecker::new();
        let result = checker.finish_block();
        assert!(result.is_err());
    }
    #[test]
    fn test_check_well_formedness_ok() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        block.add(Name::str("g"), nat_lit(0), nat_lit(2));
        assert!(MutualChecker::check_well_formedness(&block).is_ok());
    }
    #[test]
    fn test_check_well_formedness_empty() {
        let block = MutualBlock::new();
        assert!(MutualChecker::check_well_formedness(&block).is_err());
    }
    #[test]
    fn test_check_termination_non_recursive() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let kind = MutualChecker::check_termination(&block).expect("test operation should succeed");
        assert_eq!(kind, TerminationKind::NonRecursive);
    }
    #[test]
    fn test_check_termination_self_recursive() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("x"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let kind = MutualChecker::check_termination(&block).expect("test operation should succeed");
        assert!(matches!(kind, TerminationKind::Structural(_)));
    }
    #[test]
    fn test_check_termination_wf_fallback() {
        let mut block = MutualBlock::new();
        let body = mk_app(mk_const("f"), mk_const("something_else"));
        block.add(Name::str("f"), nat_lit(0), body);
        let kind = MutualChecker::check_termination(&block).expect("test operation should succeed");
        assert_eq!(kind, TerminationKind::WellFounded);
    }
    #[test]
    fn test_elaborate_mutual_defs() {
        let names = vec![Name::str("f"), Name::str("g")];
        let types = vec![nat_lit(0), nat_lit(0)];
        let bodies = vec![nat_lit(1), nat_lit(2)];
        let block = MutualChecker::elaborate_mutual_defs(&names, &types, &bodies)
            .expect("elaboration should succeed");
        assert_eq!(block.size(), 2);
        assert!(block.contains(&Name::str("f")));
        assert!(block.contains(&Name::str("g")));
    }
    #[test]
    fn test_elaborate_mutual_defs_mismatched() {
        let names = vec![Name::str("f")];
        let types = vec![nat_lit(0), nat_lit(0)];
        let bodies = vec![nat_lit(1)];
        assert!(MutualChecker::elaborate_mutual_defs(&names, &types, &bodies).is_err());
    }
    #[test]
    fn test_elaborate_mutual_defs_empty() {
        let result = MutualChecker::elaborate_mutual_defs(&[], &[], &[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_encode_recursion_non_recursive() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let result = MutualChecker::encode_recursion(block, &TerminationKind::NonRecursive);
        assert!(result.is_ok());
    }
    #[test]
    fn test_split_mutual_block() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        block.add(Name::str("g"), nat_lit(0), nat_lit(2));
        block.set_univ_params(vec![Name::str("u")]);
        let decls = MutualChecker::split_mutual_block(&block);
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].name(), &Name::str("f"));
        assert_eq!(decls[1].name(), &Name::str("g"));
        assert_eq!(decls[0].univ_params().len(), 1);
    }
    #[test]
    fn test_call_graph_empty() {
        let block = MutualBlock::new();
        let cg = CallGraph::build_from_block(&block);
        assert!(!cg.is_recursive());
        assert!(!cg.is_mutually_recursive());
    }
    #[test]
    fn test_call_graph_non_recursive() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let cg = CallGraph::build_from_block(&block);
        assert!(!cg.is_recursive());
        assert!(!cg.is_self_recursive(&Name::str("f")));
    }
    #[test]
    fn test_call_graph_self_recursive() {
        let mut block = MutualBlock::new();
        let body = mk_app(mk_const("f"), Expr::BVar(0));
        block.add(Name::str("f"), nat_lit(0), body);
        let cg = CallGraph::build_from_block(&block);
        assert!(cg.is_recursive());
        assert!(cg.is_self_recursive(&Name::str("f")));
    }
    #[test]
    fn test_call_graph_mutual_recursive() {
        let mut block = MutualBlock::new();
        let body_f = mk_app(mk_const("g"), Expr::BVar(0));
        let body_g = mk_app(mk_const("f"), Expr::BVar(0));
        block.add(Name::str("f"), nat_lit(0), body_f);
        block.add(Name::str("g"), nat_lit(0), body_g);
        let cg = CallGraph::build_from_block(&block);
        assert!(cg.is_mutually_recursive());
    }
    #[test]
    fn test_call_graph_get_calls() {
        let mut block = MutualBlock::new();
        let body = mk_app(mk_const("f"), Expr::BVar(0));
        block.add(Name::str("f"), nat_lit(0), body);
        let cg = CallGraph::build_from_block(&block);
        let calls = cg.get_calls(&Name::str("f"));
        assert!(!calls.is_empty());
    }
    #[test]
    fn test_call_graph_structurally_decreasing() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("x"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let cg = CallGraph::build_from_block(&block);
        assert!(cg.is_structurally_decreasing(&Name::str("f"), 0));
    }
    #[test]
    fn test_call_graph_find_decreasing_arg() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("x"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let cg = CallGraph::build_from_block(&block);
        assert_eq!(cg.find_decreasing_arg(&Name::str("f")), Some(0));
    }
    #[test]
    fn test_call_graph_scc_single() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let cg = CallGraph::build_from_block(&block);
        let sccs = cg.strongly_connected_components();
        assert_eq!(sccs.len(), 1);
    }
    #[test]
    fn test_call_graph_scc_mutual() {
        let mut block = MutualBlock::new();
        let body_f = mk_app(mk_const("g"), Expr::BVar(0));
        let body_g = mk_app(mk_const("f"), Expr::BVar(0));
        block.add(Name::str("f"), nat_lit(0), body_f);
        block.add(Name::str("g"), nat_lit(0), body_g);
        let cg = CallGraph::build_from_block(&block);
        let sccs = cg.strongly_connected_components();
        let has_joint = sccs.iter().any(|scc| scc.len() == 2);
        assert!(has_joint);
    }
    #[test]
    fn test_call_graph_scc_no_recursion() {
        let mut block = MutualBlock::new();
        block.add(Name::str("a"), nat_lit(0), nat_lit(1));
        block.add(Name::str("b"), nat_lit(0), nat_lit(2));
        let cg = CallGraph::build_from_block(&block);
        let sccs = cg.strongly_connected_components();
        assert_eq!(sccs.len(), 2);
    }
    #[test]
    fn test_structural_recursion_detect() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("x"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let mut sr = StructuralRecursion::new(block);
        assert!(sr.detect_structural_recursion().is_ok());
        assert!(sr.get_recursive_args().contains_key(&Name::str("f")));
    }
    #[test]
    fn test_structural_recursion_encode() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("x"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let mut sr = StructuralRecursion::new(block);
        sr.detect_structural_recursion()
            .expect("test operation should succeed");
        let encoded = sr
            .encode_as_recursor_application()
            .expect("type conversion should succeed");
        assert_eq!(encoded.size(), 1);
    }
    #[test]
    fn test_structural_recursion_non_recursive() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let mut sr = StructuralRecursion::new(block);
        assert!(sr.detect_structural_recursion().is_ok());
        assert!(sr.get_recursive_args().is_empty());
    }
    #[test]
    fn test_wf_recursion_new() {
        let block = MutualBlock::new();
        let wfr = WellFoundedRecursion::new(block);
        assert!(wfr.measure.is_none());
        assert!(wfr.rel.is_none());
    }
    #[test]
    fn test_wf_recursion_set_measure() {
        let block = MutualBlock::new();
        let mut wfr = WellFoundedRecursion::new(block);
        wfr.set_measure(Name::str("my_measure"));
        assert_eq!(wfr.measure, Some(Name::str("my_measure")));
    }
    #[test]
    fn test_wf_recursion_set_relation() {
        let block = MutualBlock::new();
        let mut wfr = WellFoundedRecursion::new(block);
        wfr.set_relation(mk_const("lt"));
        assert!(wfr.rel.is_some());
    }
    #[test]
    fn test_wf_recursion_detect_decreasing() {
        let mut block = MutualBlock::new();
        let body = mk_app(mk_const("f"), mk_const("something"));
        block.add(Name::str("f"), nat_lit(0), body);
        let mut wfr = WellFoundedRecursion::new(block);
        assert!(wfr.detect_decreasing_args().is_ok());
    }
    #[test]
    fn test_wf_recursion_encode_no_measure() {
        let block = MutualBlock::new();
        let wfr = WellFoundedRecursion::new(block);
        assert!(wfr.encode_as_wf_recursion().is_err());
    }
    #[test]
    fn test_wf_recursion_encode_with_measure() {
        let mut block = MutualBlock::new();
        block.add(Name::str("f"), nat_lit(0), nat_lit(1));
        let mut wfr = WellFoundedRecursion::new(block);
        wfr.set_measure(Name::str("size"));
        let result = wfr.encode_as_wf_recursion();
        assert!(result.is_ok());
    }
    #[test]
    fn test_wf_recursion_generate_proof_no_measure() {
        let block = MutualBlock::new();
        let wfr = WellFoundedRecursion::new(block);
        assert!(wfr.generate_termination_proof().is_err());
    }
    #[test]
    fn test_wf_recursion_generate_proof_with_measure() {
        let block = MutualBlock::new();
        let mut wfr = WellFoundedRecursion::new(block);
        wfr.set_measure(Name::str("size"));
        let proof = wfr
            .generate_termination_proof()
            .expect("test operation should succeed");
        assert!(matches!(proof, Expr::Const(_, _)));
    }
    #[test]
    fn test_mutual_elab_error_display() {
        assert_eq!(
            format!("{}", MutualElabError::TypeMismatch("foo".into())),
            "type mismatch: foo"
        );
        assert_eq!(
            format!("{}", MutualElabError::InvalidRecursion("bar".into())),
            "invalid recursion: bar"
        );
        assert_eq!(
            format!("{}", MutualElabError::MissingDefinition("baz".into())),
            "missing definition: baz"
        );
        assert_eq!(
            format!("{}", MutualElabError::CyclicType("cycle".into())),
            "cyclic type: cycle"
        );
        assert_eq!(
            format!("{}", MutualElabError::TerminationFailure("fail".into())),
            "termination failure: fail"
        );
        assert_eq!(
            format!("{}", MutualElabError::Other("misc".into())),
            "mutual elaboration error: misc"
        );
    }
    #[test]
    fn test_arg_relation_variants() {
        let relations = [
            ArgRelation::Equal,
            ArgRelation::Smaller,
            ArgRelation::Unknown,
        ];
        assert_eq!(relations.len(), 3);
        assert_eq!(ArgRelation::Equal, ArgRelation::Equal);
        assert_ne!(ArgRelation::Equal, ArgRelation::Smaller);
    }
    #[test]
    fn test_termination_kind_variants() {
        let non_rec = TerminationKind::NonRecursive;
        let wf = TerminationKind::WellFounded;
        let mut structural_map = HashMap::new();
        structural_map.insert(Name::str("f"), 0usize);
        let structural = TerminationKind::Structural(structural_map);
        assert_eq!(non_rec, TerminationKind::NonRecursive);
        assert_eq!(wf, TerminationKind::WellFounded);
        assert!(matches!(structural, TerminationKind::Structural(_)));
    }
    #[test]
    fn test_full_pipeline_non_recursive() {
        let names = vec![Name::str("id")];
        let types = vec![Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        )];
        let bodies = vec![Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        )];
        let block = MutualChecker::elaborate_mutual_defs(&names, &types, &bodies)
            .expect("elaboration should succeed");
        let kind = MutualChecker::check_termination(&block).expect("test operation should succeed");
        assert_eq!(kind, TerminationKind::NonRecursive);
        let decls = MutualChecker::split_mutual_block(&block);
        assert_eq!(decls.len(), 1);
    }
    #[test]
    fn test_full_pipeline_structural_recursive() {
        let mut block = MutualBlock::new();
        let body = mk_app(
            mk_const("f"),
            Expr::Proj(Name::str("n"), 0, Box::new(Expr::BVar(0))),
        );
        block.add(Name::str("f"), nat_lit(0), body);
        let kind = MutualChecker::check_termination(&block).expect("test operation should succeed");
        assert!(matches!(kind, TerminationKind::Structural(_)));
        let encoded =
            MutualChecker::encode_recursion(block, &kind).expect("test operation should succeed");
        assert_eq!(encoded.size(), 1);
    }
}
/// A node index used in Tarjan's SCC algorithm.
type SccIndex = usize;
/// Compute strongly-connected components of a directed graph using
/// Tarjan's algorithm.  The graph is represented as an adjacency list
/// indexed by `SccIndex`.
#[allow(dead_code)]
pub fn tarjan_scc(n: usize, adj: &[Vec<SccIndex>]) -> Vec<Vec<SccIndex>> {
    let mut nodes: Vec<TarjanNode> = vec![TarjanNode::default(); n];
    let mut stack: Vec<SccIndex> = Vec::new();
    let mut sccs: Vec<Vec<SccIndex>> = Vec::new();
    let mut counter = 0usize;
    fn strongconnect(
        v: SccIndex,
        adj: &[Vec<SccIndex>],
        nodes: &mut Vec<TarjanNode>,
        stack: &mut Vec<SccIndex>,
        sccs: &mut Vec<Vec<SccIndex>>,
        counter: &mut usize,
    ) {
        nodes[v].index = *counter;
        nodes[v].lowlink = *counter;
        nodes[v].discovered = true;
        nodes[v].on_stack = true;
        *counter += 1;
        stack.push(v);
        for &w in &adj[v] {
            if !nodes[w].discovered {
                strongconnect(w, adj, nodes, stack, sccs, counter);
                let wll = nodes[w].lowlink;
                if wll < nodes[v].lowlink {
                    nodes[v].lowlink = wll;
                }
            } else if nodes[w].on_stack {
                let wi = nodes[w].index;
                if wi < nodes[v].lowlink {
                    nodes[v].lowlink = wi;
                }
            }
        }
        if nodes[v].lowlink == nodes[v].index {
            let mut scc = Vec::new();
            loop {
                let w = stack.pop().expect("stack non-empty");
                nodes[w].on_stack = false;
                scc.push(w);
                if w == v {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
    for v in 0..n {
        if !nodes[v].discovered {
            strongconnect(v, adj, &mut nodes, &mut stack, &mut sccs, &mut counter);
        }
    }
    sccs
}
#[cfg(test)]
mod mutual_ext_tests {
    use super::*;
    use crate::mutual::*;
    #[test]
    fn test_tarjan_scc_no_cycle() {
        let adj = vec![vec![1], vec![2], vec![]];
        let sccs = tarjan_scc(3, &adj);
        assert_eq!(sccs.len(), 3);
        for scc in &sccs {
            assert_eq!(scc.len(), 1);
        }
    }
    #[test]
    fn test_tarjan_scc_simple_cycle() {
        let adj = vec![vec![1], vec![0]];
        let sccs = tarjan_scc(2, &adj);
        assert_eq!(sccs.len(), 1);
        assert_eq!(sccs[0].len(), 2);
    }
    #[test]
    fn test_tarjan_scc_two_components() {
        let adj = vec![vec![1], vec![0], vec![3], vec![]];
        let sccs = tarjan_scc(4, &adj);
        assert_eq!(sccs.len(), 3);
    }
    #[test]
    fn test_decl_dependency_graph_no_cycle() {
        let mut g = DeclDependencyGraph::new();
        let a = g.add_node(Name::str("f"));
        let b = g.add_node(Name::str("g"));
        g.add_edge(a, b);
        assert!(!g.has_cycle());
    }
    #[test]
    fn test_decl_dependency_graph_cycle() {
        let mut g = DeclDependencyGraph::new();
        let a = g.add_node(Name::str("f"));
        let b = g.add_node(Name::str("g"));
        g.add_edge(a, b);
        g.add_edge(b, a);
        assert!(g.has_cycle());
        let cyclic = g.cyclic_sccs();
        assert_eq!(cyclic.len(), 1);
        assert_eq!(cyclic[0].len(), 2);
    }
    #[test]
    fn test_mutual_def_cycle_detector_no_mutual() {
        let mut det = MutualDefCycleDetector::new();
        det.register(Name::str("f"));
        det.register(Name::str("g"));
        det.add_dependency(&Name::str("f"), &Name::str("g"));
        assert!(!det.has_mutual_recursion());
        assert!(det.mutual_groups().is_empty());
    }
    #[test]
    fn test_mutual_def_cycle_detector_mutual() {
        let mut det = MutualDefCycleDetector::new();
        det.register(Name::str("even"));
        det.register(Name::str("odd"));
        det.add_dependency(&Name::str("even"), &Name::str("odd"));
        det.add_dependency(&Name::str("odd"), &Name::str("even"));
        assert!(det.has_mutual_recursion());
        let groups = det.mutual_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
    }
    #[test]
    fn test_mutual_def_cycle_detector_add_dependency_unknown() {
        let mut det = MutualDefCycleDetector::new();
        det.register(Name::str("f"));
        let ok = det.add_dependency(&Name::str("f"), &Name::str("g"));
        assert!(!ok);
    }
    #[test]
    fn test_well_founded_order_display() {
        let o = WellFoundedOrder::Structural(2);
        assert_eq!(o.to_string(), "structural(arg2)");
        let o2 = WellFoundedOrder::Lexicographic(vec![0, 1]);
        assert!(o2.to_string().contains("lex"));
    }
    #[test]
    fn test_termination_measure_reliable() {
        let m = TerminationMeasure::certain(
            WellFoundedOrder::Structural(0),
            "arg 0 structurally decreases",
        );
        assert!(m.is_reliable());
        let m2 =
            TerminationMeasure::heuristic(WellFoundedOrder::Unknown, 0.5, "could not determine");
        assert!(!m2.is_reliable());
    }
    #[test]
    fn test_mutual_recursion_summary_no_mutual() {
        let mut det = MutualDefCycleDetector::new();
        det.register(Name::str("f"));
        let summary = MutualRecursionSummary::from_detector(&det, None);
        assert!(!summary.is_mutually_recursive);
        assert!(!summary.has_diagnostics());
    }
    #[test]
    fn test_mutual_recursion_summary_with_diagnostics() {
        let mut det = MutualDefCycleDetector::new();
        det.register(Name::str("f"));
        det.register(Name::str("g"));
        det.add_dependency(&Name::str("f"), &Name::str("g"));
        det.add_dependency(&Name::str("g"), &Name::str("f"));
        let mut summary = MutualRecursionSummary::from_detector(&det, None);
        summary.add_diagnostic("termination unresolved");
        assert!(summary.has_diagnostics());
    }
}
#[cfg(test)]
mod mutual_pipeline_tests {
    use super::*;
    use crate::mutual::*;
    #[test]
    fn test_mutual_elab_stage_order() {
        assert!(MutualElabStage::SigCollection < MutualElabStage::DependencyAnalysis);
        assert!(MutualElabStage::DependencyAnalysis < MutualElabStage::BodyElab);
        assert!(MutualElabStage::BodyElab < MutualElabStage::TerminationCheck);
    }
    #[test]
    fn test_mutual_elab_stage_display() {
        assert_eq!(format!("{}", MutualElabStage::BodyElab), "BodyElab");
        assert_eq!(format!("{}", MutualElabStage::Done), "Done");
    }
    #[test]
    fn test_mutual_elab_progress_advance_to_done() {
        let mut p = MutualElabProgress::new(vec![Name::str("f")]);
        for _ in 0..5 {
            p.advance();
        }
        assert!(p.is_done());
        assert!(p.is_success());
        assert_eq!(p.completed.len(), 5);
    }
    #[test]
    fn test_mutual_elab_progress_fail() {
        let mut p = MutualElabProgress::new(vec![Name::str("f")]);
        p.advance();
        p.fail(MutualElabError::TerminationFailure(
            "no measure".to_string(),
        ));
        assert!(p.is_done());
        assert!(!p.is_success());
        assert!(p.error.is_some());
    }
    #[test]
    fn test_mutual_elab_progress_stage_display() {
        let p = MutualElabProgress::new(vec![Name::str("f")]);
        assert_eq!(format!("{}", p.stage), "SigCollection");
    }
    #[test]
    fn test_mutual_elab_progress_advance_from_done_is_idempotent() {
        let mut p = MutualElabProgress::new(vec![]);
        for _ in 0..10 {
            p.advance();
        }
        assert!(p.is_done());
    }
}
#[cfg(test)]
mod mutual_budget_tests {
    use super::*;
    use crate::mutual::*;
    #[test]
    fn test_budget_default_allows_normal_scc() {
        let b = MutualElabBudget::default();
        assert!(b.allows_scc_size(10));
        assert!(!b.allows_scc_size(100));
    }
    #[test]
    fn test_budget_liberal() {
        let b = MutualElabBudget::liberal();
        assert!(b.allows_scc_size(200));
        assert!(b.allows_termination_depth(500));
    }
    #[test]
    fn test_budget_strict() {
        let b = MutualElabBudget::strict();
        assert!(!b.allows_scc_size(10));
        assert!(b.allows_scc_size(4));
    }
    #[test]
    fn test_partial_sig_resolve() {
        let mut sig = PartialSig::new(Name::str("f"));
        assert!(!sig.resolved);
        assert!(sig.best_type().is_none());
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        sig.resolve(ty.clone());
        assert!(sig.resolved);
        assert!(sig.best_type().is_some());
    }
    #[test]
    fn test_mutual_sig_collection_all_resolved() {
        let mut col = MutualSigCollection::new();
        col.add(PartialSig::new(Name::str("f")));
        col.add(PartialSig::new(Name::str("g")));
        assert!(!col.all_resolved());
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        if let Some(s) = col.get_mut(&Name::str("f")) {
            s.resolve(ty.clone());
        }
        if let Some(s) = col.get_mut(&Name::str("g")) {
            s.resolve(ty.clone());
        }
        assert!(col.all_resolved());
    }
    #[test]
    fn test_mutual_sig_collection_lookup() {
        let mut col = MutualSigCollection::new();
        col.add(PartialSig::new(Name::str("foo")));
        assert!(col.get(&Name::str("foo")).is_some());
        assert!(col.get(&Name::str("bar")).is_none());
    }
}
