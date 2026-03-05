//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BirkhoffAverage, ErgodicComponent, ErgodicDecomposition, ErgodicGroupAction,
    ErgodicTransformation, FurstenbergCorrespondenceV2, HaarMeasure, InvariantMeasure,
    KolmogorovSinaiEntropyV2, LyapunovSpectrum, MeasurablePartition, MeasurePreservingSystem,
    MetricEntropy, MixingCoefficient, PoincareRecurrence, SubShift, SymbolicSystem,
    TopologicalEntropy, TopologicalEntropyV2, UnipotentFlow,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// MeasurePreservingSystem type: (X : Type) → Measure X → (X → X) → Type
pub fn measure_preserving_system_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("Measure"), cst("X")),
            arrow(arrow(cst("X"), cst("X")), type0()),
        ),
    )
}
/// InvariantMeasure type: MeasurePreservingSystem → Prop
pub fn invariant_measure_ty() -> Expr {
    arrow(cst("MeasurePreservingSystem"), prop())
}
/// ErgodicTransformation type: (X : Type) → (X → X) → Prop
pub fn ergodic_transformation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(arrow(cst("X"), cst("X")), prop()),
    )
}
/// MixingTransformation type: (X : Type) → (X → X) → Prop
pub fn mixing_transformation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(arrow(cst("X"), cst("X")), prop()),
    )
}
/// BirkhoffAverage type: (f : X → Real) → Nat → Real
pub fn birkhoff_average_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(arrow(cst("X"), real_ty()), arrow(nat_ty(), real_ty())),
    )
}
/// BirkhoffErgodicTheorem (Prop): for μ-a.e. x, time average equals space average
pub fn birkhoff_ergodic_theorem_ty() -> Expr {
    prop()
}
/// MetricEntropy type: MeasurePreservingSystem → Real
pub fn metric_entropy_ty() -> Expr {
    arrow(cst("MeasurePreservingSystem"), real_ty())
}
/// TopologicalEntropy type: (X → X) → Real
pub fn topological_entropy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(arrow(cst("X"), cst("X")), real_ty()),
    )
}
/// VariationalPrinciple (Prop): h_top(T) = sup { h_μ(T) | μ T-invariant }
pub fn variational_principle_ty() -> Expr {
    prop()
}
/// ErgodicDecomposition (Prop): every invariant measure decomposes into ergodic components
pub fn ergodic_decomposition_ty() -> Expr {
    prop()
}
/// PoincareRecurrence (Prop): μ(A) > 0 → a.e. x ∈ A returns to A under iteration
pub fn poincare_recurrence_ty() -> Expr {
    arrow(prop(), prop())
}
/// VonNeumannMeanErgodic (Prop): Cesàro averages converge in L²
pub fn von_neumann_mean_ergodic_ty() -> Expr {
    prop()
}
/// MixingCoefficient type: Nat → Real  (alpha/beta/phi/rho/psi mixing)
pub fn mixing_coefficient_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// SubShift type: AlphabetSize → ForbiddenWords → Type
pub fn subshift_ty() -> Expr {
    arrow(nat_ty(), arrow(app(cst("List"), cst("String")), type0()))
}
/// ShiftEntropy type: SubShift → Real
pub fn shift_entropy_ty() -> Expr {
    arrow(cst("SubShift"), real_ty())
}
/// Register all ergodic-theory axioms into the given kernel environment.
pub fn build_ergodic_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("MeasurePreservingSystem", measure_preserving_system_ty()),
        ("InvariantMeasure", invariant_measure_ty()),
        ("ErgodicTransformation", ergodic_transformation_ty()),
        ("MixingTransformation", mixing_transformation_ty()),
        ("BirkhoffAverage", birkhoff_average_ty()),
        ("BirkhoffErgodicTheorem", birkhoff_ergodic_theorem_ty()),
        ("MetricEntropy", metric_entropy_ty()),
        ("TopologicalEntropy", topological_entropy_ty()),
        ("VariationalPrinciple", variational_principle_ty()),
        ("ErgodicDecomposition", ergodic_decomposition_ty()),
        ("PoincareRecurrence", poincare_recurrence_ty()),
        ("VonNeumannMeanErgodic", von_neumann_mean_ergodic_ty()),
        ("MixingCoefficient", mixing_coefficient_ty()),
        ("SubShift", subshift_ty()),
        ("ShiftEntropy", shift_entropy_ty()),
        ("IsErgodic", arrow(cst("MeasurePreservingSystem"), prop())),
        (
            "IsWeakMixing",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "IsStrongMixing",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        ("IsBernoulli", arrow(cst("MeasurePreservingSystem"), prop())),
        (
            "IsomorphicSystems",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(cst("MeasurePreservingSystem"), prop()),
            ),
        ),
        (
            "KSPartition",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "GeneratingPartition",
            arrow(
                cst("KSPartition"),
                arrow(cst("MeasurePreservingSystem"), prop()),
            ),
        ),
        (
            "SpectralMeasure",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "KoopmansOperator",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(
                    arrow(cst("X_bvar"), real_ty()),
                    arrow(cst("X_bvar"), real_ty()),
                ),
            ),
        ),
        (
            "PoincareReturnTime",
            arrow(cst("MeasurePreservingSystem"), arrow(type0(), nat_ty())),
        ),
        (
            "KacLemma",
            arrow(cst("MeasurePreservingSystem"), arrow(type0(), prop())),
        ),
        ("IsKSystem", arrow(cst("MeasurePreservingSystem"), prop())),
        (
            "IsBernoulliShift",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "WeakMixingProperty",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "StrongMixingProperty",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "MixingImplication",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "MaximalSpectralType",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "HasLebesgueSpectrum",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "SpectralMultiplicity",
            arrow(cst("MeasurePreservingSystem"), nat_ty()),
        ),
        (
            "EigenvalueGroup",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "KolmogorovSinaiEntropyAxiom",
            arrow(cst("MeasurePreservingSystem"), real_ty()),
        ),
        (
            "GeneratorTheorem",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "PinskerFactor",
            arrow(
                cst("MeasurePreservingSystem"),
                cst("MeasurePreservingSystem"),
            ),
        ),
        (
            "OrthogonalComplement",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "AbramovFormula",
            arrow(cst("MeasurePreservingSystem"), arrow(nat_ty(), prop())),
        ),
        (
            "LyapunovExponent",
            arrow(cst("MeasurePreservingSystem"), real_ty()),
        ),
        (
            "OseledetsTheorem",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "PesinFormula",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        ("PesinTheory", arrow(cst("MeasurePreservingSystem"), prop())),
        (
            "RuelleInequality",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        ("SoficShift", arrow(nat_ty(), prop())),
        ("ShiftOfFiniteType", arrow(nat_ty(), prop())),
        ("MixingShift", arrow(nat_ty(), prop())),
        ("TopologicalEntropyShift", arrow(nat_ty(), real_ty())),
        (
            "ErgodicComponent",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "ExtremalInvariantMeasure",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "SimplexOfInvariantMeasures",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "Joining",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(cst("MeasurePreservingSystem"), type0()),
            ),
        ),
        (
            "SelfJoining",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "DisjointSystems",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(cst("MeasurePreservingSystem"), prop()),
            ),
        ),
        (
            "MinimalSelfJoining",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "SimplicityCriterion",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "MultipleRecurrence",
            arrow(cst("MeasurePreservingSystem"), arrow(nat_ty(), prop())),
        ),
        (
            "FurstenbergRecurrence",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        ("SzemereditheoremnAxiom", prop()),
        ("FurstenbergCorrespondenceAxiom", arrow(prop(), prop())),
        ("UpperBanachDensity", arrow(type0(), real_ty())),
        (
            "IsMinimalSystem",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "IsUniquelyErgodic",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        ("WeylEquidistribution", arrow(real_ty(), prop())),
        (
            "EquidistributionSequence",
            arrow(arrow(nat_ty(), real_ty()), prop()),
        ),
        (
            "ZimmerCocycle",
            arrow(cst("MeasurePreservingSystem"), type0()),
        ),
        (
            "CocycleSuperrigidity",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
        (
            "OrbitEquivalent",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(cst("MeasurePreservingSystem"), prop()),
            ),
        ),
        (
            "StrongOrbitEquivalence",
            arrow(
                cst("MeasurePreservingSystem"),
                arrow(cst("MeasurePreservingSystem"), prop()),
            ),
        ),
        ("DyeTheorem", prop()),
        (
            "AmenableGroupAction",
            arrow(cst("MeasurePreservingSystem"), prop()),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
/// Return the formal statement of the Birkhoff ergodic theorem as a string.
pub fn birkhoff_theorem_statement() -> &'static str {
    "Birkhoff Ergodic Theorem: Let (X, μ, T) be a measure-preserving system and \
     f ∈ L¹(μ). Then for μ-almost every x ∈ X, the time average \
     (1/n) Σ_{k=0}^{n-1} f(T^k x) converges as n → ∞ to a T-invariant function f* ∈ L¹(μ). \
     Moreover, ∫ f* dμ = ∫ f dμ. If (X, μ, T) is ergodic then f* = ∫ f dμ  μ-a.e."
}
/// Return the formal statement of the variational principle.
pub fn variational_principle_statement() -> &'static str {
    "Variational Principle: For a continuous map T on a compact metric space X, \
     the topological entropy satisfies h_top(T) = sup { h_μ(T) | μ is a T-invariant \
     Borel probability measure }. The supremum is attained by a measure of maximal entropy."
}
/// Return the formal statement of the Poincaré recurrence theorem.
pub fn poincare_recurrence_theorem() -> &'static str {
    "Poincaré Recurrence Theorem: Let (X, μ, T) be a measure-preserving system with \
     μ(X) < ∞. For every measurable set A with μ(A) > 0, for μ-almost every x ∈ A \
     there exist infinitely many n ≥ 1 such that T^n(x) ∈ A."
}
/// Return the formal statement of the von Neumann mean ergodic theorem.
pub fn von_neumann_ergodic_theorem() -> &'static str {
    "von Neumann Mean Ergodic Theorem: Let U be a unitary operator on a Hilbert space H. \
     Then for every f ∈ H, the Cesàro averages A_n f = (1/n) Σ_{k=0}^{n-1} U^k f \
     converge in norm as n → ∞ to the orthogonal projection P f of f onto the closed \
     subspace of U-invariant vectors. In the measure-preserving setting U = T^*, \
     P projects onto L²(X, μ)^T."
}
/// Convenience alias to mirror the spec name `build_env`.
pub fn build_env(env: &mut Environment) {
    build_ergodic_theory_env(env);
}
/// Key theorems in ergodic theory.
#[allow(dead_code)]
pub fn ergodic_theory_key_theorems() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Birkhoff ergodic theorem",
            "Time averages = space averages a.e. for ergodic systems",
        ),
        (
            "von Neumann ergodic theorem",
            "L^2 convergence of Cesaro averages",
        ),
        (
            "Halmos recurrence",
            "Measure-preserving maps return to every positive-measure set",
        ),
        (
            "Rohlin lemma",
            "Aperiodic systems have towers of any height",
        ),
        (
            "Furstenberg multiple recurrence",
            "Ergodic <-> Szemeredi theorem",
        ),
        (
            "Ornstein isomorphism",
            "Bernoulli shifts classified by entropy",
        ),
        ("Oseledets theorem", "Lyapunov exponents exist a.e."),
        (
            "Variational principle",
            "Topological entropy = sup of measure-theoretic entropies",
        ),
        (
            "Sinai factor theorem",
            "Bernoulli shift is factor of positive-entropy system",
        ),
        (
            "Ratner's theorem",
            "Unipotent flows on homogeneous spaces equidistribute",
        ),
    ]
}
#[cfg(test)]
mod ergodic_ext_tests {
    use super::*;
    #[test]
    fn test_topological_entropy() {
        let te = TopologicalEntropyV2::shift_map(2);
        assert!((te.value - std::f64::consts::LN_2).abs() < 1e-10);
        assert!(te.is_finite());
    }
    #[test]
    fn test_lyapunov_spectrum() {
        let ls = LyapunovSpectrum::new("doubling map", vec![std::f64::consts::LN_2]);
        assert!((ls.max_exponent() - std::f64::consts::LN_2).abs() < 1e-10);
        assert!(ls.is_hyperbolic());
    }
    #[test]
    fn test_symbolic_system() {
        let full = SymbolicSystem::full_shift(2);
        assert!(full.is_subshift_of_finite_type());
        assert!(full.is_sofic());
    }
    #[test]
    fn test_furstenberg() {
        let fc = FurstenbergCorrespondenceV2::szemeredi();
        assert!(!fc.principle_statement().is_empty());
    }
    #[test]
    fn test_key_theorems_nonempty() {
        let thms = ergodic_theory_key_theorems();
        assert!(!thms.is_empty());
    }
}
#[cfg(test)]
mod haar_measure_tests {
    use super::*;
    #[test]
    fn test_haar_measure_su2() {
        let mu = HaarMeasure::compact_group("SU(2)");
        assert!(mu.is_unimodular);
        assert!(!mu.left_invariance().is_empty());
    }
    #[test]
    fn test_ergodic_action() {
        let ea = ErgodicGroupAction::new("Z", "[0,1]", false, true);
        assert!(!ea.orbit_equivalence_description().is_empty());
    }
}
#[cfg(test)]
mod ks_entropy_tests {
    use super::*;
    #[test]
    fn test_bernoulli_entropy() {
        let h = KolmogorovSinaiEntropyV2::bernoulli_shift_entropy(&[0.5, 0.5]);
        assert!((h - std::f64::consts::LN_2).abs() < 1e-10);
    }
    #[test]
    fn test_ks_entropy() {
        let ks = KolmogorovSinaiEntropyV2::new("doubling map", "binary", std::f64::consts::LN_2);
        assert!(!ks.generating_partition_description().is_empty());
    }
    #[test]
    fn test_measurable_partition() {
        let mp = MeasurablePartition::new("[0,1]", "binary", true);
        assert!(mp.is_generating);
    }
}
#[cfg(test)]
mod ratner_tests {
    use super::*;
    #[test]
    fn test_unipotent_flow() {
        let uf = UnipotentFlow::new("SL2(R)", "SL2(Z)", "[[1,t],[0,1]]");
        assert!(uf.is_equidistributed());
        assert!(!uf.ratner_theorem_statement().is_empty());
    }
}
