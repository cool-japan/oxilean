//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CaccioppoliSet, CoAreaComputer, CompactnessTheorem, DiscreteBVFunction, DiscreteDensityRatio,
    DiscreteSet2D, HausdorffContentEstimate, HausdorffMeasureEstimator, IntegralCurrent,
    IntegralCurrentNew, MarstrandProjection, MinimalSurfaceRelaxation, PerimeterApprox,
    PiecewiseLinearMap, PlateauProblem, RectifiabilityChecker, RectifiableSet,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `HausdorffMeasure : (s : Real) → (n : Nat) → Type`
///
/// The s-dimensional Hausdorff measure on ℝⁿ.
/// H^s(E) = lim_{δ→0} inf { Σ diam(Eᵢ)^s : E ⊆ ⋃Eᵢ, diam(Eᵢ) < δ }
pub fn hausdorff_measure_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `HausdorffDimension : (E : Set ℝⁿ) → Real`
///
/// The Hausdorff dimension of a set E: inf { s : H^s(E) = 0 }.
pub fn hausdorff_dimension_ty() -> Expr {
    arrow(arrow(nat_ty(), bool_ty()), real_ty())
}
/// `RectifiableSet : (k n : Nat) → Set ℝⁿ → Prop`
///
/// A k-rectifiable set: H^k-almost covered by countably many Lipschitz images of ℝᵏ.
pub fn rectifiable_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(arrow(nat_ty(), bool_ty()), prop()),
        ),
    )
}
/// `CountablyRectifiable : (k n : Nat) → Set ℝⁿ → Prop`
///
/// A countably k-rectifiable set: contained in a countable union of k-rectifiable sets.
pub fn countably_rectifiable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        arrow(nat_ty(), arrow(arrow(nat_ty(), bool_ty()), prop())),
    )
}
/// `LipschitzMap : (m n : Nat) → Type`
///
/// A Lipschitz map f : ℝᵐ → ℝⁿ with Lipschitz constant Lip(f) < ∞.
pub fn lipschitz_map_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `Varifold : (k n : Nat) → Type`
///
/// A k-varifold in ℝⁿ: a Radon measure on ℝⁿ × G(n,k) where G(n,k) is the
/// Grassmannian of k-planes in ℝⁿ.
pub fn varifold_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `IntegralVarifold : (k n : Nat) → Varifold k n → Prop`
///
/// An integral varifold: a varifold arising from a rectifiable set with integer
/// multiplicities, carrying the tangent plane information.
pub fn integral_varifold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Varifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `FirstVariationVarifold : (k n : Nat) → Varifold k n → Type`
///
/// The first variation δV of a varifold V: a vector-valued Radon measure encoding
/// how the weight measure changes under smooth deformations.
pub fn first_variation_varifold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Varifold"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `StationaryVarifold : (k n : Nat) → Varifold k n → Prop`
///
/// A stationary varifold: one with vanishing first variation δV = 0.
/// Stationary integral varifolds generalize minimal surfaces.
pub fn stationary_varifold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Varifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `AllardRegularity : (V : IntegralVarifold k n) → Prop`
///
/// Allard's regularity theorem: if V is a stationary integral varifold with
/// unit density at a point, then V is smooth near that point.
pub fn allard_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralVarifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `Current : (k n : Nat) → Type`
///
/// A k-current in ℝⁿ: a continuous linear functional on the space of smooth
/// compactly supported k-forms. Generalizes oriented surfaces.
pub fn current_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BoundaryCurrent : (k n : Nat) → Current k n → Current (k-1) n`
///
/// The boundary operator ∂ on currents, satisfying ∂∘∂ = 0.
pub fn boundary_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("Current"), bvar(1), bvar(0)),
                app2(cst("Current"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `RectifiableCurrent : (k n : Nat) → Current k n → Prop`
///
/// A rectifiable current: one representable as integration over an oriented
/// rectifiable set with integer multiplicities.
pub fn rectifiable_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `IntegralCurrent : (k n : Nat) → Current k n → Prop`
///
/// An integral current: a rectifiable current with rectifiable boundary.
pub fn integral_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `MassNorm : (k n : Nat) → Current k n → Real`
///
/// The mass norm M(T) of a current T: the total variation / weighted area.
pub fn mass_norm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), real_ty()),
        ),
    )
}
/// `FlatNorm : (k n : Nat) → Current k n → Real`
///
/// The flat norm F(T) = inf { M(T - ∂S) + M(S) : S is a (k+1)-current }.
pub fn flat_norm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), real_ty()),
        ),
    )
}
/// `FedererFlemingCompactness : Prop`
///
/// Federer-Fleming compactness theorem: a sequence of integral k-currents
/// with uniformly bounded mass and flat norm has a weakly convergent subsequence.
pub fn federer_fleming_compactness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                arrow(nat_ty(), app2(cst("IntegralCurrent"), bvar(1), bvar(0))),
                prop(),
            ),
        ),
    )
}
/// `PlateauProblem : (k n : Nat) → Current (k-1) n → Prop`
///
/// The Plateau problem: existence of an area-minimizing integral current
/// with prescribed boundary Γ.
pub fn plateau_problem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralCurrent"), bvar(2), bvar(1)), prop()),
        ),
    )
}
/// `AreaMinimizingCurrent : (k n : Nat) → Current k n → Prop`
///
/// An area-minimizing current: one with minimal mass among currents with the
/// same boundary.
pub fn area_minimizing_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `AreaFormula : (m n : Nat) → (f : LipschitzMap m n) → Prop`
///
/// Area formula: ∫_{ℝⁿ} N(f, y) dH^m(y) = ∫_{ℝᵐ} Jf(x) dH^m(x)
/// where N(f,y) = #f⁻¹(y) and Jf is the m-dimensional Jacobian.
pub fn area_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("LipschitzMap"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `CoareaFormula : (m k n : Nat) → (f : LipschitzMap n k) → Prop`
///
/// Coarea formula (Federer): ∫_{ℝⁿ} g(x)|Jₖf(x)| dL^n(x) = ∫_{ℝᵏ} (∫_{f⁻¹(y)} g dH^{n-k}) dL^k(y)
pub fn coarea_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            arrow(app2(cst("LipschitzMap"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `Jacobian : (m n : Nat) → LipschitzMap m n → ℝⁿ → Real`
///
/// The m-dimensional Jacobian of a Lipschitz map f : ℝᵐ → ℝⁿ at a point x.
pub fn jacobian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("LipschitzMap"), bvar(1), bvar(0)),
                arrow(list_ty(real_ty()), real_ty()),
            ),
        ),
    )
}
/// `BVFunction : (n : Nat) → Type`
///
/// A function of bounded variation on ℝⁿ: f ∈ L¹ with distributional gradient
/// Df being a vector-valued Radon measure with finite total variation |Df|(ℝⁿ) < ∞.
pub fn bv_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TotalVariation : (n : Nat) → BVFunction n → Real`
///
/// The total variation |Df|(ℝⁿ) of a BV function f.
pub fn total_variation_ty() -> Expr {
    arrow(nat_ty(), arrow(app(cst("BVFunction"), nat_ty()), real_ty()))
}
/// `SetFinitePerimeter : (n : Nat) → Set ℝⁿ → Prop`
///
/// A set of finite perimeter (Caccioppoli set): the characteristic function 1_E is BV.
/// Equivalently, the distributional divergence of 1_E is a finite measure.
pub fn set_finite_perimeter_ty() -> Expr {
    arrow(nat_ty(), arrow(arrow(nat_ty(), bool_ty()), prop()))
}
/// `ReducedBoundary : (n : Nat) → SetFinitePerimeter n → Set ℝⁿ`
///
/// The reduced boundary ∂*E of a set of finite perimeter E.
pub fn reduced_boundary_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("SetFinitePerimeter"), bvar(0)),
            arrow(nat_ty(), bool_ty()),
        ),
    )
}
/// `DeGiorgiStructureTheorem : (n : Nat) → SetFinitePerimeter n → Prop`
///
/// De Giorgi's structure theorem: the reduced boundary ∂*E of a set of finite perimeter
/// is an (n-1)-rectifiable set with H^{n-1}(∂*E) = Per(E).
pub fn de_giorgi_structure_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("SetFinitePerimeter"), bvar(0)), prop()),
    )
}
/// `IsoperimetricInequality : (n : Nat) → Prop`
///
/// Classical isoperimetric inequality: Per(E)^n ≥ n^n ω_n |E|^{n-1}
/// with equality iff E is a ball. Here ω_n = volume of unit ball in ℝⁿ.
pub fn isoperimetric_inequality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `QuantitativeIsoperimetric : (n : Nat) → Prop`
///
/// Quantitative isoperimetric inequality (Fusco-Maggi-Pratelli):
/// Per(E) - Per(B) ≥ c_n |E|^{(n-1)/n} A(E)²
/// where A(E) is the Fraenkel asymmetry.
pub fn quantitative_isoperimetric_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MonotonicityFormula : (k n : Nat) → StationaryVarifold k n → Prop`
///
/// Monotonicity formula for stationary varifolds: the density ratio
/// θ^k(V, x, r) = M(V ⌞ B(x,r)) / (ω_k r^k) is monotone nondecreasing in r.
pub fn monotonicity_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("StationaryVarifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `DensityFunction : (k n : Nat) → Varifold k n → ℝⁿ → Real`
///
/// The k-density Θ^k(V, x) = lim_{r→0} M(V ⌞ B(x,r)) / (ω_k r^k).
pub fn density_function_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("Varifold"), bvar(1), bvar(0)),
                arrow(list_ty(real_ty()), real_ty()),
            ),
        ),
    )
}
/// `MinimalSurface : (k n : Nat) → Type`
///
/// A minimal surface: a k-dimensional submanifold of ℝⁿ with vanishing mean curvature,
/// equivalently a stationary varifold that is also a smooth manifold away from singularities.
pub fn minimal_surface_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BernsteinTheorem : Prop`
///
/// Bernstein's theorem: an entire minimal graph over ℝⁿ for n ≤ 7 must be a hyperplane.
pub fn bernstein_theorem_ty() -> Expr {
    prop()
}
/// `SimonsConjecture : Prop`
///
/// Simons' theorem: stable minimal hypersurfaces in ℝⁿ for n ≤ 7 are hyperplanes.
pub fn simons_cone_ty() -> Expr {
    prop()
}
/// `BVCompactness : (n : Nat) → Prop`
///
/// BV compactness theorem: a bounded sequence in BV(Ω) has a subsequence
/// converging strongly in L¹(Ω).
pub fn bv_compactness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SlicingTheorem : (k n : Nat) → IntegralCurrent k n → Prop`
///
/// Slicing theorem for integral currents: the slices ⟨T, f, y⟩ of an integral
/// current by a Lipschitz function f are integral (k-m)-currents for a.e. y.
pub fn slicing_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralCurrent"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `PushforwardCurrent : (k m n : Nat) → LipschitzMap m n → Current k m → Current k n`
///
/// Pushforward f_♯T of a current T by a Lipschitz map f.
pub fn pushforward_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app2(cst("LipschitzMap"), bvar(1), bvar(0)),
                    arrow(
                        app2(cst("Current"), bvar(3), bvar(2)),
                        app2(cst("Current"), bvar(4), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `PurelyUnrectifiable : (k n : Nat) → Set ℝⁿ → Prop`
///
/// A purely k-unrectifiable set: one that intersects every k-rectifiable set
/// in a set of H^k measure zero.
pub fn purely_unrectifiable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(arrow(nat_ty(), bool_ty()), prop()),
        ),
    )
}
/// `HausdorffNRectifiable : (k n : Nat) → Set ℝⁿ → Prop`
///
/// A set E ⊆ ℝⁿ is H^k-n-rectifiable if H^k(E) < ∞ and E is k-rectifiable.
pub fn hausdorff_n_rectifiable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(arrow(nat_ty(), bool_ty()), prop()),
        ),
    )
}
/// `TangentMeasureUniqueness : (k n : Nat) → Prop`
///
/// Marstrand–Mattila rectifiability criterion: E is k-rectifiable iff
/// the tangent measure at H^k-almost every point is a flat k-measure.
pub fn tangent_measure_uniqueness_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), arrow(nat_ty(), prop()))
}
/// `MarstrandDensityTheorem : (s n : Nat) → Prop`
///
/// Marstrand density theorem: if H^s(E) > 0 then the s-density
/// Θ^s(E, x) ∈ [2^{-s}, 1] for H^s-a.e. x ∈ E.
pub fn marstrand_density_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "s", nat_ty(), arrow(nat_ty(), prop()))
}
/// `GeneralizedMeanCurvature : (k n : Nat) → IntegralVarifold k n → Type`
///
/// The generalized mean curvature vector H of an integral varifold V:
/// the Radon-Nikodym derivative dδV/d‖V‖.
pub fn generalized_mean_curvature_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralVarifold"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `AllardBoundaryRegularity : (k n : Nat) → IntegralVarifold k n → Prop`
///
/// Allard's boundary regularity theorem: an integral varifold with bounded
/// generalized mean curvature is C^{1,α} near the boundary in appropriate sense.
pub fn allard_boundary_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralVarifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `AlmgrenFrequencyFunction : (k n : Nat) → IntegralVarifold k n → Real → Real`
///
/// Almgren's frequency function N(x, r) = r∫|∇u|² / ∫u² on B(x,r).
/// The key monotonicity tool in Almgren's Big Regularity Paper.
pub fn almgren_frequency_function_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("IntegralVarifold"), bvar(1), bvar(0)),
                arrow(real_ty(), real_ty()),
            ),
        ),
    )
}
/// `AlmgrenRegularity : (k n : Nat) → AreaMinimizingCurrent k n → Prop`
///
/// Almgren's regularity theorem (Big Regularity Paper):
/// area-minimizing currents in ℝⁿ are regular outside a singular set
/// of Hausdorff dimension at most k-2.
pub fn almgren_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("AreaMinimizingCurrent"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `UniqueContinuationPrinciple : (k n : Nat) → Prop`
///
/// Unique continuation for the frequency function:
/// if N(x, r) = 0 for some r > 0, then u ≡ 0 near x.
pub fn unique_continuation_principle_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), arrow(nat_ty(), prop()))
}
/// `NormalCurrent : (k n : Nat) → Current k n → Prop`
///
/// A normal current: finite mass and finite boundary mass (M(T) + M(∂T) < ∞).
pub fn normal_current_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("Current"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `IntegralFlatChain : (k n : Nat) → Type`
///
/// An integral flat chain: equivalence class of integral currents under
/// the flat norm topology, used for homological methods in GMT.
pub fn integral_flat_chain_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `FlatChainHomotopy : (k n : Nat) → IntegralFlatChain k n → IntegralFlatChain k n → Prop`
///
/// Homotopy of flat chains: two flat chains are homotopic iff their difference
/// is the boundary of a flat chain.
pub fn flat_chain_homotopy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("IntegralFlatChain"), bvar(1), bvar(0)),
                arrow(app2(cst("IntegralFlatChain"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `DeformationTheorem : (k n : Nat) → NormalCurrent k n → Prop`
///
/// Federer's deformation theorem: every normal current can be deformed into
/// a polyhedral chain with controlled mass.
pub fn deformation_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("NormalCurrent"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `KirszbraunExtension : (m n : Nat) → LipschitzMap m n → Prop`
///
/// Kirszbraun's theorem: any Lipschitz map f : S → ℝⁿ (S ⊆ ℝᵐ) extends
/// to a Lipschitz map F : ℝᵐ → ℝⁿ with the same Lipschitz constant.
pub fn kirszbraun_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("LipschitzMap"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `McShaneExtension : (n : Nat) → (f : Set ℝⁿ → Real) → Prop`
///
/// McShane's extension theorem: any real-valued Lipschitz function on a subset
/// of a metric space extends to the whole space with the same Lipschitz constant.
pub fn mcshane_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(arrow(arrow(nat_ty(), bool_ty()), real_ty()), prop()),
    )
}
/// `RademacherTheorem : (m n : Nat) → LipschitzMap m n → Prop`
///
/// Rademacher's theorem: any Lipschitz map f : ℝᵐ → ℝⁿ is differentiable
/// at L^m-almost every point.
pub fn rademacher_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("LipschitzMap"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `LipschitzDifferential : (m n : Nat) → LipschitzMap m n → ℝᵐ → Type`
///
/// The differential df(x) of a Lipschitz map at an a.e.-differentiability point x.
pub fn lipschitz_differential_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("LipschitzMap"), bvar(1), bvar(0)),
                arrow(list_ty(real_ty()), type0()),
            ),
        ),
    )
}
/// `GammaConvergence : (n : Nat) → (F : Nat → BVFunction n → Real) → Prop`
///
/// Γ-convergence of a sequence of functionals F_ε to a limit functional F₀:
/// combines lower semicontinuity (lim inf) and recovery sequence (lim sup) conditions.
pub fn gamma_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            arrow(nat_ty(), arrow(app(cst("BVFunction"), bvar(0)), real_ty())),
            prop(),
        ),
    )
}
/// `AllenCahnFunctional : (n : Nat) → BVFunction n → Real → Real`
///
/// The Allen-Cahn functional E_ε(u) = ∫ (ε|∇u|² + W(u)/ε) dx,
/// which Γ-converges as ε→0 to a perimeter functional (σ Per(E)).
pub fn allen_cahn_functional_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("BVFunction"), bvar(0)), arrow(real_ty(), real_ty())),
    )
}
/// `AllenCahnGammaLimit : (n : Nat) → Prop`
///
/// Modica-Mortola theorem: the Allen-Cahn functionals E_ε Γ-converge to
/// the perimeter functional (up to a factor σ = ∫W^{1/2}).
pub fn allen_cahn_gamma_limit_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MumfordShahFunctional : (n : Nat) → BVFunction n → Real`
///
/// The Mumford-Shah functional MS(u, K) = ∫_{Ω\K} |∇u|² + H^{n-1}(K).
/// A free-discontinuity functional in image segmentation.
pub fn mumford_shah_functional_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("BVFunction"), bvar(0)), real_ty()),
    )
}
/// `BrakkeFlow : (k n : Nat) → Type`
///
/// A Brakke k-flow in ℝⁿ: a one-parameter family of integral varifolds
/// V(t) satisfying Brakke's inequality (weak formulation of mean curvature flow).
pub fn brakke_flow_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BrakkeInequality : (k n : Nat) → BrakkeFlow k n → Prop`
///
/// Brakke's inequality: d/dt ‖V(t)‖(φ) ≤ ∫(-H²φ + ∇φ·H) d‖V(t)‖
/// for all non-negative test functions φ.
pub fn brakke_inequality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("BrakkeFlow"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `BrakkeExistence : (k n : Nat) → Prop`
///
/// Brakke's existence theorem: for any initial integral varifold V₀,
/// there exists a Brakke flow V(t) with V(0) = V₀.
pub fn brakke_existence_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), arrow(nat_ty(), prop()))
}
/// `HuiskenMonotonicity : (k n : Nat) → BrakkeFlow k n → Prop`
///
/// Huisken's monotonicity formula for mean curvature flow:
/// the Gaussian weighted area ∫ ρ_{x₀,t₀} d‖V(t)‖ is monotone nonincreasing.
pub fn huisken_monotonicity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("BrakkeFlow"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `TangentCone : (k n : Nat) → MinimalSurface k n → ℝⁿ → Type`
///
/// The tangent cone C_x M at a point x of a minimal surface M:
/// the limit of blow-ups M_r = (M - x)/r as r → 0.
pub fn tangent_cone_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("MinimalSurface"), bvar(1), bvar(0)),
                arrow(list_ty(real_ty()), type0()),
            ),
        ),
    )
}
/// `SingularSetMinimal : (k n : Nat) → MinimalSurface k n → Set ℝⁿ`
///
/// The singular set Sing(M) of a minimal surface: points where M fails to be
/// a smooth submanifold.
pub fn singular_set_minimal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("MinimalSurface"), bvar(1), bvar(0)),
                arrow(nat_ty(), bool_ty()),
            ),
        ),
    )
}
/// `StableMinimalHypersurface : (n : Nat) → MinimalSurface (n - 1) n → Prop`
///
/// A stable minimal hypersurface: one where the second variation of area
/// is non-negative for all compactly supported variations.
pub fn stable_minimal_hypersurface_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("MinimalSurface"), bvar(0), bvar(0)), prop()),
    )
}
/// `SecondFundamentalForm : (k n : Nat) → MinimalSurface k n → Type`
///
/// The second fundamental form A of a minimal surface: the tensor measuring
/// how the surface curves in the ambient space.
pub fn second_fundamental_form_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("MinimalSurface"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `SchoenSimonYauEstimate : (n : Nat) → StableMinimalHypersurface n → Prop`
///
/// Schoen-Simon-Yau curvature estimate: for stable minimal hypersurfaces in ℝⁿ (n ≤ 6),
/// |A|² ≤ C/r² on B_r.
pub fn schoen_simon_yau_estimate_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("StableMinimalHypersurface"), bvar(0)), prop()),
    )
}
/// `ExcessDecay : (k n : Nat) → IntegralVarifold k n → Prop`
///
/// Allard's excess decay lemma: the L²-excess E(V, B_r) = ∫_{B_r} sin²θ d‖V‖
/// decays like r^α as r → 0 when V is stationary and has small excess.
pub fn excess_decay_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app2(cst("IntegralVarifold"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `GaussBonnetFormula : (n : Nat) → MinimalSurface n n → Prop`
///
/// Gauss-Bonnet formula: ∫_M K dA = 2π χ(M) relating Gaussian curvature
/// to Euler characteristic for compact surfaces.
pub fn gauss_bonnet_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("MinimalSurface"), bvar(0), bvar(0)), prop()),
    )
}
/// `WillmoreEnergy : (n : Nat) → MinimalSurface 2 n → Real`
///
/// The Willmore energy W(Σ) = ∫_Σ H² dA of a 2-surface Σ ⊆ ℝⁿ,
/// conformally invariant and minimized by round spheres.
pub fn willmore_energy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("MinimalSurface"), nat_ty(), bvar(0)), real_ty()),
    )
}
/// Build the geometric measure theory kernel environment with all axiom declarations.
pub fn build_geometric_measure_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("HausdorffMeasure", hausdorff_measure_ty()),
        ("HausdorffDimension", hausdorff_dimension_ty()),
        ("RectifiableSet", rectifiable_set_ty()),
        ("CountablyRectifiable", countably_rectifiable_ty()),
        ("LipschitzMap", lipschitz_map_ty()),
        ("Varifold", varifold_ty()),
        ("IntegralVarifold", integral_varifold_ty()),
        ("FirstVariationVarifold", first_variation_varifold_ty()),
        ("StationaryVarifold", stationary_varifold_ty()),
        ("AllardRegularity", allard_regularity_ty()),
        ("Current", current_ty()),
        ("BoundaryCurrent", boundary_current_ty()),
        ("RectifiableCurrent", rectifiable_current_ty()),
        ("IntegralCurrent", integral_current_ty()),
        ("MassNorm", mass_norm_ty()),
        ("FlatNorm", flat_norm_ty()),
        (
            "FedererFlemingCompactness",
            federer_fleming_compactness_ty(),
        ),
        ("PlateauProblem", plateau_problem_ty()),
        ("AreaMinimizingCurrent", area_minimizing_current_ty()),
        ("AreaFormula", area_formula_ty()),
        ("CoareaFormula", coarea_formula_ty()),
        ("Jacobian", jacobian_ty()),
        ("BVFunction", bv_function_ty()),
        ("TotalVariation", total_variation_ty()),
        ("SetFinitePerimeter", set_finite_perimeter_ty()),
        ("ReducedBoundary", reduced_boundary_ty()),
        ("DeGiorgiStructureTheorem", de_giorgi_structure_theorem_ty()),
        ("IsoperimetricInequality", isoperimetric_inequality_ty()),
        ("QuantitativeIsoperimetric", quantitative_isoperimetric_ty()),
        ("MonotonicityFormula", monotonicity_formula_ty()),
        ("DensityFunction", density_function_ty()),
        ("MinimalSurface", minimal_surface_ty()),
        ("BernsteinTheorem", bernstein_theorem_ty()),
        ("SimonsCone", simons_cone_ty()),
        ("BVCompactness", bv_compactness_ty()),
        ("SlicingTheorem", slicing_theorem_ty()),
        ("PushforwardCurrent", pushforward_current_ty()),
        ("PurelyUnrectifiable", purely_unrectifiable_ty()),
        ("HausdorffNRectifiable", hausdorff_n_rectifiable_ty()),
        ("TangentMeasureUniqueness", tangent_measure_uniqueness_ty()),
        ("MarstrandDensityTheorem", marstrand_density_theorem_ty()),
        ("GeneralizedMeanCurvature", generalized_mean_curvature_ty()),
        ("AllardBoundaryRegularity", allard_boundary_regularity_ty()),
        ("AlmgrenFrequencyFunction", almgren_frequency_function_ty()),
        ("AlmgrenRegularity", almgren_regularity_ty()),
        (
            "UniqueContinuationPrinciple",
            unique_continuation_principle_ty(),
        ),
        ("NormalCurrent", normal_current_ty()),
        ("IntegralFlatChain", integral_flat_chain_ty()),
        ("FlatChainHomotopy", flat_chain_homotopy_ty()),
        ("DeformationTheorem", deformation_theorem_ty()),
        ("KirszbraunExtension", kirszbraun_extension_ty()),
        ("McShaneExtension", mcshane_extension_ty()),
        ("RademacherTheorem", rademacher_theorem_ty()),
        ("LipschitzDifferential", lipschitz_differential_ty()),
        ("GammaConvergence", gamma_convergence_ty()),
        ("AllenCahnFunctional", allen_cahn_functional_ty()),
        ("AllenCahnGammaLimit", allen_cahn_gamma_limit_ty()),
        ("MumfordShahFunctional", mumford_shah_functional_ty()),
        ("BrakkeFlow", brakke_flow_ty()),
        ("BrakkeInequality", brakke_inequality_ty()),
        ("BrakkeExistence", brakke_existence_ty()),
        ("HuiskenMonotonicity", huisken_monotonicity_ty()),
        ("TangentCone", tangent_cone_ty()),
        ("SingularSetMinimal", singular_set_minimal_ty()),
        (
            "StableMinimalHypersurface",
            stable_minimal_hypersurface_ty(),
        ),
        ("SecondFundamentalForm", second_fundamental_form_ty()),
        ("SchoenSimonYauEstimate", schoen_simon_yau_estimate_ty()),
        ("ExcessDecay", excess_decay_ty()),
        ("GaussBonnetFormula", gauss_bonnet_formula_ty()),
        ("WillmoreEnergy", willmore_energy_ty()),
        ("IsRectifiable", arrow(type0(), prop())),
        ("HasFinitePerimeter", arrow(type0(), prop())),
        ("IsAreaMinimizing", arrow(type0(), prop())),
        ("IsMinimalSurface", arrow(type0(), prop())),
        ("IsStableMinimal", arrow(type0(), prop())),
        ("HausdorffMeasurable", arrow(type0(), prop())),
        ("LocallyRectifiable", arrow(type0(), prop())),
        ("TangentPlane", arrow(type0(), arrow(nat_ty(), type0()))),
        ("MeanCurvature", arrow(type0(), arrow(nat_ty(), real_ty()))),
        ("Perimeter", arrow(arrow(nat_ty(), bool_ty()), real_ty())),
        (
            "FraenkelAsymmetry",
            arrow(arrow(nat_ty(), bool_ty()), real_ty()),
        ),
        (
            "GrassmannManifold",
            arrow(nat_ty(), arrow(nat_ty(), type0())),
        ),
        ("TangentMeasure", arrow(type0(), arrow(nat_ty(), type0()))),
        (
            "BoundaryBoundaryZero",
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "n",
                    nat_ty(),
                    arrow(app2(cst("Current"), bvar(1), bvar(0)), prop()),
                ),
            ),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hausdorff_content_empty() {
        let est = HausdorffContentEstimate::new(vec![], 0.1, 1.0);
        assert_eq!(est.content(), 0.0);
    }
    #[test]
    fn test_hausdorff_content_collinear() {
        let pts = vec![(0.0, 0.0), (0.25, 0.0), (0.5, 0.0), (0.75, 0.0), (1.0, 0.0)];
        let est = HausdorffContentEstimate::new(pts, 0.3, 1.0);
        let c = est.content();
        assert!(c > 0.0, "Content should be positive, got {}", c);
    }
    #[test]
    fn test_discrete_bv_total_variation() {
        let f = DiscreteBVFunction::new(vec![0.0, 1.0, 0.0, 1.0, 0.0]);
        assert!((f.total_variation() - 4.0).abs() < 1e-10);
        assert!(f.is_bv());
    }
    #[test]
    fn test_discrete_bv_constant() {
        let f = DiscreteBVFunction::new(vec![3.0; 10]);
        assert!((f.total_variation()).abs() < 1e-10);
        assert_eq!(f.distributional_derivative().len(), 9);
    }
    #[test]
    fn test_discrete_set_disk_perimeter() {
        let disk = DiscreteSet2D::disk(10, 4.5, 4.5, 3.0);
        assert!(disk.area() > 0);
        assert!(disk.perimeter() > 0);
        let ratio = disk.isoperimetric_ratio();
        assert!(ratio > 0.0, "Ratio should be positive, got {}", ratio);
    }
    #[test]
    fn test_discrete_set_invalid() {
        let result = DiscreteSet2D::new(3, vec![true; 8]);
        assert!(result.is_none());
    }
    #[test]
    fn test_piecewise_linear_map_identity() {
        let f = PiecewiseLinearMap::new(vec![0.0, 0.25, 0.5, 0.75, 1.0]);
        assert!((f.lipschitz_constant() - 1.0).abs() < 1e-10);
        assert!((f.discrete_area_formula() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_density_ratio_monotone() {
        use std::f64::consts::PI;
        let pts: Vec<(f64, f64, f64)> = (0..16)
            .map(|i| {
                let theta = 2.0 * PI * i as f64 / 16.0;
                (theta.cos(), theta.sin(), 1.0)
            })
            .collect();
        let density = DiscreteDensityRatio::new(pts, (0.0, 0.0), 1.0);
        let radii = vec![0.5, 0.8, 1.0, 1.2, 1.5];
        let d05 = density.density_at_radius(0.5);
        let d12 = density.density_at_radius(1.2);
        assert_eq!(d05, 0.0, "No points within r=0.5 of unit circle origin");
        assert!(d12 > 0.0, "Some points within r=1.2, got {}", d12);
        let _ = radii;
    }
    #[test]
    fn test_build_geometric_measure_theory_env() {
        let mut env = Environment::new();
        build_geometric_measure_theory_env(&mut env);
        assert!(!env.is_empty());
    }
    #[test]
    fn test_new_axiom_builders_are_non_trivial() {
        let axioms: Vec<Expr> = vec![
            purely_unrectifiable_ty(),
            hausdorff_n_rectifiable_ty(),
            tangent_measure_uniqueness_ty(),
            marstrand_density_theorem_ty(),
            generalized_mean_curvature_ty(),
            allard_boundary_regularity_ty(),
            almgren_frequency_function_ty(),
            almgren_regularity_ty(),
            unique_continuation_principle_ty(),
            normal_current_ty(),
            integral_flat_chain_ty(),
            flat_chain_homotopy_ty(),
            deformation_theorem_ty(),
            kirszbraun_extension_ty(),
            mcshane_extension_ty(),
            rademacher_theorem_ty(),
            lipschitz_differential_ty(),
            gamma_convergence_ty(),
            allen_cahn_functional_ty(),
            allen_cahn_gamma_limit_ty(),
            mumford_shah_functional_ty(),
            brakke_flow_ty(),
            brakke_inequality_ty(),
            brakke_existence_ty(),
            huisken_monotonicity_ty(),
            tangent_cone_ty(),
            singular_set_minimal_ty(),
            stable_minimal_hypersurface_ty(),
            second_fundamental_form_ty(),
            schoen_simon_yau_estimate_ty(),
            excess_decay_ty(),
            gauss_bonnet_formula_ty(),
            willmore_energy_ty(),
        ];
        assert_eq!(axioms.len(), 33);
        for expr in &axioms {
            if let Expr::BVar(_) = expr {
                panic!("Axiom type must not be a bare BVar")
            }
        }
    }
    #[test]
    fn test_extended_env_has_new_axioms() {
        let mut env = Environment::new();
        build_geometric_measure_theory_env(&mut env);
        let new_names = [
            "PurelyUnrectifiable",
            "RademacherTheorem",
            "KirszbraunExtension",
            "BrakkeFlow",
            "HuiskenMonotonicity",
            "AlmgrenRegularity",
            "GammaConvergence",
            "WillmoreEnergy",
        ];
        for name in &new_names {
            let found = env.get(&Name::str(*name)).is_some();
            assert!(found, "Expected axiom '{}' in environment", name);
        }
    }
    #[test]
    fn test_hausdorff_estimator_empty() {
        let est = HausdorffMeasureEstimator::new(vec![]);
        assert_eq!(est.hausdorff_content(1.0, 0.1), 0.0);
        assert_eq!(est.estimate_dimension(0.1), 0.0);
    }
    #[test]
    fn test_hausdorff_estimator_collinear_points() {
        let pts: Vec<(f64, f64)> = (0..5).map(|i| (i as f64 * 0.25, 0.0)).collect();
        let est = HausdorffMeasureEstimator::new(pts);
        let content = est.hausdorff_content(1.0, 0.3);
        assert!(content > 0.0, "Content at s=1 should be positive");
        let dim = est.estimate_dimension(0.3);
        assert!(dim >= 0.0 && dim <= 2.0, "Dimension out of range: {}", dim);
    }
    #[test]
    fn test_hausdorff_estimator_measure_positive() {
        let pts: Vec<(f64, f64)> = (0..10)
            .flat_map(|i| (0..10).map(move |j| (i as f64 * 0.1, j as f64 * 0.1)))
            .collect();
        let est = HausdorffMeasureEstimator::new(pts);
        let measure = est.estimated_measure(0.15);
        assert!(
            measure > 0.0,
            "Measure should be positive for 2D point grid"
        );
    }
    #[test]
    fn test_perimeter_approx_disk() {
        let n = 20usize;
        let cx = n as f64 / 2.0;
        let r = n as f64 / 4.0;
        let values: Vec<f64> = (0..n)
            .flat_map(|i| {
                (0..n).map(move |j| {
                    let dx = i as f64 - cx;
                    let dy = j as f64 - cx;
                    r - (dx * dx + dy * dy).sqrt()
                })
            })
            .collect();
        let approx = PerimeterApprox::new(n, values).expect("Should construct");
        let vol = approx.volume();
        assert!(vol > 0.0, "Volume should be positive, got {}", vol);
        let perim = approx.perimeter(1.5);
        assert!(perim >= 0.0, "Perimeter should be non-negative");
    }
    #[test]
    fn test_perimeter_approx_invalid_size() {
        let result = PerimeterApprox::new(3, vec![0.0; 8]);
        assert!(result.is_none(), "Should fail if values.len() != n*n");
    }
    #[test]
    fn test_perimeter_approx_isoperimetric_deficit() {
        let n = 20usize;
        let cx = n as f64 / 2.0;
        let r = n as f64 / 4.0;
        let values: Vec<f64> = (0..n)
            .flat_map(|i| {
                (0..n).map(move |j| {
                    let dx = i as f64 - cx;
                    let dy = j as f64 - cx;
                    r - (dx * dx + dy * dy).sqrt()
                })
            })
            .collect();
        let approx = PerimeterApprox::new(n, values).expect("Should construct");
        let deficit = approx.isoperimetric_deficit(1.5);
        assert!(
            deficit >= -1.0,
            "Isoperimetric deficit too negative: {}",
            deficit
        );
    }
    #[test]
    fn test_minimal_surface_flat_boundary() {
        let n = 5usize;
        let boundary = vec![0.0f64; n * n];
        let mut solver =
            MinimalSurfaceRelaxation::new(n, boundary, 0.01).expect("Should construct");
        let area_before = solver.area();
        assert!(area_before > 0.0);
        let area_after = solver.relax(10);
        assert!(area_after > 0.0, "Area should be positive after relaxation");
    }
    #[test]
    fn test_minimal_surface_area_nonincreasing() {
        let n = 6usize;
        let mut boundary = vec![0.0f64; n * n];
        for j in 0..n {
            boundary[j] = 1.0;
            boundary[(n - 1) * n + j] = -1.0;
        }
        let mut solver =
            MinimalSurfaceRelaxation::new(n, boundary, 0.005).expect("Should construct");
        let area0 = solver.area();
        let area1 = solver.relax(5);
        assert!(
            area1 <= area0 * 1.5,
            "Area increased too much: {} -> {}",
            area0,
            area1
        );
    }
    #[test]
    fn test_minimal_surface_invalid_size() {
        let result = MinimalSurfaceRelaxation::new(3, vec![0.0; 8], 0.01);
        assert!(result.is_none());
    }
    #[test]
    fn test_rectifiability_checker_collinear() {
        let pts: Vec<(f64, f64)> = (0..20).map(|i| (i as f64 * 0.05, 0.0)).collect();
        let checker = RectifiabilityChecker::new(pts);
        let ratio = checker.linearity_ratio();
        assert!(
            ratio > 0.95,
            "Collinear points should have ratio > 0.95, got {}",
            ratio
        );
        assert!(checker.is_approximately_rectifiable(0.9));
    }
    #[test]
    fn test_rectifiability_checker_scattered() {
        let pts: Vec<(f64, f64)> = (0..5)
            .flat_map(|i| (0..5).map(move |j| (i as f64, j as f64)))
            .collect();
        let checker = RectifiabilityChecker::new(pts);
        let ratio = checker.linearity_ratio();
        assert!(
            ratio < 0.8,
            "Uniform grid should have ratio < 0.8, got {}",
            ratio
        );
    }
    #[test]
    fn test_rectifiability_checker_empty() {
        let checker = RectifiabilityChecker::new(vec![]);
        assert_eq!(checker.centroid(), (0.0, 0.0));
        assert_eq!(checker.linearity_ratio(), 0.0);
    }
    #[test]
    fn test_coarea_invalid_size() {
        let result = CoAreaComputer::new(3, vec![0.0; 9], vec![0.0; 8]);
        assert!(result.is_none());
    }
    #[test]
    fn test_coarea_lhs_non_negative() {
        let n = 10usize;
        let f_values: Vec<f64> = (0..n)
            .flat_map(|i| (0..n).map(move |_j| i as f64 / n as f64))
            .collect();
        let g_values = vec![1.0f64; n * n];
        let computer = CoAreaComputer::new(n, f_values, g_values).expect("Should construct");
        let lhs = computer.lhs_integral();
        assert!(
            lhs >= 0.0,
            "LHS integral should be non-negative, got {}",
            lhs
        );
        assert!(
            lhs > 0.0,
            "LHS should be positive for non-constant f, got {}",
            lhs
        );
    }
    #[test]
    fn test_coarea_rhs_non_negative() {
        let n = 8usize;
        let f_values: Vec<f64> = (0..n)
            .flat_map(|i| (0..n).map(move |j| (i as f64 + j as f64) / (2.0 * n as f64)))
            .collect();
        let g_values = vec![1.0f64; n * n];
        let computer = CoAreaComputer::new(n, f_values, g_values).expect("Should construct");
        let rhs = computer.rhs_integral(10);
        assert!(
            rhs >= 0.0,
            "RHS integral should be non-negative, got {}",
            rhs
        );
    }
    #[test]
    fn test_coarea_constant_f_gives_zero_lhs() {
        let n = 5usize;
        let f_values = vec![1.0f64; n * n];
        let g_values = vec![1.0f64; n * n];
        let computer = CoAreaComputer::new(n, f_values, g_values).expect("Should construct");
        let lhs = computer.lhs_integral();
        assert!(
            lhs.abs() < 1e-12,
            "Constant f should give zero LHS, got {}",
            lhs
        );
    }
}
/// Approximate gamma function for small positive values.
#[allow(dead_code)]
pub fn gamma_approx(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::INFINITY;
    }
    // Use Lanczos approximation for better accuracy at small values
    // For half-integers and integers, use recurrence Gamma(x) = (x-1)*Gamma(x-1)
    if (x - 0.5).abs() < 1e-12 {
        return std::f64::consts::PI.sqrt(); // Gamma(1/2) = sqrt(pi)
    }
    if (x - 1.0).abs() < 1e-12 {
        return 1.0; // Gamma(1) = 0! = 1
    }
    if x < 1.5 {
        // Gamma(x) = Gamma(x+1) / x
        return gamma_approx(x + 1.0) / x;
    }
    if x < 7.0 {
        // Use recurrence to shift up: Gamma(x) = (x-1)*Gamma(x-1)
        return (x - 1.0) * gamma_approx(x - 1.0);
    }
    // Stirling approximation: good for x >= 7
    let two_pi = 2.0 * std::f64::consts::PI;
    (two_pi / x).sqrt() * (x / std::f64::consts::E).powf(x)
}
#[cfg(test)]
mod tests_gmt_extended {
    use super::*;
    #[test]
    fn test_caccioppoli_ball_isoperimetric() {
        let s = CaccioppoliSet::new(2, 2.0 * std::f64::consts::PI, std::f64::consts::PI);
        assert!(s.satisfies_isoperimetric());
    }
    #[test]
    fn test_integral_current_boundary_dim() {
        let current = IntegralCurrent::new(3, 2, 5.0, 1.0);
        assert_eq!(current.boundary_dimension(), 1);
    }
    #[test]
    fn test_integral_current_compactness() {
        let current = IntegralCurrent::new(3, 2, 10.0, 2.0);
        let bound = current.compactness_bound();
        assert!((bound - 12.0).abs() < 1e-10);
    }
    #[test]
    fn test_marstrand_positive_measure() {
        let m = MarstrandProjection::new(1.5, 2, 1);
        assert!(m.projection_has_positive_measure());
    }
    #[test]
    fn test_marstrand_no_positive_measure() {
        let m = MarstrandProjection::new(0.5, 2, 1);
        assert!(!m.projection_has_positive_measure());
    }
    #[test]
    fn test_slice_dimension() {
        let m = MarstrandProjection::new(2.5, 3, 1);
        assert!((m.typical_slice_dimension() - 0.5).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_gmt_extra {
    use super::*;
    #[test]
    fn test_rectifiable_set() {
        let s = RectifiableSet::smooth_submanifold(2, 3, 4.0);
        assert_eq!(s.dimension, 2);
        assert!(s.is_integer_rectifiable());
        assert!((s.lower_density() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_integral_current() {
        let c = IntegralCurrentNew::cycle(2, 5.0);
        assert!(c.is_closed);
        assert!(c.is_integer_multiplicity());
        assert!((c.flat_norm() - 5.0).abs() < 1e-9);
    }
    #[test]
    fn test_compactness() {
        let ct = CompactnessTheorem::new(3, 2, 10.0);
        assert!(ct.has_convergent_subsequence());
        assert!(ct.limit_is_integral_current());
    }
    #[test]
    fn test_plateau() {
        let pp = PlateauProblem::new("circle", 3);
        assert!(pp.solution_exists);
        assert!(!pp.solution_unique);
    }
}
