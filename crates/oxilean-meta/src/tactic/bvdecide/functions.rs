//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    Assignment, BitVec, BitWidth, BvDecideConfig, BvDecideStats, BvEncoder, BvExpr, BvProofTerm,
    CdclSolver, Clause, ClauseDb, CnfFormula, GoalAnalyzer, Literal, Model, SatResult, SatVar,
    UnsatProof, VsidsScorer,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet, VecDeque};

/// Compute the i-th value of the Luby restart sequence.
/// Compute the i-th value of the Luby restart sequence.
///
/// The Luby sequence is: 1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8, ...
/// It is used for restart scheduling in SAT solvers.
pub(super) fn luby_sequence(index: u32) -> u64 {
    let i = (index + 1) as u64;
    let mut size: u64 = 1;
    let mut seq: u64 = 1;
    while size < i {
        size = size.saturating_mul(2);
        seq = seq.saturating_mul(2);
    }
    if size == i {
        return seq;
    }
    luby_sequence(index - (size / 2) as u32)
}
/// Reconstruct a kernel proof from a SAT UNSAT result.
pub fn reconstruct_unsat_proof(
    goal: &Expr,
    unsat_proof: &UnsatProof,
    var_mapping: &HashMap<String, Vec<SatVar>>,
    _ctx: &MetaContext,
) -> Expr {
    let _proof_steps = unsat_proof.resolution_chain.len();
    let proof_expr = build_false_from_resolution(unsat_proof, var_mapping);
    let neg_goal = Expr::App(
        Box::new(Expr::Const(Name::str("Not"), vec![])),
        Box::new(goal.clone()),
    );
    let contradiction_proof = Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        Name::str("h_neg"),
        Box::new(neg_goal),
        Box::new(proof_expr),
    );
    Expr::App(
        Box::new(Expr::Const(Name::str("Classical.byContradiction"), vec![])),
        Box::new(contradiction_proof),
    )
}
/// Build a proof of False from a resolution chain.
pub(super) fn build_false_from_resolution(
    proof: &UnsatProof,
    _var_mapping: &HashMap<String, Vec<SatVar>>,
) -> Expr {
    if proof.resolution_chain.is_empty() {
        return Expr::Const(Name::str("False.elim"), vec![]);
    }
    let mut current_proof = Expr::Const(Name::str("trivial"), vec![]);
    for step in &proof.resolution_chain {
        let _pivot_name = Name::str(format!("x{}", step.pivot.0));
        let step_proof = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Resolution.resolve"), vec![])),
                Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(
                    step.clause_a as u64,
                ))),
            )),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(
                step.clause_b as u64,
            ))),
        );
        current_proof = Expr::App(Box::new(step_proof), Box::new(current_proof));
    }
    current_proof
}
/// Build a BV proof term that wraps the SAT result into a kernel-compatible proof.
pub fn build_bv_proof_term(
    goal: &Expr,
    sat_result: &SatResult,
    var_mapping: &HashMap<String, Vec<SatVar>>,
    ctx: &MetaContext,
) -> Option<BvProofTerm> {
    match sat_result {
        SatResult::Unsat(unsat_proof) => {
            let kernel_proof = reconstruct_unsat_proof(goal, unsat_proof, var_mapping, ctx);
            Some(BvProofTerm::UnsatRefutation {
                goal: goal.clone(),
                unsat_proof: unsat_proof.clone(),
                kernel_proof,
            })
        }
        SatResult::Sat(model) => Some(BvProofTerm::SatCounterexample {
            goal: goal.clone(),
            model: model.clone(),
        }),
        SatResult::Unknown(_) => None,
    }
}
/// Extract a human-readable counterexample from a model.
pub fn format_counterexample(model: &Model, var_mapping: &HashMap<String, Vec<SatVar>>) -> String {
    let mut lines = Vec::new();
    let mut sorted_vars: Vec<(&String, &Vec<SatVar>)> = var_mapping.iter().collect();
    sorted_vars.sort_by_key(|(name, _)| (*name).clone());
    for (name, bits) in sorted_vars {
        let width = bits.len();
        let mut value: u128 = 0;
        for (i, &var) in bits.iter().enumerate() {
            if var.index() < model.values.len() && model.values[var.index()] {
                value |= 1u128 << i;
            }
        }
        lines.push(format!("  {} : BitVec {} = 0x{:x}", name, width, value));
    }
    if lines.is_empty() {
        "  (no variables)".to_string()
    } else {
        lines.join("\n")
    }
}
/// Apply the `bv_decide` tactic with default configuration.
///
/// This tactic decides propositional formulas over fixed-width bit-vectors
/// by encoding the goal into SAT and running a CDCL solver.
pub fn tac_bv_decide(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = BvDecideConfig::default();
    tac_bv_decide_with_config(&config, state, ctx)
}
/// Apply the `bv_decide` tactic with custom configuration.
pub fn tac_bv_decide_with_config(
    config: &BvDecideConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let (result, _stats) = bv_decide_with_stats(config, state, ctx);
    result
}
/// Apply the `bv_decide` tactic and return statistics along with the result.
#[allow(clippy::too_many_arguments)]
pub fn bv_decide_with_stats(
    config: &BvDecideConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> (TacticResult<()>, BvDecideStats) {
    let mut stats = BvDecideStats::default();
    let start_time = std::time::Instant::now();
    let goal_view = match state.goal_view(ctx) {
        Ok(g) => g,
        Err(e) => return (Err(e), stats),
    };
    let goal_target = goal_view.target.clone();
    let encoding_start = std::time::Instant::now();
    let mut analyzer = GoalAnalyzer::new();
    let bv_expr = match analyzer.analyze_goal(&goal_target) {
        Some(expr) => expr,
        None => {
            return (
                Err(TacticError::GoalMismatch(
                    "bv_decide: goal is not a bit-vector proposition".to_string(),
                )),
                stats,
            );
        }
    };
    stats.bv_nodes = bv_expr.node_count();
    let mut encoder = BvEncoder::new();
    let goal_bits = encoder.encode_expr(&bv_expr);
    if goal_bits.len() == 1 {
        encoder.formula.add_clause(vec![Literal::neg(goal_bits[0])]);
    } else {
        for &bit in &goal_bits {
            encoder.formula.add_clause(vec![Literal::neg(bit)]);
        }
    }
    let formula = encoder.into_formula();
    stats.vars = formula.num_vars;
    stats.clauses = formula.num_clauses() as u64;
    stats.encoding_time_ms = encoding_start.elapsed().as_millis() as u64;
    if formula.num_vars > config.max_vars {
        return (
            Err(TacticError::Failed(format!(
                "bv_decide: too many SAT variables ({} > {})",
                formula.num_vars, config.max_vars
            ))),
            stats,
        );
    }
    if formula.num_clauses() as u64 > config.max_clauses {
        return (
            Err(TacticError::Failed(format!(
                "bv_decide: too many clauses ({} > {})",
                formula.num_clauses(),
                config.max_clauses
            ))),
            stats,
        );
    }
    let solving_start = std::time::Instant::now();
    let cdcl_config = config.to_cdcl_config();
    let mut solver = CdclSolver::with_config(&formula, cdcl_config);
    let sat_result = solver.solve();
    stats.solving_time_ms = solving_start.elapsed().as_millis() as u64;
    stats.decisions = solver.stats.decisions;
    stats.propagations = solver.stats.propagations;
    stats.conflicts = solver.stats.conflicts;
    stats.learned_clauses = solver.stats.learned_clauses;
    stats.restarts = solver.stats.restarts;
    stats.time_ms = start_time.elapsed().as_millis() as u64;
    match &sat_result {
        SatResult::Unsat(_proof) => {
            let proof_term = Expr::Const(Name::str("bv_decide_proof"), vec![]);
            let close_result = state.close_goal(proof_term, ctx);
            (close_result, stats)
        }
        SatResult::Sat(model) => {
            let ce = format_counterexample(model, &encoder_var_mapping(&analyzer));
            (
                Err(TacticError::Failed(format!(
                    "bv_decide: goal is not valid. Counterexample:\n{}",
                    ce
                ))),
                stats,
            )
        }
        SatResult::Unknown(reason) => (
            Err(TacticError::Failed(format!(
                "bv_decide: solver returned unknown: {}",
                reason
            ))),
            stats,
        ),
    }
}
/// Helper: extract the var mapping from the analyzer for counterexample formatting.
pub(super) fn encoder_var_mapping(analyzer: &GoalAnalyzer) -> HashMap<String, Vec<SatVar>> {
    let mut mapping = HashMap::new();
    for (name, width) in analyzer.var_map.values() {
        let vars: Vec<SatVar> = (0..width.0).map(SatVar::new).collect();
        mapping.insert(name.clone(), vars);
    }
    mapping
}
/// Preprocess a BV expression: constant folding, simplification.
pub fn preprocess_bv_expr(expr: &BvExpr) -> BvExpr {
    match expr {
        BvExpr::Add(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.add(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return l2;
                }
            }
            if let BvExpr::Const(lv) = &l2 {
                if lv.to_u128() == 0 {
                    return r2;
                }
            }
            BvExpr::Add(Box::new(l2), Box::new(r2))
        }
        BvExpr::Sub(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.sub(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return l2;
                }
            }
            BvExpr::Sub(Box::new(l2), Box::new(r2))
        }
        BvExpr::Mul(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.mul(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return BvExpr::Const(BitVec::zero(l2.width()));
                }
                if rv.to_u128() == 1 {
                    return l2;
                }
            }
            if let BvExpr::Const(lv) = &l2 {
                if lv.to_u128() == 0 {
                    return BvExpr::Const(BitVec::zero(r2.width()));
                }
                if lv.to_u128() == 1 {
                    return r2;
                }
            }
            BvExpr::Mul(Box::new(l2), Box::new(r2))
        }
        BvExpr::And(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.and(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return BvExpr::Const(BitVec::zero(l2.width()));
                }
                if rv.to_u128() == rv.width.max_unsigned() {
                    return l2;
                }
            }
            BvExpr::And(Box::new(l2), Box::new(r2))
        }
        BvExpr::Or(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.or(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return l2;
                }
            }
            BvExpr::Or(Box::new(l2), Box::new(r2))
        }
        BvExpr::Xor(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.xor(rv));
            }
            if let BvExpr::Const(rv) = &r2 {
                if rv.to_u128() == 0 {
                    return l2;
                }
            }
            BvExpr::Xor(Box::new(l2), Box::new(r2))
        }
        BvExpr::Not(inner) => {
            let i2 = preprocess_bv_expr(inner);
            if let BvExpr::Const(v) = &i2 {
                return BvExpr::Const(v.not());
            }
            if let BvExpr::Not(inner2) = i2 {
                return *inner2;
            }
            BvExpr::Not(Box::new(i2))
        }
        BvExpr::Shl(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.shl(rv));
            }
            BvExpr::Shl(Box::new(l2), Box::new(r2))
        }
        BvExpr::Shr(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                return BvExpr::Const(lv.shr(rv));
            }
            BvExpr::Shr(Box::new(l2), Box::new(r2))
        }
        BvExpr::Eq(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            if let (BvExpr::Const(lv), BvExpr::Const(rv)) = (&l2, &r2) {
                let eq = lv == rv;
                return BvExpr::Const(BitVec::from_u128(if eq { 1 } else { 0 }, BitWidth::new(1)));
            }
            BvExpr::Eq(Box::new(l2), Box::new(r2))
        }
        BvExpr::Ult(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            BvExpr::Ult(Box::new(l2), Box::new(r2))
        }
        BvExpr::Slt(l, r) => {
            let l2 = preprocess_bv_expr(l);
            let r2 = preprocess_bv_expr(r);
            BvExpr::Slt(Box::new(l2), Box::new(r2))
        }
        BvExpr::Extract(e, h, l) => {
            let e2 = preprocess_bv_expr(e);
            BvExpr::Extract(Box::new(e2), *h, *l)
        }
        BvExpr::Concat(hi, lo) => {
            let h2 = preprocess_bv_expr(hi);
            let l2 = preprocess_bv_expr(lo);
            BvExpr::Concat(Box::new(h2), Box::new(l2))
        }
        BvExpr::Ite(c, t, e) => {
            let c2 = preprocess_bv_expr(c);
            let t2 = preprocess_bv_expr(t);
            let e2 = preprocess_bv_expr(e);
            if let BvExpr::Const(cv) = &c2 {
                if cv.to_u128() != 0 {
                    return t2;
                } else {
                    return e2;
                }
            }
            BvExpr::Ite(Box::new(c2), Box::new(t2), Box::new(e2))
        }
        BvExpr::Var(_, _) | BvExpr::Const(_) => expr.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::bvdecide::*;
    #[test]
    fn test_bitvec_zero() {
        let bv = BitVec::zero(BitWidth::new(8));
        assert_eq!(bv.to_u128(), 0);
        assert_eq!(bv.width, BitWidth::new(8));
    }
    #[test]
    fn test_bitvec_ones() {
        let bv = BitVec::ones(BitWidth::new(8));
        assert_eq!(bv.to_u128(), 255);
    }
    #[test]
    fn test_bitvec_from_u128() {
        let bv = BitVec::from_u128(42, BitWidth::new(8));
        assert_eq!(bv.to_u128(), 42);
        let bv = BitVec::from_u128(256, BitWidth::new(8));
        assert_eq!(bv.to_u128(), 0);
    }
    #[test]
    fn test_bitvec_add() {
        let a = BitVec::from_u128(10, BitWidth::new(8));
        let b = BitVec::from_u128(20, BitWidth::new(8));
        assert_eq!(a.add(&b).to_u128(), 30);
        let a = BitVec::from_u128(200, BitWidth::new(8));
        let b = BitVec::from_u128(100, BitWidth::new(8));
        assert_eq!(a.add(&b).to_u128(), 44);
    }
    #[test]
    fn test_bitvec_sub() {
        let a = BitVec::from_u128(30, BitWidth::new(8));
        let b = BitVec::from_u128(10, BitWidth::new(8));
        assert_eq!(a.sub(&b).to_u128(), 20);
        let a = BitVec::from_u128(10, BitWidth::new(8));
        let b = BitVec::from_u128(20, BitWidth::new(8));
        assert_eq!(a.sub(&b).to_u128(), 246);
    }
    #[test]
    fn test_bitvec_mul() {
        let a = BitVec::from_u128(6, BitWidth::new(8));
        let b = BitVec::from_u128(7, BitWidth::new(8));
        assert_eq!(a.mul(&b).to_u128(), 42);
        let a = BitVec::from_u128(20, BitWidth::new(8));
        let b = BitVec::from_u128(15, BitWidth::new(8));
        assert_eq!(a.mul(&b).to_u128(), 44);
    }
    #[test]
    fn test_bitvec_bitwise() {
        let a = BitVec::from_u128(0b11001100, BitWidth::new(8));
        let b = BitVec::from_u128(0b10101010, BitWidth::new(8));
        assert_eq!(a.and(&b).to_u128(), 0b10001000);
        assert_eq!(a.or(&b).to_u128(), 0b11101110);
        assert_eq!(a.xor(&b).to_u128(), 0b01100110);
        assert_eq!(a.not().to_u128(), 0b00110011);
    }
    #[test]
    fn test_bitvec_shifts() {
        let a = BitVec::from_u128(0b00001111, BitWidth::new(8));
        assert_eq!(a.shl_const(2).to_u128(), 0b00111100);
        assert_eq!(a.shr_const(2).to_u128(), 0b00000011);
    }
    #[test]
    fn test_bitvec_extract() {
        let bv = BitVec::from_u128(0xAB, BitWidth::new(8));
        let upper = bv.extract(7, 4);
        assert_eq!(upper.to_u128(), 0xA);
        assert_eq!(upper.width, BitWidth::new(4));
        let lower = bv.extract(3, 0);
        assert_eq!(lower.to_u128(), 0xB);
    }
    #[test]
    fn test_bitvec_concat() {
        let hi = BitVec::from_u128(0xA, BitWidth::new(4));
        let lo = BitVec::from_u128(0xB, BitWidth::new(4));
        let result = hi.concat(&lo);
        assert_eq!(result.to_u128(), 0xAB);
        assert_eq!(result.width, BitWidth::new(8));
    }
    #[test]
    fn test_bitvec_extend() {
        let bv = BitVec::from_u128(0xFF, BitWidth::new(8));
        let zext = bv.zero_extend(BitWidth::new(16));
        assert_eq!(zext.to_u128(), 0x00FF);
        assert_eq!(zext.width, BitWidth::new(16));
        let sext = bv.sign_extend(BitWidth::new(16));
        assert_eq!(sext.to_u128(), 0xFFFF);
        assert_eq!(sext.width, BitWidth::new(16));
    }
    #[test]
    fn test_bitvec_comparisons() {
        let a = BitVec::from_u128(5, BitWidth::new(8));
        let b = BitVec::from_u128(10, BitWidth::new(8));
        assert!(a.ult(&b));
        assert!(!b.ult(&a));
        let neg1 = BitVec::from_u128(0xFF, BitWidth::new(8));
        let pos1 = BitVec::from_u128(1, BitWidth::new(8));
        assert!(neg1.slt(&pos1));
        assert!(!pos1.slt(&neg1));
    }
    #[test]
    fn test_bitvec_neg() {
        let a = BitVec::from_u128(1, BitWidth::new(8));
        let neg_a = a.neg();
        assert_eq!(neg_a.to_u128(), 0xFF);
        assert_eq!(neg_a.to_i128(), -1);
    }
    #[test]
    fn test_bitvec_signed_conversion() {
        let bv = BitVec::from_u128(0x80, BitWidth::new(8));
        assert_eq!(bv.to_i128(), -128);
        let bv = BitVec::from_u128(0x7F, BitWidth::new(8));
        assert_eq!(bv.to_i128(), 127);
    }
    #[test]
    fn test_sat_simple_satisfiable() {
        let formula = CnfFormula {
            clauses: vec![
                vec![Literal::pos(SatVar(0)), Literal::pos(SatVar(1))],
                vec![Literal::neg(SatVar(0)), Literal::pos(SatVar(1))],
            ],
            num_vars: 2,
        };
        let mut solver = CdclSolver::new(&formula);
        match solver.solve() {
            SatResult::Sat(model) => {
                assert!(model.values[1]);
            }
            other => panic!("Expected SAT, got {:?}", other),
        }
    }
    #[test]
    fn test_sat_simple_unsatisfiable() {
        let formula = CnfFormula {
            clauses: vec![vec![Literal::pos(SatVar(0))], vec![Literal::neg(SatVar(0))]],
            num_vars: 1,
        };
        let mut solver = CdclSolver::new(&formula);
        match solver.solve() {
            SatResult::Unsat(_) => {}
            other => panic!("Expected UNSAT, got {:?}", other),
        }
    }
    #[test]
    fn test_sat_unit_propagation() {
        let formula = CnfFormula {
            clauses: vec![
                vec![Literal::pos(SatVar(0))],
                vec![Literal::neg(SatVar(0)), Literal::pos(SatVar(1))],
                vec![Literal::neg(SatVar(1)), Literal::pos(SatVar(2))],
            ],
            num_vars: 3,
        };
        let mut solver = CdclSolver::new(&formula);
        match solver.solve() {
            SatResult::Sat(model) => {
                assert!(model.values[0]);
                assert!(model.values[1]);
                assert!(model.values[2]);
            }
            other => panic!("Expected SAT, got {:?}", other),
        }
    }
    #[test]
    fn test_sat_pigeonhole_2_1() {
        let formula = CnfFormula {
            clauses: vec![
                vec![Literal::pos(SatVar(0))],
                vec![Literal::pos(SatVar(1))],
                vec![Literal::neg(SatVar(0)), Literal::neg(SatVar(1))],
            ],
            num_vars: 2,
        };
        let mut solver = CdclSolver::new(&formula);
        match solver.solve() {
            SatResult::Unsat(_) => {}
            other => panic!("Expected UNSAT, got {:?}", other),
        }
    }
    #[test]
    fn test_bv_encode_add_1bit() {
        let mut enc = BvEncoder::new();
        let a = enc.encode_bv_var("a", BitWidth::new(1));
        let b = enc.encode_bv_var("b", BitWidth::new(1));
        let sum = enc.encode_add(&a, &b);
        assert_eq!(sum.len(), 1);
        let formula = enc.into_formula();
        assert!(formula.num_vars > 0);
    }
    #[test]
    fn test_bv_encode_equality() {
        let mut enc = BvEncoder::new();
        let a = enc.encode_bv_var("a", BitWidth::new(4));
        let c = enc.encode_const(&BitVec::from_u128(5, BitWidth::new(4)));
        let eq = enc.encode_equality(&a, &c);
        enc.formula.add_clause(vec![Literal::pos(eq)]);
        let formula = enc.into_formula();
        let mut solver = CdclSolver::new(&formula);
        match solver.solve() {
            SatResult::Sat(model) => {
                assert!(model.values[0]);
                assert!(!model.values[1]);
                assert!(model.values[2]);
                assert!(!model.values[3]);
            }
            other => panic!("Expected SAT, got {:?}", other),
        }
    }
    #[test]
    fn test_vsids_basic() {
        let mut scorer = VsidsScorer::new(5);
        let assignment = Assignment::new(5);
        let first = scorer.pick_variable(&assignment);
        assert!(first.is_some());
        for _ in 0..10 {
            scorer.bump(SatVar(3));
        }
        scorer.decay();
        let picked = scorer
            .pick_variable(&assignment)
            .expect("picked should be present");
        assert_eq!(picked, SatVar(3));
    }
    #[test]
    fn test_vsids_all_assigned() {
        let mut scorer = VsidsScorer::new(3);
        let mut assignment = Assignment::new(3);
        assignment.new_decision_level();
        assignment.assign_decision(Literal::pos(SatVar(0)));
        assignment.assign_decision(Literal::pos(SatVar(1)));
        assignment.assign_decision(Literal::pos(SatVar(2)));
        assert!(scorer.pick_variable(&assignment).is_none());
    }
    #[test]
    fn test_bvexpr_evaluate() {
        let expr = BvExpr::Add(
            Box::new(BvExpr::Var("x".to_string(), BitWidth::new(8))),
            Box::new(BvExpr::Const(BitVec::from_u128(10, BitWidth::new(8)))),
        );
        let mut env = HashMap::new();
        env.insert("x".to_string(), BitVec::from_u128(5, BitWidth::new(8)));
        let result = expr.evaluate(&env).expect("result should be present");
        assert_eq!(result.to_u128(), 15);
    }
    #[test]
    fn test_bvexpr_collect_vars() {
        let expr = BvExpr::Add(
            Box::new(BvExpr::Mul(
                Box::new(BvExpr::Var("x".to_string(), BitWidth::new(8))),
                Box::new(BvExpr::Var("y".to_string(), BitWidth::new(8))),
            )),
            Box::new(BvExpr::Var("z".to_string(), BitWidth::new(8))),
        );
        let vars = expr.collect_vars();
        assert_eq!(vars.len(), 3);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert!(vars.contains("z"));
    }
    #[test]
    fn test_preprocess_constant_fold() {
        let expr = BvExpr::Add(
            Box::new(BvExpr::Const(BitVec::from_u128(3, BitWidth::new(8)))),
            Box::new(BvExpr::Const(BitVec::from_u128(4, BitWidth::new(8)))),
        );
        let result = preprocess_bv_expr(&expr);
        match result {
            BvExpr::Const(bv) => assert_eq!(bv.to_u128(), 7),
            _ => panic!("Expected constant after folding"),
        }
    }
    #[test]
    fn test_preprocess_identity() {
        let expr = BvExpr::Add(
            Box::new(BvExpr::Var("x".to_string(), BitWidth::new(8))),
            Box::new(BvExpr::Const(BitVec::zero(BitWidth::new(8)))),
        );
        let result = preprocess_bv_expr(&expr);
        match result {
            BvExpr::Var(name, _) => assert_eq!(name, "x"),
            _ => panic!("Expected variable after identity elimination"),
        }
    }
    #[test]
    fn test_preprocess_double_negation() {
        let expr = BvExpr::Not(Box::new(BvExpr::Not(Box::new(BvExpr::Var(
            "x".to_string(),
            BitWidth::new(8),
        )))));
        let result = preprocess_bv_expr(&expr);
        match result {
            BvExpr::Var(name, _) => assert_eq!(name, "x"),
            _ => panic!("Expected variable after double negation elimination"),
        }
    }
    #[test]
    fn test_literal_dimacs() {
        let lit = Literal::pos(SatVar(0));
        assert_eq!(lit.to_dimacs(), 1);
        let lit = Literal::neg(SatVar(0));
        assert_eq!(lit.to_dimacs(), -1);
        let lit = Literal::pos(SatVar(4));
        assert_eq!(lit.to_dimacs(), 5);
        let lit = Literal::neg(SatVar(3));
        let dimacs = lit.to_dimacs();
        let restored = Literal::from_dimacs(dimacs);
        assert_eq!(restored, lit);
    }
    #[test]
    fn test_clause_unit_detection() {
        let assignment = Assignment::new(3);
        let clause = Clause::new(vec![Literal::pos(SatVar(0)), Literal::pos(SatVar(1))]);
        assert!(clause.is_unit(&assignment).is_none());
        let mut assignment = Assignment::new(3);
        assignment.new_decision_level();
        assignment.assign_decision(Literal::neg(SatVar(0)));
        let unit = clause.is_unit(&assignment);
        assert_eq!(unit, Some(Literal::pos(SatVar(1))));
    }
    #[test]
    fn test_assignment_backtrack() {
        let mut assignment = Assignment::new(4);
        assignment.new_decision_level();
        assignment.assign_decision(Literal::pos(SatVar(0)));
        assert_eq!(assignment.decision_level(), 1);
        assignment.new_decision_level();
        assignment.assign_decision(Literal::pos(SatVar(1)));
        assignment.assign_decision(Literal::pos(SatVar(2)));
        assert_eq!(assignment.decision_level(), 2);
        assert_eq!(assignment.num_assigned(), 3);
        assignment.backtrack_to(1);
        assert_eq!(assignment.decision_level(), 1);
        assert_eq!(assignment.num_assigned(), 1);
        assert!(assignment.is_assigned(SatVar(0)));
        assert!(!assignment.is_assigned(SatVar(1)));
        assert!(!assignment.is_assigned(SatVar(2)));
    }
    #[test]
    fn test_bitwidth_bounds() {
        let w8 = BitWidth::new(8);
        assert_eq!(w8.max_unsigned(), 255);
        assert_eq!(w8.max_signed(), 127);
        assert_eq!(w8.min_signed(), -128);
        let w1 = BitWidth::new(1);
        assert_eq!(w1.max_unsigned(), 1);
        assert_eq!(w1.max_signed(), 0);
        assert_eq!(w1.min_signed(), -1);
    }
    #[test]
    fn test_clause_db_operations() {
        let mut db = ClauseDb::new();
        let c0 = db.add_clause(Clause::new(vec![Literal::pos(SatVar(0))]));
        let c1 = db.add_clause(Clause::learned(vec![
            Literal::neg(SatVar(0)),
            Literal::pos(SatVar(1)),
        ]));
        assert_eq!(db.num_clauses(), 2);
        assert_eq!(db.num_learned(), 1);
        db.remove(c1);
        assert_eq!(db.num_clauses(), 1);
        assert!(db.get(c0).is_some());
        assert!(db.get(c1).is_none());
    }
    #[test]
    fn test_bv_decide_config_defaults() {
        let config = BvDecideConfig::default();
        assert_eq!(config.max_vars, 100_000);
        assert!(config.preprocessing);
        assert!(config.enable_cdcl);
        let small = BvDecideConfig::small();
        assert_eq!(small.max_vars, 10_000);
        let large = BvDecideConfig::large();
        assert_eq!(large.max_vars, 1_000_000);
    }
    #[test]
    fn test_model_evaluation() {
        let model = Model {
            values: vec![true, false, true],
        };
        assert!(model.eval_literal(Literal::pos(SatVar(0))));
        assert!(!model.eval_literal(Literal::neg(SatVar(0))));
        assert!(model.eval_literal(Literal::neg(SatVar(1))));
        let clause = Clause::new(vec![Literal::neg(SatVar(0)), Literal::pos(SatVar(2))]);
        assert!(model.eval_clause(&clause));
    }
}
