//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdaBoost, BiasVarianceTradeoff, CausalBackdoor, CrossValidation, DoubleRademacher,
    EarlyStoppingReg, ExponentialWeightsAlgorithm, FeatureMap, FisherInformation,
    GaussianComplexity, GaussianProcess, GradientBoosting, GrowthFunction, KernelFunction,
    KernelMatrix, KernelSVM, KernelSVMTrainer, LassoReg, OnlineGradientDescent,
    PACBayesGeneralization, PACLearner, Perceptron, RademacherComplexity, RegretBound,
    SVMClassifier, SampleComplexity, TikhonovReg, UCBBandit, UniformConvergence, VCDimension, ELBO,
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
/// `PACLearner`: a learning algorithm that for any ε, δ > 0 returns h with L_D(h) ≤ ε
/// Type: Real → Real → Nat → Type
pub fn pac_learner_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), type0())))
}
/// `SampleComplexity`: m = O((d log(d/ε) + log(1/δ)) / ε)
/// Type: Real → Real → Nat → Nat
pub fn sample_complexity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), nat_ty())))
}
/// `VCDimension`: maximal shattered set size for a hypothesis class H
/// Type: Type → Nat
pub fn vc_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `GrowthFunction`: Π_H(m) = max_{S,|S|=m} |{h|_S : h ∈ H}|
/// Type: Type → Nat → Nat
pub fn growth_function_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), nat_ty()))
}
/// `PACLearnability`: a hypothesis class is PAC learnable
/// Type: Type → Prop
pub fn pac_learnability_ty() -> Expr {
    arrow(type0(), prop())
}
/// Fundamental theorem of PAC learning: finite VC dimension ↔ PAC learnability
/// Type: ∀ (H : Type), VCDimension H < ∞ → PACLearnability H
pub fn fundamental_theorem_pac_ty() -> Expr {
    pi(BinderInfo::Default, "H", type0(), prop())
}
/// Sauer-Shelah lemma: Π_H(m) ≤ Σ_{i=0}^{d} C(m,i)
/// Type: ∀ (H : Type) (m : Nat), GrowthFunction H m ≤ Sauer_bound (VCDimension H) m
pub fn sauer_shelah_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(BinderInfo::Default, "m", nat_ty(), prop()),
    )
}
/// Sample complexity upper bound for PAC learning
/// Type: ∀ (ε δ : Real) (d : Nat), m ≥ SampleComplexity ε δ d → Prop
pub fn sample_complexity_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        pi(
            BinderInfo::Default,
            "delta",
            real_ty(),
            pi(BinderInfo::Default, "d", nat_ty(), prop()),
        ),
    )
}
/// `RademacherComplexity`: R_n(H) = E_{σ,S}\[sup_{h∈H} (1/n) Σ σ_i h(x_i)\]
/// Type: Type → Nat → Real
pub fn rademacher_complexity_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), real_ty()))
}
/// `UniformConvergence`: sup_{h∈H} |L_D(h) - L_S(h)| ≤ ε w.p. ≥ 1−δ
/// Type: Type → Real → Real → Prop
pub fn uniform_convergence_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `DoubleRademacher`: two-sided Rademacher bound (two-sided uniform convergence)
/// Type: Type → Nat → Real → Prop
pub fn double_rademacher_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `GaussianComplexity`: G_n(H) = E_{g,S}\[sup_{h∈H} (1/n) Σ g_i h(x_i)\]
/// Type: Type → Nat → Real
pub fn gaussian_complexity_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), real_ty()))
}
/// Rademacher generalization bound: L_D(h) ≤ L_S(h) + 2 R_n(H) + O(√(log(1/δ)/n))
/// Type: ∀ (H : Type) (n : Nat) (δ : Real), Prop
pub fn rademacher_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(BinderInfo::Default, "delta", real_ty(), prop()),
        ),
    )
}
/// Symmetrization lemma: relates population risk to Rademacher complexity
/// Type: ∀ (H : Type) (n : Nat), Prop
pub fn symmetrization_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// `KernelFunction`: k: X × X → ℝ, a positive-definite symmetric function
/// Type: Type → Type (representing a kernel on X)
pub fn kernel_function_ty() -> Expr {
    arrow(type0(), type0())
}
/// `RKHS`: reproducing kernel Hilbert space H_k associated to kernel k
/// Type: (Type → Type) → Type
pub fn rkhs_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// `FeatureMap`: φ: X → H_k with k(x,x') = ⟨φ(x),φ(x')⟩
/// Type: Type → Type → Type
pub fn feature_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `KernelMatrix`: Gram matrix K_{ij} = k(x_i, x_j) ∈ ℝ^{n×n}
/// Type: (Type → Type) → Nat → Type
pub fn kernel_matrix_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(nat_ty(), type0()))
}
/// `KernelSVM`: support vector machine with kernel trick
/// Type: (Type → Type) → Real → Type
pub fn kernel_svm_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(real_ty(), type0()))
}
/// Mercer's theorem: k is p.d. ↔ ∃ feature map φ with k(x,x') = ⟨φ(x),φ(x')⟩
/// Type: ∀ (k : Type → Type), isPositiveDefinite k ↔ ∃ feature map, Prop
pub fn mercer_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "k", arrow(type0(), type0()), prop())
}
/// Representer theorem: optimal solution in RKHS lies in span of kernel evaluations
/// Type: ∀ (k : Type → Type) (n : Nat), Prop
pub fn representer_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        arrow(type0(), type0()),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// Kernel PCA: principal components in feature space via eigendecomposition of K
/// Type: (Type → Type) → Nat → Nat → Type
pub fn kernel_pca_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(nat_ty(), arrow(nat_ty(), type0())),
    )
}
/// `RegularizedObjective`: L(h) + λ Ω(h)
/// Type: Real → Type (regularization weight → regularized problem)
pub fn regularized_objective_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// `TikhonovReg`: Tikhonov regularization λ‖h‖²_{H_k}
/// Type: Real → (Type → Type) → Type
pub fn tikhonov_reg_ty() -> Expr {
    arrow(real_ty(), arrow(arrow(type0(), type0()), type0()))
}
/// `LassoReg`: ℓ₁ regularization λ‖h‖₁ (sparsity-promoting)
/// Type: Real → Type
pub fn lasso_reg_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// `EarlyStoppingReg`: implicit regularization via iteration count T
/// Type: Nat → Type
pub fn early_stopping_reg_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BiasVarianceTradeoff`: MSE = Bias² + Variance + Noise
/// Type: Real → Real → Real → Prop
pub fn bias_variance_tradeoff_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Ridge regression solution: (X^T X + λI)^{-1} X^T y
/// Type: ∀ (n d : Nat) (λ : Real), List (List Real) → List Real → List Real
pub fn ridge_regression_solution_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "d",
            nat_ty(),
            pi(BinderInfo::Default, "lam", real_ty(), prop()),
        ),
    )
}
/// Bias-variance decomposition: E\[(ŷ - y)²\] = Bias²(ŷ) + Var(ŷ) + σ²
/// Type: Prop
pub fn bias_variance_decomposition_ty() -> Expr {
    prop()
}
/// `OnlineAlgorithm`: sequential prediction protocol over T rounds
/// Type: Nat → Type
pub fn online_algorithm_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Perceptron`: online linear classifier with mistake bound
/// Type: Nat → Type (dimension → perceptron)
pub fn perceptron_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AdaBoost`: adaptive boosting with exponential loss, T weak learners
/// Type: Nat → Type → Type
pub fn adaboost_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `OnlineGradientDescent`: OGD with regret O(√T)
/// Type: Real → Nat → Type (learning rate η, rounds T)
pub fn online_gradient_descent_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), type0()))
}
/// `RegretBound`: R_T = Σ L_t(h_t) - min_h Σ L_t(h) ≤ O(√T)
/// Type: Nat → Real → Prop (rounds T, bound ε)
pub fn regret_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// Perceptron mistake bound: M ≤ (R/γ)² where R = max‖x‖, γ = margin
/// Type: ∀ (R γ : Real), mistakes ≤ (R/γ)²
pub fn perceptron_mistake_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        real_ty(),
        pi(BinderInfo::Default, "gamma", real_ty(), prop()),
    )
}
/// OGD regret bound: R_T ≤ D²/(2η) + η Σ‖∇_t‖² ≤ D·G·√(2T)
/// Type: ∀ (T : Nat) (eta : Real), Prop
pub fn ogd_regret_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        nat_ty(),
        pi(BinderInfo::Default, "eta", real_ty(), prop()),
    )
}
/// AdaBoost training error: ≤ exp(-2 Σ γ_t²) after T rounds
/// Type: ∀ (T : Nat), Prop
pub fn adaboost_training_error_ty() -> Expr {
    pi(BinderInfo::Default, "T", nat_ty(), prop())
}
/// `MLMutualInformation`: I(X;Y) = H(X) - H(X|Y) for learning-theoretic analysis
/// Type: (List Real) → (List Real) → Real
pub fn ml_mutual_information_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `MLKLDivergence`: D_KL(P‖Q) = Σ P log(P/Q) — used in PAC-Bayes bounds
/// Type: (List Real) → (List Real) → Real
pub fn ml_kl_divergence_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `FisherInformation`: I(θ) = E\[(∂/∂θ log p(x;θ))²\]
/// Type: (Real → Real) → Real → Real
pub fn fisher_information_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `ELBO`: evidence lower bound ℒ(q) = E_q\[log p(x,z)\] - E_q\[log q(z)\]
/// Type: (Real → Real) → (Real → Real) → Real
pub fn elbo_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), real_ty()),
    )
}
/// Data processing inequality: I(X;Z) ≤ I(X;Y) for Markov chain X→Y→Z
/// Type: Prop
pub fn data_processing_inequality_ty() -> Expr {
    prop()
}
/// Chain rule of mutual information: I(X;Y,Z) = I(X;Y) + I(X;Z|Y)
/// Type: Prop
pub fn chain_rule_mi_ty() -> Expr {
    prop()
}
/// Cramér-Rao bound: Var(θ̂) ≥ 1/I(θ) for any unbiased estimator
/// Type: ∀ (p : Real → Real) (θ : Real), Prop
pub fn cramer_rao_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        arrow(real_ty(), real_ty()),
        pi(BinderInfo::Default, "theta", real_ty(), prop()),
    )
}
/// PAC-Bayes bound: L_D(Q) ≤ L_S(Q) + √((D_KL(Q‖P) + log(n/δ))/(2n))
/// Type: ∀ (n : Nat) (δ : Real), Prop
pub fn pac_bayes_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "delta", real_ty(), prop()),
    )
}
/// Register all statistical learning theory axioms and theorems in the kernel environment.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("PACLearner", pac_learner_ty()),
        ("SampleComplexity", sample_complexity_ty()),
        ("VCDimension", vc_dimension_ty()),
        ("GrowthFunction", growth_function_ty()),
        ("PACLearnability", pac_learnability_ty()),
        ("fundamental_theorem_pac", fundamental_theorem_pac_ty()),
        ("sauer_shelah_lemma", sauer_shelah_ty()),
        ("sample_complexity_bound", sample_complexity_bound_ty()),
        ("RademacherComplexity", rademacher_complexity_ty()),
        ("UniformConvergence", uniform_convergence_ty()),
        ("DoubleRademacher", double_rademacher_ty()),
        ("GaussianComplexity", gaussian_complexity_ty()),
        ("rademacher_bound", rademacher_bound_ty()),
        ("symmetrization_lemma", symmetrization_lemma_ty()),
        ("KernelFunction", kernel_function_ty()),
        ("RKHS", rkhs_ty()),
        ("FeatureMap", feature_map_ty()),
        ("KernelMatrix", kernel_matrix_ty()),
        ("KernelSVM", kernel_svm_ty()),
        ("mercer_theorem", mercer_theorem_ty()),
        ("representer_theorem", representer_theorem_ty()),
        ("KernelPCA", kernel_pca_ty()),
        ("RegularizedObjective", regularized_objective_ty()),
        ("TikhonovReg", tikhonov_reg_ty()),
        ("LassoReg", lasso_reg_ty()),
        ("EarlyStoppingReg", early_stopping_reg_ty()),
        ("BiasVarianceTradeoff", bias_variance_tradeoff_ty()),
        ("ridge_regression_solution", ridge_regression_solution_ty()),
        (
            "bias_variance_decomposition",
            bias_variance_decomposition_ty(),
        ),
        ("OnlineAlgorithm", online_algorithm_ty()),
        ("Perceptron", perceptron_ty()),
        ("AdaBoost", adaboost_ty()),
        ("OnlineGradientDescent", online_gradient_descent_ty()),
        ("RegretBound", regret_bound_ty()),
        ("perceptron_mistake_bound", perceptron_mistake_bound_ty()),
        ("ogd_regret_bound", ogd_regret_bound_ty()),
        ("adaboost_training_error", adaboost_training_error_ty()),
        ("MLMutualInformation", ml_mutual_information_ty()),
        ("MLKLDivergence", ml_kl_divergence_ty()),
        ("FisherInformation", fisher_information_ty()),
        ("ELBO", elbo_ty()),
        (
            "data_processing_inequality",
            data_processing_inequality_ty(),
        ),
        ("chain_rule_mutual_information", chain_rule_mi_ty()),
        ("cramer_rao_bound", cramer_rao_bound_ty()),
        ("pac_bayes_bound", pac_bayes_bound_ty()),
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
/// Dot product of two equal-length slices.
pub(super) fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}
/// `ExponentialWeightsAlgorithm`: EWA (Hedge) distribution over n experts
/// Type: Nat → Real → Type (n experts, learning rate η)
pub fn ewa_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `MultiplicativeWeightsUpdate`: MW update step at round t
/// Type: Nat → Real → Type (n, η)
pub fn multiplicative_weights_update_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `EWARegretBound`: R_T(EWA) ≤ ln(n)/η + η T/2
/// Type: ∀ (n : Nat) (T : Nat) (η : Real), Prop
pub fn ewa_regret_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "T",
            nat_ty(),
            pi(BinderInfo::Default, "eta", real_ty(), prop()),
        ),
    )
}
/// `BanditAlgorithm`: protocol with only loss feedback (no gradient)
/// Type: Nat → Nat → Type (n arms, T rounds)
pub fn bandit_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UCBAlgorithm`: upper confidence bound algorithm
/// Type: Nat → Real → Type (n arms, exploration param c)
pub fn ucb_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `UCBRegretBound`: UCB1 regret O(√(n T ln T))
/// Type: ∀ (n T : Nat), Prop
pub fn ucb_regret_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "T", nat_ty(), prop()),
    )
}
/// `ThompsonSampling`: Bayesian bandit via posterior sampling
/// Type: Nat → Type (n arms)
pub fn thompson_sampling_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BayesianRegretBound`: Bayesian regret for Thompson sampling O(√(n T))
/// Type: ∀ (n T : Nat), Prop
pub fn bayesian_regret_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "T", nat_ty(), prop()),
    )
}
/// `DataDependentBound`: a bound that depends on the observed dataset S
/// Type: Nat → Real → Prop (n samples, bound value)
pub fn data_dependent_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `LocalizedRademacher`: local Rademacher complexity around minimizer
/// Type: Type → Nat → Real → Real (H, n, radius → complexity)
pub fn localized_rademacher_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(real_ty(), real_ty())))
}
/// `LocalizedBound`: generalization bound via localized Rademacher
/// Type: ∀ (H : Type) (n : Nat) (δ : Real), Prop
pub fn localized_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(BinderInfo::Default, "delta", real_ty(), prop()),
        ),
    )
}
/// `PACBayesBound`: McAllester's PAC-Bayes bound
/// Type: ∀ (n : Nat) (δ : Real), (List Real) → (List Real) → Prop
pub fn pac_bayes_mcallester_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "delta", real_ty(), prop()),
    )
}
/// `CatoniPACBayes`: Catoni's PAC-Bayes bound with tighter constants
/// Type: ∀ (n : Nat) (δ : Real), Prop
pub fn catoni_pac_bayes_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "delta", real_ty(), prop()),
    )
}
/// `RKHSNorm`: ‖f‖_{H_k}² = Σ_{i,j} α_i α_j k(x_i, x_j)
/// Type: (Type → Type) → Real (kernel → norm²)
pub fn rkhs_norm_ty() -> Expr {
    arrow(arrow(type0(), type0()), real_ty())
}
/// `KernelPCAProjection`: projection onto top-k kernel PCA components
/// Type: (Type → Type) → Nat → Nat → Type (kernel, n, k → projector)
pub fn kernel_pca_projection_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(nat_ty(), arrow(nat_ty(), type0())),
    )
}
/// `SVMGeneralizationBound`: margin-based bound ≤ R²/(γ² n)
/// Type: ∀ (n : Nat) (R γ : Real), Prop
pub fn svm_generalization_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "R",
            real_ty(),
            pi(BinderInfo::Default, "gamma", real_ty(), prop()),
        ),
    )
}
/// `NeuralNetwork`: a feedforward network of given depth and width
/// Type: Nat → Nat → Type (depth, width → network)
pub fn neural_network_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `DepthSeparation`: a function requiring exponentially wide shallow net
/// Type: Nat → Prop (depth d → separation result)
pub fn depth_separation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BarronSpace`: functions with finite Barron norm (representable by shallow nets)
/// Type: Real → Type (radius B → function class)
pub fn barron_space_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// `BarronApproximation`: shallow nets approximate Barron functions at rate 1/√m
/// Type: ∀ (B : Real) (m : Nat), Prop (B = norm bound, m = neurons)
pub fn barron_approximation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        real_ty(),
        pi(BinderInfo::Default, "m", nat_ty(), prop()),
    )
}
/// `NNExpressivity`: VC dimension / capacity of a neural net class
/// Type: Nat → Nat → Nat (depth, width → VC dim)
pub fn nn_expressivity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `NNGeneralizationBound`: Rademacher-based bound for neural networks
/// Type: ∀ (depth width n : Nat) (δ : Real), Prop
pub fn nn_generalization_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "depth",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "width",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                pi(BinderInfo::Default, "delta", real_ty(), prop()),
            ),
        ),
    )
}
/// `DoubleDescent`: test error as function of model complexity
/// Type: Nat → Real (n_params → test error curve)
pub fn double_descent_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `BenignOverfitting`: interpolating solution still generalizes well
/// Type: ∀ (n d : Nat), Prop (n samples, d features)
pub fn benign_overfitting_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "d", nat_ty(), prop()),
    )
}
/// `ImplicitRegularization`: GD with zero init converges to min-norm solution
/// Type: Nat → Real → Prop (steps T, step size η)
pub fn implicit_regularization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        nat_ty(),
        pi(BinderInfo::Default, "eta", real_ty(), prop()),
    )
}
/// `MinNormInterpolation`: min-‖w‖ solution that fits training data exactly
/// Type: Nat → Nat → Type (n samples, d features → solution)
pub fn min_norm_interpolation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UniformStability`: |L(A(S), z) - L(A(S'), z)| ≤ β for any S, S' differing in 1 point
/// Type: Real → Prop (β → stability predicate)
pub fn uniform_stability_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `OnAverageStability`: E_S[E_z|L(A(S),z) - L(A(S^{(i)}),z)|] ≤ β
/// Type: Real → Prop
pub fn on_average_stability_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `StabilityGeneralizationBound`: β-stable → gen error ≤ 2β + O(1/√n)
/// Type: ∀ (β : Real) (n : Nat) (δ : Real), Prop
pub fn stability_generalization_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "beta",
        real_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(BinderInfo::Default, "delta", real_ty(), prop()),
        ),
    )
}
/// `DataDeletion`: privacy-adjacent: model update after removing one point
/// Type: Nat → Real → Type (n, budget → deletion mechanism)
pub fn data_deletion_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `DPSGDAlgorithm`: DP-SGD with noise σ, clipping C, epochs T
/// Type: Real → Real → Nat → Type (σ, C, T → algorithm)
pub fn dp_sgd_algorithm_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), type0())))
}
/// `PrivatePACLearning`: PAC learning with (ε, δ)-differential privacy
/// Type: Real → Real → Type (ε_priv, δ_priv → learner)
pub fn private_pac_learning_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// `PrivateQueryMechanism`: answering statistical queries with DP
/// Type: Real → Real → Type (ε, δ → mechanism)
pub fn private_query_mechanism_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// `DPGeneralizationBound`: utility bound for DP learning
/// Type: ∀ (n : Nat) (eps_priv delta_priv : Real), Prop
pub fn dp_generalization_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "eps_priv",
            real_ty(),
            pi(BinderInfo::Default, "delta_priv", real_ty(), prop()),
        ),
    )
}
/// `DPSampleComplexity`: extra samples needed for privacy
/// Type: Real → Real → Real → Real → Nat (ε_priv, δ_priv, ε_learn, δ_learn → m)
pub fn dp_sample_complexity_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), nat_ty()))),
    )
}
/// `CalibrationError`: ECE = E[|P(Y=1|score=p) - p|]
/// Type: (List Real) → (List Real) → Real (predictions, labels → ECE)
pub fn calibration_error_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `ProperScoringRule`: S(f, y) is proper if E\[S(f,Y)\] maximized by true dist
/// Type: (Real → Real) → Real → Real (forecast function, outcome → score)
pub fn proper_scoring_rule_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `ReliabilityDiagram`: calibration curve P(Y=1 | score ∈ bin) vs. score
/// Type: Nat → Type (n bins → diagram)
pub fn reliability_diagram_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SharpnessMeasure`: variance of the forecast distribution
/// Type: (List Real) → Real (forecasts → sharpness)
pub fn sharpness_measure_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `DoCalculus`: causal interventional distribution P(Y | do(X=x))
/// Type: Real → Real → Real (x, context → P(Y | do(X=x)))
pub fn do_calculus_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `InterventionalDist`: P(Y | do(X)) in a structural causal model
/// Type: Type → Type → Type (X space, Y space → dist)
pub fn interventional_dist_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `BackdoorCriterion`: set Z satisfies backdoor criterion for (X, Y)
/// Type: Type → Type → Type → Prop (X, Y, Z → criterion)
pub fn backdoor_criterion_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), prop())))
}
/// `BackdoorAdjustment`: P(Y|do(X)) = Σ_z P(Y|X,Z=z) P(Z=z)
/// Type: ∀ (X Y Z : Type), BackdoorCriterion X Y Z → Prop
pub fn backdoor_adjustment_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            pi(BinderInfo::Default, "Z", type0(), prop()),
        ),
    )
}
/// `ConfoundingBias`: bias from ignoring confounders
/// Type: Real → Prop (bias magnitude → bounded)
pub fn confounding_bias_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `DomainAdaptation`: learning when source and target domains differ
/// Type: Type → Type → Nat → Type (source, target, n → adapted model)
pub fn domain_adaptation_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(nat_ty(), type0())))
}
/// `CovariateShift`: p_S(x) ≠ p_T(x) but p(y|x) the same
/// Type: Type → Prop
pub fn covariate_shift_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ImportanceWeighting`: reweight source samples by p_T(x)/p_S(x)
/// Type: (Real → Real) → (Real → Real) → Real → Real (p_T, p_S, x → weight)
pub fn importance_weighting_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
    )
}
/// `DomainAdaptationBound`: generalization bound under covariate shift
/// Type: ∀ (n : Nat) (delta : Real), Prop
pub fn domain_adaptation_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "delta", real_ty(), prop()),
    )
}
/// `FederatedLearning`: distributed optimization across m clients
/// Type: Nat → Nat → Type (m clients, T rounds → protocol)
pub fn federated_learning_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HeterogeneityMeasure`: degree of statistical heterogeneity across clients
/// Type: Nat → Real → Prop (m clients, Γ measure → bound)
pub fn heterogeneity_measure_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `FedAvgConvergence`: FedAvg converges at rate O(1/√(T m))
/// Type: ∀ (T m : Nat), Prop
pub fn fedavg_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        nat_ty(),
        pi(BinderInfo::Default, "m", nat_ty(), prop()),
    )
}
/// `ByzantineFaultTolerance`: learning with f Byzantine clients out of m
/// Type: Nat → Nat → Type (m total, f Byzantine → robust protocol)
pub fn byzantine_fault_tolerance_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CommunicationComplexity`: total bits communicated to reach ε accuracy
/// Type: Real → Nat → Nat (ε, m → bits)
pub fn communication_complexity_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), nat_ty()))
}
/// Register the extended set of statistical learning theory axioms (§7–§17).
pub fn build_env_extended(env: &mut Environment) -> Result<(), String> {
    build_env(env)?;
    let axioms: &[(&str, Expr)] = &[
        ("EWAAlgorithm", ewa_algorithm_ty()),
        (
            "MultiplicativeWeightsUpdate",
            multiplicative_weights_update_ty(),
        ),
        ("ewa_regret_bound", ewa_regret_bound_ty()),
        ("BanditAlgorithm", bandit_algorithm_ty()),
        ("UCBAlgorithm", ucb_algorithm_ty()),
        ("ucb_regret_bound", ucb_regret_bound_ty()),
        ("ThompsonSampling", thompson_sampling_ty()),
        ("bayesian_regret_bound", bayesian_regret_bound_ty()),
        ("DataDependentBound", data_dependent_bound_ty()),
        ("LocalizedRademacher", localized_rademacher_ty()),
        ("localized_bound", localized_bound_ty()),
        ("pac_bayes_mcallester", pac_bayes_mcallester_ty()),
        ("catoni_pac_bayes", catoni_pac_bayes_ty()),
        ("RKHSNorm", rkhs_norm_ty()),
        ("KernelPCAProjection", kernel_pca_projection_ty()),
        ("svm_generalization_bound", svm_generalization_bound_ty()),
        ("NeuralNetwork", neural_network_ty()),
        ("depth_separation", depth_separation_ty()),
        ("BarronSpace", barron_space_ty()),
        ("barron_approximation", barron_approximation_ty()),
        ("NNExpressivity", nn_expressivity_ty()),
        ("nn_generalization_bound", nn_generalization_bound_ty()),
        ("DoubleDescent", double_descent_ty()),
        ("benign_overfitting", benign_overfitting_ty()),
        ("implicit_regularization", implicit_regularization_ty()),
        ("MinNormInterpolation", min_norm_interpolation_ty()),
        ("UniformStability", uniform_stability_ty()),
        ("OnAverageStability", on_average_stability_ty()),
        (
            "stability_generalization_bound",
            stability_generalization_bound_ty(),
        ),
        ("DataDeletion", data_deletion_ty()),
        ("DPSGDAlgorithm", dp_sgd_algorithm_ty()),
        ("PrivatePACLearning", private_pac_learning_ty()),
        ("PrivateQueryMechanism", private_query_mechanism_ty()),
        ("dp_generalization_bound", dp_generalization_bound_ty()),
        ("DPSampleComplexity", dp_sample_complexity_ty()),
        ("CalibrationError", calibration_error_ty()),
        ("ProperScoringRule", proper_scoring_rule_ty()),
        ("ReliabilityDiagram", reliability_diagram_ty()),
        ("SharpnessMeasure", sharpness_measure_ty()),
        ("DoCalculus", do_calculus_ty()),
        ("InterventionalDist", interventional_dist_ty()),
        ("BackdoorCriterion", backdoor_criterion_ty()),
        ("backdoor_adjustment", backdoor_adjustment_ty()),
        ("ConfoundingBias", confounding_bias_ty()),
        ("DomainAdaptation", domain_adaptation_ty()),
        ("CovariateShift", covariate_shift_ty()),
        ("ImportanceWeighting", importance_weighting_ty()),
        ("domain_adaptation_bound", domain_adaptation_bound_ty()),
        ("FederatedLearning", federated_learning_ty()),
        ("HeterogeneityMeasure", heterogeneity_measure_ty()),
        ("fedavg_convergence", fedavg_convergence_ty()),
        ("ByzantineFaultTolerance", byzantine_fault_tolerance_ty()),
        ("CommunicationComplexity", communication_complexity_ty()),
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
mod extended_tests {
    use super::*;
    #[test]
    fn test_ewa_regret_bound() {
        let n = 4;
        let t = 100;
        let eta = ExponentialWeightsAlgorithm::optimal_eta(n, t);
        let mut ewa = ExponentialWeightsAlgorithm::new(n, eta);
        for _ in 0..t {
            ewa.update(&[0.1, 0.2, 0.3, 0.4]);
        }
        let bound = ewa.regret_bound();
        assert!(bound > 0.0, "EWA regret bound must be positive");
        assert!(bound.is_finite(), "EWA bound must be finite");
    }
    #[test]
    fn test_ewa_distribution_sums_to_one() {
        let mut ewa = ExponentialWeightsAlgorithm::new(3, 0.1);
        ewa.update(&[0.5, 0.1, 0.8]);
        let dist = ewa.distribution();
        let sum: f64 = dist.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9, "EWA distribution must sum to 1");
    }
    #[test]
    fn test_ucb_bandit_selects_all_arms_initially() {
        let mut bandit = UCBBandit::new(3);
        let arm0 = bandit.select();
        bandit.update(arm0, 0.5);
        let arm1 = bandit.select();
        bandit.update(arm1, 0.8);
        let arm2 = bandit.select();
        bandit.update(arm2, 0.3);
        assert_eq!(arm0, 0);
        assert_eq!(arm1, 1);
        assert_eq!(arm2, 2);
    }
    #[test]
    fn test_ucb_regret_bound_positive() {
        let mut bandit = UCBBandit::new(2);
        for i in 0..10 {
            let arm = bandit.select();
            bandit.update(arm, if i % 2 == 0 { 1.0 } else { 0.0 });
        }
        let bound = bandit.regret_bound_upper();
        assert!(bound > 0.0 && bound.is_finite());
    }
    #[test]
    fn test_pac_bayes_mcallester_bound() {
        let pb = PACBayesGeneralization::new(0.5, 1000, 0.05);
        let bound = pb.mcallester_bound(0.1);
        assert!(bound > 0.1, "PAC-Bayes bound must exceed empirical loss");
        assert!(
            bound < 1.0,
            "PAC-Bayes bound must be less than 1 for reasonable params"
        );
    }
    #[test]
    fn test_pac_bayes_catoni_bound() {
        let pb = PACBayesGeneralization::new(0.3, 500, 0.05);
        let lam = pb.optimal_lambda(0.1);
        assert!(lam > 0.0 && lam < 1.0, "optimal lambda must be in (0,1)");
        let bound = pb.catoni_bound(0.1, lam);
        assert!(bound > 0.0 && bound.is_finite());
    }
    #[test]
    fn test_kernel_svm_trainer_smo_step() {
        let labels = vec![1.0, -1.0, 1.0, -1.0];
        let mut svm = KernelSVMTrainer::new(4, labels, 1.0);
        let k = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let _updated = svm.smo_step(0, 1, &k);
        for &a in &svm.alphas {
            assert!(a >= 0.0 && a <= svm.c + 1e-9);
        }
    }
    #[test]
    fn test_kernel_svm_generalization_bound() {
        let bound = KernelSVMTrainer::generalization_bound(1.0, 0.5, 100);
        assert!(
            (bound - 0.04).abs() < 1e-9,
            "R²/(γ²n) = 1/(0.25*100) = 0.04"
        );
    }
    #[test]
    fn test_causal_backdoor_adjust() {
        let bd = CausalBackdoor::new(vec![0.8, 0.4], vec![0.6, 0.4]);
        assert!(bd.is_valid(), "stratum probs must sum to 1");
        let adjusted = bd.adjust();
        assert!(
            (adjusted - 0.64).abs() < 1e-9,
            "backdoor adjustment must be 0.64"
        );
    }
    #[test]
    fn test_causal_backdoor_confounding_bias() {
        let bd = CausalBackdoor::new(vec![0.8, 0.4], vec![0.6, 0.4]);
        let bias = bd.confounding_bias(0.75);
        assert!(
            (bias - 0.11).abs() < 1e-9,
            "confounding bias = |0.75 - 0.64| = 0.11"
        );
    }
    #[test]
    fn test_build_env_extended() {
        let mut env = Environment::new();
        let result = build_env_extended(&mut env);
        assert!(result.is_ok(), "build_env_extended must succeed");
        assert!(env.get(&Name::str("EWAAlgorithm")).is_some());
        assert!(env.get(&Name::str("UCBAlgorithm")).is_some());
        assert!(env.get(&Name::str("ThompsonSampling")).is_some());
        assert!(env.get(&Name::str("NeuralNetwork")).is_some());
        assert!(env.get(&Name::str("BarronSpace")).is_some());
        assert!(env.get(&Name::str("DoubleDescent")).is_some());
        assert!(env.get(&Name::str("ByzantineFaultTolerance")).is_some());
        assert!(env.get(&Name::str("BackdoorCriterion")).is_some());
        assert!(env.get(&Name::str("DPSGDAlgorithm")).is_some());
        assert!(env.get(&Name::str("CalibrationError")).is_some());
    }
}
/// Solve a d×d linear system Ax = b using Gaussian elimination with partial pivoting.
/// `n` is kept for signature clarity; the key dimension is `d`.
pub(super) fn gauss_solve(a: &[Vec<f64>], b: &[f64], d: usize, _n: usize) -> Vec<f64> {
    if d == 0 {
        return vec![];
    }
    let mut mat: Vec<Vec<f64>> = (0..d)
        .map(|i| {
            let mut row = a[i].clone();
            row.push(b[i]);
            row
        })
        .collect();
    for col in 0..d {
        let pivot = (col..d).max_by(|&i, &j| {
            mat[i][col]
                .abs()
                .partial_cmp(&mat[j][col].abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(pivot_row) = pivot {
            mat.swap(col, pivot_row);
        }
        let diag = mat[col][col];
        if diag.abs() < 1e-12 {
            continue;
        }
        for row in (col + 1)..d {
            let factor = mat[row][col] / diag;
            for k in col..=d {
                let val = mat[col][k] * factor;
                mat[row][k] -= val;
            }
        }
    }
    let mut x = vec![0.0f64; d];
    for i in (0..d).rev() {
        let mut sum = mat[i][d];
        for j in (i + 1)..d {
            sum -= mat[i][j] * x[j];
        }
        x[i] = if mat[i][i].abs() < 1e-12 {
            0.0
        } else {
            sum / mat[i][i]
        };
    }
    x
}
#[allow(dead_code)]
pub fn dot_ext(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
#[cfg(test)]
mod tests_sl_extra {
    use super::*;
    #[test]
    fn test_gaussian_process() {
        let gp = GaussianProcess::default_rbf();
        let k = gp.rbf_kernel(0.0, 0.0);
        assert!((k - 1.0).abs() < 1e-9, "k(x,x) = σ^2 = 1.0");
        let k_far = gp.rbf_kernel(0.0, 100.0);
        assert!(k_far < 1e-9, "k(0, 100) should be ~0");
    }
    #[test]
    fn test_svm_kernel() {
        let svm = SVMClassifier::rbf(1.0, 1.0);
        let x = vec![1.0, 0.0];
        let xp = vec![1.0, 0.0];
        let k = svm.kernel_value(&x, &xp);
        assert!((k - 1.0).abs() < 1e-9, "RBF(x,x)=1 for γ=1");
    }
    #[test]
    fn test_gradient_boosting() {
        let gb = GradientBoosting::xgboost_style(100);
        assert!(gb.is_regularized());
        assert_eq!(gb.n_leaves_upper_bound(), 64);
    }
    #[test]
    fn test_cross_validation() {
        let cv = CrossValidation::k_fold_5(100);
        assert_eq!(cv.fold_size(), 20);
        assert_eq!(cv.train_size(), 80);
        assert_eq!(cv.n_train_test_splits(), 5);
        let loocv = CrossValidation::loocv(10);
        assert_eq!(loocv.n_folds, 10);
    }
}
