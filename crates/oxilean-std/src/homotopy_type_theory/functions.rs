//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BlakerssMasseyThm, Circle, CohomologyOp, EilenbergMacLaneSpace, FiberSequence,
    FibrationSequenceComputer, FundamentalGroup, HomotopyEquivalence, HomotopyGroupComputer,
    HomotopyGroups, HomotopyLevel, HopfFibration, IdentityType, IsContr, PathComposition,
    PathCompositionChain, PathInversion, PushoutType, Suspension, SuspensionType, Transport,
    Truncation, TruncationComputer, UnivalentFibration,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// `IdentityType : ∀ (A : Type) (a b : A), Type`
///
/// The identity type a =_A b: elements are proofs of equality / paths from a to b.
/// In HoTT this is the fundamental notion: equalities are paths in a space.
pub fn identity_type_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), type0())))
}
/// `PathComposition : ∀ {A : Type} {a b c : A}, a = b → b = c → a = c`
///
/// Path concatenation p · q: given p : a = b and q : b = c, produces p · q : a = c.
/// This makes each type A into an ∞-groupoid.
pub fn path_composition_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    bvar(2),
                    arrow(
                        app3(cst("IdentityType"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("IdentityType"), bvar(4), bvar(2), bvar(1)),
                            app3(cst("IdentityType"), bvar(5), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `PathInversion : ∀ {A : Type} {a b : A}, a = b → b = a`
///
/// Path inversion p^{-1}: given p : a = b, produces p⁻¹ : b = a.
pub fn path_inversion_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    app3(cst("IdentityType"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("IdentityType"), bvar(3), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}
/// `Transport : ∀ {A : Type} (P : A → Type) {a b : A}, a = b → P a → P b`
///
/// Transport (substitution) along a path: given p : a = b and u : P a,
/// produces p_*(u) : P b. Dependent elimination of identity type.
pub fn transport_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(bvar(0), type0()),
            arrow(
                bvar(1),
                arrow(
                    bvar(2),
                    arrow(
                        app3(cst("IdentityType"), bvar(3), bvar(2), bvar(1)),
                        arrow(app(bvar(4), bvar(3)), app(bvar(5), bvar(3))),
                    ),
                ),
            ),
        ),
    )
}
/// `PathInduction : ∀ {A : Type} (C : ∀ (x y : A), x = y → Type), (∀ x, C x x refl) → ∀ {x y} (p : x = y), C x y p`
///
/// The J eliminator: path induction principle. Any property of paths that holds
/// for reflexivity holds for all paths.
pub fn path_induction_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(
                arrow(bvar(1), app2(bvar(1), bvar(0), bvar(0))),
                arrow(
                    bvar(2),
                    arrow(
                        bvar(3),
                        arrow(
                            app3(cst("IdentityType"), bvar(4), bvar(3), bvar(2)),
                            app2(bvar(5), bvar(3), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ReflPath : ∀ {A : Type} (a : A), a = a`
///
/// Reflexivity: the constant path at a point.
pub fn refl_path_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            app3(cst("IdentityType"), bvar(1), bvar(0), bvar(0)),
        ),
    )
}
/// `PathAssoc : ∀ {A : Type} {a b c d : A} (p : a=b) (q : b=c) (r : c=d), (p·q)·r = p·(q·r)`
///
/// Associativity of path concatenation: paths form a groupoid.
pub fn path_assoc_ty() -> Expr {
    impl_pi("A", type0(), prop())
}
/// `PathLeftUnit : ∀ {A : Type} {a b : A} (p : a = b), refl · p = p`
///
/// Left unit law for path concatenation.
pub fn path_left_unit_ty() -> Expr {
    impl_pi("A", type0(), prop())
}
/// `PathRightUnit : ∀ {A : Type} {a b : A} (p : a = b), p · refl = p`
///
/// Right unit law for path concatenation.
pub fn path_right_unit_ty() -> Expr {
    impl_pi("A", type0(), prop())
}
/// `TwoPath : ∀ {A : Type} {a b : A} (p q : a = b), Type`
///
/// A 2-cell α : p = q between parallel paths p, q : a = b.
/// Represents a homotopy between paths (path between paths).
pub fn two_path_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    app3(cst("IdentityType"), bvar(2), bvar(1), bvar(0)),
                    arrow(
                        app3(cst("IdentityType"), bvar(3), bvar(2), bvar(1)),
                        type0(),
                    ),
                ),
            ),
        ),
    )
}
/// `HomotopyGroupoid : Type → Type`
///
/// The fundamental ∞-groupoid of a type A: an ω-groupoid structure
/// where n-cells are n-fold iterated path types.
pub fn homotopy_groupoid_ty() -> Expr {
    arrow(type0(), type1())
}
/// `EckmannHilton : ∀ {A : Type} {a : A}, IsSet (a = a) → Prop`
///
/// The Eckmann-Hilton argument: π₂(A) is abelian because
/// horizontal and vertical composition of 2-cells must coincide.
pub fn eckmann_hilton_ty() -> Expr {
    impl_pi("A", type0(), prop())
}
/// `HomotopyLevel : Type`
///
/// The homotopy level (n-type): Contr (−2), HProp (−1), HSet (0), 1-Groupoid (1), ...
pub fn homotopy_level_ty() -> Expr {
    type0()
}
/// `IsContr : Type → Prop`
///
/// Contractibility: A is contractible iff ∃ (a : A). ∀ (b : A). a = b.
/// Contractible types are the (-2)-truncated types.
pub fn is_contr_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IsProp : Type → Prop`
///
/// Mere proposition: A is a proposition iff ∀ (a b : A). a = b.
/// Propositions are (-1)-truncated types.
pub fn is_prop_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IsSet : Type → Prop`
///
/// h-Set: A is a set iff ∀ (a b : A) (p q : a = b). p = q.
/// Sets are 0-truncated types.
pub fn is_set_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IsNType : Nat → Type → Prop`
///
/// n-truncatedness: A is an n-type if all (n+2)-fold iterated identity types are contractible.
pub fn is_n_type_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `HomotopyEquivalence : ∀ (A B : Type), Type`
///
/// A homotopy equivalence A ≃ B: a function f : A → B with quasi-inverse g
/// and homotopies η : g ∘ f ~ id and ε : f ∘ g ~ id.
pub fn homotopy_equivalence_ty() -> Expr {
    impl_pi("A", type0(), impl_pi("B", type0(), type0()))
}
/// `IdToEquiv : ∀ {A B : Type}, A = B → A ≃ B`
///
/// Transport identity to equivalence: coerce an equality proof to a homotopy equivalence.
/// The map whose inverse (up to propositional equality) is the univalence map.
pub fn id_to_equiv_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app3(cst("IdentityType"), type0(), bvar(1), bvar(0)),
                app2(cst("HomotopyEquivalence"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `UnivalenceAxiom : ∀ (A B : Type), IsEquiv (idToEquiv A B)`
///
/// Voevodsky's univalence axiom: the map idToEquiv is itself an equivalence,
/// so (A = B) ≃ (A ≃ B). This makes the universe univalent.
pub fn univalence_axiom_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            app2(
                cst("HomotopyEquivalence"),
                app3(cst("IdentityType"), type0(), bvar(1), bvar(0)),
                app2(cst("HomotopyEquivalence"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `FunctionExtensionality : ∀ {A : Type} {B : A → Type} (f g : Π x, B x), (∀ x, f x = g x) → f = g`
///
/// Function extensionality: pointwise equal functions are equal.
/// Follows from univalence via the interval HIT, or can be taken as an axiom.
pub fn function_extensionality_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            arrow(bvar(0), type0()),
            arrow(
                arrow(bvar(1), app(bvar(1), bvar(0))),
                arrow(arrow(bvar(2), app(bvar(2), bvar(1))), prop()),
            ),
        ),
    )
}
/// `Happly : ∀ {A : Type} {B : A → Type} {f g : Π x, B x}, f = g → ∀ x, f x = g x`
///
/// Application of a path between functions to an argument.
/// The inverse of function extensionality.
pub fn happly_ty() -> Expr {
    impl_pi("A", type0(), arrow(arrow(bvar(0), type0()), prop()))
}
/// `UA : ∀ {A B : Type}, A ≃ B → A = B`
///
/// The univalence map (ua): convert a homotopy equivalence to a path in the universe.
pub fn ua_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("HomotopyEquivalence"), bvar(1), bvar(0)),
                app3(cst("IdentityType"), type0(), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `Circle : Type`
///
/// The circle S¹ as a HIT: generated by a point `base : S¹` and
/// a loop `loop : base = base`.
pub fn circle_ty() -> Expr {
    type0()
}
/// `CircleBase : Circle`
///
/// The base point of the circle.
pub fn circle_base_ty() -> Expr {
    cst("Circle")
}
/// `CircleLoop : base = base`
///
/// The loop at the base point of the circle.
pub fn circle_loop_ty() -> Expr {
    app3(
        cst("IdentityType"),
        cst("Circle"),
        cst("CircleBase"),
        cst("CircleBase"),
    )
}
/// `CircleInd : ∀ (P : Circle → Type), P base → (loop_* p = p) → ∀ x, P x`
///
/// The elimination principle for S¹: to define a function out of S¹,
/// provide a point over base and a path over loop.
pub fn circle_ind_ty() -> Expr {
    arrow(
        arrow(cst("Circle"), type0()),
        arrow(
            app(bvar(0), cst("CircleBase")),
            arrow(cst("Circle"), app(bvar(2), bvar(0))),
        ),
    )
}
/// `Suspension : Type → Type`
///
/// The suspension ΣA of a type A: a HIT with two points N, S : ΣA
/// and a meridian merid : A → N = S.
pub fn suspension_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SuspNorth : ∀ (A : Type), Suspension A`
///
/// The north pole of the suspension.
pub fn susp_north_ty() -> Expr {
    impl_pi("A", type0(), app(cst("Suspension"), bvar(0)))
}
/// `SuspSouth : ∀ (A : Type), Suspension A`
///
/// The south pole of the suspension.
pub fn susp_south_ty() -> Expr {
    impl_pi("A", type0(), app(cst("Suspension"), bvar(0)))
}
/// `SuspMerid : ∀ (A : Type), A → SuspNorth A = SuspSouth A`
///
/// The meridian: for each a : A, a path from North to South.
pub fn susp_merid_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            app3(
                cst("IdentityType"),
                app(cst("Suspension"), bvar(1)),
                app(cst("SuspNorth"), bvar(2)),
                app(cst("SuspSouth"), bvar(3)),
            ),
        ),
    )
}
/// `Pushout : ∀ (A B C : Type), (C → A) → (C → B) → Type`
///
/// The pushout A ⊔_C B: a HIT with constructors left : A → Pushout,
/// right : B → Pushout, and glue : ∀ c, left(f c) = right(g c).
pub fn pushout_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    arrow(bvar(0), bvar(2)),
                    arrow(arrow(bvar(1), bvar(2)), type0()),
                ),
            ),
        ),
    )
}
/// `Truncation : Nat → Type → Type`
///
/// The n-truncation ‖A‖_n: the universal n-type approximation of A.
/// ‖A‖_{-1} is the propositional truncation (mere existence).
pub fn truncation_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `TruncationIn : ∀ (n : Nat) (A : Type), A → Truncation n A`
///
/// The canonical inclusion of A into its n-truncation.
pub fn truncation_in_ty() -> Expr {
    arrow(
        nat_ty(),
        impl_pi(
            "A",
            type0(),
            arrow(bvar(0), app2(cst("Truncation"), bvar(1), bvar(1))),
        ),
    )
}
/// `Quotient : ∀ (A : Type), (A → A → Prop) → Type`
///
/// The set quotient A/R by an equivalence relation R:
/// a HIT with constructor [_] : A → A/R and path quot : ∀ a b, R a b → [a] = [b].
pub fn quotient_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), type0()),
    )
}
/// `FundamentalGroup : ∀ (X : Type) (x : X), Type`
///
/// π₁(X, x) = Ω(X, x) = (x = x): the loop space / fundamental group.
pub fn fundamental_group_ty() -> Expr {
    impl_pi("X", type0(), arrow(bvar(0), type0()))
}
/// `HomotopyGroups : Nat → ∀ (X : Type) (x : X), Type`
///
/// π_n(X, x) = ‖Ω^n X‖_0: the n-th homotopy group of X at x.
pub fn homotopy_groups_ty() -> Expr {
    arrow(nat_ty(), impl_pi("X", type0(), arrow(bvar(0), type0())))
}
/// `FiberSequence : ∀ (F E B : Type), (F → E) → (E → B) → Prop`
///
/// A fiber sequence F → E → B: E is a fibration over B with fiber F.
/// Gives rise to a long exact sequence of homotopy groups.
pub fn fiber_sequence_ty() -> Expr {
    impl_pi(
        "F",
        type0(),
        impl_pi(
            "E",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(
                    arrow(bvar(2), bvar(1)),
                    arrow(arrow(bvar(2), bvar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `BlakerssMasseyThm : ∀ (m n : Nat) (f : A → B) (g : A → C), Prop`
///
/// The Blakers-Massey theorem: connectivity of the pushout in terms of
/// connectivity of f and g. A key theorem of synthetic homotopy theory.
pub fn blakers_massey_thm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `HopfFibration : S1 → S3 → S2`
///
/// The Hopf fibration: a fiber bundle S¹ → S³ → S²
/// encoded using HITs. Its existence proves π₃(S²) ≠ 0.
pub fn hopf_fibration_ty() -> Expr {
    arrow(cst("Suspension"), app(cst("Suspension"), cst("Circle")))
}
/// `LoopSpaceDelooping : ∀ (G : Type), IsGroup G → ∃ (BG : Type), ΩBG ≃ G`
///
/// Delooping: every group G is the loop space of its classifying space BG.
pub fn loop_space_delooping_ty() -> Expr {
    impl_pi("G", type0(), prop())
}
/// `ConnectedCover : ∀ (n : Nat) (X : Type), Type`
///
/// The n-connected cover X⟨n⟩ of a space X: kills all homotopy groups below n.
pub fn connected_cover_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `FundamentalGroupoid : Type → Type`
///
/// The fundamental groupoid Π₁(X) of a space X: objects are points,
/// morphisms x → y are homotopy classes of paths x = y.
pub fn fundamental_groupoid_ty() -> Expr {
    arrow(type0(), type1())
}
/// `VanKampenThm : ∀ (A B C : Type) (f : C → A) (g : C → B), Prop`
///
/// The Seifert–van Kampen theorem in HoTT: π₁(A ⊔_C B) ≅ π₁(A) *_{π₁(C)} π₁(B).
/// Computed via the pushout HIT.
pub fn van_kampen_thm_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    arrow(bvar(0), bvar(2)),
                    arrow(arrow(bvar(1), bvar(2)), prop()),
                ),
            ),
        ),
    )
}
/// `SeifertVanKampenHIT : ∀ (A B C : Type), (C → A) → (C → B) → Type`
///
/// The pushout-based formulation of Seifert–van Kampen: the fundamental group
/// of a pushout is the amalgamated free product.
pub fn seifert_van_kampen_hit_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    arrow(bvar(0), bvar(2)),
                    arrow(arrow(bvar(1), bvar(2)), type0()),
                ),
            ),
        ),
    )
}
/// `IsOneGroupoid : Type → Prop`
///
/// A type is a 1-groupoid iff all its path spaces are sets (0-truncated).
pub fn is_one_groupoid_ty() -> Expr {
    arrow(type0(), prop())
}
/// `TruncationModality : Nat → Type → Type`
///
/// The truncation modality |−|_n: the universal map to n-types,
/// left adjoint to the inclusion of n-types into all types.
pub fn truncation_modality_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `TruncationRecursor : ∀ (n : Nat) (A B : Type), IsNType n B → (A → B) → Truncation n A → B`
///
/// Recursion principle for n-truncation: maps out of ‖A‖_n into n-types.
pub fn truncation_recursor_ty() -> Expr {
    arrow(
        nat_ty(),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(
                    app2(cst("IsNType"), bvar(2), bvar(0)),
                    arrow(
                        arrow(bvar(2), bvar(1)),
                        arrow(app2(cst("Truncation"), bvar(4), bvar(3)), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `PropTruncation : Type → Prop`
///
/// The propositional truncation ‖A‖: the (-1)-truncation of A.
/// ‖A‖ is inhabited iff A is (mere existence).
pub fn prop_truncation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SetTruncation : Type → Type`
///
/// The set truncation ‖A‖₀: freely add the axiom that all paths are equal.
pub fn set_truncation_ty() -> Expr {
    arrow(type0(), type0())
}
/// `EilenbergMacLane : Type → Nat → Type`
///
/// K(G, n): the Eilenberg–MacLane space for a group G and level n.
/// Characterized by π_n(K(G,n)) = G and all other homotopy groups trivial.
pub fn eilenberg_mac_lane_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `EilenbergMacLaneLoop : ∀ (G : Type) (n : Nat), Ω^n K(G,n) ≃ G`
///
/// The loop space delooping characterization of Eilenberg–MacLane spaces:
/// the n-fold loop space of K(G,n) is equivalent to G.
pub fn eilenberg_mac_lane_loop_ty() -> Expr {
    impl_pi(
        "G",
        type0(),
        arrow(
            nat_ty(),
            app2(
                cst("HomotopyEquivalence"),
                app2(cst("EilenbergMacLane"), bvar(1), bvar(0)),
                bvar(2),
            ),
        ),
    )
}
/// `EilenbergMacLaneUnique : ∀ (G : Type) (n : Nat), IsContr (K(G,n) : Type)`
///
/// Uniqueness of Eilenberg–MacLane spaces: K(G,n) is unique up to equivalence.
pub fn eilenberg_mac_lane_unique_ty() -> Expr {
    impl_pi("G", type0(), arrow(nat_ty(), prop()))
}
/// `Cohomology : Type → Nat → Type → Type`
///
/// H^n(X; G): cohomology of X with coefficients in G,
/// defined as maps X → K(G, n) in HoTT (Brown representability).
pub fn cohomology_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(type0(), type0())))
}
/// `CupProduct : ∀ (X : Type) (G : Type) (m n : Nat), H^m(X;G) → H^n(X;G) → H^(m+n)(X;G)`
///
/// The cup product in cohomology, making H*(X; G) into a graded ring.
pub fn cup_product_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        impl_pi(
            "G",
            type0(),
            arrow(
                nat_ty(),
                arrow(
                    nat_ty(),
                    arrow(
                        app3(cst("Cohomology"), bvar(3), bvar(1), bvar(2)),
                        arrow(app3(cst("Cohomology"), bvar(4), bvar(1), bvar(3)), type0()),
                    ),
                ),
            ),
        ),
    )
}
/// `CohomologyLongExact : ∀ (A B : Type) (f : A → B), Prop`
///
/// The long exact sequence in cohomology arising from a cofiber sequence.
pub fn cohomology_long_exact_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
    )
}
/// `JamesConstruction : Type → Type`
///
/// The James construction J(X): the free topological monoid on a pointed type X.
/// J(X) ≃ ΩΣX as spaces.
pub fn james_construction_ty() -> Expr {
    arrow(type0(), type0())
}
/// `JamesSplitting : ∀ (X : Type), ΣJ(X) ≃ ∨_{n≥1} X^∧n`
///
/// The James splitting: the suspension of J(X) splits as a wedge of smash powers.
pub fn james_splitting_ty() -> Expr {
    impl_pi("X", type0(), prop())
}
/// `SuspLoopAdjunction : ∀ (A B : Type), (ΣA → B) ≃ (A → ΩB)`
///
/// The suspension–loop adjunction: maps out of the suspension of A
/// correspond to maps from A into the loop space of B.
pub fn susp_loop_adjunction_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            app2(
                cst("HomotopyEquivalence"),
                arrow(app(cst("Suspension"), bvar(1)), bvar(0)),
                arrow(bvar(2), app2(cst("HomotopyGroups"), bvar(1), bvar(2))),
            ),
        ),
    )
}
/// `PointedType : Type`
///
/// A pointed type (A, a₀): a type together with a distinguished base point.
pub fn pointed_type_ty() -> Expr {
    type1()
}
/// `LoopSpace : ∀ (A : Type) (a : A), Type`
///
/// The loop space Ω(A, a) = (a = a) at a base point.
pub fn loop_space_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), type0()))
}
/// `Spectrum : Type`
///
/// A spectrum E = {E_n, σ_n : E_n ≃ ΩE_{n+1}} in HoTT:
/// a sequence of pointed types with equivalences to the loop spaces of the next.
pub fn spectrum_ty() -> Expr {
    type1()
}
/// `SphereSpectrum : Spectrum`
///
/// The sphere spectrum 𝕊: the suspension spectrum of S⁰.
/// π_n(𝕊) = π_n^s(S⁰) are the stable homotopy groups of spheres.
pub fn sphere_spectrum_ty() -> Expr {
    cst("Spectrum")
}
/// `StableHomotopyGroups : Spectrum → Nat → Type`
///
/// The stable homotopy groups π_n^s(E) of a spectrum E.
pub fn stable_homotopy_groups_ty() -> Expr {
    arrow(cst("Spectrum"), arrow(nat_ty(), type0()))
}
/// `Modality : Type`
///
/// A modality ○ on a type theory: a reflective subuniverse (○-modal types)
/// with unit η_A : A → ○A and a unique extension property.
pub fn modality_ty() -> Expr {
    type1()
}
/// `LexModality : Modality → Prop`
///
/// A lex (left exact) modality: one that preserves finite limits,
/// in particular preserves identity types.
pub fn lex_modality_ty() -> Expr {
    arrow(cst("Modality"), prop())
}
/// `ReflectiveSubuniverse : (Type → Prop) → Prop`
///
/// A reflective subuniverse: a predicate P on types such that the full
/// sub-∞-category of P-types is reflective.
pub fn reflective_subuniverse_ty() -> Expr {
    arrow(arrow(type0(), prop()), prop())
}
/// `ModalUnit : ∀ (○ : Modality) (A : Type), A → ○ A`
///
/// The unit of the modality: the canonical map from A to its ○-reflection.
pub fn modal_unit_ty() -> Expr {
    arrow(
        cst("Modality"),
        impl_pi("A", type0(), arrow(bvar(0), type0())),
    )
}
/// `ModalInduction : ∀ (○ : Modality) (A : Type) (B : ○A → Type),
///   IsModal ○ B → (∀ a, B (η a)) → ∀ x, B x`
///
/// The induction principle for the modality: to prove B for all modal-reflected
/// elements, it suffices to prove B for elements in the image of η.
pub fn modal_induction_ty() -> Expr {
    arrow(cst("Modality"), impl_pi("A", type0(), prop()))
}
/// `ShapeModality : Type → Type`
///
/// The shape modality ʃ: maps a type/space to its underlying homotopy type.
/// In cohesive HoTT, ʃA is the "discretization" of A.
pub fn shape_modality_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FlatModality : Type → Type`
///
/// The flat modality ♭: maps a type to its "discrete" version by
/// forgetting cohesive structure.
pub fn flat_modality_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SharpModality : Type → Type`
///
/// The sharp modality ♯: maps a type to its "codiscrete" version —
/// the right adjoint to the counit ♭A → A.
pub fn sharp_modality_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CohesionAxiom : ∀ (A : Type), ♭A → ʃA`
///
/// The fundamental cohesion axiom: there is a natural transformation ♭ → ʃ,
/// giving the "points-to-pieces" map.
pub fn cohesion_axiom_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("FlatModality"), bvar(0)),
            app(cst("ShapeModality"), bvar(1)),
        ),
    )
}
/// `DifferentialCohesion : ∀ (A : Type), Type`
///
/// Differential cohesion: an additional ℑ (infinitesimal shape) modality
/// related to formal smoothness and the de Rham stack.
pub fn differential_cohesion_ty() -> Expr {
    arrow(type0(), type0())
}
/// `DirectedPath : ∀ (A : Type) (a b : A), Type`
///
/// Directed paths (morphisms) in a directed type: unlike paths,
/// directed paths need not be invertible.
pub fn directed_path_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), type0())))
}
/// `TwoSidedFibration : ∀ (A B : Type), (A → B → Type) → Prop`
///
/// A two-sided fibration (profunctor) from A to B: a type family P : A → B → Type
/// with covariant functoriality in B and contravariant in A.
pub fn two_sided_fibration_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(arrow(bvar(1), arrow(bvar(0), type0())), prop()),
        ),
    )
}
/// `CommaType : ∀ (A B C : Type), (A → C) → (B → C) → Type`
///
/// The comma type (f ↓ g): pairs (a : A, b : B, p : f a = g b).
/// Models the comma category construction in directed HoTT.
pub fn comma_type_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    arrow(bvar(2), bvar(0)),
                    arrow(arrow(bvar(2), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `KanFilling : ∀ (A : Type) (φ : FaceFormula), Partial φ A → A`
///
/// The Kan filling operation: given a partial element defined on faces φ,
/// compute a complete element. This implements computational univalence.
pub fn kan_filling_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ComputationalUnivalence : ∀ (A B : Type) (e : A ≃ B), ua e ▷ id = e`
///
/// Computational univalence (as in cubical type theory): the map ua is a
/// strict section of idToEquiv, giving definitional computation rules.
pub fn computational_univalence_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(app2(cst("HomotopyEquivalence"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `GlueType : ∀ (A : Type) (φ : FaceFormula) (T : Partial φ Type) (e : Partial φ (T ≃ A)), Type`
///
/// The Glue type constructor from CCHM: allows constructing types that
/// are equivalent to A on faces φ, enabling the definition of ua.
pub fn glue_type_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(app2(cst("HomotopyEquivalence"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `KuratowskiFinite : Type → Prop`
///
/// Kuratowski-finite: A is K-finite iff ∃ n, there is a surjection Fin n → A.
/// In HoTT, this is the correct constructive notion of finite set.
pub fn kuratowski_finite_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BishopFinite : Type → Prop`
///
/// Bishop-finite: A is Bishop-finite iff A ≃ Fin n for some n : ℕ.
/// Stronger than Kuratowski-finite; requires decidable equality.
pub fn bishop_finite_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DedekindFinite : Type → Prop`
///
/// Dedekind-finite: A is Dedekind-finite iff every injection A → A is surjective.
/// In classical mathematics: A is finite iff A is Dedekind-finite.
pub fn dedekind_finite_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FiniteChoice : ∀ (A : Type), BishopFinite A → (A → ‖B‖) → ‖A → B‖`
///
/// Finite choice: for Bishop-finite types, dependent choice holds.
/// In HoTT, propositional truncation distributes over finite types.
pub fn finite_choice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("BishopFinite"), bvar(0)), prop()),
    )
}
/// `Resizing : ∀ (P : Prop), P → PropInSmallUniverse`
///
/// Voevodsky's propositional resizing axiom: every proposition in a large
/// universe is equivalent to one in a small universe.
pub fn resizing_ty() -> Expr {
    arrow(prop(), prop())
}
/// `PropResizing : ∀ (P : Type), IsProp P → ∃ (Q : Prop), P ≃ Q`
///
/// Full propositional resizing: every h-proposition is equivalent to
/// a proposition in the lowest universe.
pub fn prop_resizing_ty() -> Expr {
    impl_pi("P", type0(), arrow(app(cst("IsProp"), bvar(0)), prop()))
}
/// `SetQuotientCardinality : ∀ (A : Type) (R : A → A → Prop), Card(A/R) ≤ Card(A)`
///
/// The cardinality of a set quotient does not exceed the cardinality of the original.
pub fn set_quotient_cardinality_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), prop()),
    )
}
/// Register all homotopy type theory axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("IdentityType", identity_type_ty()),
        ("PathComposition", path_composition_ty()),
        ("PathInversion", path_inversion_ty()),
        ("Transport", transport_ty()),
        ("PathInduction", path_induction_ty()),
        ("ReflPath", refl_path_ty()),
        ("PathAssoc", path_assoc_ty()),
        ("PathLeftUnit", path_left_unit_ty()),
        ("PathRightUnit", path_right_unit_ty()),
        ("TwoPath", two_path_ty()),
        ("HomotopyGroupoid", homotopy_groupoid_ty()),
        ("EckmannHilton", eckmann_hilton_ty()),
        ("HomotopyLevel", homotopy_level_ty()),
        ("IsContr", is_contr_ty()),
        ("IsProp", is_prop_ty()),
        ("IsSet", is_set_ty()),
        ("IsNType", is_n_type_ty()),
        ("HomotopyEquivalence", homotopy_equivalence_ty()),
        ("IdToEquiv", id_to_equiv_ty()),
        ("UnivalenceAxiom", univalence_axiom_ty()),
        ("FunctionExtensionality", function_extensionality_ty()),
        ("Happly", happly_ty()),
        ("UA", ua_ty()),
        ("Circle", circle_ty()),
        ("CircleBase", circle_base_ty()),
        ("CircleInd", circle_ind_ty()),
        ("Suspension", suspension_ty()),
        ("SuspNorth", susp_north_ty()),
        ("SuspSouth", susp_south_ty()),
        ("SuspMerid", susp_merid_ty()),
        ("Pushout", pushout_ty()),
        ("Truncation", truncation_ty()),
        ("TruncationIn", truncation_in_ty()),
        ("Quotient", quotient_ty()),
        ("FundamentalGroup", fundamental_group_ty()),
        ("HomotopyGroups", homotopy_groups_ty()),
        ("FiberSequence", fiber_sequence_ty()),
        ("BlakerssMasseyThm", blakers_massey_thm_ty()),
        ("HopfFibration", hopf_fibration_ty()),
        ("LoopSpaceDelooping", loop_space_delooping_ty()),
        ("ConnectedCover", connected_cover_ty()),
        ("FundamentalGroupoid", fundamental_groupoid_ty()),
        ("VanKampenThm", van_kampen_thm_ty()),
        ("SeifertVanKampenHIT", seifert_van_kampen_hit_ty()),
        ("IsOneGroupoid", is_one_groupoid_ty()),
        ("TruncationModality", truncation_modality_ty()),
        ("TruncationRecursor", truncation_recursor_ty()),
        ("PropTruncation", prop_truncation_ty()),
        ("SetTruncation", set_truncation_ty()),
        ("EilenbergMacLane", eilenberg_mac_lane_ty()),
        ("EilenbergMacLaneLoop", eilenberg_mac_lane_loop_ty()),
        ("EilenbergMacLaneUnique", eilenberg_mac_lane_unique_ty()),
        ("Cohomology", cohomology_ty()),
        ("CupProduct", cup_product_ty()),
        ("CohomologyLongExact", cohomology_long_exact_ty()),
        ("JamesConstruction", james_construction_ty()),
        ("JamesSplitting", james_splitting_ty()),
        ("SuspLoopAdjunction", susp_loop_adjunction_ty()),
        ("PointedType", pointed_type_ty()),
        ("LoopSpace", loop_space_ty()),
        ("Spectrum", spectrum_ty()),
        ("SphereSpectrum", sphere_spectrum_ty()),
        ("StableHomotopyGroups", stable_homotopy_groups_ty()),
        ("Modality", modality_ty()),
        ("LexModality", lex_modality_ty()),
        ("ReflectiveSubuniverse", reflective_subuniverse_ty()),
        ("ModalUnit", modal_unit_ty()),
        ("ModalInduction", modal_induction_ty()),
        ("ShapeModality", shape_modality_ty()),
        ("FlatModality", flat_modality_ty()),
        ("SharpModality", sharp_modality_ty()),
        ("CohesionAxiom", cohesion_axiom_ty()),
        ("DifferentialCohesion", differential_cohesion_ty()),
        ("DirectedPath", directed_path_ty()),
        ("TwoSidedFibration", two_sided_fibration_ty()),
        ("CommaType", comma_type_ty()),
        ("KanFilling", kan_filling_ty()),
        ("ComputationalUnivalence", computational_univalence_ty()),
        ("GlueType", glue_type_ty()),
        ("KuratowskiFinite", kuratowski_finite_ty()),
        ("BishopFinite", bishop_finite_ty()),
        ("DedekindFinite", dedekind_finite_ty()),
        ("FiniteChoice", finite_choice_ty()),
        ("Resizing", resizing_ty()),
        ("PropResizing", prop_resizing_ty()),
        ("SetQuotientCardinality", set_quotient_cardinality_ty()),
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
/// The two-point type (= 𝟚 = Bool): the suspension of 𝟘 is 𝟙,
/// the suspension of 𝟙 is S¹, the suspension of S¹ is S², etc.
pub type SuspBool = Suspension<bool>;
#[cfg(test)]
mod tests {
    use super::*;
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        build_env(&mut env);
        env
    }
    #[test]
    fn test_identity_type_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IdentityType")).is_some());
    }
    #[test]
    fn test_univalence_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("UnivalenceAxiom")).is_some());
    }
    #[test]
    fn test_circle_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Circle")).is_some());
        assert!(env.get(&Name::str("CircleBase")).is_some());
        assert!(env.get(&Name::str("CircleInd")).is_some());
    }
    #[test]
    fn test_suspension_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Suspension")).is_some());
        assert!(env.get(&Name::str("SuspNorth")).is_some());
        assert!(env.get(&Name::str("SuspSouth")).is_some());
        assert!(env.get(&Name::str("SuspMerid")).is_some());
    }
    #[test]
    fn test_homotopy_level_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IsContr")).is_some());
        assert!(env.get(&Name::str("IsProp")).is_some());
        assert!(env.get(&Name::str("IsSet")).is_some());
    }
    #[test]
    fn test_path_composition_rust() {
        let p = IdentityType {
            type_name: "Nat".to_string(),
            left: "a".to_string(),
            right: "b".to_string(),
        };
        let q = IdentityType {
            type_name: "Nat".to_string(),
            left: "b".to_string(),
            right: "c".to_string(),
        };
        let pq = PathComposition::compose(p, q);
        assert!(pq.is_some());
        let pq = pq.expect("pq should be valid");
        assert_eq!(pq.left, "a");
        assert_eq!(pq.right, "c");
    }
    #[test]
    fn test_path_inversion_rust() {
        let p = IdentityType {
            type_name: "Nat".to_string(),
            left: "a".to_string(),
            right: "b".to_string(),
        };
        let inv = PathInversion::invert(p);
        assert_eq!(inv.left, "b");
        assert_eq!(inv.right, "a");
    }
    #[test]
    fn test_homotopy_level_ordering() {
        assert!(HomotopyLevel::Contr < HomotopyLevel::HProp);
        assert!(HomotopyLevel::HProp < HomotopyLevel::HSet);
        assert_eq!(HomotopyLevel::HSet.as_int(), Some(0));
        assert_eq!(HomotopyLevel::Contr.as_int(), Some(-2));
    }
    #[test]
    fn test_homotopy_equivalence_compose() {
        let e1 = HomotopyEquivalence::new("A", "B", "f", "g");
        let e2 = HomotopyEquivalence::new("B", "C", "h", "k");
        let composed = e1.compose(e2);
        assert!(composed.is_some());
        let c = composed.expect("composed should be valid");
        assert_eq!(c.domain, "A");
        assert_eq!(c.codomain, "C");
    }
    #[test]
    fn test_circle_pi1() {
        let pi1 = FundamentalGroup::of_circle();
        assert_eq!(pi1.generators.len(), 1);
        assert_eq!(pi1.relations.len(), 0);
    }
    #[test]
    fn test_hopf_fibration() {
        let seq = HopfFibration::fiber_sequence();
        assert_eq!(seq.fiber, "Circle");
        assert_eq!(seq.base, "S2");
        assert_eq!(HopfFibration::hopf_invariant(), 1);
    }
    #[test]
    fn test_blakers_massey() {
        let thm = BlakerssMasseyThm::new(2, 3);
        assert_eq!(thm.pushout_connectivity(), 5);
    }
    #[test]
    fn test_truncation() {
        let t: Truncation<i32> = Truncation::prop_trunc(42);
        assert_eq!(t.level, -1);
        assert_eq!(t.element, 42);
    }
    #[test]
    fn test_van_kampen_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("VanKampenThm")).is_some());
        assert!(env.get(&Name::str("SeifertVanKampenHIT")).is_some());
        assert!(env.get(&Name::str("FundamentalGroupoid")).is_some());
    }
    #[test]
    fn test_truncation_modalities_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("TruncationModality")).is_some());
        assert!(env.get(&Name::str("TruncationRecursor")).is_some());
        assert!(env.get(&Name::str("PropTruncation")).is_some());
        assert!(env.get(&Name::str("SetTruncation")).is_some());
        assert!(env.get(&Name::str("IsOneGroupoid")).is_some());
    }
    #[test]
    fn test_eilenberg_mac_lane_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("EilenbergMacLane")).is_some());
        assert!(env.get(&Name::str("EilenbergMacLaneLoop")).is_some());
        assert!(env.get(&Name::str("EilenbergMacLaneUnique")).is_some());
    }
    #[test]
    fn test_cohomology_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Cohomology")).is_some());
        assert!(env.get(&Name::str("CupProduct")).is_some());
        assert!(env.get(&Name::str("CohomologyLongExact")).is_some());
    }
    #[test]
    fn test_james_and_susp_loop_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("JamesConstruction")).is_some());
        assert!(env.get(&Name::str("JamesSplitting")).is_some());
        assert!(env.get(&Name::str("SuspLoopAdjunction")).is_some());
    }
    #[test]
    fn test_spectra_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Spectrum")).is_some());
        assert!(env.get(&Name::str("SphereSpectrum")).is_some());
        assert!(env.get(&Name::str("StableHomotopyGroups")).is_some());
        assert!(env.get(&Name::str("PointedType")).is_some());
        assert!(env.get(&Name::str("LoopSpace")).is_some());
    }
    #[test]
    fn test_modalities_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Modality")).is_some());
        assert!(env.get(&Name::str("LexModality")).is_some());
        assert!(env.get(&Name::str("ReflectiveSubuniverse")).is_some());
        assert!(env.get(&Name::str("ModalUnit")).is_some());
        assert!(env.get(&Name::str("ModalInduction")).is_some());
    }
    #[test]
    fn test_cohesive_hott_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ShapeModality")).is_some());
        assert!(env.get(&Name::str("FlatModality")).is_some());
        assert!(env.get(&Name::str("SharpModality")).is_some());
        assert!(env.get(&Name::str("CohesionAxiom")).is_some());
        assert!(env.get(&Name::str("DifferentialCohesion")).is_some());
    }
    #[test]
    fn test_directed_type_theory_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("DirectedPath")).is_some());
        assert!(env.get(&Name::str("TwoSidedFibration")).is_some());
        assert!(env.get(&Name::str("CommaType")).is_some());
    }
    #[test]
    fn test_cubical_axioms_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("KanFilling")).is_some());
        assert!(env.get(&Name::str("ComputationalUnivalence")).is_some());
        assert!(env.get(&Name::str("GlueType")).is_some());
    }
    #[test]
    fn test_finiteness_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("KuratowskiFinite")).is_some());
        assert!(env.get(&Name::str("BishopFinite")).is_some());
        assert!(env.get(&Name::str("DedekindFinite")).is_some());
        assert!(env.get(&Name::str("FiniteChoice")).is_some());
    }
    #[test]
    fn test_resizing_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Resizing")).is_some());
        assert!(env.get(&Name::str("PropResizing")).is_some());
        assert!(env.get(&Name::str("SetQuotientCardinality")).is_some());
    }
    #[test]
    fn test_truncation_computer_contr() {
        let tc = TruncationComputer::new("Unit", vec![]);
        assert_eq!(tc.truncation_level(), HomotopyLevel::Contr);
        assert!(tc.is_prop());
        assert!(tc.is_set());
    }
    #[test]
    fn test_truncation_computer_hset() {
        let tc = TruncationComputer::new("Nat", vec![0]);
        assert_eq!(tc.truncation_level(), HomotopyLevel::HProp);
        let tc2 = TruncationComputer::new("Int", vec![1]);
        assert_eq!(tc2.truncation_level(), HomotopyLevel::HSet);
        assert!(tc2.is_set());
    }
    #[test]
    fn test_truncation_computer_infty_groupoid() {
        let tc = TruncationComputer::new("Circle", vec![2]);
        assert_eq!(tc.truncation_level(), HomotopyLevel::OneGroupoid);
        assert!(!tc.is_set());
        let desc = tc.describe();
        assert!(desc.contains("Circle"));
    }
    #[test]
    fn test_path_composition_chain() {
        let mut chain = PathCompositionChain::new();
        let p = IdentityType {
            type_name: "A".into(),
            left: "a".into(),
            right: "b".into(),
        };
        let q = IdentityType {
            type_name: "A".into(),
            left: "b".into(),
            right: "c".into(),
        };
        let r = IdentityType {
            type_name: "A".into(),
            left: "c".into(),
            right: "a".into(),
        };
        assert!(chain.extend(p));
        assert!(chain.extend(q));
        assert!(chain.extend(r));
        assert_eq!(chain.length, 3);
        assert!(chain.is_loop());
    }
    #[test]
    fn test_path_composition_chain_mismatch() {
        let mut chain = PathCompositionChain::new();
        let p = IdentityType {
            type_name: "A".into(),
            left: "a".into(),
            right: "b".into(),
        };
        let q = IdentityType {
            type_name: "A".into(),
            left: "c".into(),
            right: "d".into(),
        };
        chain.extend(p);
        assert!(!chain.extend(q));
        assert_eq!(chain.length, 1);
    }
    #[test]
    fn test_homotopy_group_computer_spheres() {
        assert_eq!(HomotopyGroupComputer::sphere_homotopy_group(3, 3), "Z");
        assert_eq!(HomotopyGroupComputer::sphere_homotopy_group(3, 2), "Z");
        assert_eq!(HomotopyGroupComputer::sphere_homotopy_group(1, 2), "0");
        assert_eq!(HomotopyGroupComputer::sphere_homotopy_group(4, 3), "Z/2");
    }
    #[test]
    fn test_homotopy_group_simply_connected() {
        assert!(HomotopyGroupComputer::is_simply_connected("S2"));
        assert!(HomotopyGroupComputer::is_simply_connected("S3"));
        assert!(!HomotopyGroupComputer::is_simply_connected("Circle"));
    }
    #[test]
    fn test_fibration_sequence_computer_hopf() {
        let comp = FibrationSequenceComputer::new("Circle", "S3", "S2");
        let les = comp.hopf_les();
        assert!(!les.is_empty());
        assert!(comp.non_trivial_count() > 3);
    }
    #[test]
    fn test_eilenberg_mac_lane_k_z_1() {
        let km = EilenbergMacLaneSpace::k_z_1();
        assert_eq!(km.traditional_name(), Some("S1"));
        assert_eq!(km.n, 1);
        let ls = km.loop_space();
        assert!(ls.is_some());
        assert_eq!(ls.expect("ls should be valid").n, 0);
    }
    #[test]
    fn test_eilenberg_mac_lane_delooping() {
        let km = EilenbergMacLaneSpace::k_z_2();
        assert_eq!(km.traditional_name(), Some("CP_inf"));
        let del = km.delooping();
        assert_eq!(del.n, 3);
        assert_eq!(del.group, "Z");
    }
    #[test]
    fn test_eilenberg_mac_lane_cohomology_desc() {
        let km = EilenbergMacLaneSpace::new("Z/2", 3);
        let desc = km.top_cohomology_description();
        assert!(desc.contains("H^3"));
        assert!(desc.contains("Z/2"));
    }
}
#[cfg(test)]
mod tests_htt_extra {
    use super::*;
    #[test]
    fn test_univalent_fibration() {
        let uf = UnivalentFibration::universe_fibration();
        assert!(uf.is_univalent);
        assert_eq!(uf.base_type, "U");
    }
    #[test]
    fn test_suspension_type() {
        let s1 = SuspensionType::circle();
        assert_eq!(s1.base, "Bool");
        assert!(s1.homotopy_groups_are_interesting());
    }
    #[test]
    fn test_pushout_type() {
        let coprod = PushoutType::coproduct("A", "B");
        assert_eq!(coprod.type_c, "0");
        let susp = PushoutType::suspension("S^1");
        assert_eq!(susp.type_a, "1");
    }
    #[test]
    fn test_cohomology_op() {
        let sq2 = CohomologyOp::steenrod_sq(2);
        assert_eq!(sq2.degree_shift(), 2);
        assert!(sq2.is_stable);
        assert!(sq2.is_primary());
    }
}
