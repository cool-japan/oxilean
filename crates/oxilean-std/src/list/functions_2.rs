//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::functions::*;

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Literal;
    /// Set up an environment with prerequisites and List.
    fn full_env() -> (Environment, InductiveEnv) {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        add_axiom(&mut env, "Nat", vec![], type1()).expect("InductiveEnv::new should succeed");
        add_axiom(&mut env, "Nat.zero", vec![], nat_ty()).expect("unwrap should succeed");
        add_axiom(&mut env, "Nat.succ", vec![], arrow(nat_ty(), nat_ty()))
            .expect("unwrap should succeed");
        add_axiom(
            &mut env,
            "Nat.add",
            vec![],
            arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        )
        .expect("operation should succeed");
        add_axiom(
            &mut env,
            "Nat.le",
            vec![],
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        )
        .expect("operation should succeed");
        add_axiom(&mut env, "Bool", vec![], type1()).expect("unwrap should succeed");
        add_axiom(&mut env, "true", vec![], bool_ty()).expect("unwrap should succeed");
        add_axiom(&mut env, "false", vec![], bool_ty()).expect("unwrap should succeed");
        let eq_ty = implicit_pi(
            "alpha",
            type1(),
            default_pi("a", Expr::BVar(0), default_pi("b", Expr::BVar(1), prop())),
        );
        add_axiom(&mut env, "Eq", vec![], eq_ty).expect("operation should succeed");
        add_axiom(
            &mut env,
            "Iff",
            vec![],
            arrow(prop(), arrow(prop(), prop())),
        )
        .expect("operation should succeed");
        add_axiom(&mut env, "Or", vec![], arrow(prop(), arrow(prop(), prop())))
            .expect("unwrap should succeed");
        add_axiom(&mut env, "False", vec![], prop()).expect("unwrap should succeed");
        add_axiom(&mut env, "Option", vec![], arrow(type1(), type1()))
            .expect("unwrap should succeed");
        add_axiom(
            &mut env,
            "Prod",
            vec![],
            arrow(type1(), arrow(type1(), type1())),
        )
        .expect("operation should succeed");
        build_list_env(&mut env, &mut ind_env).expect("build_list_env should succeed");
        add_axiom(
            &mut env,
            "List.Mem",
            vec![],
            implicit_pi(
                "alpha",
                type1(),
                default_pi(
                    "a",
                    Expr::BVar(0),
                    default_pi("l", list_of(Expr::BVar(1)), prop()),
                ),
            ),
        )
        .expect("operation should succeed");
        (env, ind_env)
    }
    #[test]
    fn test_build_list_env() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List")));
        assert!(env.contains(&Name::str("List.nil")));
        assert!(env.contains(&Name::str("List.cons")));
        assert!(env.contains(&Name::str("List.rec")));
    }
    #[test]
    fn test_list_env_operations_present() {
        let (env, _) = full_env();
        let ops = [
            "List.map",
            "List.filter",
            "List.foldr",
            "List.foldl",
            "List.reverse",
            "List.length",
            "List.append",
            "List.head?",
            "List.tail",
            "List.nth?",
            "List.zip",
            "List.take",
            "List.drop",
            "List.any",
            "List.all",
            "List.replicate",
            "List.join",
            "List.iota",
            "List.range",
            "List.enumFrom",
        ];
        for name in &ops {
            assert!(
                env.contains(&Name::str(*name)),
                "missing operation: {}",
                name
            );
        }
    }
    #[test]
    fn test_list_env_theorems_present() {
        let (env, _) = full_env();
        let theorems = [
            "List.nil_append",
            "List.append_nil",
            "List.append_assoc",
            "List.length_nil",
            "List.length_cons",
            "List.length_append",
            "List.map_nil",
            "List.map_cons",
            "List.map_map",
            "List.map_id",
            "List.reverse_nil",
            "List.reverse_cons",
            "List.reverse_reverse",
            "List.filter_nil",
            "List.foldr_nil",
            "List.foldl_nil",
            "List.length_reverse",
            "List.length_map",
            "List.length_filter_le",
            "List.mem_nil",
            "List.mem_cons",
        ];
        for name in &theorems {
            assert!(env.contains(&Name::str(*name)), "missing theorem: {}", name);
        }
    }
    #[test]
    fn test_list_type() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let list_nat = list_type(nat);
        assert!(matches!(list_nat, Expr::App(_, _)));
    }
    #[test]
    fn test_list_nil_expr() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let nil = list_nil(nat);
        assert!(matches!(nil, Expr::App(_, _)));
    }
    #[test]
    fn test_list_cons_expr() {
        let head = Expr::Lit(Literal::Nat(1));
        let tail = list_nil(nat_ty());
        let cons = list_cons(head, tail);
        assert!(matches!(cons, Expr::App(_, _)));
    }
    #[test]
    fn test_list_map_expr() {
        let f = Expr::Const(Name::str("some_fn"), vec![]);
        let l = list_nil(nat_ty());
        let expr = list_map(f, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_filter_expr() {
        let p = Expr::Const(Name::str("is_even"), vec![]);
        let l = list_nil(nat_ty());
        let expr = list_filter(p, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_foldr_expr() {
        let f = Expr::Const(Name::str("add"), vec![]);
        let init = Expr::Lit(Literal::Nat(0));
        let l = list_nil(nat_ty());
        let expr = list_foldr(f, init, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_foldl_expr() {
        let f = Expr::Const(Name::str("add"), vec![]);
        let init = Expr::Lit(Literal::Nat(0));
        let l = list_nil(nat_ty());
        let expr = list_foldl(f, init, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_reverse_expr() {
        let l = list_nil(nat_ty());
        let expr = list_reverse(l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_length_expr() {
        let l = list_nil(nat_ty());
        let expr = list_length(l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_append_expr() {
        let l1 = list_nil(nat_ty());
        let l2 = list_nil(nat_ty());
        let expr = list_append(l1, l2);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_head_expr() {
        let l = list_nil(nat_ty());
        let expr = list_head(l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_tail_expr() {
        let l = list_nil(nat_ty());
        let expr = list_tail(l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_take_expr() {
        let n = Expr::Lit(Literal::Nat(3));
        let l = list_nil(nat_ty());
        let expr = list_take(n, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_drop_expr() {
        let n = Expr::Lit(Literal::Nat(2));
        let l = list_nil(nat_ty());
        let expr = list_drop(n, l);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_replicate_expr() {
        let n = Expr::Lit(Literal::Nat(5));
        let x = Expr::Lit(Literal::Nat(42));
        let expr = list_replicate(n, x);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_join_expr() {
        let ll = list_nil(list_type(nat_ty()));
        let expr = list_join(ll);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_list_range_expr() {
        let n = Expr::Lit(Literal::Nat(10));
        let expr = list_range(n);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_list_from_vec_empty() {
        let l = mk_list_from_vec(nat_ty(), vec![]);
        assert!(matches!(l, Expr::App(_, _)));
        if let Expr::App(f, _) = &l {
            assert!(matches!(f.as_ref(), Expr::Const(_, _)));
        }
    }
    #[test]
    fn test_mk_list_from_vec_single() {
        let l = mk_list_from_vec(nat_ty(), vec![Expr::Lit(Literal::Nat(1))]);
        assert!(matches!(l, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_list_from_vec_multiple() {
        let l = mk_list_from_vec(
            nat_ty(),
            vec![
                Expr::Lit(Literal::Nat(1)),
                Expr::Lit(Literal::Nat(2)),
                Expr::Lit(Literal::Nat(3)),
            ],
        );
        assert!(matches!(l, Expr::App(_, _)));
        if let Expr::App(f, tail) = &l {
            if let Expr::App(cons_f, head) = f.as_ref() {
                assert!(matches!(cons_f.as_ref(), Expr::Const(_, _)));
                assert_eq!(**head, Expr::Lit(Literal::Nat(1)));
            } else {
                panic!("expected App(List.cons, head)");
            }
            assert!(matches!(tail.as_ref(), Expr::App(_, _)));
        }
    }
    #[test]
    fn test_list_type_decl_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List"))
            .expect("declaration 'List' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_list_nil_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.nil"))
            .expect("declaration 'List.nil' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_list_cons_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.cons"))
            .expect("declaration 'List.cons' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_list_rec_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.rec"))
            .expect("declaration 'List.rec' should exist in env");
        assert!(decl.ty().is_pi());
        assert_eq!(decl.univ_params().len(), 1);
    }
    #[test]
    fn test_list_map_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.map"))
            .expect("declaration 'List.map' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_list_append_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.append"))
            .expect("declaration 'List.append' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_list_length_returns_nat() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.length"))
            .expect("declaration 'List.length' should exist in env");
        let ty = decl.ty();
        if let Expr::Pi(BinderInfo::Implicit, _, _, body) = ty {
            if let Expr::Pi(BinderInfo::Default, _, _, cod) = body.as_ref() {
                assert_eq!(**cod, nat_ty());
            } else {
                panic!("expected Pi");
            }
        } else {
            panic!("expected implicit Pi");
        }
    }
    #[test]
    fn test_list_declarations_are_axioms() {
        let (env, _) = full_env();
        for name in &["List", "List.nil", "List.cons", "List.rec", "List.map"] {
            let decl = env
                .get(&Name::str(*name))
                .expect("operation should succeed");
            assert!(
                matches!(decl, Declaration::Axiom { .. }),
                "{} should be an axiom",
                name
            );
        }
    }
    #[test]
    fn test_nil_append_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.nil_append"))
            .expect("declaration 'List.nil_append' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_reverse_reverse_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.reverse_reverse"))
            .expect("declaration 'List.reverse_reverse' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_env_declaration_count() {
        let (env, _) = full_env();
        assert!(env.len() >= 40);
    }
    #[test]
    fn test_list_operations_combined() {
        let l = mk_list_from_vec(
            nat_ty(),
            vec![
                Expr::Lit(Literal::Nat(1)),
                Expr::Lit(Literal::Nat(2)),
                Expr::Lit(Literal::Nat(3)),
            ],
        );
        let rev = list_reverse(l);
        let f = Expr::Const(Name::str("succ"), vec![]);
        let mapped = list_map(f, rev);
        assert!(matches!(mapped, Expr::App(_, _)));
    }
    #[test]
    fn test_list_append_two_lists() {
        let l1 = mk_list_from_vec(nat_ty(), vec![Expr::Lit(Literal::Nat(1))]);
        let l2 = mk_list_from_vec(nat_ty(), vec![Expr::Lit(Literal::Nat(2))]);
        let appended = list_append(l1, l2);
        assert!(matches!(appended, Expr::App(_, _)));
    }
    #[test]
    fn test_unzip_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.unzip")));
    }
    #[test]
    fn test_partition_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.partition")));
    }
    #[test]
    fn test_span_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.span")));
    }
    #[test]
    fn test_find_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.find?")));
    }
    #[test]
    fn test_count_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.count")));
    }
    #[test]
    fn test_intersperse_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.intersperse")));
    }
    #[test]
    fn test_transpose_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.transpose")));
    }
    #[test]
    fn test_perm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Perm")));
    }
    #[test]
    fn test_perm_refl_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Perm.refl")));
    }
    #[test]
    fn test_perm_symm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Perm.symm")));
    }
    #[test]
    fn test_perm_trans_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Perm.trans")));
    }
    #[test]
    fn test_sublist_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Sublist")));
    }
    #[test]
    fn test_sublist_refl_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Sublist.refl")));
    }
    #[test]
    fn test_sublist_trans_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Sublist.trans")));
    }
    #[test]
    fn test_decidable_mem_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.Decidable.mem")));
    }
    #[test]
    fn test_take_append_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.take_append")));
    }
    #[test]
    fn test_drop_append_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.drop_append")));
    }
    #[test]
    fn test_count_le_length_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("List.count_le_length")));
    }
    #[test]
    fn test_unzip_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.unzip"))
            .expect("declaration 'List.unzip' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_partition_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.partition"))
            .expect("declaration 'List.partition' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_find_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.find?"))
            .expect("declaration 'List.find?' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_count_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.count"))
            .expect("declaration 'List.count' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_perm_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.Perm"))
            .expect("declaration 'List.Perm' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_sublist_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("List.Sublist"))
            .expect("declaration 'List.Sublist' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_all_new_list_operations_present() {
        let (env, _) = full_env();
        let new_ops = [
            "List.unzip",
            "List.partition",
            "List.span",
            "List.find?",
            "List.count",
            "List.intersperse",
            "List.transpose",
            "List.Perm",
            "List.Perm.refl",
            "List.Perm.symm",
            "List.Perm.trans",
            "List.Sublist",
            "List.Sublist.refl",
            "List.Sublist.trans",
            "List.Decidable.mem",
            "List.take_append",
            "List.drop_append",
            "List.count_le_length",
        ];
        for name in &new_ops {
            assert!(
                env.contains(&Name::str(*name)),
                "missing operation: {}",
                name
            );
        }
    }
}
