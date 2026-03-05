//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BidirType, BidirectionalTypechecker, HomotopyLevel, HottPath, MlttTerm,
    NormalizationByEvaluation, STLCTerm, STLCType, STLCTypechecker, SystemFType,
    SystemFTypeInference, UniverseChecker, UniverseConstraint, UniverseExpr, UniverseLevel,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_homotopy_level_ordering() {
        assert!(HomotopyLevel::Contractible < HomotopyLevel::Proposition);
        assert!(HomotopyLevel::Proposition < HomotopyLevel::Set);
        assert!(HomotopyLevel::Set < HomotopyLevel::Groupoid);
        assert!(HomotopyLevel::Groupoid < HomotopyLevel::TwoGroupoid);
        assert!(HomotopyLevel::TwoGroupoid < HomotopyLevel::N(5));
        assert!(HomotopyLevel::Contractible.truncation_level() == -2);
        assert!(HomotopyLevel::Proposition.truncation_level() == -1);
        assert!(HomotopyLevel::Set.truncation_level() == 0);
    }
    #[test]
    fn test_universe_level_ops() {
        let u0 = UniverseLevel::zero();
        let u1 = u0.succ();
        let u2 = u1.succ();
        assert_eq!(u0.0, 0);
        assert_eq!(u1.0, 1);
        assert_eq!(u2.0, 2);
        assert_eq!(UniverseLevel::max(u1, u2), u2);
        assert_eq!(UniverseLevel::max(u2, u0), u2);
        assert_eq!(UniverseLevel::imax(u2, u0), u0);
        assert_eq!(UniverseLevel::imax(u0, u2), u2);
        assert_eq!(UniverseLevel::imax(u1, u2), u2);
    }
    #[test]
    fn test_mltt_term_app() {
        let f = MlttTerm::Var("f".to_string());
        let a = MlttTerm::Var("a".to_string());
        let t = MlttTerm::app(f, a);
        assert!(t.is_neutral());
        assert!(!t.is_type());
    }
    #[test]
    fn test_mltt_term_lam() {
        let body = MlttTerm::Var("x".to_string());
        let lam = MlttTerm::lam("x", body);
        assert!(!lam.is_neutral());
        assert!(!lam.is_type());
    }
    #[test]
    fn test_mltt_term_is_neutral() {
        assert!(MlttTerm::Var("x".to_string()).is_neutral());
        assert!(MlttTerm::App(
            Box::new(MlttTerm::Var("f".to_string())),
            Box::new(MlttTerm::Zero)
        )
        .is_neutral());
        assert!(MlttTerm::Fst(Box::new(MlttTerm::Var("p".to_string()))).is_neutral());
        assert!(MlttTerm::Snd(Box::new(MlttTerm::Var("p".to_string()))).is_neutral());
        assert!(!MlttTerm::Nat.is_neutral());
        assert!(!MlttTerm::Zero.is_neutral());
        assert!(!MlttTerm::Unit.is_neutral());
        assert!(!MlttTerm::Star.is_neutral());
    }
    #[test]
    fn test_mltt_term_beta_reduce() {
        let id_lam = MlttTerm::lam("x", MlttTerm::Var("x".to_string()));
        let redex = MlttTerm::App(Box::new(id_lam), Box::new(MlttTerm::Nat));
        let reduced = redex.beta_reduce();
        assert!(reduced.is_some());
        assert!(matches!(
            reduced.expect("reduced should be valid"),
            MlttTerm::Nat
        ));
        let pair = MlttTerm::Pair(
            Box::new(MlttTerm::Var("a".to_string())),
            Box::new(MlttTerm::Var("b".to_string())),
        );
        let fst_pair = MlttTerm::Fst(Box::new(pair));
        let fst_reduced = fst_pair.beta_reduce();
        assert!(matches!(fst_reduced, Some(MlttTerm::Var(n)) if n == "a"));
        assert!(MlttTerm::Var("x".to_string()).beta_reduce().is_none());
    }
    #[test]
    fn test_hott_path_refl() {
        let a = MlttTerm::Var("a".to_string());
        let p = HottPath::refl(a);
        assert!(p.is_refl());
    }
    #[test]
    fn test_hott_path_sym() {
        let a = MlttTerm::Var("a".to_string());
        let b = MlttTerm::Var("b".to_string());
        let path = HottPath {
            start: a.clone(),
            end: b.clone(),
            proof: MlttTerm::Var("p".to_string()),
        };
        let sym_path = path.sym();
        assert!(matches!(& sym_path.start, MlttTerm::Var(n) if n == "b"));
        assert!(matches!(& sym_path.end, MlttTerm::Var(n) if n == "a"));
        assert!(matches!(sym_path.proof, MlttTerm::J { .. }));
        assert!(!sym_path.is_refl());
    }
    #[test]
    fn test_stlc_type_display() {
        let t = STLCType::fun(
            STLCType::Base("A".to_string()),
            STLCType::Base("B".to_string()),
        );
        assert_eq!(t.to_string(), "(A → B)");
        let u = STLCType::prod(STLCType::Base("X".to_string()), STLCType::Unit);
        assert_eq!(u.to_string(), "(X × ⊤)");
    }
    #[test]
    fn test_stlc_typechecker_var() {
        let mut tc = STLCTypechecker::new();
        tc.extend("x".to_string(), STLCType::Base("Nat".to_string()));
        let ty = tc
            .synth(&STLCTerm::Var("x".to_string()))
            .expect("extend should succeed");
        assert_eq!(ty, STLCType::Base("Nat".to_string()));
    }
    #[test]
    fn test_stlc_typechecker_lam_app() {
        let mut tc = STLCTypechecker::new();
        let id_term = STLCTerm::Lam(
            "x".to_string(),
            STLCType::Base("Nat".to_string()),
            Box::new(STLCTerm::Var("x".to_string())),
        );
        let ty = tc.synth(&id_term).expect("synth should succeed");
        assert_eq!(
            ty,
            STLCType::fun(
                STLCType::Base("Nat".to_string()),
                STLCType::Base("Nat".to_string())
            )
        );
        tc.extend("n".to_string(), STLCType::Base("Nat".to_string()));
        let app_term = STLCTerm::App(Box::new(id_term), Box::new(STLCTerm::Var("n".to_string())));
        let app_ty = tc.synth(&app_term).expect("synth should succeed");
        assert_eq!(app_ty, STLCType::Base("Nat".to_string()));
    }
    #[test]
    fn test_stlc_typechecker_pair_proj() {
        let tc = STLCTypechecker::new();
        let pair = STLCTerm::Pair(Box::new(STLCTerm::Star), Box::new(STLCTerm::Star));
        let ty = tc.synth(&pair).expect("synth should succeed");
        assert_eq!(ty, STLCType::prod(STLCType::Unit, STLCType::Unit));
        let fst = STLCTerm::Fst(Box::new(pair.clone()));
        let fst_ty = tc.synth(&fst).expect("synth should succeed");
        assert_eq!(fst_ty, STLCType::Unit);
        let snd = STLCTerm::Snd(Box::new(pair));
        let snd_ty = tc.synth(&snd).expect("synth should succeed");
        assert_eq!(snd_ty, STLCType::Unit);
    }
    #[test]
    fn test_stlc_typechecker_unbound_err() {
        let tc = STLCTypechecker::new();
        let result = tc.synth(&STLCTerm::Var("z".to_string()));
        assert!(result.is_err());
    }
    #[test]
    fn test_system_f_type_instantiate() {
        let id_type = SystemFType::Forall(
            "alpha".to_string(),
            Box::new(SystemFType::Fun(
                Box::new(SystemFType::TyVar("alpha".to_string())),
                Box::new(SystemFType::TyVar("alpha".to_string())),
            )),
        );
        let mut eng = SystemFTypeInference::new();
        let inst = eng
            .inst(&id_type, &SystemFType::Base("Nat".to_string()))
            .expect("inst should succeed");
        assert_eq!(
            inst,
            SystemFType::Fun(
                Box::new(SystemFType::Base("Nat".to_string())),
                Box::new(SystemFType::Base("Nat".to_string()))
            )
        );
        let fresh = eng.fresh_tyvar();
        assert_eq!(fresh, "α0");
    }
    #[test]
    fn test_system_f_free_in() {
        let ty = SystemFType::Fun(
            Box::new(SystemFType::TyVar("α".to_string())),
            Box::new(SystemFType::Base("Nat".to_string())),
        );
        assert!(ty.free_in("α"));
        assert!(!ty.free_in("β"));
    }
    #[test]
    fn test_nbe_normalise_identity() {
        let id_lam = STLCTerm::Lam(
            "x".to_string(),
            STLCType::Unit,
            Box::new(STLCTerm::Var("x".to_string())),
        );
        let app = STLCTerm::App(Box::new(id_lam), Box::new(STLCTerm::Star));
        let nf = NormalizationByEvaluation::normalise(&app);
        assert!(matches!(nf, STLCTerm::Star));
    }
    #[test]
    fn test_nbe_normalise_pair_fst() {
        let pair = STLCTerm::Pair(Box::new(STLCTerm::Star), Box::new(STLCTerm::Star));
        let fst = STLCTerm::Fst(Box::new(pair));
        let nf = NormalizationByEvaluation::normalise(&fst);
        assert!(matches!(nf, STLCTerm::Star));
    }
    #[test]
    fn test_bidir_checker_type_universe() {
        let bc = BidirectionalTypechecker::new();
        let ty = bc
            .synth(&BidirType::Type(0))
            .expect("BidirectionalTypechecker::new should succeed");
        assert_eq!(ty, BidirType::Type(1));
    }
    #[test]
    fn test_bidir_checker_fun_type() {
        let bc = BidirectionalTypechecker::new();
        let fun_ty = BidirType::Fun(Box::new(BidirType::Type(0)), Box::new(BidirType::Type(0)));
        let kind = bc.synth(&fun_ty).expect("synth should succeed");
        assert_eq!(kind, BidirType::Type(0));
    }
    #[test]
    fn test_bidir_cumulativity() {
        let bc = BidirectionalTypechecker::new();
        assert!(bc.check(&BidirType::Type(0), &BidirType::Type(2)).is_ok());
        assert!(bc.check(&BidirType::Type(2), &BidirType::Type(0)).is_err());
    }
    #[test]
    fn test_universe_checker_consistent() {
        let mut uc = UniverseChecker::new();
        uc.declare("u".to_string());
        uc.declare("v".to_string());
        uc.add_constraint(UniverseConstraint::Le(
            UniverseExpr::Var("u".to_string()),
            UniverseExpr::Var("v".to_string()),
        ));
        assert!(uc.is_consistent(3));
        assert_eq!(uc.num_vars(), 2);
        assert_eq!(uc.num_constraints(), 1);
    }
    #[test]
    fn test_universe_checker_inconsistent() {
        let mut uc = UniverseChecker::new();
        uc.declare("u".to_string());
        uc.add_constraint(UniverseConstraint::Lt(
            UniverseExpr::Var("u".to_string()),
            UniverseExpr::Var("u".to_string()),
        ));
        assert!(!uc.is_consistent(5));
    }
    #[test]
    fn test_universe_expr_eval() {
        let mut assign = std::collections::HashMap::new();
        assign.insert("u".to_string(), 2u32);
        assign.insert("v".to_string(), 3u32);
        let e = UniverseExpr::Max(
            Box::new(UniverseExpr::Var("u".to_string())),
            Box::new(UniverseExpr::Succ(Box::new(UniverseExpr::Var(
                "v".to_string(),
            )))),
        );
        assert_eq!(e.eval(&assign), Some(4));
        let ie = UniverseExpr::IMax(
            Box::new(UniverseExpr::Lit(5)),
            Box::new(UniverseExpr::Lit(0)),
        );
        assert_eq!(ie.eval(&assign), Some(0));
    }
    #[test]
    fn test_build_type_theory_env_no_panic() {
        let mut env = Environment::new();
        build_type_theory_env(&mut env);
    }
}
