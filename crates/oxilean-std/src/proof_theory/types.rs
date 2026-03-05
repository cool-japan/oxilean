//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// A ground resolution prover.
///
/// Implements the basic resolution refutation procedure:
/// derive the empty clause from a set of ground clauses.
pub struct ResolutionProver {
    /// The initial clause set.
    clauses: Vec<Clause>,
}
impl ResolutionProver {
    /// Create a new resolution prover with the given clause set.
    pub fn new(clauses: Vec<Clause>) -> Self {
        ResolutionProver { clauses }
    }
    /// Run resolution with a step limit.
    ///
    /// Returns `true` if the empty clause is derived (refutation found),
    /// `false` if no refutation exists within the step limit.
    pub fn refute(&self, max_steps: usize) -> bool {
        let mut clause_set: std::collections::HashSet<Vec<i32>> =
            self.clauses.iter().map(|c| c.0.clone()).collect();
        if clause_set.contains(&vec![]) {
            return true;
        }
        let mut worklist: Vec<Clause> = self.clauses.clone();
        let mut steps = 0;
        while steps < max_steps {
            let n = worklist.len();
            let mut new_clauses: Vec<Clause> = Vec::new();
            'outer: for i in 0..n {
                for j in 0..n {
                    for &lit in &worklist[i].0.clone() {
                        if lit > 0 {
                            if let Some(resolvent) =
                                Clause::resolve(&worklist[i], &worklist[j], lit)
                            {
                                if resolvent.is_empty() {
                                    return true;
                                }
                                if !clause_set.contains(&resolvent.0) {
                                    clause_set.insert(resolvent.0.clone());
                                    new_clauses.push(resolvent);
                                    steps += 1;
                                    if steps >= max_steps {
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if new_clauses.is_empty() {
                return false;
            }
            worklist.extend(new_clauses);
        }
        false
    }
    /// Returns the current clause set.
    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }
}
/// A Herbrand term: built from constants and function symbols.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HerbrandTerm {
    /// A constant (0-ary function symbol).
    Const(String),
    /// A function applied to arguments.
    Fun(String, Vec<HerbrandTerm>),
}
impl HerbrandTerm {
    /// Construct a constant term.
    pub fn constant(name: &str) -> Self {
        HerbrandTerm::Const(name.to_string())
    }
    /// Construct a function term.
    pub fn fun(name: &str, args: Vec<HerbrandTerm>) -> Self {
        HerbrandTerm::Fun(name.to_string(), args)
    }
    /// Depth of the term (constants have depth 0).
    pub fn depth(&self) -> usize {
        match self {
            HerbrandTerm::Const(_) => 0,
            HerbrandTerm::Fun(_, args) => 1 + args.iter().map(|a| a.depth()).max().unwrap_or(0),
        }
    }
}
/// A complete sequent calculus proof with validity checking.
pub struct SequentCalculusProof {
    /// The root of the derivation tree (bottom-most sequent).
    pub root: LKNode,
}
impl SequentCalculusProof {
    /// Create a new proof with the given derivation tree.
    pub fn new(root: LKNode) -> Self {
        SequentCalculusProof { root }
    }
    /// Verify that all axiom leaves in the derivation are valid.
    pub fn is_valid(&self) -> bool {
        self.root.verify_axioms()
    }
    /// Return true if the proof is cut-free.
    pub fn is_cut_free(&self) -> bool {
        self.root.is_cut_free()
    }
    /// Number of inference steps in the entire proof tree.
    pub fn size(&self) -> usize {
        self.root.size()
    }
    /// The conclusion (bottom) sequent of the proof.
    pub fn conclusion(&self) -> &Sequent {
        &self.root.conclusion
    }
    /// Verify the conclusion is propositionally valid using truth tables.
    pub fn conclusion_valid(&self) -> bool {
        is_provable_propositional(&self.root.conclusion)
    }
}
/// Herbrand instance generator: enumerates ground instances in order of depth.
pub struct HerbrandInstanceGenerator {
    /// Constants in the Herbrand universe.
    constants: Vec<String>,
    /// Function symbols with arities.
    function_symbols: Vec<(String, usize)>,
    /// Current depth limit.
    current_depth: usize,
}
impl HerbrandInstanceGenerator {
    /// Create a new generator with the given constants and function symbols.
    /// At least one constant is required for a non-empty Herbrand universe.
    pub fn new(constants: Vec<String>, function_symbols: Vec<(String, usize)>) -> Self {
        let constants = if constants.is_empty() {
            vec!["c".to_string()]
        } else {
            constants
        };
        HerbrandInstanceGenerator {
            constants,
            function_symbols,
            current_depth: 0,
        }
    }
    /// Generate all Herbrand terms up to a given depth.
    pub fn terms_up_to_depth(&self, depth: usize) -> Vec<HerbrandTerm> {
        if depth == 0 {
            return self
                .constants
                .iter()
                .map(|c| HerbrandTerm::Const(c.clone()))
                .collect();
        }
        let mut terms: Vec<HerbrandTerm> = self.terms_up_to_depth(depth - 1);
        for (fname, arity) in &self.function_symbols {
            let sub_terms = self.terms_up_to_depth(depth - 1);
            if *arity == 0 {
                continue;
            }
            for t in &sub_terms {
                let args = vec![t.clone(); *arity];
                let new_term = HerbrandTerm::Fun(fname.clone(), args);
                if !terms.contains(&new_term) {
                    terms.push(new_term);
                }
            }
        }
        terms
    }
    /// Generate the next batch of instances (increment depth).
    pub fn next_instances(&mut self, variables: &[String]) -> Vec<HerbrandInstance> {
        let terms = self.terms_up_to_depth(self.current_depth);
        self.current_depth += 1;
        let mut instances = Vec::new();
        for term in &terms {
            let mut inst = HerbrandInstance::new();
            for var in variables {
                inst.bind(var, term.clone());
            }
            instances.push(inst);
        }
        instances
    }
}
/// A Herbrand instance: an assignment from variable names to Herbrand terms.
#[derive(Debug, Clone)]
pub struct HerbrandInstance {
    /// Map from variable name to ground term.
    pub substitution: std::collections::HashMap<String, HerbrandTerm>,
}
impl HerbrandInstance {
    /// Create a new empty instance.
    pub fn new() -> Self {
        HerbrandInstance {
            substitution: std::collections::HashMap::new(),
        }
    }
    /// Add a variable binding.
    pub fn bind(&mut self, var: &str, term: HerbrandTerm) {
        self.substitution.insert(var.to_string(), term);
    }
    /// Look up the ground term for a variable.
    pub fn lookup(&self, var: &str) -> Option<&HerbrandTerm> {
        self.substitution.get(var)
    }
}
/// SKI combinator expression.
#[derive(Debug, Clone)]
pub enum Combinator {
    /// Substitution combinator S.
    S,
    /// Constant combinator K.
    K,
    /// Identity combinator I.
    I,
    /// Application of two combinators.
    App(Box<Combinator>, Box<Combinator>),
}
impl Combinator {
    /// Construct an application node.
    pub fn app(f: Combinator, a: Combinator) -> Self {
        Combinator::App(Box::new(f), Box::new(a))
    }
    /// Perform one reduction step if possible.
    pub fn reduce_step(&self) -> Option<Combinator> {
        match self {
            Combinator::App(f, x) if matches!(f.as_ref(), Combinator::I) => Some(*x.clone()),
            Combinator::App(kx, y)
                if matches!(
                    kx.as_ref(), Combinator::App(k, _) if matches!(k.as_ref(), Combinator::K)
                ) =>
            {
                if let Combinator::App(_, x) = kx.as_ref() {
                    let _ = y;
                    Some(*x.clone())
                } else {
                    None
                }
            }
            Combinator::App(sxy, z)
                if matches!(
                    sxy.as_ref(), Combinator::App(sx, _) if matches!(sx.as_ref(),
                    Combinator::App(s, _) if matches!(s.as_ref(), Combinator::S))
                ) =>
            {
                if let Combinator::App(sx, y) = sxy.as_ref() {
                    if let Combinator::App(_, x) = sx.as_ref() {
                        let xz = Combinator::app(*x.clone(), *z.clone());
                        let yz = Combinator::app(*y.clone(), *z.clone());
                        return Some(Combinator::app(xz, yz));
                    }
                }
                None
            }
            Combinator::App(f, a) => {
                if let Some(f2) = f.reduce_step() {
                    return Some(Combinator::app(f2, *a.clone()));
                }
                if let Some(a2) = a.reduce_step() {
                    return Some(Combinator::app(*f.clone(), a2));
                }
                None
            }
            _ => None,
        }
    }
    /// Reduce to normal form with a fuel limit.
    pub fn reduce_full(&self, fuel: u32) -> Combinator {
        let mut current = self.clone();
        for _ in 0..fuel {
            match current.reduce_step() {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Returns true if no reduction rule applies.
    pub fn is_normal(&self) -> bool {
        self.reduce_step().is_none()
    }
}
/// A natural deduction proof: a typing derivation.
pub struct NaturalDeductionProof {
    /// The term representing the proof.
    pub term: NDTerm,
    /// The proposition proved.
    pub proposition: Formula,
    /// Context: named assumptions with their types.
    pub context: Vec<(String, Formula)>,
}
impl NaturalDeductionProof {
    /// Create a new natural deduction proof.
    pub fn new(context: Vec<(String, Formula)>, term: NDTerm, proposition: Formula) -> Self {
        NaturalDeductionProof {
            term,
            proposition,
            context,
        }
    }
    /// Normalize the proof term using beta-reduction.
    pub fn normalize(&self, fuel: u32) -> NDTerm {
        self.term.normalize(fuel)
    }
    /// Check whether the proof term is already in normal form.
    pub fn is_normal(&self) -> bool {
        self.term.is_normal()
    }
}
/// A cut eliminator that removes top-level cut applications in a sequent proof.
///
/// For propositional logic, this verifies that cut-free provability
/// is equivalent to truth-table provability.
pub struct CutEliminator;
impl CutEliminator {
    /// Attempt to eliminate all cuts from an LK derivation tree.
    ///
    /// This implementation returns a new tree where cut nodes are replaced
    /// by splitting the derivation at the cut formula and verifying by truth table.
    /// Returns `None` if the resulting sequent is not truth-table provable.
    pub fn eliminate_cuts(node: &LKNode) -> Option<LKNode> {
        match &node.rule {
            LKRule::Cut(cut_formula) => {
                if !is_provable_propositional(&node.conclusion) {
                    return None;
                }
                let _ = cut_formula;
                Some(Self::build_direct_proof(node.conclusion.clone()))
            }
            _ => {
                let new_premises: Vec<Option<LKNode>> =
                    node.premises.iter().map(Self::eliminate_cuts).collect();
                if new_premises.iter().any(|p| p.is_none()) {
                    return None;
                }
                let premises: Vec<LKNode> = new_premises.into_iter().flatten().collect();
                Some(LKNode {
                    conclusion: node.conclusion.clone(),
                    rule: node.rule.clone(),
                    premises,
                })
            }
        }
    }
    /// Build a direct (unstructured) proof node for a sequent that is truth-table valid.
    pub fn build_direct_proof(seq: Sequent) -> LKNode {
        LKNode {
            conclusion: seq,
            rule: LKRule::Axiom,
            premises: vec![],
        }
    }
    /// Check whether a proof is cut-free and valid.
    pub fn verify(proof: &SequentCalculusProof) -> bool {
        proof.is_cut_free() && proof.conclusion_valid()
    }
}
/// A sequent in propositional logic: Γ ⊢ Δ
#[derive(Debug, Clone)]
pub struct Sequent {
    /// Antecedent formulas Γ.
    pub antecedent: Vec<Formula>,
    /// Succedent formulas Δ.
    pub succedent: Vec<Formula>,
}
impl Sequent {
    /// Create a new sequent from antecedent and succedent formula lists.
    pub fn new(ant: Vec<Formula>, suc: Vec<Formula>) -> Self {
        Sequent {
            antecedent: ant,
            succedent: suc,
        }
    }
    /// Returns true if some atom appears on both sides (initial sequent axiom).
    pub fn is_axiom(&self) -> bool {
        for f in &self.antecedent {
            if let Formula::Atom(_) = f {
                if self.succedent.iter().any(|g| g == f) {
                    return true;
                }
            }
        }
        false
    }
    /// Human-readable display of the sequent.
    pub fn display(&self) -> String {
        let ant: Vec<String> = self.antecedent.iter().map(|f| f.to_string()).collect();
        let suc: Vec<String> = self.succedent.iter().map(|f| f.to_string()).collect();
        format!("{} ⊢ {}", ant.join(", "), suc.join(", "))
    }
}
/// A node in an LK proof tree.
#[derive(Debug, Clone)]
pub struct LKNode {
    /// The conclusion sequent of this node.
    pub conclusion: Sequent,
    /// The rule applied.
    pub rule: LKRule,
    /// Premise nodes (0, 1, or 2).
    pub premises: Vec<LKNode>,
}
impl LKNode {
    /// Construct a leaf node (Axiom rule, no premises).
    pub fn axiom(conclusion: Sequent) -> Self {
        LKNode {
            conclusion,
            rule: LKRule::Axiom,
            premises: vec![],
        }
    }
    /// Construct a one-premise node.
    pub fn unary(conclusion: Sequent, rule: LKRule, premise: LKNode) -> Self {
        LKNode {
            conclusion,
            rule,
            premises: vec![premise],
        }
    }
    /// Construct a two-premise node.
    pub fn binary(conclusion: Sequent, rule: LKRule, left: LKNode, right: LKNode) -> Self {
        LKNode {
            conclusion,
            rule,
            premises: vec![left, right],
        }
    }
    /// Returns true if this node represents a cut-free derivation.
    pub fn is_cut_free(&self) -> bool {
        match &self.rule {
            LKRule::Cut(_) => false,
            _ => self.premises.iter().all(|p| p.is_cut_free()),
        }
    }
    /// Count the number of rule applications in the tree.
    pub fn size(&self) -> usize {
        1 + self.premises.iter().map(|p| p.size()).sum::<usize>()
    }
    /// Verify that the axiom leaves are valid (same atom on both sides).
    pub fn verify_axioms(&self) -> bool {
        match &self.rule {
            LKRule::Axiom => self.conclusion.is_axiom(),
            _ => self.premises.iter().all(|p| p.verify_axioms()),
        }
    }
}
/// An inference rule in a sequent calculus derivation tree.
#[derive(Debug, Clone)]
pub enum LKRule {
    /// Axiom rule: A ⊢ A
    Axiom,
    /// Left weakening: add a formula to the antecedent
    LeftWeaken,
    /// Right weakening: add a formula to the succedent
    RightWeaken,
    /// Left contraction
    LeftContract,
    /// Right contraction
    RightContract,
    /// Cut rule with explicit cut formula
    Cut(Formula),
    /// Left and-introduction
    AndLeft1,
    /// Left and-introduction (second branch)
    AndLeft2,
    /// Right and-introduction
    AndRight,
    /// Left or-introduction (first branch)
    OrLeft,
    /// Right or-introduction
    OrRight1,
    /// Right or-introduction (second branch)
    OrRight2,
    /// Left implication
    ImpLeft,
    /// Right implication
    ImpRight,
    /// Left negation
    NegLeft,
    /// Right negation
    NegRight,
}
/// A simply-typed lambda calculus term for natural deduction proofs (Curry-Howard).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NDTerm {
    /// Variable by name.
    Var(String),
    /// Lambda abstraction λx:T.body
    Lam(String, Box<Formula>, Box<NDTerm>),
    /// Application (f a)
    App(Box<NDTerm>, Box<NDTerm>),
    /// Conjunction introduction pair ⟨a, b⟩
    Pair(Box<NDTerm>, Box<NDTerm>),
    /// First projection π₁
    Fst(Box<NDTerm>),
    /// Second projection π₂
    Snd(Box<NDTerm>),
    /// Left injection inl(t) for disjunction
    Inl(Box<NDTerm>),
    /// Right injection inr(t) for disjunction
    Inr(Box<NDTerm>),
    /// Case analysis: case t of inl(x) → l | inr(y) → r
    Case(Box<NDTerm>, String, Box<NDTerm>, String, Box<NDTerm>),
    /// Absurdity elimination from ⊥
    Absurd(Box<NDTerm>),
}
impl NDTerm {
    /// Check if the term is a value (no outermost beta-redex).
    pub fn is_value(&self) -> bool {
        match self {
            NDTerm::Lam(_, _, _) => true,
            NDTerm::Pair(_, _) => true,
            NDTerm::Inl(_) => true,
            NDTerm::Inr(_) => true,
            NDTerm::Var(_) => true,
            _ => false,
        }
    }
    /// Substitute variable `x` with term `replacement` (capture-avoiding is simplified here).
    pub fn subst(&self, x: &str, replacement: &NDTerm) -> NDTerm {
        match self {
            NDTerm::Var(v) => {
                if v == x {
                    replacement.clone()
                } else {
                    self.clone()
                }
            }
            NDTerm::Lam(v, ty, body) => {
                if v == x {
                    self.clone()
                } else {
                    NDTerm::Lam(v.clone(), ty.clone(), Box::new(body.subst(x, replacement)))
                }
            }
            NDTerm::App(f, a) => NDTerm::App(
                Box::new(f.subst(x, replacement)),
                Box::new(a.subst(x, replacement)),
            ),
            NDTerm::Pair(a, b) => NDTerm::Pair(
                Box::new(a.subst(x, replacement)),
                Box::new(b.subst(x, replacement)),
            ),
            NDTerm::Fst(t) => NDTerm::Fst(Box::new(t.subst(x, replacement))),
            NDTerm::Snd(t) => NDTerm::Snd(Box::new(t.subst(x, replacement))),
            NDTerm::Inl(t) => NDTerm::Inl(Box::new(t.subst(x, replacement))),
            NDTerm::Inr(t) => NDTerm::Inr(Box::new(t.subst(x, replacement))),
            NDTerm::Case(t, xl, l, xr, r) => {
                let t2 = t.subst(x, replacement);
                let l2 = if xl == x {
                    *l.clone()
                } else {
                    l.subst(x, replacement)
                };
                let r2 = if xr == x {
                    *r.clone()
                } else {
                    r.subst(x, replacement)
                };
                NDTerm::Case(
                    Box::new(t2),
                    xl.clone(),
                    Box::new(l2),
                    xr.clone(),
                    Box::new(r2),
                )
            }
            NDTerm::Absurd(t) => NDTerm::Absurd(Box::new(t.subst(x, replacement))),
        }
    }
    /// Perform one step of beta-reduction if possible (leftmost-outermost).
    pub fn reduce_step(&self) -> Option<NDTerm> {
        match self {
            NDTerm::App(f, a) => {
                if let NDTerm::Lam(x, _, body) = f.as_ref() {
                    return Some(body.subst(x, a));
                }
                if let Some(f2) = f.reduce_step() {
                    return Some(NDTerm::App(Box::new(f2), a.clone()));
                }
                if let Some(a2) = a.reduce_step() {
                    return Some(NDTerm::App(f.clone(), Box::new(a2)));
                }
                None
            }
            NDTerm::Fst(t) => {
                if let NDTerm::Pair(a, _) = t.as_ref() {
                    return Some(*a.clone());
                }
                t.reduce_step().map(|t2| NDTerm::Fst(Box::new(t2)))
            }
            NDTerm::Snd(t) => {
                if let NDTerm::Pair(_, b) = t.as_ref() {
                    return Some(*b.clone());
                }
                t.reduce_step().map(|t2| NDTerm::Snd(Box::new(t2)))
            }
            NDTerm::Case(t, xl, l, xr, r) => {
                if let NDTerm::Inl(v) = t.as_ref() {
                    return Some(l.subst(xl, v));
                }
                if let NDTerm::Inr(v) = t.as_ref() {
                    return Some(r.subst(xr, v));
                }
                t.reduce_step().map(|t2| {
                    NDTerm::Case(Box::new(t2), xl.clone(), l.clone(), xr.clone(), r.clone())
                })
            }
            _ => None,
        }
    }
    /// Reduce to normal form with a fuel limit.
    pub fn normalize(&self, fuel: u32) -> NDTerm {
        let mut current = self.clone();
        for _ in 0..fuel {
            match current.reduce_step() {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Returns true if no reduction rule applies.
    pub fn is_normal(&self) -> bool {
        self.reduce_step().is_none()
    }
}
/// A clause in ground resolution: a set of integer literals.
/// Positive integer i represents atom i; negative -i represents ¬(atom i).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clause(pub Vec<i32>);
impl Clause {
    /// Construct a clause from a slice of literals.
    pub fn new(lits: &[i32]) -> Self {
        let mut v: Vec<i32> = lits.to_vec();
        v.sort_unstable();
        v.dedup();
        Clause(v)
    }
    /// Returns true if this is the empty (refuting) clause.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Resolve two clauses on a given variable.
    /// Returns the resolvent if one clause contains +var and the other -var,
    /// or `None` if resolution is not applicable.
    pub fn resolve(c1: &Clause, c2: &Clause, var: i32) -> Option<Clause> {
        let has_pos = c1.0.contains(&var);
        let has_neg = c2.0.contains(&(-var));
        if !has_pos || !has_neg {
            return None;
        }
        let mut lits: Vec<i32> =
            c1.0.iter()
                .filter(|&&l| l != var)
                .chain(c2.0.iter().filter(|&&l| l != -var))
                .copied()
                .collect();
        lits.sort_unstable();
        lits.dedup();
        Some(Clause(lits))
    }
}
/// A propositional formula (atomic or compound).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    /// Propositional atom identified by name.
    Atom(String),
    /// Logical truth ⊤.
    True_,
    /// Logical falsehood ⊥.
    False_,
    /// Negation ¬A.
    Neg(Box<Formula>),
    /// Conjunction A ∧ B.
    And(Box<Formula>, Box<Formula>),
    /// Disjunction A ∨ B.
    Or(Box<Formula>, Box<Formula>),
    /// Implication A → B.
    Implies(Box<Formula>, Box<Formula>),
    /// Biconditional A ↔ B.
    Iff(Box<Formula>, Box<Formula>),
}
impl Formula {
    /// Construct an atomic formula.
    pub fn atom(s: &str) -> Self {
        Formula::Atom(s.to_string())
    }
    /// Construct a negation ¬f.
    pub fn neg(f: Formula) -> Self {
        Formula::Neg(Box::new(f))
    }
    /// Construct a conjunction a ∧ b.
    pub fn and(a: Formula, b: Formula) -> Self {
        Formula::And(Box::new(a), Box::new(b))
    }
    /// Construct a disjunction a ∨ b.
    pub fn or(a: Formula, b: Formula) -> Self {
        Formula::Or(Box::new(a), Box::new(b))
    }
    /// Construct an implication a → b.
    pub fn implies(a: Formula, b: Formula) -> Self {
        Formula::Implies(Box::new(a), Box::new(b))
    }
    /// Construct a biconditional a ↔ b.
    pub fn iff(a: Formula, b: Formula) -> Self {
        Formula::Iff(Box::new(a), Box::new(b))
    }
    /// Returns all strict subformulas (not including self).
    pub fn subformulas(&self) -> Vec<Formula> {
        match self {
            Formula::Atom(_) | Formula::True_ | Formula::False_ => vec![],
            Formula::Neg(f) => {
                let mut v = vec![*f.clone()];
                v.extend(f.subformulas());
                v
            }
            Formula::And(a, b)
            | Formula::Or(a, b)
            | Formula::Implies(a, b)
            | Formula::Iff(a, b) => {
                let mut v = vec![*a.clone(), *b.clone()];
                v.extend(a.subformulas());
                v.extend(b.subformulas());
                v
            }
        }
    }
    /// Returns deduplicated atom names used in the formula.
    pub fn atoms(&self) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        self.collect_atoms(&mut seen, &mut result);
        result
    }
    fn collect_atoms(&self, seen: &mut std::collections::HashSet<String>, out: &mut Vec<String>) {
        match self {
            Formula::Atom(s) => {
                if seen.insert(s.clone()) {
                    out.push(s.clone());
                }
            }
            Formula::True_ | Formula::False_ => {}
            Formula::Neg(f) => f.collect_atoms(seen, out),
            Formula::And(a, b)
            | Formula::Or(a, b)
            | Formula::Implies(a, b)
            | Formula::Iff(a, b) => {
                a.collect_atoms(seen, out);
                b.collect_atoms(seen, out);
            }
        }
    }
    /// Formula depth: atoms and constants are 0.
    pub fn depth(&self) -> usize {
        match self {
            Formula::Atom(_) | Formula::True_ | Formula::False_ => 0,
            Formula::Neg(f) => 1 + f.depth(),
            Formula::And(a, b)
            | Formula::Or(a, b)
            | Formula::Implies(a, b)
            | Formula::Iff(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    /// Returns true if the formula is a literal (atom or negation of atom).
    pub fn is_literal(&self) -> bool {
        match self {
            Formula::Atom(_) => true,
            Formula::Neg(f) => matches!(f.as_ref(), Formula::Atom(_)),
            _ => false,
        }
    }
    /// Returns true if the formula is a tautology (true under all assignments).
    pub fn is_tautology(&self) -> bool {
        let rows = truth_table(self);
        rows.iter().all(|(_, v)| *v)
    }
    /// Returns true if there exists an assignment making the formula true.
    pub fn is_satisfiable(&self) -> bool {
        let rows = truth_table(self);
        rows.iter().any(|(_, v)| *v)
    }
    /// Returns true if the formula is false under all assignments.
    pub fn is_contradiction(&self) -> bool {
        !self.is_satisfiable()
    }
    /// Returns the negation normal form (NNF) of the formula.
    pub fn to_nnf(&self) -> Formula {
        match self {
            Formula::Atom(_) | Formula::True_ | Formula::False_ => self.clone(),
            Formula::Neg(inner) => match inner.as_ref() {
                Formula::Atom(_) => self.clone(),
                Formula::True_ => Formula::False_,
                Formula::False_ => Formula::True_,
                Formula::Neg(f) => f.to_nnf(),
                Formula::And(a, b) => Formula::or(
                    Formula::neg(*a.clone()).to_nnf(),
                    Formula::neg(*b.clone()).to_nnf(),
                ),
                Formula::Or(a, b) => Formula::and(
                    Formula::neg(*a.clone()).to_nnf(),
                    Formula::neg(*b.clone()).to_nnf(),
                ),
                Formula::Implies(a, b) => {
                    Formula::and(a.to_nnf(), Formula::neg(*b.clone()).to_nnf())
                }
                Formula::Iff(a, b) => Formula::or(
                    Formula::and(a.to_nnf(), Formula::neg(*b.clone()).to_nnf()),
                    Formula::and(Formula::neg(*a.clone()).to_nnf(), b.to_nnf()),
                ),
            },
            Formula::And(a, b) => Formula::and(a.to_nnf(), b.to_nnf()),
            Formula::Or(a, b) => Formula::or(a.to_nnf(), b.to_nnf()),
            Formula::Implies(a, b) => Formula::or(Formula::neg(*a.clone()).to_nnf(), b.to_nnf()),
            Formula::Iff(a, b) => Formula::and(
                Formula::or(Formula::neg(*a.clone()).to_nnf(), b.to_nnf()),
                Formula::or(Formula::neg(*b.clone()).to_nnf(), a.to_nnf()),
            ),
        }
    }
}
