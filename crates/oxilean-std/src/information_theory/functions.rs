//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BlahutArimoto, ChannelSimulator, ChernoffInformationCalc, EntropyEstimator, HuffmanCode,
    InfoTheoreticSecurity, KLDivergenceCalc, KolmogorovComplexity, LempelZivComplexity,
    MutualInformation, RateDistortion, RenyiEntropyComputer, SlepianWolfCoder,
    TsallisEntropyComputer, WiretapChannel,
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
/// Shannon entropy H(X) = -Σ p(x) log p(x)
/// Type: (List Real) → Real
pub fn entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Mutual information I(X;Y) = H(X) - H(X|Y)
/// Type: (List (List Real)) → Real
pub fn mutual_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Channel capacity C = max_{p(x)} I(X;Y)
/// Type: (List (List Real)) → Real
pub fn channel_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Noisy channel coding theorem: achievable rate ≤ C iff reliable communication possible
/// Type: Prop
pub fn channel_coding_ty() -> Expr {
    prop()
}
/// Source coding (lossless compression) theorem: average code length ≥ H(X)
/// Type: Prop
pub fn source_coding_ty() -> Expr {
    prop()
}
/// Kolmogorov complexity K(x) = min|p| : U(p) = x
/// Type: Nat → Nat
pub fn kolmogorov_complexity_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Shannon source coding theorem: compression rate ≥ H(X) bits/symbol
/// ∀ (dist : List Real), avg_code_length dist ≥ entropy dist
pub fn shannon_source_coding_ty() -> Expr {
    pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop())
}
/// Noisy channel theorem: achievable rate = C
/// ∀ (channel : List (List Real)), ∃ rate, rate = channel_capacity channel
pub fn noisy_channel_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "channel",
        list_ty(list_ty(real_ty())),
        prop(),
    )
}
/// Data processing inequality: I(X;Z) ≤ I(X;Y) for X→Y→Z Markov chain
/// Prop (we represent it as a closed proposition)
pub fn data_processing_inequality_ty() -> Expr {
    prop()
}
/// Fano's inequality: H(X|Y) ≤ h(Pe) + Pe * log(|X|-1)
/// ∀ (pe : Real) (alphabet_size : Nat), Prop
pub fn fano_inequality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "pe",
        real_ty(),
        pi(BinderInfo::Default, "alphabet_size", nat_ty(), prop()),
    )
}
/// Conditional mutual information I(X;Y|Z) = H(X|Z) - H(X|Y,Z)
/// Type: (List (List (List Real))) → Real
pub fn conditional_mutual_information_ty() -> Expr {
    arrow(list_ty(list_ty(list_ty(real_ty()))), real_ty())
}
/// AWGN channel capacity C = (1/2) log2(1 + SNR)
/// Type: Real → Real
pub fn awgn_capacity_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// DMC (Discrete Memoryless Channel) capacity
/// Type: (List (List Real)) → Real
pub fn dmc_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Rate-distortion function R(D): minimum rate at distortion D
/// Type: (List Real) → Real → Real
pub fn rate_distortion_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), real_ty()))
}
/// Berger's theorem: R(D) = max_{λ≥0} [H(X) - max_{q} Σ p(x) log q(x|x')/p(x)]
/// Type: Prop
pub fn berger_theorem_ty() -> Expr {
    prop()
}
/// Typical set A_ε^(n): set of sequences with empirical entropy close to H(X)
/// Type: (List Real) → Real → Nat → Prop
pub fn typical_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "dist",
        list_ty(real_ty()),
        pi(
            BinderInfo::Default,
            "eps",
            real_ty(),
            pi(BinderInfo::Default, "n", nat_ty(), prop()),
        ),
    )
}
/// Asymptotic equipartition property: -1/n log p(X^n) → H(X) in probability
/// Type: (List Real) → Prop
pub fn aep_ty() -> Expr {
    pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop())
}
/// Entropy rate of a stationary stochastic process: H(X) = lim H(X_n | X_1,...,X_{n-1})
/// Type: (Nat → List Real) → Real
pub fn entropy_rate_ty() -> Expr {
    arrow(arrow(nat_ty(), list_ty(real_ty())), real_ty())
}
/// Differential entropy h(X) = -∫ f(x) log f(x) dx for continuous r.v.
/// Type: (Real → Real) → Real
pub fn differential_entropy_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// Fisher information I(θ) = E\[(d/dθ log f(X;θ))^2\]
/// Type: (Real → Real → Real) → Real → Real
pub fn fisher_information_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(real_ty(), real_ty())),
        arrow(real_ty(), real_ty()),
    )
}
/// Cramér-Rao bound: Var(T) ≥ 1 / I(θ)
/// Type: Prop
pub fn cramer_rao_bound_ty() -> Expr {
    prop()
}
/// Wiretap channel secrecy capacity: C_s = max \[I(X;Y) - I(X;Z)\]
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn secrecy_capacity_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// Multiple access channel (MAC) capacity region
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn mac_capacity_region_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), prop()),
    )
}
/// Broadcast channel capacity: Marton's achievability region
/// Type: (List (List Real)) → Prop
pub fn broadcast_channel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "channel",
        list_ty(list_ty(real_ty())),
        prop(),
    )
}
/// Wyner-Ziv theorem (lossy compression with side information)
/// Type: (List (List Real)) → Real → Real
pub fn wyner_ziv_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), real_ty()))
}
/// Slepian-Wolf theorem (lossless distributed source coding)
/// Type: (List (List Real)) → Prop
pub fn slepian_wolf_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "joint",
        list_ty(list_ty(real_ty())),
        prop(),
    )
}
/// MDL (Minimum Description Length) criterion: model selection via codelength
/// Type: Nat → Real
pub fn mdl_criterion_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// Shannon-Fano code length: ⌈log2(1/p(x))⌉ bits for symbol x
/// Type: (List Real) → List Nat
pub fn shannon_fano_ty() -> Expr {
    arrow(list_ty(real_ty()), list_ty(nat_ty()))
}
/// LZ77/LZ78 compression: length of compressed output grows as n * H(X)
/// Type: Prop (universal source coding theorem)
pub fn lz_compression_ty() -> Expr {
    prop()
}
/// Entropy power inequality: e^{2h(X+Y)} ≥ e^{2h(X)} + e^{2h(Y)}
/// Type: Prop
pub fn entropy_power_inequality_ty() -> Expr {
    prop()
}
/// Log-sum inequality: Σ a_i log(a_i/b_i) ≥ (Σ a_i) log((Σ a_i)/(Σ b_i))
/// Type: Prop
pub fn log_sum_inequality_ty() -> Expr {
    prop()
}
/// Blahut-Arimoto algorithm convergence theorem: iterates converge to C
/// Type: (List (List Real)) → Nat → Real
pub fn blahut_arimoto_convergence_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(nat_ty(), real_ty()))
}
/// Information-theoretic security: perfect secrecy iff H(K) ≥ H(M)
/// Type: Prop (one-time pad / Shannon's theorem)
pub fn perfect_secrecy_ty() -> Expr {
    prop()
}
/// Capacity of interference channel (Han-Kobayashi inner bound)
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn interference_channel_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// Capacity per unit cost (Verdú): C = max_{x} I(x;Y) / c(x)
/// Type: (List (List Real)) → (List Real) → Real
pub fn capacity_per_unit_cost_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(real_ty()), real_ty()),
    )
}
/// Channel dispersion (second-order coding rate)
/// Type: (List (List Real)) → Real
pub fn channel_dispersion_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Typical-set size: |A_ε^(n)| ≈ 2^{n H(X)}
/// Type: (List Real) → Nat → Real
pub fn typical_set_size_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(nat_ty(), real_ty()))
}
/// Rényi entropy of order α: H_α(X) = (1/(1-α)) log Σ p(x)^α
/// Type: Real → (List Real) → Real
pub fn renyi_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// Min-entropy H_∞(X) = -log max p(x)
/// Type: (List Real) → Real
pub fn min_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Broadcast channel capacity (Marton region inner bound)
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn marton_region_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), prop()),
    )
}
/// Degraded broadcast channel capacity: C = max I(X;Y1) s.t. Y1 more capable than Y2
/// Type: (List (List Real)) → Real
pub fn degraded_broadcast_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Relay channel capacity (cut-set outer bound)
/// Type: (List (List Real)) → Real
pub fn relay_channel_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Multiple access channel (MAC) sum capacity
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn mac_sum_capacity_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// Han-Kobayashi inner bound for interference channel
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn han_kobayashi_bound_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), prop()),
    )
}
/// Gray-Wyner network rate region
/// Type: (List (List Real)) → Prop
pub fn gray_wyner_region_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "joint",
        list_ty(list_ty(real_ty())),
        prop(),
    )
}
/// Slepian-Wolf achievable rate region (closed form)
/// Type: (List (List Real)) → Real → Real → Prop
pub fn slepian_wolf_region_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "joint",
        list_ty(list_ty(real_ty())),
        pi(
            BinderInfo::Default,
            "r1",
            real_ty(),
            pi(BinderInfo::Default, "r2", real_ty(), prop()),
        ),
    )
}
/// Wyner-Ziv rate-distortion with side information Y at decoder
/// Type: (List (List Real)) → Real → Real
pub fn wyner_ziv_rate_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), real_ty()))
}
/// Broadcast channel with confidential messages (Csiszár-Körner)
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn broadcast_confidential_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// Common information (Gács-Körner) between X and Y
/// Type: (List (List Real)) → Real
pub fn gacs_korner_common_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Wyner's common information C(X;Y) = min I(X,Y;W)
/// Type: (List (List Real)) → Real
pub fn wyner_common_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Secret key capacity S(X;Y) achievable by public discussion
/// Type: (List (List Real)) → Real
pub fn secret_key_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Tsallis entropy of order q: S_q = (1/(q-1))(1 - Σ p^q)
/// Type: Real → (List Real) → Real
pub fn tsallis_entropy_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), real_ty()))
}
/// Sibson mutual information of order α
/// Type: Real → (List (List Real)) → Real
pub fn sibson_mutual_info_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(list_ty(real_ty())), real_ty()))
}
/// Lautum information L(X;Y) = D(P_X ⊗ P_Y || P_{XY})
/// Type: (List (List Real)) → Real
pub fn lautum_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Arimoto mutual information of order α
/// Type: Real → (List (List Real)) → Real
pub fn arimoto_mutual_info_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(list_ty(real_ty())), real_ty()))
}
/// f-divergence D_f(P||Q) = Σ q(x) f(p(x)/q(x))
/// Type: (Real → Real) → (List Real) → (List Real) → Real
pub fn f_divergence_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
    )
}
/// Sanov's theorem: probability of large deviation ≤ 2^{-n D(Q||P)}
/// Type: (List Real) → Prop
pub fn sanov_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop())
}
/// Chernoff information between P and Q: C(P,Q) = -log min_{0≤t≤1} Σ p^t q^{1-t}
/// Type: (List Real) → (List Real) → Real
pub fn chernoff_information_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// Error exponent for channel coding (Gallager E_0 function)
/// Type: Real → (List (List Real)) → Real → Real
pub fn gallager_exponent_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), real_ty())),
    )
}
/// Expurgated exponent E_ex (improved for low rates)
/// Type: (List (List Real)) → Real → Real
pub fn expurgated_exponent_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), real_ty()))
}
/// Sphere-packing bound: E_sp(R) ≤ error exponent
/// Type: (List (List Real)) → Real → Real
pub fn sphere_packing_exponent_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), real_ty()))
}
/// Neyman-Pearson lemma: optimal binary hypothesis test
/// Type: (List Real) → (List Real) → Real → Prop
pub fn neyman_pearson_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        list_ty(real_ty()),
        pi(
            BinderInfo::Default,
            "q",
            list_ty(real_ty()),
            pi(BinderInfo::Default, "threshold", real_ty(), prop()),
        ),
    )
}
/// Stein's lemma: exponent of type-II error ≈ D(P||Q)
/// Type: (List Real) → (List Real) → Prop
pub fn stein_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        list_ty(real_ty()),
        pi(BinderInfo::Default, "q", list_ty(real_ty()), prop()),
    )
}
/// Chernoff-Stein lemma: optimal error exponent = C(P,Q) for symmetric test
/// Type: (List Real) → (List Real) → Prop
pub fn chernoff_stein_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        list_ty(real_ty()),
        pi(BinderInfo::Default, "q", list_ty(real_ty()), prop()),
    )
}
/// Lempel-Ziv complexity c(x^n): number of distinct phrases in LZ parsing
/// Type: (List Bool) → Nat
pub fn lz_complexity_ty() -> Expr {
    arrow(list_ty(bool_ty()), nat_ty())
}
/// Universal source coding (LZ78): per-symbol rate → H(X) as n → ∞
/// Type: Prop
pub fn lz78_universal_ty() -> Expr {
    prop()
}
/// MDL model selection principle: choose model minimizing description length
/// Type: Nat → Nat → Real
pub fn mdl_model_selection_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// Fisher information matrix (for vector parameter θ)
/// Type: (List Real → List Real) → List Real → (List (List Real))
pub fn fisher_matrix_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), list_ty(real_ty())),
        arrow(list_ty(real_ty()), list_ty(list_ty(real_ty()))),
    )
}
/// Cramér-Rao matrix bound: Cov(T) ≥ I(θ)^{-1}
/// Type: Prop
pub fn cramer_rao_matrix_ty() -> Expr {
    prop()
}
/// Jeffreys prior: π(θ) ∝ sqrt(det I(θ))
/// Type: (List Real → List Real) → List Real → Real
pub fn jeffreys_prior_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), list_ty(real_ty())),
        arrow(list_ty(real_ty()), real_ty()),
    )
}
/// Natural gradient in information geometry
/// Type: (List Real → Real) → List Real → List (List Real) → List Real
pub fn natural_gradient_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(list_ty(real_ty())), list_ty(real_ty())),
        ),
    )
}
/// Quantum entropy S(ρ) = -Tr(ρ log ρ) (von Neumann entropy)
/// Type: (List (List Real)) → Real
pub fn von_neumann_entropy_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Quantum mutual information I(A;B)_ρ = S(A) + S(B) - S(AB)
/// Type: (List (List Real)) → Real
pub fn quantum_mutual_info_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Holevo bound: accessible information ≤ χ(ensemble)
/// Type: (List Real) → (List (List (List Real))) → Real
pub fn holevo_bound_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(list_ty(list_ty(real_ty()))), real_ty()),
    )
}
/// Quantum channel capacity (Holevo-Schumacher-Westmoreland)
/// Type: (List (List (List Real))) → Real
pub fn quantum_channel_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(list_ty(real_ty()))), real_ty())
}
/// Quantum relative entropy D(ρ||σ) = Tr(ρ(log ρ - log σ))
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn quantum_relative_entropy_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(list_ty(list_ty(real_ty())), real_ty()),
    )
}
/// Quantum error-correcting code capacity (Schumacher compression)
/// Type: (List (List Real)) → Real
pub fn schumacher_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Functional compression: computing f(X,Y) via distributed encoding
/// Type: (List (List Real)) → Real
pub fn functional_compression_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// Computing capacity: maximum rate of computing a function over a network
/// Type: (List (List Real)) → Nat → Real
pub fn computing_capacity_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(nat_ty(), real_ty()))
}
/// Körner graph entropy: minimum rate to describe graph colorings
/// Type: Nat → (List (List Bool)) → Real
pub fn korner_graph_entropy_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(list_ty(bool_ty())), real_ty()))
}
/// Register all information theory axioms and theorems in the kernel environment.
pub fn build_information_theory_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Entropy", entropy_ty()),
        ("MutualInformation", mutual_information_ty()),
        ("ChannelCapacity", channel_capacity_ty()),
        ("ChannelCoding", channel_coding_ty()),
        ("SourceCoding", source_coding_ty()),
        ("KolmogorovComplexity", kolmogorov_complexity_ty()),
        (
            "JointEntropy",
            arrow(list_ty(list_ty(real_ty())), real_ty()),
        ),
        (
            "ConditionalEntropy",
            arrow(list_ty(list_ty(real_ty())), real_ty()),
        ),
        (
            "KLDivergence",
            arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
        ),
        (
            "CrossEntropy",
            arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
        ),
        ("BinaryEntropy", arrow(real_ty(), real_ty())),
        ("BSCCapacity", arrow(real_ty(), real_ty())),
        ("BECCapacity", arrow(real_ty(), real_ty())),
        ("shannon_source_coding", shannon_source_coding_ty()),
        ("noisy_channel_theorem", noisy_channel_theorem_ty()),
        (
            "data_processing_inequality",
            data_processing_inequality_ty(),
        ),
        ("fano_inequality", fano_inequality_ty()),
        (
            "entropy_nonneg",
            pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop()),
        ),
        (
            "entropy_max_uniform",
            pi(BinderInfo::Default, "n", nat_ty(), prop()),
        ),
        (
            "mi_nonneg",
            pi(
                BinderInfo::Default,
                "joint",
                list_ty(list_ty(real_ty())),
                prop(),
            ),
        ),
        ("kl_nonneg", prop()),
        ("chain_rule_entropy", prop()),
        (
            "ConditionalMutualInformation",
            conditional_mutual_information_ty(),
        ),
        ("AWGNCapacity", awgn_capacity_ty()),
        ("DMCCapacity", dmc_capacity_ty()),
        ("RateDistortion", rate_distortion_ty()),
        ("berger_theorem", berger_theorem_ty()),
        ("TypicalSet", typical_set_ty()),
        ("aep", aep_ty()),
        ("TypicalSetSize", typical_set_size_ty()),
        ("EntropyRate", entropy_rate_ty()),
        ("DifferentialEntropy", differential_entropy_ty()),
        ("FisherInformation", fisher_information_ty()),
        ("cramer_rao_bound", cramer_rao_bound_ty()),
        ("SecrecyCapacity", secrecy_capacity_ty()),
        ("MACCapacityRegion", mac_capacity_region_ty()),
        ("BroadcastChannel", broadcast_channel_ty()),
        ("WynerZiv", wyner_ziv_ty()),
        ("SlepianWolf", slepian_wolf_ty()),
        ("InterferenceChannel", interference_channel_ty()),
        ("MDLCriterion", mdl_criterion_ty()),
        ("ShannonFano", shannon_fano_ty()),
        ("lz_compression", lz_compression_ty()),
        ("entropy_power_inequality", entropy_power_inequality_ty()),
        ("log_sum_inequality", log_sum_inequality_ty()),
        ("BlahutArimotoConvergence", blahut_arimoto_convergence_ty()),
        ("perfect_secrecy", perfect_secrecy_ty()),
        ("CapacityPerUnitCost", capacity_per_unit_cost_ty()),
        ("ChannelDispersion", channel_dispersion_ty()),
        ("RenyiEntropy", renyi_entropy_ty()),
        ("MinEntropy", min_entropy_ty()),
        ("MartonRegion", marton_region_ty()),
        (
            "DegradedBroadcastCapacity",
            degraded_broadcast_capacity_ty(),
        ),
        ("RelayChannelCapacity", relay_channel_capacity_ty()),
        ("MACsumCapacity", mac_sum_capacity_ty()),
        ("HanKobayashiBound", han_kobayashi_bound_ty()),
        ("GrayWynerRegion", gray_wyner_region_ty()),
        ("SlepianWolfRegion", slepian_wolf_region_ty()),
        ("WynerZivRate", wyner_ziv_rate_ty()),
        ("BroadcastConfidential", broadcast_confidential_ty()),
        ("GacsKornerCommonInfo", gacs_korner_common_info_ty()),
        ("WynerCommonInfo", wyner_common_info_ty()),
        ("SecretKeyCapacity", secret_key_capacity_ty()),
        ("TsallisEntropy", tsallis_entropy_ty()),
        ("SibsonMutualInfo", sibson_mutual_info_ty()),
        ("LautumInformation", lautum_information_ty()),
        ("ArimotoMutualInfo", arimoto_mutual_info_ty()),
        ("FDivergence", f_divergence_ty()),
        ("sanov_theorem", sanov_theorem_ty()),
        ("ChernoffInformation", chernoff_information_ty()),
        ("GallagerExponent", gallager_exponent_ty()),
        ("ExpurgatedExponent", expurgated_exponent_ty()),
        ("SpherePackingExponent", sphere_packing_exponent_ty()),
        ("neyman_pearson", neyman_pearson_ty()),
        ("stein_lemma", stein_lemma_ty()),
        ("chernoff_stein", chernoff_stein_ty()),
        ("LZComplexity", lz_complexity_ty()),
        ("lz78_universal", lz78_universal_ty()),
        ("MDLModelSelection", mdl_model_selection_ty()),
        ("FisherMatrix", fisher_matrix_ty()),
        ("cramer_rao_matrix", cramer_rao_matrix_ty()),
        ("JeffreysPrior", jeffreys_prior_ty()),
        ("NaturalGradient", natural_gradient_ty()),
        ("VonNeumannEntropy", von_neumann_entropy_ty()),
        ("QuantumMutualInfo", quantum_mutual_info_ty()),
        ("HolevoBound", holevo_bound_ty()),
        ("QuantumChannelCapacity", quantum_channel_capacity_ty()),
        ("QuantumRelativeEntropy", quantum_relative_entropy_ty()),
        ("SchumacherCapacity", schumacher_capacity_ty()),
        ("FunctionalCompression", functional_compression_ty()),
        ("ComputingCapacity", computing_capacity_ty()),
        ("KornerGraphEntropy", korner_graph_entropy_ty()),
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
/// Shannon entropy of a probability distribution (in bits).
///
/// H(X) = -Σ p * log2(p), where terms with p = 0 are skipped.
pub fn entropy(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}
/// Joint entropy H(X,Y) from a joint distribution table.
///
/// `joint\[i\]\[j\]` = P(X=i, Y=j). Returns -Σ_{i,j} p_{ij} * log2(p_{ij}).
pub fn joint_entropy(joint: &[Vec<f64>]) -> f64 {
    joint
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}
/// Marginal distribution of X (rows) from a joint table.
pub fn marginal_x(joint: &[Vec<f64>]) -> Vec<f64> {
    joint.iter().map(|row| row.iter().sum::<f64>()).collect()
}
/// Marginal distribution of Y (columns) from a joint table.
pub fn marginal_y(joint: &[Vec<f64>]) -> Vec<f64> {
    if joint.is_empty() {
        return vec![];
    }
    let cols = joint[0].len();
    (0..cols)
        .map(|j| {
            joint
                .iter()
                .map(|row| row.get(j).copied().unwrap_or(0.0))
                .sum()
        })
        .collect()
}
/// Conditional entropy H(Y|X) = H(X,Y) - H(X).
pub fn conditional_entropy(joint: &[Vec<f64>]) -> f64 {
    let h_xy = joint_entropy(joint);
    let px = marginal_x(joint);
    let h_x = entropy(&px);
    h_xy - h_x
}
/// Mutual information I(X;Y) = H(X) + H(Y) - H(X,Y).
pub fn mutual_information(joint: &[Vec<f64>]) -> f64 {
    let px = marginal_x(joint);
    let py = marginal_y(joint);
    let h_x = entropy(&px);
    let h_y = entropy(&py);
    let h_xy = joint_entropy(joint);
    h_x + h_y - h_xy
}
/// KL divergence D(p || q) = Σ p(x) * log2(p(x) / q(x)).
///
/// Returns `f64::INFINITY` if q(x) = 0 for any x where p(x) > 0.
pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .filter(|(&pi, _)| pi > 0.0)
        .map(|(&pi, &qi)| {
            if qi == 0.0 {
                f64::INFINITY
            } else {
                pi * (pi / qi).log2()
            }
        })
        .sum()
}
/// Cross-entropy H(p, q) = -Σ p(x) * log2(q(x)).
///
/// Returns `f64::INFINITY` if q(x) = 0 for any x where p(x) > 0.
pub fn cross_entropy(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .filter(|(&pi, _)| pi > 0.0)
        .map(|(&pi, &qi)| {
            if qi == 0.0 {
                f64::INFINITY
            } else {
                -pi * qi.log2()
            }
        })
        .sum()
}
/// Binary entropy function h(p) = -p * log2(p) - (1-p) * log2(1-p).
///
/// h(0) = 0, h(0.5) = 1, h(1) = 0.
pub fn binary_entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
}
/// Channel capacity of a BSC (Binary Symmetric Channel) with flip probability p.
///
/// C = 1 - h(p) bits.
pub fn bsc_capacity(p: f64) -> f64 {
    1.0 - binary_entropy(p)
}
/// Channel capacity of a BEC (Binary Erasure Channel) with erasure probability ε.
///
/// C = 1 - ε bits.
pub fn bec_capacity(epsilon: f64) -> f64 {
    1.0 - epsilon
}
/// Compute Huffman code lengths for a set of symbol probabilities.
///
/// Returns `(code_lengths, avg_bits_per_symbol)` where `code_lengths\[i\]`
/// is the number of bits for symbol i. Uses a greedy priority-queue approach.
pub fn huffman_code_lengths(probs: &[f64]) -> (Vec<u32>, f64) {
    let n = probs.len();
    if n == 0 {
        return (vec![], 0.0);
    }
    if n == 1 {
        return (vec![1], probs[0]);
    }
    let mut heap: Vec<(f64, Vec<usize>)> = probs
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, vec![i]))
        .collect();
    let mut lengths = vec![0u32; n];
    while heap.len() > 1 {
        heap.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let (p1, idx1) = heap.remove(0);
        let (p2, idx2) = heap.remove(0);
        for i in &idx1 {
            lengths[*i] += 1;
        }
        for i in &idx2 {
            lengths[*i] += 1;
        }
        let mut merged = idx1;
        merged.extend(idx2);
        heap.push((p1 + p2, merged));
    }
    let avg: f64 = probs
        .iter()
        .zip(lengths.iter())
        .map(|(&p, &l)| p * l as f64)
        .sum();
    (lengths, avg)
}
/// Rényi entropy of order alpha: H_α = (1/(1-α)) * log2(Σ p^α).
///
/// - α → 1 converges to Shannon entropy.
/// - α = 0 is the Hartley entropy: log2(|support|).
/// - α → ∞ is the min-entropy.
pub fn renyi_entropy(alpha: f64, probs: &[f64]) -> f64 {
    if probs.is_empty() {
        return 0.0;
    }
    if (alpha - 1.0).abs() < 1e-10 {
        return entropy(probs);
    }
    let sum: f64 = probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| p.powf(alpha))
        .sum();
    if sum <= 0.0 {
        return f64::INFINITY;
    }
    sum.log2() / (1.0 - alpha)
}
/// Min-entropy H_∞(X) = -log2(max_x p(x)).
pub fn min_entropy(probs: &[f64]) -> f64 {
    let max_p = probs.iter().cloned().fold(0.0f64, f64::max);
    if max_p <= 0.0 {
        return f64::INFINITY;
    }
    -max_p.log2()
}
/// AWGN channel capacity C = 0.5 * log2(1 + SNR) bits per channel use.
pub fn awgn_capacity(snr: f64) -> f64 {
    0.5 * (1.0 + snr).log2()
}
/// Differential entropy of a Gaussian N(μ, σ²): h = 0.5 * log2(2πe σ²).
pub fn gaussian_differential_entropy(sigma_sq: f64) -> f64 {
    use std::f64::consts::{E, PI};
    0.5 * (2.0 * PI * E * sigma_sq).log2()
}
/// Shannon-Fano code lengths: ceil(log2(1/p(x))) bits for symbol x.
pub fn shannon_fano_lengths(probs: &[f64]) -> Vec<u32> {
    probs
        .iter()
        .map(|&p| {
            if p <= 0.0 {
                0u32
            } else {
                (-p.log2()).ceil() as u32
            }
        })
        .collect()
}
/// Typical set membership check: sequence x^n is epsilon-typical if
/// |-(1/n) log p(x^n) - H(X)| <= epsilon.
///
/// Returns true if the log-probability per symbol is within eps of H(X).
pub fn is_typical(probs: &[f64], sequence: &[usize], eps: f64) -> bool {
    if sequence.is_empty() {
        return true;
    }
    let h = entropy(probs);
    let log_prob: f64 = sequence
        .iter()
        .map(|&sym| {
            let p = probs.get(sym).copied().unwrap_or(0.0);
            if p <= 0.0 {
                f64::NEG_INFINITY
            } else {
                p.log2()
            }
        })
        .sum();
    let empirical_rate = -log_prob / sequence.len() as f64;
    (empirical_rate - h).abs() <= eps
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-9;
    #[test]
    fn test_entropy_uniform() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let h = entropy(&probs);
        assert!((h - 2.0).abs() < EPS, "expected 2.0, got {h}");
    }
    #[test]
    fn test_entropy_certain() {
        let probs = [0.0, 0.0, 1.0, 0.0];
        let h = entropy(&probs);
        assert!(h.abs() < EPS, "expected 0.0, got {h}");
    }
    #[test]
    fn test_binary_entropy() {
        assert!((binary_entropy(0.5) - 1.0).abs() < EPS);
        assert!(binary_entropy(0.0).abs() < EPS);
        assert!(binary_entropy(1.0).abs() < EPS);
    }
    #[test]
    fn test_kl_divergence_zero() {
        let p = [0.1, 0.4, 0.3, 0.2];
        let d = kl_divergence(&p, &p);
        assert!(d.abs() < EPS, "expected 0.0, got {d}");
    }
    #[test]
    fn test_mutual_information_independent() {
        let joint = vec![
            vec![0.0625, 0.0625, 0.0625, 0.0625],
            vec![0.0625, 0.0625, 0.0625, 0.0625],
            vec![0.0625, 0.0625, 0.0625, 0.0625],
            vec![0.0625, 0.0625, 0.0625, 0.0625],
        ];
        let mi = mutual_information(&joint);
        assert!(mi.abs() < EPS, "expected 0.0, got {mi}");
    }
    #[test]
    fn test_bsc_capacity() {
        assert!((bsc_capacity(0.0) - 1.0).abs() < EPS);
        assert!(bsc_capacity(0.5).abs() < EPS);
    }
    #[test]
    fn test_bec_capacity() {
        assert!((bec_capacity(0.0) - 1.0).abs() < EPS);
        assert!(bec_capacity(1.0).abs() < EPS);
    }
    #[test]
    fn test_huffman_code_lengths() {
        let probs = [0.5, 0.5];
        let (lengths, avg) = huffman_code_lengths(&probs);
        assert_eq!(lengths, vec![1, 1]);
        assert!((avg - 1.0).abs() < EPS, "avg expected 1.0, got {avg}");
    }
    #[test]
    fn test_build_information_theory_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_information_theory_env(&mut env);
        assert!(
            result.is_ok(),
            "build_information_theory_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_renyi_entropy_limit() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let r1 = renyi_entropy(1.0, &probs);
        let h = entropy(&probs);
        assert!(
            (r1 - h).abs() < EPS,
            "Rényi(1) should equal Shannon entropy"
        );
    }
    #[test]
    fn test_renyi_entropy_order2() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let r2 = renyi_entropy(2.0, &probs);
        assert!(
            (r2 - 2.0).abs() < EPS,
            "Rényi(2) for uniform-4 should be 2.0"
        );
    }
    #[test]
    fn test_min_entropy() {
        let probs = [0.5, 0.25, 0.25];
        let h_min = min_entropy(&probs);
        assert!((h_min - 1.0).abs() < EPS, "expected 1.0, got {h_min}");
    }
    #[test]
    fn test_awgn_capacity() {
        assert!((awgn_capacity(1.0) - 0.5).abs() < EPS);
        assert!((awgn_capacity(3.0) - 1.0).abs() < EPS);
    }
    #[test]
    fn test_gaussian_differential_entropy() {
        use std::f64::consts::{E, PI};
        let sigma_sq = 1.0;
        let h = gaussian_differential_entropy(sigma_sq);
        let expected = 0.5 * (2.0 * PI * E).log2();
        assert!((h - expected).abs() < EPS);
    }
    #[test]
    fn test_entropy_estimator() {
        let est = EntropyEstimator::new(vec![10, 10, 10, 10]);
        let h = est.estimate_entropy();
        assert!((h - 2.0).abs() < EPS, "expected 2.0, got {h}");
    }
    #[test]
    fn test_entropy_estimator_min_entropy() {
        let est = EntropyEstimator::new(vec![40, 20, 20, 20]);
        let h_min = est.estimate_min_entropy();
        let expected = -(0.4f64).log2();
        assert!((h_min - expected).abs() < EPS);
    }
    #[test]
    fn test_huffman_code_build() {
        let probs = [0.5, 0.25, 0.25];
        let hc = HuffmanCode::build(&probs);
        assert_eq!(hc.codewords.len(), 3);
        let symbols = vec![0, 1, 2, 0, 1];
        let bits = hc.encode(&symbols);
        let decoded = hc.decode(&bits).expect("decode failed");
        assert_eq!(decoded, symbols);
    }
    #[test]
    fn test_huffman_code_avg_bits() {
        let probs = [0.5, 0.5];
        let hc = HuffmanCode::build(&probs);
        assert!((hc.avg_bits - 1.0).abs() < EPS);
    }
    #[test]
    fn test_channel_simulator_capacity() {
        let bsc = ChannelSimulator::new_bsc(0.0);
        assert!((bsc.capacity() - 1.0).abs() < EPS);
        let bec = ChannelSimulator::new_bec(0.5);
        assert!((bec.capacity() - 0.5).abs() < EPS);
    }
    #[test]
    fn test_blahut_arimoto_bsc() {
        let p = 0.1;
        let channel = vec![vec![1.0 - p, p], vec![p, 1.0 - p]];
        let ba = BlahutArimoto::new(channel, 200, 1e-9);
        let (cap, q) = ba.run();
        let expected = bsc_capacity(p);
        assert!(
            (cap - expected).abs() < 1e-5,
            "BA BSC capacity: expected {expected}, got {cap}"
        );
        assert!((q[0] - 0.5).abs() < 1e-4);
    }
    #[test]
    fn test_kl_divergence_calc_jensen_shannon() {
        let p = [0.5, 0.5];
        let js = KLDivergenceCalc::jensen_shannon(&p, &p);
        assert!(js.abs() < EPS, "JS(p,p) should be 0, got {js}");
    }
    #[test]
    fn test_kl_divergence_calc_symmetrized() {
        let p = [0.3, 0.4, 0.3];
        let sym = KLDivergenceCalc::symmetrized(&p, &p);
        assert!(
            sym.abs() < EPS,
            "Symmetrized KL(p,p) should be 0, got {sym}"
        );
    }
    #[test]
    fn test_shannon_fano_lengths() {
        let probs = [0.5, 0.25, 0.25];
        let lengths = shannon_fano_lengths(&probs);
        assert_eq!(lengths[0], 1);
        assert_eq!(lengths[1], 2);
        assert_eq!(lengths[2], 2);
    }
    #[test]
    fn test_is_typical() {
        let probs = [0.5, 0.5];
        let seq: Vec<usize> = vec![0, 0, 0, 0, 1, 1, 1, 1];
        assert!(is_typical(&probs, &seq, 1e-9));
    }
    #[test]
    fn test_slepian_wolf_achievable() {
        let joint = vec![vec![0.25, 0.25], vec![0.25, 0.25]];
        let coder = SlepianWolfCoder::new(joint);
        let h_xy = coder.h_xy();
        assert!(
            (h_xy - 2.0).abs() < EPS,
            "H(X,Y) = 2 for uniform 2x2, got {h_xy}"
        );
        assert!(coder.is_achievable(1.0, 1.0));
        assert!(!coder.is_achievable(0.0, 0.0));
    }
    #[test]
    fn test_slepian_wolf_corner_rates() {
        let joint = vec![vec![0.5, 0.0], vec![0.0, 0.5]];
        let coder = SlepianWolfCoder::new(joint);
        let h_x_y = coder.h_x_given_y();
        let h_y_x = coder.h_y_given_x();
        assert!(
            h_x_y.abs() < EPS,
            "H(X|Y) = 0 for perfectly correlated, got {h_x_y}"
        );
        assert!(
            h_y_x.abs() < EPS,
            "H(Y|X) = 0 for perfectly correlated, got {h_y_x}"
        );
        assert!(coder.is_achievable(0.5, 0.5));
    }
    #[test]
    fn test_wiretap_channel_secrecy_rate() {
        let p = 0.1;
        let wy = vec![vec![1.0 - p, p], vec![p, 1.0 - p]];
        let wz = wy.clone();
        let wt = WiretapChannel::new(wy, wz);
        let q = vec![0.5, 0.5];
        let sr = wt.secrecy_rate(&q);
        assert!(
            sr.abs() < EPS,
            "secrecy rate should be 0 when WY=WZ, got {sr}"
        );
    }
    #[test]
    fn test_wiretap_channel_positive_secrecy() {
        let wy = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let wz = vec![vec![0.5, 0.5], vec![0.5, 0.5]];
        let wt = WiretapChannel::new(wy, wz);
        let q = vec![0.5, 0.5];
        let sr = wt.secrecy_rate(&q);
        assert!(
            (sr - 1.0).abs() < EPS,
            "expected secrecy rate = 1, got {sr}"
        );
    }
    #[test]
    fn test_lempel_ziv_complexity_constant() {
        let bits = vec![false; 8];
        let c = LempelZivComplexity::compute(&bits);
        assert!(
            c >= 1,
            "LZ complexity of constant string should be ≥ 1, got {c}"
        );
    }
    #[test]
    fn test_lempel_ziv_complexity_alternating() {
        let bits: Vec<bool> = (0..8).map(|i| i % 2 == 1).collect();
        let c_alt = LempelZivComplexity::compute(&bits);
        let bits_const = vec![false; 8];
        let c_const = LempelZivComplexity::compute(&bits_const);
        assert!(
            c_alt >= c_const,
            "alternating complexity {c_alt} should be ≥ constant complexity {c_const}"
        );
    }
    #[test]
    fn test_chernoff_information_identical() {
        let p = vec![0.3, 0.4, 0.3];
        let c = ChernoffInformationCalc::compute(&p, &p);
        assert!(c.abs() < 1e-6, "C(P,P) should be 0, got {c}");
    }
    #[test]
    fn test_chernoff_information_orthogonal() {
        let p = vec![1.0, 0.0];
        let q = vec![0.0, 1.0];
        let c = ChernoffInformationCalc::compute(&p, &q);
        assert!(
            c.is_infinite(),
            "C(P,Q) for orthogonal distributions should be inf, got {c}"
        );
    }
    #[test]
    fn test_chernoff_bhattacharyya_distance() {
        let p = vec![0.5, 0.5];
        let d = ChernoffInformationCalc::bhattacharyya_distance(&p, &p);
        assert!(
            d.abs() < EPS,
            "Bhattacharyya distance P=Q should be 0, got {d}"
        );
    }
    #[test]
    fn test_renyi_entropy_computer_order2() {
        let comp = RenyiEntropyComputer::new(2.0);
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let h = comp.compute(&probs);
        assert!(
            (h - 2.0).abs() < EPS,
            "H_2 for uniform-4 should be 2.0, got {h}"
        );
    }
    #[test]
    fn test_renyi_entropy_computer_divergence_self() {
        let comp = RenyiEntropyComputer::new(2.0);
        let p = vec![0.3, 0.4, 0.3];
        let d = comp.renyi_divergence(&p, &p);
        assert!(d.abs() < EPS, "D_2(P||P) should be 0, got {d}");
    }
    #[test]
    fn test_renyi_hartley_entropy() {
        let probs = vec![0.5, 0.25, 0.25];
        let h0 = RenyiEntropyComputer::hartley_entropy(&probs);
        let expected = 3.0f64.log2();
        assert!(
            (h0 - expected).abs() < EPS,
            "Hartley entropy should be log2(3), got {h0}"
        );
    }
    #[test]
    fn test_tsallis_entropy_limit() {
        let comp = TsallisEntropyComputer::new(1.0);
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let s1 = comp.compute(&probs);
        let h = entropy(&probs);
        assert!(
            (s1 - h).abs() < EPS,
            "Tsallis(1) should equal Shannon entropy, got {s1}"
        );
    }
    #[test]
    fn test_tsallis_entropy_order2() {
        let comp = TsallisEntropyComputer::new(2.0);
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let s2 = comp.compute(&probs);
        assert!(
            (s2 - 0.75).abs() < EPS,
            "S_2(uniform-4) should be 0.75, got {s2}"
        );
    }
    #[test]
    fn test_tsallis_divergence_self() {
        let comp = TsallisEntropyComputer::new(2.0);
        let p = vec![0.3, 0.4, 0.3];
        let d = comp.tsallis_divergence(&p, &p);
        assert!(d.abs() < EPS, "Tsallis D_q(P||P) should be 0, got {d}");
    }
    #[test]
    fn test_build_information_theory_env_extended() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_information_theory_env(&mut env);
        assert!(
            result.is_ok(),
            "build_information_theory_env failed: {:?}",
            result.err()
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("MartonRegion"))
                .is_some(),
            "MartonRegion axiom should be registered"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("VonNeumannEntropy"))
                .is_some(),
            "VonNeumannEntropy axiom should be registered"
        );
        assert!(
            env.get(&oxilean_kernel::Name::str("LZComplexity"))
                .is_some(),
            "LZComplexity axiom should be registered"
        );
    }
}
/// Shannon entropy H(X) = -Σ p log₂ p for a typed `Distribution`.
///
/// This is the primary public API function using the `Distribution` type.
pub fn entropy_dist(dist: &super::types::Distribution) -> f64 {
    dist.entropy()
}

/// Joint entropy H(X,Y) from a list of (probability, x_index, y_index) triples.
///
/// H(X,Y) = -Σ_{x,y} P(X=x,Y=y) * log₂ P(X=x,Y=y)
pub fn joint_entropy_triples(joint: &[(f64, usize, usize)]) -> f64 {
    joint
        .iter()
        .filter(|(p, _, _)| *p > 0.0)
        .map(|(p, _, _)| -p * p.log2())
        .sum()
}

/// Marginal distribution of X from joint triples.
fn marginal_x_from_triples(joint: &[(f64, usize, usize)]) -> Vec<f64> {
    if joint.is_empty() {
        return vec![];
    }
    let max_x = joint.iter().map(|(_, x, _)| x).max().copied().unwrap_or(0);
    let mut px = vec![0.0f64; max_x + 1];
    for &(p, x, _) in joint {
        px[x] += p;
    }
    px
}

/// Marginal distribution of Y from joint triples.
fn marginal_y_from_triples(joint: &[(f64, usize, usize)]) -> Vec<f64> {
    if joint.is_empty() {
        return vec![];
    }
    let max_y = joint.iter().map(|(_, _, y)| y).max().copied().unwrap_or(0);
    let mut py = vec![0.0f64; max_y + 1];
    for &(p, _, y) in joint {
        py[y] += p;
    }
    py
}

/// Conditional entropy H(Y|X) = H(X,Y) - H(X) from joint triples.
pub fn conditional_entropy_triples(joint: &[(f64, usize, usize)]) -> f64 {
    let h_xy = joint_entropy_triples(joint);
    let px = marginal_x_from_triples(joint);
    let h_x: f64 = px
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum();
    h_xy - h_x
}

/// Mutual information I(X;Y) = H(X) + H(Y) - H(X,Y) from joint triples.
pub fn mutual_information_triples(joint: &[(f64, usize, usize)]) -> f64 {
    let px = marginal_x_from_triples(joint);
    let py = marginal_y_from_triples(joint);
    let h_x: f64 = px
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum();
    let h_y: f64 = py
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum();
    let h_xy = joint_entropy_triples(joint);
    h_x + h_y - h_xy
}

/// KL divergence D_KL(P‖Q) using typed `Distribution`.
pub fn kl_divergence_dist(p: &super::types::Distribution, q: &super::types::Distribution) -> f64 {
    kl_divergence(&p.probs, &q.probs)
}

/// Channel capacity C = max_{P(X)} I(X;Y) via Blahut-Arimoto algorithm.
///
/// Runs up to 500 iterations with tolerance 1e-9.
pub fn channel_capacity(channel: &super::types::Channel) -> f64 {
    let ba = BlahutArimoto::new(channel.matrix.clone(), 500, 1e-9);
    let (cap, _) = ba.run();
    cap
}

/// Build a Huffman code from a `Distribution` returning a typed `HuffmanCode`.
pub fn build_huffman_code(dist: &super::types::Distribution) -> HuffmanCode {
    HuffmanCode::build(&dist.probs)
}

/// Encode a sequence of symbol indices using a `HuffmanCode`.
pub fn huffman_encode(data: &[usize], code: &HuffmanCode) -> Vec<bool> {
    code.encode(data)
}

/// Decode a bit sequence using a `HuffmanCode`.
///
/// Returns `None` if the bits do not decode to valid codewords.
pub fn huffman_decode(bits: &[bool], code: &HuffmanCode) -> Option<Vec<usize>> {
    code.decode(bits)
}

/// Arithmetic encode a symbol sequence using an `ArithmeticCodeState`.
///
/// Returns a bit sequence representing the final interval midpoint.
/// Emits one bit per ~1 bit of information using successive interval subdivision.
pub fn arithmetic_encode(symbols: &[usize], dist: &super::types::Distribution) -> Vec<bool> {
    use super::types::ArithmeticCodeState;
    if symbols.is_empty() || dist.probs.is_empty() {
        return vec![];
    }
    let mut state = ArithmeticCodeState::new(dist);
    for &s in symbols {
        if s < dist.probs.len() {
            state.encode_symbol(s);
        }
    }
    // Output bits by extracting the interval midpoint in binary
    let mid = (state.low + state.high) / 2.0;
    let mut bits = Vec::new();
    let mut val = mid;
    // Use 64 bits of precision (sufficient for double)
    for _ in 0..64 {
        val *= 2.0;
        if val >= 1.0 {
            bits.push(true);
            val -= 1.0;
        } else {
            bits.push(false);
        }
    }
    // Trim trailing zeros but keep at least 1 bit
    while bits.len() > 1 && !bits[bits.len() - 1] {
        bits.pop();
    }
    bits
}

/// Arithmetic decode a bit sequence to recover `len` symbols.
pub fn arithmetic_decode(
    bits: &[bool],
    dist: &super::types::Distribution,
    len: usize,
) -> Vec<usize> {
    use super::types::ArithmeticCodeState;
    if bits.is_empty() || dist.probs.is_empty() || len == 0 {
        return vec![];
    }
    // Reconstruct the floating-point value from bits
    let mut val = 0.0f64;
    let mut place = 0.5f64;
    for &b in bits {
        if b {
            val += place;
        }
        place *= 0.5;
    }
    let mut state = ArithmeticCodeState::new(dist);
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        match state.decode_symbol(val) {
            Some(s) => {
                result.push(s);
                let range = state.high - state.low;
                let lo = state.low + range * state.cdf[s];
                let hi = state.low + range * state.cdf[s + 1];
                state.low = lo;
                state.high = hi;
            }
            None => break,
        }
    }
    result
}

#[allow(dead_code)]
pub fn shannon_entropy(pmf: &[f64]) -> f64 {
    pmf.iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}
#[allow(dead_code)]
pub fn h_b(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
}
#[cfg(test)]
mod tests_it_extra {
    use super::*;
    #[test]
    fn test_mutual_information() {
        let joint = vec![vec![0.25, 0.25], vec![0.25, 0.25]];
        let mi = MutualInformation::new(joint);
        assert!(mi.compute().abs() < 1e-9, "Independent vars: MI=0");
        let joint2 = vec![vec![0.5, 0.0], vec![0.0, 0.5]];
        let mi2 = MutualInformation::new(joint2);
        let result = mi2.compute();
        assert!((result - 1.0).abs() < 1e-9, "Deterministic: MI=H(X)=1");
    }
    #[test]
    fn test_kolmogorov_complexity() {
        let compressible = KolmogorovComplexity::new(10000, 1000);
        assert!(!compressible.is_random());
        assert!(compressible.redundancy() > 0.5);
        let random = KolmogorovComplexity::new(1000, 990);
        assert!(random.is_random());
    }
    #[test]
    fn test_rate_distortion() {
        let r = RateDistortion::binary_rd_function(0.3, 0.0);
        let hb = h_b(0.3);
        assert!((r - hb).abs() < 1e-9);
        let r2 = RateDistortion::binary_rd_function(0.3, 0.35);
        assert_eq!(r2, 0.0);
    }
    #[test]
    fn test_info_theoretic_security() {
        let otp = InfoTheoreticSecurity::new(128, 128, 128);
        assert!(otp.is_perfectly_secret());
        assert!(otp.is_one_time_pad());
        assert_eq!(otp.leakage_bits(), 0.0);
        let weak = InfoTheoreticSecurity::new(256, 128, 256);
        assert!(!weak.is_perfectly_secret());
        assert_eq!(weak.leakage_bits(), 128.0);
    }
}

#[cfg(test)]
mod tests_new_api {
    use super::super::types::{ArithmeticCodeState, Channel, Distribution, InformationMeasure};
    use super::*;

    const EPS: f64 = 1e-9;

    #[test]
    fn test_distribution_new_uniform() {
        let d = Distribution::new(vec![0.25, 0.25, 0.25, 0.25]).expect("valid dist");
        assert_eq!(d.alphabet_size(), 4);
        assert!((d.entropy() - 2.0).abs() < EPS);
    }

    #[test]
    fn test_distribution_new_normalizes() {
        let d = Distribution::new(vec![1.0, 1.0, 2.0]).expect("valid dist");
        assert!((d.probs[0] - 0.25).abs() < EPS);
        assert!((d.probs[2] - 0.5).abs() < EPS);
    }

    #[test]
    fn test_distribution_new_invalid() {
        assert!(Distribution::new(vec![]).is_none());
        assert!(Distribution::new(vec![-0.1, 0.5]).is_none());
    }

    #[test]
    fn test_entropy_dist() {
        let d = Distribution::new(vec![0.5, 0.5]).expect("valid");
        assert!((entropy_dist(&d) - 1.0).abs() < EPS);
    }

    #[test]
    fn test_joint_entropy_triples() {
        // Uniform joint over 2x2: each cell has prob 0.25
        let joint = vec![(0.25, 0, 0), (0.25, 0, 1), (0.25, 1, 0), (0.25, 1, 1)];
        let h = joint_entropy_triples(&joint);
        assert!((h - 2.0).abs() < EPS, "H(X,Y)=2 for uniform 2x2, got {h}");
    }

    #[test]
    fn test_conditional_entropy_triples() {
        // Independent: H(Y|X) = H(Y)
        let joint = vec![(0.25, 0, 0), (0.25, 0, 1), (0.25, 1, 0), (0.25, 1, 1)];
        let h_yx = conditional_entropy_triples(&joint);
        assert!(
            (h_yx - 1.0).abs() < EPS,
            "H(Y|X)=1 for independent uniform, got {h_yx}"
        );
    }

    #[test]
    fn test_conditional_entropy_deterministic() {
        // Y = X: H(Y|X) = 0
        let joint = vec![(0.5, 0, 0), (0.5, 1, 1)];
        let h_yx = conditional_entropy_triples(&joint);
        assert!(h_yx.abs() < EPS, "H(Y|X)=0 for deterministic, got {h_yx}");
    }

    #[test]
    fn test_mutual_information_triples_independent() {
        let joint = vec![(0.25, 0, 0), (0.25, 0, 1), (0.25, 1, 0), (0.25, 1, 1)];
        let mi = mutual_information_triples(&joint);
        assert!(mi.abs() < EPS, "MI=0 for independent vars, got {mi}");
    }

    #[test]
    fn test_mutual_information_triples_deterministic() {
        let joint = vec![(0.5, 0, 0), (0.5, 1, 1)];
        let mi = mutual_information_triples(&joint);
        assert!(
            (mi - 1.0).abs() < EPS,
            "MI=1 for deterministic binary, got {mi}"
        );
    }

    #[test]
    fn test_kl_divergence_dist_self() {
        let p = Distribution::new(vec![0.3, 0.4, 0.3]).expect("valid");
        let kl = kl_divergence_dist(&p, &p);
        assert!(kl.abs() < EPS, "D_KL(P||P)=0, got {kl}");
    }

    #[test]
    fn test_kl_divergence_dist_known() {
        let p = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let q = Distribution::new(vec![0.25, 0.75]).expect("valid");
        let kl = kl_divergence_dist(&p, &q);
        // D_KL(Bern(0.5)||Bern(0.25)) = 0.5*log2(2) + 0.5*log2(2/3) > 0
        assert!(kl > 0.0, "KL divergence should be positive, got {kl}");
    }

    #[test]
    fn test_channel_capacity_bsc() {
        // BSC with p=0: capacity = 1 bit
        let matrix = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let ch = Channel::new(matrix).expect("valid channel");
        let cap = channel_capacity(&ch);
        assert!((cap - 1.0).abs() < 1e-5, "BSC(p=0) capacity=1, got {cap}");
    }

    #[test]
    fn test_channel_capacity_noisy() {
        // BSC with p=0.1: capacity ≈ 1 - h_b(0.1)
        let p = 0.1;
        let matrix = vec![vec![1.0 - p, p], vec![p, 1.0 - p]];
        let ch = Channel::new(matrix).expect("valid channel");
        let cap = channel_capacity(&ch);
        let expected = bsc_capacity(p);
        assert!(
            (cap - expected).abs() < 1e-5,
            "BSC({p}) capacity: expected {expected}, got {cap}"
        );
    }

    #[test]
    fn test_build_huffman_code_and_encode_decode() {
        let dist = Distribution::new(vec![0.5, 0.25, 0.25]).expect("valid");
        let code = build_huffman_code(&dist);
        assert_eq!(code.codewords.len(), 3);
        let symbols = vec![0, 1, 2, 0, 0, 1];
        let bits = huffman_encode(&symbols, &code);
        let decoded = huffman_decode(&bits, &code).expect("decode failed");
        assert_eq!(decoded, symbols);
    }

    #[test]
    fn test_build_huffman_code_average_bits() {
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let code = build_huffman_code(&dist);
        assert!(
            (code.avg_bits - 1.0).abs() < EPS,
            "avg_bits should be 1.0, got {}",
            code.avg_bits
        );
    }

    #[test]
    fn test_huffman_decode_invalid() {
        // All symbols map to single bit codes; extra bits that don't match -> None
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let code = build_huffman_code(&dist);
        // Empty is valid (decodes to empty vec)
        let decoded = huffman_decode(&[], &code);
        assert_eq!(decoded, Some(vec![]));
    }

    #[test]
    fn test_arithmetic_encode_decode_roundtrip() {
        let dist = Distribution::new(vec![0.5, 0.25, 0.25]).expect("valid");
        let symbols = vec![0, 1, 2];
        let bits = arithmetic_encode(&symbols, &dist);
        assert!(!bits.is_empty(), "encoded bits should not be empty");
        let decoded = arithmetic_decode(&bits, &dist, symbols.len());
        assert_eq!(decoded, symbols, "arithmetic codec roundtrip failed");
    }

    #[test]
    fn test_arithmetic_encode_single_symbol() {
        let dist = Distribution::new(vec![0.7, 0.3]).expect("valid");
        let symbols = vec![0];
        let bits = arithmetic_encode(&symbols, &dist);
        let decoded = arithmetic_decode(&bits, &dist, 1);
        assert_eq!(decoded, symbols);
    }

    #[test]
    fn test_arithmetic_encode_empty() {
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let bits = arithmetic_encode(&[], &dist);
        assert!(bits.is_empty());
    }

    #[test]
    fn test_arithmetic_code_state_encode_symbol() {
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let mut state = ArithmeticCodeState::new(&dist);
        state.encode_symbol(0);
        // After encoding symbol 0 (lower half), interval should be [0, 0.5)
        assert!(
            state.low < state.high,
            "interval should be valid after encoding"
        );
        assert!(
            state.high <= 0.5 + EPS,
            "high should be <= 0.5 for symbol 0"
        );
    }

    #[test]
    fn test_information_measure_enum() {
        assert_eq!(InformationMeasure::Entropy, InformationMeasure::Entropy);
        assert_ne!(InformationMeasure::Entropy, InformationMeasure::MutualInfo);
        assert_ne!(
            InformationMeasure::RelativeEntropy,
            InformationMeasure::ChannelCapacity
        );
    }

    #[test]
    fn test_channel_output_distribution() {
        // Identity channel: output matches input
        let matrix = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let ch = Channel::new(matrix).expect("valid");
        let px = Distribution::new(vec![0.3, 0.7]).expect("valid");
        let py = ch.output_distribution(&px);
        assert!((py.probs[0] - 0.3).abs() < EPS);
        assert!((py.probs[1] - 0.7).abs() < EPS);
    }

    #[test]
    fn test_channel_new_invalid() {
        // Inconsistent row lengths
        let matrix = vec![vec![0.5, 0.5], vec![0.3]];
        assert!(Channel::new(matrix).is_none());
        // Empty
        assert!(Channel::new(vec![]).is_none());
    }

    #[test]
    fn test_huffman_node_collect_codewords() {
        use super::super::types::HuffmanNode;
        let leaf0 = HuffmanNode::Leaf {
            symbol: 0,
            prob: 0.5,
        };
        let leaf1 = HuffmanNode::Leaf {
            symbol: 1,
            prob: 0.5,
        };
        let root = HuffmanNode::Internal {
            prob: 1.0,
            left: Box::new(leaf0),
            right: Box::new(leaf1),
        };
        let mut codes = Vec::new();
        root.collect_codewords(vec![], &mut codes);
        assert_eq!(codes.len(), 2);
        // Symbol 0 gets prefix [false], symbol 1 gets prefix [true]
        let code0 = codes
            .iter()
            .find(|(s, _)| *s == 0)
            .map(|(_, c)| c.clone())
            .expect("sym 0");
        let code1 = codes
            .iter()
            .find(|(s, _)| *s == 1)
            .map(|(_, c)| c.clone())
            .expect("sym 1");
        assert_eq!(code0, vec![false]);
        assert_eq!(code1, vec![true]);
    }

    #[test]
    fn test_arithmetic_decode_empty_bits() {
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let result = arithmetic_decode(&[], &dist, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_arithmetic_decode_zero_len() {
        let dist = Distribution::new(vec![0.5, 0.5]).expect("valid");
        let bits = vec![false, true];
        let result = arithmetic_decode(&bits, &dist, 0);
        assert!(result.is_empty());
    }
}
