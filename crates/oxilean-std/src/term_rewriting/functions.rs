//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    DependencyPairGraph, EGraph, KnuthBendixData, NarrowingSystem, PolynomialInterpretation,
    ReductionStrategy, RewritingLogicTheory, Rule, Srs, Strategy, StringRewritingSystem,
    Substitution, Term, TreeAutomaton, Trs,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn list_nat() -> Expr {
    app(cst("List"), nat_ty())
}
pub fn option_ty(t: Expr) -> Expr {
    app(cst("Option"), t)
}
/// `Term : Type` — the type of rewriting terms (first-order terms over signature)
pub fn term_ty() -> Expr {
    type0()
}
/// `Signature : Type` — function symbols with arities
pub fn signature_ty() -> Expr {
    type0()
}
/// `Variable : Type` — the type of term variables
pub fn variable_ty() -> Expr {
    type0()
}
/// `Substitution : Type` — a mapping from variables to terms
pub fn substitution_ty() -> Expr {
    arrow(variable_ty(), term_ty())
}
/// `RewriteRule : Type` — a pair (lhs, rhs) of terms
pub fn rewrite_rule_ty() -> Expr {
    type0()
}
/// `TRS : Type` — a term rewriting system (a set of rewrite rules)
pub fn trs_ty() -> Expr {
    type0()
}
/// `Reduction : Term → Term → Prop` — one-step reduction relation
pub fn reduction_ty() -> Expr {
    arrow(term_ty(), arrow(term_ty(), prop()))
}
/// `ReflTransClosure : (Term → Term → Prop) → Term → Term → Prop`
pub fn refl_trans_closure_ty() -> Expr {
    arrow(
        arrow(term_ty(), arrow(term_ty(), prop())),
        arrow(term_ty(), arrow(term_ty(), prop())),
    )
}
/// `Confluence : TRS → Prop`
pub fn confluence_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `LocalConfluence : TRS → Prop`
pub fn local_confluence_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `StrongNormalization : TRS → Prop` (every reduction sequence terminates)
pub fn strong_normalization_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `WeakNormalization : TRS → Prop` (every term has a normal form)
pub fn weak_normalization_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `NormalForm : TRS → Term → Prop`
pub fn normal_form_ty() -> Expr {
    arrow(trs_ty(), arrow(term_ty(), prop()))
}
/// `CriticalPair : TRS → Term → Term → Prop`
pub fn critical_pair_ty() -> Expr {
    arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop())))
}
/// `Orthogonal : TRS → Prop` — left-linear with no critical pairs
pub fn orthogonal_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `LeftLinear : TRS → Prop`
pub fn left_linear_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `GroundTRS : TRS → Prop` — all rules are ground (no variables)
pub fn ground_trs_ty() -> Expr {
    arrow(trs_ty(), prop())
}
/// `Position : Type` — a path in a term tree (list of natural numbers)
pub fn position_ty() -> Expr {
    list_nat()
}
/// `Subterm : Term → Position → Term` — subterm at a position
pub fn subterm_ty() -> Expr {
    arrow(term_ty(), arrow(position_ty(), term_ty()))
}
/// `Replace : Term → Position → Term → Term` — replace subterm at position
pub fn replace_ty() -> Expr {
    arrow(term_ty(), arrow(position_ty(), arrow(term_ty(), term_ty())))
}
/// `Unifier : Term → Term → Substitution → Prop`
pub fn unifier_ty() -> Expr {
    arrow(
        term_ty(),
        arrow(term_ty(), arrow(substitution_ty(), prop())),
    )
}
/// `MGU : Term → Term → Substitution → Prop` — most general unifier
pub fn mgu_ty() -> Expr {
    arrow(
        term_ty(),
        arrow(term_ty(), arrow(substitution_ty(), prop())),
    )
}
/// `KBCompletion : TRS → TRS → Prop` — Knuth-Bendix completion result
pub fn kb_completion_ty() -> Expr {
    arrow(trs_ty(), arrow(trs_ty(), prop()))
}
/// `WordProblem : TRS → Term → Term → Bool` — decidability of equality
pub fn word_problem_ty() -> Expr {
    arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), bool_ty())))
}
/// `StringRewritingSystem : Type` — SRS over an alphabet
pub fn srs_ty() -> Expr {
    type0()
}
/// `ReductionStrategy : Type` — innermost / outermost / parallel
pub fn reduction_strategy_ty() -> Expr {
    type0()
}
/// `EquationalUnification : Term → Term → Term → Prop`
pub fn equational_unification_ty() -> Expr {
    arrow(term_ty(), arrow(term_ty(), arrow(term_ty(), prop())))
}
/// Newman's Lemma: locally confluent + strongly normalizing ⟹ confluent
/// `NewmansLemma : ∀ (R : TRS), LocalConfluence R → StrongNormalization R → Confluence R`
pub fn newmans_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        trs_ty(),
        arrow(
            app(cst("LocalConfluence"), bvar(0)),
            arrow(
                app(cst("StrongNormalization"), bvar(1)),
                app(cst("Confluence"), bvar(2)),
            ),
        ),
    )
}
/// Church-Rosser: confluence ↔ Church-Rosser property
/// `ChurchRosser : ∀ R : TRS, Confluence R ↔ ChurchRosserProp R`
pub fn church_rosser_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        trs_ty(),
        app2(
            cst("Iff"),
            app(cst("Confluence"), bvar(0)),
            app(cst("ChurchRosserProp"), bvar(0)),
        ),
    )
}
/// Orthogonal TRS are confluent
/// `OrthogonalConfluent : ∀ R : TRS, Orthogonal R → Confluence R`
pub fn orthogonal_confluent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        trs_ty(),
        arrow(
            app(cst("Orthogonal"), bvar(0)),
            app(cst("Confluence"), bvar(0)),
        ),
    )
}
/// Rédei's theorem: finitely generated commutative monoids are finitely presented
pub fn redei_theorem_ty() -> Expr {
    prop()
}
/// Ground confluence decidability
pub fn ground_confluence_decidable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        trs_ty(),
        arrow(
            app(cst("GroundTRS"), bvar(0)),
            app(cst("Decidable"), app(cst("Confluence"), bvar(1))),
        ),
    )
}
/// Unique normal forms: confluent ⟹ unique normal forms
pub fn unique_normal_forms_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        trs_ty(),
        arrow(
            app(cst("Confluence"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                term_ty(),
                pi(
                    BinderInfo::Default,
                    "u",
                    term_ty(),
                    pi(
                        BinderInfo::Default,
                        "v",
                        term_ty(),
                        arrow(
                            app3(cst("ReducesTo"), bvar(3), bvar(2), bvar(1)),
                            arrow(
                                app3(cst("ReducesTo"), bvar(4), bvar(3), bvar(0)),
                                app2(cst("Eq"), bvar(2), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Knuth-Bendix completeness: KB produces a complete system when it terminates
pub fn kb_completeness_ty() -> Expr {
    prop()
}
/// Populate `env` with all TRS kernel declarations.
pub fn build_term_rewriting_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("TrsTerm", term_ty()),
        ("TrsSignature", signature_ty()),
        ("TrsVariable", variable_ty()),
        ("TrsSubstitution", substitution_ty()),
        ("TrsRewriteRule", rewrite_rule_ty()),
        ("TRS", trs_ty()),
        ("TrsPosition", position_ty()),
        ("SRS", srs_ty()),
        ("ReductionStrategy", reduction_strategy_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("Confluence", confluence_ty()),
        ("LocalConfluence", local_confluence_ty()),
        ("StrongNormalization", strong_normalization_ty()),
        ("WeakNormalization", weak_normalization_ty()),
        ("NormalForm", normal_form_ty()),
        ("CriticalPair", critical_pair_ty()),
        ("Orthogonal", orthogonal_ty()),
        ("LeftLinear", left_linear_ty()),
        ("GroundTRS", ground_trs_ty()),
        ("Unifier", unifier_ty()),
        ("MGU", mgu_ty()),
        ("KBCompletion", kb_completion_ty()),
        ("EquationalUnification", equational_unification_ty()),
        ("ChurchRosserProp", arrow(trs_ty(), prop())),
        (
            "ReducesTo",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
        (
            "EquivalentUnder",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("trsSubterm", subterm_ty()),
        ("trsReplace", replace_ty()),
        (
            "trsApplySubst",
            arrow(substitution_ty(), arrow(term_ty(), term_ty())),
        ),
        (
            "trsUnify",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        (
            "trsMGU",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        (
            "trsNormalForm",
            arrow(trs_ty(), arrow(term_ty(), term_ty())),
        ),
        (
            "trsReduceInnermost",
            arrow(trs_ty(), arrow(term_ty(), term_ty())),
        ),
        (
            "trsReduceOutermost",
            arrow(trs_ty(), arrow(term_ty(), term_ty())),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("NewmansLemma", newmans_lemma_ty()),
        ("ChurchRosserThm", church_rosser_ty()),
        ("OrthogonalConfluentThm", orthogonal_confluent_ty()),
        ("RedeiTheorem", redei_theorem_ty()),
        (
            "GroundConfluenceDecidable",
            ground_confluence_decidable_ty(),
        ),
        ("UniqueNormalForms", unique_normal_forms_ty()),
        ("KBCompletenessThm", kb_completeness_ty()),
        ("TerminationImpliesWN", prop()),
        ("SNImpliesWN", prop()),
        ("ConfluenceImpliesLocalConfluence", prop()),
        ("LeftLinearNoCPConfluent", prop()),
        ("CriticalPairLemma", prop()),
        ("CommutativityWordProblem", prop()),
        ("TotalTerminationCriterion", prop()),
        ("KBTermination", prop()),
        ("SubstitutionLemma", prop()),
        ("PositionInduction", prop()),
        ("ReplaceLemma", prop()),
        ("DepthTermination", prop()),
        ("LexicographicPathOrder", prop()),
        ("RecursivePathOrder", prop()),
        ("PolynomialInterpretationThm", prop()),
        ("EquationalTheoryAxiom", arrow(trs_ty(), prop())),
        (
            "RewritingModuloE",
            arrow(
                trs_ty(),
                arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
            ),
        ),
        (
            "ACRewriting",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
        (
            "ACUnification",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        ("BEquationalAxiom", arrow(trs_ty(), prop())),
        ("HigherOrderTRS", arrow(trs_ty(), prop())),
        ("CombReductionSystem", arrow(trs_ty(), prop())),
        ("HRSConfluence", arrow(trs_ty(), prop())),
        ("LambdaCalculusEncoding", arrow(term_ty(), term_ty())),
        ("BetaReduction", arrow(term_ty(), option_ty(term_ty()))),
        (
            "DependencyPair",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
        ("DependencyGraph", arrow(trs_ty(), type0())),
        (
            "DependencyPairMethod",
            arrow(trs_ty(), arrow(trs_ty(), prop())),
        ),
        ("SccsTermination", arrow(trs_ty(), prop())),
        ("DPChain", arrow(trs_ty(), arrow(term_ty(), prop()))),
        (
            "ReductionOrdering",
            arrow(arrow(term_ty(), arrow(term_ty(), prop())), prop()),
        ),
        ("PolynomialOrder", arrow(nat_ty(), arrow(trs_ty(), prop()))),
        ("RecursivePathOrdering", arrow(trs_ty(), prop())),
        ("KnuthBendixOrder", arrow(trs_ty(), prop())),
        ("WeightFunction", arrow(term_ty(), nat_ty())),
        (
            "SimplificationOrder",
            arrow(arrow(term_ty(), arrow(term_ty(), prop())), prop()),
        ),
        (
            "NarrowingStep",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
        (
            "LazyNarrowing",
            arrow(trs_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        (
            "BasicNarrowing",
            arrow(trs_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        ("NarrowingCompleteness", arrow(trs_ty(), prop())),
        (
            "NarrowingUnification",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        ("TreeAutomaton", type0()),
        ("RegularTreeLanguage", arrow(cst("TreeAutomaton"), type0())),
        (
            "TreeAutomataIntersection",
            arrow(
                cst("TreeAutomaton"),
                arrow(cst("TreeAutomaton"), cst("TreeAutomaton")),
            ),
        ),
        (
            "TreeAutomataUnion",
            arrow(
                cst("TreeAutomaton"),
                arrow(cst("TreeAutomaton"), cst("TreeAutomaton")),
            ),
        ),
        (
            "TreeAutomataComplementation",
            arrow(cst("TreeAutomaton"), cst("TreeAutomaton")),
        ),
        (
            "TRSPreservesRegularity",
            arrow(trs_ty(), arrow(cst("TreeAutomaton"), prop())),
        ),
        (
            "TreeLanguageMembership",
            arrow(term_ty(), arrow(cst("TreeAutomaton"), prop())),
        ),
        (
            "CongruenceClosure",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
        ("GroundCongruenceClosure", arrow(trs_ty(), prop())),
        (
            "NelsonOppenCombination",
            arrow(trs_ty(), arrow(trs_ty(), prop())),
        ),
        ("SharingCongruence", arrow(trs_ty(), prop())),
        ("HuetsCompletion", arrow(trs_ty(), arrow(trs_ty(), prop()))),
        ("PetersonStickel", arrow(trs_ty(), arrow(trs_ty(), prop()))),
        (
            "OrderedCompletion",
            arrow(trs_ty(), arrow(trs_ty(), prop())),
        ),
        ("CompletionTerminates", arrow(trs_ty(), prop())),
        ("RosensLemma", prop()),
        (
            "ModularConfluence",
            arrow(trs_ty(), arrow(trs_ty(), prop())),
        ),
        ("ParallelClosure", arrow(trs_ty(), prop())),
        (
            "ConfluentIntersection",
            arrow(trs_ty(), arrow(trs_ty(), prop())),
        ),
        ("RewritingLogicTheory", type0()),
        (
            "ConcurrentRewrite",
            arrow(
                cst("RewritingLogicTheory"),
                arrow(term_ty(), arrow(term_ty(), prop())),
            ),
        ),
        (
            "RewritingLogicSoundness",
            arrow(cst("RewritingLogicTheory"), prop()),
        ),
        (
            "MaudeSystemAxiom",
            arrow(cst("RewritingLogicTheory"), prop()),
        ),
        ("SufficientCompleteness", arrow(trs_ty(), prop())),
        (
            "GroundReducibility",
            arrow(trs_ty(), arrow(term_ty(), prop())),
        ),
        (
            "InductiveTheorem",
            arrow(trs_ty(), arrow(term_ty(), prop())),
        ),
        ("ConstructorSystem", arrow(trs_ty(), prop())),
        (
            "ACMatching",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        (
            "ACUMatching",
            arrow(term_ty(), arrow(term_ty(), option_ty(substitution_ty()))),
        ),
        ("FreeAlgebraConstraint", arrow(trs_ty(), prop())),
        (
            "MatchingModuloAxiom",
            arrow(trs_ty(), arrow(term_ty(), arrow(term_ty(), prop()))),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
/// Robinson's unification algorithm (syntactic).
///
/// Returns `Some(mgu)` if `s` and `t` unify, `None` otherwise.
pub fn unify(s: &Term, t: &Term) -> Option<Substitution> {
    let mut equations: Vec<(Term, Term)> = vec![(s.clone(), t.clone())];
    let mut subst = Substitution::new();
    while let Some((lhs, rhs)) = equations.pop() {
        let lhs = lhs.apply(&subst);
        let rhs = rhs.apply(&subst);
        if lhs == rhs {
            continue;
        }
        match (&lhs, &rhs) {
            (Term::Var(x), _) => {
                if rhs.contains(&Term::Var(*x)) {
                    return None;
                }
                let bind = Substitution {
                    map: HashMap::from([(*x, rhs.clone())]),
                };
                let mut new_map = HashMap::new();
                for (&v, t) in &subst.map {
                    new_map.insert(v, t.apply(&bind));
                }
                new_map.insert(*x, rhs.clone());
                subst.map = new_map;
            }
            (_, Term::Var(y)) => {
                if lhs.contains(&Term::Var(*y)) {
                    return None;
                }
                let bind = Substitution {
                    map: HashMap::from([(*y, lhs.clone())]),
                };
                let mut new_map = HashMap::new();
                for (&v, t) in &subst.map {
                    new_map.insert(v, t.apply(&bind));
                }
                new_map.insert(*y, lhs.clone());
                subst.map = new_map;
            }
            (Term::Fun(f, fa), Term::Fun(g, ga)) => {
                if f != g || fa.len() != ga.len() {
                    return None;
                }
                for (a, b) in fa.iter().zip(ga.iter()) {
                    equations.push((a.clone(), b.clone()));
                }
            }
        }
    }
    Some(subst)
}
/// Checks whether two terms are unifiable.
pub fn unifiable(s: &Term, t: &Term) -> bool {
    unify(s, t).is_some()
}
/// Computes all critical pairs of two rules (or a rule with itself).
///
/// A critical pair arises when the lhs of one rule (after renaming) overlaps
/// with a non-variable subterm of the lhs of another rule.
///
/// Returns a list of `(left_result, right_result)` pairs that must converge
/// for the TRS to be locally confluent.
pub fn critical_pairs(r1: &Rule, r2: &Rule, offset: u32) -> Vec<(Term, Term)> {
    let r1 = r1.clone();
    let r2 = r2.rename(offset);
    let mut pairs = Vec::new();
    fn non_var_positions(t: &Term) -> Vec<Vec<usize>> {
        match t {
            Term::Var(_) => vec![],
            Term::Fun(_, args) => {
                let mut out = vec![vec![]];
                for (i, a) in args.iter().enumerate() {
                    for mut p in non_var_positions(a) {
                        let mut full = vec![i];
                        full.append(&mut p);
                        out.push(full);
                    }
                }
                out
            }
        }
    }
    for pos in non_var_positions(&r1.lhs) {
        if let Some(sub) = r1.lhs.subterm_at(&pos) {
            if let Some(sigma) = unify(sub, &r2.lhs) {
                let left = r1.lhs.replace_at(&pos, r2.rhs.apply(&sigma)).apply(&sigma);
                let right = r1.rhs.apply(&sigma);
                if left != right {
                    pairs.push((left, right));
                }
            }
        }
    }
    pairs
}
/// Checks whether all critical pairs of a TRS converge (local confluence).
///
/// Each critical pair `(s, t)` is checked by normalizing both sides and
/// verifying they reach the same normal form (up to `limit` steps).
pub fn check_local_confluence(trs: &Trs, limit: usize) -> bool {
    let n = trs.rules.len();
    for i in 0..n {
        for j in 0..n {
            let pairs = critical_pairs(
                &trs.rules[i],
                &trs.rules[j],
                (i * 1000 + j * 1000 + 2000) as u32,
            );
            for (s, t) in pairs {
                let ns = trs.normalize_innermost(&s, limit);
                let nt = trs.normalize_innermost(&t, limit);
                if ns != nt {
                    return false;
                }
            }
        }
    }
    true
}
/// Ordering function type: returns `Ordering` for two terms.
pub type TermOrdering = fn(&Term, &Term) -> std::cmp::Ordering;
/// Apply one reduction step using the given strategy.
pub fn reduce_step(trs: &Trs, term: &Term, strategy: Strategy) -> Option<Term> {
    match strategy {
        Strategy::Innermost | Strategy::Lazy => trs.reduce_innermost(term),
        Strategy::Outermost | Strategy::Parallel => trs.reduce_outermost(term),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn f(args: Vec<Term>) -> Term {
        Term::Fun("f".into(), args)
    }
    fn g(args: Vec<Term>) -> Term {
        Term::Fun("g".into(), args)
    }
    fn a() -> Term {
        Term::Fun("a".into(), vec![])
    }
    fn b() -> Term {
        Term::Fun("b".into(), vec![])
    }
    fn x0() -> Term {
        Term::Var(0)
    }
    fn x1() -> Term {
        Term::Var(1)
    }
    /// Verify that the kernel environment builds without errors.
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_term_rewriting_env(&mut env);
        assert!(result.is_ok());
    }
    /// Test basic unification: f(x0, a) =? f(b, y1).
    #[test]
    fn test_unification_basic() {
        let s = f(vec![x0(), a()]);
        let t = f(vec![b(), x1()]);
        let mgu = unify(&s, &t).expect("should unify");
        let s2 = s.apply(&mgu);
        let t2 = t.apply(&mgu);
        assert_eq!(s2, t2);
    }
    /// Unification should fail when occurs-check fires.
    #[test]
    fn test_unification_occurs_check() {
        let result = unify(&x0(), &f(vec![x0()]));
        assert!(result.is_none(), "occurs check should prevent self-loop");
    }
    /// Unification of incompatible function symbols should fail.
    #[test]
    fn test_unification_clash() {
        let result = unify(&f(vec![a()]), &g(vec![a()]));
        assert!(result.is_none());
    }
    /// Test innermost normalization on a simple TRS: f(f(x)) → f(x)
    #[test]
    fn test_innermost_normalization() {
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(f(vec![f(vec![x0()])]), f(vec![x0()])));
        let term = f(vec![f(vec![f(vec![a()])])]);
        let nf = trs.normalize_innermost(&term, 10);
        assert_eq!(nf, f(vec![a()]));
    }
    /// Test SRS word problem: ba → ab style commutation.
    #[test]
    fn test_srs_word_problem() {
        let mut srs = Srs::new();
        srs.add_rule("ba", "ab");
        let nf = srs.normalize("bba", 10);
        assert_eq!(nf, "abb");
    }
    /// Test SRS: rules that reduce to empty string.
    #[test]
    fn test_srs_reduction_to_empty() {
        let mut srs = Srs::new();
        srs.add_rule("ab", "");
        let nf = srs.normalize("aabb", 10);
        assert_eq!(nf, "");
    }
    /// Test critical pair detection for a trivial confluent system.
    #[test]
    fn test_critical_pairs_confluent() {
        let r1 = Rule::new(f(vec![a()]), b());
        let r2 = Rule::new(f(vec![a()]), b());
        let pairs = critical_pairs(&r1, &r2, 100);
        for (s, t) in &pairs {
            assert_eq!(s, t, "non-trivial critical pair: {} ≠ {}", s, t);
        }
    }
    /// Test local confluence check on a known confluent system.
    #[test]
    fn test_local_confluence() {
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(f(vec![a()]), b()));
        assert!(check_local_confluence(&trs, 20));
    }
    /// Test DependencyPairGraph construction from a simple TRS.
    #[test]
    fn test_dependency_pair_graph_from_trs() {
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(
            Term::Fun("f".into(), vec![Term::Var(0)]),
            Term::Fun("g".into(), vec![Term::Var(0)]),
        ));
        trs.add_rule(Rule::new(Term::Fun("g".into(), vec![Term::Var(0)]), a()));
        let graph = DependencyPairGraph::from_trs(&trs);
        assert!(!graph.pairs.is_empty());
    }
    /// Test that a simple TRS with no recursion has all-trivial SCCs.
    #[test]
    fn test_dependency_pair_graph_trivial_sccs() {
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(Term::Fun("f".into(), vec![Term::Var(0)]), a()));
        let graph = DependencyPairGraph::from_trs(&trs);
        assert!(graph.all_sccs_trivial());
    }
    /// Test PolynomialInterpretation orients a simple decreasing rule.
    #[test]
    fn test_polynomial_interpretation_termination() {
        let mut interp = PolynomialInterpretation::new();
        interp.set("f", vec![1, 1]);
        let rule = Rule::new(Term::Fun("f".into(), vec![Term::Var(0)]), Term::Var(0));
        assert!(interp.orients_rule(&rule, 5));
    }
    /// Test PolynomialInterpretation fails for a non-decreasing rule.
    #[test]
    fn test_polynomial_interpretation_fails() {
        let mut interp = PolynomialInterpretation::new();
        interp.set("f", vec![1, 1]);
        let rule = Rule::new(
            Term::Fun("f".into(), vec![Term::Var(0)]),
            Term::Fun("f".into(), vec![Term::Var(0)]),
        );
        assert!(!interp.orients_rule(&rule, 3));
    }
    /// Test NarrowingSystem: simple one-step narrowing.
    #[test]
    fn test_narrowing_step() {
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(f(vec![a()]), b()));
        let mut ns = NarrowingSystem::new(trs);
        let term = f(vec![Term::Var(0)]);
        let steps = ns.narrow_step(&term);
        assert!(!steps.is_empty());
        assert!(steps.iter().any(|(_, t)| *t == b()));
    }
    /// Test TreeAutomaton accepts a simple term.
    #[test]
    fn test_tree_automaton_accepts() {
        let mut ta = TreeAutomaton::new(2);
        ta.add_final(1);
        ta.add_transition("a", vec![], 0);
        ta.add_transition("b", vec![], 0);
        ta.add_transition("f", vec![0], 1);
        assert!(ta.accepts(&f(vec![a()])));
        assert!(!ta.accepts(&a()));
    }
    /// Test TreeAutomaton non-empty language check.
    #[test]
    fn test_tree_automaton_nonempty() {
        let mut ta = TreeAutomaton::new(2);
        ta.add_final(1);
        ta.add_transition("a", vec![], 0);
        ta.add_transition("f", vec![0], 1);
        assert!(!ta.is_empty());
    }
    /// Test DependencyPairGraph SCC detection on a cycle.
    #[test]
    fn test_dependency_pair_graph_cycle() {
        let mut graph = DependencyPairGraph::new();
        let i = graph.add_pair("f", "g");
        let j = graph.add_pair("g", "f");
        graph.add_edge(i, j);
        graph.add_edge(j, i);
        assert!(!graph.all_sccs_trivial());
    }
    /// Test that PolynomialInterpretation proves termination of f(f(x)) → f(x).
    #[test]
    fn test_polynomial_interpretation_proves_termination() {
        let mut interp = PolynomialInterpretation::new();
        interp.set("f", vec![1, 1]);
        let mut trs = Trs::new();
        trs.add_rule(Rule::new(f(vec![f(vec![x0()])]), f(vec![x0()])));
        assert!(interp.proves_termination(&trs, 5));
    }
    /// Test TreeAutomaton empty language detection.
    #[test]
    fn test_tree_automaton_empty_language() {
        let mut ta = TreeAutomaton::new(2);
        ta.add_final(1);
        assert!(ta.is_empty());
    }
    /// Test NarrowingSystem basic_narrow with depth 0 returns input term.
    #[test]
    fn test_narrowing_depth_zero() {
        let trs = Trs::new();
        let mut ns = NarrowingSystem::new(trs);
        let term = f(vec![a()]);
        let results = ns.basic_narrow(&term, 0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, term);
    }
}
#[cfg(test)]
mod tests_term_rewriting_ext {
    use super::*;
    #[test]
    fn test_string_rewriting() {
        let mut srs = StringRewritingSystem::new(vec!['a', 'b']);
        srs.add_rule("aa", "a");
        srs.add_rule("bb", "b");
        let result = srs.normalize("aabb", 10);
        assert_eq!(result, "ab", "aabb -> ab after normalizing");
    }
    #[test]
    fn test_string_rewriting_no_rule() {
        let srs = StringRewritingSystem::new(vec!['a', 'b']);
        let result = srs.normalize("abc", 10);
        assert_eq!(result, "abc");
        assert_eq!(srs.num_rules(), 0);
    }
    #[test]
    fn test_egraph_union_find() {
        let mut eg = EGraph::new();
        let n1 = eg.add_node("x");
        let n2 = eg.add_node("y");
        let n3 = eg.add_node("x+1");
        assert!(!eg.are_equal(n1, n2));
        eg.union(n1, n2);
        assert!(eg.are_equal(n1, n2));
        assert!(!eg.are_equal(n1, n3));
        assert_eq!(eg.num_classes(), 3);
    }
    #[test]
    fn test_rewriting_logic_theory() {
        let mut th = RewritingLogicTheory::new();
        th.add_sort("State");
        th.add_equation("0 + x", "x");
        th.add_rw_rule("step", "s → t", "t");
        assert!(!th.is_equational());
        assert_eq!(th.signature_size(), 2);
        assert!(th.entailment_description().contains("1 sorts"));
    }
    #[test]
    fn test_string_rewriting_equal_modulo() {
        let mut srs = StringRewritingSystem::new(vec!['a', 'b']);
        srs.add_rule("ab", "ba");
        assert!(srs.are_equal_modulo("ab", "ba", 5));
    }
}
#[cfg(test)]
mod tests_term_rewriting_ext2 {
    use super::*;
    #[test]
    fn test_knuth_bendix() {
        let mut kb = KnuthBendixData::new("LPO");
        kb.add_oriented_rule("i(i(x))", "x");
        kb.add_oriented_rule("e * x", "x");
        kb.add_critical_pair("i(i(x))", "x");
        kb.mark_confluent();
        assert!(kb.is_confluent);
        assert_eq!(kb.num_rules(), 2);
        assert!(kb.description().contains("confluent=true"));
    }
}
#[cfg(test)]
mod tests_term_rewriting_ext3 {
    use super::*;
    #[test]
    fn test_reduction_strategy() {
        let s = ReductionStrategy::LeftmostOutermost;
        assert!(s.is_complete());
        assert!(s.normalizing_for_orthogonal());
        assert!(s.name().contains("Normal"));
        let li = ReductionStrategy::LeftmostInnermost;
        assert!(!li.is_complete());
        assert_eq!(li.lambda_calculus_analog(), "Call-by-value");
        let needed = ReductionStrategy::Needed;
        assert!(needed.is_complete());
        assert_eq!(needed.lambda_calculus_analog(), "Call-by-need (lazy)");
    }
}
