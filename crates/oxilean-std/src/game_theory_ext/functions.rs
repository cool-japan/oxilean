//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AuctionGame, CooperativeGame, CorrelatedEquilibrium, CorrelatedEquilibriumSolver, ESSChecker,
    EvolutionaryGame, ExhaustiveSearch, GaleShapleyAlgorithm, MechanismDesignChecker,
    ReplicatorDynamics, ReplicatorDynamicsEvo, StackelbergGame, StackelbergGameExt,
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
pub fn subgame_perfect_eq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("ExtensiveFormGame"), bvar(0)), prop()),
    )
}
pub fn perfect_bayesian_eq_ty() -> Expr {
    prop()
}
pub fn folk_theorem_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn discount_factor_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn stochastic_game_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn stochastic_shapley_value_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("StochasticGame"), bvar(0)), list_ty(real_ty())),
    )
}
pub fn ess_predicate_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("EvolutionaryGame"), prop()))
}
pub fn replicator_ode_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), list_ty(real_ty())))
}
pub fn mean_field_game_solution_ty() -> Expr {
    prop()
}
pub fn isaacs_equation_ty() -> Expr {
    prop()
}
pub fn dominant_strategy_ic_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn individual_rationality_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn vcg_mechanism_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn vcg_truthful_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("VCGMechanism"), bvar(0)), prop()),
    )
}
pub fn myerson_optimal_mechanism_ty() -> Expr {
    prop()
}
pub fn stable_matching_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop())),
    )
}
pub fn deferred_acceptance_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn all_pay_auction_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn condorcet_winner_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), prop()))
}
pub fn arrow_impossibility_ty() -> Expr {
    prop()
}
pub fn sen_impossibility_ty() -> Expr {
    prop()
}
pub fn core_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("CooperativeGame"), bvar(0)),
            arrow(list_ty(real_ty()), prop()),
        ),
    )
}
pub fn nucleolus_unique_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CooperativeGame"), bvar(0)), prop()),
    )
}
pub fn nash_bargaining_solution_ty() -> Expr {
    prop()
}
pub fn correlated_equilibrium_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn rationalizability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn best_response_convergence_ty() -> Expr {
    prop()
}
pub fn fictitious_play_convergence_ty() -> Expr {
    prop()
}
pub fn borda_count_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), nat_ty()))
}
pub fn gibbard_satterthwaite_ty() -> Expr {
    prop()
}
pub fn superadditive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CooperativeGame"), bvar(0)), prop()),
    )
}
pub fn convex_game_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("CooperativeGame"), bvar(0)), prop()),
    )
}
pub fn price_of_anarchy_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn price_of_stability_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn potential_game_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn quantal_response_eq_ty() -> Expr {
    arrow(
        real_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
        ),
    )
}
pub fn level_k_thinking_ty() -> Expr {
    arrow(
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
        ),
    )
}
pub fn global_game_ty() -> Expr {
    prop()
}
pub fn supermodular_game_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn congestion_game_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn network_formation_game_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn social_network_eq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NetworkFormationGame"), bvar(0)), prop()),
    )
}
pub fn folk_theorem_nash_threats_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn cheap_talk_eq_ty() -> Expr {
    prop()
}
pub fn separating_eq_ty() -> Expr {
    prop()
}
pub fn pooling_eq_ty() -> Expr {
    prop()
}
pub fn discount_factor_admissible_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn grim_trigger_strategy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn tit_for_tat_strategy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
pub fn trembling_hand_perfect_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn sequential_equilibrium_ty() -> Expr {
    prop()
}
pub fn proper_equilibrium_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn coalition_proof_nash_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn strong_nash_equilibrium_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
    )
}
pub fn epsilon_nash_ty() -> Expr {
    arrow(
        real_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(app(cst("NormalFormGame"), bvar(0)), prop()),
        ),
    )
}
pub fn nash_demand_game_ty() -> Expr {
    prop()
}
pub fn rubinstein_bargaining_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn banach_mazur_game_ty() -> Expr {
    prop()
}
pub fn colonel_blotto_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn war_of_attrition_eq_ty() -> Expr {
    prop()
}
pub fn tullock_contest_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn evolutionary_mutation_stability_ty() -> Expr {
    arrow(real_ty(), prop())
}
pub fn replicator_mutator_dynamics_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), list_ty(real_ty())))
}
pub fn hotelling_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn bertrand_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
pub fn cournot_eq_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Register all extended game-theory axioms in `env`.
pub fn build_game_theory_ext_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("SubgamePerfectEq", subgame_perfect_eq_ty()),
        ("PerfectBayesianEq", perfect_bayesian_eq_ty()),
        ("FolkTheorem", folk_theorem_ty()),
        ("DiscountFactor", discount_factor_ty()),
        ("StochasticGame", stochastic_game_ty()),
        ("StochasticShapleyValue", stochastic_shapley_value_ty()),
        ("ESSPredicate", ess_predicate_ty()),
        ("ReplicatorODE", replicator_ode_ty()),
        ("MeanFieldGameSolution", mean_field_game_solution_ty()),
        ("IsaacsEquation", isaacs_equation_ty()),
        ("DominantStrategyIC", dominant_strategy_ic_ty()),
        ("IndividualRationality", individual_rationality_ty()),
        ("VCGMechanism", vcg_mechanism_ty()),
        ("VCGTruthful", vcg_truthful_ty()),
        ("MyersonOptimalMechanism", myerson_optimal_mechanism_ty()),
        ("StableMatching", stable_matching_ty()),
        ("DeferredAcceptance", deferred_acceptance_ty()),
        ("AllPayAuctionEq", all_pay_auction_eq_ty()),
        ("CondorcetWinner", condorcet_winner_ty()),
        ("ArrowImpossibility", arrow_impossibility_ty()),
        ("SenImpossibility", sen_impossibility_ty()),
        ("Core", core_ty()),
        ("NucleolusUnique", nucleolus_unique_ty()),
        ("NashBargainingSolution", nash_bargaining_solution_ty()),
        ("CorrelatedEquilibrium", correlated_equilibrium_ty()),
        ("Rationalizability", rationalizability_ty()),
        ("BestResponseConvergence", best_response_convergence_ty()),
        (
            "FictitiousPlayConvergence",
            fictitious_play_convergence_ty(),
        ),
        ("BordaCount", borda_count_ty()),
        ("GibbardSatterthwaite", gibbard_satterthwaite_ty()),
        ("Superadditive", superadditive_ty()),
        ("ConvexGame", convex_game_ty()),
        ("PriceOfAnarchy", price_of_anarchy_ty()),
        ("PriceOfStability", price_of_stability_ty()),
        ("PotentialGame", potential_game_ty()),
        ("QuantalResponseEq", quantal_response_eq_ty()),
        ("LevelKThinking", level_k_thinking_ty()),
        ("GlobalGame", global_game_ty()),
        ("SupermodularGame", supermodular_game_ty()),
        ("CongestionGame", congestion_game_ty()),
        ("NetworkFormationGame", network_formation_game_ty()),
        ("SocialNetworkEq", social_network_eq_ty()),
        ("FolkTheoremNashThreats", folk_theorem_nash_threats_ty()),
        ("CheapTalkEq", cheap_talk_eq_ty()),
        ("SeparatingEq", separating_eq_ty()),
        ("PoolingEq", pooling_eq_ty()),
        ("DiscountFactorAdmissible", discount_factor_admissible_ty()),
        ("GrimTriggerStrategy", grim_trigger_strategy_ty()),
        ("TitForTatStrategy", tit_for_tat_strategy_ty()),
        ("TremblingHandPerfect", trembling_hand_perfect_ty()),
        ("SequentialEquilibrium", sequential_equilibrium_ty()),
        ("ProperEquilibrium", proper_equilibrium_ty()),
        ("CoalitionProofNash", coalition_proof_nash_ty()),
        ("StrongNashEquilibrium", strong_nash_equilibrium_ty()),
        ("EpsilonNash", epsilon_nash_ty()),
        ("NashDemandGame", nash_demand_game_ty()),
        ("RubinsteinBargaining", rubinstein_bargaining_ty()),
        ("BanachMazurGame", banach_mazur_game_ty()),
        ("ColonelBlottoEq", colonel_blotto_eq_ty()),
        ("WarOfAttritionEq", war_of_attrition_eq_ty()),
        ("TullockContestEq", tullock_contest_eq_ty()),
        (
            "EvolutionaryMutationStability",
            evolutionary_mutation_stability_ty(),
        ),
        (
            "ReplicatorMutatorDynamics",
            replicator_mutator_dynamics_ty(),
        ),
        ("HotellingEq", hotelling_eq_ty()),
        ("BertrandEq", bertrand_eq_ty()),
        ("CournotEq", cournot_eq_ty()),
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
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ess_prisoners_dilemma() {
        let matrix = vec![vec![3.0, 0.0], vec![5.0, 1.0]];
        let strategies = vec!["Cooperate".to_string(), "Defect".to_string()];
        let game = EvolutionaryGame::new(matrix, strategies);
        let ess = game.evolutionarily_stable_strategy();
        assert!(ess.contains(&1));
        assert!(!ess.contains(&0));
    }
    #[test]
    fn test_nash_evolutionary() {
        let matrix = vec![vec![3.0, 0.0], vec![5.0, 1.0]];
        let strategies = vec!["Cooperate".to_string(), "Defect".to_string()];
        let game = EvolutionaryGame::new(matrix, strategies);
        let nash = game.nash_evolutionary();
        assert!(nash.contains(&1));
    }
    #[test]
    fn test_replicator_dynamics_stable() {
        let matrix = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let strategies = vec!["A".to_string(), "B".to_string()];
        let game = EvolutionaryGame::new(matrix, strategies);
        let next = game.replicator_dynamics(0.01);
        assert!((next[0] - 0.5).abs() < 1e-6);
        assert!((next[1] - 0.5).abs() < 1e-6);
    }
    #[test]
    fn test_shapley_value_symmetric() {
        let chars = vec![0.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 3.0];
        let players = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let game = CooperativeGame::new(players, chars);
        let phi = game.shapley_value();
        for v in &phi {
            assert!((v - 1.0).abs() < 1e-9);
        }
    }
    #[test]
    fn test_core_nonempty() {
        let chars = vec![0.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 3.0];
        let players = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let game = CooperativeGame::new(players, chars);
        assert!(game.core_is_nonempty());
    }
    #[test]
    fn test_minimax() {
        let payoffs = vec![vec![3.0, -1.0], vec![2.0, 1.0]];
        let search = ExhaustiveSearch::new(2, payoffs);
        assert!((search.minimax() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_solve_zero_sum_saddle_point() {
        let payoffs = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
        let search = ExhaustiveSearch::new(2, payoffs);
        let (i, j, v) = search.solve_zero_sum();
        assert_eq!(i, 1);
        assert_eq!(j, 0);
        assert!((v - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_alpha_beta() {
        let payoffs = vec![vec![3.0, -1.0], vec![2.0, 1.0]];
        let search = ExhaustiveSearch::new(2, payoffs);
        let v = search.alpha_beta(2);
        assert!((v - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_stackelberg_equilibrium() {
        let leader_payoffs = vec![vec![4.0, 1.0], vec![2.0, 3.0]];
        let follower_payoffs = vec![vec![1.0, 4.0], vec![3.0, 2.0]];
        let game = StackelbergGame::new(
            vec!["L0".to_string(), "L1".to_string()],
            vec!["F0".to_string(), "F1".to_string()],
            leader_payoffs,
            follower_payoffs,
        );
        let (li, fj, _payoff) = game.stackelberg_equilibrium();
        assert_eq!(li, 1);
        assert_eq!(fj, 0);
    }
    #[test]
    fn test_second_price_equilibrium() {
        let game = AuctionGame::new(3, vec![10.0, 7.0, 5.0]);
        let (winner, price) = game.second_price_equilibrium();
        assert_eq!(winner, 0);
        assert!((price - 7.0).abs() < 1e-9);
    }
    #[test]
    fn test_first_price_bayes_nash() {
        let game = AuctionGame::new(2, vec![0.8, 0.6]);
        let bids = game.first_price_bayes_nash();
        assert!((bids[0] - 0.4).abs() < 1e-9);
        assert!((bids[1] - 0.3).abs() < 1e-9);
    }
    #[test]
    fn test_replicator_dynamics_simulation() {
        let payoff = vec![vec![0.0, 3.0], vec![1.0, 2.0]];
        let sim = ReplicatorDynamics::new(payoff);
        let init = vec![0.5, 0.5];
        let final_pop = sim.simulate(&init, 0.01, 100);
        let total: f64 = final_pop.iter().sum();
        assert!((total - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_ess_checker_prisoners_dilemma() {
        let payoff = vec![vec![3.0, 0.0], vec![5.0, 1.0]];
        let checker = ESSChecker::new(payoff);
        assert!(!checker.is_ess(0));
        assert!(checker.is_ess(1));
        assert!(checker.has_ess());
    }
    #[test]
    fn test_ess_checker_coordination() {
        let payoff = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let checker = ESSChecker::new(payoff);
        assert!(checker.is_ess(0));
        assert!(checker.is_ess(1));
        let all = checker.all_ess();
        assert_eq!(all.len(), 2);
    }
    #[test]
    fn test_gale_shapley_stable() {
        let proposer_prefs = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let acceptor_prefs = vec![vec![2, 1, 0], vec![0, 1, 2], vec![0, 1, 2]];
        let gs = GaleShapleyAlgorithm::new(3, proposer_prefs, acceptor_prefs);
        let matching = gs.run();
        assert!(gs.is_stable(&matching));
    }
    #[test]
    fn test_mechanism_design_checker_dsic() {
        let valuations = vec![1.0, 0.6];
        let allocations = vec![1.0, 0.0];
        let transfers = vec![0.6, 0.0];
        let checker = MechanismDesignChecker::new(valuations, allocations, transfers);
        assert!(checker.is_ir());
        assert!(checker.is_dsic());
        assert!((checker.revenue() - 0.6).abs() < 1e-9);
    }
    #[test]
    fn test_correlated_equilibrium_prisoners_dilemma() {
        let payoff_a = vec![vec![3.0, 0.0], vec![5.0, 1.0]];
        let payoff_b = vec![vec![3.0, 5.0], vec![0.0, 1.0]];
        let solver = CorrelatedEquilibriumSolver::new(payoff_a, payoff_b);
        let sigma = vec![vec![0.0, 0.0], vec![0.0, 1.0]];
        assert!(solver.is_correlated_equilibrium(&sigma));
    }
    #[test]
    fn test_cooperative_game_superadditive() {
        let chars = vec![0.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 3.0];
        let players = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let game = CooperativeGame::new(players, chars);
        assert!(game.is_superadditive());
    }
    #[test]
    fn test_cooperative_game_convex() {
        let chars = vec![0.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 3.0];
        let players = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let game = CooperativeGame::new(players, chars);
        assert!(game.is_convex());
    }
    #[test]
    fn test_all_pay_auction_equilibrium() {
        let game = AuctionGame::new(2, vec![0.8, 0.6]);
        let bids = game.all_pay_equilibrium();
        let expected_0 = 0.8_f64.powf(2.0) / 2.0;
        let expected_1 = 0.6_f64.powf(2.0) / 2.0;
        assert!((bids[0] - expected_0).abs() < 1e-9);
        assert!((bids[1] - expected_1).abs() < 1e-9);
    }
    #[test]
    fn test_build_game_theory_ext_env() {
        let mut env = oxilean_kernel::Environment::new();
        build_game_theory_ext_env(&mut env);
        for name in &[
            "SubgamePerfectEq",
            "FolkTheorem",
            "VCGTruthful",
            "StableMatching",
            "ArrowImpossibility",
            "CorrelatedEquilibrium",
            "PotentialGame",
            "GibbardSatterthwaite",
        ] {
            assert!(
                env.get_type(&Name::str(*name)).is_some(),
                "missing axiom: {name}"
            );
        }
    }
}
#[cfg(test)]
mod tests_game_theory_extended {
    use super::*;
    #[test]
    fn test_replicator_dynamics_rhs() {
        let rd = ReplicatorDynamicsEvo::new(vec![vec![3.0, 0.0], vec![5.0, 1.0]]);
        let x = vec![0.5, 0.5];
        let rhs = rd.replicator_rhs(&x);
        let sum: f64 = rhs.iter().sum();
        assert!(sum.abs() < 1e-10, "Sum of RHS = {sum}");
    }
    #[test]
    fn test_replicator_simplex_projection() {
        let rd = ReplicatorDynamicsEvo::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        let x = vec![0.3, 0.7];
        let proj = rd.project_simplex(&x);
        let sum: f64 = proj.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_correlated_equilibrium_valid() {
        let ce = CorrelatedEquilibrium::new(2, 2);
        assert!(ce.is_valid());
        assert!((ce.total_mass() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_stackelberg_equilibrium() {
        let game = StackelbergGameExt::new(
            2,
            2,
            vec![vec![2.0, 0.0], vec![0.0, 1.0]],
            vec![vec![1.0, 0.0], vec![0.0, 2.0]],
        );
        let (li, fj, _payoff) = game.stackelberg_equilibrium();
        assert_eq!(li, 0);
        assert_eq!(fj, 0);
    }
    #[test]
    fn test_follower_best_response() {
        let game = StackelbergGameExt::new(
            2,
            2,
            vec![vec![3.0, 1.0], vec![0.0, 2.0]],
            vec![vec![1.0, 3.0], vec![2.0, 0.0]],
        );
        assert_eq!(game.follower_best_response(0), 1);
    }
}
