//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Level, Name};
use std::collections::HashSet;

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NonEmptyVec, PathBuf, PrefixCounter, ProofAnalysis, ProofAnalyzer, ProofCertificate,
    ProofComplexity, ProofNormalizer, ProofObligation, ProofSkeleton, ProofState, ProofTerm,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Known classical axioms that make proofs non-constructive.
pub(crate) const CLASSICAL_AXIOMS: &[&str] = &[
    "Classical.choice",
    "Classical.em",
    "Classical.byContradiction",
    "Quotient.sound",
    "propext",
];
/// Collect all axiom-like constant dependencies from an expression.
pub fn collect_axiom_deps(expr: &Expr) -> HashSet<Name> {
    let mut result = HashSet::new();
    collect_constants_impl(expr, &mut result);
    result
}
/// Collect constant names recursively.
pub(super) fn collect_constants_impl(expr: &Expr, result: &mut HashSet<Name>) {
    match expr {
        Expr::Const(name, _) => {
            result.insert(name.clone());
        }
        Expr::App(f, a) => {
            collect_constants_impl(f, result);
            collect_constants_impl(a, result);
        }
        Expr::Lam(_, _, ty, body) => {
            collect_constants_impl(ty, result);
            collect_constants_impl(body, result);
        }
        Expr::Pi(_, _, ty, body) => {
            collect_constants_impl(ty, result);
            collect_constants_impl(body, result);
        }
        Expr::Let(_, ty, val, body) => {
            collect_constants_impl(ty, result);
            collect_constants_impl(val, result);
            collect_constants_impl(body, result);
        }
        Expr::Proj(_, _, e) => {
            collect_constants_impl(e, result);
        }
        _ => {}
    }
}
/// Check if proof irrelevance applies between two terms of a given type.
///
/// Returns true if the type is a Prop and both terms are proofs,
/// meaning they should be considered definitionally equal.
pub fn is_proof_irrelevant(ty: &Expr, _t1: &Expr, _t2: &Expr) -> bool {
    ProofTerm::could_be_prop(ty)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_is_proof() {
        let term = Expr::Lit(Literal::Nat(42));
        let prop = Expr::Sort(Level::zero());
        assert!(ProofTerm::is_proof(&term, &prop));
    }
    #[test]
    fn test_is_proof_non_prop() {
        let term = Expr::Lit(Literal::Nat(42));
        let non_prop = Expr::Sort(Level::succ(Level::zero()));
        assert!(!ProofTerm::is_proof(&term, &non_prop));
    }
    #[test]
    fn test_size_simple() {
        let term = Expr::Lit(Literal::Nat(42));
        assert_eq!(ProofTerm::size(&term), 1);
    }
    #[test]
    fn test_size_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(ProofTerm::size(&app), 3);
    }
    #[test]
    fn test_depth() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(ProofTerm::depth(&app), 1);
    }
    #[test]
    fn test_is_constructive() {
        let term = Expr::BVar(0);
        assert!(ProofTerm::is_constructive(&term));
    }
    #[test]
    fn test_is_non_constructive() {
        let term = Expr::Const(Name::str("Classical.choice"), vec![]);
        assert!(!ProofTerm::is_constructive(&term));
    }
    #[test]
    fn test_collect_constants() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let g = Expr::Const(Name::str("g"), vec![]);
        let app = Expr::App(Box::new(f), Box::new(g));
        let consts = ProofTerm::collect_constants(&app);
        assert!(consts.contains(&Name::str("f")));
        assert!(consts.contains(&Name::str("g")));
    }
    #[test]
    fn test_could_be_prop() {
        assert!(ProofTerm::could_be_prop(&Expr::Sort(Level::zero())));
        assert!(!ProofTerm::could_be_prop(&Expr::Sort(Level::succ(
            Level::zero()
        ))));
    }
    #[test]
    fn test_is_sort_zero() {
        assert!(ProofTerm::is_sort_zero(&Expr::Sort(Level::zero())));
        assert!(!ProofTerm::is_sort_zero(&Expr::Sort(Level::succ(
            Level::zero()
        ))));
    }
    #[test]
    fn test_same_proposition_structure() {
        let p1 = Expr::Const(Name::str("True"), vec![]);
        let p2 = Expr::Const(Name::str("True"), vec![]);
        assert!(ProofTerm::same_proposition_structure(&p1, &p2));
        let p3 = Expr::Const(Name::str("False"), vec![]);
        assert!(!ProofTerm::same_proposition_structure(&p1, &p3));
    }
    #[test]
    fn test_proof_irrelevance() {
        let prop_ty = Expr::Sort(Level::zero());
        let proof1 = Expr::Const(Name::str("p1"), vec![]);
        let proof2 = Expr::Const(Name::str("p2"), vec![]);
        assert!(is_proof_irrelevant(&prop_ty, &proof1, &proof2));
        let type_ty = Expr::Sort(Level::succ(Level::zero()));
        assert!(!is_proof_irrelevant(&type_ty, &proof1, &proof2));
    }
}
/// Classifies a proof term into a `ProofComplexity` variant.
#[allow(dead_code)]
pub fn classify_proof(term: &Expr) -> ProofComplexity {
    match term {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Sort(_) | Expr::Lit(_) => {
            ProofComplexity::Atomic
        }
        Expr::Lam(_, _, _, _) => ProofComplexity::Abstraction,
        Expr::App(_, _) => ProofComplexity::Application,
        Expr::Let(_, _, _, _) => ProofComplexity::LetBinding,
        Expr::Proj(_, _, _) => ProofComplexity::Projection,
        Expr::Pi(_, _, _, _) => ProofComplexity::Composite,
    }
}
/// Returns the set of all axiom constants depended upon by `term`.
///
/// This traverses the AST and collects names matching known axiom patterns.
#[allow(dead_code)]
pub fn axiom_dependencies(term: &Expr) -> HashSet<Name> {
    let mut deps = HashSet::new();
    axiom_deps_impl(term, &mut deps);
    deps
}
pub(super) fn axiom_deps_impl(term: &Expr, deps: &mut HashSet<Name>) {
    match term {
        Expr::Const(name, _) if CLASSICAL_AXIOMS.contains(&name.to_string().as_str()) => {
            deps.insert(name.clone());
        }
        Expr::Const(_, _) => {}
        Expr::App(f, a) => {
            axiom_deps_impl(f, deps);
            axiom_deps_impl(a, deps);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            axiom_deps_impl(ty, deps);
            axiom_deps_impl(body, deps);
        }
        Expr::Let(_, ty, val, body) => {
            axiom_deps_impl(ty, deps);
            axiom_deps_impl(val, deps);
            axiom_deps_impl(body, deps);
        }
        Expr::Proj(_, _, e) => axiom_deps_impl(e, deps),
        _ => {}
    }
}
/// Count the number of times a given constant name appears in `term`.
#[allow(dead_code)]
pub fn count_const_occurrences(term: &Expr, target: &Name) -> usize {
    match term {
        Expr::Const(name, _) if name == target => 1,
        Expr::Const(_, _) => 0,
        Expr::App(f, a) => count_const_occurrences(f, target) + count_const_occurrences(a, target),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_const_occurrences(ty, target) + count_const_occurrences(body, target)
        }
        Expr::Let(_, ty, val, body) => {
            count_const_occurrences(ty, target)
                + count_const_occurrences(val, target)
                + count_const_occurrences(body, target)
        }
        Expr::Proj(_, _, e) => count_const_occurrences(e, target),
        _ => 0,
    }
}
/// Returns `true` if `term` contains no free variables (is a closed term).
///
/// A closed term has no `FVar` nodes; BVar nodes are allowed (they are bound).
#[allow(dead_code)]
pub fn is_closed_term(term: &Expr) -> bool {
    match term {
        Expr::FVar(_) => false,
        Expr::BVar(_) | Expr::Const(_, _) | Expr::Sort(_) | Expr::Lit(_) => true,
        Expr::App(f, a) => is_closed_term(f) && is_closed_term(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            is_closed_term(ty) && is_closed_term(body)
        }
        Expr::Let(_, ty, val, body) => {
            is_closed_term(ty) && is_closed_term(val) && is_closed_term(body)
        }
        Expr::Proj(_, _, e) => is_closed_term(e),
    }
}
/// Checks whether two proof terms have structurally identical shapes,
/// ignoring the actual constants and literals at the leaves.
#[allow(dead_code)]
pub fn same_proof_shape(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(_), Expr::FVar(_)) => true,
        (Expr::Const(_, _), Expr::Const(_, _)) => true,
        (Expr::Sort(_), Expr::Sort(_)) => true,
        (Expr::Lit(_), Expr::Lit(_)) => true,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            same_proof_shape(f1, f2) && same_proof_shape(a1, a2)
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
            same_proof_shape(ty1, ty2) && same_proof_shape(b1, b2)
        }
        (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            same_proof_shape(ty1, ty2) && same_proof_shape(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            same_proof_shape(ty1, ty2) && same_proof_shape(v1, v2) && same_proof_shape(b1, b2)
        }
        (Expr::Proj(i1, _, e1), Expr::Proj(i2, _, e2)) => i1 == i2 && same_proof_shape(e1, e2),
        _ => false,
    }
}
/// Estimate the "proof effort" metric: a rough measure of how hard it
/// was to construct the term, based on its size and classical axiom usage.
#[allow(dead_code)]
pub fn proof_effort(term: &Expr) -> u64 {
    let size = ProofTerm::size(term) as u64;
    let depth = ProofTerm::depth(term) as u64;
    let classical_penalty: u64 = if ProofTerm::is_constructive(term) {
        0
    } else {
        100
    };
    size + depth * 2 + classical_penalty
}
#[cfg(test)]
mod extended_proof_tests {
    use super::*;
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[allow(dead_code)]
    fn mk_type1() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    #[test]
    fn test_classify_atomic_const() {
        let term = mk_const("True");
        assert_eq!(classify_proof(&term), ProofComplexity::Atomic);
    }
    #[test]
    fn test_classify_bvar() {
        let term = Expr::BVar(0);
        assert_eq!(classify_proof(&term), ProofComplexity::Atomic);
    }
    #[test]
    fn test_classify_app() {
        let f = mk_const("f");
        let a = mk_const("a");
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(classify_proof(&app), ProofComplexity::Application);
    }
    #[test]
    fn test_proof_analysis_size() {
        let term = Expr::App(Box::new(mk_const("f")), Box::new(mk_const("a")));
        let analysis = ProofAnalysis::analyse(&term);
        assert_eq!(analysis.size, 3);
        assert_eq!(analysis.app_count, 1);
        assert_eq!(analysis.lambda_count, 0);
    }
    #[test]
    fn test_proof_obligation_discharge() {
        let prop = mk_prop();
        let mut obl = ProofObligation::new("main", prop);
        assert!(!obl.is_discharged());
        obl.discharge(mk_const("proof"));
        assert!(obl.is_discharged());
    }
    #[test]
    fn test_proof_state_remaining() {
        let mut state = ProofState::new();
        state.add_obligation("goal1", mk_prop());
        state.add_obligation("goal2", mk_prop());
        assert_eq!(state.remaining(), 2);
        assert!(!state.is_complete());
        state.discharge_next(mk_const("p1"));
        assert_eq!(state.remaining(), 1);
        state.discharge_next(mk_const("p2"));
        assert!(state.is_complete());
    }
    #[test]
    fn test_beta_reduce_simple() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_prop()),
            Box::new(body),
        );
        let arg = mk_const("myarg");
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let reduced = ProofNormalizer::beta_reduce(&app);
        assert_eq!(reduced, arg);
    }
    #[test]
    fn test_count_redexes_none() {
        let term = mk_const("pure");
        assert_eq!(ProofNormalizer::count_redexes(&term), 0);
        assert!(ProofNormalizer::is_beta_normal(&term));
    }
    #[test]
    fn test_count_redexes_one() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_prop()),
            Box::new(body),
        );
        let app = Expr::App(Box::new(lam), Box::new(mk_const("arg")));
        assert_eq!(ProofNormalizer::count_redexes(&app), 1);
        assert!(!ProofNormalizer::is_beta_normal(&app));
    }
    #[test]
    fn test_axiom_dependencies_classical() {
        let term = mk_const("Classical.em");
        let deps = axiom_dependencies(&term);
        assert!(deps.contains(&Name::str("Classical.em")));
    }
    #[test]
    fn test_axiom_dependencies_constructive() {
        let term = mk_const("Nat.succ");
        let deps = axiom_dependencies(&term);
        assert!(deps.is_empty());
    }
    #[test]
    fn test_count_const_occurrences() {
        let target = Name::str("f");
        let f1 = Expr::Const(target.clone(), vec![]);
        let f2 = Expr::Const(target.clone(), vec![]);
        let app = Expr::App(Box::new(f1), Box::new(f2));
        assert_eq!(count_const_occurrences(&app, &target), 2);
    }
    #[test]
    fn test_is_closed_const() {
        let term = mk_const("True");
        assert!(is_closed_term(&term));
    }
    #[test]
    fn test_is_not_closed_fvar() {
        let term = Expr::FVar(crate::FVarId(0));
        assert!(!is_closed_term(&term));
    }
    #[test]
    fn test_same_proof_shape_bvar() {
        let a = Expr::BVar(2);
        let b = Expr::BVar(2);
        let c = Expr::BVar(3);
        assert!(same_proof_shape(&a, &b));
        assert!(!same_proof_shape(&a, &c));
    }
    #[test]
    fn test_proof_effort_classical() {
        let classical_term = mk_const("Classical.em");
        let constructive_term = mk_const("Nat.succ");
        assert!(proof_effort(&classical_term) > proof_effort(&constructive_term));
    }
    #[test]
    fn test_proof_normalizer_subst_bvar_shift() {
        let term = Expr::BVar(1);
        let result = ProofNormalizer::subst_bvar(term, 0, &mk_const("x"));
        assert_eq!(result, Expr::BVar(0));
    }
}
/// Check whether a term contains a sorry-like pattern.
#[allow(dead_code)]
pub(crate) fn contains_classical_sorry(term: &Expr) -> bool {
    match term {
        Expr::Const(n, _) => {
            let s = n.to_string();
            s.contains("sorry") || s.contains("Lean.Elab.Tactic.sorry")
        }
        Expr::App(f, a) => contains_classical_sorry(f) || contains_classical_sorry(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_classical_sorry(ty) || contains_classical_sorry(body)
        }
        Expr::Let(_, ty, val, body) => {
            contains_classical_sorry(ty)
                || contains_classical_sorry(val)
                || contains_classical_sorry(body)
        }
        _ => false,
    }
}
#[cfg(test)]
mod extra_proof_tests {
    use super::*;
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_proof_skeleton_no_holes() {
        let proof = mk_const("True.intro");
        let ty = mk_const("True");
        let skel = ProofSkeleton::new(proof, ty);
        assert!(skel.is_complete());
        assert_eq!(skel.num_holes(), 0);
    }
    #[test]
    fn test_proof_skeleton_with_sorry() {
        let sorry = mk_const("sorry");
        let ty = mk_const("Nat");
        let skel = ProofSkeleton::new(sorry, ty);
        assert!(!skel.is_complete());
        assert_eq!(skel.num_holes(), 1);
    }
    #[test]
    fn test_proof_skeleton_display() {
        let proof = mk_const("True.intro");
        let ty = mk_const("True");
        let skel = ProofSkeleton::new(proof, ty);
        let s = format!("{}", skel);
        assert!(s.contains("ProofSkeleton"));
    }
    #[test]
    fn test_proof_certificate_no_sorry() {
        let prop = mk_const("True");
        let proof = mk_const("True.intro");
        let cert = ProofCertificate::new(prop, proof);
        assert!(!cert.has_sorry);
    }
    #[test]
    fn test_proof_certificate_trusted() {
        let prop = mk_const("True");
        let proof = mk_const("True.intro");
        let cert = ProofCertificate::new(prop, proof);
        assert!(cert.is_trusted());
    }
    #[test]
    fn test_proof_analyzer_constructive() {
        let proof = mk_const("And.intro");
        assert!(ProofAnalyzer::is_constructive(&proof));
    }
    #[test]
    fn test_proof_analyzer_classical() {
        let proof = mk_const("Classical.em");
        assert!(ProofAnalyzer::uses_classical(&proof));
        assert!(!ProofAnalyzer::is_constructive(&proof));
    }
    #[test]
    fn test_proof_analyzer_count_apps() {
        let f = mk_const("f");
        let a = mk_const("a");
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(ProofAnalyzer::count_applications(&app), 1);
    }
    #[test]
    fn test_proof_analyzer_count_apps_nested() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let app1 = Expr::App(Box::new(f), Box::new(a));
        let app2 = Expr::App(Box::new(app1), Box::new(b));
        assert_eq!(ProofAnalyzer::count_applications(&app2), 2);
    }
    #[test]
    fn test_contains_classical_sorry_false() {
        let proof = mk_const("True.intro");
        assert!(!contains_classical_sorry(&proof));
    }
    #[test]
    fn test_contains_classical_sorry_true() {
        let sorry = mk_const("sorry");
        assert!(contains_classical_sorry(&sorry));
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
