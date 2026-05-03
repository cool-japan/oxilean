//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BigFiveStats, BigFiveSystem, ComputableFunction, ConservationResult, ConstructivePrinciple,
    IndependenceResult, OmegaModel, Pi11Sentence, ProofSystem, RCA0AxiomKind, RMA0System,
    RMHierarchy, RMPrinciple, RMScoreboard, RMStrength, RMTheorem, RamseyColoringFinder,
    WeakKonigTree,
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
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
/// SecondOrderArithmetic: the ambient formal system (ℕ + set variables).
/// SecondOrderArithmetic : Type
pub fn second_order_arithmetic_ty() -> Expr {
    type0()
}
/// ArithmeticalFormula: a formula with only numeric (first-order) quantifiers.
/// ArithmeticalFormula : Prop
pub fn arithmetical_formula_ty() -> Expr {
    prop()
}
/// Sigma01Formula: bounded existential formula.
pub fn sigma01_formula_ty() -> Expr {
    prop()
}
/// Pi01Formula: bounded universal formula.
pub fn pi01_formula_ty() -> Expr {
    prop()
}
/// Delta01Formula: both Σ⁰_1 and Π⁰_1 (recursive / decidable).
pub fn delta01_formula_ty() -> Expr {
    prop()
}
/// Sigma11Formula: formula with a leading existential set quantifier.
pub fn sigma11_formula_ty() -> Expr {
    prop()
}
/// Pi11Formula: formula with a leading universal set quantifier.
pub fn pi11_formula_ty() -> Expr {
    prop()
}
/// RCA0: Recursive Comprehension Axiom (base system).
/// RCA0 : Prop  (provable sentences in the system)
pub fn rca0_ty() -> Expr {
    prop()
}
/// Provable_RCA0: a sentence φ is provable in RCA₀.
/// Provable_RCA0 : ArithmeticalFormula → Prop
pub fn provable_rca0_ty() -> Expr {
    arrow(arithmetical_formula_ty(), prop())
}
/// WKL0: Weak König's Lemma (over RCA₀).
/// WKL0 : Prop
pub fn wkl0_ty() -> Expr {
    prop()
}
/// WeakKonigsLemma: every infinite binary tree has an infinite path.
/// WeakKonigsLemma : BinaryTree → Prop → Prop
pub fn weak_konigs_lemma_ty() -> Expr {
    arrow(
        cst("BinaryTree"),
        arrow(app(cst("IsInfinite"), cst("BinaryTree")), prop()),
    )
}
/// ACA0: Arithmetical Comprehension Axiom.
/// ACA0 : Prop
pub fn aca0_ty() -> Expr {
    prop()
}
/// ArithmeticalComprehension: ∀ arithmetical φ, the set {n | φ(n)} exists.
/// ArithmeticalComprehension : ArithmeticalFormula → Prop
pub fn arithmetical_comprehension_ty() -> Expr {
    arrow(arithmetical_formula_ty(), prop())
}
/// ATR0: Arithmetical Transfinite Recursion.
/// ATR0 : Prop
pub fn atr0_ty() -> Expr {
    prop()
}
/// ArithmeticalTransfiniteRecursion:
///   for any well-ordering ≺ and arithmetical operation φ,
///   the hierarchy indexed by ≺ exists.
pub fn arithmetical_transfinite_recursion_ty() -> Expr {
    arrow(
        cst("WellOrdering"),
        arrow(arithmetical_formula_ty(), prop()),
    )
}
/// Pi11CA0: Π¹_1 Comprehension Axiom.
/// Pi11CA0 : Prop
pub fn pi11_ca0_ty() -> Expr {
    prop()
}
/// Pi11Comprehension: ∀ Π¹_1 formula φ, the set {n | φ(n)} exists.
pub fn pi11_comprehension_ty() -> Expr {
    arrow(pi11_formula_ty(), prop())
}
/// Conservative: system S₁ is conservative over S₂ for class Γ.
/// Conservative : System → System → FormulaClass → Prop
pub fn conservative_ty() -> Expr {
    arrow(
        cst("System"),
        arrow(cst("System"), arrow(cst("FormulaClass"), prop())),
    )
}
/// WKL0ConservativeOverRCA0:
///   WKL₀ is Π¹_1-conservative over RCA₀.
///   (Any Π¹_1 sentence provable in WKL₀ is already provable in RCA₀.)
pub fn wkl0_conservative_over_rca0_ty() -> Expr {
    app3(
        cst("Conservative"),
        cst("WKL0"),
        cst("RCA0"),
        cst("Pi11FormulasClass"),
    )
}
/// ACA0EquivalentToPA:
///   ACA₀ is conservative over first-order Peano Arithmetic (PA).
///   For every first-order sentence φ: ACA₀ ⊢ φ ↔ PA ⊢ φ.
pub fn aca0_conservative_over_pa_ty() -> Expr {
    app3(
        cst("Conservative"),
        cst("ACA0"),
        cst("PeanoArithmetic"),
        cst("FirstOrderFormulas"),
    )
}
/// ATR0ConservativeOverRCA0ForPi12:
///   ATR₀ is Π¹_2-conservative over ACA₀.
pub fn atr0_conservative_ty() -> Expr {
    app3(
        cst("Conservative"),
        cst("ATR0"),
        cst("ACA0"),
        cst("Pi12FormulasClass"),
    )
}
/// OmegaModelWKL0:
///   Every countable coded ω-model of RCA₀ embeds into a model of WKL₀.
pub fn omega_model_wkl0_ty() -> Expr {
    arrow(
        app(cst("OmegaModel"), cst("RCA0")),
        app(cst("OmegaModel"), cst("WKL0")),
    )
}
/// OrderTypeEquivalence: the subsystems form a linear order under provability.
/// RCA₀ < WKL₀ < ACA₀ < ATR₀ < Π¹_1-CA₀
pub fn subsystem_linear_order_ty() -> Expr {
    arrow(cst("System"), arrow(cst("System"), prop()))
}
/// BolzanoWeierstrass: every bounded sequence of reals has a convergent subsequence.
/// Equivalent to ACA₀ over RCA₀.
pub fn bolzano_weierstrass_ty() -> Expr {
    arrow(
        app(cst("BoundedSequence"), cst("Real")),
        app(cst("HasConvergentSubsequence"), bvar(0)),
    )
}
/// HahnBanachTheorem: over WKL₀.
pub fn hahn_banach_ty() -> Expr {
    arrow(cst("NormedSpace"), arrow(cst("LinearFunctional"), prop()))
}
/// BrouwerFixedPoint: every continuous function from disk to disk has a fixed point.
/// Equivalent to WKL₀ over RCA₀.
pub fn brouwer_fixed_point_ty() -> Expr {
    impl_pi(
        "f",
        arrow(cst("Disk"), cst("Disk")),
        arrow(
            app(cst("Continuous"), bvar(0)),
            app(cst("HasFixedPoint"), bvar(1)),
        ),
    )
}
/// MaximalIdealTheorem: every commutative ring has a maximal ideal.
/// Equivalent to WKL₀ over RCA₀.
pub fn maximal_ideal_theorem_ty() -> Expr {
    impl_pi("R", cst("CommRing"), app(cst("HasMaximalIdeal"), bvar(0)))
}
/// CompletenessTheorem: Gödel completeness for countable languages.
/// Equivalent to WKL₀ over RCA₀.
pub fn completeness_theorem_ty() -> Expr {
    impl_pi(
        "L",
        cst("CountableLanguage"),
        arrow(
            app(cst("Consistent"), bvar(0)),
            app(cst("HasModel"), bvar(1)),
        ),
    )
}
/// KonigLemma: every infinite, finitely-branching tree has an infinite path.
/// Equivalent to WKL₀ over RCA₀ for binary trees (the full Königs lemma is ACA₀).
pub fn konig_lemma_ty() -> Expr {
    arrow(
        cst("FinBranchingTree"),
        arrow(
            app(cst("IsInfiniteTree"), bvar(0)),
            app(cst("HasInfinitePath"), bvar(1)),
        ),
    )
}
/// RamseyN2K: Ramsey's theorem for n-tuples with k colors.
/// Ramsey(n, k) : Prop — every k-coloring of \[ℕ\]^n has an infinite homogeneous set.
pub fn ramsey_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// RT22: Ramsey's theorem for pairs with 2 colors (Ramsey(2, 2)).
/// This is the most-studied case; its exact strength is between RCA₀ and ACA₀.
pub fn rt22_ty() -> Expr {
    prop()
}
/// RT21: Ramsey's theorem for pairs, stable version (Σ⁰_2-conservative over RCA₀).
pub fn rt21_ty() -> Expr {
    prop()
}
/// SRT22: Stable Ramsey's theorem for pairs (Cholak–Jockusch–Slaman).
pub fn srt22_ty() -> Expr {
    prop()
}
/// CAC: Chain-Antichain principle (Dilworth).
/// Every partial order on ℕ has an infinite chain or antichain.
/// CAC ↔ ACA₀ over RCA₀ (Hirschfeldt–Shore).
pub fn cac_ty() -> Expr {
    impl_pi(
        "P",
        cst("PartialOrder"),
        arrow(
            app(cst("IsInfinitePoset"), bvar(0)),
            app2(cst("ChainOrAntichain"), bvar(1), bvar(0)),
        ),
    )
}
/// ADS: Ascending / Descending Sequence principle.
/// Every infinite linear order has an infinite ascending or descending sequence.
pub fn ads_ty() -> Expr {
    impl_pi(
        "L",
        cst("LinearOrder"),
        arrow(
            app(cst("IsInfiniteOrder"), bvar(0)),
            app2(cst("AscOrDescSeq"), bvar(1), bvar(0)),
        ),
    )
}
/// SADS: Stable Ascending/Descending Sequence.
pub fn sads_ty() -> Expr {
    impl_pi(
        "L",
        cst("LinearOrder"),
        arrow(
            app(cst("IsStableOrder"), bvar(0)),
            app2(cst("AscOrDescSeq"), bvar(1), bvar(0)),
        ),
    )
}
/// DNR: Diagonally Non-recursive functions exist.
/// Strictly between RCA₀ and WKL₀.
pub fn dnr_ty() -> Expr {
    app(cst("Exists"), cst("DiagonallyNonRecursive"))
}
/// FSSets: the set of finite sums of an infinite sequence.
/// FSSets : (Nat → Nat) → Set Nat
pub fn fssets_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), app(cst("Set"), nat_ty()))
}
/// HindmanTheorem: for any finite coloring of ℕ there is an infinite set
/// whose finite sum set is monochromatic.
/// HindmanTheorem : Nat → Prop
///   (parameterized by number of colors)
pub fn hindman_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// IdempotentUltrafilter: an ultrafilter p such that p + p = p (in βℕ).
/// IdempotentUltrafilter : Type
pub fn idempotent_ultrafilter_ty() -> Expr {
    type0()
}
/// HindmanFromIdempotent: Hindman's theorem follows from the existence of
/// idempotent ultrafilters.
pub fn hindman_from_idempotent_ty() -> Expr {
    arrow(cst("IdempotentUltrafilter"), hindman_theorem_ty())
}
/// HindmanStrength: Hindman's theorem is provable in ACA₀⁺ but not in ACA₀.
/// (Blass–Hirst–Simpson)
pub fn hindman_strength_ty() -> Expr {
    app2(cst("ProvableIn"), cst("HindmanTheorem"), cst("ACA0Plus"))
}
/// Compute a lower bound on the Ramsey number R(s, t) using the inequality
/// R(s, t) ≥ R(s-1, t) + R(s, t-1) (Erdős–Szekeres recursion lower bound).
/// Returns exact values for small cases.
pub fn ramsey_number_lower_bound(s: u32, t: u32) -> u32 {
    match (s, t) {
        (1, _) => 1,
        (_, 1) => 1,
        (2, k) | (k, 2) => k,
        (3, 3) => 6,
        (3, 4) | (4, 3) => 9,
        (3, 5) | (5, 3) => 14,
        (3, 6) | (6, 3) => 18,
        (3, 7) | (7, 3) => 23,
        (3, 8) | (8, 3) => 28,
        (3, 9) | (9, 3) => 36,
        (4, 4) => 18,
        (4, 5) | (5, 4) => 25,
        _ => {
            let n = (s + t - 2) as u64;
            let k = (s - 1) as u64;
            let mut binom: u64 = 1;
            for i in 0..k {
                binom = binom.saturating_mul(n - i).saturating_div(i + 1);
            }
            binom.min(u32::MAX as u64) as u32
        }
    }
}
/// Check whether a coloring of pairs from {0..n} is a valid k-coloring
/// (all values are < k).
pub fn is_valid_coloring(n: usize, coloring: &[Vec<u32>], k: u32) -> bool {
    for i in 0..n {
        if coloring.len() <= i {
            return false;
        }
        for j in (i + 1)..n {
            if coloring[i].len() <= j {
                return false;
            }
            if coloring[i][j] >= k {
                return false;
            }
        }
    }
    true
}
/// Find the largest monochromatic clique in a 2-coloring of pairs.
/// Returns (color, set of vertices forming an approximately maximal monochromatic set).
/// Uses a greedy approach for tractability.
pub fn greedy_homogeneous_set(n: usize, coloring: &[Vec<u32>]) -> (u32, Vec<usize>) {
    let mut best: (u32, Vec<usize>) = (0, vec![]);
    for start_color in 0..2u32 {
        let mut hom_set = vec![0usize];
        for v in 1..n {
            let monochromatic = hom_set.iter().all(|&u| {
                let (i, j) = (u.min(v), u.max(v));
                coloring
                    .get(i)
                    .and_then(|row| row.get(j))
                    .copied()
                    .unwrap_or(2)
                    == start_color
            });
            if monochromatic {
                hom_set.push(v);
            }
        }
        if hom_set.len() > best.1.len() {
            best = (start_color, hom_set);
        }
    }
    best
}
/// ComputableFunction: a (partial) function ℕ → ℕ computed by a Turing machine.
/// ComputableFunction : Type
pub fn computable_function_ty() -> Expr {
    type0()
}
/// TuringDegree: an equivalence class of sets under Turing reducibility.
/// TuringDegree : Type
pub fn turing_degree_ty() -> Expr {
    type0()
}
/// TuringReducible: A is Turing-reducible to B (A ≤_T B).
/// TuringReducible : Set Nat → Set Nat → Prop
pub fn turing_reducible_ty() -> Expr {
    arrow(
        app(cst("Set"), nat_ty()),
        arrow(app(cst("Set"), nat_ty()), prop()),
    )
}
/// ComputablelyEnumerable: X is computably enumerable (Σ⁰_1 set).
/// ComputablelyEnumerable : Set Nat → Prop
pub fn computably_enumerable_ty() -> Expr {
    arrow(app(cst("Set"), nat_ty()), prop())
}
/// HaltingProblem: the set K = {e | φ_e(e) ↓} is c.e. but not computable.
/// HaltingProblem : Set Nat
pub fn halting_problem_ty() -> Expr {
    app(cst("Set"), nat_ty())
}
/// HaltingProblemIsCE: K is computably enumerable (provable in RCA₀).
pub fn halting_problem_is_ce_ty() -> Expr {
    app(cst("ComputablelyEnumerable"), cst("HaltingProblem"))
}
/// HaltingProblemNotComputable: K is not computable (provable in RCA₀ via diagonalisation).
pub fn halting_problem_not_computable_ty() -> Expr {
    app(cst("Not"), app(cst("Computable"), cst("HaltingProblem")))
}
/// PostTheorem: X is c.e. iff X is Σ⁰_1-definable (provable in RCA₀).
/// PostTheorem : Set Nat → Prop
pub fn post_theorem_ty() -> Expr {
    arrow(
        app(cst("Set"), nat_ty()),
        arrow(
            app(cst("ComputablelyEnumerable"), bvar(0)),
            app(cst("Sigma01Definable"), bvar(1)),
        ),
    )
}
/// RecursiveSeparation: two disjoint c.e. sets can be separated by a computable set
/// iff one reduces to the other — the c.e. non-computable separation.
pub fn recursive_separation_ty() -> Expr {
    arrow(
        app(cst("Set"), nat_ty()),
        arrow(
            app(cst("Set"), nat_ty()),
            arrow(
                app(cst("Disjoint"), bvar(0)),
                app2(cst("HasComputableSeparation"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// OracleComputable: f is computable relative to oracle X.
/// OracleComputable : (Set Nat) → (Nat → Nat) → Prop
pub fn oracle_computable_ty() -> Expr {
    arrow(
        app(cst("Set"), nat_ty()),
        arrow(arrow(nat_ty(), nat_ty()), prop()),
    )
}
/// InfiniteBinaryTree: a subtree of 2^{<ω} with no infinite path.
/// InfiniteBinaryTree : Type
pub fn infinite_binary_tree_ty() -> Expr {
    type0()
}
/// KonigsLemmaForBinaryTrees: every infinite binary tree has an infinite branch.
/// This is exactly WKL₀ (over RCA₀).
pub fn konigs_lemma_binary_ty() -> Expr {
    impl_pi(
        "T",
        cst("InfiniteBinaryTree"),
        app(cst("HasInfiniteBranch"), bvar(0)),
    )
}
/// HasInfiniteBranch: an infinite binary tree T has an infinite branch.
/// HasInfiniteBranch : InfiniteBinaryTree → Prop
pub fn has_infinite_branch_ty() -> Expr {
    arrow(cst("InfiniteBinaryTree"), prop())
}
/// BinaryTreePath: a path (branch) through a binary tree up to length n.
/// BinaryTreePath : Nat → Type
pub fn binary_tree_path_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// WKL0EquivalentToKonigsLemma: the two formulations are interderivable over RCA₀.
pub fn wkl0_equiv_konig_ty() -> Expr {
    app2(cst("Iff"), cst("WKL0"), cst("KonigsLemmaForBinaryTrees"))
}
/// HyperArithmetical: a set X ⊆ ℕ is hyperarithmetical (in Δ¹_1).
/// HyperArithmetical : Set Nat → Prop
pub fn hyperarithmetical_ty() -> Expr {
    arrow(app(cst("Set"), nat_ty()), prop())
}
/// HyperArithmeticalHierarchy: the hierarchy indexed by recursive ordinals.
/// HyperArithmeticalHierarchy : RecursiveOrdinal → Set Nat → Prop
pub fn hyperarithmetical_hierarchy_ty() -> Expr {
    arrow(
        cst("RecursiveOrdinal"),
        arrow(app(cst("Set"), nat_ty()), prop()),
    )
}
/// Pi11Comprehension0: Π¹_1-CA₀ directly asserts set existence for Π¹_1 formulas.
/// Equivalent to: every Π¹_1 set exists.
pub fn pi11_ca0_set_existence_ty() -> Expr {
    impl_pi(
        "phi",
        pi11_formula_ty(),
        app(cst("Exists"), app(cst("SetOf"), bvar(0))),
    )
}
/// BarInduction: Brouwer's bar induction principle.
/// Equivalent to ATR₀ over RCA₀ (in the linear-order formulation).
/// BarInduction : WellFoundedTree → Prop
pub fn bar_induction_ty() -> Expr {
    arrow(cst("WellFoundedTree"), prop())
}
/// OpenDeterminacy: every open game on ℕ is determined.
/// Equivalent to ATR₀ over RCA₀ (Steel 1976).
pub fn open_determinacy_ty() -> Expr {
    impl_pi("G", cst("OpenGame"), app(cst("IsDetermined"), bvar(0)))
}
/// BorelDeterminacy: every Borel game on ℕ is determined (Martin 1975).
/// Requires at least Σ¹_1 sets; not provable in second-order arithmetic.
pub fn borel_determinacy_ty() -> Expr {
    impl_pi("G", cst("BorelGame"), app(cst("IsDetermined"), bvar(0)))
}
/// ProjectiveDeterminacy: every projective game is determined.
/// Follows from large cardinal axioms (Woodin cardinals).
pub fn projective_determinacy_ty() -> Expr {
    impl_pi(
        "G",
        cst("ProjectiveGame"),
        app(cst("IsDetermined"), bvar(0)),
    )
}
/// Sigma11Determinacy: every Σ¹_1 (analytic) game is determined.
/// Equivalent to Π¹_1-CA₀ over ATR₀ (Tanaka 1990).
pub fn sigma11_determinacy_ty() -> Expr {
    impl_pi("G", cst("Sigma11Game"), app(cst("IsDetermined"), bvar(0)))
}
/// WellOrderingTheorem: every set can be well-ordered.
/// Equivalent to ATR₀ over RCA₀ (for countable sets, Friedman).
pub fn well_ordering_theorem_ty() -> Expr {
    impl_pi("S", type0(), app(cst("CanBeWellOrdered"), bvar(0)))
}
/// ComparisonOfWellOrderings: any two well-orderings are comparable.
/// Equivalent to ATR₀ over RCA₀.
pub fn comparison_of_well_orderings_ty() -> Expr {
    arrow(
        cst("WellOrdering"),
        arrow(
            cst("WellOrdering"),
            app2(cst("ComparableOrders"), bvar(0), bvar(0)),
        ),
    )
}
/// WellOrderingIsLinear: every well-ordering is a linear ordering.
/// Provable in RCA₀.
pub fn well_ordering_is_linear_ty() -> Expr {
    impl_pi("W", cst("WellOrdering"), app(cst("IsLinearOrder"), bvar(0)))
}
/// InfiniteLinearOrderHasOmegaOrOmegaStar:
///   Every countably infinite linear ordering contains a copy of ω or ω*.
///   Equivalent to ACA₀ over RCA₀.
pub fn linear_order_omega_ty() -> Expr {
    impl_pi(
        "L",
        cst("LinearOrder"),
        arrow(
            app(cst("IsInfiniteOrder"), bvar(0)),
            app2(cst("ContainsOmegaOrOmegaStar"), bvar(1), bvar(0)),
        ),
    )
}
/// ScattereredLinearOrdering: L has no dense sub-ordering.
/// ScatteredLinearOrdering : LinearOrder → Prop
pub fn scattered_linear_ordering_ty() -> Expr {
    arrow(cst("LinearOrder"), prop())
}
/// HausdorffScatteredCharacterization: L is scattered iff it embeds no η-type ordering.
/// Equivalent to ATR₀ over RCA₀.
pub fn hausdorff_scattered_ty() -> Expr {
    impl_pi(
        "L",
        cst("LinearOrder"),
        arrow(
            app(cst("ScatteredLinearOrdering"), bvar(0)),
            app(cst("EmbedsFreeOfEta"), bvar(1)),
        ),
    )
}
/// ThinSetTheorem: for every f : \[ℕ\]² → k, there is an infinite set S
/// such that f omits at least one color on \[S\]².
/// Strength: strictly between SRT²_2 and RT²_2.
pub fn thin_set_theorem_ty() -> Expr {
    arrow(
        nat_ty(),
        impl_pi(
            "f",
            arrow(app(cst("Pairs"), nat_ty()), bvar(1)),
            app(cst("HasThinInfiniteSet"), bvar(0)),
        ),
    )
}
/// FreeSetTheorem: for every f : \[ℕ\]² → ℕ, there is an infinite set S
/// such that f(x,y) ∉ S for all x,y ∈ S with x ≠ y.
/// Equivalent to RT²_2 over RCA₀ (Cholak–Giusto–Hirst–Jockusch).
pub fn free_set_theorem_ty() -> Expr {
    impl_pi(
        "f",
        arrow(app(cst("Pairs"), nat_ty()), nat_ty()),
        app(cst("HasFreeInfiniteSet"), bvar(0)),
    )
}
/// CohesivenessTheorem: for every sequence of sets (X_i), there is an infinite
/// set S almost contained in each X_i or its complement.
/// COH: strictly between RCA₀ and RT²_2 (Jockusch–Lempp–Slaman).
pub fn cohesiveness_theorem_ty() -> Expr {
    impl_pi(
        "seq",
        arrow(nat_ty(), app(cst("Set"), nat_ty())),
        app(cst("HasCohesiveSet"), bvar(0)),
    )
}
/// OmittingTypesTheorem: every consistent theory omits a non-principal type
/// unless it has a prime model.
/// Equivalent to ACA₀ over RCA₀ (Hirschfeldt–Shore).
pub fn omitting_types_theorem_ty() -> Expr {
    impl_pi(
        "T",
        cst("ConsistentTheory"),
        arrow(
            app(cst("HasNonPrincipalType"), bvar(0)),
            app(cst("HasOmittingModel"), bvar(1)),
        ),
    )
}
/// LowBasisTheorem: every infinite binary tree has an infinite branch of low degree.
/// Provable in RCA₀ (Jockusch–Soare 1972).
pub fn low_basis_theorem_ty() -> Expr {
    impl_pi(
        "T",
        cst("InfiniteBinaryTree"),
        app(cst("HasLowBranch"), bvar(0)),
    )
}
/// DecidabilityOfRCA0: it is decidable whether a given sentence is provable in RCA₀.
/// This is a meta-level Π⁰_1 statement.
pub fn decidability_rca0_ty() -> Expr {
    app(cst("IsDecidable"), cst("RCA0Provability"))
}
/// Sigma0nFormula: a Σ⁰_n formula (bounded quantifier alternations, n many).
/// Sigma0nFormula : Nat → Type
pub fn sigma0n_formula_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Pi0nFormula: a Π⁰_n formula.
pub fn pi0n_formula_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Delta11Formula: both Σ¹_1 and Π¹_1 (hyperarithmetical).
pub fn delta11_formula_ty() -> Expr {
    prop()
}
/// Sigma0nInduction: Σ⁰_n-induction axiom scheme.
/// Sigma0nInduction : Nat → Prop
pub fn sigma0n_induction_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Sigma01Comprehension: RCA₀'s comprehension scheme — sets exist for Σ⁰_1 formulas.
/// This is the comprehension half of RCA₀.
pub fn sigma01_comprehension_ty() -> Expr {
    arrow(sigma01_formula_ty(), prop())
}
/// Delta01Comprehension: sets exist for Δ⁰_1 (recursive) predicates.
/// Also called primitive recursive comprehension.
pub fn delta01_comprehension_ty() -> Expr {
    arrow(delta01_formula_ty(), prop())
}
/// Register all reverse-mathematics axioms into a fresh kernel environment.
pub fn build_reverse_mathematics_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("SecondOrderArithmetic", second_order_arithmetic_ty()),
        ("ArithmeticalFormula", arithmetical_formula_ty()),
        ("Sigma01Formula", sigma01_formula_ty()),
        ("Pi01Formula", pi01_formula_ty()),
        ("Delta01Formula", delta01_formula_ty()),
        ("Sigma11Formula", sigma11_formula_ty()),
        ("Pi11Formula", pi11_formula_ty()),
        ("RCA0", rca0_ty()),
        ("Provable_RCA0", provable_rca0_ty()),
        ("WKL0", wkl0_ty()),
        ("WeakKonigsLemma", weak_konigs_lemma_ty()),
        ("ACA0", aca0_ty()),
        ("ArithmeticalComprehension", arithmetical_comprehension_ty()),
        ("ATR0", atr0_ty()),
        (
            "ArithmeticalTransfiniteRecursion",
            arithmetical_transfinite_recursion_ty(),
        ),
        ("Pi11CA0", pi11_ca0_ty()),
        ("Pi11Comprehension", pi11_comprehension_ty()),
        ("Conservative", conservative_ty()),
        ("WKL0ConservativeOverRCA0", wkl0_conservative_over_rca0_ty()),
        ("ACA0ConservativeOverPA", aca0_conservative_over_pa_ty()),
        ("ATR0ConservativeResult", atr0_conservative_ty()),
        ("OmegaModelWKL0", omega_model_wkl0_ty()),
        ("SubsystemLinearOrder", subsystem_linear_order_ty()),
        ("BolzanoWeierstrass", bolzano_weierstrass_ty()),
        ("HahnBanachTheorem", hahn_banach_ty()),
        ("BrouwerFixedPoint", brouwer_fixed_point_ty()),
        ("MaximalIdealTheorem", maximal_ideal_theorem_ty()),
        ("CompletenessTheorem", completeness_theorem_ty()),
        ("KonigLemma", konig_lemma_ty()),
        ("Ramsey", ramsey_ty()),
        ("RT22", rt22_ty()),
        ("RT21", rt21_ty()),
        ("SRT22", srt22_ty()),
        ("CAC", cac_ty()),
        ("ADS", ads_ty()),
        ("SADS", sads_ty()),
        ("DNR", dnr_ty()),
        ("FSSets", fssets_ty()),
        ("HindmanTheorem", hindman_theorem_ty()),
        ("IdempotentUltrafilter", idempotent_ultrafilter_ty()),
        ("HindmanFromIdempotent", hindman_from_idempotent_ty()),
        ("HindmanStrength", hindman_strength_ty()),
        ("ComputableFunction", computable_function_ty()),
        ("TuringDegree", turing_degree_ty()),
        ("TuringReducible", turing_reducible_ty()),
        ("ComputablelyEnumerable", computably_enumerable_ty()),
        ("HaltingProblem", halting_problem_ty()),
        ("HaltingProblemIsCE", halting_problem_is_ce_ty()),
        (
            "HaltingProblemNotComputable",
            halting_problem_not_computable_ty(),
        ),
        ("PostTheorem", post_theorem_ty()),
        ("RecursiveSeparation", recursive_separation_ty()),
        ("OracleComputable", oracle_computable_ty()),
        ("InfiniteBinaryTree", infinite_binary_tree_ty()),
        ("KonigsLemmaForBinaryTrees", konigs_lemma_binary_ty()),
        ("HasInfiniteBranch", has_infinite_branch_ty()),
        ("BinaryTreePath", binary_tree_path_ty()),
        ("WKL0EquivalentToKonigsLemma", wkl0_equiv_konig_ty()),
        ("HyperArithmetical", hyperarithmetical_ty()),
        (
            "HyperArithmeticalHierarchy",
            hyperarithmetical_hierarchy_ty(),
        ),
        ("Pi11CA0SetExistence", pi11_ca0_set_existence_ty()),
        ("BarInduction", bar_induction_ty()),
        ("OpenDeterminacy", open_determinacy_ty()),
        ("BorelDeterminacy", borel_determinacy_ty()),
        ("ProjectiveDeterminacy", projective_determinacy_ty()),
        ("Sigma11Determinacy", sigma11_determinacy_ty()),
        ("WellOrderingTheorem", well_ordering_theorem_ty()),
        (
            "ComparisonOfWellOrderings",
            comparison_of_well_orderings_ty(),
        ),
        ("WellOrderingIsLinear", well_ordering_is_linear_ty()),
        (
            "InfiniteLinearOrderHasOmegaOrOmegaStar",
            linear_order_omega_ty(),
        ),
        ("ScatteredLinearOrdering", scattered_linear_ordering_ty()),
        ("HausdorffScattered", hausdorff_scattered_ty()),
        ("ThinSetTheorem", thin_set_theorem_ty()),
        ("FreeSetTheorem", free_set_theorem_ty()),
        ("CohesivenessTheorem", cohesiveness_theorem_ty()),
        ("OmittingTypesTheorem", omitting_types_theorem_ty()),
        ("LowBasisTheorem", low_basis_theorem_ty()),
        ("DecidabilityOfRCA0", decidability_rca0_ty()),
        ("Sigma0nFormula", sigma0n_formula_ty()),
        ("Pi0nFormula", pi0n_formula_ty()),
        ("Delta11Formula", delta11_formula_ty()),
        ("Sigma0nInduction", sigma0n_induction_ty()),
        ("Sigma01Comprehension", sigma01_comprehension_ty()),
        ("Delta01Comprehension", delta01_comprehension_ty()),
        ("System", type0()),
        ("FormulaClass", type0()),
        ("WellOrdering", type0()),
        ("BinaryTree", type0()),
        ("FinBranchingTree", type0()),
        ("PartialOrder", type0()),
        ("LinearOrder", type0()),
        ("OmegaModel", arrow(type0(), type0())),
        ("IsInfinite", arrow(type0(), prop())),
        ("IsInfiniteTree", arrow(cst("BinaryTree"), prop())),
        ("HasInfinitePath", arrow(cst("BinaryTree"), prop())),
        ("IsInfinitePoset", arrow(cst("PartialOrder"), prop())),
        ("IsInfiniteOrder", arrow(cst("LinearOrder"), prop())),
        ("IsStableOrder", arrow(cst("LinearOrder"), prop())),
        (
            "ChainOrAntichain",
            arrow(cst("PartialOrder"), arrow(type0(), prop())),
        ),
        (
            "AscOrDescSeq",
            arrow(cst("LinearOrder"), arrow(type0(), prop())),
        ),
        ("DiagonallyNonRecursive", type0()),
        ("PeanoArithmetic", type0()),
        ("FirstOrderFormulas", type0()),
        ("Pi11FormulasClass", type0()),
        ("Pi12FormulasClass", type0()),
        ("ACA0Plus", type0()),
        ("ProvableIn", arrow(type0(), arrow(type0(), prop()))),
        ("Real", type0()),
        ("BoundedSequence", arrow(type0(), type0())),
        ("HasConvergentSubsequence", arrow(type0(), prop())),
        ("NormedSpace", type0()),
        ("LinearFunctional", type0()),
        ("Disk", type0()),
        ("CommRing", type0()),
        ("CountableLanguage", type0()),
        ("Continuous", arrow(arrow(cst("Disk"), cst("Disk")), prop())),
        (
            "HasFixedPoint",
            arrow(arrow(cst("Disk"), cst("Disk")), prop()),
        ),
        ("HasMaximalIdeal", arrow(cst("CommRing"), prop())),
        ("Consistent", arrow(cst("CountableLanguage"), prop())),
        ("HasModel", arrow(cst("CountableLanguage"), prop())),
        ("Set", arrow(type0(), type0())),
        ("Not", arrow(prop(), prop())),
        ("Computable", arrow(app(cst("Set"), nat_ty()), prop())),
        ("Sigma01Definable", arrow(app(cst("Set"), nat_ty()), prop())),
        ("Disjoint", arrow(app(cst("Set"), nat_ty()), prop())),
        (
            "HasComputableSeparation",
            arrow(
                app(cst("Set"), nat_ty()),
                arrow(app(cst("Set"), nat_ty()), prop()),
            ),
        ),
        ("RecursiveOrdinal", type0()),
        ("WellFoundedTree", type0()),
        ("OpenGame", type0()),
        ("BorelGame", type0()),
        ("ProjectiveGame", type0()),
        ("Sigma11Game", type0()),
        ("IsDetermined", arrow(type0(), prop())),
        ("CanBeWellOrdered", arrow(type0(), prop())),
        (
            "ComparableOrders",
            arrow(cst("WellOrdering"), arrow(cst("WellOrdering"), prop())),
        ),
        ("IsLinearOrder", arrow(cst("WellOrdering"), prop())),
        (
            "ContainsOmegaOrOmegaStar",
            arrow(cst("LinearOrder"), arrow(type0(), prop())),
        ),
        ("EmbedsFreeOfEta", arrow(cst("LinearOrder"), prop())),
        ("Pairs", arrow(type0(), type0())),
        (
            "HasThinInfiniteSet",
            arrow(arrow(app(cst("Pairs"), nat_ty()), nat_ty()), prop()),
        ),
        (
            "HasFreeInfiniteSet",
            arrow(arrow(app(cst("Pairs"), nat_ty()), nat_ty()), prop()),
        ),
        (
            "HasCohesiveSet",
            arrow(arrow(nat_ty(), app(cst("Set"), nat_ty())), prop()),
        ),
        ("ConsistentTheory", type0()),
        (
            "HasNonPrincipalType",
            arrow(cst("ConsistentTheory"), prop()),
        ),
        ("HasOmittingModel", arrow(cst("ConsistentTheory"), prop())),
        ("HasLowBranch", arrow(cst("InfiniteBinaryTree"), prop())),
        ("IsDecidable", arrow(prop(), prop())),
        ("RCA0Provability", prop()),
        ("Exists", arrow(type0(), prop())),
        ("SetOf", arrow(type0(), type0())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
    ];
    for (name, ty) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        });
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_reverse_mathematics_env() {
        let env = build_reverse_mathematics_env();
        assert!(env.get(&Name::str("RCA0")).is_some());
        assert!(env.get(&Name::str("WKL0")).is_some());
        assert!(env.get(&Name::str("ACA0")).is_some());
        assert!(env.get(&Name::str("ATR0")).is_some());
        assert!(env.get(&Name::str("Pi11CA0")).is_some());
        assert!(env.get(&Name::str("WKL0ConservativeOverRCA0")).is_some());
        assert!(env.get(&Name::str("RT22")).is_some());
        assert!(env.get(&Name::str("HindmanTheorem")).is_some());
        assert!(env.get(&Name::str("IdempotentUltrafilter")).is_some());
    }
    #[test]
    fn test_big_five_ordering() {
        assert!(BigFiveSystem::RCA0 < BigFiveSystem::WKL0);
        assert!(BigFiveSystem::WKL0 < BigFiveSystem::ACA0);
        assert!(BigFiveSystem::ACA0 < BigFiveSystem::ATR0);
        assert!(BigFiveSystem::ATR0 < BigFiveSystem::Pi11CA0);
        assert!(BigFiveSystem::Pi11CA0.at_least_as_strong_as(&BigFiveSystem::RCA0));
    }
    #[test]
    fn test_big_five_names() {
        assert_eq!(BigFiveSystem::RCA0.name(), "RCA₀");
        assert_eq!(BigFiveSystem::WKL0.name(), "WKL₀");
        assert_eq!(BigFiveSystem::ACA0.name(), "ACA₀");
        assert_eq!(BigFiveSystem::ATR0.name(), "ATR₀");
        assert_eq!(BigFiveSystem::Pi11CA0.name(), "Π¹₁-CA₀");
    }
    #[test]
    fn test_proof_theoretic_ordinals() {
        assert_eq!(BigFiveSystem::ACA0.proof_theoretic_ordinal(), "ε₀");
        assert_eq!(BigFiveSystem::ATR0.proof_theoretic_ordinal(), "Γ₀");
    }
    #[test]
    fn test_conservation_results() {
        let c = ConservationResult::wkl0_over_rca0();
        assert!(c.is_valid_direction());
        assert_eq!(c.stronger, BigFiveSystem::WKL0);
        assert_eq!(c.weaker, BigFiveSystem::RCA0);
        let c2 = ConservationResult::atr0_over_aca0();
        assert!(c2.is_valid_direction());
    }
    #[test]
    fn test_rm_principles() {
        let rt22 = RMPrinciple::rt22();
        assert_eq!(rt22.strength, RMStrength::BetweenWKL0AndACA0);
        assert!(!rt22.equivalent_to(&BigFiveSystem::ACA0));
        let bw = RMPrinciple::bolzano_weierstrass();
        assert!(bw.equivalent_to(&BigFiveSystem::ACA0));
        let brouwer = RMPrinciple::brouwer();
        assert!(brouwer.equivalent_to(&BigFiveSystem::WKL0));
    }
    #[test]
    fn test_ramsey_number_lower_bound() {
        assert_eq!(ramsey_number_lower_bound(2, 5), 5);
        assert_eq!(ramsey_number_lower_bound(3, 3), 6);
        assert_eq!(ramsey_number_lower_bound(3, 4), 9);
        assert_eq!(ramsey_number_lower_bound(4, 4), 18);
        assert_eq!(
            ramsey_number_lower_bound(3, 5),
            ramsey_number_lower_bound(5, 3)
        );
    }
    #[test]
    fn test_greedy_homogeneous_set() {
        let n = 4;
        let coloring = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let (color, set) = greedy_homogeneous_set(n, &coloring);
        assert_eq!(color, 0);
        assert_eq!(set.len(), 4);
    }
    #[test]
    fn test_new_axioms_in_env() {
        let env = build_reverse_mathematics_env();
        assert!(env.get(&Name::str("ComputableFunction")).is_some());
        assert!(env.get(&Name::str("TuringDegree")).is_some());
        assert!(env.get(&Name::str("TuringReducible")).is_some());
        assert!(env.get(&Name::str("ComputablelyEnumerable")).is_some());
        assert!(env.get(&Name::str("HaltingProblem")).is_some());
        assert!(env.get(&Name::str("HaltingProblemIsCE")).is_some());
        assert!(env.get(&Name::str("HaltingProblemNotComputable")).is_some());
        assert!(env.get(&Name::str("PostTheorem")).is_some());
        assert!(env.get(&Name::str("OracleComputable")).is_some());
        assert!(env.get(&Name::str("InfiniteBinaryTree")).is_some());
        assert!(env.get(&Name::str("KonigsLemmaForBinaryTrees")).is_some());
        assert!(env.get(&Name::str("HasInfiniteBranch")).is_some());
        assert!(env.get(&Name::str("WKL0EquivalentToKonigsLemma")).is_some());
        assert!(env.get(&Name::str("HyperArithmetical")).is_some());
        assert!(env.get(&Name::str("BarInduction")).is_some());
        assert!(env.get(&Name::str("OpenDeterminacy")).is_some());
        assert!(env.get(&Name::str("BorelDeterminacy")).is_some());
        assert!(env.get(&Name::str("ProjectiveDeterminacy")).is_some());
        assert!(env.get(&Name::str("Sigma11Determinacy")).is_some());
        assert!(env.get(&Name::str("WellOrderingTheorem")).is_some());
        assert!(env.get(&Name::str("ComparisonOfWellOrderings")).is_some());
        assert!(env
            .get(&Name::str("InfiniteLinearOrderHasOmegaOrOmegaStar"))
            .is_some());
        assert!(env.get(&Name::str("HausdorffScattered")).is_some());
        assert!(env.get(&Name::str("ThinSetTheorem")).is_some());
        assert!(env.get(&Name::str("FreeSetTheorem")).is_some());
        assert!(env.get(&Name::str("CohesivenessTheorem")).is_some());
        assert!(env.get(&Name::str("OmittingTypesTheorem")).is_some());
        assert!(env.get(&Name::str("LowBasisTheorem")).is_some());
        assert!(env.get(&Name::str("Sigma0nFormula")).is_some());
        assert!(env.get(&Name::str("Delta11Formula")).is_some());
        assert!(env.get(&Name::str("Sigma0nInduction")).is_some());
        assert!(env.get(&Name::str("Delta01Comprehension")).is_some());
    }
    #[test]
    fn test_computable_function() {
        let f = ComputableFunction::indicator_below(5);
        assert_eq!(f.eval(0), Some(1));
        assert_eq!(f.eval(4), Some(1));
        assert_eq!(f.eval(5), Some(0));
        assert!(f.is_total_up_to(8));
        assert_eq!(f.oracle_query(3, 100), Some(1));
    }
    #[test]
    fn test_weak_konig_tree_complete() {
        let t = WeakKonigTree::complete(3);
        assert!(t.is_infinite());
        assert_eq!(t.count_at_depth(3), 8);
        let path = t.greedy_path();
        assert_eq!(path.len(), 3);
        assert_eq!(path, vec![0u8, 0u8, 0u8]);
    }
    #[test]
    fn test_weak_konig_tree_with_only_one_branch() {
        let mut t = WeakKonigTree {
            max_depth: 3,
            nodes: vec![],
            depths: vec![],
        };
        t.nodes.push(0);
        t.depths.push(0);
        t.nodes.push(1);
        t.depths.push(1);
        t.nodes.push(3);
        t.depths.push(2);
        t.nodes.push(7);
        t.depths.push(3);
        assert!(t.is_infinite());
        let path = t.greedy_path();
        assert_eq!(path, vec![1u8, 1u8, 1u8]);
    }
    #[test]
    fn test_ramsey_coloring_finder_uniform() {
        let mut finder = RamseyColoringFinder::new_uniform(5, 2);
        let (c, clique) = finder.best_monochromatic_clique();
        assert_eq!(c, 0);
        assert_eq!(clique.len(), 5);
        finder.set_color(0, 1, 1);
        finder.set_color(0, 2, 1);
        finder.set_color(1, 2, 1);
        let (c2, clique2) = finder.best_monochromatic_clique();
        assert!(clique2.len() >= 2);
        assert!(c2 < 2);
    }
    #[test]
    fn test_rma0_system() {
        let inst = RMA0System::sigma01_comprehension_for("phi_halting");
        assert!(inst.verify());
        assert_eq!(inst.kind, RCA0AxiomKind::Sigma01Comprehension);
        let summary = inst.summary();
        assert!(summary.contains("VALID"));
        assert!(summary.contains("phi_halting"));
    }
    #[test]
    fn test_rm_hierarchy_display() {
        let h = RMHierarchy::standard();
        let out = h.display();
        assert!(out.contains("RCA₀"));
        assert!(out.contains("WKL₀"));
        assert!(out.contains("ACA₀"));
        assert!(out.contains("ATR₀"));
        assert!(out.contains("Π¹₁-CA₀"));
        assert!(out.contains("König"));
        assert!(out.contains("Bolzano"));
    }
    #[test]
    fn test_rm_hierarchy_entry_lookup() {
        let h = RMHierarchy::standard();
        let entry = h
            .entry_for(&BigFiveSystem::ATR0)
            .expect("entry_for should succeed");
        assert_eq!(entry.system, BigFiveSystem::ATR0);
        assert!(entry.equivalents.contains(&"Open determinacy"));
    }
}
/// Standard library of RM theorems from classical analysis.
#[allow(dead_code)]
pub fn standard_rm_theorems() -> Vec<RMTheorem> {
    vec![
        RMTheorem::new(
            "BolzanoWeierstrass",
            "Every bounded sequence in ℝ has a convergent subsequence.",
            "ACA₀",
            vec![
                "Sequential compactness of [0,1]",
                "Monotone convergence theorem",
            ],
            false,
        ),
        RMTheorem::new(
            "HeineCantorUniformContinuity",
            "Every continuous function on a closed bounded interval is uniformly continuous.",
            "WKL₀",
            vec!["Heine-Borel theorem", "Fan theorem"],
            false,
        ),
        RMTheorem::new(
            "IntermediateValueTheorem",
            "If f: [0,1] → ℝ is continuous and f(0) < 0 < f(1), then ∃x, f(x) = 0.",
            "WKL₀",
            vec!["Brouwer's fixed point theorem (1D)"],
            false,
        ),
        RMTheorem::new(
            "MaximumMinimumTheorem",
            "Every continuous real function on [0,1] attains its maximum and minimum.",
            "WKL₀",
            vec!["Compact implies sequentially compact (metric)"],
            false,
        ),
        RMTheorem::new(
            "MonotoneConvergence",
            "Every bounded monotone sequence of reals converges.",
            "ACA₀",
            vec!["Bolzano-Weierstrass", "Sequential compactness"],
            false,
        ),
        RMTheorem::new(
            "CauchyCharacterization",
            "A sequence is convergent iff it is a Cauchy sequence.",
            "ACA₀",
            vec!["Completeness of ℝ"],
            false,
        ),
        RMTheorem::new(
            "KonigLemma",
            "Every infinite, finitely-branching tree has an infinite path.",
            "WKL₀",
            vec!["Heine-Borel theorem", "Weak König's Lemma"],
            false,
        ),
        RMTheorem::new(
            "RamseyFinite",
            "For all k, m, every k-coloring of [N]^2 has a monochromatic set of size m.",
            "RCA₀",
            Vec::<&str>::new(),
            true,
        ),
        RMTheorem::new(
            "RamseyInfinite",
            "For all k, every k-coloring of [N]^2 has an infinite monochromatic set.",
            "ACA₀",
            vec!["Ascending/Descending Sequence principle"],
            false,
        ),
        RMTheorem::new(
            "HilbertBasisTheorem",
            "Every ideal in a polynomial ring over a field is finitely generated.",
            "ATR₀",
            vec!["Open determinacy", "Comparability of well-orderings"],
            false,
        ),
        RMTheorem::new(
            "SilverTheorem",
            "Every Borel graph on a Polish space satisfies the Galvin-Prikry property.",
            "Π¹₁-CA₀",
            vec!["Analytic determinacy (weaker form)"],
            false,
        ),
        RMTheorem::new(
            "LindelofTheorem",
            "Every open cover of a separable metric space has a countable subcover.",
            "RCA₀",
            Vec::<&str>::new(),
            true,
        ),
    ]
}
/// Known Π¹₁ sentences and their proof-theoretic strengths.
#[allow(dead_code)]
pub fn known_pi11_sentences() -> Vec<Pi11Sentence> {
    vec![
        Pi11Sentence::new("Con(PA)", "Consistency of Peano Arithmetic", Some("ε₀")),
        Pi11Sentence::new("WO(ε₀)", "ε₀ is a well-ordering", Some("ε₀")),
        Pi11Sentence::new("Con(ATR₀)", "Consistency of ATR₀", Some("Γ₀")),
        Pi11Sentence::new("GoodmanTheorem", "The Goodman theorem for HA", Some("ε₀")),
        Pi11Sentence::new("Con(Z₂)", "Consistency of Z₂", Some("φ_ω(0)")),
        Pi11Sentence::new(
            "ParisHarrington",
            "The Paris-Harrington principle",
            Some("ε₀"),
        ),
        Pi11Sentence::new(
            "GoodsteinSeq",
            "Every Goodstein sequence terminates",
            Some("ε₀"),
        ),
        Pi11Sentence::new("KirbyParis", "Kirby-Paris Hydra theorem", Some("ε₀")),
    ]
}
/// Checks whether `system_a` is (weakly) contained in `system_b` by subsystem strength.
#[allow(dead_code)]
pub fn subsystem_le(system_a: &str, system_b: &str) -> bool {
    let order = ["RCA₀", "WKL₀", "ACA₀", "ATR₀", "Π¹₁-CA₀"];
    let pos_a = order.iter().position(|&s| s == system_a);
    let pos_b = order.iter().position(|&s| s == system_b);
    match (pos_a, pos_b) {
        (Some(a), Some(b)) => a <= b,
        _ => false,
    }
}
/// Known standard ω-models.
#[allow(dead_code)]
pub fn standard_omega_models() -> Vec<OmegaModel> {
    vec![
        OmegaModel::rec_sets(),
        OmegaModel::standard_aca0(),
        OmegaModel::new("HYP", "ATR₀", false, "Hyperarithmetical sets: Δ¹₁ sets"),
        OmegaModel::new("∆¹₂", "Π¹₁-CA₀", false, "Second-order definable sets: ∆¹₂"),
        OmegaModel::new(
            "L_ω₁ᶜᵏ",
            "Π¹₁-CA₀",
            false,
            "Gödel's L up to the Church-Kleene ordinal",
        ),
    ]
}
#[cfg(test)]
mod rm_extended_tests {
    use super::*;
    #[test]
    fn test_proof_system_name() {
        assert_eq!(ProofSystem::PRA.name(), "PRA");
        assert_eq!(ProofSystem::Z2.name(), "Z₂");
        assert_eq!(ProofSystem::Custom("ZFC".into()).name(), "ZFC");
    }
    #[test]
    fn test_is_conservative_over() {
        assert!(ProofSystem::Z2.is_conservative_over(&ProofSystem::PeanoPA));
        assert!(ProofSystem::PeanoPA.is_conservative_over(&ProofSystem::RobinsonQ));
        assert!(!ProofSystem::PRA.is_conservative_over(&ProofSystem::Z2));
    }
    #[test]
    fn test_stronger_systems() {
        let s = ProofSystem::PRA.stronger_systems();
        assert_eq!(s.len(), 5);
        assert!(s.contains(&ProofSystem::Z2));
    }
    #[test]
    fn test_rm_theorem_summary() {
        let thm = RMTheorem::new("Test", "Some theorem", "ACA₀", vec!["Equiv1"], false);
        let s = thm.summary();
        assert!(s.contains("Test"));
        assert!(s.contains("ACA₀"));
        assert!(s.contains("Equiv1"));
    }
    #[test]
    fn test_standard_rm_theorems() {
        let thms = standard_rm_theorems();
        assert!(!thms.is_empty());
        assert!(thms.iter().any(|t| t.name == "BolzanoWeierstrass"));
        assert!(thms.iter().any(|t| t.name == "KonigLemma"));
    }
    #[test]
    fn test_subsystem_le() {
        assert!(subsystem_le("RCA₀", "ACA₀"));
        assert!(subsystem_le("ACA₀", "ACA₀"));
        assert!(!subsystem_le("ACA₀", "WKL₀"));
        assert!(subsystem_le("WKL₀", "Π¹₁-CA₀"));
    }
    #[test]
    fn test_pi11_sentences() {
        let sents = known_pi11_sentences();
        assert!(!sents.is_empty());
        assert!(sents.iter().any(|s| s.name == "Con(PA)"));
        let disp = sents[0].display();
        assert!(disp.contains("Con(PA)"));
    }
    #[test]
    fn test_omega_models() {
        let rec = OmegaModel::rec_sets();
        assert!(rec.is_minimal);
        assert_eq!(rec.satisfies, "RCA₀");
        let models = standard_omega_models();
        assert_eq!(models.len(), 5);
        assert!(models.iter().any(|m| m.name == "REC"));
    }
    #[test]
    fn test_rm_scoreboard() {
        let sb = RMScoreboard::standard();
        assert!(!sb.theorems.is_empty());
        let aca0_count = sb.count_in("ACA₀");
        assert!(aca0_count > 0);
    }
}
/// Library of constructive principles.
#[allow(dead_code)]
pub fn constructive_principles() -> Vec<ConstructivePrinciple> {
    vec![
        ConstructivePrinciple::new("MarkovPrinciple", "LLPO", false, Some("Markov's principle")),
        ConstructivePrinciple::new("LLPO", "Law of Excluded Middle", false, Some("LPO")),
        ConstructivePrinciple::new("LPO", "Law of Excluded Middle", false, Some("LEM")),
        ConstructivePrinciple::new("WLPO", "Weak LPO", false, Some("LPO")),
        ConstructivePrinciple::new("BishopFan", "Fan Theorem", true, None::<&str>),
        ConstructivePrinciple::new(
            "ChoiceSequences",
            "Axiom of Choice (Countable)",
            false,
            Some("Countable choice"),
        ),
        ConstructivePrinciple::new("DependentChoice", "Dependent Choice", false, Some("DC")),
        ConstructivePrinciple::new(
            "UniformContinuity",
            "Uniform Continuity on Cantor Space",
            true,
            None::<&str>,
        ),
        ConstructivePrinciple::new(
            "ContBar",
            "Continuous Functions on Baire Space",
            false,
            Some("Bar Induction"),
        ),
    ]
}
/// Standard independence results.
#[allow(dead_code)]
pub fn standard_independence_results() -> Vec<IndependenceResult> {
    vec![
        IndependenceResult::new("ContinuumHypothesis", "ZFC", true, "Cohen 1963"),
        IndependenceResult::new("GeneralizedContinuumHypothesis", "ZFC", true, "Cohen 1963"),
        IndependenceResult::new("AxiomOfChoice", "ZF", true, "Cohen 1963"),
        IndependenceResult::new("SuslinHypothesis", "ZFC", true, "Solovay-Tennenbaum 1971"),
        IndependenceResult::new("BorelConjecture", "ZFC", true, "Laver 1976"),
        IndependenceResult::new("MartinAxiom", "ZFC", true, "Independent from ZFC+¬CH"),
        IndependenceResult::new("WhiteheadProblem", "ZFC", true, "Shelah 1974"),
        IndependenceResult::new("KaplanProblem", "ZFC", true, "Independent from ZFC"),
        IndependenceResult::new("ProperForcingAxiom", "ZFC", true, "Baumgartner 1984"),
        IndependenceResult::new("VopenkaPrinciple", "ZFC", true, "Requires a large cardinal"),
        IndependenceResult::new("Goodstein", "PA", true, "Kirby-Paris 1982"),
        IndependenceResult::new("ParisHarrington", "PA", true, "Paris-Harrington 1977"),
        IndependenceResult::new("Consistency(PA)", "PA", true, "Gödel 1931"),
    ]
}
#[cfg(test)]
mod rm_extended2_tests {
    use super::*;
    #[test]
    fn test_constructive_principles() {
        let ps = constructive_principles();
        assert!(!ps.is_empty());
        let mp = ps
            .iter()
            .find(|p| p.name == "MarkovPrinciple")
            .expect("find should succeed");
        assert!(!mp.constructively_provable);
        assert!(mp.required_axiom.is_some());
    }
    #[test]
    fn test_independence_results() {
        let irs = standard_independence_results();
        assert!(!irs.is_empty());
        let ch = irs
            .iter()
            .find(|r| r.statement == "ContinuumHypothesis")
            .expect("find should succeed");
        assert!(ch.is_independent);
        assert_eq!(ch.base_theory, "ZFC");
        let disp = ch.display();
        assert!(disp.contains("INDEPENDENT"));
    }
    #[test]
    fn test_big_five_stats() {
        let sb = RMScoreboard::standard();
        let stats = BigFiveStats::from_scoreboard(&sb);
        assert!(stats.total() > 0);
        let disp = stats.display();
        assert!(disp.contains("RCA₀"));
        assert!(disp.contains("ACA₀"));
    }
    #[test]
    fn test_constructive_principle_display() {
        let p = ConstructivePrinciple::new("TestPrinciple", "LEM", false, Some("MP"));
        let d = p.display();
        assert!(d.contains("TestPrinciple"));
        assert!(d.contains("constructive=false"));
    }
}
