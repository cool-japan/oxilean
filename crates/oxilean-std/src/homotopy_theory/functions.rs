//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CoveringSpaceData, FibrationData, FundamentalGroupData, HomotopyGroupTable, PostnikovSection,
    PostnikovSectionData,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// `Path : ∀ (α : Type) (a b : α), Type`
///
/// The path type: an element of `Path α a b` is a continuous path from a to b
/// in the space α (in HoTT this is the identity/equality type).
pub fn path_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), arrow(bvar(1), type0())))
}
/// `PathConcat : ∀ (α : Type) (a b c : α), Path α a b → Path α b c → Path α a c`
///
/// Concatenation of paths: given p : a = b and q : b = c, produces p · q : a = c.
pub fn path_concat_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    bvar(2),
                    arrow(
                        app3(cst("Path"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("Path"), bvar(4), bvar(2), bvar(1)),
                            app3(cst("Path"), bvar(5), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `PathInverse : ∀ (α : Type) (a b : α), Path α a b → Path α b a`
///
/// Inversion of a path: given p : a = b, produces p⁻¹ : b = a.
pub fn path_inverse_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    app3(cst("Path"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("Path"), bvar(3), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}
/// `Homotopy : ∀ (α β : Type), (α → β) → (α → β) → Type`
///
/// A homotopy between two functions f g : α → β is a pointwise path:
/// H : ∀ x, f x = g x.
pub fn homotopy_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(arrow(bvar(2), bvar(2)), type0()),
            ),
        ),
    )
}
/// `FibSeq : ∀ (F E B : Type), (F → E) → (E → B) → Prop`
///
/// A fiber sequence F → E → B: E → B is a fibration with fiber F,
/// i.e., F ≃ fiber(E → B, *).
pub fn fib_seq_ty() -> Expr {
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
/// `LoopSpace : ∀ (α : Type) (x : α), Type`
///
/// The based loop space Ω(α, x) = Path α x x: continuous loops based at x.
pub fn loop_space_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), type0()))
}
/// `SuspensionType : Type → Type`
///
/// The (unreduced) suspension ΣX of a type X: the pushout of
/// X ← X → X along the two constant maps to the two poles.
pub fn suspension_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FreudenthalSuspension : ∀ (X : Type) (n : Nat), Prop`
///
/// The Freudenthal suspension theorem: if X is n-connected, then the
/// suspension map πₖ(X) → πₖ₊₁(ΣX) is an isomorphism for k ≤ 2n.
pub fn freudenthal_suspension_ty() -> Expr {
    impl_pi("X", type0(), arrow(nat_ty(), prop()))
}
/// `HomotopyGroup : ∀ (n : Nat) (α : Type) (x : α), Type`
///
/// The n-th homotopy group πₙ(α, x) of a pointed space (α, x).
/// π₀ = set of connected components, π₁ = fundamental group, etc.
pub fn homotopy_group_ty() -> Expr {
    arrow(nat_ty(), impl_pi("α", type0(), arrow(bvar(0), type0())))
}
/// `FundamentalGroupoid : Type → Type`
///
/// The fundamental groupoid Π₁(X): the category whose objects are points of X
/// and morphisms are homotopy classes of paths.
pub fn fundamental_groupoid_ty() -> Expr {
    arrow(type0(), type0())
}
/// `KSpaceAxiom : ∀ (G : Type) (n : Nat), Prop`
///
/// The existence axiom for Eilenberg-MacLane spaces K(G, n):
/// a space whose only nontrivial homotopy group is πₙ ≅ G.
pub fn k_space_axiom_ty() -> Expr {
    impl_pi("G", type0(), arrow(nat_ty(), prop()))
}
/// `HomotopyEquivalence : ∀ (α β : Type), Prop`
///
/// A homotopy equivalence between α and β: maps f : α → β, g : β → α with
/// g ∘ f ~ id_α and f ∘ g ~ id_β.
pub fn homotopy_equivalence_ty() -> Expr {
    impl_pi("α", type0(), impl_pi("β", type0(), prop()))
}
/// `FiberBundle : ∀ (E B F : Type), (E → B) → Prop`
///
/// A fiber bundle with total space E, base space B, fiber F, and
/// projection p : E → B such that every point in B has a local
/// trivialisation homeomorphic to U × F.
pub fn fiber_bundle_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi("F", type0(), arrow(arrow(bvar(2), bvar(1)), prop())),
        ),
    )
}
/// `FiberBundleSection : ∀ (E B : Type), (E → B) → Type`
///
/// A section of a fiber bundle p : E → B is a map s : B → E such that
/// p ∘ s = id_B.
pub fn fiber_bundle_section_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), type0())),
    )
}
/// `PrincipalBundle : ∀ (G E B : Type), (E → B) → Prop`
///
/// A principal G-bundle: a fiber bundle where the fiber is a group G
/// and G acts freely and transitively on the fibers.
pub fn principal_bundle_ty() -> Expr {
    impl_pi(
        "G",
        type0(),
        impl_pi(
            "E",
            type0(),
            impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
        ),
    )
}
/// `CoveringSpace : ∀ (E B : Type), (E → B) → Prop`
///
/// A covering space p : E → B: a surjective continuous map such that
/// every point of B has an evenly covered neighbourhood.
pub fn covering_space_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
    )
}
/// `FundamentalGroupAction : ∀ (B : Type) (b : B), Type`
///
/// The monodromy action of π₁(B, b) on the fiber of a covering space.
/// Encodes the deck transformations induced by loops in the base.
pub fn fundamental_group_action_ty() -> Expr {
    impl_pi("B", type0(), arrow(bvar(0), type0()))
}
/// `UniversalCover : ∀ (B : Type) (b : B), Type`
///
/// The universal covering space of a pointed connected space (B, b):
/// the unique simply-connected covering space.
pub fn universal_cover_ty() -> Expr {
    impl_pi("B", type0(), arrow(bvar(0), type0()))
}
/// `WhiteheadTheorem : ∀ (X Y : Type), (X → Y) → Prop`
///
/// Whitehead's theorem: a map f : X → Y between simply-connected CW-complexes
/// that induces isomorphisms on all homotopy groups is a homotopy equivalence.
pub fn whitehead_theorem_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        impl_pi("Y", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
    )
}
/// `HurewiczMap : ∀ (n : Nat) (X : Type) (x : X), HomotopyGroup n X x → Type`
///
/// The Hurewicz homomorphism h : πₙ(X, x) → Hₙ(X; ℤ) from the n-th
/// homotopy group to the n-th singular homology group.
pub fn hurewicz_map_ty() -> Expr {
    arrow(nat_ty(), impl_pi("X", type0(), arrow(bvar(0), type0())))
}
/// `HurewiczTheorem : ∀ (n : Nat) (X : Type) (x : X), Prop`
///
/// Hurewicz theorem: if X is (n-1)-connected for n ≥ 2, then
/// πₙ(X, x) ≅ Hₙ(X; ℤ) via the Hurewicz homomorphism.
pub fn hurewicz_theorem_ty() -> Expr {
    arrow(nat_ty(), impl_pi("X", type0(), arrow(bvar(0), prop())))
}
/// `VanKampen : ∀ (X A B : Type), (A → X) → (B → X) → Prop`
///
/// Seifert-van Kampen theorem: if X = A ∪ B with A, B, A ∩ B path-connected,
/// then π₁(X) ≅ π₁(A) *_{π₁(A∩B)} π₁(B) (amalgamated free product).
pub fn van_kampen_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(
                    arrow(bvar(1), bvar(2)),
                    arrow(arrow(bvar(1), bvar(3)), prop()),
                ),
            ),
        ),
    )
}
/// `LongExactFibration : ∀ (F E B : Type) (b : B), Prop`
///
/// Long exact sequence of a fibration F → E → B:
/// ⋯ → πₙ(F) → πₙ(E) → πₙ(B) → πₙ₋₁(F) → ⋯
pub fn long_exact_fibration_ty() -> Expr {
    impl_pi(
        "F",
        type0(),
        impl_pi("E", type0(), impl_pi("B", type0(), prop())),
    )
}
/// `HomotopyExtensionProperty : ∀ (A X : Type), (A → X) → Prop`
///
/// A pair (X, A) has the HEP (is a cofibration pair) if every homotopy of A
/// extends to a homotopy of X. Equivalently, i : A ↪ X is a cofibration.
pub fn homotopy_extension_property_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi("X", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
    )
}
/// `StableHomotopyGroup : ∀ (k : Nat) (X : Type), Type`
///
/// The k-th stable homotopy group πₖˢ(X) = colim_{n→∞} πₙ₊ₖ(ΣⁿX).
pub fn stable_homotopy_group_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `SuspensionStability : ∀ (k n : Nat) (X : Type), Prop`
///
/// Suspension stability (Freudenthal's theorem in stable range):
/// πₙ₊ₖ(ΣⁿX) ≅ πₙ₊ₖ₊₁(Σⁿ⁺¹X) for n > k + 1.
pub fn suspension_stability_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(type0(), prop())))
}
/// `ReducedSuspension : Type → Type`
///
/// The reduced suspension ΣX = X * S⁰ (smash product with S¹):
/// collapses the two base points of the unreduced suspension to a single point.
pub fn reduced_suspension_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PostnikovSection : ∀ (X : Type) (n : Nat), Type`
///
/// The n-th Postnikov section P_n(X): the n-truncation of X,
/// i.e., τ_{≤n}(X), which has the same πₖ as X for k ≤ n and trivial
/// higher homotopy groups.
pub fn postnikov_section_ty() -> Expr {
    impl_pi("X", type0(), arrow(nat_ty(), type0()))
}
/// `PostnikovTower : ∀ (X : Type), Type`
///
/// The full Postnikov tower of X: the sequence P_0(X) ← P_1(X) ← P_2(X) ← ⋯
/// together with the principal fibrations connecting them.
pub fn postnikov_tower_ty() -> Expr {
    arrow(type0(), type0())
}
/// `KInvariant : ∀ (X : Type) (n : Nat), Type`
///
/// The k-invariant (Postnikov invariant) kₙ : P_{n-1}(X) → K(πₙX, n+1):
/// the obstruction to lifting the Postnikov section, living in Hⁿ⁺¹(P_{n-1}X; πₙX).
pub fn k_invariant_ty() -> Expr {
    impl_pi("X", type0(), arrow(nat_ty(), type0()))
}
/// `ObstructionClass : ∀ (X B : Type) (n : Nat), (X → B) → Type`
///
/// The obstruction class in Hⁿ⁺¹(B; πₙ(X)) to extending or deforming
/// a map from the n-skeleton of B into X.
pub fn obstruction_class_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(nat_ty(), arrow(arrow(bvar(2), bvar(1)), type0())),
        ),
    )
}
/// `ObstructionTheory : ∀ (X B : Type), Prop`
///
/// The general obstruction theory framework: successive obstructions in
/// cohomology with local coefficients to extending maps over skeleta.
pub fn obstruction_theory_ty() -> Expr {
    impl_pi("X", type0(), impl_pi("B", type0(), prop()))
}
/// `ModelCategory : Type → Prop`
///
/// A model category structure on a category C: three distinguished classes
/// of morphisms (weak equivalences, cofibrations, fibrations) satisfying
/// the Quillen axioms MC1–MC5.
pub fn model_category_ty() -> Expr {
    arrow(type0(), prop())
}
/// `WeakEquivalence : ∀ (C : Type), (C → C → Prop)`
///
/// The class of weak equivalences in a model category C:
/// morphisms inducing isomorphisms on all homotopy groups.
pub fn weak_equivalence_ty() -> Expr {
    impl_pi("C", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `CofibrationMorphism : ∀ (C : Type), C → C → Prop`
///
/// The class of cofibrations in a model category C:
/// morphisms with the left lifting property with respect to acyclic fibrations.
pub fn cofibration_morphism_ty() -> Expr {
    impl_pi("C", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `FibrationMorphism : ∀ (C : Type), C → C → Prop`
///
/// The class of fibrations in a model category C:
/// morphisms with the right lifting property with respect to acyclic cofibrations.
pub fn fibration_morphism_ty() -> Expr {
    impl_pi("C", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `QuillenAdjunction : ∀ (C D : Type), (C → D) → (D → C) → Prop`
///
/// A Quillen adjunction (F ⊣ G) between model categories C and D:
/// F preserves cofibrations and acyclic cofibrations (equivalently,
/// G preserves fibrations and acyclic fibrations).
pub fn quillen_adjunction_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi(
            "D",
            type0(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(arrow(bvar(1), bvar(2)), prop()),
            ),
        ),
    )
}
/// `QuillenEquivalence : ∀ (C D : Type), (C → D) → (D → C) → Prop`
///
/// A Quillen equivalence: a Quillen adjunction (F ⊣ G) such that F and G
/// induce inverse equivalences on the homotopy categories Ho(C) ≃ Ho(D).
pub fn quillen_equivalence_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi(
            "D",
            type0(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(arrow(bvar(1), bvar(2)), prop()),
            ),
        ),
    )
}
/// `DerivedFunctor : ∀ (C D : Type), (C → D) → Type`
///
/// The (total) left or right derived functor LF (or RF) of a functor F
/// between model categories, computed via cofibrant/fibrant replacement.
pub fn derived_functor_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi("D", type0(), arrow(arrow(bvar(1), bvar(0)), type0())),
    )
}
/// `SerreSpectralSeq : ∀ (F E B : Type), Prop`
///
/// The Serre spectral sequence for a fibration F → E → B:
/// E₂^{p,q} = Hₚ(B; Hq(F)) ⟹ H_{p+q}(E).
pub fn serre_spectral_seq_ty() -> Expr {
    impl_pi(
        "F",
        type0(),
        impl_pi("E", type0(), impl_pi("B", type0(), prop())),
    )
}
/// `AtiyahHirzebruch : ∀ (X : Type) (h : Nat → Type), Prop`
///
/// The Atiyah-Hirzebruch spectral sequence for a generalised cohomology theory h*:
/// E₂^{p,q} = Hᵖ(X; hq(pt)) ⟹ h^{p+q}(X).
pub fn atiyah_hirzebruch_ty() -> Expr {
    impl_pi("X", type0(), arrow(arrow(nat_ty(), type0()), prop()))
}
/// `SteenrodAlgebra : Nat → Type`
///
/// The Steenrod algebra A_p at a prime p: the algebra of stable cohomology
/// operations for mod-p cohomology, generated by Sq^i (p=2) or P^i, β (p odd).
pub fn steenrod_algebra_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SteenrodSquare : ∀ (i : Nat) (X : Type), (X → Type) → (X → Type)`
///
/// The i-th Steenrod square Sq^i : H^n(X; F₂) → H^{n+i}(X; F₂):
/// a natural stable cohomology operation, Sq^0 = id, Sq^n = cup-square on H^n.
pub fn steenrod_square_ty() -> Expr {
    arrow(
        nat_ty(),
        impl_pi(
            "X",
            type0(),
            arrow(arrow(bvar(0), type0()), arrow(bvar(1), type0())),
        ),
    )
}
/// `AdamOperations : ∀ (k : Nat) (X : Type), Type`
///
/// Adams operations ψ^k in K-theory: ring homomorphisms K(X) → K(X)
/// characterised by ψ^k(L) = L^k on line bundles.
pub fn adam_operations_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `BrownRepresentability : ∀ (h : Type → Type), Prop`
///
/// Brown's representability theorem: every cohomology theory h^n on CW-complexes
/// is representable, i.e., h^n(X) ≅ [X, K_n] for some spectrum K.
pub fn brown_representability_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// `InfinityGroupoid : Type → Type`
///
/// The fundamental ∞-groupoid Π_∞(X) of a space X: the (∞,0)-category
/// whose objects are points, 1-morphisms are paths, 2-morphisms are
/// homotopies between paths, and so on at all levels.
pub fn infinity_groupoid_ty() -> Expr {
    arrow(type0(), type0())
}
/// `UnivalenceAxiom : ∀ (α β : Type), Prop`
///
/// Voevodsky's univalence axiom in HoTT: (α = β) ≃ (α ≃ β),
/// i.e., equivalence of types is equivalent to equality of types.
pub fn univalence_axiom_ty() -> Expr {
    impl_pi("α", type0(), impl_pi("β", type0(), prop()))
}
/// `HigherInductiveType : ∀ (spec : Type), Type`
///
/// A higher inductive type (HIT): a type defined by point constructors
/// (ordinary) and path constructors (paths, higher paths, etc.),
/// as in the circle S¹, suspension, pushouts, truncations.
pub fn higher_inductive_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// `HomotopyPushout : ∀ (A B C : Type), (A → B) → (A → C) → Type`
///
/// The homotopy pushout (double mapping cylinder / homotopy colimit) of
/// a span B ← A → C, a fundamental HIT construction.
pub fn homotopy_pushout_ty() -> Expr {
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
                    arrow(bvar(2), bvar(1)),
                    arrow(arrow(bvar(3), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `BlakersMassey : ∀ (A B C : Type) (m n : Nat), (A → B) → (A → C) → Prop`
///
/// The Blakers-Massey theorem (homotopy excision): if f : A → B is
/// m-connected and g : A → C is n-connected, then the natural map from
/// A to the homotopy pullback of B and C over the pushout is
/// (m + n - 1)-connected.
pub fn blakers_massey_ty() -> Expr {
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
                    nat_ty(),
                    arrow(
                        nat_ty(),
                        arrow(
                            arrow(bvar(4), bvar(3)),
                            arrow(arrow(bvar(5), bvar(3)), prop()),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ConnectednessDegree : ∀ (X : Type) (n : Nat), Prop`
///
/// X is n-connected: πₖ(X, x) is trivial for all k ≤ n and all base points x.
pub fn connectedness_degree_ty() -> Expr {
    impl_pi("X", type0(), arrow(nat_ty(), prop()))
}
/// Register all homotopy theory axioms into the given kernel environment.
pub fn register_homotopy_theory(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Path", path_ty()),
        ("PathConcat", path_concat_ty()),
        ("PathInverse", path_inverse_ty()),
        ("Homotopy", homotopy_ty()),
        ("FibSeq", fib_seq_ty()),
        ("LoopSpace", loop_space_ty()),
        ("SuspensionType", suspension_type_ty()),
        ("FreudenthalSuspension", freudenthal_suspension_ty()),
        ("HomotopyGroup", homotopy_group_ty()),
        ("FundamentalGroupoid", fundamental_groupoid_ty()),
        ("KSpaceAxiom", k_space_axiom_ty()),
        ("HomotopyEquivalence", homotopy_equivalence_ty()),
        ("FiberBundle", fiber_bundle_ty()),
        ("FiberBundleSection", fiber_bundle_section_ty()),
        ("PrincipalBundle", principal_bundle_ty()),
        ("CoveringSpace", covering_space_ty()),
        ("FundamentalGroupAction", fundamental_group_action_ty()),
        ("UniversalCover", universal_cover_ty()),
        ("WhiteheadTheorem", whitehead_theorem_ty()),
        ("HurewiczMap", hurewicz_map_ty()),
        ("HurewiczTheorem", hurewicz_theorem_ty()),
        ("VanKampen", van_kampen_ty()),
        ("LongExactFibration", long_exact_fibration_ty()),
        (
            "HomotopyExtensionProperty",
            homotopy_extension_property_ty(),
        ),
        ("StableHomotopyGroup", stable_homotopy_group_ty()),
        ("SuspensionStability", suspension_stability_ty()),
        ("ReducedSuspension", reduced_suspension_ty()),
        ("PostnikovSection", postnikov_section_ty()),
        ("PostnikovTower", postnikov_tower_ty()),
        ("KInvariant", k_invariant_ty()),
        ("ObstructionClass", obstruction_class_ty()),
        ("ObstructionTheory", obstruction_theory_ty()),
        ("ModelCategory", model_category_ty()),
        ("WeakEquivalence", weak_equivalence_ty()),
        ("CofibrationMorphism", cofibration_morphism_ty()),
        ("FibrationMorphism", fibration_morphism_ty()),
        ("QuillenAdjunction", quillen_adjunction_ty()),
        ("QuillenEquivalence", quillen_equivalence_ty()),
        ("DerivedFunctor", derived_functor_ty()),
        ("SerreSpectralSeq", serre_spectral_seq_ty()),
        ("AtiyahHirzebruch", atiyah_hirzebruch_ty()),
        ("SteenrodAlgebra", steenrod_algebra_ty()),
        ("SteenrodSquare", steenrod_square_ty()),
        ("AdamOperations", adam_operations_ty()),
        ("BrownRepresentability", brown_representability_ty()),
        ("InfinityGroupoid", infinity_groupoid_ty()),
        ("UnivalenceAxiom", univalence_axiom_ty()),
        ("HigherInductiveType", higher_inductive_type_ty()),
        ("HomotopyPushout", homotopy_pushout_ty()),
        ("BlakersMassey", blakers_massey_ty()),
        ("ConnectednessDegree", connectedness_degree_ty()),
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
/// Compute the connectivity of a product space: min(conn(A), conn(B)).
pub fn product_connectivity(conn_a: i32, conn_b: i32) -> i32 {
    conn_a.min(conn_b)
}
/// Compute the connectivity of a join A * B: conn(A) + conn(B) + 2.
pub fn join_connectivity(conn_a: i32, conn_b: i32) -> i32 {
    conn_a + conn_b + 2
}
/// Blakers-Massey connectivity bound for homotopy pushout:
/// if f : A → B is m-connected and g : A → C is n-connected,
/// the natural map A → B ×_{B∪_A C} C is (m+n-1)-connected.
pub fn blakers_massey_connectivity(m: i32, n: i32) -> i32 {
    (m + n - 1).max(-1)
}
/// Check whether the Freudenthal suspension theorem applies:
/// X n-connected ⟹ suspension map πₖ(X) → πₖ₊₁(ΣX) is iso for k ≤ 2n.
pub fn freudenthal_stable_range(n_connected: u32, k: u32) -> bool {
    k <= 2 * n_connected
}
#[cfg(test)]
mod tests {
    use super::*;
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        register_homotopy_theory(&mut env);
        env
    }
    #[test]
    fn test_path_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Path")).is_some());
    }
    #[test]
    fn test_path_concat_and_inverse_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PathConcat")).is_some());
        assert!(env.get(&Name::str("PathInverse")).is_some());
    }
    #[test]
    fn test_homotopy_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Homotopy")).is_some());
    }
    #[test]
    fn test_fib_seq_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FibSeq")).is_some());
    }
    #[test]
    fn test_loop_space_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("LoopSpace")).is_some());
    }
    #[test]
    fn test_suspension_and_freudenthal_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("SuspensionType")).is_some());
        assert!(env.get(&Name::str("FreudenthalSuspension")).is_some());
    }
    #[test]
    fn test_homotopy_group_and_groupoid_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("HomotopyGroup")).is_some());
        assert!(env.get(&Name::str("FundamentalGroupoid")).is_some());
    }
    #[test]
    fn test_k_space_and_homotopy_equivalence_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("KSpaceAxiom")).is_some());
        assert!(env.get(&Name::str("HomotopyEquivalence")).is_some());
    }
    #[test]
    fn test_fiber_bundle_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FiberBundle")).is_some());
        assert!(env.get(&Name::str("FiberBundleSection")).is_some());
        assert!(env.get(&Name::str("PrincipalBundle")).is_some());
    }
    #[test]
    fn test_covering_space_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("CoveringSpace")).is_some());
        assert!(env.get(&Name::str("FundamentalGroupAction")).is_some());
        assert!(env.get(&Name::str("UniversalCover")).is_some());
    }
    #[test]
    fn test_whitehead_hurewicz_vankampen_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("WhiteheadTheorem")).is_some());
        assert!(env.get(&Name::str("HurewiczMap")).is_some());
        assert!(env.get(&Name::str("HurewiczTheorem")).is_some());
        assert!(env.get(&Name::str("VanKampen")).is_some());
    }
    #[test]
    fn test_les_and_hep_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("LongExactFibration")).is_some());
        assert!(env.get(&Name::str("HomotopyExtensionProperty")).is_some());
    }
    #[test]
    fn test_stable_homotopy_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("StableHomotopyGroup")).is_some());
        assert!(env.get(&Name::str("SuspensionStability")).is_some());
        assert!(env.get(&Name::str("ReducedSuspension")).is_some());
    }
    #[test]
    fn test_postnikov_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PostnikovSection")).is_some());
        assert!(env.get(&Name::str("PostnikovTower")).is_some());
        assert!(env.get(&Name::str("KInvariant")).is_some());
    }
    #[test]
    fn test_obstruction_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ObstructionClass")).is_some());
        assert!(env.get(&Name::str("ObstructionTheory")).is_some());
    }
    #[test]
    fn test_model_category_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ModelCategory")).is_some());
        assert!(env.get(&Name::str("WeakEquivalence")).is_some());
        assert!(env.get(&Name::str("CofibrationMorphism")).is_some());
        assert!(env.get(&Name::str("FibrationMorphism")).is_some());
        assert!(env.get(&Name::str("QuillenAdjunction")).is_some());
        assert!(env.get(&Name::str("QuillenEquivalence")).is_some());
        assert!(env.get(&Name::str("DerivedFunctor")).is_some());
    }
    #[test]
    fn test_spectral_sequences_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("SerreSpectralSeq")).is_some());
        assert!(env.get(&Name::str("AtiyahHirzebruch")).is_some());
    }
    #[test]
    fn test_steenrod_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("SteenrodAlgebra")).is_some());
        assert!(env.get(&Name::str("SteenrodSquare")).is_some());
        assert!(env.get(&Name::str("AdamOperations")).is_some());
    }
    #[test]
    fn test_brown_infinity_hott_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("BrownRepresentability")).is_some());
        assert!(env.get(&Name::str("InfinityGroupoid")).is_some());
        assert!(env.get(&Name::str("UnivalenceAxiom")).is_some());
        assert!(env.get(&Name::str("HigherInductiveType")).is_some());
        assert!(env.get(&Name::str("HomotopyPushout")).is_some());
    }
    #[test]
    fn test_blakers_massey_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("BlakersMassey")).is_some());
        assert!(env.get(&Name::str("ConnectednessDegree")).is_some());
    }
    #[test]
    fn test_fundamental_group_data() {
        let g = FundamentalGroupData::circle();
        assert_eq!(g.rank(), 1);
        assert!(!g.is_trivial);
        assert!(g.is_abelian);
        let t = FundamentalGroupData::torus();
        assert_eq!(t.rank(), 2);
        assert_eq!(t.num_relations(), 1);
        let trivial = FundamentalGroupData::trivial("S^2");
        assert!(trivial.is_trivial);
        assert_eq!(trivial.rank(), 0);
        let rp2 = FundamentalGroupData::real_projective_plane();
        assert_eq!(rp2.rank(), 1);
        let surf = FundamentalGroupData::orientable_surface(2);
        assert_eq!(surf.rank(), 4);
    }
    #[test]
    fn test_covering_space_data() {
        let cov = CoveringSpaceData::real_over_circle();
        assert!(cov.is_universal);
        assert!(cov.sheet_count.is_none());
        let euler = cov.euler_characteristic_total(0);
        assert_eq!(euler, None);
        let sphere = CoveringSpaceData::sphere_over_projective(2);
        assert_eq!(sphere.sheet_count, Some(2));
        assert!(sphere.is_universal);
        let winding = CoveringSpaceData::circle_winding(3);
        assert_eq!(winding.sheet_count, Some(3));
        assert_eq!(winding.euler_characteristic_total(0), Some(0));
    }
    #[test]
    fn test_homotopy_group_table() {
        let table = HomotopyGroupTable::classical();
        assert_eq!(table.lookup(1, 2), Some("0"));
        assert_eq!(table.lookup(2, 3), Some("0"));
        assert_eq!(table.lookup(1, 1), Some("Z"));
        assert_eq!(table.lookup(3, 3), Some("Z"));
        assert_eq!(table.lookup(3, 2), Some("Z"));
        assert_eq!(table.lookup(6, 3), Some("Z/12"));
        assert!(table.lookup(20, 5).is_none());
        assert!(table.num_nontrivial() > 0);
    }
    #[test]
    fn test_fibration_data() {
        let hopf = FibrationData::hopf_s1();
        assert_eq!(hopf.fiber, "S^1");
        assert_eq!(hopf.total, "S^3");
        assert_eq!(hopf.base, "S^2");
        assert!(hopf.is_principal);
        let pl = FibrationData::path_loop("S^2");
        assert_eq!(pl.base, "S^2");
        assert!(!pl.is_principal);
        let ub = FibrationData::universal_bundle("U(n)");
        assert_eq!(ub.structure_group, Some("U(n)".to_string()));
    }
    #[test]
    fn test_postnikov_section_data() {
        let p2 = PostnikovSectionData::s2_p2();
        assert_eq!(p2.truncation_level, 2);
        assert!(p2.is_em_space);
        assert_eq!(p2.num_groups(), 1);
        let p3 = PostnikovSectionData::s2_p3();
        assert_eq!(p3.truncation_level, 3);
        assert!(!p3.is_em_space);
        assert!(p3.k_invariant.is_some());
        let custom = PostnikovSectionData::new(
            "X",
            4,
            vec![(2, "Z".to_string()), (4, "Z/2".to_string())],
            None,
        );
        assert_eq!(custom.num_groups(), 2);
    }
    #[test]
    fn test_connectivity_utils() {
        assert_eq!(product_connectivity(2, 3), 2);
        assert_eq!(join_connectivity(1, 2), 5);
        assert_eq!(blakers_massey_connectivity(3, 4), 6);
        assert_eq!(blakers_massey_connectivity(0, 0), -1);
        assert!(freudenthal_stable_range(2, 4));
        assert!(!freudenthal_stable_range(2, 5));
    }
    #[test]
    fn test_total_axiom_count() {
        let env = registered_env();
        let names = [
            "Path",
            "PathConcat",
            "PathInverse",
            "Homotopy",
            "FibSeq",
            "LoopSpace",
            "SuspensionType",
            "FreudenthalSuspension",
            "HomotopyGroup",
            "FundamentalGroupoid",
            "KSpaceAxiom",
            "HomotopyEquivalence",
            "FiberBundle",
            "FiberBundleSection",
            "PrincipalBundle",
            "CoveringSpace",
            "FundamentalGroupAction",
            "UniversalCover",
            "WhiteheadTheorem",
            "HurewiczMap",
            "HurewiczTheorem",
            "VanKampen",
            "LongExactFibration",
            "HomotopyExtensionProperty",
            "StableHomotopyGroup",
            "SuspensionStability",
            "ReducedSuspension",
            "PostnikovSection",
            "PostnikovTower",
            "KInvariant",
            "ObstructionClass",
            "ObstructionTheory",
            "ModelCategory",
            "WeakEquivalence",
            "CofibrationMorphism",
            "FibrationMorphism",
            "QuillenAdjunction",
            "QuillenEquivalence",
            "DerivedFunctor",
            "SerreSpectralSeq",
            "AtiyahHirzebruch",
            "SteenrodAlgebra",
            "SteenrodSquare",
            "AdamOperations",
            "BrownRepresentability",
            "InfinityGroupoid",
            "UnivalenceAxiom",
            "HigherInductiveType",
            "HomotopyPushout",
            "BlakersMassey",
            "ConnectednessDegree",
        ];
        for name in names {
            assert!(
                env.get(&Name::str(name)).is_some(),
                "Missing axiom: {}",
                name
            );
        }
        assert_eq!(names.len(), 51);
    }
}
pub fn htpy_ext_nat_ty() -> Expr {
    cst("Nat")
}
pub fn htpy_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn htpy_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn htpy_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn htpy_ext_impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn htpy_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn htpy_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn htpy_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    htpy_ext_app(htpy_ext_app(f, a), b)
}
pub fn htpy_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `HigherHomotopyGroup : ∀ (n : Nat) (X : Type) (x : X), Type`
///
/// The n-th homotopy group πₙ(X, x) for n ≥ 2, which is abelian.
pub fn higher_homotopy_group_ty() -> Expr {
    htpy_ext_arrow(
        htpy_ext_nat_ty(),
        htpy_ext_impl_pi(
            "X",
            htpy_ext_type0(),
            htpy_ext_arrow(htpy_ext_bvar(0), htpy_ext_type0()),
        ),
    )
}
/// `AbelnessHigherGroups : ∀ (n : Nat) (X : Type), n ≥ 2 → IsAbelian (πₙ X) → Prop`
///
/// For n ≥ 2 the homotopy groups πₙ(X) are abelian.
pub fn abelness_higher_groups_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(
            htpy_ext_nat_ty(),
            htpy_ext_arrow(
                htpy_ext_app(htpy_ext_cst("GeTwo"), htpy_ext_bvar(0)),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// `HomotopyLongExactSeq : ∀ (F E B : Type), Fibration F E B → Prop`
///
/// Long exact sequence in homotopy for a fibration F → E → B.
pub fn homotopy_long_exact_seq_ty() -> Expr {
    htpy_ext_impl_pi(
        "F",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "E",
            htpy_ext_type0(),
            htpy_ext_impl_pi(
                "B",
                htpy_ext_type0(),
                htpy_ext_arrow(
                    htpy_ext_app2(
                        htpy_ext_app(htpy_ext_cst("Fibration"), htpy_ext_bvar(2)),
                        htpy_ext_bvar(1),
                        htpy_ext_bvar(0),
                    ),
                    htpy_ext_prop(),
                ),
            ),
        ),
    )
}
/// `FreudenthalSuspThm : ∀ (X : Type) (n : Nat), IsNConnected X n → Prop`
///
/// Freudenthal suspension theorem: Σ : πₖ(X) → πₖ₊₁(ΣX) is iso for k ≤ 2n.
pub fn freudenthal_susp_thm_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(
            htpy_ext_nat_ty(),
            htpy_ext_arrow(
                htpy_ext_app2(
                    htpy_ext_cst("IsNConnected"),
                    htpy_ext_bvar(1),
                    htpy_ext_bvar(0),
                ),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// `HopfFibration : Prop`
///
/// The Hopf fibration S¹ → S³ → S², giving π₃(S²) ≅ ℤ.
pub fn hopf_fibration_ty() -> Expr {
    htpy_ext_prop()
}
/// `SerreSpectralSequence : ∀ (F E B : Type), Fibration F E B → Nat → Nat → Type`
///
/// The Serre spectral sequence E_r^{p,q} ⟹ H_{p+q}(E).
pub fn serre_spectral_sequence_ty() -> Expr {
    htpy_ext_impl_pi(
        "F",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "E",
            htpy_ext_type0(),
            htpy_ext_impl_pi(
                "B",
                htpy_ext_type0(),
                htpy_ext_arrow(
                    htpy_ext_app2(
                        htpy_ext_app(htpy_ext_cst("Fibration"), htpy_ext_bvar(2)),
                        htpy_ext_bvar(1),
                        htpy_ext_bvar(0),
                    ),
                    htpy_ext_arrow(
                        htpy_ext_nat_ty(),
                        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
                    ),
                ),
            ),
        ),
    )
}
/// `PostnikovTruncation : ∀ (X : Type) (n : Nat), Type`
///
/// The Postnikov n-truncation τ_{≤n}(X).
pub fn postnikov_truncation_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `WhiteheadEquivCriteria : ∀ (X Y : Type) (f : X → Y), IsWeakEquiv f → IsHomotopyEquiv f`
///
/// Whitehead's theorem: a weak homotopy equivalence between CW-complexes is genuine.
pub fn whitehead_equiv_criteria_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "Y",
            htpy_ext_type0(),
            htpy_ext_arrow(
                htpy_ext_arrow(htpy_ext_bvar(1), htpy_ext_bvar(0)),
                htpy_ext_arrow(
                    htpy_ext_app(htpy_ext_cst("IsWeakEquiv"), htpy_ext_bvar(0)),
                    htpy_ext_app(htpy_ext_cst("IsHomotopyEquiv"), htpy_ext_bvar(1)),
                ),
            ),
        ),
    )
}
/// `CellularApproximation : ∀ (X Y : Type) (f : X → Y), Prop`
///
/// Every continuous map between CW-complexes is homotopic to a cellular map.
pub fn cellular_approximation_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "Y",
            htpy_ext_type0(),
            htpy_ext_arrow(
                htpy_ext_arrow(htpy_ext_bvar(1), htpy_ext_bvar(0)),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// `BrownRepresentabilityThm : ∀ (F : CW_op → Set), IsCohomological F → Type`
///
/// Brown representability: any cohomology theory on CW-complexes is representable.
pub fn brown_representability_thm_ty() -> Expr {
    htpy_ext_impl_pi(
        "F",
        htpy_ext_arrow(htpy_ext_type0(), htpy_ext_type0()),
        htpy_ext_arrow(
            htpy_ext_app(htpy_ext_cst("IsCohomological"), htpy_ext_bvar(0)),
            htpy_ext_type0(),
        ),
    )
}
/// `EilenbergMacLaneSpace : ∀ (G : Type) (n : Nat), Type`
///
/// An Eilenberg-MacLane space K(G, n): has πₙ ≅ G, all other homotopy trivial.
pub fn eilenberg_maclane_space_ty() -> Expr {
    htpy_ext_impl_pi(
        "G",
        htpy_ext_type0(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `HomotopyExcision : ∀ (A B C : Type) (m n : Nat), Prop`
///
/// Homotopy excision (Blakers-Massey): connectivity of homotopy pushout map.
pub fn homotopy_excision_ty() -> Expr {
    htpy_ext_impl_pi(
        "A",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "B",
            htpy_ext_type0(),
            htpy_ext_impl_pi(
                "C",
                htpy_ext_type0(),
                htpy_ext_arrow(
                    htpy_ext_nat_ty(),
                    htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_prop()),
                ),
            ),
        ),
    )
}
/// `HurewiczIsomorphism : ∀ (X : Type) (n : Nat), IsHighlyConnected X → Prop`
///
/// Hurewicz theorem: πₙ(X) → Hₙ(X; ℤ) is an isomorphism when X is (n-1)-connected.
pub fn hurewicz_isomorphism_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(
            htpy_ext_nat_ty(),
            htpy_ext_arrow(
                htpy_ext_app(htpy_ext_cst("IsHighlyConnected"), htpy_ext_bvar(1)),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// `JHomomorphism : ∀ (n : Nat), (πₙ(SO) → πₙˢ) → Prop`
///
/// The J-homomorphism relates homotopy groups of SO to stable homotopy of spheres.
pub fn j_homomorphism_ty() -> Expr {
    htpy_ext_arrow(
        htpy_ext_nat_ty(),
        htpy_ext_arrow(
            htpy_ext_arrow(
                htpy_ext_app(htpy_ext_cst("PiN"), htpy_ext_cst("SO")),
                htpy_ext_app(htpy_ext_cst("StablePi"), htpy_ext_bvar(1)),
            ),
            htpy_ext_prop(),
        ),
    )
}
/// `StableHomotopySpheres : ∀ (k : Nat), Type`
///
/// The k-th stable homotopy group of spheres πₖˢ = colim πₙ₊ₖ(Sⁿ).
pub fn stable_homotopy_spheres_ty() -> Expr {
    htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0())
}
/// `AdamsSpectralSequence : ∀ (X Y : Type) (s t : Nat), Type`
///
/// Adams spectral sequence Ext^{s,t}_{A}(H*(Y), H*(X)) ⟹ πˢ_{t-s}(Y, X).
pub fn adams_spectral_sequence_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "Y",
            htpy_ext_type0(),
            htpy_ext_arrow(
                htpy_ext_nat_ty(),
                htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
            ),
        ),
    )
}
/// `ChromaticLayer : ∀ (n p : Nat), Type`
///
/// Chromatic homotopy layer at height n and prime p: Morava K-theory K(n).
pub fn chromatic_layer_ty() -> Expr {
    htpy_ext_arrow(
        htpy_ext_nat_ty(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `MoravaKTheory : ∀ (n p : Nat), Type`
///
/// The Morava K-theory spectrum K(n) at height n and prime p.
pub fn morava_k_theory_ty() -> Expr {
    htpy_ext_arrow(
        htpy_ext_nat_ty(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `BottPeriodicityStable : Prop`
///
/// Bott periodicity in stable homotopy: π_{n+2}(U) ≅ πₙ(U).
pub fn bott_periodicity_stable_ty() -> Expr {
    htpy_ext_prop()
}
/// `SullivanMinimalModelAxiom : ∀ (X : Type), Type`
///
/// The Sullivan minimal model M(X): a minimal CDGA quasi-isomorphic to A_PL(X).
pub fn sullivan_minimal_model_axiom_ty() -> Expr {
    htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_type0())
}
/// `RationalHomotopyType : ∀ (X : Type), Type`
///
/// The rationalization X_ℚ of a simply connected space X.
pub fn rational_homotopy_type_ty() -> Expr {
    htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_type0())
}
/// `LocalizationAtPrime : ∀ (X : Type) (p : Nat), Type`
///
/// The p-localization X_{(p)}: inverting all primes except p.
pub fn localization_at_prime_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `PCompletion : ∀ (X : Type) (p : Nat), Type`
///
/// The p-completion X^∧_p (Bousfield-Kan completion).
pub fn p_completion_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
    )
}
/// `NilpotentSpace : ∀ (X : Type), Prop`
///
/// X is nilpotent: π₁(X) is nilpotent and acts nilpotently on higher πₙ(X).
pub fn nilpotent_space_ty() -> Expr {
    htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_prop())
}
/// `RationalHurewicz : ∀ (X : Type) (n : Nat), Prop`
///
/// Rational Hurewicz: πₙ(X) ⊗ ℚ ≅ Hₙ(X; ℚ) for simply connected X.
pub fn rational_hurewicz_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_prop()),
    )
}
/// `SpectralSequenceConvergence : ∀ (E : BigradeType), Prop`
///
/// A spectral sequence converges when pages stabilize to E_∞.
pub fn spectral_sequence_convergence_ty() -> Expr {
    htpy_ext_impl_pi(
        "E",
        htpy_ext_arrow(
            htpy_ext_nat_ty(),
            htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_type0()),
        ),
        htpy_ext_prop(),
    )
}
/// `LoopSpaceDelooping : ∀ (X : Type), ∃ BX, ΩBX ≃ X`
///
/// Delooping: a grouplike A_∞ space has a delooping BX with ΩBX ≃ X.
pub fn loop_space_delooping_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_app(
            htpy_ext_cst("Exists"),
            htpy_ext_arrow(htpy_ext_bvar(0), htpy_ext_prop()),
        ),
    )
}
/// `InfiniteLoopSpace : ∀ (X : Type), Prop`
///
/// X is an infinite loop space: X ≃ ΩX₁ ≃ Ω²X₂ ≃ ⋯
pub fn infinite_loop_space_ty() -> Expr {
    htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_prop())
}
/// `SpectrumType : Type`
///
/// A spectrum: a sequence of pointed spaces with structure maps ΣXₙ → Xₙ₊₁.
pub fn spectrum_type_ty() -> Expr {
    htpy_ext_type0()
}
/// `SuspensionSpectrumOf : ∀ (X : Type), Type`
///
/// The suspension spectrum Σ^∞ X associated to a space X.
pub fn suspension_spectrum_of_ty() -> Expr {
    htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_type0())
}
/// `SmashProduct : ∀ (X Y : Type), Type`
///
/// Smash product X ∧ Y = X × Y / (X ∨ Y).
pub fn smash_product_ty() -> Expr {
    htpy_ext_impl_pi(
        "X",
        htpy_ext_type0(),
        htpy_ext_impl_pi("Y", htpy_ext_type0(), htpy_ext_type0()),
    )
}
/// `TateSpectrum : ∀ (G X : Type), Type`
///
/// The Tate spectrum X^{tG} for a spectrum with G-action.
pub fn tate_spectrum_ty() -> Expr {
    htpy_ext_impl_pi(
        "G",
        htpy_ext_type0(),
        htpy_ext_impl_pi("X", htpy_ext_type0(), htpy_ext_type0()),
    )
}
/// `KTheorySpectrum : Prop`
///
/// Complex K-theory KU as a ring spectrum satisfying Bott periodicity.
pub fn k_theory_spectrum_ty() -> Expr {
    htpy_ext_prop()
}
/// `CobordismSpectrum : Prop`
///
/// Complex cobordism spectrum MU.
pub fn cobordism_spectrum_ty() -> Expr {
    htpy_ext_prop()
}
/// `ThomIsomorphism : ∀ (E B : Type) (ξ : VectorBundle E B), Prop`
///
/// Thom isomorphism: H̃^{n+k}(Th(ξ)) ≅ Hⁿ(B) for a rank-k vector bundle ξ.
pub fn thom_isomorphism_ty() -> Expr {
    htpy_ext_impl_pi(
        "E",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "B",
            htpy_ext_type0(),
            htpy_ext_arrow(
                htpy_ext_app2(
                    htpy_ext_cst("VectorBundle"),
                    htpy_ext_bvar(1),
                    htpy_ext_bvar(0),
                ),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// `PontryaginThom : ∀ (M : Type) (n k : Nat), Prop`
///
/// Pontryagin-Thom construction: framed bordism Ωⁿ_fr ≅ πₙˢ.
pub fn pontryagin_thom_ty() -> Expr {
    htpy_ext_arrow(
        htpy_ext_type0(),
        htpy_ext_arrow(
            htpy_ext_nat_ty(),
            htpy_ext_arrow(htpy_ext_nat_ty(), htpy_ext_prop()),
        ),
    )
}
/// `EinfRingSpectrum : ∀ (R : Type), Prop`
///
/// An E_∞ ring spectrum: a commutative monoid in the ∞-category of spectra.
pub fn einf_ring_spectrum_ty() -> Expr {
    htpy_ext_impl_pi("R", htpy_ext_type0(), htpy_ext_prop())
}
/// `EquivariantHomotopy : ∀ (G X : Type), GSpace G X → Prop`
///
/// Equivariant homotopy theory: homotopy for spaces with a group action.
pub fn equivariant_homotopy_ty() -> Expr {
    htpy_ext_impl_pi(
        "G",
        htpy_ext_type0(),
        htpy_ext_impl_pi(
            "X",
            htpy_ext_type0(),
            htpy_ext_arrow(
                htpy_ext_app2(htpy_ext_cst("GSpace"), htpy_ext_bvar(1), htpy_ext_bvar(0)),
                htpy_ext_prop(),
            ),
        ),
    )
}
/// Register all extended homotopy theory axioms.
pub fn register_homotopy_theory_ext(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("HigherHomotopyGroup", higher_homotopy_group_ty()),
        ("AbelnessHigherGroups", abelness_higher_groups_ty()),
        ("HomotopyLongExactSeq", homotopy_long_exact_seq_ty()),
        ("FreudenthalSuspThm", freudenthal_susp_thm_ty()),
        ("HopfFibration", hopf_fibration_ty()),
        ("SerreSpectralSequence", serre_spectral_sequence_ty()),
        ("PostnikovTruncation", postnikov_truncation_ty()),
        ("WhiteheadEquivCriteria", whitehead_equiv_criteria_ty()),
        ("CellularApproximation", cellular_approximation_ty()),
        ("BrownRepresentabilityThm", brown_representability_thm_ty()),
        ("EilenbergMacLaneSpace", eilenberg_maclane_space_ty()),
        ("HomotopyExcision", homotopy_excision_ty()),
        ("HurewiczIsomorphism", hurewicz_isomorphism_ty()),
        ("JHomomorphism", j_homomorphism_ty()),
        ("StableHomotopySpheres", stable_homotopy_spheres_ty()),
        ("AdamsSpectralSequence", adams_spectral_sequence_ty()),
        ("ChromaticLayer", chromatic_layer_ty()),
        ("MoravaKTheory", morava_k_theory_ty()),
        ("BottPeriodicityStable", bott_periodicity_stable_ty()),
        (
            "SullivanMinimalModelAxiom",
            sullivan_minimal_model_axiom_ty(),
        ),
        ("RationalHomotopyType", rational_homotopy_type_ty()),
        ("LocalizationAtPrime", localization_at_prime_ty()),
        ("PCompletion", p_completion_ty()),
        ("NilpotentSpace", nilpotent_space_ty()),
        ("RationalHurewicz", rational_hurewicz_ty()),
        (
            "SpectralSequenceConvergence",
            spectral_sequence_convergence_ty(),
        ),
        ("LoopSpaceDelooping", loop_space_delooping_ty()),
        ("InfiniteLoopSpace", infinite_loop_space_ty()),
        ("SpectrumType", spectrum_type_ty()),
        ("SuspensionSpectrumOf", suspension_spectrum_of_ty()),
        ("SmashProduct", smash_product_ty()),
        ("TateSpectrum", tate_spectrum_ty()),
        ("KTheorySpectrum", k_theory_spectrum_ty()),
        ("CobordismSpectrum", cobordism_spectrum_ty()),
        ("ThomIsomorphism", thom_isomorphism_ty()),
        ("PontryaginThom", pontryagin_thom_ty()),
        ("EinfRingSpectrum", einf_ring_spectrum_ty()),
        ("EquivariantHomotopy", equivariant_homotopy_ty()),
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
