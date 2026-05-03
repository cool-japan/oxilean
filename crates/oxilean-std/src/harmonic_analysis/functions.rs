//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BMOData, CalderonZygmundDecomp, CalderonZygmundOperator, Convolution, FourierMultiplierOp,
    FourierRestrictionData, FourierTransform, HardySpace, HardySpaceAtom, LittlewoodPaleySquare,
    MaximalFunctionData, MultilinearCZData, OscillatoryIntegralData,
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
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `SchwartzSpace : (n : Nat) → Type`
///
/// The Schwartz space S(ℝⁿ): smooth rapidly decaying functions with all
/// derivatives decaying faster than any polynomial.
pub fn schwartz_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TemperedDistribution : (n : Nat) → Type`
///
/// The space of tempered distributions S'(ℝⁿ): continuous linear functionals on S(ℝⁿ).
pub fn tempered_distribution_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LpSpace : (p : Real) → (n : Nat) → Type`
///
/// The Lebesgue space Lᵖ(ℝⁿ): functions with finite p-th power integral.
pub fn lp_space_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `WeakLpSpace : (p : Real) → (n : Nat) → Type`
///
/// The weak Lᵖ space (Lorentz space L^{p,∞}): functions f with
/// |{|f| > λ}| ≤ (C/λ)^p for all λ > 0.
pub fn weak_lp_space_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `TorusSpace : (n : Nat) → Type`
///
/// The n-dimensional torus 𝕋ⁿ = (ℝ/ℤ)ⁿ as the domain for Fourier series.
pub fn torus_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FourierCoefficient : (n : Nat) → L²(𝕋ⁿ) → ℤⁿ → ℂ`
///
/// The k-th Fourier coefficient f̂(k) = ∫_{𝕋ⁿ} f(x) e^{-2πi⟨k,x⟩} dx.
pub fn fourier_coefficient_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("L2Space"), bvar(0)),
            arrow(list_ty(int_ty()), complex_ty()),
        ),
    )
}
/// `FourierTransform : (n : Nat) → SchwartzSpace n → SchwartzSpace n`
///
/// The Fourier transform F: S(ℝⁿ) → S(ℝⁿ), F\[f\](ξ) = ∫ f(x) e^{-2πi⟨x,ξ⟩} dx.
pub fn fourier_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("SchwartzSpace"), bvar(0)),
            app(cst("SchwartzSpace"), bvar(1)),
        ),
    )
}
/// `InverseFourierTransform : (n : Nat) → SchwartzSpace n → SchwartzSpace n`
///
/// The inverse Fourier transform F⁻¹\[g\](x) = ∫ g(ξ) e^{2πi⟨x,ξ⟩} dξ.
pub fn inverse_fourier_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("SchwartzSpace"), bvar(0)),
            app(cst("SchwartzSpace"), bvar(1)),
        ),
    )
}
/// `PlancherelTheorem : (n : Nat) → Prop`
///
/// Plancherel's theorem: ‖F\[f\]‖_{L²} = ‖f‖_{L²} for all f ∈ L²(ℝⁿ).
/// The Fourier transform extends to a unitary operator on L²(ℝⁿ).
pub fn plancherel_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("LpSpace"), real_ty(), bvar(0)), prop()),
    )
}
/// `FourierInversionTheorem : (n : Nat) → Prop`
///
/// Fourier inversion: F⁻¹ ∘ F = id on S(ℝⁿ), and in L² sense.
pub fn fourier_inversion_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ConvolutionTheorem : (n : Nat) → Prop`
///
/// Convolution theorem: F\[f * g\] = F\[f\] · F\[g\].
pub fn convolution_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RieszThorinInterpolation : Prop`
///
/// Riesz-Thorin interpolation theorem: if T : Lᵖ⁰ → Lq⁰ with norm M₀
/// and T : Lᵖ¹ → Lq¹ with norm M₁, then T : Lᵖᵗ → Lqᵗ with norm M₀^{1-t} M₁^t,
/// where 1/pₜ = (1-t)/p₀ + t/p₁.
pub fn riesz_thorin_interpolation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p0",
        real_ty(),
        pi(
            BinderInfo::Default,
            "p1",
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `MarcinkiewiczInterpolation : Prop`
///
/// Marcinkiewicz interpolation: if T is weak (p₀, p₀) and weak (p₁, p₁),
/// then T is strong (p, p) for p₀ < p < p₁.
pub fn marcinkiewicz_interpolation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p0",
        real_ty(),
        arrow(real_ty(), prop()),
    )
}
/// `BoundedOperator : (p q n : Real) → Type`
///
/// A bounded linear operator T : Lᵖ(ℝⁿ) → Lq(ℝⁿ).
pub fn bounded_operator_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), type0())))
}
/// `WeakTypeBound : (p q : Real) → BoundedOperator p q n → Prop`
///
/// Weak type (p, q) bound: |{|Tf| > λ}| ≤ (‖f‖_p / λ)^q.
pub fn weak_type_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(type0(), prop())))
}
/// `HilbertTransform : (n : Nat) → Prop`
///
/// The Hilbert transform: Hf(x) = p.v. ∫ f(y)/(x-y) dy, bounded on Lᵖ for 1 < p < ∞.
pub fn hilbert_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("LpSpace"), real_ty(), bvar(0)), prop()),
    )
}
/// `RieszTransform : (j n : Nat) → Type`
///
/// The j-th Riesz transform: Rⱼf(x) = c_n p.v. ∫ (xⱼ - yⱼ)/|x-y|^{n+1} f(y) dy.
/// Generalizes the Hilbert transform to higher dimensions.
pub fn riesz_transform_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CZKernel : (n : Nat) → Type`
///
/// A Calderón-Zygmund kernel K(x, y) on ℝⁿ: a function satisfying
/// size condition |K(x,y)| ≤ C/|x-y|^n and smoothness conditions.
pub fn cz_kernel_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CZOperator : (n : Nat) → CZKernel n → Type`
///
/// A Calderón-Zygmund operator: Tf(x) = p.v. ∫ K(x,y) f(y) dy.
pub fn cz_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CZKernel"), bvar(0)), type0()),
    )
}
/// `CZTheorem : (n : Nat) → CZOperator n → Prop`
///
/// Calderón-Zygmund theorem: CZ operators are bounded on Lᵖ for 1 < p < ∞,
/// weak (1,1), and map L∞ into BMO.
pub fn cz_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CZOperator"), bvar(0)), prop()),
    )
}
/// `T1Theorem : (n : Nat) → Prop`
///
/// David-Journé T(1) theorem: a singular integral operator with standard kernel
/// is bounded on L² iff T(1) ∈ BMO, T*(1) ∈ BMO, and the weak boundedness holds.
pub fn t1_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HardyLittlewoodMaximal : (n : Nat) → Lp n → Lp n`
///
/// The Hardy-Littlewood maximal function: Mf(x) = sup_{r>0} (1/|B(x,r)|) ∫_{B(x,r)} |f(y)| dy.
pub fn hardy_littlewood_maximal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app2(cst("LpSpace"), real_ty(), bvar(0)),
            app2(cst("LpSpace"), real_ty(), bvar(1)),
        ),
    )
}
/// `MaximalFunctionWeakBound : (n : Nat) → Prop`
///
/// Weak (1,1) bound for Hardy-Littlewood maximal function:
/// |{Mf > λ}| ≤ (C_n / λ) ‖f‖_{L¹}.
pub fn maximal_function_weak_bound_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MaximalFunctionStrongBound : (n p : Nat) → Prop`
///
/// Strong (p, p) bound: ‖Mf‖_{Lᵖ} ≤ C_{n,p} ‖f‖_{Lᵖ} for 1 < p ≤ ∞.
pub fn maximal_function_strong_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `VitaliCovering : (n : Nat) → Prop`
///
/// Vitali covering lemma: from any collection of balls, extract a subcollection
/// of disjoint balls covering (1/5 of) the original union.
pub fn vitali_covering_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BMOSpace : (n : Nat) → Type`
///
/// The space of functions of bounded mean oscillation:
/// ‖f‖_{BMO} = sup_Q (1/|Q|) ∫_Q |f - f_Q| dx < ∞.
pub fn bmo_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BMONorm : (n : Nat) → BMOSpace n → Real`
///
/// The BMO seminorm ‖f‖_{BMO}.
pub fn bmo_norm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("BMOSpace"), bvar(0)), real_ty()),
    )
}
/// `JohnNirenbergInequality : (n : Nat) → Prop`
///
/// John-Nirenberg inequality: for f ∈ BMO, the level sets have exponential decay:
/// |{x ∈ Q : |f(x) - f_Q| > λ}| ≤ C|Q| exp(-c λ / ‖f‖_{BMO}).
pub fn john_nirenberg_inequality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BMOCharacterization : (n : Nat) → Prop`
///
/// Fefferman-Stein characterization: BMO = (H¹)* (dual of Hardy space).
pub fn bmo_characterization_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HardySpace : (p : Real) → (n : Nat) → Type`
///
/// The Hardy space Hᵖ(ℝⁿ): defined via maximal functions or atomic decomposition.
/// For p=1: H¹ = {f ∈ L¹ : Mf ∈ L¹} where Mf is the grand maximal function.
pub fn hardy_space_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `AtomicDecomposition : (n : Nat) → H¹ n → Prop`
///
/// Atomic decomposition of H¹: every f ∈ H¹ decomposes as f = Σ λⱼ aⱼ
/// where aⱼ are atoms (supported on cubes, zero mean, L∞ bound).
pub fn atomic_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("HardySpace"), real_ty(), bvar(0)), prop()),
    )
}
/// `FeffermanSteinDuality : (n : Nat) → Prop`
///
/// Fefferman-Stein H¹-BMO duality: (H¹(ℝⁿ))* = BMO(ℝⁿ) as Banach spaces.
pub fn fefferman_stein_duality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HardySpaceLpEmbedding : (p n : Nat) → Prop`
///
/// For p > 1: Hᵖ = Lᵖ as sets with equivalent norms.
pub fn hardy_space_lp_embedding_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `LittlewoodPaleyDecomposition : (n : Nat) → SchwartzSpace n → Type`
///
/// Littlewood-Paley decomposition: f = Σⱼ Δⱼf where Δⱼ is the j-th Paley block
/// (frequency localisation to annulus {2ʲ ≤ |ξ| < 2^{j+1}}).
pub fn littlewood_paley_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("SchwartzSpace"), bvar(0)), type0()),
    )
}
/// `SquareFunction : (n : Nat) → LpSpace p n → Real`
///
/// The Littlewood-Paley square function: S(f)(x) = (Σⱼ |Δⱼf(x)|²)^{1/2}.
pub fn square_function_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("LpSpace"), real_ty(), bvar(0)), real_ty()),
    )
}
/// `LittlewoodPaleyInequality : (p n : Nat) → Prop`
///
/// Littlewood-Paley inequality: ‖f‖_{Lᵖ} ≈ ‖S(f)‖_{Lᵖ} for 1 < p < ∞.
pub fn littlewood_paley_inequality_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `PaleyBlock : (j n : Nat) → Type`
///
/// A Paley frequency block Δⱼ: the Fourier multiplier with symbol φ(2^{-j} ξ)
/// where φ is a bump function supported on an annulus.
pub fn paley_block_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BesovSpace : (s p q n : Real) → Type`
///
/// The Besov space B^s_{p,q}(ℝⁿ): characterised via LP decomposition as
/// {f : ‖{2^{sj} ‖Δⱼf‖_{Lᵖ}}‖_{ℓq} < ∞}.
pub fn besov_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        real_ty(),
        pi(
            BinderInfo::Default,
            "p",
            real_ty(),
            pi(
                BinderInfo::Default,
                "q",
                real_ty(),
                arrow(nat_ty(), type0()),
            ),
        ),
    )
}
/// `TriebelLizorkinSpace : (s p q n : Real) → Type`
///
/// The Triebel-Lizorkin space F^s_{p,q}(ℝⁿ): Sobolev and Hardy spaces are
/// special cases.
pub fn triebel_lizorkin_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        real_ty(),
        pi(
            BinderInfo::Default,
            "p",
            real_ty(),
            pi(
                BinderInfo::Default,
                "q",
                real_ty(),
                arrow(nat_ty(), type0()),
            ),
        ),
    )
}
/// `SymbolClass : (m rho delta : Real) → (n : Nat) → Type`
///
/// Symbol class Sᵐ_{ρ,δ}(ℝⁿ): smooth functions a(x, ξ) satisfying
/// |∂^α_ξ ∂^β_x a(x, ξ)| ≤ C_{α,β} (1 + |ξ|)^{m - ρ|α| + δ|β|}.
pub fn symbol_class_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        real_ty(),
        pi(
            BinderInfo::Default,
            "rho",
            real_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                arrow(nat_ty(), type0()),
            ),
        ),
    )
}
/// `PseudodiffOperator : (m rho delta n : Real) → SymbolClass m rho delta n → Type`
///
/// A pseudodifferential operator Op(a) with symbol a ∈ Sᵐ_{ρ,δ}:
/// Op(a)f(x) = (2π)^{-n} ∫ e^{i⟨x,ξ⟩} a(x, ξ) f̂(ξ) dξ.
#[allow(clippy::too_many_arguments)]
pub fn pseudodiff_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        real_ty(),
        pi(
            BinderInfo::Default,
            "rho",
            real_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                arrow(nat_ty(), type0()),
            ),
        ),
    )
}
/// `PseudodiffLpBound : (m p n : Real) → Prop`
///
/// Lᵖ continuity: Op(a) with a ∈ S⁰_{1,0} is bounded on Lᵖ for 1 < p < ∞.
pub fn pseudodiff_lp_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `Microlocalization : (n : Nat) → Type`
///
/// Microlocalization / wave front set: describes singularities of distributions
/// in both position and frequency (cotangent bundle T*ℝⁿ).
pub fn microlocalization_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EllipticRegularity : (m n : Real) → Prop`
///
/// Elliptic regularity: if P ∈ Op Sᵐ_{1,0} is elliptic and Pu ∈ Hˢ, then u ∈ H^{s+m}.
pub fn elliptic_regularity_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// H^p space atomic: HpAtom (p : Real) (n : Nat) : Type
/// An Hᵖ-atom: a function supported on a cube Q with zero mean and L∞ ≤ |Q|^{-1/p}.
pub fn hp_atom_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// H^p Riesz factorization: HpRieszFactorization (p q r : Real) (n : Nat) : Prop
/// For p=1: H¹ = L² · L² (Riesz factorization).
pub fn hp_riesz_factorization_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// VMO space: VMOSpace (n : Nat) : Type
/// The space of functions of vanishing mean oscillation: VMO = closure of C_c in BMO.
pub fn vmo_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// VMO characterization: VMOCharacterization (n : Nat) : Prop
/// VMO = (H¹)_0, the predual of H¹ restricted to compact support.
pub fn vmo_characterization_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Calderon-Zygmund decomposition: CZDecomposition (n : Nat) (f : L¹ n) : Prop
/// For f ∈ L¹, α > 0: decompose as f = g + b where g ∈ L∞, b supported on cubes.
pub fn cz_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app2(cst("LpSpace"), real_ty(), bvar(0)), prop()),
    )
}
/// Stopping time argument: StoppingTimeArgument (n : Nat) : Prop
/// The stopping time (first exit) argument used in martingale / CZ proofs.
pub fn stopping_time_argument_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Mihlin multiplier theorem: MihlinMultiplierTheorem (n : Nat) : Prop
/// A Fourier multiplier m(ξ) satisfying |∂^α m(ξ)| ≤ C_α |ξ|^{-|α|} (|α| ≤ n+1)
/// defines an Lᵖ-bounded operator for 1 < p < ∞.
pub fn mihlin_multiplier_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Hormander multiplier theorem: HormanderMultiplierTheorem (n : Nat) : Prop
/// A weaker Sobolev condition on the symbol (Hörmander's version of Mihlin).
pub fn hormander_multiplier_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Paraproduct: Paraproduct (n : Nat) : Type
/// The Bony paraproduct Π(f, g) = Σⱼ Δⱼf · Sⱼg decomposing bilinear forms
/// into paraproduct, remainder, and conjugate paraproduct.
pub fn paraproduct_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Paraproduct boundedness: ParaproductBoundedness (n : Nat) : Prop
pub fn paraproduct_boundedness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Muckenhoupt A_p weight: MuckenhouptApWeight (p n : Real) : Prop
/// A non-negative function w satisfying the Muckenhoupt Aₚ condition:
/// sup_Q (1/|Q| ∫_Q w) · (1/|Q| ∫_Q w^{-1/(p-1)})^{p-1} < ∞.
pub fn muckenhoupt_ap_weight_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// Weighted Lp inequality: WeightedLpInequality (p n : Real) : Prop
/// The Hardy-Littlewood maximal function M is bounded on Lᵖ(w) iff w ∈ Aₚ.
pub fn weighted_lp_inequality_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// Two-weight inequality: TwoWeightInequality (T : Operator) (n : Nat) : Prop
/// A singular integral T is bounded from Lᵖ(u) to Lᵖ(v) under the bump condition.
pub fn two_weight_inequality_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// Sobolev space with weight: SobolevSpaceWeighted (s p n : Real) : Type
pub fn sobolev_space_weighted_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        real_ty(),
        arrow(real_ty(), arrow(nat_ty(), type0())),
    )
}
/// Compact group Fourier transform: CompactGroupFourier (G : CompactGroup) : Type
/// Generalised Fourier transform using Peter-Weyl theory on compact groups.
pub fn compact_group_fourier_ty() -> Expr {
    arrow(type0(), type0())
}
/// Nilpotent group Fourier: NilpotentGroupFourier (G : NilpotentGroup) : Type
pub fn nilpotent_group_fourier_ty() -> Expr {
    arrow(type0(), type0())
}
/// Sub-Riemannian Laplacian: SubRiemannianLaplacian (M : SubRiemannianManifold) : Type
/// The horizontal Laplacian (sum of squares of vector fields satisfying Hormander condition).
pub fn sub_riemannian_laplacian_ty() -> Expr {
    arrow(type0(), type0())
}
/// Stationary phase expansion: StationaryPhaseExpansion (k : Nat) : Prop
/// ∫ e^{iλφ(x)} a(x) dx = e^{iλφ(x₀)} (2π/λ)^{n/2} |det φ''(x₀)|^{-1/2} (a(x₀) + O(1/λ)).
pub fn stationary_phase_expansion_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Van der Corput lemma: VanDerCorputLemma (k : Nat) : Prop
/// |∫_a^b e^{iλφ(x)} dx| ≤ C_k λ^{-1/k} when |φ^{(k)}| ≥ 1.
pub fn van_der_corput_lemma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Stein-Tomas restriction theorem: SteinTomasRestriction (n : Nat) : Prop
/// The Fourier transform restricts to the sphere S^{n-1} as a bounded map
/// L^p(ℝⁿ) → L²(S^{n-1}) for p ≤ 2(n+1)/(n+3).
pub fn stein_tomas_restriction_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Fourier restriction problem: FourierRestriction (n : Nat) (S : HypersurfaceType) : Prop
/// The Fourier restriction conjecture: Rf : L^p(ℝⁿ) → L^q(S) for which (p,q)?
pub fn fourier_restriction_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// Weyl law: WeylLaw (M : RiemannianManifold) : Prop
/// #{λⱼ ≤ Λ} ~ (vol M) (4π)^{-n/2} Γ(n/2+1)^{-1} Λ^{n/2} as Λ → ∞.
pub fn weyl_law_ty() -> Expr {
    arrow(type0(), prop())
}
/// Heat kernel: HeatKernel (M : RiemannianManifold) : Type
/// The fundamental solution p_t(x, y) of the heat equation ∂_t u = Δ u.
pub fn heat_kernel_ty() -> Expr {
    arrow(type0(), type0())
}
/// Lichnerowicz inequality: LichnerowiczInequality (M : RiemannianManifold) : Prop
/// If Ric ≥ κ > 0 then λ₁(M) ≥ n/(n-1) · κ.
pub fn lichnerowicz_inequality_ty() -> Expr {
    arrow(type0(), prop())
}
/// Gabor frame: GaborFrame (d : Nat) : Type
/// A Gabor system {g_{m,n}} = {e^{2πi m α x} g(x - n β)} forming a frame in L²(ℝᵈ).
pub fn gabor_frame_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Wavelet frame: WaveletFrame (d : Nat) : Type
/// A wavelet system {ψ_{j,k}} forming a frame in L²(ℝᵈ).
pub fn wavelet_frame_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Balian-Low theorem: BalianLowTheorem : Prop
/// A Gabor system {e^{2πi m b x} g(x - n a)} cannot be an orthonormal basis
/// if both ∫|x g(x)|² dx < ∞ and ∫|ξ ĝ(ξ)|² dξ < ∞.
pub fn balian_low_theorem_ty() -> Expr {
    prop()
}
/// Frame bounds: FrameBounds (d : Nat) : Prop
/// A frame satisfies A‖f‖² ≤ Σ|⟨f, ψₙ⟩|² ≤ B‖f‖² with 0 < A ≤ B < ∞.
pub fn frame_bounds_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Uncertainty principle: UncertaintyPrinciple (n : Nat) : Prop
/// Heisenberg uncertainty: ‖xf‖ · ‖ξf̂‖ ≥ (n/4π) ‖f‖².
pub fn uncertainty_principle_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Hardy space on group: HardySpaceOnGroup (G : LieGroup) : Type
/// The real Hardy space Hᵖ(G) defined via group convolution with approximate identity.
pub fn hardy_space_on_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// Build the harmonic analysis kernel environment with all axiom declarations.
pub fn build_harmonic_analysis_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("SchwartzSpace", schwartz_space_ty()),
        ("TemperedDistribution", tempered_distribution_ty()),
        ("LpSpace", lp_space_ty()),
        ("WeakLpSpace", weak_lp_space_ty()),
        ("TorusSpace", torus_space_ty()),
        ("L2Space", arrow(nat_ty(), type0())),
        ("FourierCoefficient", fourier_coefficient_ty()),
        ("FourierTransform", fourier_transform_ty()),
        ("InverseFourierTransform", inverse_fourier_transform_ty()),
        ("PlancherelTheorem", plancherel_theorem_ty()),
        ("FourierInversionTheorem", fourier_inversion_theorem_ty()),
        ("ConvolutionTheorem", convolution_theorem_ty()),
        ("RieszThorinInterpolation", riesz_thorin_interpolation_ty()),
        (
            "MarcinkiewiczInterpolation",
            marcinkiewicz_interpolation_ty(),
        ),
        ("BoundedOperator", bounded_operator_ty()),
        ("WeakTypeBound", weak_type_bound_ty()),
        ("HilbertTransform", hilbert_transform_ty()),
        ("RieszTransform", riesz_transform_ty()),
        ("CZKernel", cz_kernel_ty()),
        ("CZOperator", cz_operator_ty()),
        ("CZTheorem", cz_theorem_ty()),
        ("T1Theorem", t1_theorem_ty()),
        ("HardyLittlewoodMaximal", hardy_littlewood_maximal_ty()),
        ("MaximalFunctionWeakBound", maximal_function_weak_bound_ty()),
        (
            "MaximalFunctionStrongBound",
            maximal_function_strong_bound_ty(),
        ),
        ("VitaliCovering", vitali_covering_ty()),
        ("BMOSpace", bmo_space_ty()),
        ("BMONorm", bmo_norm_ty()),
        ("JohnNirenbergInequality", john_nirenberg_inequality_ty()),
        ("BMOCharacterization", bmo_characterization_ty()),
        ("HardySpace", hardy_space_ty()),
        ("AtomicDecomposition", atomic_decomposition_ty()),
        ("FeffermanSteinDuality", fefferman_stein_duality_ty()),
        ("HardySpaceLpEmbedding", hardy_space_lp_embedding_ty()),
        (
            "LittlewoodPaleyDecomposition",
            littlewood_paley_decomposition_ty(),
        ),
        ("SquareFunction", square_function_ty()),
        (
            "LittlewoodPaleyInequality",
            littlewood_paley_inequality_ty(),
        ),
        ("PaleyBlock", paley_block_ty()),
        ("BesovSpace", besov_space_ty()),
        ("TriebelLizorkinSpace", triebel_lizorkin_space_ty()),
        ("SymbolClass", symbol_class_ty()),
        ("PseudodiffOperator", pseudodiff_operator_ty()),
        ("PseudodiffLpBound", pseudodiff_lp_bound_ty()),
        ("Microlocalization", microlocalization_ty()),
        ("EllipticRegularity", elliptic_regularity_ty()),
        ("IsElliptic", arrow(type0(), prop())),
        ("IsHypoelliptic", arrow(type0(), prop())),
        ("IsSelfAdjoint", arrow(type0(), prop())),
        ("IsUnitary", arrow(type0(), prop())),
        ("HasBoundedExtension", arrow(type0(), prop())),
        ("IsWeakType11", arrow(type0(), prop())),
        (
            "FourierMultiplier",
            arrow(arrow(nat_ty(), complex_ty()), type0()),
        ),
        ("ConvolutionKernel", arrow(nat_ty(), type0())),
        (
            "PrincipalValueIntegral",
            arrow(arrow(real_ty(), real_ty()), real_ty()),
        ),
        (
            "OscillatoryIntegral",
            arrow(nat_ty(), arrow(nat_ty(), real_ty())),
        ),
        (
            "StationaryPhaseMethod",
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
        ("SobolevSpace", arrow(real_ty(), arrow(nat_ty(), type0()))),
        (
            "SobolevEmbedding",
            arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop()))),
        ),
        ("WaveFrontSet", arrow(type0(), arrow(nat_ty(), type0()))),
        ("HpAtom", hp_atom_ty()),
        ("HpRieszFactorization", hp_riesz_factorization_ty()),
        ("VMOSpace", vmo_space_ty()),
        ("VMOCharacterization", vmo_characterization_ty()),
        ("CZDecomposition", cz_decomposition_ty()),
        ("StoppingTimeArgument", stopping_time_argument_ty()),
        ("MihlinMultiplierTheorem", mihlin_multiplier_theorem_ty()),
        (
            "HormanderMultiplierTheorem",
            hormander_multiplier_theorem_ty(),
        ),
        ("Paraproduct", paraproduct_ty()),
        ("ParaproductBoundedness", paraproduct_boundedness_ty()),
        ("MuckenhouptApWeight", muckenhoupt_ap_weight_ty()),
        ("WeightedLpInequality", weighted_lp_inequality_ty()),
        ("TwoWeightInequality", two_weight_inequality_ty()),
        ("SobolevSpaceWeighted", sobolev_space_weighted_ty()),
        ("CompactGroupFourier", compact_group_fourier_ty()),
        ("NilpotentGroupFourier", nilpotent_group_fourier_ty()),
        ("SubRiemannianLaplacian", sub_riemannian_laplacian_ty()),
        ("StationaryPhaseExpansion", stationary_phase_expansion_ty()),
        ("VanDerCorputLemma", van_der_corput_lemma_ty()),
        ("SteinTomasRestriction", stein_tomas_restriction_ty()),
        ("FourierRestriction", fourier_restriction_ty()),
        ("WeylLaw", weyl_law_ty()),
        ("HeatKernel", heat_kernel_ty()),
        ("LichnerowiczInequality", lichnerowicz_inequality_ty()),
        ("GaborFrame", gabor_frame_ty()),
        ("WaveletFrame", wavelet_frame_ty()),
        ("BalianLowTheorem", balian_low_theorem_ty()),
        ("FrameBounds", frame_bounds_ty()),
        ("UncertaintyPrinciple", uncertainty_principle_ty()),
        ("HardySpaceOnGroup", hardy_space_on_group_ty()),
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
/// Compute the N-point Discrete Fourier Transform of a real-valued sequence.
///
/// DFT\[k\] = Σ_{n=0}^{N-1} x\[n\] e^{-2πi nk/N}
/// Returns (real part, imaginary part) pairs.
pub fn dft(signal: &[f64]) -> Vec<(f64, f64)> {
    let n = signal.len();
    if n == 0 {
        return vec![];
    }
    let two_pi_over_n = 2.0 * std::f64::consts::PI / n as f64;
    (0..n)
        .map(|k| {
            let (re, im) = signal
                .iter()
                .enumerate()
                .fold((0.0, 0.0), |(re, im), (j, &x)| {
                    let angle = two_pi_over_n * (k * j) as f64;
                    (re + x * angle.cos(), im - x * angle.sin())
                });
            (re, im)
        })
        .collect()
}
/// Compute the inverse DFT.
pub fn idft(spectrum: &[(f64, f64)]) -> Vec<f64> {
    let n = spectrum.len();
    if n == 0 {
        return vec![];
    }
    let two_pi_over_n = 2.0 * std::f64::consts::PI / n as f64;
    let n_f = n as f64;
    (0..n)
        .map(|j| {
            spectrum
                .iter()
                .enumerate()
                .map(|(k, &(re, im))| {
                    let angle = two_pi_over_n * (k * j) as f64;
                    (re * angle.cos() - im * angle.sin()) / n_f
                })
                .sum()
        })
        .collect()
}
/// Compute the L² norm of a DFT spectrum (for verifying Plancherel).
pub fn dft_l2_norm_squared(spectrum: &[(f64, f64)]) -> f64 {
    spectrum
        .iter()
        .map(|(re, im)| re * re + im * im)
        .sum::<f64>()
        / spectrum.len() as f64
}
/// Compute the L² norm squared of the original signal.
pub fn signal_l2_norm_squared(signal: &[f64]) -> f64 {
    signal.iter().map(|&x| x * x).sum()
}
/// Discrete Hardy-Littlewood maximal function on a 1D signal.
///
/// M\[f\](i) = max_{r ≥ 0} (1/(2r+1)) Σ_{j=i-r}^{i+r} |f(j)|
/// (with zero-boundary conditions outside \[0, n)).
pub fn discrete_maximal_function(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n == 0 {
        return vec![];
    }
    (0..n)
        .map(|i| {
            let mut best = signal[i].abs();
            for r in 1..=n {
                let lo = i.saturating_sub(r);
                let hi = (i + r).min(n - 1);
                let window: f64 = (lo..=hi).map(|j| signal[j].abs()).sum();
                let avg = window / (hi - lo + 1) as f64;
                if avg > best {
                    best = avg;
                }
                if lo == 0 && hi == n - 1 {
                    break;
                }
            }
            best
        })
        .collect()
}
/// Compute the discrete BMO seminorm of a signal.
///
/// ‖f‖_{BMO} = sup_I (1/|I|) Σ_{i ∈ I} |f(i) - f_I|
/// where f_I = mean of f on interval I, and the sup is over all sub-intervals I.
pub fn discrete_bmo_seminorm(signal: &[f64]) -> f64 {
    let n = signal.len();
    if n <= 1 {
        return 0.0;
    }
    let mut best = 0.0_f64;
    for lo in 0..n {
        for hi in lo..n {
            let len = (hi - lo + 1) as f64;
            let mean: f64 = (lo..=hi).map(|i| signal[i]).sum::<f64>() / len;
            let osc: f64 = (lo..=hi).map(|i| (signal[i] - mean).abs()).sum::<f64>() / len;
            if osc > best {
                best = osc;
            }
        }
    }
    best
}
/// Computes the interpolated operator norm at exponent t ∈ \[0,1\] via Riesz-Thorin.
///
/// Given T : Lᵖ⁰ → Lq⁰ with norm M₀ and T : Lᵖ¹ → Lq¹ with norm M₁,
/// the interpolated norm at parameter t is M₀^{1-t} M₁^t.
pub fn riesz_thorin_bound(m0: f64, m1: f64, t: f64) -> f64 {
    m0.powf(1.0 - t) * m1.powf(t)
}
/// Computes the interpolated exponent 1/pₜ = (1-t)/p₀ + t/p₁.
pub fn interpolated_exponent(p0: f64, p1: f64, t: f64) -> f64 {
    let inv_p = (1.0 - t) / p0 + t / p1;
    1.0 / inv_p
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dft_constant_signal() {
        let signal = vec![1.0, 1.0, 1.0, 1.0];
        let spectrum = dft(&signal);
        assert!(
            (spectrum[0].0 - 4.0).abs() < 1e-10,
            "DFT[0] re = {}",
            spectrum[0].0
        );
        assert!(spectrum[0].1.abs() < 1e-10, "DFT[0] im = {}", spectrum[0].1);
        for k in 1..4 {
            let mag = (spectrum[k].0 * spectrum[k].0 + spectrum[k].1 * spectrum[k].1).sqrt();
            assert!(mag < 1e-10, "DFT[{}] magnitude = {}", k, mag);
        }
    }
    #[test]
    fn test_plancherel_dft() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let spectrum = dft(&signal);
        let signal_norm_sq = signal_l2_norm_squared(&signal);
        let spectrum_norm_sq = dft_l2_norm_squared(&spectrum);
        assert!(
            (signal_norm_sq - spectrum_norm_sq).abs() < 1e-8,
            "Plancherel: signal_norm² = {}, spectrum_norm² = {}",
            signal_norm_sq,
            spectrum_norm_sq
        );
    }
    #[test]
    fn test_idft_roundtrip() {
        let signal = vec![1.0, -1.0, 2.0, 0.5];
        let spectrum = dft(&signal);
        let recovered = idft(&spectrum);
        for (i, (&orig, rec)) in signal.iter().zip(recovered.iter()).enumerate() {
            assert!(
                (orig - rec).abs() < 1e-10,
                "roundtrip mismatch at {}: {} vs {}",
                i,
                orig,
                rec
            );
        }
    }
    #[test]
    fn test_discrete_maximal_function_monotone_dominates() {
        let signal = vec![1.0, -2.0, 3.0, -1.0, 0.5];
        let mf = discrete_maximal_function(&signal);
        for (i, (&x, &mx)) in signal.iter().zip(mf.iter()).enumerate() {
            assert!(
                mx >= x.abs() - 1e-12,
                "M[f][{}] = {} < |f[{}]| = {}",
                i,
                mx,
                i,
                x.abs()
            );
        }
    }
    #[test]
    fn test_bmo_seminorm_constant() {
        let signal = vec![5.0; 8];
        let bmo = discrete_bmo_seminorm(&signal);
        assert!(bmo.abs() < 1e-10, "BMO of constant = {}", bmo);
    }
    #[test]
    fn test_bmo_seminorm_step() {
        let signal = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let bmo = discrete_bmo_seminorm(&signal);
        assert!(
            bmo > 0.0,
            "BMO of step function should be positive, got {}",
            bmo
        );
    }
    #[test]
    fn test_riesz_thorin_bounds() {
        let m0 = 2.0_f64;
        let m1 = 8.0_f64;
        assert!((riesz_thorin_bound(m0, m1, 0.0) - m0).abs() < 1e-10);
        assert!((riesz_thorin_bound(m0, m1, 1.0) - m1).abs() < 1e-10);
        let expected_half = (m0 * m1).sqrt();
        assert!((riesz_thorin_bound(m0, m1, 0.5) - expected_half).abs() < 1e-10);
    }
    #[test]
    fn test_build_harmonic_analysis_env() {
        let mut env = Environment::new();
        build_harmonic_analysis_env(&mut env);
        assert!(!env.is_empty());
    }
    #[test]
    fn test_hardy_space_atom_canonical() {
        let atom = HardySpaceAtom::canonical(8, 0, 7);
        assert!(atom.is_valid_atom(), "canonical atom should be valid");
    }
    #[test]
    fn test_hardy_space_atom_zero_mean() {
        let values = vec![0.25, 0.25, -0.25, -0.25];
        let atom = HardySpaceAtom::new(0, 3, values);
        assert!(atom.has_zero_mean());
        assert!(atom.satisfies_linfty_bound());
        assert!(atom.has_compact_support());
        assert!(atom.is_valid_atom());
    }
    #[test]
    fn test_hardy_space_atom_not_valid_nonzero_mean() {
        let values = vec![0.5, 0.5];
        let atom = HardySpaceAtom::new(0, 1, values);
        assert!(!atom.has_zero_mean());
        assert!(!atom.is_valid_atom());
    }
    #[test]
    fn test_cz_decomposition_basic() {
        let signal = vec![1.0, 1.0, 5.0, 1.0, 1.0];
        let cz = CalderonZygmundDecomp::new(signal, 2.0);
        assert!(cz.verify_decomposition());
        assert!(cz.good_part_bounded());
    }
    #[test]
    fn test_cz_decomposition_constant() {
        let signal = vec![1.0; 8];
        let cz = CalderonZygmundDecomp::new(signal, 2.0);
        assert!(cz.verify_decomposition());
        let g = cz.good_part();
        for (gi, si) in g.iter().zip(cz.signal.iter()) {
            assert!((gi - si).abs() < 1e-12);
        }
    }
    #[test]
    fn test_fourier_multiplier_identity() {
        let n = 8;
        let multiplier = FourierMultiplierOp::new(vec![1.0; n]);
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = multiplier.apply(&signal);
        for (r, s) in result.iter().zip(signal.iter()) {
            assert!((r - s).abs() < 1e-10, "identity: {} vs {}", r, s);
        }
    }
    #[test]
    fn test_fourier_multiplier_zero() {
        let n = 4;
        let multiplier = FourierMultiplierOp::new(vec![0.0; n]);
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let result = multiplier.apply(&signal);
        for &r in &result {
            assert!(r.abs() < 1e-10, "zero multiplier: {}", r);
        }
    }
    #[test]
    fn test_fourier_multiplier_l2_norm() {
        let multiplier = FourierMultiplierOp::hilbert_multiplier(8);
        let norm = multiplier.l2_operator_norm();
        assert!((norm - 1.0).abs() < 1e-12, "Hilbert norm = {}", norm);
        assert!(multiplier.is_l2_bounded());
    }
    #[test]
    fn test_littlewood_paley_square_function_trivial() {
        let lp = LittlewoodPaleySquare::new(vec![0.0; 8]);
        let sf = lp.square_function_pointwise();
        for &v in &sf {
            assert!(v.abs() < 1e-10, "zero signal SF: {}", v);
        }
    }
    #[test]
    fn test_littlewood_paley_square_function_norms() {
        let signal = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let lp = LittlewoodPaleySquare::new(signal);
        let (sq_norm, sig_norm, ratio) = lp.verify_lp_inequality();
        assert!(
            sig_norm > 0.0,
            "signal norm should be positive, got {}",
            sig_norm
        );
        assert!(
            ratio > 0.0,
            "LP ratio should be positive: sq={} sig={} ratio={}",
            sq_norm,
            sig_norm,
            ratio
        );
    }
    #[test]
    fn test_build_env_has_new_axioms() {
        let mut env = Environment::new();
        build_harmonic_analysis_env(&mut env);
        for name in &[
            "VMOSpace",
            "HpAtom",
            "MihlinMultiplierTheorem",
            "Paraproduct",
            "MuckenhouptApWeight",
            "SteinTomasRestriction",
            "WeylLaw",
            "GaborFrame",
            "WaveletFrame",
            "BalianLowTheorem",
            "UncertaintyPrinciple",
        ] {
            assert!(
                env.contains(&Name::str(*name)),
                "Missing new axiom: {}",
                name
            );
        }
    }
}
/// Build a kernel `Environment` with harmonic-analysis axioms.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    build_harmonic_analysis_env(&mut env);
    env
}
#[cfg(test)]
mod tests_harmonic_analysis_ext {
    use super::*;
    #[test]
    fn test_cz_operator() {
        let h = CalderonZygmundOperator::hilbert_transform();
        assert!(h.l2_bounded);
        assert!(h.lp_boundedness(2.0));
        assert!(h.lp_boundedness(3.0));
        assert!(!h.lp_boundedness(1.0));
        assert!(h.weak_type_one_one());
        assert!(h.t1_theorem_condition().contains("T(1)"));
    }
    #[test]
    fn test_maximal_function() {
        let mf = MaximalFunctionData::new(1, vec![1.0, 3.0, 2.0, 4.0, 1.0]);
        let m2 = mf.hl_maximal_at(2, 1);
        assert!((m2 - 4.0).abs() < 1e-10);
        let weak = mf.weak_type_bound_approx(1.0);
        assert!((weak - 11.0).abs() < 1e-10);
        let lp = mf.lp_norm_estimate(2.0);
        assert!(lp > 0.0);
    }
    #[test]
    fn test_oscillatory_integral() {
        let oi = OscillatoryIntegralData::new("x^2", "χ", 100.0, 2);
        let decay = oi.stationary_phase_decay();
        assert!((decay - 0.01).abs() < 1e-10);
        let vdc = oi.van_der_corput_bound(2);
        assert!((vdc - 0.1).abs() < 1e-10);
    }
    #[test]
    fn test_fourier_restriction() {
        let fr = FourierRestrictionData::new("S^2", 3);
        assert!((fr.stein_tomas_p - 4.0).abs() < 1e-10);
        assert!(fr.stein_tomas_statement().contains("Stein-Tomas"));
    }
}
#[cfg(test)]
mod tests_harmonic_analysis_ext2 {
    use super::*;
    #[test]
    fn test_bmo_data() {
        let bmo = BMOData::new(vec![3.0, 3.0, 3.0]);
        assert!(bmo.bmo_seminorm < 1e-10);
        assert!(bmo.is_vmo_approx(1e-9));
        let bmo2 = BMOData::new(vec![1.0, 5.0, 1.0, 5.0]);
        assert!(bmo2.bmo_seminorm > 0.0);
        assert!(!bmo2.is_vmo_approx(0.1));
    }
    #[test]
    fn test_riesz_transform() {
        let r1 = CalderonZygmundOperator::riesz_transform(1, 3);
        assert_eq!(r1.name, "R_1");
        assert_eq!(r1.dimension, 3);
        assert!(r1.weak_type_one_one());
        assert!(r1.cotlar_stein_description().contains("R_1"));
    }
}
#[cfg(test)]
mod tests_harmonic_analysis_ext3 {
    use super::*;
    #[test]
    fn test_multilinear_cz() {
        let mcz = MultilinearCZData::new("T", 2);
        assert_eq!(mcz.m, 2);
        assert!(mcz.is_bounded);
        let r = MultilinearCZData::holder_exponent(&[2.0, 2.0]);
        assert!((r - 1.0).abs() < 1e-10);
    }
}
