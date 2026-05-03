//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// An operation symbol in a signature: name and arity.
pub struct OpSymbol {
    pub name: String,
    pub arity: usize,
}
impl OpSymbol {
    /// Create a new operation symbol.
    pub fn new(name: &str, arity: usize) -> Self {
        OpSymbol {
            name: name.to_string(),
            arity,
        }
    }
    /// Return true if this operation is a constant (arity 0).
    pub fn is_constant(&self) -> bool {
        self.arity == 0
    }
}
/// Boolean algebra structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BooleanAlgebra {
    pub name: String,
    pub is_atomic: bool,
    pub is_complete: bool,
    pub cardinality_description: String,
}
impl BooleanAlgebra {
    #[allow(dead_code)]
    pub fn power_set(set_name: &str) -> Self {
        Self {
            name: format!("P({})", set_name),
            is_atomic: true,
            is_complete: true,
            cardinality_description: format!("2^|{}|", set_name),
        }
    }
    #[allow(dead_code)]
    pub fn finite(n: u32) -> Self {
        Self {
            name: format!("2^{n}"),
            is_atomic: true,
            is_complete: true,
            cardinality_description: format!("2^{n} elements"),
        }
    }
    #[allow(dead_code)]
    pub fn stone_representation(&self) -> String {
        format!(
            "Stone duality: {} <-> Stone space (compact Hausdorff zero-dimensional)",
            self.name
        )
    }
    #[allow(dead_code)]
    pub fn is_representable(&self) -> bool {
        true
    }
}
/// Distributive lattice (Birkhoff representation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DistributiveLattice {
    pub name: String,
    pub is_finite: bool,
}
impl DistributiveLattice {
    #[allow(dead_code)]
    pub fn new(name: &str, finite: bool) -> Self {
        Self {
            name: name.to_string(),
            is_finite: finite,
        }
    }
    #[allow(dead_code)]
    pub fn birkhoff_representation(&self) -> String {
        if self.is_finite {
            format!(
                "Finite dist. lattice {} <-> J({}) (poset of join-irreducibles)",
                self.name, self.name
            )
        } else {
            format!(
                "Infinite dist. lattice {} <-> lattice of down-sets",
                self.name
            )
        }
    }
}
/// A term in a term algebra: either a variable or a function application.
#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    /// A variable with an index.
    Var(usize),
    /// An application of an operation symbol to subterms.
    App { op: String, args: Vec<Term> },
}
impl Term {
    /// Construct a variable term.
    pub fn var(index: usize) -> Self {
        Term::Var(index)
    }
    /// Construct a function application term.
    pub fn apply(op: &str, args: Vec<Term>) -> Self {
        Term::App {
            op: op.to_string(),
            args,
        }
    }
    /// Construct a constant term (arity-0 operation).
    pub fn constant(op: &str) -> Self {
        Term::App {
            op: op.to_string(),
            args: vec![],
        }
    }
    /// Return the depth (maximum nesting) of this term.
    pub fn depth(&self) -> usize {
        match self {
            Term::Var(_) => 0,
            Term::App { args, .. } => 1 + args.iter().map(|a| a.depth()).max().unwrap_or(0),
        }
    }
    /// Return the set of variable indices appearing in this term.
    pub fn variables(&self) -> Vec<usize> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort_unstable();
        vars.dedup();
        vars
    }
    fn collect_vars(&self, acc: &mut Vec<usize>) {
        match self {
            Term::Var(i) => acc.push(*i),
            Term::App { args, .. } => {
                for a in args {
                    a.collect_vars(acc);
                }
            }
        }
    }
    /// Substitute variable `var_idx` with `replacement` throughout this term.
    pub fn subst(&self, var_idx: usize, replacement: &Term) -> Term {
        match self {
            Term::Var(i) if *i == var_idx => replacement.clone(),
            Term::Var(_) => self.clone(),
            Term::App { op, args } => Term::App {
                op: op.clone(),
                args: args.iter().map(|a| a.subst(var_idx, replacement)).collect(),
            },
        }
    }
    /// Evaluate this term in a given algebra, given an assignment of variables to carrier elements.
    pub fn eval(&self, alg: &Algebra, assignment: &[usize]) -> Option<usize> {
        match self {
            Term::Var(i) => assignment.get(*i).copied(),
            Term::App { op, args } => {
                let evaluated: Option<Vec<usize>> =
                    args.iter().map(|a| a.eval(alg, assignment)).collect();
                let arg_vals = evaluated?;
                alg.apply_op(op, &arg_vals)
            }
        }
    }
}
/// A finite concrete algebra over a signature.
///
/// The carrier is `{0, 1, ..., carrier_size - 1}`.
pub struct Algebra {
    pub carrier_size: usize,
    pub signature: Signature,
    pub tables: std::collections::HashMap<String, Vec<Vec<usize>>>,
}
impl Algebra {
    /// Create a new algebra with the given carrier size and signature.
    pub fn new(carrier: usize, sig: Signature) -> Self {
        Algebra {
            carrier_size: carrier,
            signature: sig,
            tables: std::collections::HashMap::new(),
        }
    }
    /// Define the operation table for a named operation.
    pub fn define_op(&mut self, name: &str, table: Vec<Vec<usize>>) {
        self.tables.insert(name.to_string(), table);
    }
    /// Apply the named operation to the given arguments, returning the result.
    pub fn apply_op(&self, name: &str, args: &[usize]) -> Option<usize> {
        let table = self.tables.get(name)?;
        if args.is_empty() {
            return table.first().and_then(|row| row.first()).copied();
        }
        let current: &Vec<Vec<usize>> = table;
        let first_arg = *args.first()?;
        if first_arg >= current.len() {
            return None;
        }
        if args.len() == 1 {
            return current[first_arg].first().copied();
        }
        let second_arg = args[1];
        let row = current[first_arg].get(second_arg)?;
        Some(*row)
    }
    /// Check whether all table entries refer to elements within the carrier.
    pub fn is_closed(&self) -> bool {
        for table in self.tables.values() {
            for row in table {
                for &val in row {
                    if val >= self.carrier_size {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Return a stub count of valid congruences (placeholder implementation).
    pub fn find_congruences(&self) -> usize {
        if self.carrier_size <= 1 {
            1
        } else {
            2
        }
    }
}
/// Birkhoff variety: a class of algebras defined by equations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BirkhoffVariety {
    pub name: String,
    pub signature_name: String,
    pub axioms: Vec<String>,
}
impl BirkhoffVariety {
    #[allow(dead_code)]
    pub fn new(name: &str, sig: &str, axioms: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            signature_name: sig.to_string(),
            axioms: axioms.into_iter().map(String::from).collect(),
        }
    }
    #[allow(dead_code)]
    pub fn groups() -> Self {
        Self::new(
            "Groups",
            "(*, inv, e)",
            vec![
                "x * (y * z) = (x * y) * z",
                "x * e = x",
                "e * x = x",
                "x * inv(x) = e",
                "inv(x) * x = e",
            ],
        )
    }
    #[allow(dead_code)]
    pub fn lattices() -> Self {
        Self::new(
            "Lattices",
            "(meet, join)",
            vec![
                "meet(x, x) = x",
                "join(x, x) = x",
                "meet(x, y) = meet(y, x)",
                "join(x, y) = join(y, x)",
                "meet(x, join(x, y)) = x",
                "join(x, meet(x, y)) = x",
            ],
        )
    }
    #[allow(dead_code)]
    pub fn rings() -> Self {
        Self::new(
            "Rings",
            "(+, *, 0, -, 1)",
            vec![
                "x + (y + z) = (x + y) + z",
                "x + 0 = x",
                "x + (-x) = 0",
                "x + y = y + x",
                "x * (y * z) = (x * y) * z",
                "x * 1 = x",
                "1 * x = x",
                "x * (y + z) = x * y + x * z",
                "(x + y) * z = x * z + y * z",
            ],
        )
    }
    #[allow(dead_code)]
    pub fn is_equationally_definable(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn closed_under_hsp(&self) -> bool {
        true
    }
}
/// Omega-categorical structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OmegaCategoricalStructure {
    pub name: String,
    pub automorphism_group_description: String,
}
impl OmegaCategoricalStructure {
    #[allow(dead_code)]
    pub fn new(name: &str, aut_group: &str) -> Self {
        Self {
            name: name.to_string(),
            automorphism_group_description: aut_group.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn is_oligomorphic(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn thomas_conjecture(&self) -> String {
        "Thomas: Constraint satisfaction for omega-categorical structures is in NP or NP-complete or P"
            .to_string()
    }
}
/// Quasi-variety: class closed under S, P, ultraproducts.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuasiVariety {
    pub name: String,
    pub quasi_equations: Vec<String>,
}
impl QuasiVariety {
    #[allow(dead_code)]
    pub fn new(name: &str, eqs: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            quasi_equations: eqs.into_iter().map(String::from).collect(),
        }
    }
    #[allow(dead_code)]
    pub fn every_variety_is_quasivariety(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn is_axiomatized_by_implications(&self) -> bool {
        true
    }
}
/// A rewrite rule: lhs → rhs.
#[derive(Clone, Debug)]
pub struct RewriteRule {
    pub lhs: Term,
    pub rhs: Term,
    pub name: String,
}
impl RewriteRule {
    /// Create a new rewrite rule.
    pub fn new(name: &str, lhs: Term, rhs: Term) -> Self {
        RewriteRule {
            lhs,
            rhs,
            name: name.to_string(),
        }
    }
    /// Attempt to match `pattern` against `target`, binding variables.
    /// Returns Some(substitution) if match succeeds; each entry is (var_idx, term).
    pub fn match_term<'a>(
        pattern: &'a Term,
        target: &'a Term,
        subst: &mut Vec<(usize, Term)>,
    ) -> bool {
        match (pattern, target) {
            (Term::Var(i), _) => {
                if let Some((_, existing)) = subst.iter().find(|(v, _)| v == i) {
                    existing == target
                } else {
                    subst.push((*i, target.clone()));
                    true
                }
            }
            (
                Term::App {
                    op: op1,
                    args: args1,
                },
                Term::App {
                    op: op2,
                    args: args2,
                },
            ) => {
                if op1 != op2 || args1.len() != args2.len() {
                    return false;
                }
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    if !Self::match_term(a1, a2, subst) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
    /// Apply the substitution to a term.
    pub fn apply_subst(term: &Term, subst: &[(usize, Term)]) -> Term {
        let mut result = term.clone();
        for (var_idx, replacement) in subst {
            result = result.subst(*var_idx, replacement);
        }
        result
    }
    /// Try to apply this rule at the top level of `t`.
    /// Returns Some(rewritten) if the rule fires; None otherwise.
    pub fn try_apply_top(&self, t: &Term) -> Option<Term> {
        let mut subst = Vec::new();
        if Self::match_term(&self.lhs, t, &mut subst) {
            Some(Self::apply_subst(&self.rhs, &subst))
        } else {
            None
        }
    }
}
/// A simple Knuth-Bendix completion procedure.
///
/// Given a set of equations, attempts to produce a confluent and terminating TRS.
pub struct KnuthBendixCompletion {
    pub trs: TermRewriteSystem,
    /// Pending equations (lhs, rhs) to orient.
    pub pending: Vec<(Term, Term)>,
    /// Maximum completion steps.
    pub max_steps: usize,
}
impl KnuthBendixCompletion {
    /// Create a new KB completion instance from initial equations.
    pub fn new(sig: Signature, equations: Vec<(Term, Term)>, max_steps: usize) -> Self {
        KnuthBendixCompletion {
            trs: TermRewriteSystem::new(sig),
            pending: equations,
            max_steps,
        }
    }
    /// Orient an equation into a rule using a simple heuristic: deeper lhs on left.
    fn orient(lhs: &Term, rhs: &Term) -> Option<(Term, Term)> {
        match lhs.depth().cmp(&rhs.depth()) {
            std::cmp::Ordering::Greater => Some((lhs.clone(), rhs.clone())),
            std::cmp::Ordering::Less => Some((rhs.clone(), lhs.clone())),
            std::cmp::Ordering::Equal => {
                let lv = lhs.variables().len();
                let rv = rhs.variables().len();
                if lv > rv {
                    Some((lhs.clone(), rhs.clone()))
                } else if rv > lv {
                    Some((rhs.clone(), lhs.clone()))
                } else {
                    None
                }
            }
        }
    }
    /// Run the completion procedure.
    /// Returns true if completion succeeded (no unorientable equations remain).
    pub fn run(&mut self) -> bool {
        for _step in 0..self.max_steps {
            if self.pending.is_empty() {
                return true;
            }
            let (lhs, rhs) = self.pending.remove(0);
            let lhs_nf = self.trs.normalize(&lhs, 1000);
            let rhs_nf = self.trs.normalize(&rhs, 1000);
            if lhs_nf == rhs_nf {
                continue;
            }
            if let Some((oriented_lhs, oriented_rhs)) = Self::orient(&lhs_nf, &rhs_nf) {
                let rule = RewriteRule::new("kb_rule", oriented_lhs, oriented_rhs);
                let mut new_trs = TermRewriteSystem::new(Signature::new());
                new_trs.rules.clone_from(&self.trs.rules);
                new_trs.rules.push(rule.clone());
                let crit_pairs = new_trs.find_critical_pairs();
                self.trs.add_rule(rule);
                for (t1, t2) in crit_pairs {
                    self.pending.push((t1, t2));
                }
            } else {
                return false;
            }
        }
        self.pending.is_empty()
    }
    /// Return the resulting confluent TRS rules.
    pub fn result_rules(&self) -> &[RewriteRule] {
        &self.trs.rules
    }
}
/// Direct product decomposition theorem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectProductDecomposition {
    pub algebra: String,
    pub factors: Vec<String>,
}
impl DirectProductDecomposition {
    #[allow(dead_code)]
    pub fn new(alg: &str, factors: Vec<&str>) -> Self {
        Self {
            algebra: alg.to_string(),
            factors: factors.into_iter().map(String::from).collect(),
        }
    }
    #[allow(dead_code)]
    pub fn is_nontrivial(&self) -> bool {
        self.factors.len() > 1
    }
    #[allow(dead_code)]
    pub fn subdirectly_irreducible_components(&self) -> Vec<String> {
        self.factors.clone()
    }
}
/// Clone (universal algebra sense): set of operations closed under composition and containing projections.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Clone {
    pub name: String,
    pub contains_projections: bool,
    pub closed_under_composition: bool,
}
impl Clone {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            contains_projections: true,
            closed_under_composition: true,
        }
    }
    #[allow(dead_code)]
    pub fn clone_of_algebra(algebra: &str) -> Self {
        Self::new(&format!("Clo({})", algebra))
    }
    #[allow(dead_code)]
    pub fn galois_connection_description(&self) -> String {
        format!(
            "Galois connection: Clo <-> Inv (polymorphisms and invariant relations for {})",
            self.name
        )
    }
}
/// The term algebra for a signature and variable set.
///
/// Stores the signature and supports term construction/evaluation.
pub struct TermAlgebra {
    pub signature: Signature,
    pub num_vars: usize,
}
impl TermAlgebra {
    /// Create a term algebra with the given signature and number of variables.
    pub fn new(sig: Signature, num_vars: usize) -> Self {
        TermAlgebra {
            signature: sig,
            num_vars,
        }
    }
    /// Validate that a term is well-formed with respect to the signature.
    pub fn is_well_formed(&self, term: &Term) -> bool {
        match term {
            Term::Var(i) => *i < self.num_vars,
            Term::App { op, args } => {
                if let Some(sym) = self.signature.operations.iter().find(|s| s.name == *op) {
                    sym.arity == args.len() && args.iter().all(|a| self.is_well_formed(a))
                } else {
                    false
                }
            }
        }
    }
    /// Build the term `op(x_0, x_1, ..., x_{arity-1})` for a given op name.
    pub fn free_term(&self, op_name: &str) -> Option<Term> {
        let sym = self
            .signature
            .operations
            .iter()
            .find(|s| s.name == *op_name)?;
        let args: Vec<Term> = (0..sym.arity).map(Term::var).collect();
        Some(Term::apply(op_name, args))
    }
}
/// Tarski's characterization of representable relation algebras.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RelationAlgebra {
    pub name: String,
    pub is_representable: bool,
    pub is_simple: bool,
}
impl RelationAlgebra {
    #[allow(dead_code)]
    pub fn new(name: &str, rep: bool, simple: bool) -> Self {
        Self {
            name: name.to_string(),
            is_representable: rep,
            is_simple: simple,
        }
    }
    #[allow(dead_code)]
    pub fn operators_description(&self) -> String {
        "(+, *, -, 0, 1, ;, ^, 1')".to_string()
    }
    #[allow(dead_code)]
    pub fn representable_iff_field_of_sets(&self) -> bool {
        self.is_representable
    }
}
/// A congruence relation on a finite algebra, represented as a partition
/// of carrier elements into equivalence classes.
pub struct CongruenceRelation {
    /// `parent\[i\]` is the representative of element `i` (union-find style).
    parent: Vec<usize>,
    /// Size of the carrier.
    pub carrier_size: usize,
}
impl CongruenceRelation {
    /// Create the discrete congruence (equality) on a carrier of given size.
    pub fn discrete(carrier_size: usize) -> Self {
        CongruenceRelation {
            parent: (0..carrier_size).collect(),
            carrier_size,
        }
    }
    /// Create the total congruence (all elements equivalent).
    pub fn total(carrier_size: usize) -> Self {
        let mut rel = Self::discrete(carrier_size);
        for i in 1..carrier_size {
            rel.merge(0, i);
        }
        rel
    }
    /// Find the representative of the class containing `x`.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }
    /// Merge the classes of `x` and `y`.
    pub fn merge(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.parent[ry] = rx;
        }
    }
    /// Return true iff `x` and `y` are in the same equivalence class.
    pub fn are_equiv(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Return the equivalence classes as a list of sorted lists.
    pub fn classes(&mut self) -> Vec<Vec<usize>> {
        let mut map: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for i in 0..self.carrier_size {
            let root = self.find(i);
            map.entry(root).or_default().push(i);
        }
        let mut classes: Vec<Vec<usize>> = map.into_values().collect();
        for cls in &mut classes {
            cls.sort_unstable();
        }
        classes.sort_by_key(|c| c[0]);
        classes
    }
    /// Return the number of equivalence classes.
    pub fn num_classes(&mut self) -> usize {
        self.classes().len()
    }
}
/// A term rewriting system: a collection of rewrite rules.
pub struct TermRewriteSystem {
    pub rules: Vec<RewriteRule>,
    pub signature: Signature,
}
impl TermRewriteSystem {
    /// Create an empty TRS.
    pub fn new(sig: Signature) -> Self {
        TermRewriteSystem {
            rules: Vec::new(),
            signature: sig,
        }
    }
    /// Add a rewrite rule.
    pub fn add_rule(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Perform one outermost-innermost rewrite step on `t`.
    /// Returns Some(result) if any rule applies; None if already normal.
    pub fn rewrite_step(&self, t: &Term) -> Option<Term> {
        for rule in &self.rules {
            if let Some(result) = rule.try_apply_top(t) {
                return Some(result);
            }
        }
        match t {
            Term::Var(_) => None,
            Term::App { op, args } => {
                let mut new_args = args.clone();
                for (i, arg) in args.iter().enumerate() {
                    if let Some(new_arg) = self.rewrite_step(arg) {
                        new_args[i] = new_arg;
                        return Some(Term::App {
                            op: op.clone(),
                            args: new_args,
                        });
                    }
                }
                None
            }
        }
    }
    /// Reduce `t` to normal form (up to a step limit to avoid infinite loops).
    pub fn normalize(&self, t: &Term, max_steps: usize) -> Term {
        let mut current = t.clone();
        for _ in 0..max_steps {
            match self.rewrite_step(&current) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Check for local confluence by testing critical pairs (simplified).
    /// Returns a list of (t1, t2) pairs where t1 and t2 are the two reducts
    /// of a critical overlap between rules i and j.
    pub fn find_critical_pairs(&self) -> Vec<(Term, Term)> {
        let mut pairs = Vec::new();
        for (i, r1) in self.rules.iter().enumerate() {
            for (j, r2) in self.rules.iter().enumerate() {
                if i == j {
                    continue;
                }
                let mut subst1 = Vec::new();
                let mut subst2 = Vec::new();
                if RewriteRule::match_term(&r1.lhs, &r2.lhs, &mut subst1)
                    && RewriteRule::match_term(&r2.lhs, &r1.lhs, &mut subst2)
                {
                    let t1 = RewriteRule::apply_subst(&r1.rhs, &subst1);
                    let t2 = RewriteRule::apply_subst(&r2.rhs, &subst2);
                    pairs.push((t1, t2));
                }
            }
        }
        pairs
    }
}
/// An algebraic signature: a collection of operation symbols.
pub struct Signature {
    pub operations: Vec<OpSymbol>,
}
impl Signature {
    /// Create an empty signature.
    pub fn new() -> Self {
        Signature {
            operations: Vec::new(),
        }
    }
    /// Add an operation symbol to the signature.
    pub fn add_op(&mut self, op: OpSymbol) {
        self.operations.push(op);
    }
    /// Return the list of (name, arity) pairs.
    pub fn arities(&self) -> Vec<(String, usize)> {
        self.operations
            .iter()
            .map(|op| (op.name.clone(), op.arity))
            .collect()
    }
    /// Check whether an operation with the given name exists.
    pub fn has_op(&self, name: &str) -> bool {
        self.operations.iter().any(|op| op.name == name)
    }
}
/// An equational law: a named identity lhs ≈ rhs (as strings/term descriptions).
pub struct EquationalLaw {
    pub name: String,
    pub lhs: String,
    pub rhs: String,
}
impl EquationalLaw {
    /// Create a new equational law.
    pub fn new(name: &str, lhs: &str, rhs: &str) -> Self {
        EquationalLaw {
            name: name.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
        }
    }
    /// Return true if the law is trivially true (lhs == rhs syntactically).
    pub fn is_trivial(&self) -> bool {
        self.lhs == self.rhs
    }
}
/// Simple rewriting system with string rules (distinct from TermRewriteSystem above).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimpleRewriteSystem {
    pub name: String,
    pub rules: Vec<(String, String)>,
    pub is_confluent: bool,
    pub is_terminating: bool,
}
impl SimpleRewriteSystem {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rules: Vec::new(),
            is_confluent: false,
            is_terminating: false,
        }
    }
    #[allow(dead_code)]
    pub fn add_rule(&mut self, lhs: &str, rhs: &str) {
        self.rules.push((lhs.to_string(), rhs.to_string()));
    }
    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.is_confluent && self.is_terminating
    }
    #[allow(dead_code)]
    pub fn normal_form_unique(&self) -> bool {
        self.is_complete()
    }
    #[allow(dead_code)]
    pub fn critical_pairs_description(&self) -> String {
        format!("Critical pairs for {}: overlaps of LHS of rules", self.name)
    }
}
/// Algebra homomorphism.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlgebraHomomorphism {
    pub source: String,
    pub target: String,
    pub is_injective: bool,
    pub is_surjective: bool,
}
impl AlgebraHomomorphism {
    #[allow(dead_code)]
    pub fn new(src: &str, tgt: &str, inj: bool, surj: bool) -> Self {
        Self {
            source: src.to_string(),
            target: tgt.to_string(),
            is_injective: inj,
            is_surjective: surj,
        }
    }
    #[allow(dead_code)]
    pub fn is_isomorphism(&self) -> bool {
        self.is_injective && self.is_surjective
    }
    #[allow(dead_code)]
    pub fn is_embedding(&self) -> bool {
        self.is_injective
    }
}
/// Free algebra in a variety.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FreeAlgebra {
    pub variety_name: String,
    pub num_generators: usize,
}
impl FreeAlgebra {
    #[allow(dead_code)]
    pub fn new(variety: &str, generators: usize) -> Self {
        Self {
            variety_name: variety.to_string(),
            num_generators: generators,
        }
    }
    #[allow(dead_code)]
    pub fn universal_property(&self) -> String {
        format!(
            "Free {}-algebra on {} generators: any map to any {}-algebra extends uniquely",
            self.variety_name, self.num_generators, self.variety_name
        )
    }
    #[allow(dead_code)]
    pub fn word_problem_decidable(&self) -> bool {
        true
    }
}
/// Maltsev conditions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MaltsevCondition {
    pub name: String,
    pub terms: Vec<String>,
    pub equations: Vec<String>,
}
impl MaltsevCondition {
    #[allow(dead_code)]
    pub fn congruence_permutability() -> Self {
        Self {
            name: "Congruence permutability".to_string(),
            terms: vec!["p(x,y,z)".to_string()],
            equations: vec!["p(x,x,y) = y".to_string(), "p(x,y,y) = x".to_string()],
        }
    }
    #[allow(dead_code)]
    pub fn congruence_distributivity(n: usize) -> Self {
        Self {
            name: format!("Congruence distributivity (Jonsson n={})", n),
            terms: (0..=n).map(|i| format!("d_{i}(x,y,z)")).collect(),
            equations: vec![
                "d_0(x,y,z) = x".to_string(),
                format!("d_{}(x,y,z) = z", n),
                "d_i(x,x,z) = d_{i+1}(x,x,z) for even i".to_string(),
                "d_i(x,z,z) = d_{i+1}(x,z,z) for odd i".to_string(),
            ],
        }
    }
    #[allow(dead_code)]
    pub fn characterizes_variety(&self) -> bool {
        true
    }
}
/// A variety: an equationally defined class of algebras.
pub struct Variety {
    pub name: String,
    pub laws: Vec<EquationalLaw>,
    pub signature: Signature,
}
impl Variety {
    /// Create a new variety with the given name and signature.
    pub fn new(name: &str, sig: Signature) -> Self {
        Variety {
            name: name.to_string(),
            laws: Vec::new(),
            signature: sig,
        }
    }
    /// Add an equational law to the variety.
    pub fn add_law(&mut self, law: EquationalLaw) {
        self.laws.push(law);
    }
    /// Return true if the variety is (at least axiomatically) a group variety,
    /// i.e. it includes associativity, identity, and inverse laws.
    pub fn is_group_variety(&self) -> bool {
        self.includes_law("assoc") && self.includes_law("identity") && self.includes_law("inverse")
    }
    /// Return true if the variety contains a law with the given name.
    pub fn includes_law(&self, name: &str) -> bool {
        self.laws.iter().any(|l| l.name == name)
    }
}
/// Congruence lattice of an algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CongruenceLattice {
    pub algebra_name: String,
    pub is_distributive: bool,
    pub is_modular: bool,
}
impl CongruenceLattice {
    #[allow(dead_code)]
    pub fn new(name: &str, dist: bool, modular: bool) -> Self {
        Self {
            algebra_name: name.to_string(),
            is_distributive: dist,
            is_modular: modular,
        }
    }
    #[allow(dead_code)]
    pub fn has_permuting_congruences(&self) -> bool {
        false
    }
    #[allow(dead_code)]
    pub fn jonssontheorem_description(&self) -> String {
        format!(
            "Jonsson's theorem: Subdirectly irreducible members of variety of {}",
            self.algebra_name
        )
    }
}
