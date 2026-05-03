//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Antichain, BitVectorBooleanAlgebra, Chain, ClosureOperatorFinite, CompleteLatticeFinite,
    FiniteLattice, FormalContext, GaloisConnection, GaloisConnectionFinite,
    HeytingAlgebraFiniteTop, MVAlgebra, ResidLattice,
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// Partial order type.
///
/// PartialOrder A : (A → A → Prop) → Prop
/// The proposition that a relation on A is a partial order.
pub fn partial_order_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), prop()),
    )
}
/// Lattice type: a partial order with binary join (∨) and meet (∧).
///
/// Lattice A leq : (A → A → A) → (A → A → A) → Prop
pub fn lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(
                arrow(bvar(1), arrow(bvar(2), bvar(3))),
                arrow(arrow(bvar(2), arrow(bvar(3), bvar(4))), prop()),
            ),
        ),
    )
}
/// Distributive lattice: x ∧ (y ∨ z) = (x ∧ y) ∨ (x ∧ z).
///
/// DistributiveLattice A : Lattice A leq meet join → Prop
pub fn distributive_lattice_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("Lattice"), bvar(0)), prop()))
}
/// Boolean algebra: complemented distributive lattice.
///
/// BooleanAlgebra A : DistributiveLattice A L → (A → A) → Prop
pub fn boolean_algebra_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("DistributiveLattice"), bvar(0)),
            arrow(arrow(bvar(1), bvar(2)), prop()),
        ),
    )
}
/// Complete lattice: has supremum and infimum for arbitrary subsets.
///
/// CompleteLattice A : (Set A → A) → (Set A → A) → Prop
pub fn complete_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(app(cst("Set"), bvar(0)), bvar(1)),
            arrow(arrow(app(cst("Set"), bvar(1)), bvar(2)), prop()),
        ),
    )
}
/// Galois connection: (f, g) : A → B with f ⊣ g.
///
/// GaloisConnection f g : (A → B) → (B → A) → Prop
/// f(a) ≤ b ↔ a ≤ g(b)
pub fn galois_connection_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(arrow(bvar(1), bvar(2)), prop()),
            ),
        ),
    )
}
/// Knaster-Tarski Fixed Point Theorem.
///
/// Every monotone function on a complete lattice has a fixed point.
/// Moreover, the set of fixed points forms a complete lattice.
pub fn knaster_tarski_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("CompleteLattice"), bvar(0)),
            arrow(
                arrow(bvar(1), bvar(2)),
                arrow(
                    app(cst("Monotone"), bvar(0)),
                    app2(cst("HasFixedPoint"), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}
/// Birkhoff Representation Theorem.
///
/// Every finite distributive lattice is isomorphic to the lattice of
/// downsets of its join-irreducible elements.
pub fn birkhoff_representation_ty() -> Expr {
    impl_pi(
        "L",
        type0(),
        arrow(
            app(cst("FiniteDistributiveLattice"), bvar(0)),
            app2(
                cst("LatticeIso"),
                bvar(1),
                app(cst("Downsets"), app(cst("JoinIrreducibles"), bvar(2))),
            ),
        ),
    )
}
/// Stone Duality Theorem.
///
/// The category of Boolean algebras is dually equivalent to the category
/// of Stone spaces (compact, totally disconnected Hausdorff spaces).
pub fn stone_duality_ty() -> Expr {
    arrow(
        app(cst("BooleanAlgebra"), cst("B")),
        app2(cst("CategoryDualEquiv"), cst("BoolAlg"), cst("StoneSpace")),
    )
}
/// Dilworth's Theorem.
///
/// In any finite partially ordered set, the minimum number of chains
/// needed to partition the set equals the maximum size of an antichain.
pub fn dilworth_theorem_ty() -> Expr {
    impl_pi(
        "P",
        type0(),
        arrow(
            app(cst("FinitePartialOrder"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("MinChainCover"), bvar(1)),
                app(cst("MaxAntichain"), bvar(2)),
            ),
        ),
    )
}
/// Closure operator: an extensive, idempotent, monotone endofunction on a poset.
///
/// ClosureOperator A : (A → A) → Prop
pub fn closure_operator_ty() -> Expr {
    impl_pi("A", type0(), arrow(arrow(bvar(0), bvar(1)), prop()))
}
/// Heyting algebra: a bounded distributive lattice with implication.
///
/// HeytingAlgebra A : DistributiveLattice A → (A → A → A) → Prop
pub fn heyting_algebra_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("DistributiveLattice"), bvar(0)),
            arrow(arrow(bvar(1), arrow(bvar(2), bvar(3))), prop()),
        ),
    )
}
/// Residuated lattice: lattice with multiplication and residuals.
///
/// ResiduatedLattice A : Lattice A leq → (A → A → A) → Prop
pub fn residuated_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(arrow(bvar(1), arrow(bvar(2), bvar(3))), prop()),
        ),
    )
}
/// Frame: a complete lattice satisfying the infinite distributive law.
///
/// Frame A : CompleteLattice A → Prop
pub fn frame_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("CompleteLattice"), bvar(0)), prop()),
    )
}
/// Locale: the pointfree topology dual to a topological space.
///
/// Locale A : Frame A → Prop
pub fn locale_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("Frame"), bvar(0)), prop()))
}
/// Quantale: a complete lattice with an associative binary operation
/// distributing over arbitrary joins.
///
/// Quantale A : CompleteLattice A → (A → A → A) → Prop
pub fn quantale_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("CompleteLattice"), bvar(0)),
            arrow(arrow(bvar(1), arrow(bvar(2), bvar(3))), prop()),
        ),
    )
}
/// Directed-complete partial order (dcpo): every directed subset has a supremum.
///
/// Dcpo A : PartialOrder A leq → Prop
pub fn dcpo_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("PartialOrder"), bvar(0)), prop()),
    )
}
/// Scott topology on a dcpo: open sets are upper sets closed under directed sups.
///
/// ScottTopology A : Dcpo A → (Set (Set A) → Prop) → Prop
pub fn scott_topology_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Dcpo"), bvar(0)),
            arrow(
                arrow(app(cst("Set"), app(cst("Set"), bvar(1))), prop()),
                prop(),
            ),
        ),
    )
}
/// MV-algebra (Lukasiewicz logic): a bounded commutative monoid with negation.
///
/// MVAlgebra A : BooleanAlgebra A → (A → A → A) → Prop
pub fn mv_algebra_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("BooleanAlgebra"), bvar(0)),
            arrow(arrow(bvar(1), arrow(bvar(2), bvar(3))), prop()),
        ),
    )
}
/// Effect algebra: partial commutative monoid for quantum events.
///
/// EffectAlgebra A : (A → A → Option A) → (A → A) → Prop
pub fn effect_algebra_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), app(cst("Option"), bvar(2)))),
            arrow(arrow(bvar(2), bvar(3)), prop()),
        ),
    )
}
/// Orthomodular lattice: a lattice with orthocomplementation satisfying the
/// orthomodular law (quantum logic).
///
/// OrthomodularLattice A : Lattice A → (A → A) → Prop
pub fn orthomodular_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(arrow(bvar(1), bvar(2)), prop()),
        ),
    )
}
/// Semimodular lattice: covers are preserved by join with any element.
///
/// SemimodularLattice A : Lattice A → Prop
pub fn semimodular_lattice_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("Lattice"), bvar(0)), prop()))
}
/// Geometric lattice: semimodular lattice in which every element is a join
/// of atoms (corresponds to a matroid).
///
/// GeometricLattice A : SemimodularLattice A → Prop
pub fn geometric_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("SemimodularLattice"), bvar(0)), prop()),
    )
}
/// Lattice-ordered group (l-group): a group that is also a lattice with
/// the group operations compatible with the order.
///
/// LatticeOrderedGroup G : Lattice G → Group G → Prop
pub fn lattice_ordered_group_ty() -> Expr {
    impl_pi(
        "G",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(app(cst("Group"), bvar(1)), prop()),
        ),
    )
}
/// Congruence lattice: the lattice of congruence relations on an algebra.
///
/// CongruenceLattice A : Algebra A → CompleteLattice (Con A) → Prop
pub fn congruence_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Algebra"), bvar(0)),
            arrow(
                app(cst("CompleteLattice"), app(cst("Con"), bvar(1))),
                prop(),
            ),
        ),
    )
}
/// Free lattice generated by a set: satisfies the universal property.
///
/// FreeLattice X : (X → Lattice) → Prop
pub fn free_lattice_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        arrow(arrow(bvar(0), app(cst("Lattice"), bvar(1))), prop()),
    )
}
/// Fuzzy lattice: a lattice equipped with a truth-value in \[0,1\].
///
/// FuzzyLattice A : Lattice A → (A → Real) → Prop
pub fn fuzzy_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(arrow(bvar(1), cst("Real")), prop()),
        ),
    )
}
/// Birkhoff subdirect representation: every lattice embeds subdirectly
/// into subdirectly irreducible lattices.
///
/// BirkhoffSubdirect L : Lattice L → Prop
pub fn birkhoff_subdirect_ty() -> Expr {
    impl_pi("L", type0(), arrow(app(cst("Lattice"), bvar(0)), prop()))
}
/// Dedekind-MacNeille completion: the smallest complete lattice
/// containing a given poset.
///
/// DedekindMacNeille P : PartialOrder P → CompleteLattice (DM P) → Prop
pub fn dedekind_macneille_ty() -> Expr {
    impl_pi(
        "P",
        type0(),
        arrow(
            app(cst("PartialOrder"), bvar(0)),
            arrow(app(cst("CompleteLattice"), app(cst("DM"), bvar(1))), prop()),
        ),
    )
}
/// Infinite meet axiom for complete lattices: the infimum of any subset exists.
///
/// InfExists A : CompleteLattice A → forall S : Set A, exists inf : A, IsInf S inf → Prop
pub fn inf_exists_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("CompleteLattice"), bvar(0)),
            arrow(
                app(cst("Set"), bvar(1)),
                app2(cst("Exists"), app(cst("Set"), bvar(2)), cst("IsInf")),
            ),
        ),
    )
}
/// Infinite join axiom for complete lattices: the supremum of any subset exists.
///
/// SupExists A : CompleteLattice A → forall S : Set A, exists sup : A, IsSup S sup → Prop
pub fn sup_exists_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("CompleteLattice"), bvar(0)),
            arrow(
                app(cst("Set"), bvar(1)),
                app2(cst("Exists"), app(cst("Set"), bvar(2)), cst("IsSup")),
            ),
        ),
    )
}
/// Scott continuity: a function between dcpos that preserves directed sups.
///
/// ScottContinuous A B : Dcpo A → Dcpo B → (A → B) → Prop
pub fn scott_continuous_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app(cst("Dcpo"), bvar(1)),
                arrow(
                    app(cst("Dcpo"), bvar(1)),
                    arrow(arrow(bvar(2), bvar(2)), prop()),
                ),
            ),
        ),
    )
}
/// Ascending chain condition (ACC): every ascending chain stabilizes.
///
/// ACC A : PartialOrder A → Prop
pub fn acc_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("PartialOrder"), bvar(0)), prop()),
    )
}
/// Descending chain condition (DCC): every descending chain stabilizes.
///
/// DCC A : PartialOrder A → Prop
pub fn dcc_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("PartialOrder"), bvar(0)), prop()),
    )
}
/// Lattice congruence: a congruence relation compatible with meet and join.
///
/// LatticeCongruence A : Lattice A → (A → A → Prop) → Prop
pub fn lattice_congruence_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(arrow(bvar(1), arrow(bvar(2), prop())), prop()),
        ),
    )
}
/// Stone representation theorem for distributive lattices:
/// every distributive lattice embeds into a powerset Boolean algebra.
///
/// StoneRepresentation L : DistributiveLattice L → Prop
pub fn stone_representation_ty() -> Expr {
    impl_pi(
        "L",
        type0(),
        arrow(app(cst("DistributiveLattice"), bvar(0)), prop()),
    )
}
/// Galois closure: the composition g ∘ f is a closure operator.
///
/// GaloisClosure A B : GaloisConnection A B f g → ClosureOperator A (g ∘ f) → Prop
pub fn galois_closure_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("GaloisConnection"), bvar(1), bvar(0)),
                arrow(app(cst("ClosureOperator"), bvar(2)), prop()),
            ),
        ),
    )
}
/// Modular lattice law: a ≤ c → a ∨ (b ∧ c) = (a ∨ b) ∧ c.
///
/// ModularLattice A : Lattice A → Prop
pub fn modular_lattice_ty() -> Expr {
    impl_pi("A", type0(), arrow(app(cst("Lattice"), bvar(0)), prop()))
}
/// Rank function on a geometric lattice (from the matroid).
///
/// RankFunction A : GeometricLattice A → (A → Nat) → Prop
pub fn rank_function_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("GeometricLattice"), bvar(0)),
            arrow(arrow(bvar(1), cst("Nat")), prop()),
        ),
    )
}
/// Build an OxiLean environment containing the lattice theory axioms.
pub fn build_lattice_theory_env() -> Environment {
    let mut env = Environment::new();
    let _ = env.add(Declaration::Axiom {
        name: Name::str("PartialOrder"),
        univ_params: vec![],
        ty: partial_order_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Lattice"),
        univ_params: vec![],
        ty: lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DistributiveLattice"),
        univ_params: vec![],
        ty: distributive_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("BooleanAlgebra"),
        univ_params: vec![],
        ty: boolean_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CompleteLattice"),
        univ_params: vec![],
        ty: complete_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GaloisConnection"),
        univ_params: vec![],
        ty: galois_connection_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("KnasterTarski"),
        univ_params: vec![],
        ty: knaster_tarski_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("BirkhoffRepresentation"),
        univ_params: vec![],
        ty: birkhoff_representation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("StoneDuality"),
        univ_params: vec![],
        ty: stone_duality_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DilworthTheorem"),
        univ_params: vec![],
        ty: dilworth_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ClosureOperator"),
        univ_params: vec![],
        ty: closure_operator_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HeytingAlgebra"),
        univ_params: vec![],
        ty: heyting_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ResiduatedLattice"),
        univ_params: vec![],
        ty: residuated_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Frame"),
        univ_params: vec![],
        ty: frame_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Locale"),
        univ_params: vec![],
        ty: locale_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Quantale"),
        univ_params: vec![],
        ty: quantale_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Dcpo"),
        univ_params: vec![],
        ty: dcpo_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ScottTopology"),
        univ_params: vec![],
        ty: scott_topology_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MVAlgebra"),
        univ_params: vec![],
        ty: mv_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("EffectAlgebra"),
        univ_params: vec![],
        ty: effect_algebra_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("OrthomodularLattice"),
        univ_params: vec![],
        ty: orthomodular_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SemimodularLattice"),
        univ_params: vec![],
        ty: semimodular_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GeometricLattice"),
        univ_params: vec![],
        ty: geometric_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LatticeOrderedGroup"),
        univ_params: vec![],
        ty: lattice_ordered_group_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CongruenceLattice"),
        univ_params: vec![],
        ty: congruence_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FreeLattice"),
        univ_params: vec![],
        ty: free_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FuzzyLattice"),
        univ_params: vec![],
        ty: fuzzy_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("BirkhoffSubdirect"),
        univ_params: vec![],
        ty: birkhoff_subdirect_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DedekindMacNeille"),
        univ_params: vec![],
        ty: dedekind_macneille_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("InfExists"),
        univ_params: vec![],
        ty: inf_exists_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SupExists"),
        univ_params: vec![],
        ty: sup_exists_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ScottContinuous"),
        univ_params: vec![],
        ty: scott_continuous_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ACC"),
        univ_params: vec![],
        ty: acc_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DCC"),
        univ_params: vec![],
        ty: dcc_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LatticeCongruence"),
        univ_params: vec![],
        ty: lattice_congruence_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("StoneRepresentation"),
        univ_params: vec![],
        ty: stone_representation_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GaloisClosure"),
        univ_params: vec![],
        ty: galois_closure_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ModularLattice"),
        univ_params: vec![],
        ty: modular_lattice_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("RankFunction"),
        univ_params: vec![],
        ty: rank_function_ty(),
    });
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    fn chain_lattice(n: usize) -> FiniteLattice {
        let mut lat = FiniteLattice::new(n);
        for i in 0..n {
            for j in i..n {
                lat.set_order(i, j);
            }
        }
        lat
    }
    fn boolean_two() -> FiniteLattice {
        chain_lattice(2)
    }
    fn boolean_four() -> FiniteLattice {
        let mut lat = FiniteLattice::new(4);
        lat.set_order(0, 1);
        lat.set_order(0, 2);
        lat.set_order(0, 3);
        lat.set_order(1, 3);
        lat.set_order(2, 3);
        lat
    }
    #[test]
    fn test_finite_lattice_le() {
        let lat = chain_lattice(4);
        assert!(lat.le(0, 3));
        assert!(lat.le(1, 2));
        assert!(!lat.le(2, 1));
    }
    #[test]
    fn test_finite_lattice_meet_join_chain() {
        let lat = chain_lattice(4);
        assert_eq!(lat.meet(1, 3), 1);
        assert_eq!(lat.join(1, 3), 3);
        assert_eq!(lat.meet(2, 2), 2);
    }
    #[test]
    fn test_top_bottom() {
        let lat = chain_lattice(5);
        assert_eq!(lat.bottom(), 0);
        assert_eq!(lat.top(), 4);
    }
    #[test]
    fn test_chain_is_distributive() {
        let lat = chain_lattice(4);
        assert!(lat.is_distributive());
    }
    #[test]
    fn test_boolean_four_is_boolean() {
        let lat = boolean_four();
        assert!(lat.is_distributive());
        assert!(lat.is_boolean());
    }
    #[test]
    fn test_chain_struct() {
        let lat = chain_lattice(5);
        let mut c = Chain::new();
        c.add(0);
        c.add(2);
        c.add(4);
        assert!(c.is_chain_in(&lat));
        assert_eq!(c.len(), 3);
    }
    #[test]
    fn test_antichain_struct() {
        let lat = boolean_four();
        let mut ac = Antichain::new();
        ac.add(1);
        ac.add(2);
        assert!(ac.is_antichain_in(&lat));
        assert_eq!(ac.len(), 2);
        let mut bad_ac = Antichain::new();
        bad_ac.add(0);
        bad_ac.add(1);
        assert!(!bad_ac.is_antichain_in(&lat));
    }
    #[test]
    fn test_build_lattice_theory_env() {
        let env = build_lattice_theory_env();
        assert!(env.get(&Name::str("PartialOrder")).is_some());
        assert!(env.get(&Name::str("Lattice")).is_some());
        assert!(env.get(&Name::str("BooleanAlgebra")).is_some());
        assert!(env.get(&Name::str("KnasterTarski")).is_some());
        assert!(env.get(&Name::str("StoneDuality")).is_some());
        assert!(env.get(&Name::str("HeytingAlgebra")).is_some());
        assert!(env.get(&Name::str("Frame")).is_some());
        assert!(env.get(&Name::str("Locale")).is_some());
        assert!(env.get(&Name::str("Quantale")).is_some());
        assert!(env.get(&Name::str("Dcpo")).is_some());
        assert!(env.get(&Name::str("ScottTopology")).is_some());
        assert!(env.get(&Name::str("MVAlgebra")).is_some());
        assert!(env.get(&Name::str("EffectAlgebra")).is_some());
        assert!(env.get(&Name::str("OrthomodularLattice")).is_some());
        assert!(env.get(&Name::str("SemimodularLattice")).is_some());
        assert!(env.get(&Name::str("GeometricLattice")).is_some());
        assert!(env.get(&Name::str("LatticeOrderedGroup")).is_some());
        assert!(env.get(&Name::str("CongruenceLattice")).is_some());
        assert!(env.get(&Name::str("FreeLattice")).is_some());
        assert!(env.get(&Name::str("FuzzyLattice")).is_some());
        assert!(env.get(&Name::str("BirkhoffSubdirect")).is_some());
        assert!(env.get(&Name::str("DedekindMacNeille")).is_some());
        assert!(env.get(&Name::str("ACC")).is_some());
        assert!(env.get(&Name::str("DCC")).is_some());
        assert!(env.get(&Name::str("LatticeCongruence")).is_some());
        assert!(env.get(&Name::str("StoneRepresentation")).is_some());
        assert!(env.get(&Name::str("ModularLattice")).is_some());
        assert!(env.get(&Name::str("RankFunction")).is_some());
    }
    #[test]
    fn test_complete_lattice_finite_inf_sup() {
        let lat = chain_lattice(5);
        let cl = CompleteLatticeFinite::new(lat);
        assert_eq!(cl.sup(&[1, 2, 3]), 3);
        assert_eq!(cl.inf(&[1, 2, 3]), 1);
        assert_eq!(cl.sup(&[]), 0);
        assert_eq!(cl.inf(&[]), 4);
    }
    #[test]
    fn test_galois_connection_finite_adjoint() {
        let pa = chain_lattice(3);
        let pb = chain_lattice(3);
        let f = vec![0, 1, 2];
        let g = vec![0, 1, 2];
        let gc = GaloisConnectionFinite::new(pa, pb, f, g);
        assert!(gc.is_adjoint());
        assert_eq!(gc.closure(1), 1);
    }
    #[test]
    fn test_galois_connection_finite_non_adjoint() {
        let pa = chain_lattice(3);
        let pb = chain_lattice(3);
        let f = vec![2, 2, 2];
        let g = vec![0, 0, 0];
        let gc = GaloisConnectionFinite::new(pa, pb, f, g);
        assert!(!gc.is_adjoint());
    }
    #[test]
    fn test_bitvector_boolean_algebra() {
        let ba = BitVectorBooleanAlgebra::new(4);
        assert_eq!(ba.top(), 15);
        assert_eq!(ba.bottom(), 0);
        assert_eq!(ba.complement(5), 10);
        assert_eq!(ba.meet(3, 5), 1);
        assert_eq!(ba.join(3, 5), 7);
        let atoms = ba.atoms();
        assert_eq!(atoms, vec![1, 2, 4, 8]);
        assert_eq!(ba.stone_set(5), vec![0, 2]);
    }
    #[test]
    fn test_heyting_algebra_discrete() {
        let ha = HeytingAlgebraFiniteTop::discrete(2);
        assert_eq!(ha.opens.len(), 4);
        assert_eq!(ha.top(), 3);
        assert_eq!(ha.bottom(), 0);
        let a = 0b01u64;
        let neg_a = ha.pseudo_complement(a);
        let neg_neg_a = ha.pseudo_complement(neg_a);
        assert_eq!(neg_neg_a, a);
    }
    #[test]
    fn test_closure_operator_finite() {
        let lat = chain_lattice(4);
        let co = ClosureOperatorFinite::from_fn(lat, |x| if x + 1 < 4 { x + 1 } else { x });
        assert!(co.is_extensive());
        assert!(co.is_idempotent());
        assert!(co.is_monotone());
    }
    #[test]
    fn test_closure_operator_identity_is_closure() {
        let lat = chain_lattice(5);
        let co = ClosureOperatorFinite::from_fn(lat, |x| x);
        assert!(co.is_extensive());
        assert!(co.is_idempotent());
        assert!(co.is_monotone());
        assert_eq!(co.fixed_points().len(), 5);
    }
}
pub fn lt2_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn lt2_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn lt2_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn lt2_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn lt2_ext_impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn lt2_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn lt2_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    lt2_ext_app(lt2_ext_app(f, a), b)
}
pub fn lt2_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn lt2_ext_nat_ty() -> Expr {
    lt2_ext_cst("Nat")
}
/// `ResidLatticeAxiom : ∀ (A : Type), BoundedLattice A → (A → A → A) → (A → A → A) → Prop`
///
/// A residuated lattice: bounded lattice with product and residuals.
pub fn resid_lattice_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("BoundedLattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(
                    lt2_ext_bvar(1),
                    lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_bvar(3)),
                ),
                lt2_ext_arrow(
                    lt2_ext_arrow(
                        lt2_ext_bvar(2),
                        lt2_ext_arrow(lt2_ext_bvar(3), lt2_ext_bvar(4)),
                    ),
                    lt2_ext_prop(),
                ),
            ),
        ),
    )
}
/// `MVAlgebraAxiom : ∀ (A : Type), (A → A → A) → (A → A) → Prop`
///
/// MV-algebra (Lukasiewicz many-valued logic).
pub fn mv_algebra_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_arrow(
                lt2_ext_bvar(0),
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
            ),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `BLAlgebra : ∀ (A : Type), ResidLattice A → Prop`
///
/// BL-algebra (basic logic): residuated lattice with divisibility and prelinearity.
pub fn bl_algebra_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("ResidLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `EffectAlgebraAxiom : ∀ (A : Type), (A → A → A) → (A → A) → Prop`
///
/// Effect algebra for quantum logic: partial sum with orthocomplement.
pub fn effect_algebra_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_arrow(
                lt2_ext_bvar(0),
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
            ),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `OrthomodularLaw : ∀ (A : Type), OrthoLattice A → Prop`
///
/// Orthomodular law: a ≤ b → a ∨ (a⊥ ∧ b) = b.
pub fn orthomodular_law_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("OrthoLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `QuantumLogicAxiom : ∀ (H : HilbertSpace), OrthomodularLattice (Subspaces H) → Prop`
///
/// Quantum logic from the orthomodular lattice of closed subspaces.
pub fn quantum_logic_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "H",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(
                lt2_ext_cst("OrthomodularLattice"),
                lt2_ext_app(lt2_ext_cst("Subspaces"), lt2_ext_bvar(0)),
            ),
            lt2_ext_prop(),
        ),
    )
}
/// `MedianAlgebra : ∀ (A : Type), (A → A → A → A) → Prop`
///
/// Median algebra: ternary operation m(x, y, z) satisfying the median axioms.
pub fn median_algebra_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_arrow(
                lt2_ext_bvar(0),
                lt2_ext_arrow(
                    lt2_ext_bvar(1),
                    lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_bvar(3)),
                ),
            ),
            lt2_ext_prop(),
        ),
    )
}
/// `DeMorganAlgebra : ∀ (A : Type), DistributiveLattice A → (A → A) → Prop`
///
/// De Morgan algebra: distributive lattice with involution satisfying De Morgan laws.
pub fn de_morgan_algebra_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("DistributiveLattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `StoneAlgebra : ∀ (A : Type), PseudocomplementedLattice A → Prop`
///
/// Stone algebra: pseudocomplemented distributive lattice with x* ∨ x** = ⊤.
pub fn stone_algebra_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("PseudocomplementedLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `RelativePseudocomplement : ∀ (A : Type), Lattice A → (A → A → A) → Prop`
///
/// Relative pseudocomplement (Heyting implication): a → b = max{c : a ∧ c ≤ b}.
pub fn relative_pseudocomplement_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(
                    lt2_ext_bvar(1),
                    lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_bvar(3)),
                ),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `CompleteHeytingAlgebra : ∀ (A : Type), Frame A → Prop`
///
/// Complete Heyting algebra (frame): a ∧ (⋁ᵢ bᵢ) = ⋁ᵢ (a ∧ bᵢ).
pub fn complete_heyting_algebra_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `LocaleMap : ∀ (A B : Type), Frame A → Frame B → (B → A) → Prop`
///
/// A locale map (frame homomorphism): preserves finite meets and arbitrary joins.
pub fn locale_map_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_impl_pi(
            "B",
            lt2_ext_type0(),
            lt2_ext_arrow(
                lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(1)),
                lt2_ext_arrow(
                    lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(1)),
                    lt2_ext_arrow(
                        lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_bvar(3)),
                        lt2_ext_prop(),
                    ),
                ),
            ),
        ),
    )
}
/// `ImplicativeLattice : ∀ (A : Type), Lattice A → (A → A → A) → Prop`
///
/// Implicative lattice: a ∧ b ≤ c ↔ a ≤ b → c.
pub fn implicative_lattice_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(
                    lt2_ext_bvar(1),
                    lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_bvar(3)),
                ),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `NucleusOnFrame : ∀ (A : Type), Frame A → (A → A) → Prop`
///
/// Nucleus on a frame: j(a ∧ b) = j(a) ∧ j(b), extensive, idempotent, monotone.
pub fn nucleus_on_frame_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `PriestleyDuality : ∀ (A : Type), DistributiveLattice A → Prop`
///
/// Priestley duality: distributive lattices ≃ Priestley spaces.
pub fn priestley_duality_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("DistributiveLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `SpectralSpace : ∀ (A : Type), Prop`
///
/// Spectral space: compact T₀ sober with compact opens forming a base.
pub fn spectral_space_ty() -> Expr {
    lt2_ext_impl_pi("A", lt2_ext_type0(), lt2_ext_prop())
}
/// `CoherentLocale : ∀ (A : Type), Frame A → Prop`
///
/// Coherent locale: joins of compact elements, compact elements closed under meet.
pub fn coherent_locale_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `LGroupAxiom : ∀ (A : Type), Group A → Lattice A → Prop`
///
/// Lattice-ordered group: order and group operations are compatible.
pub fn l_group_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Group"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(1)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `LRingAxiom : ∀ (A : Type), Ring A → Lattice A → Prop`
///
/// Lattice-ordered ring: a ≥ 0 and b ≥ 0 implies ab ≥ 0.
pub fn l_ring_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Ring"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(1)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `FormalConceptAxiom : ∀ (G M : Type), (G → M → Prop) → Type`
///
/// A formal concept in FCA: pair (A, B) of extent and intent.
pub fn formal_concept_axiom_ty() -> Expr {
    lt2_ext_impl_pi(
        "G",
        lt2_ext_type0(),
        lt2_ext_impl_pi(
            "M",
            lt2_ext_type0(),
            lt2_ext_arrow(
                lt2_ext_arrow(
                    lt2_ext_bvar(1),
                    lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_prop()),
                ),
                lt2_ext_type0(),
            ),
        ),
    )
}
/// `ConceptLattice : ∀ (G M : Type), FormalContext G M → Type`
///
/// The concept lattice of a formal context: complete lattice of all formal concepts.
pub fn concept_lattice_ty() -> Expr {
    lt2_ext_impl_pi(
        "G",
        lt2_ext_type0(),
        lt2_ext_impl_pi(
            "M",
            lt2_ext_type0(),
            lt2_ext_arrow(
                lt2_ext_app2(
                    lt2_ext_cst("FormalContext"),
                    lt2_ext_bvar(1),
                    lt2_ext_bvar(0),
                ),
                lt2_ext_type0(),
            ),
        ),
    )
}
/// `PointFreeTopology : ∀ (A : Type), Frame A → Prop`
///
/// Point-free topology (locale theory): topology described by a frame.
pub fn point_free_topology_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Frame"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `WadjunctionPair : ∀ (A B : Type), (A → B) → (B → A) → Prop`
///
/// Weak adjunction (Galois connection): f(a) ≤ b ↔ a ≤ g(b).
pub fn wadjunction_pair_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_impl_pi(
            "B",
            lt2_ext_type0(),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(0)),
                lt2_ext_arrow(
                    lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_bvar(2)),
                    lt2_ext_prop(),
                ),
            ),
        ),
    )
}
/// `ArchimedeanLatticeGroup : ∀ (A : Type), LGroup A → Prop`
///
/// Archimedean l-group: for all a > 0 and b > 0, ∃n such that na > b.
pub fn archimedean_lattice_group_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("LGroup"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `HollandRepresentation : ∀ (A : Type), LGroup A → Prop`
///
/// Holland's theorem: every l-group embeds into order-preserving permutations of a chain.
pub fn holland_representation_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("LGroup"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `ContinuousLattice : ∀ (A : Type), CompleteLattice A → Prop`
///
/// Continuous lattice: every element is the join of elements way-below it.
pub fn continuous_lattice_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("CompleteLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// `WayBelow : ∀ (A : Type), CompleteLattice A → A → A → Prop`
///
/// Way-below relation (approximation): x ≪ y iff for every directed D with y ≤ ⋁D, ∃d ∈ D, x ≤ d.
pub fn way_below_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("CompleteLattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_bvar(1),
                lt2_ext_arrow(lt2_ext_bvar(2), lt2_ext_prop()),
            ),
        ),
    )
}
/// `LatticeDimension : ∀ (A : Type), Lattice A → Nat → Prop`
///
/// Lattice dimension: length of the longest chain.
pub fn lattice_dimension_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(lt2_ext_nat_ty(), lt2_ext_prop()),
        ),
    )
}
/// `SupermodularFunction : ∀ (A : Type), Lattice A → (A → Nat) → Prop`
///
/// Supermodular function: f(x ∨ y) + f(x ∧ y) ≥ f(x) + f(y).
pub fn supermodular_function_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_nat_ty()),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `SubmodularFunction : ∀ (A : Type), Lattice A → (A → Nat) → Prop`
///
/// Submodular function: f(x ∨ y) + f(x ∧ y) ≤ f(x) + f(y).
pub fn submodular_function_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Lattice"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_arrow(lt2_ext_bvar(1), lt2_ext_nat_ty()),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `FreeDistributiveLattice : ∀ (Vars : Type), Type`
///
/// Free distributive lattice on a set of variables.
pub fn free_distributive_lattice_ty() -> Expr {
    lt2_ext_impl_pi("Vars", lt2_ext_type0(), lt2_ext_type0())
}
/// `WeyteringSheaf : ∀ (A : Type), Locale A → Sheaf A → Prop`
///
/// Sheaf on a locale: contravariant functor satisfying gluing and locality.
pub fn weytering_sheaf_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("Locale"), lt2_ext_bvar(0)),
            lt2_ext_arrow(
                lt2_ext_app(lt2_ext_cst("Sheaf"), lt2_ext_bvar(1)),
                lt2_ext_prop(),
            ),
        ),
    )
}
/// `StoneIdentity : ∀ (A : Type), PseudocomplementedLattice A → Prop`
///
/// The Stone identity in a Stone algebra: x* ∨ x** = ⊤.
pub fn stone_identity_ty() -> Expr {
    lt2_ext_impl_pi(
        "A",
        lt2_ext_type0(),
        lt2_ext_arrow(
            lt2_ext_app(lt2_ext_cst("PseudocomplementedLattice"), lt2_ext_bvar(0)),
            lt2_ext_prop(),
        ),
    )
}
/// Register all extended lattice theory axioms.
pub fn register_lattice_theory_ext(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ResidLatticeAxiom", resid_lattice_axiom_ty()),
        ("MVAlgebraAxiom", mv_algebra_axiom_ty()),
        ("BLAlgebra", bl_algebra_ty()),
        ("EffectAlgebraAxiom", effect_algebra_axiom_ty()),
        ("OrthomodularLaw", orthomodular_law_ty()),
        ("QuantumLogicAxiom", quantum_logic_axiom_ty()),
        ("MedianAlgebra", median_algebra_ty()),
        ("DeMorganAlgebra", de_morgan_algebra_ty()),
        ("StoneAlgebra", stone_algebra_ty()),
        ("RelativePseudocomplement", relative_pseudocomplement_ty()),
        ("CompleteHeytingAlgebra", complete_heyting_algebra_ty()),
        ("LocaleMap", locale_map_ty()),
        ("ImplicativeLattice", implicative_lattice_ty()),
        ("NucleusOnFrame", nucleus_on_frame_ty()),
        ("PriestleyDuality", priestley_duality_ty()),
        ("SpectralSpace", spectral_space_ty()),
        ("CoherentLocale", coherent_locale_ty()),
        ("LGroupAxiom", l_group_axiom_ty()),
        ("LRingAxiom", l_ring_axiom_ty()),
        ("FormalConceptAxiom", formal_concept_axiom_ty()),
        ("ConceptLattice", concept_lattice_ty()),
        ("PointFreeTopology", point_free_topology_ty()),
        ("WadjunctionPair", wadjunction_pair_ty()),
        ("ArchimedeanLatticeGroup", archimedean_lattice_group_ty()),
        ("HollandRepresentation", holland_representation_ty()),
        ("ContinuousLattice", continuous_lattice_ty()),
        ("WayBelow", way_below_ty()),
        ("LatticeDimension", lattice_dimension_ty()),
        ("SupermodularFunction", supermodular_function_ty()),
        ("SubmodularFunction", submodular_function_ty()),
        ("FreeDistributiveLattice", free_distributive_lattice_ty()),
        ("WeyteringSheaf", weytering_sheaf_ty()),
        ("StoneIdentity", stone_identity_ty()),
        ("MedianQuasilattice", median_algebra_ty()),
        ("LGroupArchimedean", archimedean_lattice_group_ty()),
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
