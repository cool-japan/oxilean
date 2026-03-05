//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DefuzzMethod, FiniteMTLAlgebra, FuzzyCMeans, FuzzyClustering, FuzzyInferenceSystem,
    FuzzyMetricSpace, FuzzyRoughApprox, FuzzySet, GradualElement, LinguisticHedgeApplier,
    MamdaniEngine, ManyValuedLogic, TConorm, TNorm, TNormComputer, TriangularFuzzyNum,
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
/// De Morgan dual: from t-norm get t-conorm via S(a,b) = 1 − T(1−a, 1−b).
pub fn de_morgan_dual_conorm(t: TNorm, a: f64, b: f64) -> f64 {
    1.0 - t.eval(1.0 - a, 1.0 - b)
}
/// Triangular membership function: zero outside [a, c], peak 1 at b.
pub fn triangular_mf(x: f64, a: f64, b: f64, c: f64) -> f64 {
    if x <= a || x >= c {
        0.0
    } else if x <= b {
        (x - a) / (b - a)
    } else {
        (c - x) / (c - b)
    }
}
/// Trapezoidal membership function: 1 on [b, c], ramps on [a,b] and [c,d].
pub fn trapezoidal_mf(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    if x <= a || x >= d {
        0.0
    } else if x <= b {
        (x - a) / (b - a)
    } else if x <= c {
        1.0
    } else {
        (d - x) / (d - c)
    }
}
/// Gaussian membership function: exp(−(x − c)² / (2σ²)).
pub fn gaussian_mf(x: f64, center: f64, sigma: f64) -> f64 {
    let z = (x - center) / sigma;
    (-0.5 * z * z).exp()
}
/// Sigmoid membership function: 1 / (1 + exp(−a(x − c))).
pub fn sigmoid_mf(x: f64, a: f64, c: f64) -> f64 {
    1.0 / (1.0 + (-a * (x - c)).exp())
}
/// Bell-shaped (generalized bell) membership function.
pub fn bell_mf(x: f64, a: f64, b: f64, c: f64) -> f64 {
    1.0 / (1.0 + ((x - c) / a).abs().powf(2.0 * b))
}
/// Fuzzy intersection using minimum t-norm.
pub fn fuzzy_intersection(a: &FuzzySet, b: &FuzzySet) -> FuzzySet {
    assert_eq!(a.universe_size, b.universe_size);
    let membership = a
        .membership
        .iter()
        .zip(b.membership.iter())
        .map(|(&x, &y)| x.min(y))
        .collect();
    FuzzySet {
        universe_size: a.universe_size,
        membership,
    }
}
/// Fuzzy union using maximum t-conorm.
pub fn fuzzy_union(a: &FuzzySet, b: &FuzzySet) -> FuzzySet {
    assert_eq!(a.universe_size, b.universe_size);
    let membership = a
        .membership
        .iter()
        .zip(b.membership.iter())
        .map(|(&x, &y)| x.max(y))
        .collect();
    FuzzySet {
        universe_size: a.universe_size,
        membership,
    }
}
/// Fuzzy difference: A \ B = A ∩ ¬B.
pub fn fuzzy_difference(a: &FuzzySet, b: &FuzzySet) -> FuzzySet {
    fuzzy_intersection(a, &b.complement())
}
/// Cartesian product of two fuzzy sets using minimum.
pub fn fuzzy_cartesian_product(a: &FuzzySet, b: &FuzzySet) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; b.universe_size]; a.universe_size];
    for i in 0..a.universe_size {
        for j in 0..b.universe_size {
            result[i][j] = a.membership[i].min(b.membership[j]);
        }
    }
    result
}
/// Defuzzify a fuzzy set given the universe points (domain values).
pub fn defuzzify(fuzzy: &FuzzySet, domain: &[f64], method: DefuzzMethod) -> f64 {
    assert_eq!(fuzzy.universe_size, domain.len());
    match method {
        DefuzzMethod::CentroidOfArea => {
            let num: f64 = fuzzy
                .membership
                .iter()
                .zip(domain.iter())
                .map(|(&mu, &x)| mu * x)
                .sum();
            let den: f64 = fuzzy.membership.iter().sum();
            if den.abs() < 1e-12 {
                0.0
            } else {
                num / den
            }
        }
        DefuzzMethod::MeanOfMaxima => {
            let max_val = fuzzy.height();
            let maxima: Vec<f64> = fuzzy
                .membership
                .iter()
                .zip(domain.iter())
                .filter(|(&mu, _)| (mu - max_val).abs() < 1e-9)
                .map(|(_, &x)| x)
                .collect();
            if maxima.is_empty() {
                0.0
            } else {
                maxima.iter().sum::<f64>() / maxima.len() as f64
            }
        }
        DefuzzMethod::SmallestOfMaxima => {
            let max_val = fuzzy.height();
            fuzzy
                .membership
                .iter()
                .zip(domain.iter())
                .filter(|(&mu, _)| (mu - max_val).abs() < 1e-9)
                .map(|(_, &x)| x)
                .next()
                .unwrap_or(0.0)
        }
        DefuzzMethod::LargestOfMaxima => {
            let max_val = fuzzy.height();
            fuzzy
                .membership
                .iter()
                .zip(domain.iter())
                .filter(|(&mu, _)| (mu - max_val).abs() < 1e-9)
                .map(|(_, &x)| x)
                .next_back()
                .unwrap_or(0.0)
        }
        DefuzzMethod::BisectorOfArea => {
            let total: f64 = fuzzy.membership.iter().sum();
            let half = total / 2.0;
            let mut running = 0.0;
            for (i, &mu) in fuzzy.membership.iter().enumerate() {
                running += mu;
                if running >= half {
                    return domain[i];
                }
            }
            *domain.last().unwrap_or(&0.0)
        }
    }
}
/// FuzzySet type: FuzzySet A = A → [0,1] (represented as A → Real).
pub fn fuzzy_set_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), cst("Real")))
}
/// Membership function type: MembershipFn A = A → [0,1].
pub fn membership_fn_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), cst("Real")))
}
/// T-norm type: TNorm = Real → Real → Real satisfying T-norm axioms.
pub fn tnorm_ty() -> Expr {
    arrow(cst("Real"), arrow(cst("Real"), cst("Real")))
}
/// T-conorm type: TConorm = Real → Real → Real.
pub fn tconorm_ty() -> Expr {
    arrow(cst("Real"), arrow(cst("Real"), cst("Real")))
}
/// Negation function type: Negation = Real → Real (N(0)=1, N(1)=0).
pub fn fuzzy_negation_ty() -> Expr {
    arrow(cst("Real"), cst("Real"))
}
/// FuzzyRelation type: FuzzyRelation A = A → A → Real.
pub fn fuzzy_relation_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), cst("Real"))))
}
/// Residuated lattice type for algebraic semantics.
pub fn residuated_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("PartialOrder"), bvar(0)),
            arrow(
                tnorm_ty(),
                arrow(arrow(cst("Real"), arrow(cst("Real"), cst("Real"))), prop()),
            ),
        ),
    )
}
/// MTL algebra type.
pub fn mtl_algebra_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("ResiduatedLattice"), bvar(0)), prop()),
    )
}
/// BL algebra type (MTL + divisibility).
pub fn bl_algebra_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("MTLAlgebra"), bvar(0)), prop()))
}
/// MV algebra type (MTL + double negation).
pub fn mv_algebra_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("BLAlgebra"), bvar(0)), prop()))
}
/// Fuzzy metric space type (George-Veeramani).
pub fn fuzzy_metric_space_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), arrow(cst("Real"), cst("Real")))),
            arrow(cst("TNorm"), prop()),
        ),
    )
}
/// Fuzzy topological space type (Chang-style).
pub fn fuzzy_topological_space_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        arrow(app(cst("Set"), app(cst("FuzzySet"), bvar(0))), prop()),
    )
}
/// Completeness of Łukasiewicz logic: a formula is valid iff it is a tautology
/// in all MV-algebras.
pub fn lukasiewicz_completeness_ty() -> Expr {
    arrow(
        app(cst("LukasiewiczFormula"), cst("phi")),
        arrow(
            app(cst("MVAlgebraValid"), cst("phi")),
            app(cst("LukasiewiczProvable"), cst("phi")),
        ),
    )
}
/// Standard completeness theorem for MTL.
pub fn mtl_standard_completeness_ty() -> Expr {
    arrow(
        app(cst("MTLFormula"), cst("phi")),
        arrow(
            app(cst("MTLValid"), cst("phi")),
            app(cst("MTLProvable"), cst("phi")),
        ),
    )
}
/// Representation theorem for BL algebras: every BL algebra is a
/// subdirect product of BL-chains.
pub fn bl_representation_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("BLAlgebra"), bvar(0)),
            app2(cst("SubdirectProduct"), bvar(1), cst("BLChains")),
        ),
    )
}
/// Gödel logic has the finite model property.
pub fn godel_finite_model_property_ty() -> Expr {
    arrow(
        app(cst("GodelFormula"), cst("phi")),
        arrow(
            app2(cst("Satisfiable"), cst("phi"), cst("FiniteGodelAlgebra")),
            app(cst("GodelValid"), cst("phi")),
        ),
    )
}
/// Every fuzzy metric space (GV-style with continuous t-norm) induces a
/// Hausdorff topology.
pub fn fuzzy_metric_hausdorff_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        arrow(
            app2(cst("FuzzyMetricSpace"), bvar(0), cst("ProductTNorm")),
            app2(
                cst("HausdorffTopology"),
                bvar(1),
                cst("FuzzyMetricTopology"),
            ),
        ),
    )
}
/// Defuzzification theorem: the centroid defuzzifier is continuous in the
/// Hausdorff metric on compact fuzzy sets.
pub fn centroid_continuity_ty() -> Expr {
    arrow(
        app(cst("CompactFuzzySet"), cst("A")),
        app(cst("Continuous"), app(cst("Centroid"), cst("A"))),
    )
}
/// De Morgan duality between t-norms and t-conorms under standard negation.
pub fn de_morgan_duality_ty() -> Expr {
    impl_pi(
        "T",
        tnorm_ty(),
        arrow(
            app(cst("TNormAxioms"), bvar(0)),
            app(
                cst("TConormAxioms"),
                app2(cst("DeMorganDual"), bvar(1), cst("StandardNeg")),
            ),
        ),
    )
}
/// Alpha-cut decomposition: every fuzzy set is the union of its alpha-cuts.
pub fn alpha_cut_decomposition_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "mu",
            arrow(bvar(0), cst("Real")),
            app2(
                cst("FuzzySetEq"),
                bvar(0),
                app(cst("AlphaCutUnion"), bvar(1)),
            ),
        ),
    )
}
/// Build the fuzzy logic kernel environment with all declarations.
pub fn build_fuzzy_logic_env() -> Environment {
    let mut env = Environment::new();
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzySet"),
        univ_params: vec![],
        ty: fuzzy_set_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MembershipFn"),
        univ_params: vec![],
        ty: membership_fn_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("TNorm"),
        univ_params: vec![],
        ty: tnorm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("TConorm"),
        univ_params: vec![],
        ty: tconorm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyNegation"),
        univ_params: vec![],
        ty: fuzzy_negation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyRelation"),
        univ_params: vec![],
        ty: fuzzy_relation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ResiduatedLattice"),
        univ_params: vec![],
        ty: residuated_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MTLAlgebra"),
        univ_params: vec![],
        ty: mtl_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("BLAlgebra"),
        univ_params: vec![],
        ty: bl_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MVAlgebra"),
        univ_params: vec![],
        ty: mv_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyMetricSpace"),
        univ_params: vec![],
        ty: fuzzy_metric_space_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyTopologicalSpace"),
        univ_params: vec![],
        ty: fuzzy_topological_space_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LukasiewiczCompleteness"),
        univ_params: vec![],
        ty: lukasiewicz_completeness_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MTLStandardCompleteness"),
        univ_params: vec![],
        ty: mtl_standard_completeness_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("BLRepresentation"),
        univ_params: vec![],
        ty: bl_representation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GodelFiniteModelProperty"),
        univ_params: vec![],
        ty: godel_finite_model_property_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyMetricHausdorff"),
        univ_params: vec![],
        ty: fuzzy_metric_hausdorff_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CentroidContinuity"),
        univ_params: vec![],
        ty: centroid_continuity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DeMorganDuality"),
        univ_params: vec![],
        ty: de_morgan_duality_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("AlphaCutDecomposition"),
        univ_params: vec![],
        ty: alpha_cut_decomposition_ty(),
    });
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuzzy_set_basic() {
        let mut fs = FuzzySet::new(5);
        fs.set(0, 0.2);
        fs.set(1, 0.7);
        fs.set(2, 1.0);
        fs.set(3, 0.5);
        fs.set(4, 0.0);
        assert!((fs.height() - 1.0).abs() < 1e-9);
        assert!(fs.is_normal());
        assert_eq!(fs.core(), vec![2]);
        assert_eq!(fs.support(), vec![0, 1, 2, 3]);
        let cut = fs.alpha_cut(0.5);
        assert!(cut.contains(&1));
        assert!(cut.contains(&2));
        assert!(cut.contains(&3));
        assert!(!cut.contains(&0));
    }
    #[test]
    fn test_fuzzy_set_operations() {
        let mut a = FuzzySet::new(4);
        a.set(0, 0.3);
        a.set(1, 0.8);
        a.set(2, 1.0);
        a.set(3, 0.6);
        let mut b = FuzzySet::new(4);
        b.set(0, 0.9);
        b.set(1, 0.4);
        b.set(2, 0.5);
        b.set(3, 1.0);
        let inter = fuzzy_intersection(&a, &b);
        assert!((inter.get(0) - 0.3).abs() < 1e-9);
        assert!((inter.get(1) - 0.4).abs() < 1e-9);
        let union = fuzzy_union(&a, &b);
        assert!((union.get(0) - 0.9).abs() < 1e-9);
        assert!((union.get(1) - 0.8).abs() < 1e-9);
        let comp = a.complement();
        assert!((comp.get(2) - 0.0).abs() < 1e-9);
        assert!((comp.get(0) - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_tnorm_properties() {
        let t = TNorm::Minimum;
        assert!(t.is_commutative_sample(0.3, 0.7));
        assert!(t.is_associative_sample(0.2, 0.5, 0.8));
        let t_luk = TNorm::Lukasiewicz;
        assert!((t_luk.eval(0.4, 0.7) - 0.1).abs() < 1e-9);
        assert!((t_luk.eval(0.3, 0.3) - 0.0).abs() < 1e-9);
        let t_prod = TNorm::Product;
        assert!((t_prod.eval(0.5, 0.6) - 0.3).abs() < 1e-9);
    }
    #[test]
    fn test_membership_functions() {
        assert!((triangular_mf(2.0, 1.0, 2.0, 3.0) - 1.0).abs() < 1e-9);
        assert!((triangular_mf(1.5, 1.0, 2.0, 3.0) - 0.5).abs() < 1e-9);
        assert!((triangular_mf(0.5, 1.0, 2.0, 3.0)).abs() < 1e-9);
        assert!((gaussian_mf(0.0, 0.0, 1.0) - 1.0).abs() < 1e-9);
        let g = gaussian_mf(1.0, 0.0, 1.0);
        assert!((g - (-0.5_f64).exp()).abs() < 1e-9);
        assert!((trapezoidal_mf(2.5, 1.0, 2.0, 3.0, 4.0) - 1.0).abs() < 1e-9);
        assert!((trapezoidal_mf(1.5, 1.0, 2.0, 3.0, 4.0) - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_many_valued_logic() {
        let luk = ManyValuedLogic::Lukasiewicz;
        let a = 0.4_f64;
        assert!((luk.neg(luk.neg(a)) - a).abs() < 1e-9);
        assert!((luk.residuum(a, 1.0) - 1.0).abs() < 1e-9);
        let godel = ManyValuedLogic::Godel;
        assert!((godel.residuum(0.7, 0.7) - 1.0).abs() < 1e-9);
        assert!((godel.residuum(0.3, 0.9) - 1.0).abs() < 1e-9);
        let prod = ManyValuedLogic::Product;
        assert!((prod.conj(a, 1.0) - a).abs() < 1e-9);
    }
    #[test]
    fn test_defuzzification() {
        let mut fs = FuzzySet::new(5);
        fs.set(0, 0.0);
        fs.set(1, 0.5);
        fs.set(2, 1.0);
        fs.set(3, 0.5);
        fs.set(4, 0.0);
        let domain = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let coa = defuzzify(&fs, &domain, DefuzzMethod::CentroidOfArea);
        assert!((coa - 2.0).abs() < 1e-9);
        let mom = defuzzify(&fs, &domain, DefuzzMethod::MeanOfMaxima);
        assert!((mom - 2.0).abs() < 1e-9);
        let som = defuzzify(&fs, &domain, DefuzzMethod::SmallestOfMaxima);
        assert!((som - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_finite_mtl_algebra() {
        let tnorm = vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 1, 2]];
        let alg = FiniteMTLAlgebra::from_tnorm(3, tnorm);
        assert!(alg.satisfies_prelinearity());
        assert!(alg.satisfies_divisibility());
        assert_eq!(alg.neg(0), 2);
        assert_eq!(alg.neg(2), 0);
    }
    #[test]
    fn test_fuzzy_metric_space() {
        let t_grid = vec![0.5, 1.0, 2.0, f64::INFINITY];
        let mut fms = FuzzyMetricSpace::new(3, t_grid);
        for x in 0..3 {
            for t_idx in 0..4 {
                fms.set_metric(x, x, t_idx, 1.0);
            }
        }
        fms.set_metric(0, 1, 0, 0.2);
        fms.set_metric(0, 1, 1, 0.6);
        fms.set_metric(0, 1, 2, 0.9);
        fms.set_metric(0, 1, 3, 1.0);
        fms.set_metric(0, 2, 0, 0.1);
        fms.set_metric(0, 2, 1, 0.4);
        fms.set_metric(0, 2, 2, 0.7);
        fms.set_metric(0, 2, 3, 1.0);
        fms.set_metric(1, 2, 0, 0.3);
        fms.set_metric(1, 2, 1, 0.5);
        fms.set_metric(1, 2, 2, 0.8);
        fms.set_metric(1, 2, 3, 1.0);
        assert!(fms.check_diagonal_axiom());
        assert!(fms.check_limit_axiom());
        assert!(fms.check_non_separability());
    }
    #[test]
    fn test_build_fuzzy_logic_env() {
        let env = build_fuzzy_logic_env();
        assert!(env.get(&Name::str("FuzzySet")).is_some());
        assert!(env.get(&Name::str("TNorm")).is_some());
        assert!(env.get(&Name::str("MTLAlgebra")).is_some());
        assert!(env.get(&Name::str("BLAlgebra")).is_some());
        assert!(env.get(&Name::str("MVAlgebra")).is_some());
        assert!(env.get(&Name::str("FuzzyMetricSpace")).is_some());
        assert!(env.get(&Name::str("LukasiewiczCompleteness")).is_some());
        assert!(env.get(&Name::str("AlphaCutDecomposition")).is_some());
    }
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// Archimedean t-norm: T(a,a) < a for all a ∈ (0,1).
pub fn archimedean_tnorm_ty() -> Expr {
    arrow(tnorm_ty(), prop())
}
/// Nilpotent t-norm: continuous Archimedean t-norm with nilpotent elements.
pub fn nilpotent_tnorm_ty() -> Expr {
    arrow(tnorm_ty(), prop())
}
/// Strict t-norm: continuous Archimedean without zero divisors.
pub fn strict_tnorm_ty() -> Expr {
    arrow(tnorm_ty(), prop())
}
/// Frank t-norm family: F_s(a,b) indexed by s ∈ [0,∞].
pub fn frank_tnorm_ty() -> Expr {
    arrow(real_ty(), tnorm_ty())
}
/// Yager t-norm family: T_p(a,b) = max(0, 1 − ((1−a)^p + (1−b)^p)^{1/p}).
pub fn yager_tnorm_ty() -> Expr {
    arrow(real_ty(), tnorm_ty())
}
/// De Morgan triplet: (T, S, N) where S(a,b) = N(T(N(a), N(b))).
pub fn de_morgan_triplet_ty() -> Expr {
    app3(cst("Triple"), tnorm_ty(), tconorm_ty(), fuzzy_negation_ty())
}
/// Possibility measure: Π(A ∪ B) = max(Π(A), Π(B)).
pub fn possibility_measure_ty() -> Expr {
    arrow(arrow(prop(), real_ty()), prop())
}
/// Necessity measure: N(A) = 1 − Π(¬A).
pub fn necessity_measure_ty() -> Expr {
    arrow(arrow(prop(), real_ty()), prop())
}
/// Maxitivity: Π(A ∪ B) = max(Π(A), Π(B)) (characteristic of possibility measures).
pub fn maxitivity_ty() -> Expr {
    arrow(arrow(prop(), real_ty()), prop())
}
/// Possibility distribution: π: U → [0,1] with sup π(u) = 1.
pub fn possibility_distribution_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Qualitative possibility theory: comparative possibility ordering.
pub fn qualitative_possibility_ty() -> Expr {
    prop()
}
/// Lower approximation: POS(X) = {x : [x]_R ⊆ X}.
pub fn lower_approximation_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Upper approximation: NEG(X) = {x : [x]_R ∩ X ≠ ∅}.
pub fn upper_approximation_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Boundary region: BND(X) = UPP(X) \ LOW(X).
pub fn boundary_region_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Rough set reduct: minimal set of attributes preserving the discernibility.
pub fn rough_reduct_ty() -> Expr {
    arrow(list_ty(nat_ty()), prop())
}
/// Attribute dependency: γ(P, Q) = |POS_P(Q)| / |U|.
pub fn attribute_dependency_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), real_ty()))
}
/// Variable precision rough set: allows small classification error ε.
pub fn variable_precision_rough_ty() -> Expr {
    arrow(real_ty(), arrow(prop(), prop()))
}
/// Fuzzy number: normal convex fuzzy set on R with unimodal membership.
pub fn fuzzy_number_ty() -> Expr {
    prop()
}
/// LR fuzzy number: defined by left shape L and right shape R.
pub fn lr_fuzzy_number_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Extension principle (Zadeh): f̃(A) = {(y, sup_{f(x)=y} μ_A(x))}.
pub fn extension_principle_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(prop(), prop()))
}
/// Fuzzy interval arithmetic: addition of fuzzy numbers.
pub fn fuzzy_addition_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Fuzzy interval arithmetic: multiplication of fuzzy numbers.
pub fn fuzzy_multiplication_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Triangular fuzzy number: (a, b, c) with triangular membership.
pub fn triangular_fuzzy_number_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Fuzzy rule base: collection of IF-THEN rules for a fuzzy controller.
pub fn fuzzy_rule_base_ty() -> Expr {
    arrow(list_ty(prop()), prop())
}
/// Fuzzy inference engine: maps inputs through rule base to output fuzzy set.
pub fn fuzzy_inference_engine_ty() -> Expr {
    arrow(list_ty(real_ty()), prop())
}
/// Defuzzification strategy as a function from fuzzy set to crisp value.
pub fn defuzzification_strategy_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// Approximate reasoning: from fuzzy rule base and observation, conclude.
pub fn approximate_reasoning_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Fuzzy PID controller: proportional-integral-derivative with fuzzy gains.
pub fn fuzzy_pid_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Type-2 fuzzy set: membership function itself is a fuzzy set.
pub fn type2_fuzzy_set_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Interval type-2 fuzzy set: upper and lower membership functions.
pub fn interval_type2_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// Footprint of uncertainty (FOU): region between upper and lower MFs.
pub fn footprint_uncertainty_ty() -> Expr {
    arrow(prop(), prop())
}
/// Type reduction: maps type-2 fuzzy set to type-1 (Karnik-Mendel algorithm).
pub fn type_reduction_ty() -> Expr {
    arrow(prop(), prop())
}
/// Linguistic variable: (name, term set, universe, membership functions).
pub fn linguistic_variable_ty() -> Expr {
    prop()
}
/// Linguistic hedge: modifier like "very", "more or less", "somewhat".
pub fn linguistic_hedge_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Computing with words: processing linguistic information via fuzzy sets.
pub fn computing_with_words_ty() -> Expr {
    prop()
}
/// Linguistic approximation: find the linguistic term closest to a fuzzy set.
pub fn linguistic_approximation_ty() -> Expr {
    arrow(prop(), prop())
}
/// Fuzzy c-means (FCM): soft partitioning of data into c clusters.
pub fn fuzzy_cmeans_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// Possibilistic c-means (PCM): replaces probability constraint with typicality.
pub fn possibilistic_cmeans_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// Gustafson-Kessel (GK) algorithm: FCM with adaptive distance norm.
pub fn gustafson_kessel_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// Fuzzy silhouette: validity index for fuzzy partitions.
pub fn fuzzy_silhouette_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// Partition coefficient (Bezdek): V_PC = (1/N) Σ Σ u_{ik}^2.
pub fn partition_coefficient_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// Intuitionistic fuzzy set (Atanassov): membership μ, non-membership ν, μ+ν ≤ 1.
pub fn intuitionistic_fuzzy_set_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// Bipolar fuzzy set: positive and negative membership on [−1,1].
pub fn bipolar_fuzzy_set_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// Pythagorean fuzzy set: μ^2 + ν^2 ≤ 1 (more general than IFS).
pub fn pythagorean_fuzzy_set_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// Hesitant fuzzy set: membership is a set of possible degrees.
pub fn hesitant_fuzzy_set_ty() -> Expr {
    arrow(arrow(real_ty(), list_ty(real_ty())), prop())
}
/// q-rung orthopair fuzzy set: μ^q + ν^q ≤ 1.
pub fn q_rung_orthopair_ty() -> Expr {
    arrow(nat_ty(), arrow(prop(), prop()))
}
/// Neutrosophic set: truth T, indeterminacy I, falsity F ∈ [0,1].
pub fn neutrosophic_set_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// Single-valued neutrosophic set: T, I, F ∈ [0,1], T+I+F ≤ 3.
pub fn single_valued_neutrosophic_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Neutrosophic logic conjunction: min(T1,T2), max(I1,I2), max(F1,F2).
pub fn neutrosophic_conjunction_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Neutrosophic logic disjunction: max(T1,T2), min(I1,I2), min(F1,F2).
pub fn neutrosophic_disjunction_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// Interval neutrosophic set: T, I, F are sub-intervals of [0,1].
pub fn interval_neutrosophic_ty() -> Expr {
    arrow(prop(), prop())
}
/// Register all extended fuzzy logic axioms in the environment.
pub fn build_fuzzy_logic_env_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ArchimedeanTNorm", archimedean_tnorm_ty()),
        ("NilpotentTNorm", nilpotent_tnorm_ty()),
        ("StrictTNorm", strict_tnorm_ty()),
        ("FrankTNorm", frank_tnorm_ty()),
        ("YagerTNorm", yager_tnorm_ty()),
        ("DeMorganTriplet", de_morgan_triplet_ty()),
        ("PossibilityMeasure", possibility_measure_ty()),
        ("NecessityMeasure", necessity_measure_ty()),
        ("Maxitivity", maxitivity_ty()),
        ("PossibilityDistribution", possibility_distribution_ty()),
        ("QualitativePossibility", qualitative_possibility_ty()),
        ("LowerApproximation", lower_approximation_ty()),
        ("UpperApproximation", upper_approximation_ty()),
        ("BoundaryRegion", boundary_region_ty()),
        ("RoughReduct", rough_reduct_ty()),
        ("AttributeDependency", attribute_dependency_ty()),
        ("VariablePrecisionRough", variable_precision_rough_ty()),
        ("FuzzyNumber", fuzzy_number_ty()),
        ("LRFuzzyNumber", lr_fuzzy_number_ty()),
        ("ExtensionPrinciple", extension_principle_ty()),
        ("FuzzyAddition", fuzzy_addition_ty()),
        ("FuzzyMultiplication", fuzzy_multiplication_ty()),
        ("TriangularFuzzyNumber", triangular_fuzzy_number_ty()),
        ("FuzzyRuleBase", fuzzy_rule_base_ty()),
        ("FuzzyInferenceEngine", fuzzy_inference_engine_ty()),
        ("DefuzzificationStrategy", defuzzification_strategy_ty()),
        ("ApproximateReasoning", approximate_reasoning_ty()),
        ("FuzzyPID", fuzzy_pid_ty()),
        ("Type2FuzzySet", type2_fuzzy_set_ty()),
        ("IntervalType2", interval_type2_ty()),
        ("FootprintUncertainty", footprint_uncertainty_ty()),
        ("TypeReduction", type_reduction_ty()),
        ("LinguisticVariable", linguistic_variable_ty()),
        ("LinguisticHedge", linguistic_hedge_ty()),
        ("ComputingWithWords", computing_with_words_ty()),
        ("LinguisticApproximation", linguistic_approximation_ty()),
        ("FuzzyCMeans", fuzzy_cmeans_ty()),
        ("PossibilisticCMeans", possibilistic_cmeans_ty()),
        ("GustafsonKessel", gustafson_kessel_ty()),
        ("FuzzySilhouette", fuzzy_silhouette_ty()),
        ("PartitionCoefficient", partition_coefficient_ty()),
        ("IntuitionisticFuzzySet", intuitionistic_fuzzy_set_ty()),
        ("BipolarFuzzySet", bipolar_fuzzy_set_ty()),
        ("PythagoreanFuzzySet", pythagorean_fuzzy_set_ty()),
        ("HesitantFuzzySet", hesitant_fuzzy_set_ty()),
        ("QRungOrthopair", q_rung_orthopair_ty()),
        ("NeutrosophicSet", neutrosophic_set_ty()),
        ("SingleValuedNeutrosophic", single_valued_neutrosophic_ty()),
        ("NeutrosophicConjunction", neutrosophic_conjunction_ty()),
        ("NeutrosophicDisjunction", neutrosophic_disjunction_ty()),
        ("IntervalNeutrosophic", interval_neutrosophic_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_tnorm_computer_frank() {
        let prod = TNormComputer::frank(1.0, 0.5, 0.6);
        assert!(
            (prod - 0.3).abs() < 1e-6,
            "Frank s=1 should give product: {prod}"
        );
        let min_approx = TNormComputer::frank(1e12, 0.4, 0.7);
        assert!(
            (min_approx - 0.4).abs() < 0.01,
            "Frank large s ≈ min: {min_approx}"
        );
    }
    #[test]
    fn test_tnorm_computer_yager() {
        let large_p = TNormComputer::yager(100.0, 0.3, 0.8);
        assert!(large_p <= 0.3 + 1e-3, "Yager large p ≤ min: {large_p}");
        let luk = TNormComputer::yager(1.0, 0.6, 0.8);
        let expected = (0.6_f64 + 0.8 - 1.0).max(0.0);
        assert!(
            (luk - expected).abs() < 1e-9,
            "Yager p=1 = Lukasiewicz: {luk}"
        );
    }
    #[test]
    fn test_tnorm_computer_axioms() {
        let t = |a: f64, b: f64| TNormComputer::frank(2.0, a, b);
        let samples = [(0.3, 0.7), (0.5, 0.5), (0.1, 0.9)];
        assert!(TNormComputer::check_commutativity(&t, &samples));
        assert!(TNormComputer::check_boundary(&t, &[0.2, 0.5, 0.8]));
    }
    #[test]
    fn test_fuzzy_cmeans_basic() {
        let data: Vec<Vec<f64>> = (0..10)
            .map(|i| vec![i as f64 * 0.1])
            .chain((0..10).map(|i| vec![0.9 + i as f64 * 0.01]))
            .collect();
        let fcm = FuzzyCMeans::new(2).with_max_iter(50);
        let (membership, centers) = fcm.fit(&data);
        assert_eq!(membership.len(), data.len());
        assert_eq!(centers.len(), 2);
        for row in &membership {
            let s: f64 = row.iter().sum();
            assert!(
                (s - 1.0).abs() < 1e-6,
                "Membership row should sum to 1: {s}"
            );
        }
    }
    #[test]
    fn test_fuzzy_cmeans_partition_coefficient() {
        let data: Vec<Vec<f64>> = vec![vec![0.0], vec![0.1], vec![0.9], vec![1.0]];
        let fcm = FuzzyCMeans::new(2).with_max_iter(30);
        let (membership, _) = fcm.fit(&data);
        let pc = FuzzyCMeans::partition_coefficient(&membership);
        assert!(
            pc >= 0.5 && pc <= 1.0,
            "Partition coefficient in [0.5,1]: {pc}"
        );
    }
    #[test]
    fn test_linguistic_hedge_applier() {
        assert!((LinguisticHedgeApplier::very(0.5) - 0.25).abs() < 1e-9);
        let ml = LinguisticHedgeApplier::more_or_less(0.25);
        assert!((ml - 0.5).abs() < 1e-9);
        assert!((LinguisticHedgeApplier::not(0.3) - 0.7).abs() < 1e-9);
        let ind = LinguisticHedgeApplier::indeed(0.5);
        assert!((ind - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_linguistic_hedge_chain() {
        let d = 0.8_f64;
        let vv = LinguisticHedgeApplier::apply_chain(&["very", "very"], d);
        assert!((vv - d.powi(4)).abs() < 1e-9, "very very = μ^4: {vv}");
    }
    #[test]
    fn test_linguistic_hedge_apply_to_set() {
        let mut fs = FuzzySet::new(3);
        fs.set(0, 0.4);
        fs.set(1, 0.7);
        fs.set(2, 1.0);
        let concentrated = LinguisticHedgeApplier::apply_to_set("very", &fs);
        assert!((concentrated.get(0) - 0.16).abs() < 1e-9);
        assert!((concentrated.get(1) - 0.49).abs() < 1e-9);
        assert!((concentrated.get(2) - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_fuzzy_inference_system_mamdani() {
        let domain: Vec<f64> = (0..=10).map(|i| i as f64).collect();
        let mut fis = FuzzyInferenceSystem::mamdani(11, domain);
        let mut cons = FuzzySet::new(11);
        for i in 7..=10 {
            cons.set(i, (i as f64 - 6.0) / 4.0);
        }
        fis.add_mamdani_rule(vec![0.8], cons);
        assert!(fis.is_configured());
        let result = fis.infer_mamdani(&[vec![0.8]]);
        assert!(
            result >= 0.0 && result <= 10.0,
            "Mamdani output in domain: {result}"
        );
    }
    #[test]
    fn test_fuzzy_inference_system_sugeno() {
        let mut fis = FuzzyInferenceSystem::sugeno(1);
        fis.add_sugeno_rule(vec![0.7], vec![0.0], 0.2);
        fis.add_sugeno_rule(vec![0.3], vec![0.0], 0.8);
        assert!(fis.is_configured());
        let result = fis.infer_sugeno(&[0.5], &[vec![0.7], vec![0.3]]);
        assert!((result - 0.38).abs() < 1e-6, "Sugeno output: {result}");
    }
    #[test]
    fn test_build_fuzzy_logic_env_extended() {
        let mut env = Environment::new();
        build_fuzzy_logic_env_extended(&mut env)
            .expect("build_fuzzy_logic_env_extended should succeed");
        assert!(env.get(&Name::str("ArchimedeanTNorm")).is_some());
        assert!(env.get(&Name::str("PossibilityMeasure")).is_some());
        assert!(env.get(&Name::str("LowerApproximation")).is_some());
        assert!(env.get(&Name::str("FuzzyNumber")).is_some());
        assert!(env.get(&Name::str("FuzzyRuleBase")).is_some());
        assert!(env.get(&Name::str("Type2FuzzySet")).is_some());
        assert!(env.get(&Name::str("LinguisticVariable")).is_some());
        assert!(env.get(&Name::str("FuzzyCMeans")).is_some());
        assert!(env.get(&Name::str("IntuitionisticFuzzySet")).is_some());
        assert!(env.get(&Name::str("NeutrosophicSet")).is_some());
    }
}
#[cfg(test)]
mod tests_fuzzy_extra {
    use super::*;
    #[test]
    fn test_fuzzy_clustering() {
        let fc = FuzzyClustering::new(6, 3, 2.0);
        let pc = fc.partition_coefficient();
        assert!(
            (pc - 1.0 / 3.0).abs() < 1e-9,
            "PC should be 1/c={}",
            1.0 / 3.0
        );
        let assign = fc.hard_assignment(0);
        assert!(assign < 3);
    }
    #[test]
    fn test_mamdani_centroid() {
        let vals = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let mems = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let c = MamdaniEngine::centroid_defuzz(&vals, &mems);
        assert!((c - 2.0).abs() < 1e-9, "centroid should be 2.0");
    }
    #[test]
    fn test_gradual_element() {
        let a = GradualElement::new("hot", 0.8);
        let b = GradualElement::new("humid", 0.6);
        assert!(a.is_true());
        let c = a.complement();
        assert!((c.degree - 0.2).abs() < 1e-9);
        let conj = a.conjunction(&b);
        assert!((conj.degree - 0.6).abs() < 1e-9);
        let disj = a.disjunction(&b);
        assert!((disj.degree - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_fuzzy_rough_approx() {
        let mut fra = FuzzyRoughApprox::new(3);
        fra.set_similarity(0, 1, 0.8);
        fra.set_similarity(1, 2, 0.6);
        fra.set_similarity(0, 2, 0.4);
        let a = vec![1.0, 0.7, 0.3];
        let lower = fra.lower_approx(&a);
        let upper = fra.upper_approx(&a);
        assert_eq!(lower.len(), 3);
        assert_eq!(upper.len(), 3);
        for i in 0..3 {
            assert!(upper[i] >= lower[i] - 1e-9);
        }
    }
    #[test]
    fn test_triangular_fuzzy_num() {
        let t = TriangularFuzzyNum::new(1.0, 2.0, 4.0);
        assert!((t.membership(2.0) - 1.0).abs() < 1e-9);
        assert!((t.membership(0.5) - 0.0).abs() < 1e-9);
        assert!(t.membership(1.5) > 0.0 && t.membership(1.5) < 1.0);
        let t2 = TriangularFuzzyNum::new(0.0, 1.0, 2.0);
        let sum = t.add(&t2);
        assert!((sum.modal - 3.0).abs() < 1e-9);
        let (lo, hi) = t.alpha_cut(0.5);
        assert!(lo < hi);
        let c = t.defuzzify_centroid();
        assert!((c - 7.0 / 3.0).abs() < 1e-9);
    }
}
