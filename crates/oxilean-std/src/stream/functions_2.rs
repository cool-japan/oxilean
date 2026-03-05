//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BloomFilterRs, CountMinSketchRs, LazyStream, MealyMachineRs, MooreMachineRs, PriorityMerge,
    StreamWindow,
};

/// `Stream.zipWith`: pointwise binary operation on two streams.
/// Type: {α β γ : Type} → (α → β → γ) → Stream α → Stream β → Stream γ
pub fn strm_ext_zip_with_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(2)),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("s1"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("s2"),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    )
}
/// `Stream.ap`: applicative apply for streams.
/// Type: {α β : Type} → Stream (α → β) → Stream α → Stream β
pub fn strm_ext_ap_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("sf"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::BVar(1)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("sa"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    )
}
/// `Stream.coinductive_eq`: coinductive equality of streams.
/// Type: {α : Type} → Stream α → Stream α → Prop
pub fn strm_ext_coinductive_eq_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.map_head`: map preserves head.
/// Type: {α β : Type} → (f : α → β) → (s : Stream α) → Prop
pub fn strm_ext_map_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Stream.map_tail`: map commutes with tail.
/// Type: {α β : Type} → (f : α → β) → (s : Stream α) → Prop
pub fn strm_ext_map_tail_ty() -> Expr {
    strm_ext_map_head_ty()
}
/// `Stream.nth_tail`: nth element of tail is (n+1)-th element of original.
/// Type: {α : Type} → (n : Nat) → (s : Stream α) → Prop
pub fn strm_ext_nth_tail_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.take_drop`: take n then drop n is identity.
/// Type: {α : Type} → (n : Nat) → (s : Stream α) → Prop
pub fn strm_ext_take_drop_ty() -> Expr {
    strm_ext_nth_tail_ty()
}
/// `Stream.zip_head`: zip of two streams has paired heads.
/// Type: {α β : Type} → (s : Stream α) → (t : Stream β) → Prop
pub fn strm_ext_zip_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("t"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Stream.scan_head`: scan produces head equal to initial accumulator.
/// Type: {α β : Type} → (f : β → α → β) → (b : β) → (s : Stream α) → Prop
pub fn strm_ext_scan_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("s"),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(prop),
                    )),
                )),
            )),
        )),
    )
}
/// `Stream.interleave_head`: interleave alternates heads correctly.
/// Type: {α : Type} → (s t : Stream α) → Prop
pub fn strm_ext_interleave_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.iterate_head`: iterate starts with init.
/// Type: {α : Type} → (a : α) → (f : α → α) → Prop
pub fn strm_ext_iterate_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(prop),
            )),
        )),
    )
}
/// `Stream.const_head`: constant stream has given element as head.
/// Type: {α : Type} → (a : α) → Prop
pub fn strm_ext_const_head_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(prop),
        )),
    )
}
/// `Stream.drop_add`: drop (m + n) = drop m ∘ drop n.
/// Type: {α : Type} → (m n : Nat) → (s : Stream α) → Prop
pub fn strm_ext_drop_add_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Stream.corec_unique`: uniqueness of corecursive streams.
/// Type: {α σ : Type} → ... → Prop
pub fn strm_ext_corec_unique_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("σ"),
            Box::new(type1),
            Box::new(prop),
        )),
    )
}
/// `Stream.final_coalgebra`: Stream α is the final coalgebra of functor (α × –).
/// Type: {α : Type} → Prop
pub fn strm_ext_final_coalgebra_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(prop),
    )
}
/// `Stream.bisim_is_eq`: bisimilarity implies equality.
/// Type: {α : Type} → (s t : Stream α) → Stream.Bisim s t → s = t
pub fn strm_ext_bisim_is_eq_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("s"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Stream"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Stream"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("bisim_pf"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Stream.Bisim"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Stream.map_corec`: map commutes with corecursion.
/// Type: {α β σ : Type} → (f : α → β) → ... → Prop
pub fn strm_ext_map_corec_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("σ"),
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    )
}
/// Register extended stream axioms in the environment.
///
/// Covers: bisimulation, Mealy/Moore machines, KPN, FRP, stream differential
/// equations, productivity, guarded fixpoints, fusion laws, automata,
/// weighted automata, Bloom filter, count-min sketch, circular programming,
/// zipWith, ap, coinductive equality, and various stream lemmas.
pub fn register_stream_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("Stream.Bisim", strm_ext_bisim_ty),
        ("Stream.bisim_coind", strm_ext_bisim_coind_ty),
        ("Stream.corec", strm_ext_corec_ty),
        ("Stream.guarded_fix", strm_ext_guarded_fix_ty),
        ("Stream.coinductive_eq", strm_ext_coinductive_eq_ty),
        ("Stream.map_head", strm_ext_map_head_ty),
        ("Stream.map_tail", strm_ext_map_tail_ty),
        ("Stream.nth_tail", strm_ext_nth_tail_ty),
        ("Stream.take_drop", strm_ext_take_drop_ty),
        ("Stream.zip_head", strm_ext_zip_head_ty),
        ("Stream.scan_head", strm_ext_scan_head_ty),
        ("Stream.interleave_head", strm_ext_interleave_head_ty),
        ("Stream.iterate_head", strm_ext_iterate_head_ty),
        ("Stream.const_head", strm_ext_const_head_ty),
        ("Stream.drop_add", strm_ext_drop_add_ty),
        ("Stream.corec_unique", strm_ext_corec_unique_ty),
        ("Stream.final_coalgebra", strm_ext_final_coalgebra_ty),
        ("Stream.bisim_is_eq", strm_ext_bisim_is_eq_ty),
        ("Stream.map_corec", strm_ext_map_corec_ty),
        ("Stream.fusion_law", strm_ext_fusion_law_ty),
        ("Stream.BöhmTree", strm_ext_bohm_tree_ty),
        ("Stream.corecursion_unique", strm_ext_corecursion_unique_ty),
        ("Stream.automaton", strm_ext_automaton_ty),
        ("Stream.zipWith", strm_ext_zip_with_ty),
        ("Stream.ap", strm_ext_ap_ty),
        ("Stream.productivity", strm_ext_productivity_ty),
        ("Stream.diff_eq", strm_ext_diff_eq_ty),
        ("MealyMachine", strm_ext_mealy_machine_ty),
        ("MealyMachine.run", strm_ext_mealy_run_ty),
        ("MooreMachine", strm_ext_moore_machine_ty),
        ("KPN.channel", strm_ext_kpn_channel_ty),
        ("KPN.process", strm_ext_kpn_process_ty),
        ("FRP.Behavior", strm_ext_frp_behavior_ty),
        ("FRP.Event", strm_ext_frp_event_ty),
        ("FRP.stepper", strm_ext_frp_stepper_ty),
        ("WeightedAutomaton", strm_ext_weighted_automaton_ty),
        ("BloomFilter", strm_ext_bloom_filter_ty),
        ("CountMinSketch", strm_ext_count_min_sketch_ty),
        ("Stream.circular_prog", strm_ext_circular_prog_ty),
    ];
    for (name, ty_fn) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
#[cfg(test)]
mod stream_extended_tests {
    use super::*;
    fn full_ext_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        for name in &["Nat", "Bool", "Int"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1.clone(),
            })
            .expect("operation should succeed");
        }
        for (nm, arity) in &[("List", 1usize), ("Option", 1)] {
            let ty = Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(type2.clone()),
            );
            let _ = arity;
            env.add(Declaration::Axiom {
                name: Name::str(*nm),
                univ_params: vec![],
                ty,
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(type2.clone()),
                )),
            ),
        })
        .expect("operation should succeed");
        build_stream_env(&mut env).expect("build_stream_env should succeed");
        env
    }
    #[test]
    fn test_register_stream_extended_ok() {
        let mut env = full_ext_env();
        assert!(register_stream_extended(&mut env).is_ok());
    }
    #[test]
    fn test_bisim_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Stream.Bisim")).is_some());
    }
    #[test]
    fn test_corec_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Stream.corec")).is_some());
    }
    #[test]
    fn test_mealy_machine_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("MealyMachine")).is_some());
        assert!(env.get(&Name::str("MealyMachine.run")).is_some());
    }
    #[test]
    fn test_moore_machine_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("MooreMachine")).is_some());
    }
    #[test]
    fn test_kpn_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("KPN.channel")).is_some());
        assert!(env.get(&Name::str("KPN.process")).is_some());
    }
    #[test]
    fn test_frp_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("FRP.Behavior")).is_some());
        assert!(env.get(&Name::str("FRP.Event")).is_some());
    }
    #[test]
    fn test_bloom_filter_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("BloomFilter")).is_some());
    }
    #[test]
    fn test_count_min_sketch_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("CountMinSketch")).is_some());
    }
    #[test]
    fn test_zip_with_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Stream.zipWith")).is_some());
    }
    #[test]
    fn test_fusion_law_registered() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        assert!(env.get(&Name::str("Stream.fusion_law")).is_some());
    }
    #[test]
    fn test_mealy_machine_rs_step() {
        let mut m = MealyMachineRs::new(0u64, |s, i: u64| (s + i, s + i));
        assert_eq!(m.step(3), 3);
        assert_eq!(m.step(7), 10);
        assert_eq!(m.step(5), 15);
    }
    #[test]
    fn test_mealy_machine_rs_run_vec() {
        let mut m = MealyMachineRs::new(1u64, |s, i: u64| (s * i, s * i));
        let outputs = m.run_vec(vec![2, 3, 4]);
        assert_eq!(outputs, vec![2, 6, 24]);
    }
    #[test]
    fn test_moore_machine_rs_step() {
        let mut m = MooreMachineRs::new(0u64, |_s, i: u64| i, |s| s * 2);
        assert_eq!(m.read_output(), 0);
        m.step(5);
        assert_eq!(m.read_output(), 10);
        m.step(3);
        assert_eq!(m.read_output(), 6);
    }
    #[test]
    fn test_bloom_filter_rs_insert_query() {
        let mut bf = BloomFilterRs::new(64, 3);
        bf.insert(42);
        bf.insert(100);
        assert!(bf.query(42));
        assert!(bf.query(100));
    }
    #[test]
    fn test_bloom_filter_rs_capacity() {
        let bf = BloomFilterRs::new(128, 4);
        assert_eq!(bf.capacity(), 128);
        assert_eq!(bf.count_set(), 0);
    }
    #[test]
    fn test_count_min_sketch_rs_estimate() {
        let mut cms = CountMinSketchRs::new(4, 16);
        cms.update(7);
        cms.update(7);
        cms.update(7);
        cms.update(42);
        assert!(cms.estimate(7) >= 3);
        assert!(cms.estimate(42) >= 1);
    }
    #[test]
    fn test_count_min_sketch_total_updates() {
        let mut cms = CountMinSketchRs::new(3, 8);
        cms.update(1);
        cms.update(2);
        cms.update(3);
        assert!(cms.total_updates() >= 3);
    }
    #[test]
    fn test_stream_window_push_and_read() {
        let mut w: StreamWindow<u32> = StreamWindow::new(3);
        assert!(w.is_empty());
        w.push(1);
        w.push(2);
        w.push(3);
        assert_eq!(w.len(), 3);
        let win = w.window();
        assert_eq!(win, vec![1, 2, 3]);
    }
    #[test]
    fn test_stream_window_sliding() {
        let mut w: StreamWindow<u32> = StreamWindow::new(3);
        w.push(1);
        w.push(2);
        w.push(3);
        w.push(4);
        let win = w.window();
        assert_eq!(win.len(), 3);
        assert_eq!(win, vec![2, 3, 4]);
    }
    #[test]
    fn test_priority_merge_left_wins() {
        let left = LazyStream::iterate(0u32, |x| x + 1);
        let right = LazyStream::constant(99u32);
        let mut merge = PriorityMerge::new(left, right);
        let vals = merge.take_n(4);
        assert_eq!(vals, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_strm_ext_bisim_ty_is_pi() {
        let ty = strm_ext_bisim_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_strm_ext_corec_ty_is_pi() {
        let ty = strm_ext_corec_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_strm_ext_automaton_ty_is_pi() {
        let ty = strm_ext_automaton_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_all_extended_axioms_count() {
        let mut env = full_ext_env();
        register_stream_extended(&mut env).expect("operation should succeed");
        let extended_names = [
            "Stream.Bisim",
            "Stream.bisim_coind",
            "Stream.corec",
            "Stream.guarded_fix",
            "Stream.coinductive_eq",
            "Stream.fusion_law",
            "Stream.automaton",
            "Stream.zipWith",
            "Stream.ap",
            "MealyMachine",
            "MooreMachine",
            "KPN.channel",
            "FRP.Behavior",
            "BloomFilter",
            "CountMinSketch",
        ];
        for name in &extended_names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "{} should be registered",
                name
            );
        }
    }
}
