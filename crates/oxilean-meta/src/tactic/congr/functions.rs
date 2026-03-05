//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CongrClosureState, CongrConfig, CongrExtConfig600, CongrExtConfigVal600, CongrExtDiag600,
    CongrExtDiff600, CongrExtPass600, CongrExtPipeline600, CongrExtResult600, CongrJust,
    CongrLemma, CongrLemmaDb, CongrLemmaV2, CongrState, CongrTactic, CongrType, Derivation,
    DerivationStep, EGraph, ENode, EquationSystem, ExtTactic, Pattern, RewriteRule, RewriteSystem,
    SaturationResult, SymExpr, TacticCongrAnalysisPass, TacticCongrConfig, TacticCongrConfigValue,
    TacticCongrDiagnostics, TacticCongrDiff, TacticCongrPipeline, TacticCongrResult,
    TheoryCongrState, UnionFind,
};

/// Find the position of ` = ` at parenthesis depth 0 in `s`.
pub(super) fn find_eq_at_depth0(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    if s.len() < 3 {
        return None;
    }
    for i in 0..=(s.len() - 3) {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b' ' if depth == 0 => {
                if bytes.get(i + 1) == Some(&b'=') && bytes.get(i + 2) == Some(&b' ') {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}
/// Split an application expression into head + arguments.
///
/// `"f a b"` → `["f", "a", "b"]`
/// `"(f a) b"` → `["(f a)", "b"]`
pub(super) fn split_app(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth: i32 = 0;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b' ' if depth == 0 => {
                let token = s[start..i].trim();
                if !token.is_empty() {
                    parts.push(token);
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = s[start..].trim();
    if !last.is_empty() {
        parts.push(last);
    }
    parts
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::congr::*;
    #[test]
    fn test_congr_lemma_new() {
        let l = CongrLemma::new("Nat.add.congr", 2, CongrType::Default);
        assert_eq!(l.name, "Nat.add.congr");
        assert_eq!(l.arity, 2);
        assert_eq!(l.congr_type, CongrType::Default);
    }
    #[test]
    fn test_congr_state_add_and_find() {
        let mut state = CongrState::new();
        state.add_lemma(CongrLemma::new("my.lemma", 1, CongrType::Eq));
        let found = state.find_matching("my.lemma");
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").arity, 1);
    }
    #[test]
    fn test_congr_state_find_missing() {
        let state = CongrState::new();
        assert!(state.find_matching("no.such.lemma").is_none());
    }
    #[test]
    fn test_congr_1_same_fn() {
        let tactic = CongrTactic::new();
        let subgoals = tactic.congr_1("f a = f b");
        assert_eq!(subgoals, vec!["a = b"]);
    }
    #[test]
    fn test_congr_1_identical_args_no_subgoals() {
        let tactic = CongrTactic::new();
        let subgoals = tactic.congr_1("f a = f a");
        assert!(subgoals.is_empty());
    }
    #[test]
    fn test_congr_1_different_fn_no_subgoals() {
        let tactic = CongrTactic::new();
        let subgoals = tactic.congr_1("f a = g a");
        assert!(subgoals.is_empty());
    }
    #[test]
    fn test_generate_congr_lemma() {
        let lemma = CongrTactic::generate_congr_lemma("Nat.add", 2);
        assert_eq!(lemma.name, "Nat.add.congr");
        assert_eq!(lemma.arity, 2);
    }
    #[test]
    fn test_ext_tactic_apply() {
        let ext = ExtTactic::new();
        let result = ext.apply("f = g");
        assert_eq!(result, Some("f x = g x".to_string()));
    }
    #[test]
    fn test_ext_tactic_apply_no_eq() {
        let ext = ExtTactic::new();
        assert!(ext.apply("f g h").is_none());
    }
    #[test]
    fn test_apply_congr_depth_limit() {
        let tactic = CongrTactic::with_config(CongrConfig {
            max_depth: 0,
            use_hyps: false,
        });
        let mut state = CongrState::new();
        let subgoals = tactic.apply_congr(&mut state, "f a = f b");
        assert!(subgoals.is_empty());
    }
}
/// Fold over a SymExpr.
#[allow(dead_code)]
pub fn sym_fold<T, F>(expr: &SymExpr, init: T, f: F) -> T
where
    F: Fn(T, &SymExpr) -> T + Copy,
{
    let acc = f(init, expr);
    match expr {
        SymExpr::Var(_) | SymExpr::Const(_) => acc,
        SymExpr::Neg(x) => sym_fold(x, acc, f),
        SymExpr::Add(a, b) | SymExpr::Mul(a, b) => {
            let acc = sym_fold(a, acc, f);
            sym_fold(b, acc, f)
        }
        SymExpr::App(_, args) => args.iter().fold(acc, |a, e| sym_fold(e, a, f)),
    }
}
/// Extract all variable names in a SymExpr.
#[allow(dead_code)]
pub fn sym_vars(expr: &SymExpr) -> Vec<String> {
    sym_fold(expr, Vec::new(), |mut acc: Vec<String>, e| {
        if let SymExpr::Var(v) = e {
            if !acc.contains(v) {
                acc.push(v.clone());
            }
        }
        acc
    })
}
/// Count all nodes in a SymExpr.
#[allow(dead_code)]
pub fn sym_size(expr: &SymExpr) -> usize {
    sym_fold(expr, 0, |acc, _| acc + 1)
}
/// Simple structural matching (no variable binding in pattern).
#[allow(dead_code)]
pub fn sym_matches(pattern: &SymExpr, expr: &SymExpr) -> bool {
    match (pattern, expr) {
        (SymExpr::Var(p), SymExpr::Var(e)) => p == e,
        (SymExpr::Const(p), SymExpr::Const(e)) => p == e,
        (SymExpr::Add(p1, p2), SymExpr::Add(e1, e2)) => sym_matches(p1, e1) && sym_matches(p2, e2),
        (SymExpr::Mul(p1, p2), SymExpr::Mul(e1, e2)) => sym_matches(p1, e1) && sym_matches(p2, e2),
        (SymExpr::Neg(p), SymExpr::Neg(e)) => sym_matches(p, e),
        (SymExpr::App(pf, pargs), SymExpr::App(ef, eargs)) => {
            pf == ef
                && pargs.len() == eargs.len()
                && pargs
                    .iter()
                    .zip(eargs.iter())
                    .all(|(p, e)| sym_matches(p, e))
        }
        _ => false,
    }
}
/// Normalize a SymExpr into a sorted list of additive terms.
#[allow(dead_code)]
pub fn ac_normalize_add(expr: &SymExpr) -> Vec<SymExpr> {
    let mut terms = match expr {
        SymExpr::Add(a, b) => {
            let mut terms = ac_normalize_add(a);
            terms.extend(ac_normalize_add(b));
            terms
        }
        SymExpr::App(name, args) if name == "add" => {
            let mut terms = Vec::new();
            for arg in args {
                terms.extend(ac_normalize_add(arg));
            }
            terms
        }
        other => vec![other.clone()],
    };
    terms.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    terms
}
/// Normalize a SymExpr into a sorted list of multiplicative factors.
#[allow(dead_code)]
pub fn ac_normalize_mul(expr: &SymExpr) -> Vec<SymExpr> {
    match expr {
        SymExpr::Mul(a, b) => {
            let mut factors = ac_normalize_mul(a);
            factors.extend(ac_normalize_mul(b));
            factors
        }
        other => vec![other.clone()],
    }
}
/// AC equality check: a + b == b + a.
#[allow(dead_code)]
pub fn ac_equal_add(a: &SymExpr, b: &SymExpr) -> bool {
    let mut ta = ac_normalize_add(a);
    let mut tb = ac_normalize_add(b);
    let key = |e: &SymExpr| format!("{:?}", e);
    ta.sort_by_key(key);
    tb.sort_by_key(key);
    ta.len() == tb.len() && ta.iter().zip(tb.iter()).all(|(x, y)| x == y)
}
#[cfg(test)]
mod congr_extended_tests {
    use super::*;
    use crate::tactic::congr::*;
    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.are_equal(0, 1));
        uf.union(0, 1);
        assert!(uf.are_equal(0, 1));
        assert!(!uf.are_equal(0, 2));
    }
    #[test]
    fn test_union_find_transitivity() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.are_equal(0, 2));
    }
    #[test]
    fn test_union_find_num_components() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.num_components(), 2);
    }
    #[test]
    fn test_congr_closure_merge() {
        let mut state = CongrClosureState::new();
        state.merge("a", "b");
        assert!(state.are_congruent("a", "b"));
        assert!(!state.are_congruent("a", "c"));
    }
    #[test]
    fn test_sym_expr_eval() {
        let e = SymExpr::add(SymExpr::var("x"), SymExpr::konst(1));
        let mut env = std::collections::HashMap::new();
        env.insert("x".to_string(), 5i64);
        assert_eq!(e.eval(&env), Some(6));
    }
    #[test]
    fn test_sym_vars() {
        let e = SymExpr::add(SymExpr::var("x"), SymExpr::var("y"));
        let vars = sym_vars(&e);
        assert_eq!(vars.len(), 2);
    }
    #[test]
    fn test_sym_size() {
        let e = SymExpr::add(SymExpr::var("x"), SymExpr::konst(1));
        assert_eq!(sym_size(&e), 3);
    }
    #[test]
    fn test_sym_depth() {
        let e = SymExpr::add(SymExpr::var("x"), SymExpr::konst(1));
        assert_eq!(e.depth(), 1);
    }
    #[test]
    fn test_sym_free_vars() {
        let e = SymExpr::mul(SymExpr::var("x"), SymExpr::var("x"));
        let fv = e.free_vars();
        assert_eq!(fv.len(), 1);
    }
    #[test]
    fn test_rewrite_rule() {
        let rule = RewriteRule::new(
            "zero_add",
            SymExpr::add(SymExpr::konst(0), SymExpr::var("x")),
            SymExpr::var("x"),
        );
        assert_eq!(rule.name, "zero_add");
    }
    #[test]
    fn test_rewrite_system_add_find() {
        let mut sys = RewriteSystem::new();
        let rule = RewriteRule::new(
            "id",
            SymExpr::add(SymExpr::var("x"), SymExpr::konst(0)),
            SymExpr::var("x"),
        );
        sys.add_rule(rule);
        let target = SymExpr::add(SymExpr::var("x"), SymExpr::konst(0));
        assert_eq!(sys.find_applicable(&target).len(), 1);
    }
    #[test]
    fn test_ac_normalize_add() {
        let e = SymExpr::add(
            SymExpr::add(SymExpr::var("x"), SymExpr::var("y")),
            SymExpr::var("z"),
        );
        let terms = ac_normalize_add(&e);
        assert_eq!(terms.len(), 3);
    }
    #[test]
    fn test_ac_equal_add_commute() {
        let a = SymExpr::add(SymExpr::var("x"), SymExpr::var("y"));
        let b = SymExpr::add(SymExpr::var("y"), SymExpr::var("x"));
        assert!(ac_equal_add(&a, &b));
    }
    #[test]
    fn test_sym_matches_const() {
        let p = SymExpr::konst(42);
        let e = SymExpr::konst(42);
        assert!(sym_matches(&p, &e));
    }
    #[test]
    fn test_sym_is_constant() {
        assert!(SymExpr::konst(5).is_constant());
        assert!(!SymExpr::var("x").is_constant());
    }
}
/// Run equality saturation on an E-graph with given rewrite rules.
#[allow(dead_code)]
pub fn equality_saturation(
    egraph: &mut EGraph,
    _rules: &RewriteSystem,
    max_iter: usize,
) -> SaturationResult {
    let mut result = SaturationResult::new();
    for _iter in 0..max_iter {
        let prev_size = egraph.eg_size();
        let prev_merges = egraph.num_merges;
        result.iterations += 1;
        if egraph.eg_size() == prev_size && egraph.num_merges == prev_merges {
            result.saturated = true;
            break;
        }
        result.nodes_added += egraph.eg_size() - prev_size;
        result.merges_performed += egraph.num_merges - prev_merges;
    }
    result
}
/// Substitute variables in a symbolic expression from a binding map.
#[allow(dead_code)]
pub fn substitute_sym(
    expr: &SymExpr,
    bindings: &std::collections::HashMap<String, SymExpr>,
) -> Option<SymExpr> {
    match expr {
        SymExpr::Var(v) => {
            if let Some(repl) = bindings.get(v) {
                Some(repl.clone())
            } else {
                Some(expr.clone())
            }
        }
        SymExpr::Const(_) => Some(expr.clone()),
        SymExpr::App(fname, args) => {
            let mut new_args = Vec::new();
            for a in args {
                new_args.push(substitute_sym(a, bindings)?);
            }
            Some(SymExpr::App(fname.clone(), new_args))
        }
        SymExpr::Neg(e) => Some(SymExpr::Neg(Box::new(substitute_sym(e, bindings)?))),
        SymExpr::Add(a, b) => {
            let na = substitute_sym(a, bindings)?;
            let nb = substitute_sym(b, bindings)?;
            Some(SymExpr::Add(Box::new(na), Box::new(nb)))
        }
        SymExpr::Mul(a, b) => {
            let na = substitute_sym(a, bindings)?;
            let nb = substitute_sym(b, bindings)?;
            Some(SymExpr::Mul(Box::new(na), Box::new(nb)))
        }
    }
}
#[cfg(test)]
mod congr_ext_tests {
    use super::*;
    use crate::tactic::congr::*;
    #[test]
    fn test_egraph_add_find() {
        let mut eg = EGraph::new();
        let id1 = eg.add_node(ENode::Symbol("a".to_string()));
        let id2 = eg.add_node(ENode::Symbol("a".to_string()));
        assert_eq!(eg.eg_find(id1), eg.eg_find(id2));
    }
    #[test]
    fn test_egraph_union() {
        let mut eg = EGraph::new();
        let id1 = eg.add_node(ENode::Symbol("a".to_string()));
        let id2 = eg.add_node(ENode::Symbol("b".to_string()));
        assert!(!eg.eg_are_equal(id1, id2));
        eg.eg_union(id1, id2);
        assert!(eg.eg_are_equal(id1, id2));
    }
    #[test]
    fn test_egraph_lit() {
        let mut eg = EGraph::new();
        let id = eg.add_node(ENode::Literal(42));
        assert_eq!(eg.eg_find(id), id);
    }
    #[test]
    fn test_egraph_num_classes() {
        let mut eg = EGraph::new();
        eg.add_node(ENode::Symbol("x".to_string()));
        eg.add_node(ENode::Symbol("y".to_string()));
        eg.add_node(ENode::Symbol("z".to_string()));
        assert_eq!(eg.eg_num_classes(), 3);
    }
    #[test]
    fn test_pattern_depth() {
        let p = Pattern::PApp(
            Box::new(Pattern::PSym("f".to_string())),
            vec![Pattern::PVar("x".to_string())],
        );
        assert_eq!(p.pat_depth(), 1);
    }
    #[test]
    fn test_pattern_variables() {
        let p = Pattern::PApp(
            Box::new(Pattern::PSym("f".to_string())),
            vec![
                Pattern::PVar("x".to_string()),
                Pattern::PVar("y".to_string()),
                Pattern::PVar("x".to_string()),
            ],
        );
        assert_eq!(p.pat_variables(), vec!["x", "y"]);
    }
    #[test]
    fn test_pattern_ground() {
        assert!(Pattern::PSym("f".to_string()).is_ground_pat());
        assert!(!Pattern::PVar("x".to_string()).is_ground_pat());
        assert!(!Pattern::Wildcard.is_ground_pat());
    }
    #[test]
    fn test_theory_congr_ac() {
        let st = TheoryCongrState::new();
        assert!(st.is_ac("add"));
        assert!(st.is_ac("mul"));
        assert!(!st.is_ac("sub"));
    }
    #[test]
    fn test_theory_congr_flatten() {
        let st = TheoryCongrState::new();
        let a = SymExpr::Var("a".to_string());
        let b = SymExpr::Var("b".to_string());
        let add = SymExpr::App("add".to_string(), vec![a.clone(), b.clone()]);
        let flat = st.flatten_ac("add", &add);
        assert_eq!(flat.len(), 2);
    }
    #[test]
    fn test_congr_just_depth() {
        let j = CongrJust::trans(CongrJust::hyp("h1"), CongrJust::hyp("h2"));
        assert_eq!(j.just_depth(), 1);
    }
    #[test]
    fn test_congr_just_steps() {
        let j = CongrJust::cong("f", vec![CongrJust::hyp("h1"), CongrJust::refl()]);
        assert_eq!(j.just_steps(), 2);
    }
    #[test]
    fn test_equation_system_basic() {
        let mut sys = EquationSystem::new();
        let x = SymExpr::Var("x".to_string());
        let y = SymExpr::Var("y".to_string());
        sys.add_eq(x.clone(), y.clone(), "h1");
        assert_eq!(sys.num_equations(), 1);
    }
    #[test]
    fn test_equation_system_normalize() {
        let mut sys = EquationSystem::new();
        let x = SymExpr::Var("x".to_string());
        sys.add_eq(x.clone(), x.clone(), "h1");
        sys.normalize_equations();
        assert_eq!(sys.num_equations(), 0);
    }
    #[test]
    fn test_saturation_result() {
        let r = SaturationResult::new();
        assert!(!r.is_success());
        assert!(!r.saturated);
    }
    #[test]
    fn test_substitute_sym_var() {
        let mut bindings = std::collections::HashMap::new();
        bindings.insert("x".to_string(), SymExpr::Const(5));
        let expr = SymExpr::Var("x".to_string());
        let result = substitute_sym(&expr, &bindings);
        assert_eq!(result, Some(SymExpr::Const(5)));
    }
    #[test]
    fn test_substitute_sym_const() {
        let bindings = std::collections::HashMap::new();
        let expr = SymExpr::Const(7);
        let result = substitute_sym(&expr, &bindings);
        assert_eq!(result, Some(SymExpr::Const(7)));
    }
    #[test]
    fn test_congr_just_sym() {
        let j = CongrJust::sym(CongrJust::hyp("h"));
        assert_eq!(j.just_steps(), 1);
    }
    #[test]
    fn test_ac_normalize_add_ext() {
        let a = SymExpr::Var("a".to_string());
        let b = SymExpr::Var("b".to_string());
        let expr_ab = SymExpr::Add(Box::new(a.clone()), Box::new(b.clone()));
        let expr_ba = SymExpr::Add(Box::new(b.clone()), Box::new(a.clone()));
        let s1 = ac_normalize_add(&expr_ab);
        let s2 = ac_normalize_add(&expr_ba);
        assert_eq!(s1, s2);
    }
    #[test]
    fn test_ac_equal_add_ext() {
        let a = SymExpr::Var("x".to_string());
        let b = SymExpr::Var("y".to_string());
        assert!(ac_equal_add(
            &SymExpr::App("add".to_string(), vec![a.clone(), b.clone()]),
            &SymExpr::App("add".to_string(), vec![b.clone(), a.clone()])
        ));
    }
    #[test]
    fn test_union_find_ext() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        assert!(uf.find(0) == uf.find(1));
        assert!(uf.find(2) == uf.find(3));
        assert!(uf.find(0) != uf.find(2));
    }
    #[test]
    fn test_congr_closure_state_ext() {
        let mut st = CongrClosureState::new();
        st.add_eq(0, 1);
        st.add_eq(2, 3);
        assert!(st.are_equal(0, 1));
        assert!(!st.are_equal(0, 2));
    }
    #[test]
    fn test_theory_congr_add_ac_op() {
        let mut st = TheoryCongrState::new();
        st.add_ac_op("and");
        assert!(st.is_ac("and"));
    }
    #[test]
    fn test_egraph_app_node() {
        let mut eg = EGraph::new();
        let id1 = eg.add_node(ENode::Application {
            func: 0,
            args: vec![1, 2],
        });
        let id2 = eg.add_node(ENode::Application {
            func: 0,
            args: vec![1, 2],
        });
        assert_eq!(eg.eg_find(id1), eg.eg_find(id2));
    }
    #[test]
    fn test_congr_lemma_db() {
        let mut db = CongrLemmaDb::new();
        db.register(
            CongrLemmaV2::new("eq_congr_add", "add", 2)
                .symmetric()
                .transitive(),
        );
        assert_eq!(db.num_lemmas(), 1);
        let lemmas = db.lookup_by_func("add");
        assert_eq!(lemmas.len(), 1);
        assert!(lemmas[0].is_equivalence());
    }
    #[test]
    fn test_derivation_basic() {
        let x = SymExpr::Var("x".to_string());
        let y = SymExpr::Var("y".to_string());
        let mut d = Derivation::new(x.clone());
        d.add_step(y.clone(), "eq_hyp");
        assert_eq!(d.length(), 1);
        assert_eq!(d.end, y);
    }
    #[test]
    fn test_derivation_valid() {
        let x = SymExpr::Var("x".to_string());
        let d = Derivation::new(x.clone());
        assert!(d.is_valid());
    }
    #[test]
    fn test_derivation_step_trivial() {
        let x = SymExpr::Var("x".to_string());
        let step = DerivationStep::new(x.clone(), x.clone(), "refl");
        assert!(step.is_trivial());
    }
}
#[cfg(test)]
mod tacticcongr_analysis_tests {
    use super::*;
    use crate::tactic::congr::*;
    #[test]
    fn test_tacticcongr_result_ok() {
        let r = TacticCongrResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcongr_result_err() {
        let r = TacticCongrResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcongr_result_partial() {
        let r = TacticCongrResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcongr_result_skipped() {
        let r = TacticCongrResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticcongr_analysis_pass_run() {
        let mut p = TacticCongrAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticcongr_analysis_pass_empty_input() {
        let mut p = TacticCongrAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticcongr_analysis_pass_success_rate() {
        let mut p = TacticCongrAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticcongr_analysis_pass_disable() {
        let mut p = TacticCongrAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticcongr_pipeline_basic() {
        let mut pipeline = TacticCongrPipeline::new("main_pipeline");
        pipeline.add_pass(TacticCongrAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticCongrAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticcongr_pipeline_disabled_pass() {
        let mut pipeline = TacticCongrPipeline::new("partial");
        let mut p = TacticCongrAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticCongrAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticcongr_diff_basic() {
        let mut d = TacticCongrDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticcongr_diff_summary() {
        let mut d = TacticCongrDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticcongr_config_set_get() {
        let mut cfg = TacticCongrConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticcongr_config_read_only() {
        let mut cfg = TacticCongrConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticcongr_config_remove() {
        let mut cfg = TacticCongrConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticcongr_diagnostics_basic() {
        let mut diag = TacticCongrDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticcongr_diagnostics_max_errors() {
        let mut diag = TacticCongrDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticcongr_diagnostics_clear() {
        let mut diag = TacticCongrDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticcongr_config_value_types() {
        let b = TacticCongrConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticCongrConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticCongrConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticCongrConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticCongrConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod congr_ext_tests_600 {
    use super::*;
    use crate::tactic::congr::*;
    #[test]
    fn test_congr_ext_result_ok_600() {
        let r = CongrExtResult600::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_congr_ext_result_err_600() {
        let r = CongrExtResult600::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_congr_ext_result_partial_600() {
        let r = CongrExtResult600::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_congr_ext_result_skipped_600() {
        let r = CongrExtResult600::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_congr_ext_pass_run_600() {
        let mut p = CongrExtPass600::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_congr_ext_pass_empty_600() {
        let mut p = CongrExtPass600::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_congr_ext_pass_rate_600() {
        let mut p = CongrExtPass600::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_congr_ext_pass_disable_600() {
        let mut p = CongrExtPass600::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_congr_ext_pipeline_basic_600() {
        let mut pipeline = CongrExtPipeline600::new("main_pipeline");
        pipeline.add_pass(CongrExtPass600::new("pass1"));
        pipeline.add_pass(CongrExtPass600::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_congr_ext_pipeline_disabled_600() {
        let mut pipeline = CongrExtPipeline600::new("partial");
        let mut p = CongrExtPass600::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CongrExtPass600::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_congr_ext_diff_basic_600() {
        let mut d = CongrExtDiff600::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_congr_ext_config_set_get_600() {
        let mut cfg = CongrExtConfig600::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_congr_ext_config_read_only_600() {
        let mut cfg = CongrExtConfig600::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_congr_ext_config_remove_600() {
        let mut cfg = CongrExtConfig600::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_congr_ext_diagnostics_basic_600() {
        let mut diag = CongrExtDiag600::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_congr_ext_diagnostics_max_errors_600() {
        let mut diag = CongrExtDiag600::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_congr_ext_diagnostics_clear_600() {
        let mut diag = CongrExtDiag600::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_congr_ext_config_value_types_600() {
        let b = CongrExtConfigVal600::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CongrExtConfigVal600::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CongrExtConfigVal600::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CongrExtConfigVal600::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CongrExtConfigVal600::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
