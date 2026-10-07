//! `DpllSolver` against a verbatim copy of its previous unit-propagation
//! code, which counted unassigned literals and kept the last one.

use super::DpllSolver;

/// The previous `clause_status`: (unset count, satisfied, last unset literal).
fn old_clause_status(clause: &[i32], assignment: &[Option<bool>]) -> (usize, bool, Option<i32>) {
    let mut unset = 0;
    let mut last_unset = None;
    for &lit in clause {
        let var = lit.unsigned_abs() as usize;
        match assignment[var] {
            None => {
                unset += 1;
                last_unset = Some(lit);
            }
            Some(val) => {
                if (lit > 0) == val {
                    return (0, true, None);
                }
            }
        }
    }
    (unset, false, last_unset)
}

/// The previous `dpll`, over the previous `clause_status`.
fn old_dpll(solver: &DpllSolver, assignment: &mut Vec<Option<bool>>) -> bool {
    let mut changed = true;
    while changed {
        changed = false;
        for clause in &solver.clauses {
            let (unset, sat, unit_lit) = old_clause_status(clause, assignment);
            if sat {
                continue;
            }
            if unset == 0 {
                return false;
            }
            if unset == 1 {
                let lit = unit_lit.expect("one unset literal was tracked");
                let var = lit.unsigned_abs() as usize;
                let val = lit > 0;
                if assignment[var] == Some(!val) {
                    return false;
                }
                assignment[var] = Some(val);
                changed = true;
            }
        }
    }
    if solver
        .clauses
        .iter()
        .all(|c| old_clause_status(c, assignment).1)
    {
        return true;
    }
    let var = match (1..=solver.n_vars).find(|&v| assignment[v].is_none()) {
        Some(v) => v,
        None => return false,
    };
    assignment[var] = Some(true);
    if old_dpll(solver, assignment) {
        return true;
    }
    assignment[var] = Some(false);
    if old_dpll(solver, assignment) {
        return true;
    }
    assignment[var] = None;
    false
}

/// The previous `solve`.
fn old_solve(solver: &DpllSolver) -> Option<Vec<bool>> {
    let mut assignment = vec![None::<bool>; solver.n_vars + 1];
    if old_dpll(solver, &mut assignment) {
        Some(
            assignment
                .into_iter()
                .skip(1)
                .map(|v| v.unwrap_or(false))
                .collect(),
        )
    } else {
        None
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn solve_equals_the_previous_solver_on_random_formulas() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut sat = 0usize;
    for _ in 0..3000 {
        let n_vars = 1 + rng.below(6) as usize;
        let mut solver = DpllSolver::new(n_vars);
        for _ in 0..rng.below(9) {
            // Clauses of 0 to 4 literals, repeats and complementary pairs included.
            let len = rng.below(5) as usize;
            let clause: Vec<i32> = (0..len)
                .map(|_| {
                    let var = 1 + rng.below(n_vars as u64) as i32;
                    if rng.below(2) == 0 {
                        var
                    } else {
                        -var
                    }
                })
                .collect();
            solver.add_clause(clause);
        }
        let new = solver.solve();
        assert_eq!(new, old_solve(&solver));
        if let Some(model) = new {
            sat += 1;
            for clause in &solver.clauses {
                assert!(clause
                    .iter()
                    .any(|&lit| model[lit.unsigned_abs() as usize - 1] == (lit > 0)));
            }
        }
    }
    assert!(sat > 0 && sat < 3000);
}

#[test]
fn clause_status_classifies_by_unassigned_literals() {
    let solver = DpllSolver::new(3);
    let a = [None, Some(true), Some(false), None];
    assert!(matches!(
        solver.clause_status(&[1, 3], &a),
        super::ClauseStatus::Satisfied
    ));
    assert!(matches!(
        solver.clause_status(&[-1, 2], &a),
        super::ClauseStatus::Falsified
    ));
    assert!(matches!(
        solver.clause_status(&[], &a),
        super::ClauseStatus::Falsified
    ));
    assert!(matches!(
        solver.clause_status(&[-1, -3, 2], &a),
        super::ClauseStatus::Unit(-3)
    ));
    assert!(matches!(
        solver.clause_status(&[3, -3], &a),
        super::ClauseStatus::Open
    ));
}
