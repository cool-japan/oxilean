//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AmariDivergence, ChernoffInformation, DistributedSourceCoder, FDivergence, FDivergenceKind,
    FiniteBlocklengthAnalyzer, MultiUserInfo, NaturalGradientDescent, NetworkCoding,
    QuantumChannelSimulator, QuantumEntropyEstimator, QuantumInfo, RateDistortion, RenyiEntropy,
    SmoothMinEntropy,
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `RenyiEntropy : Real → (List Real) → Real`
/// H_α(X) = (1/(1-α)) * log(Σ p_i^α), the Rényi entropy of order α.
/// Specializes to Shannon at α→1, min-entropy at α→∞.
pub fn renyi_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// `TsallisEntropy : Real → (List Real) → Real`
/// S_q(X) = (1/(q-1)) * (1 - Σ p_i^q), the Tsallis entropy of order q.
pub fn tsallis_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// `MinEntropy : (List Real) → Real`
/// H_∞(X) = -log(max_x p(x)), the min-entropy (worst-case unpredictability).
pub fn min_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `MaxEntropy : (List Real) → Real`
/// H_0(X) = log|supp(X)|, the max-entropy (logarithm of support size).
pub fn max_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `SmoothMinEntropy : Real → (List Real) → Real`
/// H_∞^ε(X): smooth min-entropy, maximized over ε-close distributions.
pub fn smooth_min_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// `SmoothMaxEntropy : Real → (List Real) → Real`
/// H_0^ε(X): smooth max-entropy, minimized over ε-close distributions.
pub fn smooth_max_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// `VonNeumannEntropy : (List (List Real)) → Real`
/// S(ρ) = -Tr(ρ log ρ) = -Σ λ_i log λ_i where λ_i are eigenvalues.
/// Takes density matrix encoded as a flattened list-of-rows.
pub fn von_neumann_entropy_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `QuantumMutualInfo : (List (List Real)) → Real`
/// I(A:B)_ρ = S(ρ_A) + S(ρ_B) - S(ρ_{AB}).
pub fn quantum_mutual_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `QuantumRelativeEntropy : (List (List Real)) → (List (List Real)) → Real`
/// D(ρ || σ) = Tr(ρ (log ρ - log σ)).
pub fn quantum_relative_entropy_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// `HSWChannelCapacity : (List (List Real)) → Real`
/// Holevo-Schumacher-Westmoreland capacity: C = max_{p_x, ρ_x} χ({p_x, ρ_x}),
/// where χ is the Holevo quantity S(Σ p_x N(ρ_x)) - Σ p_x S(N(ρ_x)).
/// Input: channel matrix (row = output state for each input basis).
pub fn hsw_channel_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `HolevoQuantity : (List Real) → (List (List (List Real))) → Real`
/// χ = S(Σ p_x ρ_x) - Σ p_x S(ρ_x); bounds accessible classical info.
pub fn holevo_quantity_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(list_ty(list_ty(real_ty()))), real_ty()),
    )
}
/// `QuantumFisherInfo : (List (List Real)) → Real`
/// F_Q(ρ, H) = 2 Σ_{j,k} |⟨j|H|k⟩|² (λ_j - λ_k)² / (λ_j + λ_k)
/// Quantifies sensitivity of ρ to perturbation; bounds metrology precision.
pub fn quantum_fisher_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `SLDFisherInfo : (List (List Real)) → Real`
/// F_SLD = Tr(ρ L²) where L is the symmetric logarithmic derivative.
pub fn sld_fisher_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `QuantumCramerRao : Prop`
/// Var(θ̂) ≥ 1 / (n F_Q(ρ_θ)) — quantum Cramér-Rao bound.
pub fn quantum_cramer_rao_ty() -> Expr {
    prop()
}
/// `FisherInformationMetric : (List Real) → Real`
/// g_{ij}(θ) = E[∂_i log p · ∂_j log p]; defines Riemannian metric on stat manifolds.
pub fn fisher_information_metric_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `GeometricEntropy : (List Real) → Real`
/// Entropy derived from the volume form of the Fisher-Rao metric on probability simplices.
pub fn geometric_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `AlphaConnection : Real → (List Real) → Real`
/// The α-connection on a statistical manifold; bridges e-connection (α=1) and m-connection (α=-1).
pub fn alpha_connection_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// `EntropyPower : Real → Real`
/// N(X) = (1/2πe) exp(2 h(X)) for continuous r.v. X with differential entropy h(X).
pub fn entropy_power_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `EntropyPowerInequality : Prop`
/// N(X+Y) ≥ N(X) + N(Y) for independent continuous random variables X, Y.
pub fn entropy_power_inequality_ty() -> Expr {
    prop()
}
/// `DeBruijnIdentity : Prop`
/// d/dt h(X + √t Z) = (1/2) J(X + √t Z) where J is Fisher information.
/// Connects differential entropy and Fisher information via Gaussian perturbation.
pub fn de_bruijn_identity_ty() -> Expr {
    prop()
}
/// `CostaEntropyPowerInequality : Prop`
/// Costa (1985): N(X + √t Z) is log-concave in t, strengthening Shannon's EPI.
pub fn costa_epi_ty() -> Expr {
    prop()
}
/// `SteinLemma : Real → Real → Real`
/// Stein's lemma: for H_0:P vs H_1:Q, error exponent = KL(P||Q).
/// Takes (type-I error bound ε, KL divergence) and returns exponent.
pub fn stein_lemma_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `ChernoffInformation : (List Real) → (List Real) → Real`
/// C(P,Q) = -min_{0≤λ≤1} log Σ p_i^λ q_i^(1-λ); optimal exponent in symmetric testing.
pub fn chernoff_information_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `ChernoffExponent : Prop`
/// Both error probabilities decay exponentially with exponent C(P,Q) in optimal testing.
pub fn chernoff_exponent_ty() -> Expr {
    prop()
}
/// `FDivergence : (Real → Real) → (List Real) → (List Real) → Real`
/// D_f(P||Q) = Σ q_i f(p_i / q_i); generalizes KL, TV, Hellinger, χ².
/// In kernel: function type approximated as Real → Real.
pub fn f_divergence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `TotalVariation : (List Real) → (List Real) → Real`
/// TV(P,Q) = (1/2) Σ |p_i - q_i| = (1/2) D_f(P||Q) with f(t) = |t-1|.
pub fn total_variation_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `HellingerDistance : (List Real) → (List Real) → Real`
/// H²(P,Q) = (1/2) Σ (√p_i - √q_i)² = 1 - Σ √(p_i q_i).
pub fn hellinger_distance_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `ChiSquaredDivergence : (List Real) → (List Real) → Real`
/// χ²(P||Q) = Σ (p_i - q_i)² / q_i; f-divergence with f(t) = (t-1)².
pub fn chi_squared_divergence_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `RenyiDivergence : Real → (List Real) → (List Real) → Real`
/// D_α(P||Q) = (1/(α-1)) log Σ p_i^α q_i^(1-α).
pub fn renyi_divergence_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `SibsonMutualInfo : Real → (List (List Real)) → Real`
/// I_α(X;Y) = min_Q D_α(P_{XY} || P_X ⊗ Q_Y); Sibson's Rényi mutual information.
pub fn sibson_mutual_info_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(list_ty(real_ty())), real_ty()))
}
/// `InteractionInformation : (List (List Real)) → Real`
/// II(X;Y;Z) = I(X;Y|Z) - I(X;Y) = I(X;Y;Z); can be negative (synergy).
pub fn interaction_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `MultiInformation : (List (List Real)) → Real`
/// TC(X_1,...,X_n) = Σ H(X_i) - H(X_1,...,X_n); total correlation (Watanabe).
pub fn multi_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `DualTotalCorrelation : (List (List Real)) → Real`
/// D(X_1,...,X_n) = H(X_1,...,X_n) - Σ H(X_i | X_j, j≠i); dual total correlation.
pub fn dual_total_correlation_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `KolmogorovComplexityConditional : Nat → Nat → Nat`
/// K(x|y) = min |p| : U(p, y) = x; conditional Kolmogorov complexity.
pub fn kolmogorov_complexity_conditional_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `AlgorithmicMutualInfo : Nat → Nat → Nat`
/// I(x:y) = K(x) + K(y) - K(x,y); algorithmic mutual information.
pub fn algorithmic_mutual_info_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `MDLPrinciple : Prop`
/// Minimum Description Length: optimal model minimizes K(model) + K(data|model).
pub fn mdl_principle_ty() -> Expr {
    prop()
}
/// `DirectedInformation : (List (List Real)) → Real`
/// I(X→Y) = Σ_{t=1}^n I(X^t ; Y_t | Y^{t-1}); measures causal flow X→Y.
pub fn directed_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `CausalConditionalEntropy : (List (List Real)) → Real`
/// H(Y || X) = Σ_t H(Y_t | Y^{t-1}, X^t); causal conditioning.
pub fn causal_conditional_entropy_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `KraussDirectedInfo : Prop`
/// Krauss (1987) and Massey (1990): directed information I(X→Y) ≥ 0 with equality iff Y ⊥ X.
pub fn krauss_directed_info_ty() -> Expr {
    prop()
}
/// `WynerZivRate : Real → Real → Real`
/// R_WZ(D) = min_{p(x̂|x,s)} I(X;X̂|S) − I(S;X̂) subject to distortion ≤ D.
/// Wyner-Ziv: source coding with side information at the decoder.
pub fn wyner_ziv_rate_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `GelfandPinskerCapacity : Real → Real`
/// C_GP = max_{p(u,x|s)} [ I(U;Y) − I(U;S) ]; channel coding with state info at encoder.
pub fn gelfand_pinsker_capacity_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `WiretapSecrecyCapacity : Real → Real → Real`
/// C_s = max_{p(x)} [I(X;Y_main) − I(X;Y_wiretap)]^+; Wyner wiretap channel.
pub fn wiretap_secrecy_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `SubadditivityEntropy : Prop`
/// H(X,Y) ≤ H(X) + H(Y); entropy subadditivity.
pub fn subadditivity_entropy_ty() -> Expr {
    prop()
}
/// `StrongSubadditivityQuantum : Prop`
/// S(ABC) + S(B) ≤ S(AB) + S(BC); SSA for von Neumann entropy (Lieb-Ruskai).
pub fn strong_subadditivity_quantum_ty() -> Expr {
    prop()
}
/// `PinskerInequality : Prop`
/// TV(P,Q)² ≤ (ln 2 / 2) * KL(P||Q); Pinsker's inequality.
pub fn pinsker_inequality_ty() -> Expr {
    prop()
}
/// `LogSumInequality : Prop`
/// Σ a_i log(a_i/b_i) ≥ (Σ a_i) log(Σ a_i / Σ b_i); used to prove convexity of KL.
pub fn log_sum_inequality_ty() -> Expr {
    prop()
}
/// `NaturalGradient : (List Real) → (List Real) → (List Real)`
/// The natural gradient ∇̃L = F⁻¹ ∇L where F is the Fisher information matrix.
/// Amari (1998): steepest ascent in the Riemannian manifold of distributions.
pub fn natural_gradient_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), list_ty(real_ty())),
    )
}
/// `AmariAlphaDivergence : Real → (List Real) → (List Real) → Real`
/// D^(α)(P||Q) = (4/(1-α²)) * (1 - Σ p_i^((1+α)/2) * q_i^((1-α)/2)).
/// Amari's α-divergence family; KL at α=±1, Hellinger at α=0.
pub fn amari_alpha_divergence_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// `JeffreysDivergence : (List Real) → (List Real) → Real`
/// J(P||Q) = KL(P||Q) + KL(Q||P); symmetric Jeffreys divergence.
pub fn jeffreys_divergence_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `FisherRaoDistance : (List Real) → (List Real) → Real`
/// d_FR(P, Q) = arccos(Σ √(p_i q_i)); geodesic distance on the probability simplex.
pub fn fisher_rao_distance_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `DualFlatConnection : Real → Prop`
/// Existence of dually flat (e,m)-connections on a statistical manifold at parameter α.
pub fn dual_flat_connection_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `CPTPMap : (List (List Real)) → (List (List Real)) → Bool`
/// Checks whether a linear map N: ρ ↦ Σ_k K_k ρ K_k† is completely positive and trace-preserving.
/// Input: input density matrix, Kraus operator data. Output: validity flag.
pub fn cptp_map_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), bool_ty()),
    )
}
/// `KrausRepresentation : (List (List (List Real))) → (List (List Real)) → (List (List Real))`
/// Apply a quantum channel given Kraus operators {K_k}: N(ρ) = Σ_k K_k ρ K_k†.
pub fn kraus_representation_ty() -> Expr {
    arrow(
        list_ty(list_ty(list_ty(real_ty()))),
        arrow(list_ty(list_ty(real_ty())), list_ty(list_ty(real_ty()))),
    )
}
/// `QuantumCapacityLLN : Prop`
/// LSD / quantum Shannon theorem: Q(N) = lim_{n→∞} (1/n) I_c(N^⊗n) where I_c is coherent information.
pub fn quantum_capacity_lln_ty() -> Expr {
    prop()
}
/// `PrivacyAmplification : Nat → Nat → Real → Nat`
/// Given raw key of length n, with min-entropy H_∞ ≥ k, and error ε, the
/// extractable key length is k - 2 log(1/ε).
pub fn privacy_amplification_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), nat_ty())))
}
/// `EntanglementFidelity : (List (List Real)) → (List (List Real)) → Real`
/// F_e(ρ, N) = ⟨ψ|(I ⊗ N)(|ψ⟩⟨ψ|)|ψ⟩; measures how well entanglement is preserved.
pub fn entanglement_fidelity_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// `MACCapacityRegion : Nat → Real → (List (List Real))`
/// Multiple-access channel capacity region for n senders with per-sender power P.
pub fn mac_capacity_region_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), list_ty(list_ty(real_ty()))))
}
/// `BroadcastCapacityRegion : Real → (List Real)`
/// Degraded broadcast channel capacity region (Cover 1972), parameterized by total power P.
pub fn broadcast_capacity_region_ty() -> Expr {
    arrow(real_ty(), list_ty(real_ty()))
}
/// `InterferenceChannelCapacity : Real → Real → Real`
/// Capacity region corner point for the Gaussian interference channel (simplified single number).
pub fn interference_channel_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `RelayChannelCapacity : Real → Real → Real`
/// Decode-and-forward lower bound for the relay channel (Cover-El Gamal 1979).
pub fn relay_channel_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `SecretKeyCapacity : Real → Real → Real`
/// S = I(X;Y) - I(X;Z) for a degraded wiretap channel (key agreement capacity).
pub fn secret_key_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `SlepianWolfRate : Real → Real → (List Real)`
/// Slepian-Wolf region: encode correlated sources (X,Y) without coordination.
/// R_X ≥ H(X|Y), R_Y ≥ H(Y|X), R_X + R_Y ≥ H(X,Y).
pub fn slepian_wolf_rate_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), list_ty(real_ty())))
}
/// `CorrelatedSourceCoding : Real → Real → Real`
/// Excess rate needed when jointly coding correlated sources X, Y.
pub fn correlated_source_coding_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `MultiterminalRateDistortion : Nat → Real → Real`
/// Rate-distortion function for n encoders observing a common source component.
pub fn multiterminal_rate_distortion_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `BergerTungScheme : Real → Real → Prop`
/// Achievability of the Berger-Tung inner bound for distributed source coding.
pub fn berger_tung_scheme_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `CapacityCostFunction : (Real → Real) → Real → Real`
/// C(P) = max_{p(x): E[cost] ≤ P} I(X;Y); capacity under an input cost constraint.
pub fn capacity_cost_function_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `RateDistortionMultiDescription : Nat → Real → Real`
/// MD (multiple description) coding rate for n descriptions with distortion D.
pub fn rate_distortion_multi_description_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `UniversalSourceCodingRate : Nat → Real → Real`
/// Lempel-Ziv universal coding rate: approaches H(X) with block length n at speed ~log n/n.
pub fn universal_source_coding_rate_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `ExcessRateExponent : Real → Real → Real`
/// Probability that LZ encoded length exceeds target R decays as exp(-n E(R, H)) for E > 0.
pub fn excess_rate_exponent_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `DataProcessingInequality : Prop`
/// I(X;Z) ≤ I(X;Y) whenever Z = f(Y) (post-processing cannot increase mutual information).
pub fn data_processing_inequality_ty() -> Expr {
    prop()
}
/// `ContractionCoefficient : (List (List Real)) → Real`
/// η(W) = sup_{P≠Q} TV(WP, WQ) / TV(P, Q); Dobrushin's contraction coefficient.
pub fn contraction_coefficient_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `HypercontractivityBound : Real → Real → Prop`
/// Bonami-Beckner inequality: ‖T_ρ f‖_q ≤ ‖f‖_p for ρ² ≤ (p-1)/(q-1).
pub fn hypercontractivity_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `LogarithmicSobolevInequality : Real → Prop`
/// Ent(f²) ≤ (2/ρ) E[|∇f|²]; characterizes the log-Sobolev constant ρ.
pub fn logarithmic_sobolev_inequality_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `StabilizerCode : Nat → Nat → Nat → Prop`
/// An [[n, k, d]] stabilizer code encodes k logical qubits into n physical qubits with distance d.
pub fn stabilizer_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `HammingBoundQuantum : Nat → Nat → Nat → Prop`
/// Quantum Hamming bound: Σ_{j=0}^{t} C(n,j) 3^j ≤ 2^{n-k} for an [[n,k,2t+1]] code.
pub fn hamming_bound_quantum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `ShorCode : Prop`
/// The Shor [[9,1,3]] code: first quantum error correcting code, corrects any single-qubit error.
pub fn shor_code_ty() -> Expr {
    prop()
}
/// `ToricCode : Nat → Prop`
/// The toric code on an n×n torus: topological quantum error correcting code (Kitaev 1997).
pub fn toric_code_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PolyanskiyPoorVerdu : Nat → Real → Real`
/// PPV (2010): second-order coding rate M* ≤ exp(nC - √n V Φ⁻¹(ε) + O(log n)).
pub fn polyanskiy_poor_verdu_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `ChannelDispersion : (List (List Real)) → Real`
/// V = Var[log(W(Y|X)/Q*(Y))]; channel dispersion governs second-order rate.
pub fn channel_dispersion_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `FiniteBlocklengthBound : Nat → Real → Real → Real`
/// Meta-converse / achievability bound at blocklength n, capacity C, dispersion V, error ε.
pub fn finite_blocklength_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// `OneShotCapacity : Real → (List (List Real)) → Real`
/// One-shot channel coding capacity at error ε: C^ε_1(W) in the one-shot regime.
pub fn one_shot_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(list_ty(real_ty())), real_ty()))
}
/// Register all extended information theory axioms in the kernel environment.
pub fn build_information_theory_ext_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("RenyiEntropy", renyi_entropy_ty()),
        ("TsallisEntropy", tsallis_entropy_ty()),
        ("MinEntropy", min_entropy_ty()),
        ("MaxEntropy", max_entropy_ty()),
        ("SmoothMinEntropy", smooth_min_entropy_ty()),
        ("SmoothMaxEntropy", smooth_max_entropy_ty()),
        ("VonNeumannEntropy", von_neumann_entropy_ty()),
        ("QuantumMutualInfo", quantum_mutual_info_ty()),
        ("QuantumRelativeEntropy", quantum_relative_entropy_ty()),
        ("HSWChannelCapacity", hsw_channel_capacity_ty()),
        ("HolevoQuantity", holevo_quantity_ty()),
        ("QuantumFisherInfo", quantum_fisher_info_ty()),
        ("SLDFisherInfo", sld_fisher_info_ty()),
        ("QuantumCramerRao", quantum_cramer_rao_ty()),
        ("FisherInformationMetric", fisher_information_metric_ty()),
        ("GeometricEntropy", geometric_entropy_ty()),
        ("AlphaConnection", alpha_connection_ty()),
        ("EntropyPower", entropy_power_ty()),
        ("EntropyPowerInequality", entropy_power_inequality_ty()),
        ("DeBruijnIdentity", de_bruijn_identity_ty()),
        ("CostaEntropyPowerInequality", costa_epi_ty()),
        ("SteinLemma", stein_lemma_ty()),
        ("ChernoffInformation", chernoff_information_ty()),
        ("ChernoffExponent", chernoff_exponent_ty()),
        ("FDivergence", f_divergence_ty()),
        ("TotalVariation", total_variation_ty()),
        ("HellingerDistance", hellinger_distance_ty()),
        ("ChiSquaredDivergence", chi_squared_divergence_ty()),
        ("RenyiDivergence", renyi_divergence_ty()),
        ("SibsonMutualInfo", sibson_mutual_info_ty()),
        ("InteractionInformation", interaction_information_ty()),
        ("MultiInformation", multi_information_ty()),
        ("DualTotalCorrelation", dual_total_correlation_ty()),
        (
            "KolmogorovComplexityConditional",
            kolmogorov_complexity_conditional_ty(),
        ),
        ("AlgorithmicMutualInfo", algorithmic_mutual_info_ty()),
        ("MDLPrinciple", mdl_principle_ty()),
        ("DirectedInformation", directed_information_ty()),
        ("CausalConditionalEntropy", causal_conditional_entropy_ty()),
        ("KraussDirectedInfo", krauss_directed_info_ty()),
        ("WynerZivRate", wyner_ziv_rate_ty()),
        ("GelfandPinskerCapacity", gelfand_pinsker_capacity_ty()),
        ("WiretapSecrecyCapacity", wiretap_secrecy_capacity_ty()),
        ("SubadditivityEntropy", subadditivity_entropy_ty()),
        (
            "StrongSubadditivityQuantum",
            strong_subadditivity_quantum_ty(),
        ),
        ("PinskerInequality", pinsker_inequality_ty()),
        ("LogSumInequality", log_sum_inequality_ty()),
        ("NaturalGradient", natural_gradient_ty()),
        ("AmariAlphaDivergence", amari_alpha_divergence_ty()),
        ("JeffreysDivergence", jeffreys_divergence_ty()),
        ("FisherRaoDistance", fisher_rao_distance_ty()),
        ("DualFlatConnection", dual_flat_connection_ty()),
        ("CPTPMap", cptp_map_ty()),
        ("KrausRepresentation", kraus_representation_ty()),
        ("QuantumCapacityLLN", quantum_capacity_lln_ty()),
        ("PrivacyAmplification", privacy_amplification_ty()),
        ("EntanglementFidelity", entanglement_fidelity_ty()),
        ("MACCapacityRegion", mac_capacity_region_ty()),
        ("BroadcastCapacityRegion", broadcast_capacity_region_ty()),
        (
            "InterferenceChannelCapacity",
            interference_channel_capacity_ty(),
        ),
        ("RelayChannelCapacity", relay_channel_capacity_ty()),
        ("SecretKeyCapacity", secret_key_capacity_ty()),
        ("SlepianWolfRate", slepian_wolf_rate_ty()),
        ("CorrelatedSourceCoding", correlated_source_coding_ty()),
        (
            "MultiterminalRateDistortion",
            multiterminal_rate_distortion_ty(),
        ),
        ("BergerTungScheme", berger_tung_scheme_ty()),
        ("CapacityCostFunction", capacity_cost_function_ty()),
        (
            "RateDistortionMultiDescription",
            rate_distortion_multi_description_ty(),
        ),
        (
            "UniversalSourceCodingRate",
            universal_source_coding_rate_ty(),
        ),
        ("ExcessRateExponent", excess_rate_exponent_ty()),
        ("DataProcessingInequality", data_processing_inequality_ty()),
        ("ContractionCoefficient", contraction_coefficient_ty()),
        ("HypercontractivityBound", hypercontractivity_bound_ty()),
        (
            "LogarithmicSobolevInequality",
            logarithmic_sobolev_inequality_ty(),
        ),
        ("StabilizerCode", stabilizer_code_ty()),
        ("HammingBoundQuantum", hamming_bound_quantum_ty()),
        ("ShorCode", shor_code_ty()),
        ("ToricCode", toric_code_ty()),
        ("PolyanskiyPoorVerdu", polyanskiy_poor_verdu_ty()),
        ("ChannelDispersion", channel_dispersion_ty()),
        ("FiniteBlocklengthBound", finite_blocklength_bound_ty()),
        ("OneShotCapacity", one_shot_capacity_ty()),
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
pub fn entropy(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|&&p| p > 1e-300)
        .map(|&p| -p * p.log2())
        .sum()
}
pub fn binary_entropy(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rate_distortion_binary_zero_distortion() {
        let src = vec![0.5, 0.5];
        let rd = RateDistortion::new(src, 0.0);
        let r = rd.rate_distortion_function();
        assert!((r - 1.0).abs() < 1e-6, "R(0) should be 1 bit, got {}", r);
    }
    #[test]
    fn test_rate_distortion_binary_max_distortion() {
        let src = vec![0.5, 0.5];
        let rd = RateDistortion::new(src, 0.5);
        let r = rd.rate_distortion_function();
        assert!(r < 1e-9, "R(0.5) should be 0, got {}", r);
    }
    #[test]
    fn test_blahut_arimoto_monotone() {
        let src = vec![0.5, 0.5];
        let rd = RateDistortion::new(src, 0.1);
        let curve = rd.blahut_arimoto();
        for &(rate, dist) in &curve {
            assert!(rate >= -1e-9, "rate {} is negative", rate);
            assert!(dist >= -1e-9, "dist {} is negative", dist);
        }
        assert!(!curve.is_empty());
    }
    #[test]
    fn test_mac_capacity_region_2senders() {
        let mac = MultiUserInfo::new(2, 1.0);
        let region = mac.mac_capacity_region();
        assert_eq!(region.len(), 4);
        let sum_cap = 3.0f64.log2();
        for point in &region {
            for &r in point {
                assert!(r >= -1e-9);
            }
        }
        let corner = &region[1];
        let sum: f64 = corner.iter().sum();
        assert!((sum - sum_cap).abs() < 1e-6, "sum {} != {}", sum, sum_cap);
    }
    #[test]
    fn test_broadcast_capacity_nonempty() {
        let bc = MultiUserInfo::new(2, 4.0);
        let region = bc.broadcast_capacity();
        assert_eq!(region.len(), 19);
        for &(c1, c2) in &region {
            assert!(c1 >= -1e-9);
            assert!(c2 >= -1e-9);
        }
    }
    #[test]
    fn test_max_flow_simple() {
        let nc = NetworkCoding::with_graph("0,1,3;1,2,2".to_string(), 0.0, 3, 0, 2, vec![3.0, 2.0]);
        let mf = nc.max_flow_min_cut();
        assert!((mf - 2.0).abs() < 1e-9, "max flow should be 2, got {}", mf);
    }
    #[test]
    fn test_achievable_rates_true() {
        let nc = NetworkCoding::with_graph("0,1,5;1,2,5".to_string(), 3.0, 3, 0, 2, vec![5.0, 5.0]);
        assert!(nc.achievable_rates());
    }
    #[test]
    fn test_achievable_rates_false() {
        let nc = NetworkCoding::with_graph("0,1,1;1,2,1".to_string(), 5.0, 3, 0, 2, vec![1.0, 1.0]);
        assert!(!nc.achievable_rates());
    }
    #[test]
    fn test_quantum_capacity_high_noise() {
        let qi = QuantumInfo::new(0.3);
        assert!((qi.quantum_capacity() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_quantum_capacity_low_noise() {
        let qi = QuantumInfo::new(0.0);
        assert!((qi.quantum_capacity() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_coherent_information_noiseless() {
        let qi = QuantumInfo::new(0.0);
        assert!((qi.coherent_information() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_hashing_bound_equals_quantum_capacity() {
        let qi = QuantumInfo::new(0.1);
        assert!((qi.hashing_bound() - qi.quantum_capacity()).abs() < 1e-12);
    }
    #[test]
    fn test_renyi_entropy_shannon_limit() {
        let probs = vec![0.5, 0.5];
        let re = RenyiEntropy::new(1.0);
        let h = re.compute(&probs);
        assert!((h - 1.0).abs() < 1e-9, "H_1 expected 1.0, got {h}");
    }
    #[test]
    fn test_renyi_entropy_order2_uniform() {
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let re = RenyiEntropy::new(2.0);
        let h = re.compute(&probs);
        assert!((h - 2.0).abs() < 1e-9, "H_2 expected 2.0, got {h}");
    }
    #[test]
    fn test_renyi_min_entropy() {
        let probs = vec![0.5, 0.3, 0.2];
        let h_inf = RenyiEntropy::min_entropy(&probs);
        assert!(
            (h_inf - 1.0).abs() < 1e-9,
            "H_inf expected 1.0, got {h_inf}"
        );
    }
    #[test]
    fn test_renyi_max_entropy_uniform() {
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let h0 = RenyiEntropy::max_entropy(&probs);
        assert!((h0 - 2.0).abs() < 1e-9, "H_0 expected 2.0, got {h0}");
    }
    #[test]
    fn test_renyi_smooth_min_entropy_zero_epsilon() {
        let probs = vec![0.5, 0.3, 0.2];
        let h = RenyiEntropy::smooth_min_entropy(&probs, 0.0);
        let expected = RenyiEntropy::min_entropy(&probs);
        assert!(
            (h - expected).abs() < 1e-9,
            "smooth H_inf(ε=0) should equal H_inf"
        );
    }
    #[test]
    fn test_fdivergence_kl_zero_self() {
        let p = vec![0.5, 0.3, 0.2];
        let fd = FDivergence::new(FDivergenceKind::KL);
        let d = fd.compute(&p, &p);
        assert!(d.abs() < 1e-9, "KL(p||p) expected 0, got {d}");
    }
    #[test]
    fn test_fdivergence_total_variation_identical() {
        let p = vec![0.4, 0.6];
        let tv = FDivergence::total_variation(&p, &p);
        assert!(tv.abs() < 1e-9, "TV of identical dists should be 0");
    }
    #[test]
    fn test_fdivergence_total_variation_orthogonal() {
        let p = vec![1.0, 0.0];
        let q = vec![0.0, 1.0];
        let tv = FDivergence::total_variation(&p, &q);
        assert!(
            (tv - 1.0).abs() < 1e-9,
            "TV of orthogonal dists should be 1, got {tv}"
        );
    }
    #[test]
    fn test_fdivergence_hellinger_identical() {
        let p = vec![0.5, 0.5];
        let h2 = FDivergence::hellinger_squared(&p, &p);
        assert!(h2.abs() < 1e-9, "H²(p,p) should be 0, got {h2}");
    }
    #[test]
    fn test_fdivergence_chi_squared_self_zero() {
        let p = vec![0.2, 0.5, 0.3];
        let chi = FDivergence::chi_squared(&p, &p);
        assert!(chi.abs() < 1e-9, "chi²(p||p) should be 0, got {chi}");
    }
    #[test]
    fn test_quantum_entropy_estimator_pure_state() {
        let est = QuantumEntropyEstimator::new(vec![1.0, 0.0, 0.0, 0.0]);
        let s = est.von_neumann_entropy();
        assert!(s.abs() < 1e-9, "S(pure) should be 0, got {s}");
        assert!(est.is_pure(1e-9));
    }
    #[test]
    fn test_quantum_entropy_estimator_maximally_mixed() {
        let est = QuantumEntropyEstimator::new(vec![0.25, 0.25, 0.25, 0.25]);
        let s = est.von_neumann_entropy();
        assert!(
            (s - 2.0).abs() < 1e-9,
            "S(maximally mixed 4d) should be 2, got {s}"
        );
    }
    #[test]
    fn test_chernoff_information_identical_distributions() {
        let p = vec![0.5, 0.5];
        let ci = ChernoffInformation::new(p.clone(), p);
        let c = ci.chernoff_information();
        assert!(c.abs() < 1e-6, "C(P,P) should be 0, got {c}");
    }
    #[test]
    fn test_chernoff_information_nonneg() {
        let p = vec![0.7, 0.3];
        let q = vec![0.3, 0.7];
        let ci = ChernoffInformation::new(p, q);
        let c = ci.chernoff_information();
        assert!(c >= -1e-9, "Chernoff info should be non-negative, got {c}");
    }
    #[test]
    fn test_renyi_divergence_order1_equals_kl() {
        let p = vec![0.6, 0.4];
        let q = vec![0.5, 0.5];
        let ci = ChernoffInformation::new(p.clone(), q.clone());
        let d1 = ci.renyi_divergence(1.0);
        let kl: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| if pi > 0.0 { pi * (pi / qi).log2() } else { 0.0 })
            .sum();
        assert!(
            (d1 - kl).abs() < 1e-6,
            "D_1 should equal KL, got d1={d1} kl={kl}"
        );
    }
    #[test]
    fn test_smooth_min_entropy_compute() {
        let probs = vec![0.5, 0.3, 0.2];
        let sme = SmoothMinEntropy::new(probs, 0.1);
        let h = sme.compute();
        assert!(
            h >= 1.0 - 1e-9,
            "smooth H_inf should be >= H_inf(original), got {h}"
        );
    }
    #[test]
    fn test_smooth_max_entropy_zero_epsilon() {
        let probs = vec![0.5, 0.3, 0.2];
        let sme = SmoothMinEntropy::new(probs, 0.0);
        let h0 = sme.smooth_max_entropy();
        assert!(
            (h0 - 3.0f64.log2()).abs() < 1e-9,
            "H_0^0 should be log2(3), got {h0}"
        );
    }
    #[test]
    fn test_build_information_theory_ext_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_information_theory_ext_env(&mut env);
        assert!(
            result.is_ok(),
            "build_information_theory_ext_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_natural_gradient_identity_fim() {
        let dim = 2;
        let fim = vec![1.0, 0.0, 0.0, 1.0];
        let ngd = NaturalGradientDescent::new(dim, fim, 1.0);
        let theta = vec![0.0, 0.0];
        let grad = vec![1.0, 2.0];
        let new_theta = ngd.step(&theta, &grad);
        assert!(
            (new_theta[0] - 1.0).abs() < 1e-4,
            "θ₀ expected ~1, got {}",
            new_theta[0]
        );
        assert!(
            (new_theta[1] - 2.0).abs() < 1e-4,
            "θ₁ expected ~2, got {}",
            new_theta[1]
        );
    }
    #[test]
    fn test_natural_gradient_norm_nonneg() {
        let dim = 3;
        let fim = vec![2.0, 0.5, 0.0, 0.5, 2.0, 0.5, 0.0, 0.5, 2.0];
        let ngd = NaturalGradientDescent::new(dim, fim, 0.01);
        let grad = vec![1.0, -1.0, 0.5];
        assert!(
            ngd.grad_norm(&grad) >= 0.0,
            "Riemannian norm should be non-negative"
        );
    }
    #[test]
    fn test_amari_alpha1_equals_kl() {
        let p = vec![0.6, 0.4];
        let q = vec![0.5, 0.5];
        let ad = AmariDivergence::new(1.0);
        let d = ad.compute(&p, &q);
        let kl: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| if pi > 0.0 { pi * (pi / qi).ln() } else { 0.0 })
            .sum();
        assert!(
            (d - kl).abs() < 1e-8,
            "α=1 should give KL, got d={d} kl={kl}"
        );
    }
    #[test]
    fn test_amari_jeffreys_symmetric() {
        let p = vec![0.3, 0.7];
        let q = vec![0.6, 0.4];
        let j1 = AmariDivergence::jeffreys(&p, &q);
        let j2 = AmariDivergence::jeffreys(&q, &p);
        assert!(
            (j1 - j2).abs() < 1e-9,
            "Jeffreys divergence should be symmetric"
        );
        assert!(j1 >= 0.0, "Jeffreys divergence should be non-negative");
    }
    #[test]
    fn test_fisher_rao_distance_identical() {
        let p = vec![0.25, 0.25, 0.25, 0.25];
        let d = AmariDivergence::fisher_rao_distance(&p, &p);
        assert!(d.abs() < 1e-9, "d_FR(P, P) should be 0, got {d}");
    }
    #[test]
    fn test_depolarizing_preserves_trace() {
        let qcs = QuantumChannelSimulator::new(0.1);
        let rho = vec![0.7, 0.1, 0.1, 0.3];
        let out = qcs.depolarizing(&rho);
        let trace = out[0] + out[3];
        assert!(
            (trace - 1.0).abs() < 1e-9,
            "trace after depolarizing should be 1, got {trace}"
        );
    }
    #[test]
    fn test_amplitude_damping_ground_state_invariant() {
        let qcs = QuantumChannelSimulator::new(0.3);
        let rho = vec![1.0, 0.0, 0.0, 0.0];
        let out = qcs.amplitude_damping(&rho);
        assert!((out[0] - 1.0).abs() < 1e-9);
        assert!((out[3] - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_depolarizing_zero_noise_identity() {
        let qcs = QuantumChannelSimulator::new(0.0);
        let rho = vec![0.8, 0.2, 0.2, 0.2];
        let out = qcs.depolarizing(&rho);
        for (o, r) in out.iter().zip(rho.iter()) {
            assert!((o - r).abs() < 1e-9, "zero noise should be identity");
        }
    }
    #[test]
    fn test_distributed_source_coder_symmetric() {
        let dsc = DistributedSourceCoder::new(0.5, 0.5);
        assert!(
            (dsc.mutual_information()).abs() < 1e-9,
            "I(X;Y) should be 0 when independent"
        );
    }
    #[test]
    fn test_distributed_source_coder_zero_noise() {
        let dsc = DistributedSourceCoder::new(0.5, 1e-12);
        let h_x_given_y = dsc.h_x_given_y();
        assert!(
            h_x_given_y < 1e-6,
            "H(X|Y) should be ~0 with zero noise, got {h_x_given_y}"
        );
    }
    #[test]
    fn test_slepian_wolf_achievability() {
        let dsc = DistributedSourceCoder::new(0.5, 0.1);
        let corners = dsc.slepian_wolf_corner_points();
        assert!(dsc.is_achievable(corners[0].0, corners[0].1));
        assert!(dsc.is_achievable(corners[1].0, corners[1].1));
        assert!(!dsc.is_achievable(0.0, 0.0));
    }
    #[test]
    fn test_finite_blocklength_approaches_capacity() {
        let fba = FiniteBlocklengthAnalyzer::new(1.0, 0.5, 0.01);
        let r_large = fba.effective_rate(10_000_000);
        assert!(
            (r_large - 1.0).abs() < 1e-3,
            "rate should approach capacity, got {r_large}"
        );
    }
    #[test]
    fn test_finite_blocklength_rate_gap_decreasing() {
        let fba = FiniteBlocklengthAnalyzer::new(1.0, 0.5, 0.01);
        let gap_100 = fba.rate_gap(100);
        let gap_1000 = fba.rate_gap(1000);
        assert!(
            gap_100 >= gap_1000 - 1e-9,
            "rate gap should decrease as n grows"
        );
    }
    #[test]
    fn test_finite_blocklength_min_blocklength() {
        let fba = FiniteBlocklengthAnalyzer::new(1.0, 0.5, 0.01);
        let n = fba.min_blocklength(0.9);
        assert!(n > 0, "min blocklength should be positive");
        assert!(
            n < usize::MAX,
            "min blocklength should be finite for rate < capacity"
        );
    }
}
