//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

/// Register Vec.zip axioms: zip two vectors of the same length element-wise.
#[allow(dead_code)]
pub fn register_vec_zip_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zip_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("n"),
                Box::new(nat_ty.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("xs"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Vec"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("ys"),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Vec"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::Const(Name::str("Unit"), vec![])),
                    )),
                )),
            )),
        )),
    );
    let zip_decl = Declaration::Axiom {
        name: Name::str("Vec.zip"),
        univ_params: vec![],
        ty: zip_ty,
    };
    let _ = env.add(zip_decl);
    let zip_nil_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        )),
    );
    let zip_nil_decl = Declaration::Axiom {
        name: Name::str("Vec.zip_nil"),
        univ_params: vec![],
        ty: zip_nil_ty,
    };
    let _ = env.add(zip_nil_decl);
    let zip_length_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("n"),
                Box::new(nat_ty.clone()),
                Box::new(Expr::Const(Name::str("True"), vec![])),
            )),
        )),
    );
    let zip_length_decl = Declaration::Axiom {
        name: Name::str("Vec.zip_length"),
        univ_params: vec![],
        ty: zip_length_ty,
    };
    let _ = env.add(zip_length_decl);
}
/// Register Vec.unzip axioms: unzip a vector of pairs into two vectors.
#[allow(dead_code)]
pub fn register_vec_unzip_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let unzip_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("n"),
                Box::new(nat_ty.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("xs"),
                    Box::new(Expr::Const(Name::str("Unit"), vec![])),
                    Box::new(Expr::Const(Name::str("Unit"), vec![])),
                )),
            )),
        )),
    );
    let unzip_decl = Declaration::Axiom {
        name: Name::str("Vec.unzip"),
        univ_params: vec![],
        ty: unzip_ty,
    };
    let _ = env.add(unzip_decl);
    let zip_unzip_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("n"),
                Box::new(nat_ty.clone()),
                Box::new(Expr::Const(Name::str("True"), vec![])),
            )),
        )),
    );
    let zip_unzip_decl = Declaration::Axiom {
        name: Name::str("Vec.zip_unzip"),
        univ_params: vec![],
        ty: zip_unzip_ty,
    };
    let _ = env.add(zip_unzip_decl);
}
/// Register Vec.foldl and Vec.foldr axioms.
#[allow(dead_code)]
pub fn register_vec_fold_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let foldl_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_acc"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_x"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(3)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("init"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("n"),
                        Box::new(nat_ty.clone()),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_xs"),
                            Box::new(Expr::Const(Name::str("Unit"), vec![])),
                            Box::new(Expr::BVar(4)),
                        )),
                    )),
                )),
            )),
        )),
    );
    let foldl_decl = Declaration::Axiom {
        name: Name::str("Vec.foldl"),
        univ_params: vec![],
        ty: foldl_ty,
    };
    let _ = env.add(foldl_decl);
    let foldl_nil_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        )),
    );
    let foldl_nil_decl = Declaration::Axiom {
        name: Name::str("Vec.foldl_nil"),
        univ_params: vec![],
        ty: foldl_nil_ty,
    };
    let _ = env.add(foldl_nil_decl);
    let foldl_cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        )),
    );
    let foldl_cons_decl = Declaration::Axiom {
        name: Name::str("Vec.foldl_cons"),
        univ_params: vec![],
        ty: foldl_cons_ty,
    };
    let _ = env.add(foldl_cons_decl);
    let foldr_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_f"),
                Box::new(Expr::Const(Name::str("Unit"), vec![])),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_init"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("n"),
                        Box::new(nat_ty.clone()),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_xs"),
                            Box::new(Expr::Const(Name::str("Unit"), vec![])),
                            Box::new(Expr::BVar(4)),
                        )),
                    )),
                )),
            )),
        )),
    );
    let foldr_decl = Declaration::Axiom {
        name: Name::str("Vec.foldr"),
        univ_params: vec![],
        ty: foldr_ty,
    };
    let _ = env.add(foldr_decl);
}
/// Register Vec.all and Vec.any axioms.
#[allow(dead_code)]
pub fn register_vec_predicate_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let all_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_p"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Const(Name::str("Bool"), vec![])),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_xs"),
                    Box::new(Expr::Const(Name::str("Unit"), vec![])),
                    Box::new(Expr::Const(Name::str("Bool"), vec![])),
                )),
            )),
        )),
    );
    let all_decl = Declaration::Axiom {
        name: Name::str("Vec.all"),
        univ_params: vec![],
        ty: all_ty,
    };
    let _ = env.add(all_decl);
    let any_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_p"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Const(Name::str("Bool"), vec![])),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_xs"),
                    Box::new(Expr::Const(Name::str("Unit"), vec![])),
                    Box::new(Expr::Const(Name::str("Bool"), vec![])),
                )),
            )),
        )),
    );
    let any_decl = Declaration::Axiom {
        name: Name::str("Vec.any"),
        univ_params: vec![],
        ty: any_ty,
    };
    let _ = env.add(any_decl);
    let all_true_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        )),
    );
    let all_true_decl = Declaration::Axiom {
        name: Name::str("Vec.all_true"),
        univ_params: vec![],
        ty: all_true_ty,
    };
    let _ = env.add(all_true_decl);
}
/// Register Vec.scanl axiom: prefix sums generalization.
#[allow(dead_code)]
pub fn register_vec_scan_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let scanl_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_f"),
                Box::new(Expr::Const(Name::str("Unit"), vec![])),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_init"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Implicit,
                        Name::str("n"),
                        Box::new(nat_ty.clone()),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_xs"),
                            Box::new(Expr::Const(Name::str("Unit"), vec![])),
                            Box::new(Expr::Const(Name::str("Unit"), vec![])),
                        )),
                    )),
                )),
            )),
        )),
    );
    let scanl_decl = Declaration::Axiom {
        name: Name::str("Vec.scanl"),
        univ_params: vec![],
        ty: scanl_ty,
    };
    let _ = env.add(scanl_decl);
    let scanl_length_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("n"),
                Box::new(nat_ty.clone()),
                Box::new(Expr::Const(Name::str("True"), vec![])),
            )),
        )),
    );
    let scanl_length_decl = Declaration::Axiom {
        name: Name::str("Vec.scanl_length"),
        univ_params: vec![],
        ty: scanl_length_ty,
    };
    let _ = env.add(scanl_length_decl);
}
/// Register Vec.replicate axiom: create a vector of n copies of a value.
#[allow(dead_code)]
pub fn register_vec_replicate_axioms(env: &mut Environment) {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let replicate_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_x"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Vec"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    let replicate_decl = Declaration::Axiom {
        name: Name::str("Vec.replicate"),
        univ_params: vec![],
        ty: replicate_ty,
    };
    let _ = env.add(replicate_decl);
    let rep_zero_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_x"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Const(Name::str("True"), vec![])),
        )),
    );
    let rep_zero_decl = Declaration::Axiom {
        name: Name::str("Vec.replicate_zero"),
        univ_params: vec![],
        ty: rep_zero_ty,
    };
    let _ = env.add(rep_zero_decl);
    let rep_succ_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_x"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Const(Name::str("True"), vec![])),
            )),
        )),
    );
    let rep_succ_decl = Declaration::Axiom {
        name: Name::str("Vec.replicate_succ"),
        univ_params: vec![],
        ty: rep_succ_ty,
    };
    let _ = env.add(rep_succ_decl);
    let map_rep_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(2)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("n"),
                    Box::new(nat_ty.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_x"),
                        Box::new(Expr::BVar(3)),
                        Box::new(Expr::Const(Name::str("True"), vec![])),
                    )),
                )),
            )),
        )),
    );
    let map_rep_decl = Declaration::Axiom {
        name: Name::str("Vec.map_replicate"),
        univ_params: vec![],
        ty: map_rep_ty,
    };
    let _ = env.add(map_rep_decl);
}
#[cfg(test)]
mod tests_vec_extended {
    use super::*;
    use oxilean_kernel::{Environment, InductiveEnv, Name};
    #[test]
    fn test_vec_zip_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_zip_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.zip")).is_some());
        assert!(env.get(&Name::str("Vec.zip_nil")).is_some());
        assert!(env.get(&Name::str("Vec.zip_length")).is_some());
    }
    #[test]
    fn test_vec_unzip_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_unzip_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.unzip")).is_some());
        assert!(env.get(&Name::str("Vec.zip_unzip")).is_some());
    }
    #[test]
    fn test_vec_fold_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_fold_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.foldl")).is_some());
        assert!(env.get(&Name::str("Vec.foldl_nil")).is_some());
        assert!(env.get(&Name::str("Vec.foldl_cons")).is_some());
        assert!(env.get(&Name::str("Vec.foldr")).is_some());
    }
    #[test]
    fn test_vec_predicate_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_predicate_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.all")).is_some());
        assert!(env.get(&Name::str("Vec.any")).is_some());
        assert!(env.get(&Name::str("Vec.all_true")).is_some());
    }
    #[test]
    fn test_vec_scan_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_scan_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.scanl")).is_some());
        assert!(env.get(&Name::str("Vec.scanl_length")).is_some());
    }
    #[test]
    fn test_vec_replicate_axioms_registered() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let _ = build_vec_env(&mut env, &mut ind_env);
        register_vec_replicate_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.replicate")).is_some());
        assert!(env.get(&Name::str("Vec.replicate_zero")).is_some());
        assert!(env.get(&Name::str("Vec.replicate_succ")).is_some());
        assert!(env.get(&Name::str("Vec.map_replicate")).is_some());
    }
}
