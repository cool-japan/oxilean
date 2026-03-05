//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet};

use super::types::{
    GeneralizationResult, InductionConfig, InductionScheme, MinorPremise, MutualInductionConfig,
    WellFoundedConfig,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::induction_adv::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_list_expr() -> Expr {
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        )
    }
    fn mk_bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn mk_goal_state(ctx: &mut MetaContext) -> (MVarId, TacticState) {
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        (mvar_id, state)
    }
    #[test]
    fn test_infer_scheme_nat() {
        let ctx = mk_ctx();
        let nat = mk_nat_expr();
        let scheme = infer_induction_scheme(&nat, &ctx).expect("scheme should be present");
        assert_eq!(scheme.inductive_name, Name::str("Nat"));
        assert_eq!(scheme.num_params, 0);
        assert_eq!(scheme.num_indices, 0);
        assert_eq!(scheme.minor_premises.len(), 2);
        assert_eq!(scheme.minor_premises[0].ctor_name, Name::str("Nat.zero"));
        assert_eq!(scheme.minor_premises[0].num_fields, 0);
        assert_eq!(scheme.minor_premises[0].num_recursive_args, 0);
        assert_eq!(scheme.minor_premises[1].ctor_name, Name::str("Nat.succ"));
        assert_eq!(scheme.minor_premises[1].num_fields, 1);
        assert_eq!(scheme.minor_premises[1].num_recursive_args, 1);
    }
    #[test]
    fn test_infer_scheme_list() {
        let ctx = mk_ctx();
        let list = mk_list_expr();
        let scheme = infer_induction_scheme(&list, &ctx).expect("scheme should be present");
        assert_eq!(scheme.inductive_name, Name::str("List"));
        assert_eq!(scheme.num_params, 1);
        assert_eq!(scheme.minor_premises.len(), 2);
        assert_eq!(scheme.minor_premises[0].ctor_name, Name::str("List.nil"));
        assert_eq!(scheme.minor_premises[1].ctor_name, Name::str("List.cons"));
        assert_eq!(scheme.minor_premises[1].num_fields, 2);
        assert_eq!(scheme.minor_premises[1].num_recursive_args, 1);
    }
    #[test]
    fn test_infer_scheme_bool() {
        let ctx = mk_ctx();
        let b = mk_bool_expr();
        let scheme = infer_induction_scheme(&b, &ctx).expect("scheme should be present");
        assert_eq!(scheme.minor_premises.len(), 2);
        assert_eq!(scheme.minor_premises[0].num_recursive_args, 0);
        assert_eq!(scheme.minor_premises[1].num_recursive_args, 0);
    }
    #[test]
    fn test_infer_scheme_unknown_fails() {
        let ctx = mk_ctx();
        let unknown = Expr::Const(Name::str("UnknownType"), vec![]);
        let result = infer_induction_scheme(&unknown, &ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_minor_premise_creation() {
        let mp = MinorPremise::new(Name::str("Nat.succ"), 1, 1);
        assert_eq!(mp.num_fields, 1);
        assert_eq!(mp.num_recursive_args, 1);
        assert_eq!(mp.total_binders(), 2);
        assert!(!mp.has_non_recursive_fields);
        assert_eq!(mp.default_ih_names().len(), 1);
        assert_eq!(mp.default_ih_names()[0], Name::str("ih"));
    }
    #[test]
    fn test_minor_premise_multiple_recursive() {
        let mp = MinorPremise::new(Name::str("Tree.node"), 3, 2);
        assert_eq!(mp.total_binders(), 5);
        assert!(mp.has_non_recursive_fields);
        let ihs = mp.default_ih_names();
        assert_eq!(ihs.len(), 2);
        assert_eq!(ihs[0], Name::str("ih_1"));
        assert_eq!(ihs[1], Name::str("ih_2"));
    }
    #[test]
    fn test_config_default() {
        let config = InductionConfig::default();
        assert!(!config.has_generalization());
        assert!(!config.has_custom_recursor());
        assert!(config.revert_deps);
        assert!(config.clear_target);
    }
    #[test]
    fn test_config_with_recursor() {
        let config = InductionConfig::using(Name::str("Nat.recAux"));
        assert!(config.has_custom_recursor());
        assert_eq!(config.using_recursor, Some(Name::str("Nat.recAux")));
    }
    #[test]
    fn test_config_with_generalizing() {
        let config = InductionConfig::generalizing(vec![Name::str("m"), Name::str("k")]);
        assert!(config.has_generalization());
        assert_eq!(config.generalizing.len(), 2);
    }
    #[test]
    fn test_recursor_compatibility_nat() {
        let ctx = mk_ctx();
        let nat = mk_nat_expr();
        let result = check_recursor_compatibility(&Name::str("Nat.rec"), &nat, &ctx)
            .expect("result should be present");
        assert!(result);
    }
    #[test]
    fn test_recursor_compatibility_wrong_type() {
        let ctx = mk_ctx();
        let nat = mk_nat_expr();
        let result = check_recursor_compatibility(&Name::str("List.rec"), &nat, &ctx)
            .expect("result should be present");
        assert!(!result);
    }
    #[test]
    fn test_recursor_compatibility_unknown() {
        let ctx = mk_ctx();
        let nat = mk_nat_expr();
        let result = check_recursor_compatibility(&Name::str("NonExistent.rec"), &nat, &ctx)
            .expect("value should be present");
        assert!(!result);
    }
    #[test]
    fn test_tac_induction_adv_nat() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let target = mk_nat_expr();
        let config = InductionConfig::default();
        let goals = tac_induction_adv(&target, &config, &mut state, &mut ctx)
            .expect("goals should be present");
        assert_eq!(goals.len(), 2);
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_tac_induction_adv_list() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let target = mk_list_expr();
        let config = InductionConfig::default();
        let goals = tac_induction_adv(&target, &config, &mut state, &mut ctx)
            .expect("goals should be present");
        assert_eq!(goals.len(), 2);
    }
    #[test]
    fn test_tac_induction_adv_with_names() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let target = mk_nat_expr();
        let config = InductionConfig::default()
            .with_names(vec![vec![], vec![Name::str("n"), Name::str("ih_n")]]);
        let goals = tac_induction_adv(&target, &config, &mut state, &mut ctx)
            .expect("goals should be present");
        assert_eq!(goals.len(), 2);
    }
    #[test]
    fn test_generalize_empty() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let result = tac_generalize(&[], &mut state, &mut ctx).expect("result should be present");
        assert!(result.is_trivial());
        assert_eq!(result.num_generalized, 0);
    }
    #[test]
    fn test_generalize_single() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let x = Expr::Const(Name::str("x"), vec![]);
        let pairs = vec![(x, Name::str("y"))];
        let result =
            tac_generalize(&pairs, &mut state, &mut ctx).expect("result should be present");
        assert!(!result.is_trivial());
        assert_eq!(result.num_generalized, 1);
        assert_eq!(result.reverted.len(), 1);
    }
    #[test]
    fn test_generalize_multiple() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let x = Expr::Const(Name::str("x"), vec![]);
        let y = Expr::Const(Name::str("y"), vec![]);
        let pairs = vec![(x, Name::str("a")), (y, Name::str("b"))];
        let result =
            tac_generalize(&pairs, &mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.num_generalized, 2);
    }
    #[test]
    fn test_wf_induction_nat() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let target = Expr::Const(Name::str("n"), vec![]);
        let rel = Expr::Const(Name::str("Nat.lt"), vec![]);
        let config = WellFoundedConfig::default();
        let goals = tac_well_founded_induction(&target, &rel, &config, &mut state, &mut ctx)
            .expect("value should be present");
        assert_eq!(goals.len(), 1);
    }
    #[test]
    fn test_wf_induction_with_measure() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let target = Expr::Const(Name::str("p"), vec![]);
        let rel = Expr::Const(Name::str("Nat.lt"), vec![]);
        let measure = Expr::Const(Name::str("Prod.fst"), vec![]);
        let config = WellFoundedConfig::with_measure(measure);
        assert!(config.has_measure());
        assert!(!config.has_explicit_relation());
        let goals = tac_well_founded_induction(&target, &rel, &config, &mut state, &mut ctx)
            .expect("value should be present");
        assert_eq!(goals.len(), 1);
    }
    #[test]
    fn test_mutual_induction_single_target() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let targets = vec![mk_nat_expr()];
        let config = MutualInductionConfig::for_targets(vec![Name::str("Nat")]);
        let goals = tac_mutual_induction(&targets, &config, &mut state, &mut ctx)
            .expect("goals should be present");
        assert_eq!(goals.len(), 2);
    }
    #[test]
    fn test_mutual_induction_empty_fails() {
        let mut ctx = mk_ctx();
        let (_mvar_id, mut state) = mk_goal_state(&mut ctx);
        let targets: Vec<Expr> = vec![];
        let config = MutualInductionConfig::default();
        let result = tac_mutual_induction(&targets, &config, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_decompose_app_simple() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let (head, args) = decompose_app(&nat);
        assert_eq!(head, Some(Name::str("Nat")));
        assert!(args.is_empty());
    }
    #[test]
    fn test_decompose_app_applied() {
        let list_nat = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let (head, args) = decompose_app(&list_nat);
        assert_eq!(head, Some(Name::str("List")));
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_extract_short_name() {
        assert_eq!(extract_short_name(&Name::str("Nat.succ")), "succ");
        assert_eq!(extract_short_name(&Name::str("List.cons")), "cons");
        assert_eq!(extract_short_name(&Name::str("foo")), "foo");
    }
    #[test]
    fn test_make_recursor_name() {
        let name = make_recursor_name(&Name::str("Nat"));
        assert_eq!(name, Name::str("Nat.rec"));
    }
    #[test]
    fn test_expr_mentions_name() {
        let f_x = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("x"), vec![])),
        );
        assert!(expr_mentions_name(&f_x, &Name::str("x")));
        assert!(expr_mentions_name(&f_x, &Name::str("f")));
        assert!(!expr_mentions_name(&f_x, &Name::str("y")));
    }
    #[test]
    fn test_abstract_expr_in_type() {
        let x = Expr::Const(Name::str("x"), vec![]);
        let f_x = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(x.clone()),
        );
        let result = abstract_expr_in_type(&f_x, &x);
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_rename_pi_binders() {
        let ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("a_0"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Pi(
                oxilean_kernel::BinderInfo::Default,
                Name::str("ih"),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(Expr::Sort(Level::zero())),
            )),
        );
        let names = vec![(0, Name::str("n")), (1, Name::str("ih_n"))];
        let result = rename_pi_binders(&ty, &names);
        match &result {
            Expr::Pi(_, name, _, body) => {
                assert_eq!(*name, Name::str("n"));
                match body.as_ref() {
                    Expr::Pi(_, inner_name, _, _) => {
                        assert_eq!(*inner_name, Name::str("ih_n"));
                    }
                    _ => panic!("expected inner Pi"),
                }
            }
            _ => panic!("expected Pi"),
        }
    }
    #[test]
    fn test_induction_scheme_display() {
        let scheme = InductionScheme::new_simple(Name::str("Nat.rec"), Name::str("Nat"), 3, 0, 0);
        let display = format!("{}", scheme);
        assert!(display.contains("Nat.rec"));
        assert!(display.contains("major=3"));
    }
    #[test]
    fn test_wf_config_defaults() {
        let config = WellFoundedConfig::default();
        assert!(!config.has_explicit_relation());
        assert!(!config.has_measure());
        assert!(config.auto_measure);
        assert_eq!(config.max_depth, 64);
    }
    #[test]
    fn test_mutual_config_exceeds_limit() {
        let config = MutualInductionConfig {
            max_mutual: 2,
            target_names: vec![Name::str("A"), Name::str("B"), Name::str("C")],
            ..MutualInductionConfig::default()
        };
        assert!(config.exceeds_limit());
    }
    #[test]
    fn test_generalization_result_trivial() {
        let result = GeneralizationResult {
            reverted: Vec::new(),
            new_goal: MVarId(0),
            generalized_type: Expr::Sort(Level::zero()),
            num_generalized: 0,
            hyp_positions: HashMap::new(),
            generalized_exprs: Vec::new(),
        };
        assert!(result.is_trivial());
        assert_eq!(result.num_binders(), 0);
    }
    #[test]
    fn test_count_inductive_occurrences_in_pi() {
        let nat = Name::str("Nat");
        let nat_ty = Expr::Const(nat.clone(), vec![]);
        let ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                oxilean_kernel::BinderInfo::Default,
                Name::str("m"),
                Box::new(nat_ty.clone()),
                Box::new(nat_ty),
            )),
        );
        assert_eq!(count_inductive_occurrences_in_pi(&ty, &nat), 2);
    }
    #[test]
    fn test_topological_sort_independent() {
        let hyps = vec![
            (Name::str("a"), Expr::Sort(Level::zero())),
            (Name::str("b"), Expr::Sort(Level::zero())),
        ];
        let names = vec![Name::str("a"), Name::str("b")];
        let sorted = topological_sort_hyps(&names, &hyps);
        assert_eq!(sorted.len(), 2);
    }
    #[test]
    fn test_topological_sort_dependent() {
        let hyps = vec![
            (Name::str("x"), Expr::Const(Name::str("Nat"), vec![])),
            (
                Name::str("h"),
                Expr::App(
                    Box::new(Expr::Const(Name::str("P"), vec![])),
                    Box::new(Expr::Const(Name::str("x"), vec![])),
                ),
            ),
        ];
        let names = vec![Name::str("x"), Name::str("h")];
        let sorted = topological_sort_hyps(&names, &hyps);
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0], Name::str("x"));
        assert_eq!(sorted[1], Name::str("h"));
    }
    #[test]
    fn test_induction_scheme_is_mutual() {
        let mut scheme =
            InductionScheme::new_simple(Name::str("Nat.rec"), Name::str("Nat"), 3, 0, 0);
        assert!(!scheme.is_mutual());
        scheme.mutual_names = vec![Name::str("Even"), Name::str("Odd")];
        assert!(scheme.is_mutual());
    }
}
