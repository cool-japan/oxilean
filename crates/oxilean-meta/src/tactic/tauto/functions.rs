//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CdclSolver, Clause, Counterexample, DpllSolver, ExtDpllSolver, FormulaClass, FormulaStats,
    HornClause, HornSolver, Interpolant, IntuitTactic, LtlFormula, NdContext, NdStep,
    ProofTreeNode, PropFormula, PropProof, Sequent, TautoTactic, TruthTable, WatchedClauses,
};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_prop_formula_atoms() {
        let f = PropFormula::And(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Or(
                Box::new(PropFormula::Atom("q".to_string())),
                Box::new(PropFormula::Atom("p".to_string())),
            )),
        );
        let atoms = f.atoms();
        assert_eq!(atoms, vec!["p", "q"]);
    }
    #[test]
    fn test_truth_table_true() {
        let mut asgn = HashMap::new();
        asgn.insert("p".to_string(), true);
        asgn.insert("q".to_string(), false);
        let f = PropFormula::Or(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        assert!(TruthTable::evaluate(&f, &asgn));
    }
    #[test]
    fn test_truth_table_tautology() {
        let p = PropFormula::Atom("p".to_string());
        let not_p = PropFormula::Not(Box::new(PropFormula::Atom("p".to_string())));
        let f = PropFormula::Or(Box::new(p), Box::new(not_p));
        assert!(TruthTable::is_tautology(&f));
    }
    #[test]
    fn test_truth_table_not_tautology() {
        let f = PropFormula::And(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        assert!(!TruthTable::is_tautology(&f));
    }
    #[test]
    fn test_dpll_unit_propagate() {
        let mut clauses = vec![vec![1i32], vec![1, 2], vec![-2]];
        let ok = DpllSolver::unit_propagate(&mut clauses);
        assert!(ok);
    }
    #[test]
    fn test_dpll_solve() {
        let clauses = vec![vec![1i32]];
        assert!(DpllSolver::solve(clauses, 1));
        let unsat = vec![vec![1i32], vec![-1i32]];
        assert!(!DpllSolver::solve(unsat, 1));
    }
    #[test]
    fn test_tauto_tactic() {
        let tactic = TautoTactic::new();
        let p = PropFormula::Atom("p".to_string());
        let imp = PropFormula::Implies(Box::new(p.clone()), Box::new(p));
        assert!(tactic.is_tautology(&imp));
        let q = PropFormula::Atom("p".to_string());
        assert!(!tactic.is_tautology(&q));
        let p2 = PropFormula::Atom("p".to_string());
        let q2 = PropFormula::Atom("q".to_string());
        let r2 = PropFormula::Atom("r".to_string());
        let f = PropFormula::Implies(
            Box::new(PropFormula::Implies(
                Box::new(p2.clone()),
                Box::new(q2.clone()),
            )),
            Box::new(PropFormula::Implies(
                Box::new(PropFormula::Implies(
                    Box::new(q2.clone()),
                    Box::new(r2.clone()),
                )),
                Box::new(PropFormula::Implies(Box::new(p2), Box::new(r2))),
            )),
        );
        assert!(tactic.is_tautology(&f));
    }
    #[test]
    fn test_intuit_simple() {
        let tactic = IntuitTactic::new();
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Implies(Box::new(p.clone()), Box::new(p));
        assert!(tactic.prove_intuitionist(&f));
        let p2 = PropFormula::Atom("p".to_string());
        let nn_p = PropFormula::Not(Box::new(PropFormula::Not(Box::new(p2.clone()))));
        let dne = PropFormula::Implies(Box::new(nn_p), Box::new(p2));
        assert!(!tactic.prove_intuitionist(&dne));
        let p3 = PropFormula::Atom("p".to_string());
        let q3 = PropFormula::Atom("q".to_string());
        let conj_elim = PropFormula::Implies(
            Box::new(PropFormula::And(Box::new(p3.clone()), Box::new(q3))),
            Box::new(p3),
        );
        assert!(tactic.prove_intuitionist(&conj_elim));
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_prop_formula_is_literal() {
        assert!(PropFormula::Atom("p".to_string()).is_literal());
        assert!(PropFormula::True.is_literal());
        assert!(PropFormula::False.is_literal());
        let not_p = PropFormula::Not(Box::new(PropFormula::Atom("p".to_string())));
        assert!(not_p.is_literal());
        let nn_p = PropFormula::Not(Box::new(PropFormula::Not(Box::new(PropFormula::Atom(
            "p".to_string(),
        )))));
        assert!(!nn_p.is_literal());
        let and = PropFormula::And(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        assert!(!and.is_literal());
    }
    #[test]
    fn test_prop_formula_negate() {
        let p = PropFormula::Atom("p".to_string());
        let neg = p.negate();
        assert!(matches!(neg, PropFormula::Not(_)));
    }
    #[test]
    fn test_atoms_no_duplicates() {
        let f = PropFormula::And(
            Box::new(PropFormula::Atom("x".to_string())),
            Box::new(PropFormula::Or(
                Box::new(PropFormula::Atom("x".to_string())),
                Box::new(PropFormula::Atom("y".to_string())),
            )),
        );
        let atoms = f.atoms();
        assert_eq!(atoms.len(), 2);
    }
    #[test]
    fn test_atoms_iff() {
        let f = PropFormula::Iff(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        let atoms = f.atoms();
        assert_eq!(atoms, vec!["p", "q"]);
    }
    #[test]
    fn test_truth_table_constants() {
        let asgn: HashMap<String, bool> = HashMap::new();
        assert!(TruthTable::evaluate(&PropFormula::True, &asgn));
        assert!(!TruthTable::evaluate(&PropFormula::False, &asgn));
    }
    #[test]
    fn test_truth_table_implies_false_when_true_false() {
        let mut asgn = HashMap::new();
        asgn.insert("p".to_string(), true);
        asgn.insert("q".to_string(), false);
        let imp = PropFormula::Implies(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        assert!(!TruthTable::evaluate(&imp, &asgn));
    }
    #[test]
    fn test_truth_table_iff_same_atoms() {
        let iff_pp = PropFormula::Iff(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("p".to_string())),
        );
        assert!(TruthTable::is_tautology(&iff_pp));
    }
    #[test]
    fn test_truth_table_iff_not_tautology() {
        let iff_pq = PropFormula::Iff(
            Box::new(PropFormula::Atom("p".to_string())),
            Box::new(PropFormula::Atom("q".to_string())),
        );
        assert!(!TruthTable::is_tautology(&iff_pq));
    }
    #[test]
    fn test_truth_table_not_false_is_true() {
        let f = PropFormula::Not(Box::new(PropFormula::False));
        assert!(TruthTable::is_tautology(&f));
    }
    #[test]
    fn test_truth_table_missing_atom_defaults_false() {
        let asgn: HashMap<String, bool> = HashMap::new();
        let f = PropFormula::Atom("z".to_string());
        assert!(!TruthTable::evaluate(&f, &asgn));
    }
    #[test]
    fn test_dpll_to_cnf_true_is_sat() {
        let mut solver = DpllSolver::new();
        let clauses = solver.to_cnf(&PropFormula::True);
        let nv = solver.next_var as usize;
        assert!(DpllSolver::solve(clauses, nv));
    }
    #[test]
    fn test_dpll_to_cnf_false_is_unsat() {
        let mut solver = DpllSolver::new();
        let clauses = solver.to_cnf(&PropFormula::False);
        let nv = solver.next_var as usize;
        assert!(!DpllSolver::solve(clauses, nv));
    }
    #[test]
    fn test_dpll_unit_propagate_conflict() {
        let mut clauses = vec![vec![1i32], vec![-1i32]];
        let ok = DpllSolver::unit_propagate(&mut clauses);
        assert!(!ok);
    }
    #[test]
    fn test_dpll_new_starts_at_one() {
        let solver = DpllSolver::new();
        assert_eq!(solver.next_var, 1);
    }
    #[test]
    fn test_tauto_prove_true() {
        let tac = TautoTactic::new();
        assert!(tac.prove("True"));
        assert!(!tac.prove("False"));
    }
    #[test]
    fn test_tauto_prove_atom_impl_atom() {
        let tac = TautoTactic::new();
        assert!(tac.prove("p → p"));
    }
    #[test]
    fn test_tauto_prove_iff_self() {
        let tac = TautoTactic::new();
        assert!(tac.prove("p ↔ p"));
        assert!(tac.prove("True ↔ True"));
    }
    #[test]
    fn test_tauto_parse_or() {
        let tac = TautoTactic::new();
        assert!(tac.prove("True ∨ False"));
        assert!(tac.prove("False ∨ True"));
    }
    #[test]
    fn test_tauto_parse_and() {
        let tac = TautoTactic::new();
        assert!(tac.prove("True ∧ True"));
        assert!(!tac.prove("True ∧ False"));
    }
    #[test]
    fn test_tauto_max_atoms() {
        let tac = TautoTactic::new();
        assert_eq!(tac.max_atoms(), 20);
    }
    #[test]
    fn test_tauto_not_p_not_tautology() {
        let tac = TautoTactic::new();
        assert!(!tac.prove("¬p"));
        assert!(!tac.prove("not p"));
    }
    #[test]
    fn test_tauto_new_and_max_atoms_same() {
        let t1 = TautoTactic::new();
        let t2 = TautoTactic::new();
        assert_eq!(t1.max_atoms(), t2.max_atoms());
    }
    #[test]
    fn test_intuit_conjunction_elim_right() {
        let tac = IntuitTactic::new();
        let a = PropFormula::Atom("A".to_string());
        let b = PropFormula::Atom("B".to_string());
        let conj = PropFormula::And(Box::new(a), Box::new(b.clone()));
        let goal = PropFormula::Implies(Box::new(conj), Box::new(b));
        assert!(tac.prove_intuitionist(&goal));
    }
    #[test]
    fn test_intuit_or_intro_left() {
        let tac = IntuitTactic::new();
        let a = PropFormula::Atom("A".to_string());
        let b = PropFormula::Atom("B".to_string());
        let a_or_b = PropFormula::Or(Box::new(a.clone()), Box::new(b));
        let goal = PropFormula::Implies(Box::new(a), Box::new(a_or_b));
        assert!(tac.prove_intuitionist(&goal));
    }
    #[test]
    fn test_intuit_iff_self() {
        let tac = IntuitTactic::new();
        let p = PropFormula::Atom("p".to_string());
        let iff_pp = PropFormula::Iff(Box::new(p.clone()), Box::new(p));
        assert!(tac.prove_intuitionist(&iff_pp));
    }
    #[test]
    fn test_intuit_true_provable() {
        let tac = IntuitTactic::new();
        assert!(tac.prove_intuitionist(&PropFormula::True));
    }
    #[test]
    fn test_intuit_false_not_provable() {
        let tac = IntuitTactic::new();
        assert!(!tac.prove_intuitionist(&PropFormula::False));
    }
    #[test]
    fn test_intuit_or_a_a_implies_a() {
        let tac = IntuitTactic::new();
        let a = PropFormula::Atom("A".to_string());
        let a_or_a = PropFormula::Or(Box::new(a.clone()), Box::new(a.clone()));
        let goal = PropFormula::Implies(Box::new(a_or_a), Box::new(a));
        assert!(tac.prove_intuitionist(&goal));
    }
    #[test]
    fn test_intuit_new_works() {
        let t = IntuitTactic::new();
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Implies(Box::new(p.clone()), Box::new(p));
        assert!(t.prove_intuitionist(&f));
    }
    #[test]
    fn test_atoms_nested_implies() {
        let f = PropFormula::Implies(
            Box::new(PropFormula::Atom("a".to_string())),
            Box::new(PropFormula::Implies(
                Box::new(PropFormula::Atom("b".to_string())),
                Box::new(PropFormula::Atom("a".to_string())),
            )),
        );
        let atoms = f.atoms();
        assert_eq!(atoms.len(), 2);
    }
}
#[cfg(test)]
mod ltl_tests {
    use super::*;
    use crate::tactic::tauto::*;
    fn make_trace(atoms: &[(&str, bool)]) -> std::collections::HashMap<String, bool> {
        atoms.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }
    #[test]
    fn test_ltl_atom() {
        let f = LtlFormula::Atom("p".to_string());
        let trace = vec![make_trace(&[("p", true)])];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_next() {
        let f = LtlFormula::Next(Box::new(LtlFormula::Atom("p".to_string())));
        let trace = vec![make_trace(&[("p", false)]), make_trace(&[("p", true)])];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_globally() {
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        let trace = vec![
            make_trace(&[("p", true)]),
            make_trace(&[("p", true)]),
            make_trace(&[("p", true)]),
        ];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_globally_fails() {
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        let trace = vec![make_trace(&[("p", true)]), make_trace(&[("p", false)])];
        assert!(!f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_finally() {
        let f = LtlFormula::Finally(Box::new(LtlFormula::Atom("p".to_string())));
        let trace = vec![make_trace(&[("p", false)]), make_trace(&[("p", true)])];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_until() {
        let f = LtlFormula::Until(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        let trace = vec![
            make_trace(&[("p", true), ("q", false)]),
            make_trace(&[("p", true), ("q", false)]),
            make_trace(&[("p", false), ("q", true)]),
        ];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_ltl_depth() {
        let f = LtlFormula::Globally(Box::new(LtlFormula::Finally(Box::new(LtlFormula::Atom(
            "p".to_string(),
        )))));
        assert_eq!(f.depth(), 2);
    }
    #[test]
    fn test_sequent_atoms_in_ante() {
        let a = PropFormula::Atom("A".to_string());
        let b = PropFormula::Atom("B".to_string());
        let seq = Sequent::new(vec![a, b], vec![]);
        let atoms = seq.atoms_in_ante();
        assert_eq!(atoms.len(), 2);
    }
    #[test]
    fn test_simplify_not_not() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Not(Box::new(PropFormula::Not(Box::new(p.clone()))));
        let s = simplify_to_fixpoint(&f);
        assert!(formula_eq_ext(&s, &p));
    }
    #[test]
    fn test_simplify_false_and() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::And(Box::new(PropFormula::False), Box::new(p));
        let s = simplify_prop_ext(&f);
        assert!(formula_eq_ext(&s, &PropFormula::False));
    }
    #[test]
    fn test_dpll_three_vars_sat() {
        let mut solver = ExtDpllSolver::new(3);
        solver.add_clause(Clause::new(vec![(0, true), (1, true), (2, true)]));
        solver.add_clause(Clause::new(vec![(0, false), (1, true)]));
        solver.add_clause(Clause::new(vec![(1, false), (2, true)]));
        assert!(solver.solve().is_some());
    }
}
/// Resolve clause c1 (with +var) and c2 (with -var).
#[allow(dead_code)]
pub fn resolve_clause(c1: &Clause, c2: &Clause, var: usize) -> Option<Clause> {
    let has_pos = c1.literals.iter().any(|&(v, p)| v == var && p);
    let has_neg = c2.literals.iter().any(|&(v, p)| v == var && !p);
    if !has_pos || !has_neg {
        return None;
    }
    let mut lits: Vec<(usize, bool)> = c1
        .literals
        .iter()
        .filter(|&&(v, _)| v != var)
        .chain(c2.literals.iter().filter(|&&(v, _)| v != var))
        .copied()
        .collect();
    lits.sort_unstable();
    lits.dedup();
    Some(Clause::new(lits))
}
/// Resolution refutation: try to derive empty clause.
#[allow(dead_code)]
pub fn resolution_refute(clauses: &[Clause]) -> bool {
    let mut current: Vec<Clause> = clauses.to_vec();
    let mut changed = true;
    while changed {
        changed = false;
        let n = current.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let max_var = current
                    .iter()
                    .flat_map(|c| c.literals.iter().map(|&(v, _)| v))
                    .max()
                    .unwrap_or(0);
                for var in 0..=max_var {
                    if let Some(resolvent) = resolve_clause(&current[i], &current[j], var) {
                        if resolvent.literals.is_empty() {
                            return true;
                        }
                        let already = current.iter().any(|c| {
                            let mut a = c.literals.clone();
                            let mut b = resolvent.literals.clone();
                            a.sort_unstable();
                            b.sort_unstable();
                            a == b
                        });
                        if !already {
                            current.push(resolvent);
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    false
}
/// Check structural equality of two formulas.
#[allow(dead_code)]
pub fn formula_eq_ext(a: &PropFormula, b: &PropFormula) -> bool {
    match (a, b) {
        (PropFormula::Atom(x), PropFormula::Atom(y)) => x == y,
        (PropFormula::True, PropFormula::True) => true,
        (PropFormula::False, PropFormula::False) => true,
        (PropFormula::Not(p), PropFormula::Not(q)) => formula_eq_ext(p, q),
        (PropFormula::And(p1, q1), PropFormula::And(p2, q2)) => {
            formula_eq_ext(p1, p2) && formula_eq_ext(q1, q2)
        }
        (PropFormula::Or(p1, q1), PropFormula::Or(p2, q2)) => {
            formula_eq_ext(p1, p2) && formula_eq_ext(q1, q2)
        }
        (PropFormula::Implies(p1, q1), PropFormula::Implies(p2, q2)) => {
            formula_eq_ext(p1, p2) && formula_eq_ext(q1, q2)
        }
        (PropFormula::Iff(p1, q1), PropFormula::Iff(p2, q2)) => {
            formula_eq_ext(p1, p2) && formula_eq_ext(q1, q2)
        }
        _ => false,
    }
}
/// Evaluate a propositional formula under an assignment.
#[allow(dead_code)]
pub fn eval_formula_ext(f: &PropFormula, env: &std::collections::HashMap<String, bool>) -> bool {
    match f {
        PropFormula::Atom(a) => *env.get(a).unwrap_or(&false),
        PropFormula::True => true,
        PropFormula::False => false,
        PropFormula::Not(p) => !eval_formula_ext(p, env),
        PropFormula::And(p, q) => eval_formula_ext(p, env) && eval_formula_ext(q, env),
        PropFormula::Or(p, q) => eval_formula_ext(p, env) || eval_formula_ext(q, env),
        PropFormula::Implies(p, q) => !eval_formula_ext(p, env) || eval_formula_ext(q, env),
        PropFormula::Iff(p, q) => eval_formula_ext(p, env) == eval_formula_ext(q, env),
    }
}
/// Count satisfying models.
#[allow(dead_code)]
pub fn count_models_ext(formula: &PropFormula) -> usize {
    let mut atoms: Vec<String> = formula.atoms().into_iter().collect();
    atoms.sort();
    let n = atoms.len();
    let mut count = 0;
    for mask in 0..(1u64 << n) {
        let mut env = std::collections::HashMap::new();
        for (i, atom) in atoms.iter().enumerate() {
            env.insert(atom.clone(), (mask >> i) & 1 == 1);
        }
        if eval_formula_ext(formula, &env) {
            count += 1;
        }
    }
    count
}
/// Formula depth.
#[allow(dead_code)]
pub fn formula_depth_ext(f: &PropFormula) -> usize {
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => 0,
        PropFormula::Not(p) => 1 + formula_depth_ext(p),
        PropFormula::And(p, q)
        | PropFormula::Or(p, q)
        | PropFormula::Implies(p, q)
        | PropFormula::Iff(p, q) => 1 + formula_depth_ext(p).max(formula_depth_ext(q)),
    }
}
#[allow(dead_code)]
pub fn simplify_prop_ext(f: &PropFormula) -> PropFormula {
    match f {
        PropFormula::And(p, q) => {
            let sp = simplify_prop_ext(p);
            let sq = simplify_prop_ext(q);
            match (&sp, &sq) {
                (PropFormula::False, _) | (_, PropFormula::False) => PropFormula::False,
                (PropFormula::True, _) => sq,
                (_, PropFormula::True) => sp,
                _ if formula_eq_ext(&sp, &sq) => sp,
                _ => PropFormula::And(Box::new(sp), Box::new(sq)),
            }
        }
        PropFormula::Or(p, q) => {
            let sp = simplify_prop_ext(p);
            let sq = simplify_prop_ext(q);
            match (&sp, &sq) {
                (PropFormula::True, _) | (_, PropFormula::True) => PropFormula::True,
                (PropFormula::False, _) => sq,
                (_, PropFormula::False) => sp,
                _ if formula_eq_ext(&sp, &sq) => sp,
                _ => PropFormula::Or(Box::new(sp), Box::new(sq)),
            }
        }
        PropFormula::Implies(p, q) => {
            let sp = simplify_prop_ext(p);
            let sq = simplify_prop_ext(q);
            match (&sp, &sq) {
                (PropFormula::False, _) => PropFormula::True,
                (_, PropFormula::True) => PropFormula::True,
                (PropFormula::True, _) => sq,
                _ if formula_eq_ext(&sp, &sq) => PropFormula::True,
                _ => PropFormula::Implies(Box::new(sp), Box::new(sq)),
            }
        }
        PropFormula::Not(p) => {
            let sp = simplify_prop_ext(p);
            match &sp {
                PropFormula::True => PropFormula::False,
                PropFormula::False => PropFormula::True,
                PropFormula::Not(inner) => *inner.clone(),
                _ => PropFormula::Not(Box::new(sp)),
            }
        }
        PropFormula::Iff(p, q) => {
            let sp = simplify_prop_ext(p);
            let sq = simplify_prop_ext(q);
            if formula_eq_ext(&sp, &sq) {
                return PropFormula::True;
            }
            PropFormula::Iff(Box::new(sp), Box::new(sq))
        }
        other => other.clone(),
    }
}
#[allow(dead_code)]
pub fn simplify_to_fixpoint(f: &PropFormula) -> PropFormula {
    let mut current = f.clone();
    loop {
        let next = simplify_prop_ext(&current);
        if formula_eq_ext(&current, &next) {
            return current;
        }
        current = next;
    }
}
/// NNF transformation.
#[allow(dead_code)]
pub fn to_nnf_ext(f: &PropFormula) -> PropFormula {
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => f.clone(),
        PropFormula::Not(inner) => match inner.as_ref() {
            PropFormula::Not(p) => to_nnf_ext(p),
            PropFormula::And(p, q) => PropFormula::Or(
                Box::new(to_nnf_ext(&PropFormula::Not(p.clone()))),
                Box::new(to_nnf_ext(&PropFormula::Not(q.clone()))),
            ),
            PropFormula::Or(p, q) => PropFormula::And(
                Box::new(to_nnf_ext(&PropFormula::Not(p.clone()))),
                Box::new(to_nnf_ext(&PropFormula::Not(q.clone()))),
            ),
            PropFormula::True => PropFormula::False,
            PropFormula::False => PropFormula::True,
            other => PropFormula::Not(Box::new(to_nnf_ext(other))),
        },
        PropFormula::And(p, q) => {
            PropFormula::And(Box::new(to_nnf_ext(p)), Box::new(to_nnf_ext(q)))
        }
        PropFormula::Or(p, q) => PropFormula::Or(Box::new(to_nnf_ext(p)), Box::new(to_nnf_ext(q))),
        PropFormula::Implies(p, q) => PropFormula::Or(
            Box::new(to_nnf_ext(&PropFormula::Not(p.clone()))),
            Box::new(to_nnf_ext(q)),
        ),
        PropFormula::Iff(p, q) => {
            let fwd = PropFormula::Implies(p.clone(), q.clone());
            let bwd = PropFormula::Implies(q.clone(), p.clone());
            PropFormula::And(Box::new(to_nnf_ext(&fwd)), Box::new(to_nnf_ext(&bwd)))
        }
    }
}
#[allow(dead_code)]
pub fn classify_formula_ext(f: &PropFormula) -> FormulaClass {
    if f.atoms().len() > 20 {
        return FormulaClass::Unknown;
    }
    let mut atoms: Vec<String> = f.atoms().into_iter().collect();
    atoms.sort();
    let n = atoms.len();
    let mut all_true = true;
    let mut all_false = true;
    for mask in 0..(1u64 << n) {
        let mut env = std::collections::HashMap::new();
        for (i, atom) in atoms.iter().enumerate() {
            env.insert(atom.clone(), (mask >> i) & 1 == 1);
        }
        if eval_formula_ext(f, &env) {
            all_false = false;
        } else {
            all_true = false;
        }
    }
    if all_true {
        FormulaClass::Tautology
    } else if all_false {
        FormulaClass::Contradiction
    } else {
        FormulaClass::Satisfiable
    }
}
#[cfg(test)]
mod ext_tests_2 {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_ext_dpll_sat() {
        let mut s = ExtDpllSolver::new(2);
        s.add_clause(Clause::new(vec![(0, true), (1, true)]));
        s.add_clause(Clause::new(vec![(0, false), (1, true)]));
        assert!(s.solve().is_some());
    }
    #[test]
    fn test_ext_dpll_unsat() {
        let mut s = ExtDpllSolver::new(1);
        s.add_clause(Clause::new(vec![(0, true)]));
        s.add_clause(Clause::new(vec![(0, false)]));
        assert!(s.solve().is_none());
    }
    #[test]
    fn test_resolution_refute_pos_neg() {
        let c1 = Clause::new(vec![(0, true)]);
        let c2 = Clause::new(vec![(0, false)]);
        assert!(resolution_refute(&[c1, c2]));
    }
    #[test]
    fn test_sequent_axiom_ext() {
        let a = PropFormula::Atom("A".to_string());
        let seq = Sequent::new(vec![a.clone()], vec![a]);
        assert!(seq.is_axiom());
    }
    #[test]
    fn test_count_models_ext_tautology() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Or(Box::new(p.clone()), Box::new(PropFormula::Not(Box::new(p))));
        assert_eq!(count_models_ext(&f), 2);
    }
    #[test]
    fn test_simplify_implies_self_ext() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Implies(Box::new(p.clone()), Box::new(p));
        let s = simplify_prop_ext(&f);
        assert!(formula_eq_ext(&s, &PropFormula::True));
    }
    #[test]
    fn test_to_nnf_double_neg() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Not(Box::new(PropFormula::Not(Box::new(p.clone()))));
        let nnf = to_nnf_ext(&f);
        assert!(formula_eq_ext(&nnf, &p));
    }
    #[test]
    fn test_classify_contradiction_ext() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::And(Box::new(p.clone()), Box::new(PropFormula::Not(Box::new(p))));
        assert_eq!(classify_formula_ext(&f), FormulaClass::Contradiction);
    }
    #[test]
    fn test_classify_tautology_ext() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Or(Box::new(p.clone()), Box::new(PropFormula::Not(Box::new(p))));
        assert_eq!(classify_formula_ext(&f), FormulaClass::Tautology);
    }
    #[test]
    fn test_formula_depth_ext() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Not(Box::new(PropFormula::Not(Box::new(p))));
        assert_eq!(formula_depth_ext(&f), 2);
    }
    #[test]
    fn test_prop_proof_depth_ext() {
        let p = PropProof::ImplIntro("h".to_string(), Box::new(PropProof::Hyp("h".to_string())));
        assert_eq!(p.depth(), 1);
    }
    #[test]
    fn test_prop_proof_size_ext() {
        let p = PropProof::AndIntro(
            Box::new(PropProof::Hyp("h1".to_string())),
            Box::new(PropProof::Hyp("h2".to_string())),
        );
        assert_eq!(p.size(), 3);
    }
    #[test]
    fn test_simplify_fixpoint_not_not() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Not(Box::new(PropFormula::Not(Box::new(p.clone()))));
        let s = simplify_to_fixpoint(&f);
        assert!(formula_eq_ext(&s, &p));
    }
    #[test]
    fn test_sequent_atoms_in_ante_ext() {
        let a = PropFormula::Atom("A".to_string());
        let b = PropFormula::Atom("B".to_string());
        let seq = Sequent::new(vec![a, b], vec![]);
        assert_eq!(seq.atoms_in_ante().len(), 2);
    }
    #[test]
    fn test_clause_satisfied_ext() {
        let c = Clause::new(vec![(0, true)]);
        let assign = vec![Some(true)];
        assert!(c.is_satisfied_ext(&assign));
    }
    #[test]
    fn test_clause_falsified_ext() {
        let c = Clause::new(vec![(0, true)]);
        let assign = vec![Some(false)];
        assert!(c.is_falsified_ext(&assign));
    }
}
#[allow(dead_code)]
pub fn count_total_atoms(f: &PropFormula) -> usize {
    match f {
        PropFormula::Atom(_) => 1,
        PropFormula::True | PropFormula::False => 0,
        PropFormula::Not(p) => count_total_atoms(p),
        PropFormula::And(p, q)
        | PropFormula::Or(p, q)
        | PropFormula::Implies(p, q)
        | PropFormula::Iff(p, q) => count_total_atoms(p) + count_total_atoms(q),
    }
}
#[allow(dead_code)]
pub fn count_connectives_ext(f: &PropFormula) -> usize {
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => 0,
        PropFormula::Not(p) => 1 + count_connectives_ext(p),
        PropFormula::And(p, q)
        | PropFormula::Or(p, q)
        | PropFormula::Implies(p, q)
        | PropFormula::Iff(p, q) => 1 + count_connectives_ext(p) + count_connectives_ext(q),
    }
}
#[allow(dead_code)]
pub fn count_by_type(f: &PropFormula) -> (usize, usize, usize, usize, usize) {
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => (0, 0, 0, 0, 0),
        PropFormula::Not(p) => {
            let (a, b, c, d, e) = count_by_type(p);
            (a, b, c + 1, d, e)
        }
        PropFormula::And(p, q) => {
            let (a1, b1, c1, d1, e1) = count_by_type(p);
            let (a2, b2, c2, d2, e2) = count_by_type(q);
            (a1 + a2 + 1, b1 + b2, c1 + c2, d1 + d2, e1 + e2)
        }
        PropFormula::Or(p, q) => {
            let (a1, b1, c1, d1, e1) = count_by_type(p);
            let (a2, b2, c2, d2, e2) = count_by_type(q);
            (a1 + a2, b1 + b2 + 1, c1 + c2, d1 + d2, e1 + e2)
        }
        PropFormula::Implies(p, q) => {
            let (a1, b1, c1, d1, e1) = count_by_type(p);
            let (a2, b2, c2, d2, e2) = count_by_type(q);
            (a1 + a2, b1 + b2, c1 + c2, d1 + d2 + 1, e1 + e2)
        }
        PropFormula::Iff(p, q) => {
            let (a1, b1, c1, d1, e1) = count_by_type(p);
            let (a2, b2, c2, d2, e2) = count_by_type(q);
            (a1 + a2, b1 + b2, c1 + c2, d1 + d2, e1 + e2 + 1)
        }
    }
}
#[cfg(test)]
mod horn_and_final_tests {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_horn_fact_derivable() {
        let mut solver = HornSolver::new();
        solver.add_clause(HornClause::fact("p"));
        assert!(solver.is_derivable("p"));
    }
    #[test]
    fn test_horn_rule_derivable() {
        let mut solver = HornSolver::new();
        solver.add_clause(HornClause::fact("a"));
        solver.add_clause(HornClause::fact("b"));
        solver.add_clause(HornClause::rule("c", vec!["a", "b"]));
        assert!(solver.is_derivable("c"));
    }
    #[test]
    fn test_horn_not_derivable() {
        let mut solver = HornSolver::new();
        solver.add_clause(HornClause::fact("a"));
        solver.add_clause(HornClause::rule("c", vec!["a", "b"]));
        assert!(!solver.is_derivable("c"));
    }
    #[test]
    fn test_horn_chain() {
        let mut solver = HornSolver::new();
        solver.add_clause(HornClause::fact("a"));
        solver.add_clause(HornClause::rule("b", vec!["a"]));
        solver.add_clause(HornClause::rule("c", vec!["b"]));
        assert!(solver.is_derivable("c"));
    }
    #[test]
    fn test_interpolant_trivial() {
        let i = Interpolant::trivial("p");
        assert!(!i.is_trivial());
    }
    #[test]
    fn test_cdcl_add_clause() {
        let mut solver = CdclSolver::new(3);
        solver.add_clause(vec![(0, true), (1, false)]);
        assert_eq!(solver.num_clauses(), 1);
    }
    #[test]
    fn test_cdcl_learn() {
        let mut solver = CdclSolver::new(3);
        solver.learn_clause(vec![(2, true)]);
        assert_eq!(solver.num_clauses(), 1);
    }
    #[test]
    fn test_watched_clauses() {
        let mut wc = WatchedClauses::new(3);
        wc.add_clause(vec![(0, true), (1, false)]);
        assert!(wc.num_watched_by(0, true) > 0);
    }
    #[test]
    fn test_formula_stats_atom() {
        let p = PropFormula::Atom("p".to_string());
        let stats = FormulaStats::compute(&p);
        assert_eq!(stats.atom_count, 1);
        assert_eq!(stats.connective_count, 0);
    }
    #[test]
    fn test_formula_stats_and() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f = PropFormula::And(Box::new(p), Box::new(q));
        let stats = FormulaStats::compute(&f);
        assert_eq!(stats.and_count, 1);
        assert_eq!(stats.atom_count, 2);
    }
    #[test]
    fn test_count_total_atoms_nested() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f = PropFormula::Or(
            Box::new(PropFormula::And(Box::new(p.clone()), Box::new(q.clone()))),
            Box::new(p),
        );
        assert_eq!(count_total_atoms(&f), 3);
    }
    #[test]
    fn test_resolution_no_refute() {
        let c1 = Clause::new(vec![(0, true)]);
        let c2 = Clause::new(vec![(1, false)]);
        assert!(!resolution_refute(&[c1, c2]));
    }
    #[test]
    fn test_horn_is_fact() {
        let f = HornClause::fact("x");
        assert!(f.is_fact());
    }
    #[test]
    fn test_horn_is_rule() {
        let r = HornClause::rule("y", vec!["x"]);
        assert!(!r.is_fact());
        assert!(r.is_definite());
    }
}
/// Find a counterexample to a formula.
#[allow(dead_code)]
pub fn find_counterexample(f: &PropFormula) -> Option<Counterexample> {
    let mut atoms: Vec<String> = f.atoms().into_iter().collect();
    atoms.sort();
    let n = atoms.len();
    for mask in 0..(1u64 << n) {
        let mut env = std::collections::HashMap::new();
        let mut assignment = Vec::new();
        for (i, atom) in atoms.iter().enumerate() {
            let v = (mask >> i) & 1 == 1;
            env.insert(atom.clone(), v);
            assignment.push((atom.clone(), v));
        }
        if !eval_formula_ext(f, &env) {
            return Some(Counterexample::new(assignment, "formula".to_string()));
        }
    }
    None
}
/// Collect all subformulas of a formula.
#[allow(dead_code)]
pub fn collect_subformulas(f: &PropFormula) -> Vec<PropFormula> {
    let mut result = vec![f.clone()];
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => {}
        PropFormula::Not(p) => result.extend(collect_subformulas(p)),
        PropFormula::And(p, q)
        | PropFormula::Or(p, q)
        | PropFormula::Implies(p, q)
        | PropFormula::Iff(p, q) => {
            result.extend(collect_subformulas(p));
            result.extend(collect_subformulas(q));
        }
    }
    result
}
/// Check if formula `sub` is a subformula of `f`.
#[allow(dead_code)]
pub fn is_subformula(f: &PropFormula, sub: &PropFormula) -> bool {
    if formula_eq_ext(f, sub) {
        return true;
    }
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => false,
        PropFormula::Not(p) => is_subformula(p, sub),
        PropFormula::And(p, q)
        | PropFormula::Or(p, q)
        | PropFormula::Implies(p, q)
        | PropFormula::Iff(p, q) => is_subformula(p, sub) || is_subformula(q, sub),
    }
}
/// Replace all occurrences of `old` with `new_f` in `f`.
#[allow(dead_code)]
pub fn substitute_formula(f: &PropFormula, old: &PropFormula, new_f: &PropFormula) -> PropFormula {
    if formula_eq_ext(f, old) {
        return new_f.clone();
    }
    match f {
        PropFormula::Atom(_) | PropFormula::True | PropFormula::False => f.clone(),
        PropFormula::Not(p) => PropFormula::Not(Box::new(substitute_formula(p, old, new_f))),
        PropFormula::And(p, q) => PropFormula::And(
            Box::new(substitute_formula(p, old, new_f)),
            Box::new(substitute_formula(q, old, new_f)),
        ),
        PropFormula::Or(p, q) => PropFormula::Or(
            Box::new(substitute_formula(p, old, new_f)),
            Box::new(substitute_formula(q, old, new_f)),
        ),
        PropFormula::Implies(p, q) => PropFormula::Implies(
            Box::new(substitute_formula(p, old, new_f)),
            Box::new(substitute_formula(q, old, new_f)),
        ),
        PropFormula::Iff(p, q) => PropFormula::Iff(
            Box::new(substitute_formula(p, old, new_f)),
            Box::new(substitute_formula(q, old, new_f)),
        ),
    }
}
#[cfg(test)]
mod final_tauto_ext_tests {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_find_counterexample_non_tautology() {
        let p = PropFormula::Atom("p".to_string());
        let cx = find_counterexample(&p);
        assert!(cx.is_some());
    }
    #[test]
    fn test_find_counterexample_tautology() {
        let p = PropFormula::Atom("p".to_string());
        let f = PropFormula::Or(Box::new(p.clone()), Box::new(PropFormula::Not(Box::new(p))));
        assert!(find_counterexample(&f).is_none());
    }
    #[test]
    fn test_collect_subformulas() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f = PropFormula::And(Box::new(p), Box::new(q));
        let subs = collect_subformulas(&f);
        assert_eq!(subs.len(), 3);
    }
    #[test]
    fn test_is_subformula() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f = PropFormula::And(Box::new(p.clone()), Box::new(q));
        assert!(is_subformula(&f, &p));
    }
    #[test]
    fn test_substitute_formula() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f = PropFormula::And(Box::new(p.clone()), Box::new(p.clone()));
        let s = substitute_formula(&f, &p, &q);
        matches!(s, PropFormula::And(_, _));
    }
    #[test]
    fn test_nd_context_lookup() {
        let mut ctx = NdContext::new();
        let p = PropFormula::Atom("p".to_string());
        ctx.add_hyp("h".to_string(), p.clone());
        assert!(ctx.lookup("h").is_some());
        assert!(ctx.contains_formula(&p));
    }
    #[test]
    fn test_nd_context_remove() {
        let mut ctx = NdContext::new();
        ctx.add_hyp("h".to_string(), PropFormula::True);
        ctx.remove_hyp("h");
        assert_eq!(ctx.size(), 0);
    }
    #[test]
    fn test_nd_step_depth() {
        let step = NdStep::ModusPonens(
            Box::new(NdStep::Assumption("h".to_string())),
            Box::new(NdStep::Assumption("h2".to_string())),
        );
        assert_eq!(step.depth(), 1);
    }
    #[test]
    fn test_counterexample_get_value() {
        let cx = Counterexample::new(vec![("p".to_string(), true)], "p".to_string());
        assert_eq!(cx.get_value("p"), Some(true));
        assert_eq!(cx.get_value("q"), None);
    }
    #[test]
    fn test_ltl_not() {
        let f = LtlFormula::Not(Box::new(LtlFormula::Atom("p".to_string())));
        let trace = vec![{
            let mut m = std::collections::HashMap::new();
            m.insert("p".to_string(), false);
            m
        }];
        assert!(f.eval_on_trace(&trace, 0));
    }
    #[test]
    fn test_horn_derive_all() {
        let mut solver = HornSolver::new();
        solver.add_clause(HornClause::fact("x"));
        solver.add_clause(HornClause::rule("y", vec!["x"]));
        let derived = solver.derive_all();
        assert!(derived.contains("x"));
        assert!(derived.contains("y"));
    }
}
/// Canonical form: sort atoms alphabetically before hashing.
#[allow(dead_code)]
pub fn canonical_form(f: &PropFormula) -> String {
    match f {
        PropFormula::Atom(a) => a.clone(),
        PropFormula::True => "True".to_string(),
        PropFormula::False => "False".to_string(),
        PropFormula::Not(p) => format!("Not({})", canonical_form(p)),
        PropFormula::And(p, q) => {
            let cp = canonical_form(p);
            let cq = canonical_form(q);
            let (a, b) = if cp <= cq { (cp, cq) } else { (cq, cp) };
            format!("And({},{})", a, b)
        }
        PropFormula::Or(p, q) => {
            let cp = canonical_form(p);
            let cq = canonical_form(q);
            let (a, b) = if cp <= cq { (cp, cq) } else { (cq, cp) };
            format!("Or({},{})", a, b)
        }
        PropFormula::Implies(p, q) => {
            format!("Implies({},{})", canonical_form(p), canonical_form(q))
        }
        PropFormula::Iff(p, q) => {
            let cp = canonical_form(p);
            let cq = canonical_form(q);
            let (a, b) = if cp <= cq { (cp, cq) } else { (cq, cp) };
            format!("Iff({},{})", a, b)
        }
    }
}
/// Check logical equivalence using truth tables.
#[allow(dead_code)]
pub fn are_equivalent(f: &PropFormula, g: &PropFormula) -> bool {
    let equiv = PropFormula::Iff(Box::new(f.clone()), Box::new(g.clone()));
    classify_formula_ext(&equiv) == FormulaClass::Tautology
}
#[cfg(test)]
mod canonical_and_equiv_tests {
    use super::*;
    use crate::tactic::tauto::*;
    #[test]
    fn test_canonical_form_atom() {
        let p = PropFormula::Atom("p".to_string());
        assert_eq!(canonical_form(&p), "p");
    }
    #[test]
    fn test_canonical_form_and_commutative() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f1 = PropFormula::And(Box::new(p.clone()), Box::new(q.clone()));
        let f2 = PropFormula::And(Box::new(q), Box::new(p));
        assert_eq!(canonical_form(&f1), canonical_form(&f2));
    }
    #[test]
    fn test_are_equivalent_same() {
        let p = PropFormula::Atom("p".to_string());
        assert!(are_equivalent(&p, &p));
    }
    #[test]
    fn test_are_equivalent_demorgan() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        let f1 = PropFormula::Not(Box::new(PropFormula::And(
            Box::new(p.clone()),
            Box::new(q.clone()),
        )));
        let f2 = PropFormula::Or(
            Box::new(PropFormula::Not(Box::new(p))),
            Box::new(PropFormula::Not(Box::new(q))),
        );
        assert!(are_equivalent(&f1, &f2));
    }
    #[test]
    fn test_proof_tree_leaf() {
        let node = ProofTreeNode::leaf("p", "axiom");
        assert_eq!(node.depth(), 0);
        assert_eq!(node.size(), 1);
        assert_eq!(node.leaves(), 1);
    }
    #[test]
    fn test_proof_tree_inner() {
        let left = ProofTreeNode::leaf("p", "hyp");
        let right = ProofTreeNode::leaf("q", "hyp");
        let root = ProofTreeNode::inner("p∧q", "∧I", vec![left, right]);
        assert_eq!(root.depth(), 1);
        assert_eq!(root.size(), 3);
        assert_eq!(root.leaves(), 2);
    }
    #[test]
    fn test_are_not_equivalent() {
        let p = PropFormula::Atom("p".to_string());
        let q = PropFormula::Atom("q".to_string());
        assert!(!are_equivalent(&p, &q));
    }
    #[test]
    fn test_canonical_or_commutative() {
        let p = PropFormula::Atom("a".to_string());
        let q = PropFormula::Atom("b".to_string());
        let f1 = PropFormula::Or(Box::new(p.clone()), Box::new(q.clone()));
        let f2 = PropFormula::Or(Box::new(q), Box::new(p));
        assert_eq!(canonical_form(&f1), canonical_form(&f2));
    }
}
