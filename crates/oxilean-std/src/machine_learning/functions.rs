//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Activation, AdamOptimizer, DecisionStump, ElasticWeightConsolidation, GradientDescent, KMeans,
    KnnClassifier, Layer, LinearRegression, MomentumSGD, NeuralNetwork, PACBayesBound,
    PolynomialRegression, RandomizedSmoothingClassifier, ShapleyExplainer, UncertaintySampler,
    UncertaintyStrategy,
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
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn learner_ty() -> Expr {
    type0()
}
pub fn vc_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
pub fn pac_learnable_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn neural_network_ty() -> Expr {
    type0()
}
pub fn gradient_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
pub fn kernel_method_ty() -> Expr {
    type0()
}
pub fn loss_function_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
pub fn regularizer_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
pub fn cross_validation_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), real_ty()))
}
pub fn fundamental_thm_pac_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        app2(
            cst("Iff"),
            app(cst("PACLearnable"), cst("H")),
            app2(
                cst("Nat.lt"),
                app(cst("VCDimension"), cst("H")),
                cst("Nat.infinity"),
            ),
        ),
    )
}
pub fn universal_approximation_ty() -> Expr {
    prop()
}
pub fn vc_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps",
        real_ty(),
        pi(
            BinderInfo::Default,
            "delta",
            real_ty(),
            arrow(
                app2(cst("Real.lt"), cst("Real.zero"), cst("eps")),
                arrow(
                    app2(cst("Real.lt"), cst("Real.zero"), cst("delta")),
                    app(
                        cst("Exists"),
                        pi(
                            BinderInfo::Default,
                            "m",
                            nat_ty(),
                            app2(
                                app(cst("GeneralizationBound"), cst("m")),
                                cst("eps"),
                                cst("delta"),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
pub fn no_free_lunch_ty() -> Expr {
    prop()
}
pub fn bias_variance_tradeoff_ty() -> Expr {
    prop()
}
pub fn regularization_convergence_ty() -> Expr {
    prop()
}
pub fn build_machine_learning_env(env: &mut Environment) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("Learner", learner_ty()),
        ("VCDimension", vc_dimension_ty()),
        ("PACLearnable", pac_learnable_ty()),
        ("NeuralNetwork", neural_network_ty()),
        ("Gradient", gradient_ty()),
        ("KernelMethod", kernel_method_ty()),
        ("LossFunction", loss_function_ty()),
        ("Regularizer", regularizer_ty()),
        ("CrossValidation", cross_validation_ty()),
        (
            "GeneralizationBound",
            arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
        ("Real.zero", real_ty()),
        ("Nat.infinity", nat_ty()),
        ("fundamental_thm_pac", fundamental_thm_pac_ty()),
        ("universal_approximation", universal_approximation_ty()),
        ("vc_bound", vc_bound_ty()),
        ("no_free_lunch", no_free_lunch_ty()),
        ("bias_variance_tradeoff", bias_variance_tradeoff_ty()),
        (
            "regularization_convergence",
            regularization_convergence_ty(),
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
pub fn mse_loss(y_pred: &[f64], y_true: &[f64]) -> f64 {
    let n = y_pred.len().min(y_true.len());
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(p, t)| (p - t).powi(2))
        .sum();
    sum / n as f64
}
pub fn mae_loss(y_pred: &[f64], y_true: &[f64]) -> f64 {
    let n = y_pred.len().min(y_true.len());
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(p, t)| (p - t).abs())
        .sum();
    sum / n as f64
}
pub fn binary_cross_entropy(y_pred: &[f64], y_true: &[f64]) -> f64 {
    let n = y_pred.len().min(y_true.len());
    if n == 0 {
        return 0.0;
    }
    let eps = 1e-15;
    let sum: f64 = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(&p, &t)| {
            let p_clamped = p.clamp(eps, 1.0 - eps);
            -(t * p_clamped.ln() + (1.0 - t) * (1.0 - p_clamped).ln())
        })
        .sum();
    sum / n as f64
}
pub fn hinge_loss(y_pred: &[f64], y_true: &[f64]) -> f64 {
    let n = y_pred.len().min(y_true.len());
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(&p, &t)| (1.0 - t * p).max(0.0))
        .sum();
    sum / n as f64
}
pub fn huber_loss(y_pred: &[f64], y_true: &[f64], delta: f64) -> f64 {
    let n = y_pred.len().min(y_true.len());
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = y_pred
        .iter()
        .zip(y_true.iter())
        .map(|(&p, &t)| {
            let err = (p - t).abs();
            if err <= delta {
                0.5 * err * err
            } else {
                delta * err - 0.5 * delta * delta
            }
        })
        .sum();
    sum / n as f64
}
pub fn min_max_normalize(data: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if data.is_empty() {
        return vec![];
    }
    let dim = data[0].len();
    let mut mins = vec![f64::INFINITY; dim];
    let mut maxs = vec![f64::NEG_INFINITY; dim];
    for row in data {
        for (j, &v) in row.iter().enumerate() {
            if j < dim {
                mins[j] = mins[j].min(v);
                maxs[j] = maxs[j].max(v);
            }
        }
    }
    data.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(j, &v)| {
                    let range = maxs[j] - mins[j];
                    if range.abs() < 1e-15 {
                        0.0
                    } else {
                        (v - mins[j]) / range
                    }
                })
                .collect()
        })
        .collect()
}
pub fn z_score_normalize(data: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if data.is_empty() {
        return vec![];
    }
    let dim = data[0].len();
    let n = data.len() as f64;
    let mut means = vec![0.0f64; dim];
    for row in data {
        for (j, &v) in row.iter().enumerate() {
            if j < dim {
                means[j] += v;
            }
        }
    }
    for m in &mut means {
        *m /= n;
    }
    let mut stds = vec![0.0f64; dim];
    for row in data {
        for (j, &v) in row.iter().enumerate() {
            if j < dim {
                stds[j] += (v - means[j]).powi(2);
            }
        }
    }
    for s in &mut stds {
        *s = (*s / n).sqrt();
    }
    data.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(j, &v)| {
                    if stds[j].abs() < 1e-15 {
                        0.0
                    } else {
                        (v - means[j]) / stds[j]
                    }
                })
                .collect()
        })
        .collect()
}
pub fn backprop_mse(
    net: &NeuralNetwork,
    input: &[f64],
    target: &[f64],
) -> (Vec<Vec<Vec<f64>>>, Vec<Vec<f64>>) {
    let (activations, z_cache, _) = net.forward_cached(input);
    let n_layers = net.layers.len();
    let mut weight_grads: Vec<Vec<Vec<f64>>> = Vec::with_capacity(n_layers);
    let mut bias_grads: Vec<Vec<f64>> = Vec::with_capacity(n_layers);
    for layer in &net.layers {
        weight_grads.push(vec![vec![0.0; layer.n_inputs()]; layer.n_outputs()]);
        bias_grads.push(vec![0.0; layer.n_outputs()]);
    }
    let output = &activations[n_layers];
    let mut delta: Vec<f64> = output
        .iter()
        .zip(target.iter())
        .enumerate()
        .map(|(j, (&a, &t))| {
            let dl_da = 2.0 * (a - t) / target.len() as f64;
            let da_dz = net.layers[n_layers - 1]
                .activation
                .derivative(z_cache[n_layers - 1][j]);
            dl_da * da_dz
        })
        .collect();
    for l in (0..n_layers).rev() {
        let input_to_layer = &activations[l];
        for j in 0..delta.len() {
            bias_grads[l][j] = delta[j];
            for k in 0..input_to_layer.len() {
                weight_grads[l][j][k] = delta[j] * input_to_layer[k];
            }
        }
        if l > 0 {
            let layer = &net.layers[l];
            let prev_z = &z_cache[l - 1];
            let mut new_delta = vec![0.0; prev_z.len()];
            for k in 0..prev_z.len() {
                let mut sum = 0.0;
                for j in 0..delta.len() {
                    if k < layer.weights[j].len() {
                        sum += delta[j] * layer.weights[j][k];
                    }
                }
                new_delta[k] = sum * net.layers[l - 1].activation.derivative(prev_z[k]);
            }
            delta = new_delta;
        }
    }
    (weight_grads, bias_grads)
}
pub fn train_network(
    net: &mut NeuralNetwork,
    inputs: &[Vec<f64>],
    targets: &[Vec<f64>],
    learning_rate: f64,
    epochs: u32,
) -> Vec<f64> {
    let mut loss_history = Vec::new();
    let n = inputs.len().min(targets.len());
    if n == 0 {
        return loss_history;
    }
    for _epoch in 0..epochs {
        let mut total_loss = 0.0;
        for i in 0..n {
            let output = net.forward(&inputs[i]);
            total_loss += mse_loss(&output, &targets[i]);
            let (w_grads, b_grads) = backprop_mse(net, &inputs[i], &targets[i]);
            for (l, layer) in net.layers.iter_mut().enumerate() {
                for j in 0..layer.weights.len() {
                    for k in 0..layer.weights[j].len() {
                        layer.weights[j][k] -= learning_rate * w_grads[l][j][k];
                    }
                    layer.biases[j] -= learning_rate * b_grads[l][j];
                }
            }
        }
        loss_history.push(total_loss / n as f64);
    }
    loss_history
}
pub fn accuracy(predicted: &[usize], actual: &[usize]) -> f64 {
    let n = predicted.len().min(actual.len());
    if n == 0 {
        return 0.0;
    }
    let correct = predicted
        .iter()
        .zip(actual.iter())
        .filter(|(p, a)| p == a)
        .count();
    correct as f64 / n as f64
}
pub fn precision(predicted: &[usize], actual: &[usize], class: usize) -> f64 {
    let tp = predicted
        .iter()
        .zip(actual.iter())
        .filter(|(&p, &a)| p == class && a == class)
        .count();
    let pp = predicted.iter().filter(|&&p| p == class).count();
    if pp == 0 {
        0.0
    } else {
        tp as f64 / pp as f64
    }
}
pub fn recall(predicted: &[usize], actual: &[usize], class: usize) -> f64 {
    let tp = predicted
        .iter()
        .zip(actual.iter())
        .filter(|(&p, &a)| p == class && a == class)
        .count();
    let ap = actual.iter().filter(|&&a| a == class).count();
    if ap == 0 {
        0.0
    } else {
        tp as f64 / ap as f64
    }
}
pub fn f1_score(predicted: &[usize], actual: &[usize], class: usize) -> f64 {
    let p = precision(predicted, actual, class);
    let r = recall(predicted, actual, class);
    if p + r < 1e-15 {
        0.0
    } else {
        2.0 * p * r / (p + r)
    }
}
pub fn l2_penalty(weights: &[f64], lambda: f64) -> f64 {
    lambda * weights.iter().map(|w| w * w).sum::<f64>()
}
pub fn l1_penalty(weights: &[f64], lambda: f64) -> f64 {
    lambda * weights.iter().map(|w| w.abs()).sum::<f64>()
}
pub fn elastic_net_penalty(weights: &[f64], lambda: f64, alpha: f64) -> f64 {
    alpha * l1_penalty(weights, lambda) + (1.0 - alpha) * l2_penalty(weights, lambda)
}
pub fn train_test_split<T: Clone>(data: &[T], frac: f64) -> (Vec<T>, Vec<T>) {
    let split = ((data.len() as f64) * frac).round() as usize;
    let split = split.min(data.len());
    (data[..split].to_vec(), data[split..].to_vec())
}
pub fn k_fold_indices(n: usize, k: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    if k == 0 || n == 0 {
        return vec![];
    }
    let fold_size = n / k;
    (0..k)
        .map(|i| {
            let test_start = i * fold_size;
            let test_end = if i == k - 1 { n } else { (i + 1) * fold_size };
            let test_indices: Vec<usize> = (test_start..test_end).collect();
            let train_indices: Vec<usize> =
                (0..n).filter(|idx| !test_indices.contains(idx)).collect();
            (train_indices, test_indices)
        })
        .collect()
}
pub fn neural_tangent_kernel_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), real_ty()),
    )
}
pub fn lazy_training_regime_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn mean_field_limit_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn infinite_width_limit_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn loss_landscape_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
pub fn autoencoder_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
pub fn contrastive_loss_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
pub fn self_supervised_objective_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn representation_collapse_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn disentanglement_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
pub fn message_passing_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
pub fn wl_expressiveness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn graph_isomorphism_power_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn over_smoothing_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn attention_mechanism_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), list_ty(real_ty())),
    )
}
pub fn transformer_universality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn in_context_learning_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
pub fn positional_encoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), list_ty(real_ty())))
}
pub fn pac_mdp_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
pub fn regret_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn sample_complexity_rl_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), nat_ty()))
}
pub fn exploration_exploitation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn vae_elbo_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
pub fn gan_equilibrium_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
pub fn normalizing_flow_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
pub fn diffusion_process_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(type0(), type0())))
}
pub fn score_matching_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), list_ty(real_ty())))
}
pub fn maml_convergence_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn few_shot_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
pub fn task_distribution_ty() -> Expr {
    arrow(type0(), real_ty())
}
pub fn catastrophic_forgetting_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn ewc_regularizer_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
pub fn memory_replay_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn negative_transfer_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
pub fn task_relatedness_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
pub fn transfer_excess_risk_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(nat_ty(), real_ty())))
}
pub fn demographic_parity_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
pub fn equalized_odds_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
pub fn individual_fairness_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), prop())
}
pub fn fairness_accuracy_tradeoff_ty() -> Expr {
    prop()
}
pub fn shapley_value_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(nat_ty(), real_ty()),
    )
}
pub fn shap_attribution_ty() -> Expr {
    arrow(type0(), arrow(list_ty(real_ty()), list_ty(real_ty())))
}
pub fn counterfactual_explanation_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(type0(), list_ty(real_ty())))
}
pub fn lp_adversarial_attack_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), list_ty(real_ty())))
}
pub fn certified_defense_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), prop()))
}
pub fn randomized_smoothing_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), arrow(real_ty(), prop())))
}
pub fn nas_search_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn one_shot_nas_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
pub fn cell_based_nas_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
pub fn query_complexity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), nat_ty()))
}
pub fn uncertainty_sampling_ty() -> Expr {
    arrow(type0(), arrow(list_ty(real_ty()), real_ty()))
}
pub fn optimal_stopping_al_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
pub fn pac_bayes_mcallester_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), real_ty())))
}
pub fn pac_bayes_catoni_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), real_ty())))
}
pub fn data_dependent_prior_ty() -> Expr {
    arrow(list_ty(type0()), arrow(type0(), real_ty()))
}
pub fn kl_divergence_bound_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(nat_ty(), real_ty())))
}
pub fn register_advanced_ml_axioms(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("NeuralTangentKernel", neural_tangent_kernel_ty()),
        ("LazyTrainingRegime", lazy_training_regime_ty()),
        ("MeanFieldLimit", mean_field_limit_ty()),
        ("InfiniteWidthLimit", infinite_width_limit_ty()),
        ("LossLandscape", loss_landscape_ty()),
        ("Autoencoder", autoencoder_ty()),
        ("ContrastiveLoss", contrastive_loss_ty()),
        ("SelfSupervisedObjective", self_supervised_objective_ty()),
        ("RepresentationCollapse", representation_collapse_ty()),
        ("Disentanglement", disentanglement_ty()),
        ("MessagePassing", message_passing_ty()),
        ("WLExpressiveness", wl_expressiveness_ty()),
        ("GraphIsomorphismPower", graph_isomorphism_power_ty()),
        ("OverSmoothing", over_smoothing_ty()),
        ("AttentionMechanism", attention_mechanism_ty()),
        ("TransformerUniversality", transformer_universality_ty()),
        ("InContextLearning", in_context_learning_ty()),
        ("PositionalEncoding", positional_encoding_ty()),
        ("PACMDP", pac_mdp_ty()),
        ("RegretBound", regret_bound_ty()),
        ("SampleComplexityRL", sample_complexity_rl_ty()),
        ("ExplorationExploitation", exploration_exploitation_ty()),
        ("VAELBO", vae_elbo_ty()),
        ("GANEquilibrium", gan_equilibrium_ty()),
        ("NormalizingFlow", normalizing_flow_ty()),
        ("DiffusionProcess", diffusion_process_ty()),
        ("ScoreMatching", score_matching_ty()),
        ("MAMLConvergence", maml_convergence_ty()),
        ("FewShotBound", few_shot_bound_ty()),
        ("TaskDistribution", task_distribution_ty()),
        ("CatastrophicForgetting", catastrophic_forgetting_ty()),
        ("EWCRegularizer", ewc_regularizer_ty()),
        ("MemoryReplayBound", memory_replay_bound_ty()),
        ("NegativeTransfer", negative_transfer_ty()),
        ("TaskRelatedness", task_relatedness_ty()),
        ("TransferExcessRisk", transfer_excess_risk_ty()),
        ("DemographicParity", demographic_parity_ty()),
        ("EqualizedOdds", equalized_odds_ty()),
        ("IndividualFairness", individual_fairness_ty()),
        ("FairnessAccuracyTradeoff", fairness_accuracy_tradeoff_ty()),
        ("ShapleyValue", shapley_value_ty()),
        ("SHAPAttribution", shap_attribution_ty()),
        ("CounterfactualExplanation", counterfactual_explanation_ty()),
        ("LpAdversarialAttack", lp_adversarial_attack_ty()),
        ("CertifiedDefense", certified_defense_ty()),
        ("RandomizedSmoothing", randomized_smoothing_ty()),
        ("NASSearchSpace", nas_search_space_ty()),
        ("OneShotNAS", one_shot_nas_ty()),
        ("CellBasedNAS", cell_based_nas_ty()),
        ("QueryComplexity", query_complexity_ty()),
        ("UncertaintySampling", uncertainty_sampling_ty()),
        ("OptimalStoppingAL", optimal_stopping_al_ty()),
        ("PACBayesMcAllester", pac_bayes_mcallester_ty()),
        ("PACBayesCatoni", pac_bayes_catoni_ty()),
        ("DataDependentPrior", data_dependent_prior_ty()),
        ("KLDivergenceBound", kl_divergence_bound_ty()),
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
    fn test_activation_relu() {
        let a = Activation::ReLU;
        assert_eq!(a.apply(-1.0), 0.0);
        assert_eq!(a.apply(2.0), 2.0);
        assert_eq!(a.apply(0.0), 0.0);
    }
    #[test]
    fn test_activation_sigmoid() {
        let a = Activation::Sigmoid;
        assert!((a.apply(0.0) - 0.5).abs() < 1e-10);
        assert!(a.apply(100.0) > 0.999);
        assert!(a.apply(-100.0) < 0.001);
    }
    #[test]
    fn test_activation_leaky_relu() {
        let a = Activation::LeakyReLU;
        assert_eq!(a.apply(5.0), 5.0);
        assert!((a.apply(-10.0) - (-0.1)).abs() < 1e-10);
    }
    #[test]
    fn test_activation_elu() {
        let a = Activation::ELU;
        assert_eq!(a.apply(3.0), 3.0);
        assert!(a.apply(-1.0) < 0.0);
    }
    #[test]
    fn test_softmax() {
        let vals = vec![1.0, 2.0, 3.0];
        let sm = Activation::apply_softmax(&vals);
        let sum: f64 = sm.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
        assert!(sm[2] > sm[1] && sm[1] > sm[0]);
    }
    #[test]
    fn test_layer_forward() {
        let layer = Layer::from_weights(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            Activation::Linear,
        );
        let out = layer.forward(&[3.0, 5.0]);
        assert!((out[0] - 3.0).abs() < 1e-10);
        assert!((out[1] - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_layer_with_cache() {
        let layer = Layer::from_weights(
            vec![vec![2.0, 0.0], vec![0.0, 3.0]],
            vec![1.0, -1.0],
            Activation::ReLU,
        );
        let (z, a) = layer.forward_with_cache(&[1.0, 2.0]);
        assert!((z[0] - 3.0).abs() < 1e-10);
        assert!((z[1] - 5.0).abs() < 1e-10);
        assert!((a[0] - 3.0).abs() < 1e-10);
        assert!((a[1] - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_neural_network_forward() {
        let layer = Layer::from_weights(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            Activation::Linear,
        );
        let net = NeuralNetwork::new(vec![layer]);
        let out = net.forward(&[1.0, 2.0]);
        assert!((out[0] - 1.0).abs() < 1e-10);
        assert!((out[1] - 2.0).abs() < 1e-10);
        assert_eq!(net.predict_class(&[1.0, 2.0]), 1);
        assert_eq!(net.depth(), 1);
    }
    #[test]
    fn test_backprop_reduces_loss() {
        let layer = Layer::from_weights(vec![vec![0.5]], vec![0.0], Activation::Linear);
        let mut net = NeuralNetwork::new(vec![layer]);
        let inputs = vec![vec![1.0], vec![2.0], vec![3.0]];
        let targets = vec![vec![1.0], vec![2.0], vec![3.0]];
        let history = train_network(&mut net, &inputs, &targets, 0.01, 100);
        assert!(
            history.last().expect("last should succeed")
                < history.first().expect("first should succeed")
        );
    }
    #[test]
    fn test_loss_functions() {
        let pred = vec![1.0, 2.0, 3.0];
        let true_vals = vec![1.0, 2.0, 3.0];
        assert!(mse_loss(&pred, &true_vals) < 1e-10);
        assert!(mae_loss(&pred, &true_vals) < 1e-10);
    }
    #[test]
    fn test_binary_cross_entropy() {
        let pred = vec![0.9, 0.1];
        let true_vals = vec![1.0, 0.0];
        let bce = binary_cross_entropy(&pred, &true_vals);
        let bad_pred = vec![0.1, 0.9];
        let bad_bce = binary_cross_entropy(&bad_pred, &true_vals);
        assert!(bad_bce > bce);
    }
    #[test]
    fn test_hinge_loss() {
        let pred = vec![2.0, -2.0];
        let true_labels = vec![1.0, -1.0];
        assert!(hinge_loss(&pred, &true_labels) < 1e-10);
    }
    #[test]
    fn test_huber_loss() {
        let pred = vec![1.0, 2.0];
        let true_vals = vec![1.0, 2.0];
        assert!(huber_loss(&pred, &true_vals, 1.0) < 1e-10);
    }
    #[test]
    fn test_knn_classifier_simple() {
        let mut knn = KnnClassifier::new(1);
        knn.fit(vec![
            (vec![0.0, 0.0], 0),
            (vec![1.0, 0.0], 1),
            (vec![0.0, 1.0], 0),
        ]);
        assert_eq!(knn.predict(&[0.1, 0.1]), 0);
        assert_eq!(knn.predict(&[0.9, 0.1]), 1);
    }
    #[test]
    fn test_knn_predict_proba() {
        let mut knn = KnnClassifier::new(3);
        knn.fit(vec![(vec![0.0], 0), (vec![0.1], 0), (vec![0.2], 1)]);
        let proba = knn.predict_proba(&[0.05]);
        let p0 = proba.get(&0).copied().unwrap_or(0.0);
        assert!(p0 >= 0.5);
    }
    #[test]
    fn test_decision_stump() {
        let stump = DecisionStump::new(0, 5.0, 1);
        assert_eq!(stump.predict(&[6.0]), 1);
        assert_eq!(stump.predict(&[4.0]), 0);
    }
    #[test]
    fn test_decision_stump_find_best() {
        let data = vec![
            (vec![1.0], 0),
            (vec![2.0], 0),
            (vec![3.0], 1),
            (vec![4.0], 1),
        ];
        let weights = vec![0.25, 0.25, 0.25, 0.25];
        let best = DecisionStump::find_best(&data, &weights);
        let correct: usize = data.iter().filter(|(x, y)| best.predict(x) == *y).count();
        assert!(correct >= 3);
    }
    #[test]
    fn test_kmeans_fit_2_clusters() {
        let data = vec![
            vec![0.0, 0.0],
            vec![0.1, 0.0],
            vec![10.0, 10.0],
            vec![10.1, 10.0],
        ];
        let mut km = KMeans::new(2, 100);
        let assignments = km.fit(&data, 42);
        assert_eq!(assignments[0], assignments[1]);
        assert_eq!(assignments[2], assignments[3]);
        assert_ne!(assignments[0], assignments[2]);
    }
    #[test]
    fn test_kmeans_inertia() {
        let data = vec![vec![0.0], vec![10.0]];
        let mut km = KMeans::new(2, 100);
        km.fit(&data, 0);
        assert!(km.inertia(&data) < 1.0);
    }
    #[test]
    fn test_linear_regression_fit_with_bias() {
        let x_data = vec![vec![1.0], vec![2.0], vec![3.0]];
        let y_data = vec![3.0, 5.0, 7.0];
        let model = LinearRegression::fit_least_squares(&x_data, &y_data);
        assert!((model.weights[0] - 2.0).abs() < 1e-8);
        assert!((model.bias - 1.0).abs() < 1e-8);
        assert!(model.r_squared(&x_data, &y_data) > 0.999);
    }
    #[test]
    fn test_linear_regression_mse() {
        let model = LinearRegression {
            weights: vec![1.0],
            bias: 0.0,
        };
        assert!(model.mse(&[vec![1.0], vec![2.0]], &[1.0, 2.0]) < 1e-10);
    }
    #[test]
    fn test_polynomial_regression() {
        let x_data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y_data: Vec<f64> = x_data.iter().map(|&x| x * x).collect();
        let model = PolynomialRegression::fit(&x_data, &y_data, 2, 0.0001, 5000);
        let pred = model.predict(5.0);
        assert!((pred - 25.0).abs() < 5.0, "Got {}", pred);
    }
    #[test]
    fn test_gradient_descent_quadratic() {
        let gd = GradientDescent::new(0.1, 10_000);
        let x_min = gd.minimize_quadratic(1.0, 0.0, 5.0);
        assert!(x_min.abs() < 1e-4, "got {x_min}");
    }
    #[test]
    fn test_gradient_descent_numerical() {
        let gd = GradientDescent::new(0.01, 10_000);
        let x_min = gd.minimize_numerical(&|x: f64| (x - 3.0).powi(2), 0.0);
        assert!((x_min - 3.0).abs() < 0.1, "got {x_min}");
    }
    #[test]
    fn test_momentum_sgd() {
        let opt = MomentumSGD::new(0.01, 0.9, 10_000);
        let x_min = opt.minimize_quadratic(1.0, 0.0, 10.0);
        assert!(x_min.abs() < 0.1, "got {x_min}");
    }
    #[test]
    fn test_adam_optimizer() {
        let opt = AdamOptimizer::new(0.1, 10_000);
        let x_min = opt.minimize_quadratic(1.0, 0.0, 10.0);
        assert!(x_min.abs() < 0.1, "got {x_min}");
    }
    #[test]
    fn test_min_max_normalize() {
        let data = vec![vec![0.0, 10.0], vec![5.0, 20.0], vec![10.0, 30.0]];
        let normed = min_max_normalize(&data);
        assert!((normed[0][0]).abs() < 1e-10);
        assert!((normed[2][0] - 1.0).abs() < 1e-10);
        assert!((normed[1][0] - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_z_score_normalize() {
        let data = vec![vec![1.0], vec![2.0], vec![3.0]];
        let normed = z_score_normalize(&data);
        let mean: f64 = normed.iter().map(|r| r[0]).sum::<f64>() / 3.0;
        assert!(mean.abs() < 1e-10);
    }
    #[test]
    fn test_accuracy_metric() {
        assert!((accuracy(&[0, 1, 1, 0], &[0, 1, 0, 0]) - 0.75).abs() < 1e-10);
    }
    #[test]
    fn test_precision_recall_f1() {
        let pred = vec![1, 1, 0, 0, 1];
        let actual = vec![1, 0, 0, 1, 1];
        let p = precision(&pred, &actual, 1);
        let r = recall(&pred, &actual, 1);
        let _f1 = f1_score(&pred, &actual, 1);
        assert!((p - 2.0 / 3.0).abs() < 1e-10);
        assert!((r - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_regularization() {
        let w = vec![1.0, 2.0, 3.0];
        assert!((l2_penalty(&w, 0.1) - 1.4).abs() < 1e-10);
        assert!((l1_penalty(&w, 0.1) - 0.6).abs() < 1e-10);
    }
    #[test]
    fn test_elastic_net() {
        let w = vec![1.0, 2.0];
        assert!((elastic_net_penalty(&w, 1.0, 0.0) - l2_penalty(&w, 1.0)).abs() < 1e-10);
        assert!((elastic_net_penalty(&w, 1.0, 1.0) - l1_penalty(&w, 1.0)).abs() < 1e-10);
    }
    #[test]
    fn test_train_test_split() {
        let data: Vec<i32> = (0..10).collect();
        let (train, test) = train_test_split(&data, 0.8);
        assert_eq!(train.len(), 8);
        assert_eq!(test.len(), 2);
    }
    #[test]
    fn test_k_fold_indices() {
        let folds = k_fold_indices(10, 5);
        assert_eq!(folds.len(), 5);
        for (train, test) in &folds {
            assert_eq!(test.len(), 2);
            assert_eq!(train.len(), 8);
        }
    }
    #[test]
    fn test_build_machine_learning_env() {
        let mut env = Environment::new();
        build_machine_learning_env(&mut env).expect("build_machine_learning_env should succeed");
        assert!(!env.is_empty());
    }
    #[test]
    fn test_ewc_penalty_zero_at_theta_star() {
        let mut ewc = ElasticWeightConsolidation::new(1.0);
        let params = vec![1.0, 2.0, 3.0];
        let grads = vec![vec![0.5, 0.5, 0.5], vec![0.5, 0.5, 0.5]];
        ewc.consolidate(&params, &grads);
        let pen = ewc.penalty(&params);
        assert!(pen.abs() < 1e-10, "penalty at theta* = {pen}");
    }
    #[test]
    fn test_ewc_penalty_nonzero_elsewhere() {
        let mut ewc = ElasticWeightConsolidation::new(1.0);
        let params = vec![1.0, 2.0];
        let grads = vec![vec![1.0, 1.0]];
        ewc.consolidate(&params, &grads);
        let shifted = vec![2.0, 3.0];
        let pen = ewc.penalty(&shifted);
        assert!(pen > 0.0, "penalty should be positive when shifted");
    }
    #[test]
    fn test_ewc_penalty_gradient_direction() {
        let mut ewc = ElasticWeightConsolidation::new(1.0);
        let theta_star = vec![0.0, 0.0];
        let grads = vec![vec![1.0, 1.0]];
        ewc.consolidate(&theta_star, &grads);
        let params = vec![1.0, -1.0];
        let grad = ewc.penalty_gradient(&params);
        assert!(grad[0] > 0.0);
        assert!(grad[1] < 0.0);
    }
    #[test]
    fn test_shapley_explainer_constant_model() {
        let bg = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let explainer = ShapleyExplainer::new(2, 200, bg);
        let x = vec![3.0, 4.0];
        let phi = explainer.explain(&x, &|_inp: &[f64]| 5.0_f64);
        for &v in &phi {
            assert!(
                v.abs() < 1e-10,
                "constant model => all Shapley = 0, got {v}"
            );
        }
    }
    #[test]
    fn test_shapley_explainer_linear_model() {
        let bg = vec![vec![0.0, 0.0]];
        let explainer = ShapleyExplainer::new(2, 500, bg);
        let x = vec![3.0, 5.0];
        let phi = explainer.explain(&x, &|inp: &[f64]| inp.iter().sum::<f64>());
        assert!((phi[0] - 3.0).abs() < 0.5, "phi[0]={}", phi[0]);
        assert!((phi[1] - 5.0).abs() < 0.5, "phi[1]={}", phi[1]);
    }
    #[test]
    fn test_randomized_smoothing_predict() {
        let smoother = RandomizedSmoothingClassifier::new(0.1, 200, 0.95);
        let base = |x: &[f64]| if x[0] > 0.5 { 1usize } else { 0usize };
        let cls = smoother.smooth_predict(&[1.0, 0.0], &base);
        assert_eq!(cls, 1);
    }
    #[test]
    fn test_randomized_smoothing_certify() {
        let smoother = RandomizedSmoothingClassifier::new(0.1, 500, 0.95);
        let base = |x: &[f64]| if x[0] > 0.5 { 1usize } else { 0usize };
        let (cls, radius) = smoother.certify(&[1.0, 0.0], &base);
        assert_eq!(cls, 1);
        assert!(radius >= 0.0);
    }
    #[test]
    fn test_pac_bayes_mcallester() {
        let bound_calc = PACBayesBound::new(0.05, 1000);
        let bound = bound_calc.mcallester(0.1, 1.0);
        assert!(bound > 0.1);
        assert!(bound.is_finite());
    }
    #[test]
    fn test_pac_bayes_catoni() {
        let bound_calc = PACBayesBound::new(0.05, 1000);
        let bound = bound_calc.catoni(0.1, 1.0, 1.0);
        assert!(bound.is_finite());
        assert!(bound > 0.0);
    }
    #[test]
    fn test_pac_bayes_kl_bernoulli() {
        let kl = PACBayesBound::kl_bernoulli(0.3, 0.3);
        assert!(kl.abs() < 1e-10);
        assert!(PACBayesBound::kl_bernoulli(0.1, 0.9) > 0.0);
    }
    #[test]
    fn test_pac_bayes_kl_gaussians() {
        let kl = PACBayesBound::kl_gaussians(1.0, 1.0, 1.0);
        assert!(kl.abs() < 1e-10);
        assert!(PACBayesBound::kl_gaussians(0.0, 1.0, 1.0) > 0.0);
    }
    #[test]
    fn test_uncertainty_sampler_least_confident() {
        let sampler = UncertaintySampler::new(UncertaintyStrategy::LeastConfident);
        let certain = vec![0.99, 0.01];
        let uncertain = vec![0.5, 0.5];
        assert!(sampler.score(&uncertain) > sampler.score(&certain));
    }
    #[test]
    fn test_uncertainty_sampler_margin() {
        let sampler = UncertaintySampler::new(UncertaintyStrategy::MarginSampling);
        let small_margin = vec![0.51, 0.49];
        let large_margin = vec![0.99, 0.01];
        assert!(sampler.score(&small_margin) > sampler.score(&large_margin));
    }
    #[test]
    fn test_uncertainty_sampler_entropy() {
        let sampler = UncertaintySampler::new(UncertaintyStrategy::Entropy);
        let uniform = vec![0.25, 0.25, 0.25, 0.25];
        let peaked = vec![0.97, 0.01, 0.01, 0.01];
        assert!(sampler.score(&uniform) > sampler.score(&peaked));
    }
    #[test]
    fn test_uncertainty_sampler_select_query() {
        let sampler = UncertaintySampler::new(UncertaintyStrategy::Entropy);
        let candidates = vec![vec![0.99, 0.01], vec![0.5, 0.5], vec![0.7, 0.3]];
        let idx = sampler.select_query(&candidates);
        assert_eq!(idx, 1);
    }
    #[test]
    fn test_register_advanced_ml_axioms() {
        let mut env = Environment::new();
        register_advanced_ml_axioms(&mut env);
        assert!(!env.is_empty());
    }
}
