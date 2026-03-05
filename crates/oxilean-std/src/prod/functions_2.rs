//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Level};
use oxilean_kernel::{Expr, Name};

use super::types::{AssocTriple, CurriedFn, LexPair, ProdBimap, ProdCone};

#[cfg(test)]
mod prod_extended_tests {
    use super::*;
    use oxilean_kernel::Environment;
    fn setup() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_register_prod_extended_axioms_succeeds() {
        let mut env = setup();
        assert!(register_prod_extended_axioms(&mut env).is_ok());
    }
    #[test]
    fn test_universal_property_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.universal_property")).is_some());
    }
    #[test]
    fn test_projections_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.proj_fst_law")).is_some());
        assert!(env.get(&Name::str("Prod.proj_snd_law")).is_some());
        assert!(env.get(&Name::str("Prod.eta_law")).is_some());
    }
    #[test]
    fn test_functor_laws_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.functor_map_fst")).is_some());
        assert!(env.get(&Name::str("Prod.functor_map_snd")).is_some());
        assert!(env.get(&Name::str("Prod.bifunctor_law")).is_some());
        assert!(env.get(&Name::str("Prod.bifunctor_id")).is_some());
    }
    #[test]
    fn test_comonad_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.comonad_extract")).is_some());
        assert!(env.get(&Name::str("Prod.comonad_extend")).is_some());
    }
    #[test]
    fn test_monoidal_structure_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.tensor_unit_left")).is_some());
        assert!(env.get(&Name::str("Prod.tensor_unit_right")).is_some());
        assert!(env.get(&Name::str("Prod.tensor_assoc")).is_some());
    }
    #[test]
    fn test_commutativity_iso_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.commutativity_iso")).is_some());
        assert!(env.get(&Name::str("Prod.swap_involution")).is_some());
    }
    #[test]
    fn test_assoc_iso_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.tensor_assoc")).is_some());
        assert!(env.get(&Name::str("Prod.assoc_left")).is_some());
    }
    #[test]
    fn test_unit_terminal_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.unit_terminal")).is_some());
        assert!(env.get(&Name::str("Prod.unit_terminal_unique")).is_some());
    }
    #[test]
    fn test_curry_uncurry_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.curry_law")).is_some());
        assert!(env.get(&Name::str("Prod.uncurry_law")).is_some());
        assert!(env.get(&Name::str("Prod.hom_tensor_adj")).is_some());
    }
    #[test]
    fn test_distrib_over_sum_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.distrib_over_sum")).is_some());
        assert!(env.get(&Name::str("Prod.distrib_over_sum_inv")).is_some());
    }
    #[test]
    fn test_sigma_laws_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.sigma_fst")).is_some());
        assert!(env.get(&Name::str("Prod.sigma_snd")).is_some());
        assert!(env.get(&Name::str("Prod.sigma_eta")).is_some());
    }
    #[test]
    fn test_record_nary_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.record_as_prod")).is_some());
        assert!(env.get(&Name::str("Prod.nary_tuple_3")).is_some());
    }
    #[test]
    fn test_pointwise_fn_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.pointwise_fn_prod")).is_some());
        assert!(env.get(&Name::str("Prod.pointwise_fn_prod_inv")).is_some());
    }
    #[test]
    fn test_group_ring_direct_product_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.group_direct_product")).is_some());
        assert!(env.get(&Name::str("Prod.ring_direct_product")).is_some());
    }
    #[test]
    fn test_fanout_dup_present() {
        let mut env = setup();
        register_prod_extended_axioms(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Prod.fanout_law")).is_some());
        assert!(env.get(&Name::str("Prod.dup_law")).is_some());
        assert!(env.get(&Name::str("Prod.fst_dup")).is_some());
        assert!(env.get(&Name::str("Prod.snd_dup")).is_some());
    }
    #[test]
    fn test_prod_cone_struct() {
        let cone: ProdCone<u32, u32, u32> = ProdCone::new(|x| x + 1, |x| x * 2);
        let (a, b) = cone.mediate(5u32);
        assert_eq!(a, 6);
        assert_eq!(b, 10);
    }
    #[test]
    fn test_prod_bimap_struct() {
        let bm: ProdBimap<u32, u32, u32, u32> = ProdBimap::new(|x| x + 10, |y| y * 3);
        let (c, d) = bm.apply((1u32, 2u32));
        assert_eq!(c, 11);
        assert_eq!(d, 6);
    }
    #[test]
    fn test_assoc_triple_struct() {
        let at = AssocTriple::new(((1u32, 2u32), 3u32));
        let (a, (b, c)) = at.assoc_right();
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }
    #[test]
    fn test_assoc_triple_roundtrip() {
        let original = ((10u32, 20u32), 30u32);
        let at = AssocTriple::new(original.clone());
        let right = at.assoc_right();
        let back = AssocTriple::from_right(right);
        assert_eq!(back.value, original);
    }
    #[test]
    fn test_curried_fn_struct() {
        let cf: CurriedFn<u32, u32, u32> = CurriedFn::new(|a, b| a + b);
        assert_eq!(cf.apply(3, 4), 7);
        assert_eq!(cf.uncurried((3, 4)), 7);
    }
    #[test]
    fn test_lex_pair_ordering() {
        let p1 = LexPair::new(1u32, 5u32);
        let p2 = LexPair::new(1u32, 3u32);
        let p3 = LexPair::new(2u32, 0u32);
        assert!(p2 < p1);
        assert!(p1 < p3);
    }
}
