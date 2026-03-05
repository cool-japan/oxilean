//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CompositionType, DpMeanEstimator, DpSgd, DpSyntheticData, ExponentialMechanism,
    GaussianMechanism, GaussianNoise, InferenceAttackModel, LaplaceMechanism, LaplaceNoise,
    LocalDpMechanism, PrivacyBudget, PrivacyLedger, RenyiAccountant, RenyiDp, ReportNoisyMax,
    ShuffleAmplification, SyntheticDataMethod, ZcdpBound,
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
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `Dataset : Type` — an abstract dataset type.
pub fn dataset_ty() -> Expr {
    type0()
}
/// `Mechanism : Dataset → List Real` — a randomised query mechanism.
/// Type: Dataset → (List Real)
pub fn mechanism_ty() -> Expr {
    arrow(cst("Dataset"), list_ty(real_ty()))
}
/// `(ε, δ)-DP : Mechanism → Real → Real → Prop`
/// The mechanism M satisfies (ε, δ)-DP if for all adjacent datasets D, D' and
/// all measurable sets S: Pr[M(D) ∈ S] ≤ exp(ε) * Pr[M(D') ∈ S] + δ
pub fn eps_delta_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(BinderInfo::Default, "delta", real_ty(), prop()),
        ),
    )
}
/// `PureDP : Mechanism → Real → Prop` — ε-DP (δ = 0)
pub fn pure_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(BinderInfo::Default, "eps", real_ty(), prop()),
    )
}
/// `RenyiDP : Mechanism → Real → Real → Prop`
/// M satisfies (α, ε)-RDP if the Rényi divergence D_α(M(D) || M(D')) ≤ ε
/// for all adjacent D, D'.
pub fn renyi_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(
            BinderInfo::Default,
            "alpha",
            real_ty(),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `LocalDP : Mechanism → Real → Prop` — local (user-level) differential privacy.
pub fn local_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(BinderInfo::Default, "eps", real_ty(), prop()),
    )
}
/// `Sensitivity : (Dataset → Real) → Real → Prop`
/// The query f has global L1 sensitivity Δ if
/// max_{adjacent D, D'} |f(D) - f(D')| ≤ Δ
pub fn sensitivity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Dataset"), real_ty()),
        pi(BinderInfo::Default, "delta_f", real_ty(), prop()),
    )
}
/// `L2Sensitivity : (Dataset → Real) → Real → Prop`
/// Global L2 sensitivity for the Gaussian mechanism.
pub fn l2_sensitivity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Dataset"), real_ty()),
        pi(BinderInfo::Default, "delta_f", real_ty(), prop()),
    )
}
/// `LaplaceMechanism : (Dataset → Real) → Real → Mechanism`
/// Adds Laplace noise with scale b = Δf / ε.
pub fn laplace_mechanism_ty() -> Expr {
    arrow(
        arrow(cst("Dataset"), real_ty()),
        arrow(real_ty(), mechanism_ty()),
    )
}
/// `GaussianMechanism : (Dataset → Real) → Real → Real → Mechanism`
/// Adds Gaussian noise N(0, σ²) where σ = Δ₂f * sqrt(2 ln(1.25/δ)) / ε.
pub fn gaussian_mechanism_ty() -> Expr {
    arrow(
        arrow(cst("Dataset"), real_ty()),
        arrow(real_ty(), arrow(real_ty(), mechanism_ty())),
    )
}
/// `ExponentialMechanism : (Dataset → (List Real) → Real) → Real → Mechanism`
/// Samples output r proportionally to exp(ε * u(D, r) / (2 * Δu)).
pub fn exponential_mechanism_ty() -> Expr {
    arrow(
        arrow(cst("Dataset"), arrow(list_ty(real_ty()), real_ty())),
        arrow(real_ty(), mechanism_ty()),
    )
}
/// `SparseVector : (Dataset → List Real) → Real → Nat → Mechanism`
/// Sparse Vector Technique: release noisy above-threshold answers.
pub fn sparse_vector_ty() -> Expr {
    arrow(
        arrow(cst("Dataset"), list_ty(real_ty())),
        arrow(real_ty(), arrow(nat_ty(), mechanism_ty())),
    )
}
/// Sequential composition: composing k mechanisms with budgets ε₁…εₖ
/// yields overall budget ε₁ + … + εₖ.
/// `SequentialComposition : List Real → Real → Prop`
pub fn sequential_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "budgets",
        list_ty(real_ty()),
        pi(BinderInfo::Default, "total_eps", real_ty(), prop()),
    )
}
/// Parallel composition: mechanisms on disjoint subsets each cost their own ε.
/// The overall cost is max(ε₁, …, εₖ).
/// `ParallelComposition : List Real → Real → Prop`
pub fn parallel_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "budgets",
        list_ty(real_ty()),
        pi(BinderInfo::Default, "max_eps", real_ty(), prop()),
    )
}
/// Advanced composition (k mechanisms, δ' slack):
/// Achieves ε' ≈ ε*sqrt(2k*ln(1/δ')) + k*ε*(eᵉ - 1).
/// `AdvancedComposition : Nat → Real → Real → Real → Prop`
#[allow(clippy::too_many_arguments)]
pub fn advanced_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "delta_prime",
                    real_ty(),
                    pi(BinderInfo::Default, "eps_prime", real_ty(), prop()),
                ),
            ),
        ),
    )
}
/// Rényi composition: (α, ε₁)-RDP + (α, ε₂)-RDP → (α, ε₁+ε₂)-RDP.
/// `RenyiComposition : Real → Real → Real → Real → Prop`
pub fn renyi_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "eps1",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps2",
                real_ty(),
                pi(BinderInfo::Default, "eps_sum", real_ty(), prop()),
            ),
        ),
    )
}
/// RDP to (ε, δ)-DP conversion:
/// (α, ε_rdp)-RDP → (ε_rdp + log(1/δ)/(α-1), δ)-DP for any δ > 0.
/// `RenyiToApproxDP : Real → Real → Real → Real → Prop`
pub fn renyi_to_approx_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "eps_rdp",
            real_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                pi(BinderInfo::Default, "eps_approx", real_ty(), prop()),
            ),
        ),
    )
}
/// `RenyiDivergence : List Real → List Real → Real → Real`
/// D_α(P || Q) = (1/(α-1)) * log(Σ P(x)^α * Q(x)^(1-α))
pub fn renyi_divergence_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), arrow(real_ty(), real_ty())),
    )
}
/// `MomentsAccountant : List Real → Real → Real → Prop`
/// The moments accountant tracks privacy loss random variable moments.
pub fn moments_accountant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "moments",
        list_ty(real_ty()),
        pi(
            BinderInfo::Default,
            "lambda",
            real_ty(),
            pi(BinderInfo::Default, "eps_bound", real_ty(), prop()),
        ),
    )
}
/// `DpSgd : Nat → Real → Real → Real → Prop`
/// DP-SGD (Abadi et al. 2016): with T steps, sampling rate q,
/// gradient clipping bound C, noise multiplier σ:
/// achieves (ε, δ)-DP for appropriate (ε, δ).
#[allow(clippy::too_many_arguments)]
pub fn dp_sgd_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "q",
            real_ty(),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "eps",
                    real_ty(),
                    pi(BinderInfo::Default, "delta", real_ty(), prop()),
                ),
            ),
        ),
    )
}
/// `PrivacyAmplification : Real → Real → Real → Prop`
/// Subsampling amplification: applying an ε-DP mechanism to a random subset
/// of size m from n records gives (ε', δ')-DP with ε' ≈ log(1 + q*(eᵉ - 1)).
pub fn privacy_amplification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        pi(
            BinderInfo::Default,
            "q",
            real_ty(),
            pi(BinderInfo::Default, "eps_prime", real_ty(), prop()),
        ),
    )
}
/// `ZeroConcentratedDP : Mechanism → Real → Prop`
/// (ρ)-zCDP: M satisfies zero-concentrated DP if for all α > 1 and adjacent D, D':
/// D_α(M(D) || M(D')) ≤ ρ * α
/// Bun & Steinke (2016).
pub fn zero_concentrated_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(BinderInfo::Default, "rho", real_ty(), prop()),
    )
}
/// `TruncatedConcentratedDP : Mechanism → Real → Real → Prop`
/// (ξ, ρ)-tCDP: a relaxation of zCDP that allows a small per-step ξ slack.
pub fn truncated_concentrated_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(
            BinderInfo::Default,
            "xi",
            real_ty(),
            pi(BinderInfo::Default, "rho", real_ty(), prop()),
        ),
    )
}
/// `ApproximateDP : Mechanism → Real → Real → Prop`
/// Alias emphasising the (ε, δ) pair; separate from `EpsDeltaDP` to allow
/// different elaboration paths.
pub fn approximate_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(BinderInfo::Default, "delta", real_ty(), prop()),
        ),
    )
}
/// `RandomizedResponse : Real → Mechanism`
/// Randomized response with flipping probability p satisfies local DP with
/// ε = ln((1-p)/p).
pub fn randomized_response_ty() -> Expr {
    arrow(real_ty(), mechanism_ty())
}
/// `ReportNoisyMax : (Dataset → List Real) → Real → Mechanism`
/// Report-Noisy-Max: adds Laplace noise to each score and returns the argmax.
pub fn report_noisy_max_ty() -> Expr {
    arrow(
        arrow(cst("Dataset"), list_ty(real_ty())),
        arrow(real_ty(), mechanism_ty()),
    )
}
/// `PrivacyAmplificationByShuffling : Real → Real → Real → Prop`
/// The shuffle model: applying a local randomizer ε₀-LDP and then shuffling
/// the messages achieves central DP with ε_central ≈ O(ε₀ * sqrt(ln(1/δ) / n)).
pub fn privacy_amplification_shuffling_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps_local",
        real_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                pi(BinderInfo::Default, "eps_central", real_ty(), prop()),
            ),
        ),
    )
}
/// `ZcdpToApproxDP : Real → Real → Real → Prop`
/// ρ-zCDP → (ε, δ)-DP conversion: ε = ρ + 2 * sqrt(ρ * ln(1/δ)).
pub fn zcdp_to_approx_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "rho",
        real_ty(),
        pi(
            BinderInfo::Default,
            "delta",
            real_ty(),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `ZcdpComposition : Real → Real → Real → Prop`
/// ρ₁-zCDP composed with ρ₂-zCDP gives (ρ₁ + ρ₂)-zCDP.
pub fn zcdp_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "rho1",
        real_ty(),
        pi(
            BinderInfo::Default,
            "rho2",
            real_ty(),
            pi(BinderInfo::Default, "rho_sum", real_ty(), prop()),
        ),
    )
}
/// `GroupPrivacyGeneral : Mechanism → Real → Real → Nat → Prop`
/// k-group privacy: if M satisfies (ε, δ)-DP then for groups differing in k
/// records, M satisfies (k*ε, k*e^((k-1)*ε)*δ)-DP.
#[allow(clippy::too_many_arguments)]
pub fn group_privacy_general_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(
                BinderInfo::Default,
                "delta",
                real_ty(),
                pi(BinderInfo::Default, "k", nat_ty(), prop()),
            ),
        ),
    )
}
/// `PrivateQueryRelease : Nat → Real → Real → Prop`
/// Private query release: answering n queries with error α using (ε, δ)-DP.
/// Corresponds to the Offline Private Data Release framework.
pub fn private_query_release_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n_queries",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "alpha",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(BinderInfo::Default, "delta", real_ty(), prop()),
            ),
        ),
    )
}
/// `DpErmSampleComplexity : Real → Real → Real → Real → Nat → Prop`
/// DP-ERM sample complexity: to achieve excess risk α with (ε, δ)-DP,
/// n = Ω(d / (α * ε)) samples suffice (d = dimension).
#[allow(clippy::too_many_arguments)]
pub fn dp_erm_sample_complexity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "d",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "alpha",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "delta",
                    real_ty(),
                    pi(BinderInfo::Default, "n_samples", nat_ty(), prop()),
                ),
            ),
        ),
    )
}
/// `FingerprintingLowerBound : Nat → Real → Real → Prop`
/// Fingerprinting codes lower bound (Bassily et al. 2014): any (ε, δ)-DP
/// mechanism answering n counting queries needs Ω(sqrt(n) / ε) rows.
pub fn fingerprinting_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n_queries",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(BinderInfo::Default, "n_rows_lower_bound", nat_ty(), prop()),
        ),
    )
}
/// `PrivateLearningPACComplexity : Nat → Real → Real → Real → Nat → Prop`
/// DP-PAC learning: concept class of VC-dimension d is privately PAC-learnable
/// using n = O((d + log(1/δ)) / (α * ε)) samples.
#[allow(clippy::too_many_arguments)]
pub fn private_learning_pac_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "vc_dim",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "alpha",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "delta",
                    real_ty(),
                    pi(BinderInfo::Default, "n_samples", nat_ty(), prop()),
                ),
            ),
        ),
    )
}
/// `PrivacyLossRV : Mechanism → Real`
/// The privacy loss random variable L^{M}_{D,D'} = log(Pr[M(D)=o] / Pr[M(D')=o]).
pub fn privacy_loss_rv_ty() -> Expr {
    arrow(mechanism_ty(), real_ty())
}
/// `PrivacyCLT : Nat → Real → Real → Real → Prop`
/// Central Limit Theorem for privacy (Dong et al. 2022): the privacy loss of
/// k i.i.d. mechanisms converges to a Gaussian trade-off function f-DP.
pub fn privacy_clt_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "mu",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(BinderInfo::Default, "delta_approx", real_ty(), prop()),
            ),
        ),
    )
}
/// `FDifferentialPrivacy : Mechanism → Real → Prop`
/// f-Differential Privacy (Dong, Roth, Su 2022): M satisfies f-DP if the
/// trade-off between false positive and false negative rates is bounded by f.
pub fn f_dp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        mechanism_ty(),
        pi(BinderInfo::Default, "mu", real_ty(), prop()),
    )
}
/// `GaussianDPDefinition : Real → Real → Prop`
/// The Gaussian DP definition: G_μ-DP is f_μ-DP where f_μ is the trade-off
/// function of N(0,1) vs N(μ,1) hypothesis test.
pub fn gaussian_dp_definition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "mu",
        real_ty(),
        pi(BinderInfo::Default, "eps_approx", real_ty(), prop()),
    )
}
/// `KLDivergenceBound : Real → Real → Prop`
/// KL-divergence upper bound: DP implies KL(M(D) || M(D')) ≤ ε.
pub fn kl_divergence_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        pi(BinderInfo::Default, "kl_bound", real_ty(), prop()),
    )
}
/// `TotalVariationBound : Real → Real → Real → Prop`
/// Total variation distance bound: (ε, δ)-DP implies TV(M(D), M(D')) ≤ 1 - e^{-ε} + δ.
pub fn total_variation_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        pi(
            BinderInfo::Default,
            "delta",
            real_ty(),
            pi(BinderInfo::Default, "tv_bound", real_ty(), prop()),
        ),
    )
}
/// `DpSgdGaussianMechanism : Nat → Real → Real → Real → Real → Prop`
/// DP-SGD with Gaussian mechanism (Abadi et al. 2016 exact form):
/// T steps, sampling rate q, noise σ, clipping C, achieves (ε, δ)-DP.
#[allow(clippy::too_many_arguments)]
pub fn dp_sgd_gaussian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "q",
            real_ty(),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "C",
                    real_ty(),
                    pi(
                        BinderInfo::Default,
                        "eps",
                        real_ty(),
                        pi(BinderInfo::Default, "delta", real_ty(), prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `RenyiAmplificationBySubsampling : Real → Real → Real → Real → Prop`
/// RDP amplification by Poisson subsampling: (α, ε)-RDP with sampling rate q
/// gives (α, ε')-RDP where ε' = (1/(α-1)) * log(1 + q²α*(e^ε - 1) + ...).
pub fn renyi_amplification_subsampling_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "eps_rdp",
            real_ty(),
            pi(
                BinderInfo::Default,
                "q",
                real_ty(),
                pi(BinderInfo::Default, "eps_amplified", real_ty(), prop()),
            ),
        ),
    )
}
/// `GaussianRDP : Real → Real → Real → Prop`
/// The Gaussian mechanism with L2 sensitivity Δ₂ and std σ satisfies
/// (α, Δ₂² * α / (2 * σ²))-RDP.
pub fn gaussian_rdp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "l2_sensitivity",
            real_ty(),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                pi(BinderInfo::Default, "eps_rdp", real_ty(), prop()),
            ),
        ),
    )
}
/// `LaplaceRDP : Real → Real → Real → Prop`
/// The Laplace mechanism with L1 sensitivity Δ and scale b satisfies (α, ε_rdp)-RDP.
pub fn laplace_rdp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "sensitivity",
            real_ty(),
            pi(
                BinderInfo::Default,
                "scale",
                real_ty(),
                pi(BinderInfo::Default, "eps_rdp", real_ty(), prop()),
            ),
        ),
    )
}
/// `SubsampledGaussianRDP : Real → Real → Real → Real → Prop`
/// The subsampled Gaussian mechanism (used in DP-SGD) satisfies (α, ε_rdp)-RDP
/// parameterized by sampling rate q and noise σ.
pub fn subsampled_gaussian_rdp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "q",
            real_ty(),
            pi(
                BinderInfo::Default,
                "sigma",
                real_ty(),
                pi(BinderInfo::Default, "eps_rdp", real_ty(), prop()),
            ),
        ),
    )
}
/// `PostProcessingInvariance : Mechanism → Mechanism → Prop`
/// Post-processing: applying any (randomised) function to a DP output
/// cannot increase the privacy loss.
pub fn post_processing_invariance_ty() -> Expr {
    arrow(mechanism_ty(), arrow(mechanism_ty(), prop()))
}
/// `LocalSensitivity : (Dataset → Real) → cst("Dataset") → Real → Prop`
/// Local sensitivity of f at D: max_{D' adjacent to D} |f(D) - f(D')|.
pub fn local_sensitivity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Dataset"), real_ty()),
        pi(
            BinderInfo::Default,
            "D",
            cst("Dataset"),
            pi(BinderInfo::Default, "ls", real_ty(), prop()),
        ),
    )
}
/// `SmoothSensitivity : (Dataset → Real) → Real → Real → Prop`
/// β-smooth sensitivity S_f^β(D) = max_{D'} e^{-β * dist(D,D')} * LS_f(D').
pub fn smooth_sensitivity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Dataset"), real_ty()),
        pi(
            BinderInfo::Default,
            "beta",
            real_ty(),
            pi(BinderInfo::Default, "ss", real_ty(), prop()),
        ),
    )
}
/// `PrivateSelectionCorrectness : Nat → Real → Real → Prop`
/// Private selection (Report-Noisy-Max, Exponential Mechanism):
/// the probability of selecting the highest-utility item within gap α.
pub fn private_selection_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k_candidates",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(BinderInfo::Default, "gap", real_ty(), prop()),
        ),
    )
}
/// `RandomizedResponseCorrectness : Real → Real → Prop`
/// Randomized response with flipping probability p satisfies ε-LDP
/// where ε = ln((1-p)/p).
pub fn randomized_response_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        real_ty(),
        pi(BinderInfo::Default, "eps_ldp", real_ty(), prop()),
    )
}
/// `SparseVectorAboveThreshold : (Dataset → List Real) → Real → Real → Nat → Prop`
/// Above-threshold variant of the sparse vector technique.
pub fn sparse_vector_above_threshold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "queries",
        arrow(cst("Dataset"), list_ty(real_ty())),
        pi(
            BinderInfo::Default,
            "threshold",
            real_ty(),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(BinderInfo::Default, "k", nat_ty(), prop()),
            ),
        ),
    )
}
/// Register all differential privacy axioms and theorems in the kernel environment.
pub fn build_differential_privacy_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("DP.Dataset", dataset_ty()),
        ("DP.EpsDeltaDP", eps_delta_dp_ty()),
        ("DP.PureDP", pure_dp_ty()),
        ("DP.RenyiDP", renyi_dp_ty()),
        ("DP.LocalDP", local_dp_ty()),
        ("DP.Sensitivity", sensitivity_ty()),
        ("DP.L2Sensitivity", l2_sensitivity_ty()),
        ("DP.Mechanism", mechanism_ty()),
        ("DP.LaplaceMechanism", laplace_mechanism_ty()),
        ("DP.GaussianMechanism", gaussian_mechanism_ty()),
        ("DP.ExponentialMechanism", exponential_mechanism_ty()),
        ("DP.SparseVector", sparse_vector_ty()),
        ("DP.SequentialComposition", sequential_composition_ty()),
        ("DP.ParallelComposition", parallel_composition_ty()),
        ("DP.AdvancedComposition", advanced_composition_ty()),
        ("DP.RenyiComposition", renyi_composition_ty()),
        ("DP.RenyiToApproxDP", renyi_to_approx_dp_ty()),
        ("DP.RenyiDivergence", renyi_divergence_ty()),
        ("DP.MomentsAccountant", moments_accountant_ty()),
        ("DP.DpSgd", dp_sgd_ty()),
        ("DP.PrivacyAmplification", privacy_amplification_ty()),
        (
            "DP.LaplaceMechanismCorrect",
            pi(
                BinderInfo::Default,
                "f",
                arrow(cst("Dataset"), real_ty()),
                pi(
                    BinderInfo::Default,
                    "delta_f",
                    real_ty(),
                    pi(BinderInfo::Default, "eps", real_ty(), prop()),
                ),
            ),
        ),
        (
            "DP.GaussianMechanismCorrect",
            pi(
                BinderInfo::Default,
                "f",
                arrow(cst("Dataset"), real_ty()),
                pi(
                    BinderInfo::Default,
                    "delta_f",
                    real_ty(),
                    pi(
                        BinderInfo::Default,
                        "eps",
                        real_ty(),
                        pi(BinderInfo::Default, "delta", real_ty(), prop()),
                    ),
                ),
            ),
        ),
        (
            "DP.ExponentialMechanismCorrect",
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
        (
            "DP.SparseVectorCorrect",
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(BinderInfo::Default, "k", nat_ty(), prop()),
            ),
        ),
        (
            "DP.PostProcessing",
            arrow(mechanism_ty(), arrow(mechanism_ty(), prop())),
        ),
        (
            "DP.GroupPrivacy",
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                pi(BinderInfo::Default, "k", nat_ty(), prop()),
            ),
        ),
        ("DP.ZeroConcentratedDP", zero_concentrated_dp_ty()),
        ("DP.TruncatedConcentratedDP", truncated_concentrated_dp_ty()),
        ("DP.ApproximateDP", approximate_dp_ty()),
        ("DP.RandomizedResponse", randomized_response_ty()),
        ("DP.ReportNoisyMax", report_noisy_max_ty()),
        (
            "DP.PrivacyAmplificationByShuffling",
            privacy_amplification_shuffling_ty(),
        ),
        ("DP.ZcdpToApproxDP", zcdp_to_approx_dp_ty()),
        ("DP.ZcdpComposition", zcdp_composition_ty()),
        ("DP.GroupPrivacyGeneral", group_privacy_general_ty()),
        ("DP.PrivateQueryRelease", private_query_release_ty()),
        ("DP.DpErmSampleComplexity", dp_erm_sample_complexity_ty()),
        (
            "DP.FingerprintingLowerBound",
            fingerprinting_lower_bound_ty(),
        ),
        ("DP.PrivateLearningPAC", private_learning_pac_ty()),
        ("DP.PrivacyLossRV", privacy_loss_rv_ty()),
        ("DP.PrivacyCLT", privacy_clt_ty()),
        ("DP.FDifferentialPrivacy", f_dp_ty()),
        ("DP.GaussianDPDefinition", gaussian_dp_definition_ty()),
        ("DP.KLDivergenceBound", kl_divergence_bound_ty()),
        ("DP.TotalVariationBound", total_variation_bound_ty()),
        ("DP.DpSgdGaussian", dp_sgd_gaussian_ty()),
        (
            "DP.RenyiAmplificationBySubsampling",
            renyi_amplification_subsampling_ty(),
        ),
        ("DP.GaussianRDP", gaussian_rdp_ty()),
        ("DP.LaplaceRDP", laplace_rdp_ty()),
        ("DP.SubsampledGaussianRDP", subsampled_gaussian_rdp_ty()),
        (
            "DP.PostProcessingInvariance",
            post_processing_invariance_ty(),
        ),
        ("DP.LocalSensitivity", local_sensitivity_ty()),
        ("DP.SmoothSensitivity", smooth_sensitivity_ty()),
        (
            "DP.PrivateSelectionCorrectness",
            private_selection_correctness_ty(),
        ),
        (
            "DP.RandomizedResponseCorrectness",
            randomized_response_correctness_ty(),
        ),
        (
            "DP.SparseVectorAboveThreshold",
            sparse_vector_above_threshold_ty(),
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
    Ok(())
}
/// Rényi divergence D_α(P || Q) of order α between two discrete distributions.
///
/// D_α(P || Q) = (1 / (α - 1)) * log(Σ_x P(x)^α * Q(x)^(1-α))
pub fn renyi_divergence(p: &[f64], q: &[f64], alpha: f64) -> f64 {
    assert_eq!(p.len(), q.len(), "distributions must have the same support");
    assert!(
        alpha > 0.0 && (alpha - 1.0).abs() > 1e-10,
        "α must be > 0 and ≠ 1"
    );
    let sum: f64 = p
        .iter()
        .zip(q.iter())
        .filter(|(&pi, _)| pi > 0.0)
        .map(|(&pi, &qi)| {
            if qi == 0.0 {
                f64::INFINITY
            } else {
                pi.powf(alpha) * qi.powf(1.0 - alpha)
            }
        })
        .sum();
    sum.ln() / (alpha - 1.0)
}
/// Sequential composition of ε-DP mechanisms.
///
/// Composing k mechanisms with individual budgets ε₁, …, εₖ gives total budget Σεᵢ.
pub fn sequential_compose_eps(budgets: &[f64]) -> f64 {
    budgets.iter().sum()
}
/// Sequential composition for (ε, δ)-DP mechanisms.
///
/// k mechanisms each (εᵢ, δᵢ)-DP compose to (Σεᵢ, Σδᵢ)-DP.
pub fn sequential_compose_approx(budgets: &[(f64, f64)]) -> (f64, f64) {
    budgets
        .iter()
        .fold((0.0, 0.0), |(eps_acc, delta_acc), (e, d)| {
            (eps_acc + e, delta_acc + d)
        })
}
/// Parallel composition: mechanisms on disjoint subsets.
///
/// The combined mechanism is max(ε₁, …, εₖ)-DP.
pub fn parallel_compose_eps(budgets: &[f64]) -> f64 {
    budgets.iter().cloned().fold(0.0f64, f64::max)
}
/// Advanced composition (Dwork et al. 2010).
///
/// Composing k mechanisms each (ε, δ)-DP with slack δ' gives:
///   ε' = ε * sqrt(2k * ln(1/δ')) + k * ε * (exp(ε) - 1)
///   δ_total = k * δ + δ'
///
/// This beats naive sequential composition for large k.
pub fn advanced_composition(k: usize, eps: f64, delta: f64, delta_prime: f64) -> (f64, f64) {
    assert!(eps > 0.0 && delta >= 0.0 && delta_prime > 0.0);
    let eps_prime = eps * (2.0 * k as f64 * (1.0 / delta_prime).ln()).sqrt()
        + k as f64 * eps * (eps.exp() - 1.0);
    let delta_total = k as f64 * delta + delta_prime;
    (eps_prime, delta_total)
}
/// Privacy amplification by subsampling.
///
/// Applying an ε-DP mechanism to a random subset of size m from n records:
///   ε' ≈ log(1 + q * (exp(ε) - 1))   where q = m/n (sampling rate).
///
/// For small ε this is approximately q * ε (linear amplification).
pub fn privacy_amplification_by_subsampling(eps: f64, sampling_rate: f64) -> f64 {
    assert!(eps > 0.0 && sampling_rate > 0.0 && sampling_rate <= 1.0);
    (1.0 + sampling_rate * (eps.exp() - 1.0)).ln()
}
/// DP-SGD noise multiplier σ required for (ε, δ)-DP.
///
/// Simplified formula from Abadi et al. (2016):
///   σ ≥ C * sqrt(2 * T * ln(1/δ)) / (n * ε)
/// where C is the clipping bound, T is the number of steps, n is the dataset size.
pub fn dp_sgd_noise_multiplier(
    clipping_bound: f64,
    steps: usize,
    dataset_size: usize,
    eps: f64,
    delta: f64,
) -> f64 {
    assert!(eps > 0.0 && delta > 0.0 && delta < 1.0);
    let n = dataset_size as f64;
    let t = steps as f64;
    clipping_bound * (2.0 * t * (1.0 / delta).ln()).sqrt() / (n * eps)
}
/// Exponential mechanism: sample from a score function proportional to exp(ε * u / (2 * Δu)).
///
/// Given scores `u_vals` for each candidate output and sensitivity `delta_u`,
/// returns the probability of selecting each candidate.
pub fn exponential_mechanism_probs(u_vals: &[f64], eps: f64, delta_u: f64) -> Vec<f64> {
    assert!(!u_vals.is_empty());
    assert!(eps > 0.0 && delta_u > 0.0);
    let scaled: Vec<f64> = u_vals
        .iter()
        .map(|&u| (eps * u / (2.0 * delta_u)).exp())
        .collect();
    let total: f64 = scaled.iter().sum();
    scaled.iter().map(|&s| s / total).collect()
}
/// Sparse Vector Technique: count how many queries exceed a threshold T.
///
/// Only counts the first `k` above-threshold answers (releases a boolean vector).
/// This is an educational simulation: in practice, Laplace noise is added to
/// both the threshold and each query before comparison.
pub fn sparse_vector_simulate(
    query_vals: &[f64],
    threshold: f64,
    k: usize,
    noise_threshold: f64,
    noise_queries: &[f64],
) -> Vec<Option<bool>> {
    assert_eq!(query_vals.len(), noise_queries.len());
    let noisy_threshold = threshold + noise_threshold;
    let mut above_count = 0usize;
    query_vals
        .iter()
        .zip(noise_queries.iter())
        .map(|(&q, &nq)| {
            if above_count >= k {
                None
            } else {
                let noisy_q = q + nq;
                if noisy_q >= noisy_threshold {
                    above_count += 1;
                    Some(true)
                } else {
                    Some(false)
                }
            }
        })
        .collect()
}
/// Compute the RDP guarantee of the Gaussian mechanism.
///
/// For the Gaussian mechanism with noise σ and L2 sensitivity Δ₂:
///   ε_rdp(α) = α * Δ₂² / (2 * σ²)
pub fn gaussian_rdp(alpha: f64, l2_sensitivity: f64, sigma: f64) -> f64 {
    assert!(alpha > 1.0 && sigma > 0.0 && l2_sensitivity > 0.0);
    alpha * l2_sensitivity * l2_sensitivity / (2.0 * sigma * sigma)
}
/// Compute the RDP guarantee of the Laplace mechanism.
///
/// For the Laplace mechanism with scale b = Δ/ε and L1 sensitivity Δ:
///   ε_rdp(α) = (1/(α-1)) * log( α/(2α-1) * exp((α-1)/b) + (α-1)/(2α-1) * exp(-α/b) )
///
/// Simplified bound: ε_rdp(α) ≤ α / (2 * b²)  for small noise.
/// We return the exact formula clamped for numerical stability.
pub fn laplace_rdp(alpha: f64, sensitivity: f64, scale: f64) -> f64 {
    assert!(alpha > 1.0 && scale > 0.0 && sensitivity > 0.0);
    let b = scale;
    let t1 = alpha * (-(alpha - 1.0) / b).exp() / (2.0 * alpha - 1.0);
    let t2 = (alpha - 1.0) * (alpha / b).exp() / (2.0 * alpha - 1.0);
    let inner = t1 + t2;
    if inner <= 0.0 {
        return f64::INFINITY;
    }
    inner.ln() / (alpha - 1.0)
}
/// Convert ρ-zCDP to (ε, δ)-DP.
///
/// ε = ρ + 2 * sqrt(ρ * ln(1/δ))
pub fn zcdp_to_approx_dp(rho: f64, delta: f64) -> f64 {
    assert!(rho > 0.0, "rho must be positive");
    assert!(delta > 0.0 && delta < 1.0, "delta must be in (0, 1)");
    rho + 2.0 * (rho * (1.0 / delta).ln()).sqrt()
}
/// Compose two ρ-zCDP guarantees (additive).
///
/// ρ₁-zCDP composed with ρ₂-zCDP gives (ρ₁ + ρ₂)-zCDP.
pub fn zcdp_compose(rho1: f64, rho2: f64) -> f64 {
    assert!(rho1 >= 0.0 && rho2 >= 0.0);
    rho1 + rho2
}
/// Randomized response local DP.
///
/// With flipping probability p the true bit is reported; with probability 1-p
/// the opposite is reported.
/// ε = ln((1-p) / p)  — requires p < 0.5 for ε > 0.
pub fn randomized_response_epsilon(flip_prob: f64) -> f64 {
    assert!(
        flip_prob > 0.0 && flip_prob < 0.5,
        "flip_prob must be in (0, 0.5) for positive ε"
    );
    ((1.0 - flip_prob) / flip_prob).ln()
}
/// Group privacy: k-group privacy cost given single-record (ε, δ)-DP.
///
/// Returns (k*ε,  k * e^{(k-1)*ε} * δ).
pub fn group_privacy_cost(eps: f64, delta: f64, k: usize) -> (f64, f64) {
    assert!(eps > 0.0 && delta >= 0.0 && k >= 1);
    let kf = k as f64;
    let eps_k = kf * eps;
    let delta_k = kf * ((kf - 1.0) * eps).exp() * delta;
    (eps_k, delta_k)
}
/// Tight RDP budget for subsampled Gaussian (simplified Mironov bound).
///
/// For Poisson subsampling with rate q and noise σ, the (α, ε_rdp) RDP
/// guarantee is approximately:
///   ε_rdp ≈ (1 / (α - 1)) * log(1 + q² * α * Δ² / (2σ²))
/// (valid for small q and α ≥ 2).
pub fn subsampled_gaussian_rdp(alpha: f64, q: f64, sigma: f64, sensitivity: f64) -> f64 {
    assert!(alpha > 1.0, "alpha must be > 1");
    assert!(q > 0.0 && q <= 1.0, "sampling rate must be in (0, 1]");
    assert!(sigma > 0.0, "sigma must be positive");
    assert!(sensitivity > 0.0, "sensitivity must be positive");
    let ratio = sensitivity * sensitivity / (2.0 * sigma * sigma);
    let inner = 1.0 + q * q * alpha * ratio;
    inner.ln().max(0.0) / (alpha - 1.0)
}
/// KL divergence upper bound given ε-DP.
///
/// ε-DP implies KL(M(D) || M(D')) ≤ ε * (exp(ε) - 1) / 2 ≤ ε² for ε ≤ 1.
pub fn kl_dp_bound(eps: f64) -> f64 {
    assert!(eps > 0.0);
    eps * (eps.exp() - 1.0) / 2.0
}
/// Total variation distance bound from (ε, δ)-DP.
///
/// TV(M(D), M(D')) ≤ 1 - e^{-ε} + δ ≤ ε + δ for small ε.
pub fn total_variation_dp_bound(eps: f64, delta: f64) -> f64 {
    assert!(eps >= 0.0 && delta >= 0.0);
    1.0 - (-eps).exp() + delta
}
/// DP-SGD privacy accounting via moments accountant (simplified).
///
/// Uses the tight bound σ ≥ sqrt(T) * q * Δ / ε (high-probability version).
/// Returns the total ε spent after `steps` iterations.
pub fn dp_sgd_rdp_accounting(
    steps: usize,
    sampling_rate: f64,
    sigma: f64,
    sensitivity: f64,
    alpha: f64,
) -> f64 {
    assert!(sigma > 0.0 && sampling_rate > 0.0 && alpha > 1.0);
    let per_step_rdp = subsampled_gaussian_rdp(alpha, sampling_rate, sigma, sensitivity);
    per_step_rdp * steps as f64
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-9;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_differential_privacy_env(&mut env);
        assert!(
            result.is_ok(),
            "build_differential_privacy_env should succeed: {:?}",
            result
        );
    }
    #[test]
    fn test_laplace_scale() {
        let b = LaplaceNoise::scale_for_pure_dp(1.0, 1.0);
        assert!((b - 1.0).abs() < EPS, "expected b=1.0, got {b}");
        let b2 = LaplaceNoise::scale_for_pure_dp(1.0, 0.5);
        assert!((b2 - 2.0).abs() < EPS, "expected b=2.0, got {b2}");
    }
    #[test]
    fn test_laplace_sample_from_uniform() {
        let noise = LaplaceNoise::new(1.0);
        let s = noise.sample_from_uniform(0.75);
        assert!(s > 0.0, "sample should be positive for u > 0.5, got {s}");
        let s2 = noise.sample_from_uniform(0.25);
        assert!(s2 < 0.0, "sample should be negative for u < 0.5, got {s2}");
    }
    #[test]
    fn test_gaussian_sigma() {
        let sigma = GaussianNoise::sigma_for_approx_dp(1.0, 1.0, 1e-5);
        assert!(sigma > 4.0 && sigma < 6.0, "σ out of range: {sigma}");
    }
    #[test]
    fn test_gaussian_rdp() {
        let rdp = gaussian_rdp(2.0, 1.0, 1.0);
        assert!((rdp - 1.0).abs() < EPS, "expected 1.0, got {rdp}");
    }
    #[test]
    fn test_renyi_accountant() {
        let mut acc = RenyiAccountant::new();
        acc.compose(2.0, 0.5);
        acc.compose(2.0, 0.5);
        let entry = acc.ledger.iter().find(|(a, _)| (*a - 2.0).abs() < 1e-10);
        assert!(entry.is_some());
        assert!((entry.expect("entry should be valid").1 - 1.0).abs() < EPS);
        let eps_dp = acc.to_approx_dp(2.0, 1.0, 1e-5);
        assert!(
            eps_dp > 12.0 && eps_dp < 13.0,
            "eps_dp out of range: {eps_dp}"
        );
    }
    #[test]
    fn test_sequential_composition() {
        let budgets = [0.1, 0.2, 0.3];
        let total = sequential_compose_eps(&budgets);
        assert!((total - 0.6).abs() < EPS, "expected 0.6, got {total}");
    }
    #[test]
    fn test_parallel_composition() {
        let budgets = [0.1, 0.5, 0.3];
        let max_eps = parallel_compose_eps(&budgets);
        assert!((max_eps - 0.5).abs() < EPS, "expected 0.5, got {max_eps}");
    }
    #[test]
    fn test_exponential_mechanism_probs() {
        let probs = exponential_mechanism_probs(&[1.0, 0.0], 1.0, 1.0);
        assert_eq!(probs.len(), 2);
        assert!(
            probs[0] > probs[1],
            "higher-score candidate must have higher probability"
        );
        let sum: f64 = probs.iter().sum();
        assert!(
            (sum - 1.0).abs() < EPS,
            "probabilities must sum to 1, got {sum}"
        );
    }
    #[test]
    fn test_privacy_amplification() {
        let eps_prime = privacy_amplification_by_subsampling(1.0, 0.1);
        assert!(
            eps_prime > 0.0 && eps_prime < 1.0,
            "amplified ε out of range: {eps_prime}"
        );
        assert!(eps_prime <= 1.0, "amplified ε cannot exceed original");
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        let result = build_differential_privacy_env(&mut env);
        assert!(
            result.is_ok(),
            "build_differential_privacy_env (extended) should succeed: {:?}",
            result
        );
    }
    #[test]
    fn test_laplace_mechanism_struct() {
        let mech = LaplaceMechanism::new(1.0, 2.0);
        assert!((mech.scale - 0.5).abs() < EPS, "scale = Δ/ε = 0.5");
        assert!((mech.privacy_loss() - 2.0).abs() < EPS);
        let out = mech.apply(10.0, 0.75);
        assert!(out > 10.0, "positive noise for u > 0.5");
    }
    #[test]
    fn test_gaussian_mechanism_struct() {
        let mech = GaussianMechanism::new(1.0, 1.0, 1e-5);
        assert!(mech.sigma > 0.0, "sigma must be positive");
        let rdp = mech.rdp_guarantee(2.0);
        let expected = 2.0 * 1.0 / (2.0 * mech.sigma * mech.sigma);
        assert!((rdp - expected).abs() < 1e-9, "RDP mismatch");
    }
    #[test]
    fn test_exponential_mechanism_struct() {
        let mech = ExponentialMechanism::new(1.0, 1.0);
        let scores = [2.0, 1.0, 0.0];
        let probs = mech.probabilities(&scores);
        assert_eq!(probs.len(), 3);
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9, "probs must sum to 1");
        assert!(probs[0] > probs[1], "higher score → higher prob");
        let idx = mech.sample_index(&probs, 0.0);
        assert_eq!(idx, 0, "u=0 should select index 0");
    }
    #[test]
    fn test_privacy_budget() {
        let mut budget = PrivacyBudget::new(1.0, 1e-5);
        assert!(budget.spend(0.3, 1e-6).is_ok());
        assert!(budget.spend(0.3, 1e-6).is_ok());
        assert!(
            budget.spend(0.5, 1e-6).is_err(),
            "should fail: total ε would exceed 1.0"
        );
        assert!((budget.remaining_epsilon() - 0.4).abs() < 1e-9);
    }
    #[test]
    fn test_zcdp_to_approx_dp() {
        let eps = zcdp_to_approx_dp(0.5, 1e-5);
        let ln1e5 = (1.0f64 / 1e-5f64).ln();
        let expected = 0.5 + 2.0 * (0.5 * ln1e5).sqrt();
        assert!((eps - expected).abs() < 1e-9, "zCDP conversion mismatch");
    }
    #[test]
    fn test_zcdp_compose() {
        let rho = zcdp_compose(0.3, 0.2);
        assert!((rho - 0.5).abs() < EPS, "zCDP composition should add");
    }
    #[test]
    fn test_randomized_response_epsilon() {
        let eps = randomized_response_epsilon(0.25);
        assert!((eps - 3.0f64.ln()).abs() < 1e-9, "RR epsilon mismatch");
    }
    #[test]
    fn test_group_privacy_cost() {
        let (eps_k, delta_k) = group_privacy_cost(1.0, 1e-5, 2);
        assert!((eps_k - 2.0).abs() < EPS, "2-group ε = 2ε");
        let expected_delta = 2.0 * std::f64::consts::E * 1e-5;
        assert!(
            (delta_k - expected_delta).abs() < 1e-12,
            "2-group δ mismatch"
        );
    }
    #[test]
    fn test_kl_dp_bound() {
        let bound = kl_dp_bound(0.1);
        assert!(bound > 0.0 && bound < 0.1, "KL bound for ε=0.1");
    }
    #[test]
    fn test_total_variation_dp_bound() {
        let tv = total_variation_dp_bound(1.0, 1e-5);
        let expected = 1.0 - std::f64::consts::E.recip() + 1e-5;
        assert!((tv - expected).abs() < 1e-9, "TV bound mismatch");
        assert!(tv < 1.0, "TV bound must be < 1");
    }
    #[test]
    fn test_subsampled_gaussian_rdp() {
        let rdp = subsampled_gaussian_rdp(2.0, 0.1, 1.0, 1.0);
        assert!(rdp >= 0.0, "RDP must be non-negative");
        let unsubsampled = subsampled_gaussian_rdp(2.0, 1.0, 1.0, 1.0);
        assert!(rdp < unsubsampled, "subsampling must reduce RDP");
    }
    #[test]
    fn test_dp_sgd_rdp_accounting() {
        let total_rdp = dp_sgd_rdp_accounting(100, 0.01, 1.0, 1.0, 2.0);
        assert!(total_rdp >= 0.0, "accumulated RDP must be non-negative");
        let total_rdp_200 = dp_sgd_rdp_accounting(200, 0.01, 1.0, 1.0, 2.0);
        assert!(
            (total_rdp_200 - 2.0 * total_rdp).abs() < 1e-9,
            "RDP should scale linearly with steps"
        );
    }
}
/// Summary of DP composition theorems.
#[allow(dead_code)]
pub fn dp_composition_theorems() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Basic Composition", "k mechanisms: (k*eps, k*delta)-DP"),
        (
            "Advanced Composition",
            "k mechanisms: (eps*sqrt(2k*ln(1/delta')), k*delta+delta')-DP",
        ),
        ("RDP Composition", "Sum of RDP epsilons at same alpha"),
        ("zCDP Composition", "Sum of rho values"),
        (
            "Privacy Amplification by Sampling",
            "Subsampling rate q: eps -> log(1+q*(e^eps-1))",
        ),
        (
            "Privacy Amplification by Shuffling",
            "Local eps -> central O(e^eps * sqrt(log/n))",
        ),
        (
            "Parallel Composition",
            "Disjoint datasets: max(eps_i, delta_i)",
        ),
        ("Post-Processing", "Applying any function preserves DP"),
    ]
}
#[cfg(test)]
mod dp_ext_tests {
    use super::*;
    #[test]
    fn test_renyi_dp() {
        let rdp = RenyiDp::new(2.0, 0.5);
        let (eps, _delta) = rdp.to_pure_dp();
        assert!(eps > rdp.epsilon);
    }
    #[test]
    fn test_zcdp_compose() {
        let z1 = ZcdpBound::new(0.1);
        let z2 = ZcdpBound::new(0.2);
        let z3 = z1.compose(&z2);
        assert!((z3.rho - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_dp_sgd() {
        let dp = DpSgd::new(0.01, 1.1, 1.0, 256, 1000, 60000);
        assert!((dp.sampling_rate() - 256.0 / 60000.0).abs() < 1e-10);
        let eps_rdp = dp.privacy_spent_rdp_alpha(2.0);
        assert!(eps_rdp > 0.0);
    }
    #[test]
    fn test_local_dp() {
        let m = LocalDpMechanism::randomized_response(1.0);
        assert!(m.is_locally_private());
        let var = m.variance_estimate();
        assert!(var > 0.0);
    }
    #[test]
    fn test_shuffle_amplification() {
        let sa = ShuffleAmplification::new(5.0, 10000);
        assert!(sa.is_stronger_than_local_dp());
    }
    #[test]
    fn test_composition_theorems_nonempty() {
        let thms = dp_composition_theorems();
        assert!(!thms.is_empty());
    }
}
#[cfg(test)]
mod dp_more_tests {
    use super::*;
    #[test]
    fn test_dp_mean() {
        let est = DpMeanEstimator::new(1.0, 1e-5, -1.0, 1.0, 1000);
        assert!(est.clipped_sensitivity() > 0.0);
        assert!(est.mse_gaussian_mechanism() > 0.0);
    }
    #[test]
    fn test_inference_attack() {
        let atk = InferenceAttackModel::new(0.5, 1.0);
        assert!(atk.advantage_bounded_by_dp());
    }
    #[test]
    fn test_dp_synthetic() {
        let syn = DpSyntheticData::new(1.0, 1e-5, 10, SyntheticDataMethod::PrivBayes);
        assert!(syn.marginal_error_bound() > 0.0);
    }
}
#[cfg(test)]
mod dp_ledger_tests {
    use super::*;
    #[test]
    fn test_privacy_ledger() {
        let mut ledger = PrivacyLedger::new();
        ledger.add_entry("laplace", 1.0, 0.0, CompositionType::Sequential);
        ledger.add_entry("gaussian", 0.5, 1e-5, CompositionType::Sequential);
        assert!((ledger.total_sequential_epsilon() - 1.5).abs() < 1e-10);
        assert!(ledger.total_sequential_delta() > 0.0);
    }
    #[test]
    fn test_parallel_composition() {
        let mut ledger = PrivacyLedger::new();
        ledger.add_entry("q1", 2.0, 0.0, CompositionType::Parallel);
        ledger.add_entry("q2", 3.0, 0.0, CompositionType::Parallel);
        assert!((ledger.parallel_max_epsilon() - 3.0).abs() < 1e-10);
    }
}
