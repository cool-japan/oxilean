//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{OrderedPair, Relation, SetPartition};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_set_env() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set", "Set.mem", "Set.empty", "Set.univ", "Set.singleton"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_set_operations_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Set.union",
            "Set.inter",
            "Set.diff",
            "Set.compl",
            "Set.subset",
            "Set.ssubset",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_set_image_preimage_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set.image", "Set.preimage", "Set.range"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_set_builder_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set.sep", "Set.insert", "Set.erase"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_finset_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Finset",
            "Finset.empty",
            "Finset.insert",
            "Finset.card",
            "Finset.union",
            "Finset.inter",
            "Finset.toSet",
            "Finset.sum",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_membership_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Set.mem_union",
            "Set.mem_inter",
            "Set.mem_diff",
            "Set.mem_compl",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_commutativity_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Set.union_comm",
            "Set.union_assoc",
            "Set.inter_comm",
            "Set.inter_assoc",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_identity_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Set.union_empty",
            "Set.empty_union",
            "Set.inter_univ",
            "Set.univ_inter",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_subset_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set.subset_refl", "Set.subset_trans", "Set.subset_antisymm"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_distributivity_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set.union_inter_distrib", "Set.inter_union_distrib"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_de_morgan_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in ["Set.compl_compl", "Set.compl_union", "Set.compl_inter"] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_image_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Set.image_comp",
            "Set.preimage_comp",
            "Set.image_union",
            "Set.image_inter_subset",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_finset_theorems_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        for name in [
            "Finset.card_empty",
            "Finset.card_insert",
            "Finset.card_union_le",
        ] {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_mk_set_structure() {
        let e = mk_set(nat_ty());
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Set"));
            } else {
                panic!("expected Const for Set");
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("Nat"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_mem_structure() {
        let elem = cst("x");
        let s = cst("my_set");
        let e = mk_set_mem(elem, s);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_set"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("x"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.mem"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_union_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_set_union(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("s"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.union"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_inter_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_set_inter(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.inter"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_subset_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_set_subset(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.subset"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_diff_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_set_diff(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.diff"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_compl_structure() {
        let s = cst("s");
        let e = mk_set_compl(s);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("s"));
            }
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Set.compl"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_image_structure() {
        let f = cst("my_func");
        let s = cst("my_set");
        let e = mk_set_image(f, s);
        if let Expr::App(outer_f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_set"));
            }
            if let Expr::App(g, lhs) = outer_f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("my_func"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.image"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_preimage_structure() {
        let f = cst("my_func");
        let s = cst("my_set");
        let e = mk_set_preimage(f, s);
        if let Expr::App(outer_f, _) = &e {
            if let Expr::App(g, _) = outer_f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.preimage"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_range_structure() {
        let f = cst("my_func");
        let e = mk_set_range(f);
        if let Expr::App(g, arg) = &e {
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_func"));
            }
            if let Expr::Const(n, _) = g.as_ref() {
                assert_eq!(*n, Name::str("Set.range"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_singleton_structure() {
        let a = cst("x");
        let e = mk_set_singleton(a);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("x"));
            }
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Set.singleton"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_insert_structure() {
        let a = cst("x");
        let s = cst("my_set");
        let e = mk_set_insert(a, s);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_set"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.insert"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_sep_structure() {
        let s = cst("my_set");
        let p = cst("my_pred");
        let e = mk_set_sep(s, p);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_pred"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.sep"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_empty_structure() {
        let e = mk_set_empty();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Set.empty"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_set_univ_structure() {
        let e = mk_set_univ();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Set.univ"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_finset_structure() {
        let e = mk_finset(nat_ty());
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Finset"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("Nat"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_empty_structure() {
        let e = mk_finset_empty();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Finset.empty"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_finset_insert_structure() {
        let a = cst("x");
        let s = cst("my_finset");
        let e = mk_finset_insert(a, s);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_finset"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Finset.insert"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_card_structure() {
        let s = cst("my_finset");
        let e = mk_finset_card(s);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Finset.card"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_finset"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_union_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_finset_union(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Finset.union"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_inter_structure() {
        let s = cst("s");
        let t = cst("t");
        let e = mk_finset_inter(s, t);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("t"));
            }
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Finset.inter"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_to_set_structure() {
        let s = cst("my_finset");
        let e = mk_finset_to_set(s);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Finset.toSet"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("my_finset"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_sum_structure() {
        let s = cst("my_finset");
        let f = cst("my_func");
        let e = mk_finset_sum(s, f);
        if let Expr::App(outer_f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_func"));
            }
            if let Expr::App(g, _) = outer_f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Finset.sum"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_set_type_is_type_to_type() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        let decl = env.get(&Name::str("Set"));
        assert!(decl.is_some(), "Set should exist in environment");
    }
    #[test]
    fn test_finset_type_is_type_to_type() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        let decl = env.get(&Name::str("Finset"));
        assert!(decl.is_some(), "Finset should exist in environment");
    }
    #[test]
    fn test_all_declarations_registered() {
        let mut env = Environment::new();
        build_set_env(&mut env).expect("build_set_env should succeed");
        let all_names = [
            "Set",
            "Set.mem",
            "Set.empty",
            "Set.univ",
            "Set.singleton",
            "Set.union",
            "Set.inter",
            "Set.diff",
            "Set.compl",
            "Set.subset",
            "Set.ssubset",
            "Set.image",
            "Set.preimage",
            "Set.range",
            "Set.sep",
            "Set.insert",
            "Set.erase",
            "Set.mem_union",
            "Set.mem_inter",
            "Set.mem_diff",
            "Set.mem_compl",
            "Set.union_comm",
            "Set.union_assoc",
            "Set.inter_comm",
            "Set.inter_assoc",
            "Set.union_empty",
            "Set.empty_union",
            "Set.inter_univ",
            "Set.univ_inter",
            "Set.subset_refl",
            "Set.subset_trans",
            "Set.subset_antisymm",
            "Set.union_inter_distrib",
            "Set.inter_union_distrib",
            "Set.compl_compl",
            "Set.compl_union",
            "Set.compl_inter",
            "Set.image_comp",
            "Set.preimage_comp",
            "Set.image_union",
            "Set.image_inter_subset",
            "Finset",
            "Finset.empty",
            "Finset.insert",
            "Finset.card",
            "Finset.union",
            "Finset.inter",
            "Finset.toSet",
            "Finset.sum",
            "Finset.card_empty",
            "Finset.card_insert",
            "Finset.card_union_le",
        ];
        for name in all_names {
            assert!(
                env.contains(&Name::str(name)),
                "environment should contain {name}"
            );
        }
    }
    #[test]
    fn test_set_ty_of_helper() {
        let e = set_ty_of(nat_ty());
        if let Expr::Pi(bi, _, dom, body) = &e {
            assert_eq!(*bi, BinderInfo::Default);
            assert!(matches!(dom.as_ref(), Expr::Const(_, _)));
            assert!(matches!(body.as_ref(), Expr::Sort(_)));
        } else {
            panic!("expected Pi");
        }
    }
    #[test]
    fn test_mk_set_union_of_empty() {
        let e = mk_set_union(mk_set_empty(), mk_set_empty());
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("Set.empty"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("Set.empty"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.union"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_set_inter_compl() {
        let s = cst("s");
        let e = mk_set_inter(s.clone(), mk_set_compl(s));
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_set_nested_image() {
        let f = cst("f");
        let g = cst("g");
        let s = cst("s");
        let inner = mk_set_image(f, s);
        let e = mk_set_image(g, inner);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_set_subset_refl_structure() {
        let s = cst("my_set");
        let e = mk_set_subset(s.clone(), s);
        if let Expr::App(f, rhs) = &e {
            if let Expr::Const(n, _) = rhs.as_ref() {
                assert_eq!(*n, Name::str("my_set"));
            }
            if let Expr::App(g, lhs) = f.as_ref() {
                if let Expr::Const(n, _) = lhs.as_ref() {
                    assert_eq!(*n, Name::str("my_set"));
                }
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Set.subset"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_eq_expr_set_type() {
        let ty = mk_set(nat_ty());
        let lhs = cst("s");
        let rhs = cst("t");
        let e = mk_eq_expr(ty, lhs, rhs);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::App(h, _) = g.as_ref() {
                    if let Expr::Const(n, _) = h.as_ref() {
                        assert_eq!(*n, Name::str("Eq"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_finset_nested_insert() {
        let a = cst("a");
        let b = cst("b");
        let s = mk_finset_empty();
        let s1 = mk_finset_insert(a, s);
        let s2 = mk_finset_insert(b, s1);
        assert!(matches!(s2, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_finset_card_of_empty() {
        let e = mk_finset_card(mk_finset_empty());
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Finset.card"));
            }
            if let Expr::Const(n, _) = arg.as_ref() {
                assert_eq!(*n, Name::str("Finset.empty"));
            }
        } else {
            panic!("expected App");
        }
    }
}
/// Power set functor: P(A) = set of all subsets of A.
#[allow(dead_code)]
pub fn power_set<T: Clone>(elements: &[T]) -> Vec<Vec<T>> {
    let n = elements.len();
    (0..(1usize << n))
        .map(|mask| {
            (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| elements[i].clone())
                .collect()
        })
        .collect()
}
/// Cartesian product of two finite sets.
#[allow(dead_code)]
pub fn cartesian_product<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter()
        .flat_map(|x| b.iter().map(move |y| (x.clone(), y.clone())))
        .collect()
}
/// Bell number: number of partitions of an n-element set.
#[allow(dead_code)]
pub fn bell_number(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut row = vec![1u64];
    for _ in 1..=n {
        let mut new_row = vec![*row.last().expect("row is non-empty: initialized with [1]")];
        for j in 0..row.len() {
            new_row.push(new_row[j] + row[j]);
        }
        row = new_row;
    }
    row[0]
}
#[cfg(test)]
mod tests_set_extra {
    use super::*;
    #[test]
    fn test_ordered_pair() {
        let p = OrderedPair::new(1, "hello");
        assert_eq!(p.fst, 1);
        let q = p.swap();
        assert_eq!(q.fst, "hello");
        assert_eq!(q.snd, 1);
    }
    #[test]
    fn test_relation_equivalence() {
        let dom = vec![0, 1, 2];
        let mut r = Relation::new(dom);
        r.add_pair(&0, &0);
        r.add_pair(&1, &1);
        r.add_pair(&2, &2);
        r.add_pair(&0, &1);
        r.add_pair(&1, &0);
        assert!(r.is_reflexive());
        assert!(r.is_symmetric());
        assert!(r.is_transitive());
        assert!(r.is_equivalence());
    }
    #[test]
    fn test_partial_order() {
        let dom = vec![0, 1, 2, 3];
        let mut r = Relation::new(dom);
        for i in 0..=3 {
            r.add_pair(&i, &i);
        }
        for i in 0..3 {
            r.add_pair(&i, &(i + 1));
        }
        r.add_pair(&0, &2);
        r.add_pair(&0, &3);
        r.add_pair(&1, &3);
        assert!(r.is_antisymmetric());
        assert!(r.is_partial_order());
    }
    #[test]
    fn test_power_set() {
        let ps = power_set(&[1, 2, 3]);
        assert_eq!(ps.len(), 8);
    }
    #[test]
    fn test_cartesian_product() {
        let cp = cartesian_product(&[1, 2], &['a', 'b', 'c']);
        assert_eq!(cp.len(), 6);
    }
    #[test]
    fn test_set_partition() {
        let disc = SetPartition::discrete(4);
        assert_eq!(disc.n_blocks(), 4);
        assert!(disc.is_valid());
        let mut p = SetPartition::discrete(4);
        p.merge_blocks(0, 1);
        assert_eq!(p.n_blocks(), 3);
        assert!(p.is_valid());
        let trivial = SetPartition::trivial(4);
        assert_eq!(trivial.n_blocks(), 1);
    }
    #[test]
    fn test_bell_numbers() {
        assert_eq!(bell_number(0), 1);
        assert_eq!(bell_number(1), 1);
        assert_eq!(bell_number(2), 2);
        assert_eq!(bell_number(3), 5);
        assert_eq!(bell_number(4), 15);
        assert_eq!(bell_number(5), 52);
    }
}
