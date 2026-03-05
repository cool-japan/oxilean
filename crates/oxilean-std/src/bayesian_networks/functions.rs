//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BayesianNetwork, BeliefPropagation, DagGraph, DirichletCategorical, DiscreteCpd, Factor,
    GaussianGM, GibbsSampler, HamiltonianMC, Hmm, JunctionTree, KalmanFilter1D, MarkovBlanket,
    MeanFieldVI, MetropolisHastings, VariableElimination,
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
/// `BayesianNetwork : Type` — a directed acyclic graph with CPDs.
pub fn bayesian_network_ty() -> Expr {
    type0()
}
/// `DAG : Type` — directed acyclic graph (the structural skeleton).
pub fn dag_ty() -> Expr {
    type0()
}
/// `CPD : Type → Type` — conditional probability distribution P(Xi | Pa(Xi)).
pub fn cpd_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FactorGraph : Type` — bipartite graph of variable nodes and factor nodes.
pub fn factor_graph_ty() -> Expr {
    type0()
}
/// `Factor : Type → Type` — a non-negative function over a set of variables.
pub fn factor_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `MarkovBlanket : BayesianNetwork → Node → Set Node`
pub fn markov_blanket_ty() -> Expr {
    arrow(
        cst("BayesianNetwork"),
        arrow(cst("Node"), app(cst("Set"), cst("Node"))),
    )
}
/// `DSeparated : BayesianNetwork → Set Node → Set Node → Set Node → Prop`
pub fn d_separated_ty() -> Expr {
    arrow(
        cst("BayesianNetwork"),
        arrow(
            app(cst("Set"), cst("Node")),
            arrow(
                app(cst("Set"), cst("Node")),
                arrow(app(cst("Set"), cst("Node")), prop()),
            ),
        ),
    )
}
/// `HiddenMarkovModel : Type` — latent Markov chain with emission distribution.
pub fn hmm_ty() -> Expr {
    type0()
}
/// `KalmanFilter : Type` — linear-Gaussian state-space model.
pub fn kalman_filter_ty() -> Expr {
    type0()
}
/// `CRF : Type` — conditional random field (undirected discriminative model).
pub fn crf_ty() -> Expr {
    type0()
}
/// `BeliefPropagation : FactorGraph → Assignment → Prop`
pub fn belief_propagation_ty() -> Expr {
    arrow(cst("FactorGraph"), arrow(cst("Assignment"), prop()))
}
/// **Markov Blanket Theorem**: a node Xi is conditionally independent of all
/// non-descendants given its Markov blanket.
///
/// `markov_blanket_theorem : ∀ (bn : BayesianNetwork) (i : Node),
///   ConditionallyIndependent bn i (NonDescendants bn i) (MarkovBlanket bn i)`
pub fn markov_blanket_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bn",
        cst("BayesianNetwork"),
        pi(
            BinderInfo::Default,
            "i",
            cst("Node"),
            app3(
                cst("ConditionallyIndependent"),
                bvar(1),
                bvar(0),
                app2(cst("MarkovBlanket"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// **d-Separation Soundness**: d-separation implies conditional independence.
///
/// `d_sep_implies_ci : ∀ (bn : BayesianNetwork) (X Y Z : Set Node),
///   DSeparated bn X Y Z → ConditionallyIndependentSets bn X Y Z`
pub fn d_sep_implies_ci_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bn",
        cst("BayesianNetwork"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("Set"), cst("Node")),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("Set"), cst("Node")),
                pi(
                    BinderInfo::Default,
                    "Z",
                    app(cst("Set"), cst("Node")),
                    arrow(
                        app3(cst("DSeparated"), bvar(3), bvar(2), bvar(1)),
                        app3(
                            cst("ConditionallyIndependentSets"),
                            bvar(3),
                            bvar(2),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// **Variable Elimination Correctness**: VE computes exact marginals.
pub fn variable_elimination_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "fg",
        cst("FactorGraph"),
        pi(
            BinderInfo::Default,
            "q",
            cst("Query"),
            app2(
                cst("Eq"),
                app2(cst("VariableElimination"), bvar(1), bvar(0)),
                app2(cst("TrueMarginal"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// **Viterbi Optimality**: Viterbi finds the MAP sequence for an HMM.
pub fn viterbi_optimal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "hmm",
        cst("HiddenMarkovModel"),
        pi(
            BinderInfo::Default,
            "obs",
            list_ty(cst("Observation")),
            app2(
                cst("IsMapSequence"),
                app2(cst("Viterbi"), bvar(1), bvar(0)),
                bvar(0),
            ),
        ),
    )
}
/// **Kalman Filter Optimality**: the KF is the minimum mean-squared-error
/// linear estimator for a linear-Gaussian SSM.
pub fn kalman_optimal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kf",
        cst("KalmanFilter"),
        pi(
            BinderInfo::Default,
            "obs",
            list_ty(cst("Observation")),
            app2(cst("IsMMSE"), bvar(1), bvar(0)),
        ),
    )
}
/// **Global Markov Property**: for a Bayesian network, every d-separation
/// statement corresponds to a conditional independence in the distribution.
pub fn global_markov_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bn",
        cst("BayesianNetwork"),
        app2(cst("Satisfies"), bvar(0), cst("GlobalMarkovCondition")),
    )
}
/// **Junction Tree Exactness**: belief propagation on a tree (junction tree)
/// converges in one pass and yields exact marginals.
pub fn junction_tree_exact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "jt",
        cst("JunctionTree"),
        app(cst("BeliefPropagationExact"), bvar(0)),
    )
}
/// Populate an `Environment` with all Bayesian-network axiom declarations.
pub fn build_bayesian_networks_env(
    env: &mut Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("Node", type0()),
        ("Assignment", type0()),
        ("Query", type0()),
        ("Observation", type0()),
        ("JunctionTree", type0()),
        ("BayesianNetwork", bayesian_network_ty()),
        ("DAG", dag_ty()),
        ("FactorGraph", factor_graph_ty()),
        ("HiddenMarkovModel", hmm_ty()),
        ("KalmanFilter", kalman_filter_ty()),
        ("CRF", crf_ty()),
        (
            "ConditionallyIndependent",
            arrow(
                cst("BayesianNetwork"),
                arrow(cst("Node"), arrow(app(cst("Set"), cst("Node")), prop())),
            ),
        ),
        (
            "ConditionallyIndependentSets",
            arrow(
                cst("BayesianNetwork"),
                arrow(
                    app(cst("Set"), cst("Node")),
                    arrow(app(cst("Set"), cst("Node")), prop()),
                ),
            ),
        ),
        ("DSeparated", d_separated_ty()),
        ("MarkovBlanket", markov_blanket_ty()),
        (
            "NonDescendants",
            arrow(
                cst("BayesianNetwork"),
                arrow(cst("Node"), app(cst("Set"), cst("Node"))),
            ),
        ),
        (
            "VariableElimination",
            arrow(cst("FactorGraph"), arrow(cst("Query"), cst("Assignment"))),
        ),
        (
            "TrueMarginal",
            arrow(cst("FactorGraph"), arrow(cst("Query"), cst("Assignment"))),
        ),
        (
            "Viterbi",
            arrow(
                cst("HiddenMarkovModel"),
                arrow(list_ty(cst("Observation")), list_ty(cst("Node"))),
            ),
        ),
        (
            "IsMapSequence",
            arrow(
                list_ty(cst("Node")),
                arrow(list_ty(cst("Observation")), prop()),
            ),
        ),
        (
            "IsMMSE",
            arrow(
                cst("KalmanFilter"),
                arrow(list_ty(cst("Observation")), prop()),
            ),
        ),
        ("BeliefPropagationExact", arrow(cst("JunctionTree"), prop())),
        ("GlobalMarkovCondition", type0()),
        (
            "Satisfies",
            arrow(cst("BayesianNetwork"), arrow(type0(), prop())),
        ),
        ("markov_blanket_theorem", markov_blanket_theorem_ty()),
        ("d_sep_implies_ci", d_sep_implies_ci_ty()),
        (
            "variable_elimination_correct",
            variable_elimination_correct_ty(),
        ),
        ("viterbi_optimal", viterbi_optimal_ty()),
        ("kalman_optimal", kalman_optimal_ty()),
        ("global_markov_property", global_markov_property_ty()),
        ("junction_tree_exact", junction_tree_exact_ty()),
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
mod tests {
    use super::*;
    #[test]
    fn test_dag_add_edge_and_topological_order() {
        let mut dag = DagGraph::new(4);
        assert!(dag.add_edge(0, 1));
        assert!(dag.add_edge(0, 2));
        assert!(dag.add_edge(1, 3));
        assert!(dag.add_edge(2, 3));
        assert!(!dag.add_edge(3, 0));
        let order = dag.topological_order();
        assert_eq!(order.len(), 4);
        let pos: Vec<usize> = order
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<std::collections::HashMap<_, _>>()
            .values()
            .copied()
            .collect();
        let _ = pos;
        let pos_of: std::collections::HashMap<usize, usize> =
            order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos_of[&0] < pos_of[&1]);
        assert!(pos_of[&0] < pos_of[&2]);
        assert!(pos_of[&1] < pos_of[&3]);
    }
    #[test]
    fn test_markov_blanket() {
        let mut dag = DagGraph::new(3);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(0, 2);
        let mb = dag.markov_blanket(1);
        assert!(mb.contains(&0));
        assert!(mb.contains(&2));
    }
    #[test]
    fn test_d_separation_simple() {
        let mut dag = DagGraph::new(3);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        assert!(dag.d_separated(&[0], &[2], &[1]));
        assert!(!dag.d_separated(&[0], &[2], &[]));
    }
    #[test]
    fn test_hmm_forward_uniform() {
        let hmm = Hmm::new_uniform(2, 3);
        let prob = hmm.forward(&[0, 1, 2]);
        assert!(prob > 0.0, "forward probability should be positive");
    }
    #[test]
    fn test_hmm_viterbi_length() {
        let hmm = Hmm::new_uniform(3, 4);
        let obs = vec![0usize, 2, 1, 3];
        let path = hmm.viterbi(&obs);
        assert_eq!(path.len(), obs.len());
        for &s in &path {
            assert!(s < 3, "state index out of range");
        }
    }
    #[test]
    fn test_kalman_filter_constant_signal() {
        let mut kf = KalmanFilter1D::new(1.0, 1.0, 0.01, 1.0, 0.0, 1.0);
        let obs: Vec<f64> = vec![5.0; 20];
        let estimates = kf.filter(&obs);
        let last = *estimates.last().expect("last should succeed");
        assert!(
            (last - 5.0).abs() < 0.5,
            "KF should converge near 5.0, got {last}"
        );
    }
    #[test]
    fn test_mcmc_samples_distribution() {
        let mut mh = MetropolisHastings::new(vec![0.0], 0.5, 12345);
        let samples = mh.sample(2000, |x| -0.5 * (x[0] - 3.0).powi(2));
        let mean = samples.iter().map(|s| s[0]).sum::<f64>() / samples.len() as f64;
        assert!(
            (mean - 3.0).abs() < 0.5,
            "MCMC mean should be near 3.0, got {mean}"
        );
    }
    #[test]
    fn test_build_bayesian_networks_env() {
        let mut env = Environment::new();
        build_bayesian_networks_env(&mut env).expect("env build failed");
        assert!(env.get(&Name::str("BayesianNetwork")).is_some());
        assert!(env.get(&Name::str("HiddenMarkovModel")).is_some());
        assert!(env.get(&Name::str("KalmanFilter")).is_some());
    }
    #[test]
    fn test_factor_marginalize() {
        let f = Factor {
            scope: vec![0, 1],
            cards: vec![2, 2],
            values: vec![0.25, 0.25, 0.25, 0.25],
        };
        let marg = f.marginalize(1);
        assert_eq!(marg.scope, vec![0]);
        assert!((marg.values[0] - 0.5).abs() < 1e-10);
        assert!((marg.values[1] - 0.5).abs() < 1e-10);
    }
}
/// `DynamicBayesianNetworkTy : Type` — temporal BN with two-slice transition.
pub fn dbn_type_ty() -> Expr {
    type0()
}
/// `HmmForwardLikelihood : HiddenMarkovModel → List Observation → Real`
pub fn hmm_forward_likelihood_ty() -> Expr {
    arrow(
        cst("HiddenMarkovModel"),
        arrow(list_ty(cst("Observation")), real_ty()),
    )
}
/// **HMM Forward-Backward Correctness**: forward-backward computes exact posteriors.
pub fn hmm_fb_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "h",
        cst("HiddenMarkovModel"),
        pi(
            BinderInfo::Default,
            "obs",
            list_ty(cst("Observation")),
            app3(
                cst("IsExactPosterior"),
                app2(cst("ForwardBackward"), bvar(1), bvar(0)),
                bvar(1),
                bvar(0),
            ),
        ),
    )
}
/// **DBN Unrolling**: a DBN unrolled over T steps is a standard BN.
pub fn dbn_unroll_is_bn_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "d",
        cst("DynBayesNet"),
        pi(
            BinderInfo::Default,
            "T",
            nat_ty(),
            app(
                cst("IsBayesianNetwork"),
                app2(cst("Unroll"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// **Kalman Filter MMSE**: KF is optimal linear estimator for linear-Gaussian SSM.
pub fn kalman_filter_mmse_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kf",
        cst("KalmanFilter"),
        pi(
            BinderInfo::Default,
            "obs",
            list_ty(real_ty()),
            app2(cst("IsMMSELinear"), bvar(1), bvar(0)),
        ),
    )
}
/// `StructuralCausalModelTy : Type` — SCM with noise variables and structural equations.
pub fn scm_type_ty() -> Expr {
    type0()
}
/// **Do-calculus Rule 1**: insertion/deletion of observations.
pub fn do_calc_rule1_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("SCM"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("Set"), cst("Node")),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("Set"), cst("Node")),
                pi(
                    BinderInfo::Default,
                    "Z",
                    app(cst("Set"), cst("Node")),
                    arrow(
                        app(cst("Set"), cst("Node")),
                        app3(cst("DoCalcRule1"), bvar(3), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// **Do-calculus Rule 2**: action/observation exchange.
pub fn do_calc_rule2_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("SCM"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("Set"), cst("Node")),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("Set"), cst("Node")),
                pi(
                    BinderInfo::Default,
                    "Z",
                    app(cst("Set"), cst("Node")),
                    app3(cst("DoCalcRule2"), bvar(3), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// **Counterfactual Consistency**: the counterfactual under do(X=x) equals the
/// factual when X was actually x.
pub fn counterfactual_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("SCM"),
        pi(
            BinderInfo::Default,
            "x",
            real_ty(),
            app2(cst("CounterfactualConsistency"), bvar(1), bvar(0)),
        ),
    )
}
/// **Backdoor Adjustment**: P(Y | do(X)) = Σ_Z P(Y|X,Z) P(Z) for valid backdoor set Z.
pub fn backdoor_adjustment_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("SCM"),
        pi(
            BinderInfo::Default,
            "X",
            cst("Node"),
            pi(
                BinderInfo::Default,
                "Y",
                cst("Node"),
                pi(
                    BinderInfo::Default,
                    "Z",
                    app(cst("Set"), cst("Node")),
                    arrow(
                        app3(cst("IsBackdoorSet"), bvar(3), bvar(2), bvar(1)),
                        app3(cst("BackdoorAdjustmentHolds"), bvar(3), bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `PDAG : Type` — partially directed acyclic graph (output of PC algorithm).
pub fn pdag_type_ty() -> Expr {
    type0()
}
/// **PC Algorithm Consistency**: in the large-sample limit, PC returns the true PDAG.
pub fn pc_consistent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bn",
        cst("BayesianNetwork"),
        app2(cst("IsMEC"), app(cst("PCAlgorithm"), bvar(0)), bvar(0)),
    )
}
/// **FCI Completeness**: FCI correctly identifies PAG in the presence of latent variables.
pub fn fci_complete_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("SCM"),
        app(cst("FCIReturnsCorrectPAG"), bvar(0)),
    )
}
/// **BIC Score Consistency**: BIC-based score selects the true model asymptotically.
pub fn bic_score_consistent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bn",
        cst("BayesianNetwork"),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(cst("BICScoreConsistent"), bvar(1), bvar(0)),
        ),
    )
}
/// `LinearGaussianBN : Type` — BN where each node is Gaussian linear in parents.
pub fn linear_gaussian_bn_type_ty() -> Expr {
    type0()
}
/// `CLGModel : Type` — conditional linear-Gaussian model.
pub fn clg_model_type_ty() -> Expr {
    type0()
}
/// **Linear Gaussian BN marginals are Gaussian**.
pub fn lgbn_marginals_gaussian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "lg",
        cst("LinearGaussianBN"),
        pi(
            BinderInfo::Default,
            "i",
            cst("Node"),
            app(
                cst("IsGaussian"),
                app2(cst("LGBNMarginal"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `DirichletProcess : Type` — DP with concentration parameter and base measure.
pub fn dirichlet_process_type_ty() -> Expr {
    type0()
}
/// `IndianBuffetProcess : Type` — sparse binary matrix process.
pub fn indian_buffet_process_type_ty() -> Expr {
    type0()
}
/// **DP Stick-Breaking**: a DP draw admits the stick-breaking construction.
pub fn dp_stick_breaking_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "dp",
        cst("DirProcess"),
        app(cst("HasStickBreakingRepresentation"), bvar(0)),
    )
}
/// **IBP Exchangeability**: IBP is exchangeable (de Finetti-type result).
pub fn ibp_exchangeability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ibp",
        cst("IndBufProcess"),
        app(cst("IsExchangeable"), bvar(0)),
    )
}
/// `VariationalDistribution : Type` — approximate posterior family Q.
pub fn variational_distribution_type_ty() -> Expr {
    type0()
}
/// **ELBO Lower Bound**: ELBO ≤ log p(x) with equality iff Q = P(·|x).
pub fn elbo_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "q",
        cst("VarDist"),
        pi(
            BinderInfo::Default,
            "p",
            cst("BayesianNetwork"),
            pi(
                BinderInfo::Default,
                "x",
                cst("Observation"),
                app(
                    cst("ELBOLeqLogEvidence"),
                    app3(cst("ELBO"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Mean-Field Factorization**: mean-field VI assumes Q(z) = ∏_i Q_i(z_i).
pub fn mean_field_factorization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "q",
        cst("VarDist"),
        arrow(
            app(cst("IsMeanField"), bvar(0)),
            app(cst("IsFactorized"), bvar(0)),
        ),
    )
}
/// **Variational EM convergence**: variational EM increases the ELBO at each step.
pub fn variational_em_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "q",
        cst("VarDist"),
        pi(
            BinderInfo::Default,
            "t",
            nat_ty(),
            app2(cst("ELBONonDecreasing"), bvar(1), bvar(0)),
        ),
    )
}
/// **Sum-Product correctness on trees**: exact marginals on tree factor graphs.
pub fn sum_product_tree_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "fg",
        cst("FactorGraph"),
        arrow(
            app(cst("IsTree"), bvar(0)),
            app(cst("SumProductExact"), bvar(0)),
        ),
    )
}
/// **Max-Product finds MAP on trees**.
pub fn max_product_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "fg",
        cst("FactorGraph"),
        arrow(
            app(cst("IsTree"), bvar(0)),
            app(cst("MaxProductFindsMAP"), bvar(0)),
        ),
    )
}
/// **Loopy BP convergence implies fixed point**.
pub fn loopy_bp_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "fg",
        cst("FactorGraph"),
        pi(
            BinderInfo::Default,
            "msgs",
            cst("BPMessages"),
            arrow(
                app2(cst("LoopyBPConverges"), bvar(1), bvar(0)),
                app2(cst("IsFixedPoint"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `EPApproximation : Type` — EP approximation to an intractable posterior.
pub fn ep_approximation_type_ty() -> Expr {
    type0()
}
/// **EP fixed-point condition**: at convergence, EP satisfies moment matching.
pub fn ep_moment_matching_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ep",
        cst("EPApprox"),
        arrow(
            app(cst("EPConverged"), bvar(0)),
            app(cst("MomentsMatch"), bvar(0)),
        ),
    )
}
/// **ADF consistency**: assumed density filtering gives consistent posterior approximation.
pub fn adf_consistent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "model",
        cst("LinearGaussianBN"),
        app(cst("ADFConsistent"), bvar(0)),
    )
}
/// **Gibbs sampling ergodicity**: the Gibbs chain is ergodic for positive distributions.
pub fn gibbs_ergodic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        cst("BayesianNetwork"),
        arrow(
            app(cst("IsPositive"), bvar(0)),
            app(cst("GibbsChainErgodic"), bvar(0)),
        ),
    )
}
/// **HMC detailed balance**: Hamiltonian Monte Carlo satisfies detailed balance.
pub fn hmc_detailed_balance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "target",
        cst("BayesianNetwork"),
        app(cst("HMCDetailedBalance"), bvar(0)),
    )
}
/// **Reversible chain convergence**: ergodic reversible chain converges to stationary dist.
pub fn reversible_chain_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "chain",
        cst("MarkovChain"),
        arrow(
            app(cst("IsReversible"), bvar(0)),
            app(cst("ConvergesToStationary"), bvar(0)),
        ),
    )
}
/// `BayesianNeuralNet : Type` — neural network with a prior over weights.
pub fn bayesian_neural_net_type_ty() -> Expr {
    type0()
}
/// **Weight prior predictive**: integrating over the weight prior gives predictive dist.
pub fn bnn_predictive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bnn",
        cst("BayesNeuralNet"),
        pi(
            BinderInfo::Default,
            "x",
            cst("Observation"),
            app2(cst("IsPredictive"), bvar(1), bvar(0)),
        ),
    )
}
/// **Laplace approximation**: the Laplace approx to the posterior is Gaussian at MAP.
pub fn laplace_approximation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "bnn",
        cst("BayesNeuralNet"),
        pi(
            BinderInfo::Default,
            "data",
            list_ty(cst("Observation")),
            app2(cst("LaplaceApproxValid"), bvar(1), bvar(0)),
        ),
    )
}
/// `GaussianGraphicalModel : Type` — GGM with precision matrix Λ.
pub fn ggm_type_ty() -> Expr {
    type0()
}
/// **GGM precision sparsity**: Λ_{ij} = 0 iff Xi ⊥ Xj | rest.
pub fn ggm_precision_sparsity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ggm",
        cst("GGModel"),
        pi(
            BinderInfo::Default,
            "i",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "j",
                nat_ty(),
                app2(
                    cst("Iff"),
                    app3(cst("PrecisionZero"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("GGMConditionallyIndep"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Cholesky factor positivity**: GGM has pos-def precision iff Cholesky diag is positive.
pub fn cholesky_positive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ggm",
        cst("GGModel"),
        app2(
            cst("Iff"),
            app(cst("PrecisionPosDef"), bvar(0)),
            app(cst("CholeskyDiagPositive"), bvar(0)),
        ),
    )
}
/// `LDPCCode : Type` — low-density parity-check code.
pub fn ldpc_code_type_ty() -> Expr {
    type0()
}
/// **LDPC belief propagation decoding**: BP on the Tanner graph decodes LDPC codes.
pub fn ldpc_bp_decoding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "code",
        cst("LDPCCode"),
        pi(
            BinderInfo::Default,
            "received",
            list_ty(real_ty()),
            app2(cst("BPDecodesLDPC"), bvar(1), bvar(0)),
        ),
    )
}
/// `CTBN : Type` — continuous-time Bayesian network with intensity matrices.
pub fn ctbn_type_ty() -> Expr {
    type0()
}
/// **CTBN Markov property**: a CTBN defines a Markov process.
pub fn ctbn_markov_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ctbn",
        cst("CTBNType"),
        app(cst("IsMarkovProcess"), bvar(0)),
    )
}
/// **CTBN likelihood**: the likelihood of a trajectory under a CTBN is well-defined.
pub fn ctbn_likelihood_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ctbn",
        cst("CTBNType"),
        pi(
            BinderInfo::Default,
            "traj",
            cst("Trajectory"),
            app2(cst("CTBNLikelihoodDefined"), bvar(1), bvar(0)),
        ),
    )
}
/// `CredalSet : Type` — set of probability distributions (credal set).
pub fn credal_set_type_ty() -> Expr {
    type0()
}
/// **Credal set convexity**: every credal set is convex.
pub fn credal_set_convex_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "cs",
        cst("CredalSet"),
        app(cst("IsConvex"), bvar(0)),
    )
}
/// **Imprecise probability lower prevision**: the lower prevision is superlinear.
pub fn lower_prevision_superlinear_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "cs",
        cst("CredalSet"),
        app(cst("LowerPrevisionSuperlinear"), bvar(0)),
    )
}
/// `UtilityFunction : Type` — mapping from outcomes to real-valued utilities.
pub fn utility_function_type_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// **Expected utility maximization**: the Bayesian optimal decision maximises EU.
pub fn eu_maximization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        cst("BayesianNetwork"),
        pi(
            BinderInfo::Default,
            "u",
            cst("UtilFunc"),
            pi(
                BinderInfo::Default,
                "d",
                cst("Decision"),
                arrow(
                    app3(cst("IsOptimalDecision"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("MaximisesExpectedUtility"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// **Savage's sure-thing principle**: coherence axiom for Bayesian decisions.
pub fn sure_thing_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "u",
        cst("UtilFunc"),
        app(cst("SatisfiesSureThing"), bvar(0)),
    )
}
/// Populate an `Environment` with all extended Bayesian-network axiom declarations.
pub fn build_bayesian_networks_ext_env(
    env: &mut Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("DynBayesNet", dbn_type_ty()),
        ("SCM", scm_type_ty()),
        ("PDAGType", pdag_type_ty()),
        ("LinearGaussianBN", linear_gaussian_bn_type_ty()),
        ("CLGModel", clg_model_type_ty()),
        ("DirProcess", dirichlet_process_type_ty()),
        ("IndBufProcess", indian_buffet_process_type_ty()),
        ("VarDist", variational_distribution_type_ty()),
        ("EPApprox", ep_approximation_type_ty()),
        ("BayesNeuralNet", bayesian_neural_net_type_ty()),
        ("GGModel", ggm_type_ty()),
        ("LDPCCode", ldpc_code_type_ty()),
        ("CTBNType", ctbn_type_ty()),
        ("CredalSet", credal_set_type_ty()),
        ("BPMessages", type0()),
        ("MarkovChain", type0()),
        ("Trajectory", type0()),
        ("Decision", type0()),
        ("UtilFunc", utility_function_type_ty()),
        (
            "ForwardBackward",
            arrow(
                cst("HiddenMarkovModel"),
                arrow(list_ty(cst("Observation")), type0()),
            ),
        ),
        (
            "IsExactPosterior",
            arrow(
                type0(),
                arrow(
                    cst("HiddenMarkovModel"),
                    arrow(list_ty(cst("Observation")), prop()),
                ),
            ),
        ),
        (
            "Unroll",
            arrow(cst("DynBayesNet"), arrow(nat_ty(), cst("BayesianNetwork"))),
        ),
        ("IsBayesianNetwork", arrow(type0(), prop())),
        (
            "IsMMSELinear",
            arrow(cst("KalmanFilter"), arrow(list_ty(real_ty()), prop())),
        ),
        (
            "IsBackdoorSet",
            arrow(
                cst("SCM"),
                arrow(cst("Node"), arrow(app(cst("Set"), cst("Node")), prop())),
            ),
        ),
        (
            "BackdoorAdjustmentHolds",
            arrow(cst("SCM"), arrow(cst("Node"), arrow(cst("Node"), prop()))),
        ),
        (
            "DoCalcRule1",
            arrow(
                cst("SCM"),
                arrow(
                    app(cst("Set"), cst("Node")),
                    arrow(app(cst("Set"), cst("Node")), prop()),
                ),
            ),
        ),
        (
            "DoCalcRule2",
            arrow(
                cst("SCM"),
                arrow(
                    app(cst("Set"), cst("Node")),
                    arrow(app(cst("Set"), cst("Node")), prop()),
                ),
            ),
        ),
        (
            "CounterfactualConsistency",
            arrow(cst("SCM"), arrow(real_ty(), prop())),
        ),
        (
            "PCAlgorithm",
            arrow(cst("BayesianNetwork"), cst("PDAGType")),
        ),
        (
            "IsMEC",
            arrow(cst("PDAGType"), arrow(cst("BayesianNetwork"), prop())),
        ),
        ("FCIReturnsCorrectPAG", arrow(cst("SCM"), prop())),
        (
            "BICScoreConsistent",
            arrow(cst("BayesianNetwork"), arrow(nat_ty(), prop())),
        ),
        (
            "LGBNMarginal",
            arrow(cst("LinearGaussianBN"), arrow(cst("Node"), type0())),
        ),
        ("IsGaussian", arrow(type0(), prop())),
        (
            "HasStickBreakingRepresentation",
            arrow(cst("DirProcess"), prop()),
        ),
        ("IsExchangeable", arrow(cst("IndBufProcess"), prop())),
        (
            "ELBO",
            arrow(
                cst("VarDist"),
                arrow(cst("BayesianNetwork"), arrow(cst("Observation"), real_ty())),
            ),
        ),
        ("ELBOLeqLogEvidence", arrow(real_ty(), prop())),
        ("IsMeanField", arrow(cst("VarDist"), prop())),
        ("IsFactorized", arrow(cst("VarDist"), prop())),
        (
            "ELBONonDecreasing",
            arrow(cst("VarDist"), arrow(nat_ty(), prop())),
        ),
        ("IsTree", arrow(cst("FactorGraph"), prop())),
        ("SumProductExact", arrow(cst("FactorGraph"), prop())),
        ("MaxProductFindsMAP", arrow(cst("FactorGraph"), prop())),
        (
            "LoopyBPConverges",
            arrow(cst("FactorGraph"), arrow(cst("BPMessages"), prop())),
        ),
        (
            "IsFixedPoint",
            arrow(cst("FactorGraph"), arrow(cst("BPMessages"), prop())),
        ),
        ("EPConverged", arrow(cst("EPApprox"), prop())),
        ("MomentsMatch", arrow(cst("EPApprox"), prop())),
        ("ADFConsistent", arrow(cst("LinearGaussianBN"), prop())),
        ("IsPositive", arrow(cst("BayesianNetwork"), prop())),
        ("GibbsChainErgodic", arrow(cst("BayesianNetwork"), prop())),
        ("HMCDetailedBalance", arrow(cst("BayesianNetwork"), prop())),
        ("IsReversible", arrow(cst("MarkovChain"), prop())),
        ("ConvergesToStationary", arrow(cst("MarkovChain"), prop())),
        (
            "IsPredictive",
            arrow(cst("BayesNeuralNet"), arrow(cst("Observation"), prop())),
        ),
        (
            "LaplaceApproxValid",
            arrow(
                cst("BayesNeuralNet"),
                arrow(list_ty(cst("Observation")), prop()),
            ),
        ),
        (
            "PrecisionZero",
            arrow(cst("GGModel"), arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ),
        (
            "GGMConditionallyIndep",
            arrow(cst("GGModel"), arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ),
        ("PrecisionPosDef", arrow(cst("GGModel"), prop())),
        ("CholeskyDiagPositive", arrow(cst("GGModel"), prop())),
        (
            "BPDecodesLDPC",
            arrow(cst("LDPCCode"), arrow(list_ty(real_ty()), prop())),
        ),
        ("IsMarkovProcess", arrow(cst("CTBNType"), prop())),
        (
            "CTBNLikelihoodDefined",
            arrow(cst("CTBNType"), arrow(cst("Trajectory"), prop())),
        ),
        ("IsConvex", arrow(cst("CredalSet"), prop())),
        ("LowerPrevisionSuperlinear", arrow(cst("CredalSet"), prop())),
        (
            "IsOptimalDecision",
            arrow(
                cst("BayesianNetwork"),
                arrow(cst("UtilFunc"), arrow(cst("Decision"), prop())),
            ),
        ),
        (
            "MaximisesExpectedUtility",
            arrow(
                cst("BayesianNetwork"),
                arrow(cst("UtilFunc"), arrow(cst("Decision"), prop())),
            ),
        ),
        ("SatisfiesSureThing", arrow(cst("UtilFunc"), prop())),
        ("hmm_fb_correct", hmm_fb_correct_ty()),
        ("dbn_unroll_is_bn", dbn_unroll_is_bn_ty()),
        ("kalman_filter_mmse", kalman_filter_mmse_ty()),
        ("do_calc_rule1", do_calc_rule1_ty()),
        ("do_calc_rule2", do_calc_rule2_ty()),
        (
            "counterfactual_consistency",
            counterfactual_consistency_ty(),
        ),
        ("backdoor_adjustment", backdoor_adjustment_ty()),
        ("pc_consistent", pc_consistent_ty()),
        ("fci_complete", fci_complete_ty()),
        ("bic_score_consistent", bic_score_consistent_ty()),
        ("lgbn_marginals_gaussian", lgbn_marginals_gaussian_ty()),
        ("dp_stick_breaking", dp_stick_breaking_ty()),
        ("ibp_exchangeability", ibp_exchangeability_ty()),
        ("elbo_lower_bound", elbo_lower_bound_ty()),
        ("mean_field_factorization", mean_field_factorization_ty()),
        (
            "variational_em_convergence",
            variational_em_convergence_ty(),
        ),
        ("sum_product_tree_correct", sum_product_tree_correct_ty()),
        ("max_product_map", max_product_map_ty()),
        ("loopy_bp_fixed_point", loopy_bp_fixed_point_ty()),
        ("ep_moment_matching", ep_moment_matching_ty()),
        ("adf_consistent", adf_consistent_ty()),
        ("gibbs_ergodic", gibbs_ergodic_ty()),
        ("hmc_detailed_balance", hmc_detailed_balance_ty()),
        (
            "reversible_chain_convergence",
            reversible_chain_convergence_ty(),
        ),
        ("bnn_predictive", bnn_predictive_ty()),
        ("laplace_approximation", laplace_approximation_ty()),
        ("ggm_precision_sparsity", ggm_precision_sparsity_ty()),
        ("cholesky_positive", cholesky_positive_ty()),
        ("ldpc_bp_decoding", ldpc_bp_decoding_ty()),
        ("ctbn_markov", ctbn_markov_ty()),
        ("ctbn_likelihood", ctbn_likelihood_ty()),
        ("credal_set_convex", credal_set_convex_ty()),
        (
            "lower_prevision_superlinear",
            lower_prevision_superlinear_ty(),
        ),
        ("eu_maximization", eu_maximization_ty()),
        ("sure_thing_principle", sure_thing_principle_ty()),
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
mod ext_tests {
    use super::*;
    #[test]
    fn test_gibbs_sampler_binary_chain() {
        let cpd0 = DiscreteCpd::uniform(2, vec![]);
        let cpd1 = DiscreteCpd::uniform(2, vec![2]);
        let cpd2 = DiscreteCpd::uniform(2, vec![2]);
        let cpds = vec![cpd0, cpd1, cpd2];
        let parents = vec![vec![], vec![0], vec![1]];
        let mut sampler = GibbsSampler::new(cpds, parents, 42);
        let samples = sampler.draw(100);
        assert_eq!(samples.len(), 100);
        for s in &samples {
            assert_eq!(s.len(), 3);
            assert!(s[0] < 2 && s[1] < 2 && s[2] < 2);
        }
    }
    #[test]
    fn test_dirichlet_categorical_predictive() {
        let mut dc = DirichletCategorical::new_symmetric(3, 1.0);
        for _ in 0..100 {
            dc.observe(0);
        }
        let p0 = dc.predictive(0);
        let p1 = dc.predictive(1);
        assert!(p0 > p1, "p(0) should dominate after many observations");
        let total: f64 = (0..3).map(|k| dc.predictive(k)).sum();
        assert!((total - 1.0).abs() < 1e-10, "predictive should sum to 1");
    }
    #[test]
    fn test_mean_field_vi_elbo_finite() {
        let mut vi = MeanFieldVI::new(1);
        vi.means = vec![3.0];
        let final_elbo = vi.run(|z| -0.5 * z[0] * z[0], 0.1, 50);
        assert!(
            final_elbo.is_finite(),
            "ELBO should be finite; got {final_elbo}"
        );
        assert!(final_elbo > -1e6, "ELBO too small: {final_elbo}");
    }
    #[test]
    fn test_gaussian_gm_cov_inverse() {
        let ggm = GaussianGM::new(2, vec![1.0, 0.0, 0.0, 1.0]);
        let cov = ggm.covariance().expect("should invert");
        assert!((cov[0] - 1.0).abs() < 1e-10);
        assert!(cov[1].abs() < 1e-10);
        assert!(cov[2].abs() < 1e-10);
        assert!((cov[3] - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_gaussian_gm_conditional_independence() {
        let ggm = GaussianGM::new(3, vec![2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 1.0]);
        assert!(ggm.conditionally_independent(0, 1, 1e-10));
        assert!(ggm.conditionally_independent(0, 2, 1e-10));
        assert!(!ggm.conditionally_independent(0, 0, 1e-10));
    }
    #[test]
    fn test_hmc_samples_near_mode() {
        let mut hmc = HamiltonianMC::new(vec![0.0], 0.1, 5, 99999);
        let samples = hmc.sample(500, |x| -0.5 * (x[0] - 2.0).powi(2));
        let mean = samples.iter().map(|s| s[0]).sum::<f64>() / samples.len() as f64;
        assert!(
            (mean - 2.0).abs() < 1.0,
            "HMC mean should be near 2.0, got {mean}"
        );
    }
    #[test]
    fn test_build_bayesian_networks_ext_env() {
        let mut env = Environment::new();
        build_bayesian_networks_ext_env(&mut env).expect("ext env build failed");
        assert!(env.get(&Name::str("SCM")).is_some());
        assert!(env.get(&Name::str("DirProcess")).is_some());
        assert!(env.get(&Name::str("CTBNType")).is_some());
        assert!(env.get(&Name::str("GGModel")).is_some());
        assert!(env.get(&Name::str("eu_maximization")).is_some());
    }
}
