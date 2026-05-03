//! Impl blocks for bvdecide

use super::super::functions::*;
use oxilean_kernel::Expr;
use std::collections::{HashMap, HashSet, VecDeque};

use super::defs::*;

impl CdclSolver {
    /// Create a new CDCL solver for a CNF formula.
    pub fn new(formula: &CnfFormula) -> Self {
        Self::with_config(formula, CdclConfig::default())
    }
    /// Create a new CDCL solver with custom configuration.
    #[allow(clippy::too_many_arguments)]
    pub fn with_config(formula: &CnfFormula, config: CdclConfig) -> Self {
        let num_vars = formula.num_vars as usize;
        let mut solver = CdclSolver {
            clause_db: ClauseDb::new(),
            assignment: Assignment::new(num_vars),
            vsids: VsidsScorer::with_decay(num_vars, config.vsids_decay),
            watch_lists: HashMap::new(),
            watched: HashMap::new(),
            num_vars,
            stats: CdclStats::default(),
            config,
            proof_chain: Vec::new(),
        };
        for clause_lits in &formula.clauses {
            let clause = Clause::new(clause_lits.clone());
            let idx = solver.clause_db.add_clause(clause);
            solver.setup_watches(idx);
        }
        solver.vsids.init_with_perturbation(42);
        solver
    }
    /// Set up watched literals for a clause.
    fn setup_watches(&mut self, clause_idx: usize) {
        if let Some(clause) = self.clause_db.get(clause_idx) {
            let len = clause.lits.len();
            if len == 0 {
                return;
            }
            let w1 = 0;
            let w2 = if len > 1 { 1 } else { 0 };
            let lit1 = clause.lits[w1].to_dimacs();
            let lit2 = clause.lits[w2].to_dimacs();
            self.watch_lists.entry(lit1).or_default().push(clause_idx);
            if len > 1 {
                self.watch_lists.entry(lit2).or_default().push(clause_idx);
            }
            self.watched.insert(
                clause_idx,
                WatchedInfo {
                    watch1: w1,
                    watch2: w2,
                },
            );
        }
    }
    /// Main solve loop: returns SAT, UNSAT, or Unknown.
    pub fn solve(&mut self) -> SatResult {
        if let Some(_conflict) = self.propagate() {
            if self.assignment.decision_level() == 0 {
                return SatResult::Unsat(UnsatProof {
                    resolution_chain: self.proof_chain.clone(),
                });
            }
            return SatResult::Unsat(UnsatProof {
                resolution_chain: self.proof_chain.clone(),
            });
        }
        let mut conflicts_since_restart: u64 = 0;
        let mut restart_limit = self.config.restart_base;
        let mut luby_index: u32 = 0;
        loop {
            if self.stats.conflicts >= self.config.max_conflicts {
                return SatResult::Unknown("conflict limit reached".to_string());
            }
            match self.decide() {
                Some(lit) => {
                    self.stats.decisions += 1;
                    self.assignment.new_decision_level();
                    self.assignment.assign_decision(lit);
                    while let Some(conflict_clause) = self.propagate() {
                        self.stats.conflicts += 1;
                        conflicts_since_restart += 1;
                        if self.assignment.decision_level() == 0 {
                            return SatResult::Unsat(UnsatProof {
                                resolution_chain: self.proof_chain.clone(),
                            });
                        }
                        let (learned_lits, backjump_level) = self.analyze_conflict(conflict_clause);
                        self.backjump(backjump_level);
                        let mut learned = Clause::learned(learned_lits);
                        learned.lbd = learned.compute_lbd(&self.assignment);
                        let learned_idx = self.clause_db.add_clause(learned);
                        self.setup_watches(learned_idx);
                        self.stats.learned_clauses += 1;
                        if let Some(cl) = self.clause_db.get(learned_idx) {
                            let lits_copy: Vec<Literal> = cl.lits.clone();
                            for lit in &lits_copy {
                                self.vsids.bump(lit.var);
                            }
                        }
                        self.vsids.decay();
                        self.clause_db.decay_clause_activities();
                        self.clause_db.bump_clause_activity(learned_idx);
                        if let Some(cl) = self.clause_db.get(learned_idx) {
                            if let Some(unit_lit) = cl.is_unit(&self.assignment) {
                                self.assignment.assign_propagation(unit_lit, learned_idx);
                                self.stats.propagations += 1;
                            }
                        }
                    }
                    if conflicts_since_restart >= restart_limit {
                        let bt_level = 0;
                        self.assignment.backtrack_to(bt_level);
                        conflicts_since_restart = 0;
                        luby_index += 1;
                        restart_limit = self.config.restart_base * luby_sequence(luby_index);
                        self.stats.restarts += 1;
                    }
                    if self.stats.conflicts % self.config.gc_interval == 0
                        && self.stats.conflicts > 0
                    {
                        self.clause_db
                            .gc_learned(self.config.gc_keep_fraction, &self.assignment);
                        self.stats.deletions += 1;
                    }
                }
                None => {
                    let values: Vec<bool> = (0..self.num_vars)
                        .map(|i| {
                            self.assignment
                                .value_of_var(SatVar::new(i as u32))
                                .unwrap_or(false)
                        })
                        .collect();
                    return SatResult::Sat(Model { values });
                }
            }
        }
    }
    /// Pick a decision variable and polarity using VSIDS.
    fn decide(&mut self) -> Option<Literal> {
        self.vsids
            .pick_variable(&self.assignment)
            .map(|var| Literal::pos(var))
    }
    /// Boolean Constraint Propagation: propagate all unit clauses.
    /// Returns Some(clause_index) if a conflict is found.
    fn propagate(&mut self) -> Option<usize> {
        let mut prop_queue: VecDeque<Literal> = VecDeque::new();
        for &lit in self.assignment.trail() {
            prop_queue.push_back(lit);
        }
        let indices = self.clause_db.active_indices();
        for idx in &indices {
            if let Some(clause) = self.clause_db.get(*idx) {
                if clause.is_falsified(&self.assignment) {
                    return Some(*idx);
                }
                if let Some(unit_lit) = clause.is_unit(&self.assignment) {
                    if !self.assignment.is_assigned(unit_lit.var) {
                        self.assignment.assign_propagation(unit_lit, *idx);
                        self.stats.propagations += 1;
                        prop_queue.push_back(unit_lit);
                    }
                }
            }
        }
        while let Some(_propagated_lit) = prop_queue.pop_front() {
            let active = self.clause_db.active_indices();
            for idx in &active {
                if let Some(clause) = self.clause_db.get(*idx) {
                    if clause.is_falsified(&self.assignment) {
                        return Some(*idx);
                    }
                    if let Some(unit_lit) = clause.is_unit(&self.assignment) {
                        if !self.assignment.is_assigned(unit_lit.var) {
                            self.assignment.assign_propagation(unit_lit, *idx);
                            self.stats.propagations += 1;
                            prop_queue.push_back(unit_lit);
                        }
                    }
                }
            }
        }
        None
    }
    /// Analyze a conflict clause to produce a learned clause and backjump level.
    /// Uses the first-UIP (Unique Implication Point) scheme.
    fn analyze_conflict(&mut self, conflict_clause_idx: usize) -> (Vec<Literal>, u32) {
        let current_level = self.assignment.decision_level();
        if current_level == 0 {
            return (Vec::new(), 0);
        }
        let mut seen: HashSet<SatVar> = HashSet::new();
        let mut learned_lits: Vec<Literal> = Vec::new();
        let mut counter = 0u32;
        let mut backjump_level: u32 = 0;
        if let Some(clause) = self.clause_db.get(conflict_clause_idx) {
            for &lit in &clause.lits {
                let var = lit.var;
                if !seen.contains(&var) {
                    seen.insert(var);
                    let var_level = self.assignment.decision_level_of(var).unwrap_or(0);
                    if var_level == current_level {
                        counter += 1;
                    } else if var_level > 0 {
                        learned_lits.push(lit.negate());
                        if var_level > backjump_level {
                            backjump_level = var_level;
                        }
                    }
                }
            }
        }
        let trail_snapshot: Vec<Literal> = self.assignment.trail().to_vec();
        let mut trail_idx = trail_snapshot.len();
        while counter > 1 {
            trail_idx -= 1;
            if trail_idx >= trail_snapshot.len() {
                break;
            }
            let lit = trail_snapshot[trail_idx];
            if !seen.contains(&lit.var) {
                continue;
            }
            if let Some(reason_idx) = self.assignment.reason_of(lit.var) {
                if let Some(reason_clause) = self.clause_db.get(reason_idx) {
                    let reason_lits: Vec<Literal> = reason_clause.lits.clone();
                    for &rlit in &reason_lits {
                        let rvar = rlit.var;
                        if !seen.contains(&rvar) {
                            seen.insert(rvar);
                            let var_level = self.assignment.decision_level_of(rvar).unwrap_or(0);
                            if var_level == current_level {
                                counter += 1;
                            } else if var_level > 0 {
                                learned_lits.push(rlit.negate());
                                if var_level > backjump_level {
                                    backjump_level = var_level;
                                }
                            }
                        }
                    }
                }
            }
            counter -= 1;
        }
        if trail_idx < trail_snapshot.len() {
            let uip_lit = trail_snapshot[trail_idx];
            learned_lits.insert(0, uip_lit.negate());
        }
        let mut second_highest: u32 = 0;
        for lit in &learned_lits[1..] {
            let lv = self.assignment.decision_level_of(lit.var).unwrap_or(0);
            if lv > second_highest {
                second_highest = lv;
            }
        }
        (learned_lits, second_highest)
    }
    /// Backjump to the given decision level.
    fn backjump(&mut self, level: u32) {
        self.assignment.backtrack_to(level);
    }
}

impl BvDecideConfig {
    /// Create a configuration for small/simple problems.
    pub fn small() -> Self {
        BvDecideConfig {
            max_vars: 10_000,
            max_clauses: 100_000,
            timeout_ms: 5_000,
            ..Self::default()
        }
    }
    /// Create a configuration for large/complex problems.
    pub fn large() -> Self {
        BvDecideConfig {
            max_vars: 1_000_000,
            max_clauses: 10_000_000,
            timeout_ms: 120_000,
            ..Self::default()
        }
    }
    /// Convert to CDCL solver config.
    pub fn to_cdcl_config(&self) -> CdclConfig {
        CdclConfig {
            max_conflicts: self.max_clauses,
            restart_base: self.restart_limit,
            gc_interval: 5000,
            gc_keep_fraction: 0.5,
            vsids_decay: self.vsids_decay,
        }
    }
}

impl GoalAnalyzer {
    /// Create a new goal analyzer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut known_bv_types = HashMap::new();
        for w in &[1, 8, 16, 32, 64, 128] {
            known_bv_types.insert(format!("BitVec{}", w), BitWidth::new(*w));
            known_bv_types.insert(format!("BV{}", w), BitWidth::new(*w));
            known_bv_types.insert(format!("UInt{}", w), BitWidth::new(*w));
        }
        GoalAnalyzer {
            var_map: HashMap::new(),
            next_var: 0,
            known_bv_types,
        }
    }
    /// Try to analyze a kernel expression as a BV expression.
    pub fn analyze_goal(&mut self, goal: &Expr) -> Option<BvExpr> {
        self.analyze_expr(goal)
    }
    /// Analyze a kernel expression recursively.
    fn analyze_expr(&mut self, expr: &Expr) -> Option<BvExpr> {
        match expr {
            Expr::Lit(lit) => match lit {
                oxilean_kernel::Literal::Nat(n) => {
                    let width = BitWidth::new(if *n <= u8::MAX as u64 {
                        8
                    } else if *n <= u16::MAX as u64 {
                        16
                    } else if *n <= u32::MAX as u64 {
                        32
                    } else {
                        64
                    });
                    Some(BvExpr::Const(BitVec::from_u128(*n as u128, width)))
                }
                oxilean_kernel::Literal::Str(_) => None,
            },
            Expr::FVar(fvar_id) => {
                let id = fvar_id.0;
                if let Some((name, width)) = self.var_map.get(&id) {
                    Some(BvExpr::Var(name.clone(), *width))
                } else {
                    let name = format!("v{}", self.next_var);
                    self.next_var += 1;
                    let width = BitWidth::new(32);
                    self.var_map.insert(id, (name.clone(), width));
                    Some(BvExpr::Var(name, width))
                }
            }
            Expr::App(func, arg) => self.analyze_app(func, arg),
            Expr::Const(name, _levels) => {
                let name_str = format!("{}", name);
                if name_str.contains("zero") || name_str.contains("Zero") {
                    Some(BvExpr::Const(BitVec::zero(BitWidth::new(32))))
                } else if name_str.contains("one") || name_str.contains("One") {
                    Some(BvExpr::Const(BitVec::from_u128(1, BitWidth::new(32))))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Analyze a function application.
    fn analyze_app(&mut self, func: &Expr, arg: &Expr) -> Option<BvExpr> {
        if let Expr::App(inner_func, first_arg) = func {
            let op_name = self.get_const_name(inner_func)?;
            let lhs = self.analyze_expr(first_arg)?;
            let rhs = self.analyze_expr(arg)?;
            return match op_name.as_str() {
                "BitVec.add" | "BV.add" | "HAdd.hAdd" => {
                    Some(BvExpr::Add(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.sub" | "BV.sub" | "HSub.hSub" => {
                    Some(BvExpr::Sub(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.mul" | "BV.mul" | "HMul.hMul" => {
                    Some(BvExpr::Mul(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.and" | "BV.and" | "HAnd.hAnd" => {
                    Some(BvExpr::And(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.or" | "BV.or" | "HOr.hOr" => Some(BvExpr::Or(Box::new(lhs), Box::new(rhs))),
                "BitVec.xor" | "BV.xor" | "HXor.hXor" => {
                    Some(BvExpr::Xor(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.shiftLeft" | "BV.shl" | "HShiftLeft.hShiftLeft" => {
                    Some(BvExpr::Shl(Box::new(lhs), Box::new(rhs)))
                }
                "BitVec.shiftRight" | "BV.shr" | "HShiftRight.hShiftRight" => {
                    Some(BvExpr::Shr(Box::new(lhs), Box::new(rhs)))
                }
                "BEq.beq" | "BitVec.eq" => Some(BvExpr::Eq(Box::new(lhs), Box::new(rhs))),
                "BitVec.ult" | "BV.ult" => Some(BvExpr::Ult(Box::new(lhs), Box::new(rhs))),
                "BitVec.slt" | "BV.slt" => Some(BvExpr::Slt(Box::new(lhs), Box::new(rhs))),
                "BitVec.append" | "HAppend.hAppend" => {
                    Some(BvExpr::Concat(Box::new(lhs), Box::new(rhs)))
                }
                _ => None,
            };
        }
        let op_name = self.get_const_name(func)?;
        let operand = self.analyze_expr(arg)?;
        match op_name.as_str() {
            "BitVec.not" | "BV.not" | "Complement.complement" => {
                Some(BvExpr::Not(Box::new(operand)))
            }
            _ => None,
        }
    }
    /// Extract a constant name from an expression.
    fn get_const_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Const(name, _) => Some(format!("{}", name)),
            Expr::App(f, _) => self.get_const_name(f),
            _ => None,
        }
    }
}

impl Assignment {
    /// Create a new empty assignment for the given number of variables.
    pub fn new(num_vars: usize) -> Self {
        Assignment {
            values: vec![None; num_vars],
            levels: vec![None; num_vars],
            reasons: vec![None; num_vars],
            trail: Vec::with_capacity(num_vars),
            trail_lim: Vec::new(),
            current_level: 0,
            num_vars,
        }
    }
    /// Get the current decision level.
    pub fn decision_level(&self) -> u32 {
        self.current_level
    }
    /// Start a new decision level.
    pub fn new_decision_level(&mut self) {
        self.trail_lim.push(self.trail.len());
        self.current_level += 1;
    }
    /// Assign a variable at the current level (decision).
    pub fn assign_decision(&mut self, lit: Literal) {
        let idx = lit.var.index();
        self.values[idx] = Some(lit.polarity);
        self.levels[idx] = Some(self.current_level);
        self.reasons[idx] = None;
        self.trail.push(lit);
    }
    /// Assign a variable due to propagation from a clause.
    pub fn assign_propagation(&mut self, lit: Literal, reason_clause: usize) {
        let idx = lit.var.index();
        self.values[idx] = Some(lit.polarity);
        self.levels[idx] = Some(self.current_level);
        self.reasons[idx] = Some(reason_clause);
        self.trail.push(lit);
    }
    /// Get the value of a literal under the current assignment.
    pub fn value_of(&self, lit: Literal) -> Option<bool> {
        self.values[lit.var.index()].map(|v| v == lit.polarity)
    }
    /// Get the value of a variable.
    pub fn value_of_var(&self, var: SatVar) -> Option<bool> {
        self.values[var.index()]
    }
    /// Get the decision level at which a variable was assigned.
    pub fn decision_level_of(&self, var: SatVar) -> Option<u32> {
        self.levels[var.index()]
    }
    /// Get the reason clause for a variable's assignment.
    pub fn reason_of(&self, var: SatVar) -> Option<usize> {
        self.reasons[var.index()]
    }
    /// Backtrack to the given decision level, unassigning all variables above it.
    pub fn backtrack_to(&mut self, level: u32) {
        while self.current_level > level {
            let trail_start = self.trail_lim.pop().unwrap_or(0);
            while self.trail.len() > trail_start {
                let lit = self.trail.pop().expect(
                    "trail is non-empty; loop condition guarantees trail.len() > trail_start",
                );
                let idx = lit.var.index();
                self.values[idx] = None;
                self.levels[idx] = None;
                self.reasons[idx] = None;
            }
            self.current_level -= 1;
        }
    }
    /// Check if a variable is assigned.
    pub fn is_assigned(&self, var: SatVar) -> bool {
        self.values[var.index()].is_some()
    }
    /// Number of assigned variables.
    pub fn num_assigned(&self) -> usize {
        self.values.iter().filter(|v| v.is_some()).count()
    }
    /// Get the trail.
    pub fn trail(&self) -> &[Literal] {
        &self.trail
    }
    /// Check if all variables are assigned.
    pub fn is_complete(&self) -> bool {
        self.num_assigned() == self.num_vars
    }
}

impl BvEncoder {
    /// Create a new encoder.
    pub fn new() -> Self {
        BvEncoder {
            formula: CnfFormula::new(),
            expr_cache: HashMap::new(),
            next_expr_id: 0,
            named_vars: HashMap::new(),
        }
    }
    /// Get the constructed formula.
    pub fn into_formula(self) -> CnfFormula {
        self.formula
    }
    /// Get a reference to the formula.
    pub fn formula(&self) -> &CnfFormula {
        &self.formula
    }
    /// Allocate SAT variables for a BV variable of given width.
    pub fn encode_bv_var(&mut self, name: &str, width: BitWidth) -> Vec<SatVar> {
        if let Some(existing) = self.named_vars.get(name) {
            return existing.clone();
        }
        let vars = self.formula.fresh_vars(width.0);
        self.named_vars.insert(name.to_string(), vars.clone());
        vars
    }
    /// Encode a constant BV value into SAT variables (constants are just fixed literals).
    pub fn encode_const(&mut self, bv: &BitVec) -> Vec<SatVar> {
        let w = bv.width.as_usize();
        let mut result = Vec::with_capacity(w);
        for i in 0..w {
            let var = self.formula.fresh_var();
            if bv.bits[i] {
                self.formula.add_clause(vec![Literal::pos(var)]);
            } else {
                self.formula.add_clause(vec![Literal::neg(var)]);
            }
            result.push(var);
        }
        result
    }
    /// Encode addition of two BV values as a ripple-carry adder circuit.
    /// Returns SAT variables for the sum bits.
    pub fn encode_add(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> Vec<SatVar> {
        assert_eq!(a_bits.len(), b_bits.len());
        let w = a_bits.len();
        let mut sum_bits = Vec::with_capacity(w);
        let mut carry = None;
        for i in 0..w {
            let a = a_bits[i];
            let b = b_bits[i];
            let s = self.formula.fresh_var();
            let c_out = self.formula.fresh_var();
            match carry {
                None => {
                    self.encode_xor2(a, b, s);
                    self.encode_and2(a, b, c_out);
                }
                Some(c_in) => {
                    let ab_xor = self.formula.fresh_var();
                    self.encode_xor2(a, b, ab_xor);
                    self.encode_xor2(ab_xor, c_in, s);
                    let ab_and = self.formula.fresh_var();
                    self.encode_and2(a, b, ab_and);
                    let cin_abxor = self.formula.fresh_var();
                    self.encode_and2(c_in, ab_xor, cin_abxor);
                    self.encode_or2(ab_and, cin_abxor, c_out);
                }
            }
            sum_bits.push(s);
            carry = Some(c_out);
        }
        sum_bits
    }
    /// Encode multiplication using shift-and-add.
    pub fn encode_mul(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> Vec<SatVar> {
        assert_eq!(a_bits.len(), b_bits.len());
        let w = a_bits.len();
        if w == 0 {
            return Vec::new();
        }
        let mut result: Vec<SatVar> = self.encode_zero(w);
        for (i, &b_bit) in b_bits.iter().enumerate() {
            let mut partial = Vec::with_capacity(w);
            for (j, &a_bit) in a_bits.iter().enumerate() {
                if i + j < w {
                    let p = self.formula.fresh_var();
                    self.encode_and2(a_bit, b_bit, p);
                    partial.push(p);
                }
            }
            let mut shifted = Vec::with_capacity(w);
            for _ in 0..i {
                let z = self.formula.fresh_var();
                self.formula.add_clause(vec![Literal::neg(z)]);
                shifted.push(z);
            }
            shifted.extend(partial);
            shifted.truncate(w);
            while shifted.len() < w {
                let z = self.formula.fresh_var();
                self.formula.add_clause(vec![Literal::neg(z)]);
                shifted.push(z);
            }
            result = self.encode_add(&result, &shifted);
        }
        result
    }
    /// Encode an unsigned less-than comparison: result is a single SAT var that is
    /// true iff a < b.
    pub fn encode_comparison(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> SatVar {
        assert_eq!(a_bits.len(), b_bits.len());
        let w = a_bits.len();
        if w == 0 {
            let result = self.formula.fresh_var();
            self.formula.add_clause(vec![Literal::neg(result)]);
            return result;
        }
        let mut lt = self.formula.fresh_var();
        self.formula.add_clause(vec![Literal::neg(lt)]);
        let mut eq = self.formula.fresh_var();
        self.formula.add_clause(vec![Literal::pos(eq)]);
        for i in (0..w).rev() {
            let bits_eq = self.formula.fresh_var();
            self.encode_xnor2(a_bits[i], b_bits[i], bits_eq);
            let a_less = self.formula.fresh_var();
            self.encode_and2_neg_pos(a_bits[i], b_bits[i], a_less);
            let contribution = self.formula.fresh_var();
            self.encode_and2(a_less, eq, contribution);
            let new_lt = self.formula.fresh_var();
            self.encode_or2(lt, contribution, new_lt);
            let new_eq = self.formula.fresh_var();
            self.encode_and2(eq, bits_eq, new_eq);
            lt = new_lt;
            eq = new_eq;
        }
        lt
    }
    /// Encode if-then-else: result\[i\] = cond ? then\[i\] : else\[i\].
    pub fn encode_ite(
        &mut self,
        cond: SatVar,
        then_bits: &[SatVar],
        else_bits: &[SatVar],
    ) -> Vec<SatVar> {
        assert_eq!(then_bits.len(), else_bits.len());
        let w = then_bits.len();
        let mut result = Vec::with_capacity(w);
        for i in 0..w {
            let r = self.formula.fresh_var();
            self.formula.add_clause(vec![
                Literal::neg(cond),
                Literal::neg(then_bits[i]),
                Literal::pos(r),
            ]);
            self.formula.add_clause(vec![
                Literal::neg(cond),
                Literal::pos(then_bits[i]),
                Literal::neg(r),
            ]);
            self.formula.add_clause(vec![
                Literal::pos(cond),
                Literal::neg(else_bits[i]),
                Literal::pos(r),
            ]);
            self.formula.add_clause(vec![
                Literal::pos(cond),
                Literal::pos(else_bits[i]),
                Literal::neg(r),
            ]);
            result.push(r);
        }
        result
    }
    /// Encode extraction of bits \[high:low\] from a bit-vector.
    pub fn encode_extract(&mut self, bits: &[SatVar], high: usize, low: usize) -> Vec<SatVar> {
        assert!(high >= low && high < bits.len());
        bits[low..=high].to_vec()
    }
    /// Encode concatenation: high_bits ++ low_bits.
    pub fn encode_concat(&mut self, high_bits: &[SatVar], low_bits: &[SatVar]) -> Vec<SatVar> {
        let mut result = low_bits.to_vec();
        result.extend_from_slice(high_bits);
        result
    }
    /// Encode equality constraint: all corresponding bits must be equal.
    pub fn encode_equality(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> SatVar {
        assert_eq!(a_bits.len(), b_bits.len());
        let w = a_bits.len();
        if w == 0 {
            let t = self.formula.fresh_var();
            self.formula.add_clause(vec![Literal::pos(t)]);
            return t;
        }
        let mut eq_bits = Vec::with_capacity(w);
        for i in 0..w {
            let bit_eq = self.formula.fresh_var();
            self.encode_xnor2(a_bits[i], b_bits[i], bit_eq);
            eq_bits.push(bit_eq);
        }
        self.encode_and_reduce(&eq_bits)
    }
    /// Encode bitwise NOT.
    pub fn encode_not(&mut self, bits: &[SatVar]) -> Vec<SatVar> {
        let mut result = Vec::with_capacity(bits.len());
        for &b in bits {
            let r = self.formula.fresh_var();
            self.formula
                .add_clause(vec![Literal::pos(b), Literal::pos(r)]);
            self.formula
                .add_clause(vec![Literal::neg(b), Literal::neg(r)]);
            result.push(r);
        }
        result
    }
    /// Encode bitwise AND.
    pub fn encode_bw_and(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> Vec<SatVar> {
        assert_eq!(a_bits.len(), b_bits.len());
        let mut result = Vec::with_capacity(a_bits.len());
        for i in 0..a_bits.len() {
            let r = self.formula.fresh_var();
            self.encode_and2(a_bits[i], b_bits[i], r);
            result.push(r);
        }
        result
    }
    /// Encode bitwise OR.
    pub fn encode_bw_or(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> Vec<SatVar> {
        assert_eq!(a_bits.len(), b_bits.len());
        let mut result = Vec::with_capacity(a_bits.len());
        for i in 0..a_bits.len() {
            let r = self.formula.fresh_var();
            self.encode_or2(a_bits[i], b_bits[i], r);
            result.push(r);
        }
        result
    }
    /// Encode bitwise XOR.
    pub fn encode_bw_xor(&mut self, a_bits: &[SatVar], b_bits: &[SatVar]) -> Vec<SatVar> {
        assert_eq!(a_bits.len(), b_bits.len());
        let mut result = Vec::with_capacity(a_bits.len());
        for i in 0..a_bits.len() {
            let r = self.formula.fresh_var();
            self.encode_xor2(a_bits[i], b_bits[i], r);
            result.push(r);
        }
        result
    }
    /// Encode: result <-> (a AND b)
    fn encode_and2(&mut self, a: SatVar, b: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::neg(result), Literal::pos(a)]);
        self.formula
            .add_clause(vec![Literal::neg(result), Literal::pos(b)]);
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::neg(b), Literal::pos(result)]);
    }
    /// Encode: result <-> (a OR b)
    fn encode_or2(&mut self, a: SatVar, b: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::neg(b), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::neg(result), Literal::pos(a), Literal::pos(b)]);
    }
    /// Encode: result <-> (a XOR b)
    fn encode_xor2(&mut self, a: SatVar, b: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::neg(b), Literal::neg(result)]);
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::pos(b), Literal::neg(result)]);
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::neg(b), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::pos(b), Literal::pos(result)]);
    }
    /// Encode: result <-> (a XNOR b), i.e., result <-> (a == b)
    fn encode_xnor2(&mut self, a: SatVar, b: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::neg(b), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::pos(b), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::neg(b), Literal::neg(result)]);
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::pos(b), Literal::neg(result)]);
    }
    /// Encode: result <-> (!a AND b) — used for less-than comparison.
    fn encode_and2_neg_pos(&mut self, a: SatVar, b: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::neg(result), Literal::neg(a)]);
        self.formula
            .add_clause(vec![Literal::neg(result), Literal::pos(b)]);
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::neg(b), Literal::pos(result)]);
    }
    /// Encode zero: a vector of w variables all forced to false.
    fn encode_zero(&mut self, w: usize) -> Vec<SatVar> {
        let mut result = Vec::with_capacity(w);
        for _ in 0..w {
            let v = self.formula.fresh_var();
            self.formula.add_clause(vec![Literal::neg(v)]);
            result.push(v);
        }
        result
    }
    /// Encode AND-reduction: result <-> (v[0] AND v[1] AND ... AND v[n-1]).
    fn encode_and_reduce(&mut self, vars: &[SatVar]) -> SatVar {
        if vars.is_empty() {
            let t = self.formula.fresh_var();
            self.formula.add_clause(vec![Literal::pos(t)]);
            return t;
        }
        if vars.len() == 1 {
            return vars[0];
        }
        let mut current = vars.to_vec();
        while current.len() > 1 {
            let mut next = Vec::new();
            let mut i = 0;
            while i + 1 < current.len() {
                let r = self.formula.fresh_var();
                self.encode_and2(current[i], current[i + 1], r);
                next.push(r);
                i += 2;
            }
            if i < current.len() {
                next.push(current[i]);
            }
            current = next;
        }
        current[0]
    }
    /// Encode a full BvExpr into SAT variables representing its bits.
    pub fn encode_expr(&mut self, expr: &BvExpr) -> Vec<SatVar> {
        match expr {
            BvExpr::Var(name, width) => self.encode_bv_var(name, *width),
            BvExpr::Const(bv) => self.encode_const(bv),
            BvExpr::Add(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_add(&a, &b)
            }
            BvExpr::Sub(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                let not_b = self.encode_not(&b);
                let one_bits = {
                    let w = not_b.len();
                    let one = BitVec::from_u128(1, BitWidth::new(w as u32));
                    self.encode_const(&one)
                };
                let neg_b = self.encode_add(&not_b, &one_bits);
                self.encode_add(&a, &neg_b)
            }
            BvExpr::Mul(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_mul(&a, &b)
            }
            BvExpr::And(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_bw_and(&a, &b)
            }
            BvExpr::Or(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_bw_or(&a, &b)
            }
            BvExpr::Xor(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_bw_xor(&a, &b)
            }
            BvExpr::Not(inner) => {
                let a = self.encode_expr(inner);
                self.encode_not(&a)
            }
            BvExpr::Shl(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_variable_shl(&a, &b)
            }
            BvExpr::Shr(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                self.encode_variable_shr(&a, &b)
            }
            BvExpr::Eq(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                let eq_var = self.encode_equality(&a, &b);
                vec![eq_var]
            }
            BvExpr::Ult(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                let lt_var = self.encode_comparison(&a, &b);
                vec![lt_var]
            }
            BvExpr::Slt(lhs, rhs) => {
                let a = self.encode_expr(lhs);
                let b = self.encode_expr(rhs);
                let w = a.len();
                if w == 0 {
                    let f = self.formula.fresh_var();
                    self.formula.add_clause(vec![Literal::neg(f)]);
                    return vec![f];
                }
                let mut a_flipped = a.clone();
                let mut b_flipped = b.clone();
                let msb_a = self.formula.fresh_var();
                self.encode_not_single(a[w - 1], msb_a);
                a_flipped[w - 1] = msb_a;
                let msb_b = self.formula.fresh_var();
                self.encode_not_single(b[w - 1], msb_b);
                b_flipped[w - 1] = msb_b;
                let lt_var = self.encode_comparison(&a_flipped, &b_flipped);
                vec![lt_var]
            }
            BvExpr::Extract(inner, high, low) => {
                let bits = self.encode_expr(inner);
                self.encode_extract(&bits, *high as usize, *low as usize)
            }
            BvExpr::Concat(hi, lo) => {
                let h = self.encode_expr(hi);
                let l = self.encode_expr(lo);
                self.encode_concat(&h, &l)
            }
            BvExpr::Ite(cond, then_expr, else_expr) => {
                let c = self.encode_expr(cond);
                let t = self.encode_expr(then_expr);
                let e = self.encode_expr(else_expr);
                assert!(!c.is_empty(), "ITE condition must be at least 1 bit");
                self.encode_ite(c[0], &t, &e)
            }
        }
    }
    /// Encode a single NOT gate: result <-> !a.
    fn encode_not_single(&mut self, a: SatVar, result: SatVar) {
        self.formula
            .add_clause(vec![Literal::pos(a), Literal::pos(result)]);
        self.formula
            .add_clause(vec![Literal::neg(a), Literal::neg(result)]);
    }
    /// Encode variable left shift using a barrel shifter (MUX tree).
    fn encode_variable_shl(&mut self, a_bits: &[SatVar], shift_bits: &[SatVar]) -> Vec<SatVar> {
        let w = a_bits.len();
        let mut current = a_bits.to_vec();
        for (k, &shift_bit) in shift_bits.iter().enumerate() {
            let shift_amount = 1usize << k;
            if shift_amount >= w {
                let zeroes = self.encode_zero(w);
                current = self.encode_ite_vec(shift_bit, &zeroes, &current);
                break;
            }
            let mut shifted = vec![SatVar::new(0); w];
            for i in 0..w {
                if i < shift_amount {
                    let z = self.formula.fresh_var();
                    self.formula.add_clause(vec![Literal::neg(z)]);
                    shifted[i] = z;
                } else {
                    shifted[i] = current[i - shift_amount];
                }
            }
            current = self.encode_ite_vec(shift_bit, &shifted, &current);
        }
        current
    }
    /// Encode variable right shift using a barrel shifter (MUX tree).
    fn encode_variable_shr(&mut self, a_bits: &[SatVar], shift_bits: &[SatVar]) -> Vec<SatVar> {
        let w = a_bits.len();
        let mut current = a_bits.to_vec();
        for (k, &shift_bit) in shift_bits.iter().enumerate() {
            let shift_amount = 1usize << k;
            if shift_amount >= w {
                let zeroes = self.encode_zero(w);
                current = self.encode_ite_vec(shift_bit, &zeroes, &current);
                break;
            }
            let mut shifted = vec![SatVar::new(0); w];
            for i in 0..w {
                if i + shift_amount < w {
                    shifted[i] = current[i + shift_amount];
                } else {
                    let z = self.formula.fresh_var();
                    self.formula.add_clause(vec![Literal::neg(z)]);
                    shifted[i] = z;
                }
            }
            current = self.encode_ite_vec(shift_bit, &shifted, &current);
        }
        current
    }
    /// Encode ITE for a vector of bits using a single condition variable.
    fn encode_ite_vec(
        &mut self,
        cond: SatVar,
        then_bits: &[SatVar],
        else_bits: &[SatVar],
    ) -> Vec<SatVar> {
        self.encode_ite(cond, then_bits, else_bits)
    }
}

impl Model {
    /// Evaluate a literal under this model.
    pub fn eval_literal(&self, lit: Literal) -> bool {
        let val = self.values[lit.var.index()];
        if lit.polarity {
            val
        } else {
            !val
        }
    }
    /// Evaluate a clause under this model.
    pub fn eval_clause(&self, clause: &Clause) -> bool {
        clause.lits.iter().any(|&lit| self.eval_literal(lit))
    }
}

impl BvExpr {
    /// Compute the bit-width of this expression.
    pub fn width(&self) -> BitWidth {
        match self {
            BvExpr::Var(_, w) => *w,
            BvExpr::Const(bv) => bv.width,
            BvExpr::Add(l, _) => l.width(),
            BvExpr::Sub(l, _) => l.width(),
            BvExpr::Mul(l, _) => l.width(),
            BvExpr::And(l, _) => l.width(),
            BvExpr::Or(l, _) => l.width(),
            BvExpr::Xor(l, _) => l.width(),
            BvExpr::Not(e) => e.width(),
            BvExpr::Shl(l, _) => l.width(),
            BvExpr::Shr(l, _) => l.width(),
            BvExpr::Eq(_, _) => BitWidth::new(1),
            BvExpr::Ult(_, _) => BitWidth::new(1),
            BvExpr::Slt(_, _) => BitWidth::new(1),
            BvExpr::Extract(_, high, low) => BitWidth::new(high - low + 1),
            BvExpr::Concat(hi, lo) => BitWidth::new(hi.width().0 + lo.width().0),
            BvExpr::Ite(_, t, _) => t.width(),
        }
    }
    /// Count the number of nodes in this expression tree.
    pub fn node_count(&self) -> usize {
        match self {
            BvExpr::Var(_, _) | BvExpr::Const(_) => 1,
            BvExpr::Not(e) => 1 + e.node_count(),
            BvExpr::Extract(e, _, _) => 1 + e.node_count(),
            BvExpr::Add(l, r)
            | BvExpr::Sub(l, r)
            | BvExpr::Mul(l, r)
            | BvExpr::And(l, r)
            | BvExpr::Or(l, r)
            | BvExpr::Xor(l, r)
            | BvExpr::Shl(l, r)
            | BvExpr::Shr(l, r)
            | BvExpr::Eq(l, r)
            | BvExpr::Ult(l, r)
            | BvExpr::Slt(l, r)
            | BvExpr::Concat(l, r) => 1 + l.node_count() + r.node_count(),
            BvExpr::Ite(c, t, e) => 1 + c.node_count() + t.node_count() + e.node_count(),
        }
    }
    /// Collect all variable names used in this expression.
    pub fn collect_vars(&self) -> HashSet<String> {
        let mut vars = HashSet::new();
        self.collect_vars_into(&mut vars);
        vars
    }
    fn collect_vars_into(&self, vars: &mut HashSet<String>) {
        match self {
            BvExpr::Var(name, _) => {
                vars.insert(name.clone());
            }
            BvExpr::Const(_) => {}
            BvExpr::Not(e) | BvExpr::Extract(e, _, _) => e.collect_vars_into(vars),
            BvExpr::Add(l, r)
            | BvExpr::Sub(l, r)
            | BvExpr::Mul(l, r)
            | BvExpr::And(l, r)
            | BvExpr::Or(l, r)
            | BvExpr::Xor(l, r)
            | BvExpr::Shl(l, r)
            | BvExpr::Shr(l, r)
            | BvExpr::Eq(l, r)
            | BvExpr::Ult(l, r)
            | BvExpr::Slt(l, r)
            | BvExpr::Concat(l, r) => {
                l.collect_vars_into(vars);
                r.collect_vars_into(vars);
            }
            BvExpr::Ite(c, t, e) => {
                c.collect_vars_into(vars);
                t.collect_vars_into(vars);
                e.collect_vars_into(vars);
            }
        }
    }
    /// Evaluate the expression given concrete variable assignments.
    pub fn evaluate(&self, env: &HashMap<String, BitVec>) -> Option<BitVec> {
        match self {
            BvExpr::Var(name, _) => env.get(name).cloned(),
            BvExpr::Const(bv) => Some(bv.clone()),
            BvExpr::Add(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.add(&rv))
            }
            BvExpr::Sub(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.sub(&rv))
            }
            BvExpr::Mul(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.mul(&rv))
            }
            BvExpr::And(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.and(&rv))
            }
            BvExpr::Or(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.or(&rv))
            }
            BvExpr::Xor(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.xor(&rv))
            }
            BvExpr::Not(e) => {
                let ev = e.evaluate(env)?;
                Some(ev.not())
            }
            BvExpr::Shl(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.shl(&rv))
            }
            BvExpr::Shr(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                Some(lv.shr(&rv))
            }
            BvExpr::Eq(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                let eq = lv == rv;
                Some(BitVec::from_u128(if eq { 1 } else { 0 }, BitWidth::new(1)))
            }
            BvExpr::Ult(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                let lt = lv.ult(&rv);
                Some(BitVec::from_u128(if lt { 1 } else { 0 }, BitWidth::new(1)))
            }
            BvExpr::Slt(l, r) => {
                let lv = l.evaluate(env)?;
                let rv = r.evaluate(env)?;
                let lt = lv.slt(&rv);
                Some(BitVec::from_u128(if lt { 1 } else { 0 }, BitWidth::new(1)))
            }
            BvExpr::Extract(e, high, low) => {
                let ev = e.evaluate(env)?;
                Some(ev.extract(*high, *low))
            }
            BvExpr::Concat(hi, lo) => {
                let hv = hi.evaluate(env)?;
                let lv = lo.evaluate(env)?;
                Some(hv.concat(&lv))
            }
            BvExpr::Ite(c, t, e) => {
                let cv = c.evaluate(env)?;
                let cond = cv.to_u128() != 0;
                if cond {
                    t.evaluate(env)
                } else {
                    e.evaluate(env)
                }
            }
        }
    }
}

impl ClauseDb {
    /// Create a new empty clause database.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ClauseDb {
            clauses: Vec::new(),
            original_indices: Vec::new(),
            learned_indices: Vec::new(),
            num_active: 0,
            clause_bump: 1.0,
            clause_decay: 0.999,
        }
    }
    /// Add a clause and return its index.
    pub fn add_clause(&mut self, clause: Clause) -> usize {
        let idx = self.clauses.len();
        if clause.learned {
            self.learned_indices.push(idx);
        } else {
            self.original_indices.push(idx);
        }
        self.clauses.push(Some(clause));
        self.num_active += 1;
        idx
    }
    /// Get a clause by index.
    pub fn get(&self, idx: usize) -> Option<&Clause> {
        self.clauses.get(idx).and_then(|c| c.as_ref())
    }
    /// Get a mutable reference to a clause.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Clause> {
        self.clauses.get_mut(idx).and_then(|c| c.as_mut())
    }
    /// Remove a clause by index.
    pub fn remove(&mut self, idx: usize) {
        if let Some(Some(_)) = self.clauses.get(idx) {
            self.clauses[idx] = None;
            self.num_active -= 1;
        }
    }
    /// Bump clause activity.
    pub fn bump_clause_activity(&mut self, idx: usize) {
        if let Some(Some(clause)) = self.clauses.get_mut(idx) {
            clause.activity += self.clause_bump;
            if clause.activity > 1e100 {
                self.rescale_activities();
            }
        }
    }
    /// Decay all clause activities.
    pub fn decay_clause_activities(&mut self) {
        self.clause_bump /= self.clause_decay;
    }
    /// Rescale activities to avoid overflow.
    fn rescale_activities(&mut self) {
        for clause in self.clauses.iter_mut().flatten() {
            clause.activity *= 1e-100;
        }
        self.clause_bump *= 1e-100;
    }
    /// Garbage-collect learned clauses, keeping the best ones.
    pub fn gc_learned(&mut self, keep_fraction: f64, assignment: &Assignment) {
        let mut learned_with_activity: Vec<(usize, f64, u32)> = self
            .learned_indices
            .iter()
            .filter_map(|&idx| {
                self.clauses
                    .get(idx)
                    .and_then(|c| c.as_ref().map(|clause| (idx, clause.activity, clause.lbd)))
            })
            .collect();
        learned_with_activity.sort_by(|a, b| {
            a.2.cmp(&b.2)
                .then(b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
        });
        let keep_count = (learned_with_activity.len() as f64 * keep_fraction).ceil() as usize;
        let to_remove = &learned_with_activity[keep_count..];
        for &(idx, _, _) in to_remove {
            let is_reason = assignment
                .trail()
                .iter()
                .any(|lit| assignment.reason_of(lit.var) == Some(idx));
            if !is_reason {
                self.remove(idx);
            }
        }
        self.learned_indices
            .retain(|&idx| self.clauses.get(idx).is_some_and(|c| c.is_some()));
    }
    /// Number of active clauses.
    pub fn num_clauses(&self) -> usize {
        self.num_active
    }
    /// Number of learned clauses.
    pub fn num_learned(&self) -> usize {
        self.learned_indices
            .iter()
            .filter(|&&idx| self.clauses.get(idx).is_some_and(|c| c.is_some()))
            .count()
    }
    /// Iterate over all active clause indices.
    pub fn active_indices(&self) -> Vec<usize> {
        (0..self.clauses.len())
            .filter(|&idx| self.clauses[idx].is_some())
            .collect()
    }
}

impl Clause {
    /// Create a new original (non-learned) clause.
    pub fn new(lits: Vec<Literal>) -> Self {
        Clause {
            lits,
            learned: false,
            activity: 0.0,
            lbd: 0,
        }
    }
    /// Create a learned clause.
    pub fn learned(lits: Vec<Literal>) -> Self {
        Clause {
            lits,
            learned: true,
            activity: 0.0,
            lbd: 0,
        }
    }
    /// Check if the clause is unit (exactly one unassigned literal).
    pub fn is_unit(&self, assignment: &Assignment) -> Option<Literal> {
        let mut unassigned = None;
        for &lit in &self.lits {
            match assignment.value_of(lit) {
                Some(true) => return None,
                Some(false) => {}
                None => {
                    if unassigned.is_some() {
                        return None;
                    }
                    unassigned = Some(lit);
                }
            }
        }
        unassigned
    }
    /// Check if the clause is satisfied under the current assignment.
    pub fn is_satisfied(&self, assignment: &Assignment) -> bool {
        self.lits
            .iter()
            .any(|&lit| assignment.value_of(lit) == Some(true))
    }
    /// Check if the clause is falsified (all literals are false).
    pub fn is_falsified(&self, assignment: &Assignment) -> bool {
        self.lits
            .iter()
            .all(|&lit| assignment.value_of(lit) == Some(false))
    }
    /// Number of literals.
    pub fn len(&self) -> usize {
        self.lits.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
    /// Compute LBD: number of distinct decision levels of literals.
    pub fn compute_lbd(&self, assignment: &Assignment) -> u32 {
        let levels: HashSet<u32> = self
            .lits
            .iter()
            .filter_map(|lit| assignment.decision_level_of(lit.var))
            .collect();
        levels.len() as u32
    }
}

impl Literal {
    /// Create a positive literal.
    pub fn pos(var: SatVar) -> Self {
        Literal {
            var,
            polarity: true,
        }
    }
    /// Create a negative literal.
    pub fn neg(var: SatVar) -> Self {
        Literal {
            var,
            polarity: false,
        }
    }
    /// Negate this literal.
    pub fn negate(self) -> Literal {
        Literal {
            var: self.var,
            polarity: !self.polarity,
        }
    }
    /// Encode as a DIMACS-style signed integer.
    pub fn to_dimacs(self) -> i32 {
        let idx = self.var.0 as i32 + 1;
        if self.polarity {
            idx
        } else {
            -idx
        }
    }
    /// Decode from a DIMACS-style signed integer.
    pub fn from_dimacs(val: i32) -> Self {
        assert!(val != 0, "DIMACS literal cannot be 0");
        let polarity = val > 0;
        let var = SatVar::new(val.unsigned_abs() - 1);
        Literal { var, polarity }
    }
}

impl BitVecValue {
    /// Get the width of this value.
    pub fn width(&self) -> BitWidth {
        match self {
            BitVecValue::Concrete(bv) => bv.width,
            BitVecValue::Symbolic(_, w) => *w,
            BitVecValue::Unknown(w) => *w,
        }
    }
}

impl BitVec {
    /// Create a zero bit-vector of the given width.
    pub fn zero(width: BitWidth) -> Self {
        BitVec {
            width,
            bits: vec![false; width.as_usize()],
        }
    }
    /// Create an all-ones bit-vector of the given width.
    pub fn ones(width: BitWidth) -> Self {
        BitVec {
            width,
            bits: vec![true; width.as_usize()],
        }
    }
    /// Create a bit-vector from an unsigned integer, truncated to width.
    pub fn from_u128(value: u128, width: BitWidth) -> Self {
        let w = width.as_usize();
        let mut bits = Vec::with_capacity(w);
        for i in 0..w {
            bits.push((value >> i) & 1 == 1);
        }
        BitVec { width, bits }
    }
    /// Convert to an unsigned u128 value.
    pub fn to_u128(&self) -> u128 {
        let mut result: u128 = 0;
        for (i, &b) in self.bits.iter().enumerate() {
            if b {
                result |= 1u128 << i;
            }
        }
        result
    }
    /// Convert to a signed i128 value (sign-extend the MSB).
    pub fn to_i128(&self) -> i128 {
        let w = self.width.as_usize();
        if w == 0 {
            return 0;
        }
        let unsigned = self.to_u128();
        let sign_bit = self.bits[w - 1];
        if sign_bit {
            let mask = if w < 128 { u128::MAX << w } else { 0 };
            (unsigned | mask) as i128
        } else {
            unsigned as i128
        }
    }
    /// Get a single bit (0-indexed from LSB).
    pub fn get_bit(&self, idx: usize) -> bool {
        if idx < self.bits.len() {
            self.bits[idx]
        } else {
            false
        }
    }
    /// Set a single bit.
    pub fn set_bit(&mut self, idx: usize, val: bool) {
        if idx < self.bits.len() {
            self.bits[idx] = val;
        }
    }
    /// Bitwise AND.
    pub fn and(&self, other: &BitVec) -> BitVec {
        assert_eq!(self.width, other.width);
        BitVec {
            width: self.width,
            bits: self
                .bits
                .iter()
                .zip(other.bits.iter())
                .map(|(&a, &b)| a && b)
                .collect(),
        }
    }
    /// Bitwise OR.
    pub fn or(&self, other: &BitVec) -> BitVec {
        assert_eq!(self.width, other.width);
        BitVec {
            width: self.width,
            bits: self
                .bits
                .iter()
                .zip(other.bits.iter())
                .map(|(&a, &b)| a || b)
                .collect(),
        }
    }
    /// Bitwise XOR.
    pub fn xor(&self, other: &BitVec) -> BitVec {
        assert_eq!(self.width, other.width);
        BitVec {
            width: self.width,
            bits: self
                .bits
                .iter()
                .zip(other.bits.iter())
                .map(|(&a, &b)| a ^ b)
                .collect(),
        }
    }
    /// Bitwise NOT.
    pub fn not(&self) -> BitVec {
        BitVec {
            width: self.width,
            bits: self.bits.iter().map(|&b| !b).collect(),
        }
    }
    /// Two's-complement negation.
    pub fn neg(&self) -> BitVec {
        let inverted = self.not();
        let one = BitVec::from_u128(1, self.width);
        inverted.add(&one)
    }
    /// Addition with wrapping semantics.
    pub fn add(&self, other: &BitVec) -> BitVec {
        assert_eq!(self.width, other.width);
        let w = self.width.as_usize();
        let mut result = Vec::with_capacity(w);
        let mut carry = false;
        for i in 0..w {
            let a = self.bits[i];
            let b = other.bits[i];
            let sum = a as u8 + b as u8 + carry as u8;
            result.push(sum & 1 == 1);
            carry = sum >= 2;
        }
        BitVec {
            width: self.width,
            bits: result,
        }
    }
    /// Subtraction with wrapping semantics.
    pub fn sub(&self, other: &BitVec) -> BitVec {
        self.add(&other.neg())
    }
    /// Multiplication with wrapping semantics (schoolbook algorithm).
    pub fn mul(&self, other: &BitVec) -> BitVec {
        assert_eq!(self.width, other.width);
        let w = self.width.as_usize();
        let mut result = BitVec::zero(self.width);
        for i in 0..w {
            if other.bits[i] {
                let shifted = self.shl_const(i as u32);
                result = result.add(&shifted);
            }
        }
        result
    }
    /// Logical shift left by a constant amount.
    pub fn shl_const(&self, amount: u32) -> BitVec {
        let w = self.width.as_usize();
        let amt = amount as usize;
        let mut bits = vec![false; w];
        if amt < w {
            bits[amt..w].copy_from_slice(&self.bits[..w - amt]);
        }
        BitVec {
            width: self.width,
            bits,
        }
    }
    /// Logical shift left by a bitvector amount.
    pub fn shl(&self, amount: &BitVec) -> BitVec {
        let amt = amount.to_u128() as u32;
        self.shl_const(amt)
    }
    /// Logical shift right by a constant amount.
    pub fn shr_const(&self, amount: u32) -> BitVec {
        let w = self.width.as_usize();
        let amt = amount as usize;
        let mut bits = vec![false; w];
        let len = w.saturating_sub(amt);
        if len > 0 {
            bits[..len].copy_from_slice(&self.bits[amt..amt + len]);
        }
        BitVec {
            width: self.width,
            bits,
        }
    }
    /// Logical shift right by a bitvector amount.
    pub fn shr(&self, amount: &BitVec) -> BitVec {
        let amt = amount.to_u128() as u32;
        self.shr_const(amt)
    }
    /// Extract bits \[high:low\] (inclusive) into a new bitvector.
    pub fn extract(&self, high: u32, low: u32) -> BitVec {
        assert!(high >= low, "extract: high must be >= low");
        let new_width = BitWidth::new(high - low + 1);
        let mut bits = Vec::with_capacity(new_width.as_usize());
        for i in low..=high {
            bits.push(self.get_bit(i as usize));
        }
        BitVec {
            width: new_width,
            bits,
        }
    }
    /// Concatenate: self is the high part, other is the low part.
    pub fn concat(&self, other: &BitVec) -> BitVec {
        let new_width = BitWidth::new(self.width.0 + other.width.0);
        let mut bits = other.bits.clone();
        bits.extend_from_slice(&self.bits);
        BitVec {
            width: new_width,
            bits,
        }
    }
    /// Zero-extend to a wider bit-width.
    pub fn zero_extend(&self, target_width: BitWidth) -> BitVec {
        assert!(target_width.0 >= self.width.0);
        let mut bits = self.bits.clone();
        bits.resize(target_width.as_usize(), false);
        BitVec {
            width: target_width,
            bits,
        }
    }
    /// Sign-extend to a wider bit-width.
    pub fn sign_extend(&self, target_width: BitWidth) -> BitVec {
        assert!(target_width.0 >= self.width.0);
        let sign = if self.bits.is_empty() {
            false
        } else {
            self.bits[self.width.as_usize() - 1]
        };
        let mut bits = self.bits.clone();
        bits.resize(target_width.as_usize(), sign);
        BitVec {
            width: target_width,
            bits,
        }
    }
    /// Unsigned less-than comparison.
    pub fn ult(&self, other: &BitVec) -> bool {
        self.to_u128() < other.to_u128()
    }
    /// Signed less-than comparison.
    pub fn slt(&self, other: &BitVec) -> bool {
        self.to_i128() < other.to_i128()
    }
}

impl VsidsScorer {
    /// Create a new VSIDS scorer for the given number of variables.
    pub fn new(num_vars: usize) -> Self {
        let order: Vec<SatVar> = (0..num_vars as u32).map(SatVar::new).collect();
        VsidsScorer {
            activity: vec![0.0; num_vars],
            bump_amount: 1.0,
            decay_factor: 0.95,
            order_dirty: false,
            order,
        }
    }
    /// Create with custom decay factor.
    pub fn with_decay(num_vars: usize, decay: f64) -> Self {
        let mut scorer = Self::new(num_vars);
        scorer.decay_factor = decay;
        scorer
    }
    /// Bump the activity of a variable (called when it participates in a conflict).
    pub fn bump(&mut self, var: SatVar) {
        let idx = var.index();
        if idx < self.activity.len() {
            self.activity[idx] += self.bump_amount;
            if self.activity[idx] > 1e100 {
                self.rescale();
            }
            self.order_dirty = true;
        }
    }
    /// Bump all variables that appear in a clause.
    pub fn bump_clause(&mut self, clause: &Clause) {
        for lit in &clause.lits {
            self.bump(lit.var);
        }
    }
    /// Decay all variable activities (called after each conflict).
    pub fn decay(&mut self) {
        self.bump_amount /= self.decay_factor;
    }
    /// Rescale all activities to avoid floating-point overflow.
    fn rescale(&mut self) {
        let scale = 1e-100;
        for a in &mut self.activity {
            *a *= scale;
        }
        self.bump_amount *= scale;
    }
    /// Pick the unassigned variable with the highest activity.
    pub fn pick_variable(&mut self, assignment: &Assignment) -> Option<SatVar> {
        if self.order_dirty {
            self.rebuild_order();
        }
        self.order
            .iter()
            .copied()
            .find(|&var| !assignment.is_assigned(var))
    }
    /// Rebuild the sorted order of variables by activity (descending).
    fn rebuild_order(&mut self) {
        self.order.sort_by(|a, b| {
            self.activity[b.index()]
                .partial_cmp(&self.activity[a.index()])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.order_dirty = false;
    }
    /// Get the activity of a variable.
    pub fn get_activity(&self, var: SatVar) -> f64 {
        self.activity.get(var.index()).copied().unwrap_or(0.0)
    }
    /// Initialize activities with random perturbation for tie-breaking.
    pub fn init_with_perturbation(&mut self, seed: u64) {
        let mut state = seed;
        for a in &mut self.activity {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *a = (state >> 33) as f64 * 1e-10;
        }
        self.order_dirty = true;
    }
    /// Number of variables tracked.
    pub fn num_vars(&self) -> usize {
        self.activity.len()
    }
}

impl CnfFormula {
    /// Create an empty formula.
    pub fn new() -> Self {
        CnfFormula {
            clauses: Vec::new(),
            num_vars: 0,
        }
    }
    /// Add a clause.
    pub fn add_clause(&mut self, lits: Vec<Literal>) {
        self.clauses.push(lits);
    }
    /// Allocate a fresh variable and return it.
    pub fn fresh_var(&mut self) -> SatVar {
        let v = SatVar::new(self.num_vars);
        self.num_vars += 1;
        v
    }
    /// Allocate `n` fresh variables and return them.
    pub fn fresh_vars(&mut self, n: u32) -> Vec<SatVar> {
        (0..n).map(|_| self.fresh_var()).collect()
    }
    /// Number of clauses.
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }
}

impl SatVar {
    /// Create a new variable with the given index.
    pub fn new(idx: u32) -> Self {
        SatVar(idx)
    }
    /// Return the variable index.
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl BitWidth {
    /// Create a new bit-width.
    pub fn new(w: u32) -> Self {
        assert!(w > 0, "bit-width must be positive");
        BitWidth(w)
    }
    /// Return the width as a usize.
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
    /// Maximum unsigned value representable in this width.
    pub fn max_unsigned(self) -> u128 {
        if self.0 >= 128 {
            u128::MAX
        } else {
            (1u128 << self.0) - 1
        }
    }
    /// Maximum signed value representable in this width.
    pub fn max_signed(self) -> i128 {
        if self.0 >= 128 {
            i128::MAX
        } else {
            (1i128 << (self.0 - 1)) - 1
        }
    }
    /// Minimum signed value representable in this width.
    pub fn min_signed(self) -> i128 {
        if self.0 >= 128 {
            i128::MIN
        } else {
            -(1i128 << (self.0 - 1))
        }
    }
}
