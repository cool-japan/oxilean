//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{BoundedRange, FixpointIterator, GaloisPair, MonotoneChain, OrderedHeap};

#[cfg(test)]
mod ord_extended_tests {
    use super::*;
    #[test]
    fn test_register_ord_extended_succeeds() {
        let mut env = Environment::new();
        build_ord_env(&mut env).expect("build_ord_env should succeed");
        assert!(register_ord_extended(&mut env).is_ok());
    }
    #[test]
    fn test_register_ord_extended_axioms_present() {
        let mut env = Environment::new();
        build_ord_env(&mut env).expect("build_ord_env should succeed");
        register_ord_extended(&mut env).expect("unwrap should succeed");
        assert!(env.get(&Name::str("OrdCat")).is_some());
        assert!(env.get(&Name::str("DCPO")).is_some());
        assert!(env.get(&Name::str("OmegaCPO")).is_some());
        assert!(env.get(&Name::str("CompleteLattice")).is_some());
        assert!(env.get(&Name::str("DilworthTheorem")).is_some());
        assert!(env.get(&Name::str("NatOrd")).is_some());
    }
    #[test]
    fn test_monotone_chain_push() {
        let mut chain: MonotoneChain<u32> = MonotoneChain::new();
        assert!(chain.push(1));
        assert!(chain.push(3));
        assert!(!chain.push(2));
        assert!(chain.push(5));
        assert_eq!(chain.len(), 3);
    }
    #[test]
    fn test_monotone_chain_lis() {
        let v = vec![3u32, 1, 4, 1, 5, 9, 2, 6];
        let lis = MonotoneChain::lis_from_slice(&v);
        assert!(lis.len() >= 4);
    }
    #[test]
    fn test_fixpoint_iterator_identity() {
        let mut fp: FixpointIterator<u32> = FixpointIterator::new(0, 100);
        let result = fp.compute(&|x| *x);
        assert_eq!(result, Some(0));
    }
    #[test]
    fn test_fixpoint_iterator_converges() {
        let mut fp: FixpointIterator<u32> = FixpointIterator::new(0, 20);
        let result = fp.compute(&|x| if *x < 10 { *x + 1 } else { *x });
        assert_eq!(result, Some(10));
    }
    #[test]
    fn test_ordered_heap_push_pop() {
        let mut heap: OrderedHeap<u32> = OrderedHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        heap.push(1);
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(3));
    }
    #[test]
    fn test_ordered_heap_from_vec() {
        let v = vec![5u32, 3, 7, 1, 9];
        let mut heap = OrderedHeap::from_vec(v);
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(7));
    }
    #[test]
    fn test_bounded_range_contains() {
        let r = BoundedRange::new(1u32, 10u32);
        assert!(r.contains(&5));
        assert!(r.contains(&1));
        assert!(r.contains(&10));
        assert!(!r.contains(&0));
        assert!(!r.contains(&11));
    }
    #[test]
    fn test_bounded_range_intersect() {
        let r1 = BoundedRange::new(1u32, 5u32);
        let r2 = BoundedRange::new(3u32, 8u32);
        let inter = r1.intersect(&r2).expect("intersect should succeed");
        assert_eq!(*inter.lo(), 3);
        assert_eq!(*inter.hi(), 5);
    }
    #[test]
    fn test_bounded_range_disjoint() {
        let r1 = BoundedRange::new(1u32, 3u32);
        let r2 = BoundedRange::new(5u32, 8u32);
        assert!(r1.intersect(&r2).is_none());
    }
    #[test]
    fn test_bounded_range_clamp() {
        let r = BoundedRange::new(1u32, 10u32);
        assert_eq!(r.clamp(0), 1);
        assert_eq!(r.clamp(5), 5);
        assert_eq!(r.clamp(15), 10);
    }
    #[test]
    fn test_galois_pair_closure() {
        let pair = GaloisPair::new(|x: &i32| *x, |x: &i32| *x);
        assert_eq!(pair.closure(&5), 5);
    }
    #[test]
    fn test_galois_pair_kernel() {
        let pair = GaloisPair::new(|x: &i32| *x, |x: &i32| *x);
        assert_eq!(pair.kernel(&5), 5);
    }
    #[test]
    fn test_galois_pair_condition() {
        let pair = GaloisPair::new(|x: &i32| *x, |x: &i32| *x);
        // Identity/identity is a valid Galois pair; condition holds for all a, b
        assert!(pair.check_galois_condition(&3, &5));
        assert!(pair.check_galois_condition(&5, &3));
    }
    #[test]
    fn test_axiom_type_builders_return_expr() {
        let _ = axiom_ord_cat_ty();
        let _ = axiom_galois_connection_ty();
        let _ = axiom_scott_continuous_ty();
        let _ = axiom_knaster_tarski_ty();
        let _ = axiom_compare_eq_iff_ty();
        let _ = axiom_compare_swap_ty();
        let _ = axiom_prod_ord_ty();
        let _ = axiom_list_ord_ty();
        let _ = axiom_option_ord_ty();
        let _ = axiom_dilworth_theorem_ty();
    }
}
