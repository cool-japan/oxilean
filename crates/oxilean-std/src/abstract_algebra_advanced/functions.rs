//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{CliffordAlgebra, ExteriorAlgebra};

pub(super) fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub(super) fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
pub(super) fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub(super) fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub(super) fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub(super) fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `Field : Type → Prop`
///
/// Predicate asserting that a type F carries the structure of a field:
/// commutative ring with every nonzero element invertible.
pub fn field_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FieldExtension : Type → Type → Prop`
///
/// `FieldExtension K L` holds when K and L are fields and there is a
/// field embedding ι : K → L making L a K-algebra.
/// This is the fundamental relation L/K.
pub fn field_extension_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `ExtensionDegree : ∀ (K L : Type), FieldExtension K L → Nat`
///
/// The degree \[L : K\] of a field extension, i.e., the dimension of L
/// as a K-vector space. Finite degree ↔ algebraic iff L is algebraic over K
/// when both sides are number fields.
pub fn extension_degree_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), nat_ty()),
        ),
    )
}
/// `AlgebraicElement : ∀ (K L : Type), L → Prop`
///
/// An element α ∈ L is algebraic over K if it is a root of a nonzero polynomial
/// with coefficients in K.
pub fn algebraic_element_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(BinderInfo::Default, "L", type0(), arrow(bvar(0), prop())),
    )
}
/// `AlgebraicExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is algebraic if every element of L is algebraic over K.
pub fn algebraic_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `SeparableExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is separable if the minimal polynomial of every element of L over K
/// has no repeated roots (equivalently, is separable).
pub fn separable_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `NormalExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is normal if L is a splitting field of some polynomial over K,
/// equivalently if every irreducible polynomial in K\[X\] that has one root
/// in L splits completely in L.
pub fn normal_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `GaloisExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// L/K is Galois if it is both normal and separable.
pub fn galois_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `Polynomial : Type → Type`
///
/// The type of polynomials over a coefficient ring R: `R\[X\]`.
pub fn polynomial_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SplittingField : ∀ (K : Type) (p : Polynomial K), Type`
///
/// The splitting field of polynomial p over K: the smallest extension of K
/// over which p factors completely into linear factors.
pub fn splitting_field_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        arrow(app(cst("Polynomial"), bvar(0)), type0()),
    )
}
/// `SplittingFieldUniversal : ∀ (K : Type) (p : Polynomial K),
///     FieldExtension K (SplittingField K p)`
///
/// The splitting field of p over K is indeed a field extension of K.
pub fn splitting_field_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            app(cst("Polynomial"), bvar(0)),
            app2(
                cst("FieldExtension"),
                bvar(1),
                app2(cst("SplittingField"), bvar(2), bvar(0)),
            ),
        ),
    )
}
/// `PolynomialSplits : ∀ (K L : Type) (p : Polynomial K), Prop`
///
/// p splits completely in L if every root of p lies in L.
pub fn polynomial_splits_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app(cst("Polynomial"), bvar(1)), prop()),
        ),
    )
}
/// `MinimalPolynomial : ∀ (K L : Type), L → Polynomial K`
///
/// The minimal polynomial of an algebraic element α ∈ L over K:
/// the unique monic irreducible polynomial in K\[X\] of which α is a root.
pub fn minimal_polynomial_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(bvar(0), app(cst("Polynomial"), bvar(1))),
        ),
    )
}
/// `FieldAut : ∀ (K L : Type), Type`
///
/// Field automorphisms of L fixing K pointwise: `Aut_K(L)`.
pub fn field_aut_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `GaloisGroup : ∀ (K L : Type), FieldExtension K L → Type`
///
/// The Galois group `Gal(L/K)` = `Aut_K(L)`: the group of all
/// K-automorphisms of L. This is a finite group when L/K is a finite Galois extension.
pub fn galois_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `GaloisGroupOrder : ∀ (K L : Type) (h : GaloisExtension K L),
///     |Gal(L/K)| = \[L:K\]`
///
/// For a finite Galois extension, the order of the Galois group equals
/// the degree of the extension.
pub fn galois_group_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("GaloisExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `FixedField : ∀ (K L : Type) (H : Subgroup (GaloisGroup K L ext)), Type`
///
/// The fixed field L^H of a subgroup H of the Galois group:
/// all elements of L fixed by every automorphism in H.
pub fn fixed_field_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(BinderInfo::Default, "L", type0(), arrow(type0(), type0())),
    )
}
/// `FundamentalTheoremGalois : ∀ (K L : Type) (h : GaloisExtension K L), Prop`
///
/// The fundamental theorem of Galois theory states:
/// There is an order-reversing bijection between the lattice of subgroups of Gal(L/K)
/// and the lattice of intermediate fields K ⊆ E ⊆ L.
/// - H ↦ L^H (fixed field)
/// - E ↦ Gal(L/E) (restriction)
/// Moreover: \[L : L^H\] = |H| and \[L^H : K\] = \[G : H\].
pub fn fundamental_theorem_galois_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("GaloisExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `GaloisCorrespondence : ∀ (K L : Type), GaloisExtension K L → Type`
///
/// The explicit Galois correspondence as a type: a type packaging the
/// order-reversing bijection between subgroups and intermediate fields.
pub fn galois_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("GaloisExtension"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `SolvableByRadicals : ∀ (K : Type) (p : Polynomial K), Prop`
///
/// A polynomial p is solvable by radicals over K if its splitting field
/// can be obtained by a tower of radical extensions (Galois criterion:
/// iff Gal(p/K) is a solvable group).
pub fn solvable_by_radicals_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        arrow(app(cst("Polynomial"), bvar(0)), prop()),
    )
}
/// `AbelExtension : ∀ (K L : Type), FieldExtension K L → Prop`
///
/// An extension is abelian if it is Galois and the Galois group is abelian.
pub fn abel_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            type0(),
            arrow(app2(cst("FieldExtension"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `SylowSubgroup : ∀ (G : Type) (p : Nat), Type`
///
/// A Sylow p-subgroup of G: a maximal p-subgroup, i.e., a subgroup of order p^k
/// where p^k is the largest power of p dividing |G|.
pub fn sylow_subgroup_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `SylowExistence : ∀ (G : Type) (p : Nat),
///     IsGroup G → IsPrime p → ∃ (P : SylowSubgroup G p), True`
///
/// First Sylow theorem: every finite group has a Sylow p-subgroup for each prime p.
pub fn sylow_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            arrow(
                app(cst("IsGroup"), bvar(1)),
                arrow(app(cst("IsPrime"), bvar(1)), prop()),
            ),
        ),
    )
}
/// `SylowConjugacy : ∀ (G : Type) (p : Nat)
///     (P Q : SylowSubgroup G p), ∃ g : G, P = g P g⁻¹`
///
/// Second Sylow theorem: any two Sylow p-subgroups are conjugate.
pub fn sylow_conjugacy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "p",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "P",
                app2(cst("SylowSubgroup"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "Q",
                    app2(cst("SylowSubgroup"), bvar(2), bvar(1)),
                    prop(),
                ),
            ),
        ),
    )
}
/// `SylowCount : ∀ (G : Type) (p : Nat), Nat`
///
/// The number n_p of Sylow p-subgroups of G.
/// Third Sylow theorem: n_p ≡ 1 (mod p) and n_p divides \[G : P\].
pub fn sylow_count_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), nat_ty()))
}
/// `SylowThirdTheorem : ∀ (G : Type) (p : Nat),
///     SylowCount G p ≡ 1 (mod p)`
///
/// Third Sylow theorem: the number of Sylow p-subgroups is congruent to 1 mod p.
pub fn sylow_third_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(BinderInfo::Default, "p", nat_ty(), prop()),
    )
}
/// `SylowNormal : ∀ (G : Type) (p : Nat), Prop`
///
/// The unique Sylow p-subgroup (when n_p = 1) is normal in G.
pub fn sylow_normal_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// `CompositionSeries : Type → Type`
///
/// A composition series of G: a finite chain
/// `1 = G_0 ◁ G_1 ◁ ... ◁ G_n = G`
/// where each G_{i+1}/G_i is simple.
pub fn composition_series_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CompositionFactors : ∀ (G : Type), CompositionSeries G → List Type`
///
/// The composition factors G_{i+1}/G_i of a composition series.
/// These are simple groups.
pub fn composition_factors_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(
            app(cst("CompositionSeries"), bvar(0)),
            app(cst("List"), type0()),
        ),
    )
}
/// `JordanHolder : ∀ (G : Type)
///     (s1 s2 : CompositionSeries G), Prop`
///
/// The Jordan-Hölder theorem: any two composition series of G have the same
/// length and the same composition factors (up to reordering and isomorphism).
pub fn jordan_holder_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "s1",
            app(cst("CompositionSeries"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s2",
                app(cst("CompositionSeries"), bvar(1)),
                prop(),
            ),
        ),
    )
}
/// `SimpleGroup : Type → Prop`
///
/// G is a simple group if its only normal subgroups are 1 and G itself.
pub fn simple_group_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Solvable : Type → Prop`
///
/// A group G is solvable if it has a subnormal series with abelian factors:
/// `1 = G_0 ◁ G_1 ◁ ... ◁ G_n = G` with each G_{i+1}/G_i abelian.
pub fn solvable_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Nilpotent : Type → Prop`
///
/// A group G is nilpotent if its lower central series terminates at 1.
/// Nilpotent ⇒ Solvable.
pub fn nilpotent_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DerivedSeries : Type → List Type`
///
/// The derived series G ⊇ G' ⊇ G'' ⊇ ...
/// where G^(n+1) = \[G^(n), G^(n)\]. G is solvable iff G^(n) = 1 for some n.
pub fn derived_series_ty() -> Expr {
    arrow(type0(), app(cst("List"), type0()))
}
/// `LowerCentralSeries : Type → List Type`
///
/// The lower central series γ_1(G) ⊇ γ_2(G) ⊇ ...
/// where γ_{n+1}(G) = \[G, γ_n(G)\]. G is nilpotent iff γ_n(G) = 1 for some n.
pub fn lower_central_series_ty() -> Expr {
    arrow(type0(), app(cst("List"), type0()))
}
/// `FreeGroup : Type → Type`
///
/// The free group on a generating set S: F(S).
/// Elements are equivalence classes of words in S ∪ S⁻¹ under free reduction.
pub fn free_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FreeGroupHom : ∀ {S : Type} {G : Type},
///     IsGroup G → (S → G) → FreeGroup S → G`
///
/// Universal property of free groups: a set map S → G extends uniquely to a
/// group homomorphism F(S) → G.
pub fn free_group_hom_ty() -> Expr {
    impl_pi(
        "S",
        type0(),
        impl_pi(
            "G",
            type0(),
            arrow(
                app(cst("IsGroup"), bvar(0)),
                arrow(
                    arrow(bvar(1), bvar(1)),
                    arrow(app(cst("FreeGroup"), bvar(2)), bvar(2)),
                ),
            ),
        ),
    )
}
/// `FreeGroupUniversal : ∀ (S G : Type) (h : IsGroup G) (f : S → G),
///     ∃! φ : FreeGroup S → G, IsGroupHom φ ∧ φ ∘ ι = f`
///
/// The free group is characterized by its universal property: unique extension.
pub fn free_group_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            type0(),
            arrow(
                app(cst("IsGroup"), bvar(0)),
                arrow(arrow(bvar(1), bvar(1)), prop()),
            ),
        ),
    )
}
/// `GroupPresentation : Type → Type → Type`
///
/// A group presentation ⟨S | R⟩ with generators S and relations R ⊆ F(S):
/// `G ≅ FreeGroup S / NormalClosure R`.
pub fn group_presentation_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `RelatorSet : Type → Type → Type`
///
/// The set of relators in a presentation ⟨S | R⟩:
/// elements of F(S) that are sent to the identity in G.
pub fn relator_set_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `WordProblem : ∀ (S R : Type), GroupPresentation S R → Type`
///
/// The word problem for a presented group ⟨S | R⟩:
/// decide whether two words in F(S) represent the same element of G.
/// Undecidable in general (Novikov 1955), decidable for specific classes.
pub fn word_problem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            type0(),
            arrow(app2(cst("GroupPresentation"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `NormalClosure : ∀ (G : Type), Set G → Type`
///
/// The normal closure of a subset R in G: the smallest normal subgroup
/// of G containing R.
pub fn normal_closure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("Set"), bvar(0)), type0()),
    )
}
/// `AmalgramatedProduct : Type → Type → Type → Type`
///
/// Amalgamated free product: G₁ *_H G₂ where H embeds into both G₁ and G₂.
pub fn amalgamated_product_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// `HNNExtension : Type → Type → Type`
///
/// HNN extension G*_{φ} where φ : H → K is an isomorphism between
/// subgroups H, K ≤ G.
pub fn hnn_extension_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `FinitelyGeneratedAbelianGroup : Type → Prop`
///
/// Predicate: G is a finitely generated abelian group.
pub fn finitely_generated_abelian_group_ty() -> Expr {
    arrow(type0(), prop())
}
/// `AbelianGroupInvariantFactors : Type → List Nat`
///
/// The invariant factor decomposition of a finitely generated abelian group:
/// G ≅ ℤ/d₁ × ℤ/d₂ × ... × ℤ/dₖ × ℤʳ
/// where d₁ | d₂ | ... | dₖ.
pub fn abelian_group_invariant_factors_ty() -> Expr {
    arrow(type0(), app(cst("List"), nat_ty()))
}
/// `AbelianGroupElementaryDivisors : Type → List Nat`
///
/// The elementary divisor decomposition of a finitely generated abelian group:
/// G ≅ ℤ/p₁^{e₁} × ... × ℤ/pₙ^{eₙ} × ℤʳ
/// where the pᵢ are primes (not necessarily distinct).
pub fn abelian_group_elementary_divisors_ty() -> Expr {
    arrow(type0(), app(cst("List"), nat_ty()))
}
/// `AbelianGroupRank : Type → Nat`
///
/// The free rank (torsion-free rank) r of a finitely generated abelian group.
pub fn abelian_group_rank_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `ClassificationFGAG : ∀ (G : Type), FinitelyGeneratedAbelianGroup G →
///     ∃ (r : Nat) (ds : List Nat), IsStructure G r ds`
///
/// The classification theorem for finitely generated abelian groups:
/// G is isomorphic to ℤʳ ⊕ ⊕ᵢ ℤ/dᵢ with d₁ | d₂ | ... | dₖ.
pub fn classification_fg_ag_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("FinitelyGeneratedAbelianGroup"), bvar(0)), prop()),
    )
}
/// `TorsionSubgroup : Type → Type`
///
/// The torsion subgroup of an abelian group G: all elements of finite order.
pub fn torsion_subgroup_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PrimaryComponent : Type → Nat → Type`
///
/// The p-primary component of an abelian group G:
/// { g ∈ G | pⁿg = 0 for some n ≥ 0 }.
pub fn primary_component_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `PID : Type → Prop`
///
/// Predicate asserting that R is a principal ideal domain:
/// an integral domain where every ideal is principal.
pub fn pid_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IntegralDomain : Type → Prop`
///
/// Predicate: R is an integral domain (commutative ring, no zero divisors).
pub fn integral_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FinGenModulePID : ∀ (R : Type), Type`
///
/// Type of finitely generated modules over a PID R.
pub fn fin_gen_module_pid_ty() -> Expr {
    arrow(type0(), type0())
}
/// `StructureTheoremPID : ∀ (R M : Type) (hR : PID R)
///     (hM : FinGenModule R M), Prop`
///
/// The structure theorem for finitely generated modules over a PID:
/// M ≅ Rʳ ⊕ R/⟨d₁⟩ ⊕ ... ⊕ R/⟨dₖ⟩
/// where d₁ | d₂ | ... | dₖ are the invariant factors of M.
pub fn structure_theorem_pid_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            arrow(
                app(cst("PID"), bvar(1)),
                arrow(app2(cst("FinGenModulePID"), bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `InvariantFactors : ∀ (R M : Type), List R`
///
/// The invariant factors d₁ | d₂ | ... | dₖ of a finitely generated R-module M.
pub fn invariant_factors_ty() -> Expr {
    arrow(type0(), arrow(type0(), app(cst("List"), bvar(1))))
}
/// `FreeRank : ∀ (R M : Type), Nat`
///
/// The free rank of a finitely generated module M over a PID R.
pub fn free_rank_ty() -> Expr {
    arrow(type0(), arrow(type0(), nat_ty()))
}
/// `TorsionModule : ∀ (R : Type), FinGenModulePID R → Prop`
///
/// A finitely generated module M over a PID is torsion if every element
/// is annihilated by some nonzero r ∈ R.
pub fn torsion_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(app(cst("FinGenModulePID"), bvar(0)), prop()),
    )
}
/// `ElementaryDivisors : ∀ (R M : Type), List R`
///
/// The elementary divisors (primary decomposition) of a torsion module over a PID.
pub fn elementary_divisors_ty() -> Expr {
    arrow(type0(), arrow(type0(), app(cst("List"), bvar(1))))
}
/// `SNF : ∀ (R : Type) (m n : Nat), Matrix R m n → Matrix R m n`
///
/// Smith normal form: for any matrix A over a PID there exist invertible
/// P, Q such that PAQ is diagonal with d₁ | d₂ | ... | dₘᵢₙ(m,n).
pub fn snf_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("Matrix"), bvar(3), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// `EuclideanDomain : Type → Prop`
///
/// An Euclidean domain: an integral domain with a Euclidean function
/// ν : R\{0} → ℕ satisfying the division algorithm.
/// EuclideanDomain ⇒ PID ⇒ UFD ⇒ IntegralDomain.
pub fn euclidean_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// `UFD : Type → Prop`
///
/// Unique factorisation domain: every nonzero non-unit element factors
/// uniquely (up to order and associates) into irreducibles.
pub fn ufd_ty() -> Expr {
    arrow(type0(), prop())
}
/// `GCD : ∀ (R : Type), R → R → R`
///
/// Greatest common divisor in a GCD domain.
pub fn gcd_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(bvar(0), arrow(bvar(1), bvar(1))),
    )
}
/// `BezoutDomain : Type → Prop`
///
/// A Bezout domain: a domain where every finitely generated ideal is principal.
/// PIDs are Bezout + Noetherian.
pub fn bezout_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DedekindDomain : Type → Prop`
///
/// A Dedekind domain: a Noetherian integrally closed domain of Krull dimension 1.
/// Algebraic integers in a number field form a Dedekind domain.
pub fn dedekind_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// `IdealClassGroup : Type → Type`
///
/// The ideal class group Cl(R) of a Dedekind domain R:
/// fractional ideals modulo principal ideals.
/// R is a PID iff Cl(R) is trivial.
pub fn ideal_class_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// Register all advanced abstract algebra axioms into the kernel environment.
#[allow(clippy::too_many_arguments)]
pub fn register_abstract_algebra_advanced(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Field", field_ty()),
        ("FieldExtension", field_extension_ty()),
        ("ExtensionDegree", extension_degree_ty()),
        ("AlgebraicElement", algebraic_element_ty()),
        ("AlgebraicExtension", algebraic_extension_ty()),
        ("SeparableExtension", separable_extension_ty()),
        ("NormalExtension", normal_extension_ty()),
        ("GaloisExtension", galois_extension_ty()),
        ("Polynomial", polynomial_ty()),
        ("SplittingField", splitting_field_ty()),
        ("SplittingFieldUniversal", splitting_field_universal_ty()),
        ("PolynomialSplits", polynomial_splits_ty()),
        ("MinimalPolynomial", minimal_polynomial_ty()),
        ("FieldAut", field_aut_ty()),
        ("GaloisGroup", galois_group_ty()),
        ("GaloisGroupOrder", galois_group_order_ty()),
        ("FixedField", fixed_field_ty()),
        ("FundamentalTheoremGalois", fundamental_theorem_galois_ty()),
        ("GaloisCorrespondence", galois_correspondence_ty()),
        ("SolvableByRadicals", solvable_by_radicals_ty()),
        ("AbelExtension", abel_extension_ty()),
        ("SylowSubgroup", sylow_subgroup_ty()),
        ("SylowExistence", sylow_existence_ty()),
        ("SylowConjugacy", sylow_conjugacy_ty()),
        ("SylowCount", sylow_count_ty()),
        ("SylowThirdTheorem", sylow_third_theorem_ty()),
        ("SylowNormal", sylow_normal_ty()),
        ("CompositionSeries", composition_series_ty()),
        ("CompositionFactors", composition_factors_ty()),
        ("JordanHolder", jordan_holder_ty()),
        ("SimpleGroup", simple_group_ty()),
        ("Solvable", solvable_ty()),
        ("Nilpotent", nilpotent_ty()),
        ("DerivedSeries", derived_series_ty()),
        ("LowerCentralSeries", lower_central_series_ty()),
        ("FreeGroup", free_group_ty()),
        ("FreeGroupHom", free_group_hom_ty()),
        ("FreeGroupUniversal", free_group_universal_ty()),
        ("GroupPresentation", group_presentation_ty()),
        ("RelatorSet", relator_set_ty()),
        ("WordProblem", word_problem_ty()),
        ("NormalClosure", normal_closure_ty()),
        ("AmalgramatedProduct", amalgamated_product_ty()),
        ("HNNExtension", hnn_extension_ty()),
        (
            "FinitelyGeneratedAbelianGroup",
            finitely_generated_abelian_group_ty(),
        ),
        (
            "AbelianGroupInvariantFactors",
            abelian_group_invariant_factors_ty(),
        ),
        (
            "AbelianGroupElementaryDivisors",
            abelian_group_elementary_divisors_ty(),
        ),
        ("AbelianGroupRank", abelian_group_rank_ty()),
        ("ClassificationFGAG", classification_fg_ag_ty()),
        ("TorsionSubgroup", torsion_subgroup_ty()),
        ("PrimaryComponent", primary_component_ty()),
        ("PID", pid_ty()),
        ("IntegralDomain", integral_domain_ty()),
        ("FinGenModulePID", fin_gen_module_pid_ty()),
        ("StructureTheoremPID", structure_theorem_pid_ty()),
        ("InvariantFactors", invariant_factors_ty()),
        ("FreeRank", free_rank_ty()),
        ("TorsionModule", torsion_module_ty()),
        ("ElementaryDivisors", elementary_divisors_ty()),
        ("SNF", snf_ty()),
        ("EuclideanDomain", euclidean_domain_ty()),
        ("UFD", ufd_ty()),
        ("GCD", gcd_ty()),
        ("BezoutDomain", bezout_domain_ty()),
        ("DedekindDomain", dedekind_domain_ty()),
        ("IdealClassGroup", ideal_class_group_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        register_abstract_algebra_advanced(&mut env);
        env
    }
    #[test]
    fn test_galois_theory_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FieldExtension")).is_some());
        assert!(env.get(&Name::str("GaloisGroup")).is_some());
        assert!(env.get(&Name::str("FundamentalTheoremGalois")).is_some());
        assert!(env.get(&Name::str("GaloisCorrespondence")).is_some());
        assert!(env.get(&Name::str("SolvableByRadicals")).is_some());
    }
    #[test]
    fn test_splitting_field_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Polynomial")).is_some());
        assert!(env.get(&Name::str("SplittingField")).is_some());
        assert!(env.get(&Name::str("SplittingFieldUniversal")).is_some());
        assert!(env.get(&Name::str("MinimalPolynomial")).is_some());
    }
    #[test]
    fn test_sylow_theorems_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("SylowSubgroup")).is_some());
        assert!(env.get(&Name::str("SylowExistence")).is_some());
        assert!(env.get(&Name::str("SylowConjugacy")).is_some());
        assert!(env.get(&Name::str("SylowCount")).is_some());
        assert!(env.get(&Name::str("SylowThirdTheorem")).is_some());
        assert!(env.get(&Name::str("SylowNormal")).is_some());
    }
    #[test]
    fn test_jordan_holder_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("CompositionSeries")).is_some());
        assert!(env.get(&Name::str("CompositionFactors")).is_some());
        assert!(env.get(&Name::str("JordanHolder")).is_some());
        assert!(env.get(&Name::str("SimpleGroup")).is_some());
        assert!(env.get(&Name::str("Solvable")).is_some());
        assert!(env.get(&Name::str("Nilpotent")).is_some());
    }
    #[test]
    fn test_free_groups_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("FreeGroup")).is_some());
        assert!(env.get(&Name::str("FreeGroupHom")).is_some());
        assert!(env.get(&Name::str("FreeGroupUniversal")).is_some());
        assert!(env.get(&Name::str("GroupPresentation")).is_some());
        assert!(env.get(&Name::str("WordProblem")).is_some());
        assert!(env.get(&Name::str("NormalClosure")).is_some());
    }
    #[test]
    fn test_abelian_group_structure_registered() {
        let env = registered_env();
        assert!(env
            .get(&Name::str("FinitelyGeneratedAbelianGroup"))
            .is_some());
        assert!(env
            .get(&Name::str("AbelianGroupInvariantFactors"))
            .is_some());
        assert!(env
            .get(&Name::str("AbelianGroupElementaryDivisors"))
            .is_some());
        assert!(env.get(&Name::str("AbelianGroupRank")).is_some());
        assert!(env.get(&Name::str("ClassificationFGAG")).is_some());
        assert!(env.get(&Name::str("TorsionSubgroup")).is_some());
        assert!(env.get(&Name::str("PrimaryComponent")).is_some());
    }
    #[test]
    fn test_pid_modules_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("PID")).is_some());
        assert!(env.get(&Name::str("IntegralDomain")).is_some());
        assert!(env.get(&Name::str("FinGenModulePID")).is_some());
        assert!(env.get(&Name::str("StructureTheoremPID")).is_some());
        assert!(env.get(&Name::str("InvariantFactors")).is_some());
        assert!(env.get(&Name::str("TorsionModule")).is_some());
        assert!(env.get(&Name::str("SNF")).is_some());
    }
    #[test]
    fn test_domain_hierarchy_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("EuclideanDomain")).is_some());
        assert!(env.get(&Name::str("UFD")).is_some());
        assert!(env.get(&Name::str("GCD")).is_some());
        assert!(env.get(&Name::str("BezoutDomain")).is_some());
        assert!(env.get(&Name::str("DedekindDomain")).is_some());
        assert!(env.get(&Name::str("IdealClassGroup")).is_some());
    }
}
/// Build an `Environment` populated with all advanced abstract algebra axioms,
/// plus axioms for the new structures introduced in this module.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("GaloisFieldExists", galois_field_exists_ty),
        ("GaloisFieldUnique", galois_field_unique_ty),
        ("GaloisFieldCyclic", galois_field_cyclic_ty),
        ("CoherentRingAxiom", coherent_ring_axiom_ty),
        ("NoetherianACCAx", noetherian_acc_ty),
        ("HilbertBasisAx", hilbert_basis_ty),
        ("WedderburnArtinAx", wedderburn_artin_ty),
        ("ArtinMapAx", artin_map_ty),
        ("GlobalDimAx", global_dim_ty),
        ("LocalizationExact", localization_exact_ty),
    ];
    for (name, ty_fn) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
    register_abstract_algebra_advanced(&mut env);
    env
}
pub fn galois_field_exists_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("GaloisFieldType")))
}
pub fn galois_field_unique_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("Prop")))
}
pub fn galois_field_cyclic_ty() -> Expr {
    arrow(cst("GaloisFieldType"), cst("CyclicGroup"))
}
pub fn coherent_ring_axiom_ty() -> Expr {
    arrow(cst("Ring"), cst("Prop"))
}
pub fn noetherian_acc_ty() -> Expr {
    arrow(cst("Ring"), cst("Prop"))
}
pub fn hilbert_basis_ty() -> Expr {
    arrow(cst("Ring"), cst("Ring"))
}
pub fn wedderburn_artin_ty() -> Expr {
    arrow(cst("Ring"), cst("Prop"))
}
pub fn artin_map_ty() -> Expr {
    arrow(cst("NumberField"), cst("Prop"))
}
pub fn global_dim_ty() -> Expr {
    arrow(cst("Ring"), nat_ty())
}
pub fn localization_exact_ty() -> Expr {
    arrow(cst("Ring"), arrow(cst("MultiplicativeSet"), cst("Prop")))
}
/// `TensorAlgebra : Type → Type`
///
/// The tensor algebra T(V) = ⊕_{n≥0} V^{⊗n} over a vector space V.
/// Universal property: every linear map V → A to an algebra A
/// extends uniquely to an algebra map T(V) → A.
pub fn tensor_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TensorAlgebraUniversal : ∀ (V A : Type), IsAlgebra A → (V → A) → TensorAlgebra V → A`
///
/// Universal property of the tensor algebra.
pub fn tensor_algebra_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(cst("IsAlgebra"), bvar(0)),
                arrow(
                    arrow(bvar(1), bvar(1)),
                    arrow(app(cst("TensorAlgebra"), bvar(2)), bvar(2)),
                ),
            ),
        ),
    )
}
/// `SymmetricAlgebra : Type → Type`
///
/// The symmetric algebra S(V) = T(V) / ⟨v⊗w − w⊗v⟩:
/// the free commutative algebra on the vector space V.
/// S(V) ≅ k\[x₁, …, xₙ\] when dim V = n.
pub fn symmetric_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SymmetricAlgebraUniversal : ∀ (V A : Type), IsCommAlgebra A → (V → A) → SymmAlgebra V → A`
///
/// Universal property: S(V) is the free commutative algebra on V.
pub fn symmetric_algebra_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(cst("IsCommAlgebra"), bvar(0)),
                arrow(
                    arrow(bvar(1), bvar(1)),
                    arrow(app(cst("SymmetricAlgebra"), bvar(2)), bvar(2)),
                ),
            ),
        ),
    )
}
/// `ExteriorAlgebra : Type → Type`
///
/// The exterior (Grassmann) algebra Λ(V) = T(V) / ⟨v⊗v⟩:
/// the free alternating algebra on V. Λ^k(V) is the k-th exterior power.
pub fn exterior_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ExteriorAlgebraUniversal : ∀ (V A : Type), IsAlgebra A →
///     (∀ v : V, sq v = 0) → (V → A) → ExteriorAlgebra V → A`
///
/// Universal property: Λ(V) is the free alternating algebra on V.
pub fn exterior_algebra_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(cst("IsAlgebra"), bvar(0)),
                arrow(
                    app(cst("IsAlternating"), bvar(1)),
                    arrow(
                        arrow(bvar(2), bvar(2)),
                        arrow(app(cst("ExteriorAlgebra"), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// `WedgeProduct : ∀ (V : Type) (k l : Nat),
///     ExteriorPower V k → ExteriorPower V l → ExteriorPower V (k + l)`
///
/// The wedge product ∧ : Λ^k(V) × Λ^l(V) → Λ^(k+l)(V).
pub fn wedge_product_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "l",
                nat_ty(),
                arrow(
                    app2(cst("ExteriorPower"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("ExteriorPower"), bvar(3), bvar(1)),
                        app2(
                            cst("ExteriorPower"),
                            bvar(4),
                            app2(cst("NatAdd"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CliffordAlgebra : ∀ (V : Type), QuadraticForm V → Type`
///
/// The Clifford algebra Cl(V, Q) = T(V) / ⟨v⊗v − Q(v)·1⟩.
/// Generalises exterior algebra (Q=0) and is central to spin geometry.
pub fn clifford_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(app(cst("QuadraticForm"), bvar(0)), type0()),
    )
}
/// `CliffordAlgebraUniversal : ∀ (V : Type) (Q : QuadraticForm V) (A : Type),
///     IsAlgebra A → CliffordMap V Q A → CliffordAlgebra V Q → A`
///
/// Universal property of Clifford algebra: extends Clifford maps uniquely.
pub fn clifford_algebra_universal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "Q",
            app(cst("QuadraticForm"), bvar(0)),
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                arrow(
                    app(cst("IsAlgebra"), bvar(0)),
                    arrow(
                        app3(cst("CliffordMap"), bvar(3), bvar(2), bvar(1)),
                        arrow(app2(cst("CliffordAlgebra"), bvar(4), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// `SpinGroup : ∀ (V : Type) (Q : QuadraticForm V), Type`
///
/// The spin group Spin(V, Q) inside Cl(V, Q)^×:
/// the double cover of SO(V, Q) when char ≠ 2.
pub fn spin_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(app(cst("QuadraticForm"), bvar(0)), type0()),
    )
}
/// `HopfAlgebra : Type → Prop`
///
/// A Hopf algebra is a bialgebra equipped with an antipode S: H → H,
/// satisfying m ∘ (S ⊗ id) ∘ Δ = η ∘ ε = m ∘ (id ⊗ S) ∘ Δ.
pub fn hopf_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Comultiplication : ∀ (H : Type), HopfAlgebra H → H → H ⊗ H`
///
/// The comultiplication Δ : H → H ⊗ H satisfying coassociativity.
pub fn comultiplication_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(
            app(cst("HopfAlgebra"), bvar(0)),
            arrow(bvar(1), app2(cst("TensorProduct"), bvar(2), bvar(2))),
        ),
    )
}
/// `Counit : ∀ (H : Type), HopfAlgebra H → H → k`
///
/// The counit ε : H → k satisfying the counit axioms.
pub fn counit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(
            app(cst("HopfAlgebra"), bvar(0)),
            arrow(bvar(1), cst("BaseField")),
        ),
    )
}
/// `Antipode : ∀ (H : Type), HopfAlgebra H → H → H`
///
/// The antipode S : H → H satisfying m ∘ (S ⊗ id) ∘ Δ = η ∘ ε.
pub fn antipode_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HopfAlgebra"), bvar(0)), arrow(bvar(1), bvar(2))),
    )
}
/// `HopfAntipodeAntiHom : ∀ (H : Type), HopfAlgebra H → Prop`
///
/// The antipode of a Hopf algebra is an anti-algebra homomorphism:
/// S(xy) = S(y)S(x).
pub fn hopf_antipode_anti_hom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HopfAlgebra"), bvar(0)), prop()),
    )
}
/// `HopfAlgebrasInvolutive : ∀ (H : Type), CommHopfAlgebra H → S² = id`
///
/// For a commutative or cocommutative Hopf algebra, S² = id.
pub fn hopf_antipode_involutive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("CommHopfAlgebra"), bvar(0)), prop()),
    )
}
/// `CStarAlgebra : Type → Prop`
///
/// A C*-algebra: a Banach *-algebra satisfying ‖a*a‖ = ‖a‖².
pub fn c_star_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Spectrum : ∀ (A : Type), CStarAlgebra A → A → Type`
///
/// The spectrum σ(a) = { λ ∈ ℂ | (a − λ·1) not invertible } of an element a ∈ A.
pub fn spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("CStarAlgebra"), bvar(0)), arrow(bvar(1), type0())),
    )
}
/// `GelfandNaimark : ∀ (A : Type), CommCStarAlgebra A →
///     ∃ (X : CompactHausdorff), A ≅ C(X, ℂ)`
///
/// The commutative Gelfand-Naimark theorem: every commutative C*-algebra
/// is isomorphic to C(X, ℂ) for a compact Hausdorff space X.
pub fn gelfand_naimark_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("CommCStarAlgebra"), bvar(0)), prop()),
    )
}
/// `GNSConstruction : ∀ (A : Type), CStarAlgebra A → State A → Type`
///
/// The GNS construction: every state ω on A gives a Hilbert space H_ω
/// and a *-representation π_ω : A → B(H_ω).
pub fn gns_construction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(
            app(cst("CStarAlgebra"), bvar(0)),
            arrow(app(cst("State"), bvar(1)), type0()),
        ),
    )
}
/// `VonNeumannAlgebra : Type → Prop`
///
/// A von Neumann (W*-) algebra: a *-subalgebra of B(H) closed in the weak operator topology,
/// equivalently equal to its double commutant M = M''.
pub fn von_neumann_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DoubleCommutant : ∀ (M : Type), VonNeumannAlgebra M → M = M''`
///
/// Von Neumann's double commutant theorem: M = (M')'.
pub fn double_commutant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("VonNeumannAlgebra"), bvar(0)), prop()),
    )
}
/// `TypeClassificationVNA : ∀ (M : Type), VonNeumannAlgebra M → VNAType`
///
/// Every von Neumann algebra decomposes into types I, II, III
/// (further: Iₙ, I_∞, II₁, II_∞, III_λ for 0 ≤ λ ≤ 1).
pub fn type_classification_vna_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("VonNeumannAlgebra"), bvar(0)), cst("VNAType")),
    )
}
/// `LieAlgebra : Type → Prop`
///
/// A Lie algebra: a vector space g with a bilinear bracket \[·,·\]
/// satisfying antisymmetry and the Jacobi identity.
pub fn lie_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `UniversalEnvelopingAlgebra : ∀ (g : Type), LieAlgebra g → Type`
///
/// The universal enveloping algebra U(g): the associative algebra
/// with the universal Lie algebra map g → U(g).
pub fn universal_enveloping_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        arrow(app(cst("LieAlgebra"), bvar(0)), type0()),
    )
}
/// `PBWTheorem : ∀ (g : Type) (h : LieAlgebra g),
///     U(g) has a PBW basis indexed by ordered monomials in a basis of g`
///
/// The Poincaré-Birkhoff-Witt theorem: if {x₁,...,xₙ} is an ordered basis of g,
/// then {x₁^{i₁}···xₙ^{iₙ}} is a basis for U(g).
pub fn pbw_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        arrow(app(cst("LieAlgebra"), bvar(0)), prop()),
    )
}
/// `UEAUniversalProperty : ∀ (g A : Type), LieAlgebra g → IsAlgebra A →
///     LieAlgHom g A → AlgHom (UEA g) A`
///
/// Universal property of U(g): every Lie algebra map g → A extends to an algebra map U(g) → A.
pub fn uea_universal_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(cst("LieAlgebra"), bvar(1)),
                arrow(
                    app(cst("IsAlgebra"), bvar(1)),
                    arrow(
                        app2(cst("LieAlgHom"), bvar(3), bvar(2)),
                        app2(cst("AlgHom"), app2(cst("UEA"), bvar(4), bvar(3)), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `WeylAlgebra : Nat → Type`
///
/// The n-th Weyl algebra A_n(k) = k⟨x₁,...,xₙ,∂₁,...,∂ₙ⟩ / ⟨\[∂ᵢ,xⱼ\]=δᵢⱼ⟩.
/// This is the algebra of differential operators on polynomial rings.
pub fn weyl_algebra_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WeylSimple : ∀ (n : Nat), IsSimpleAlgebra (WeylAlgebra n)`
///
/// The Weyl algebra A_n(k) is simple for char k = 0.
pub fn weyl_simple_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(cst("IsSimpleAlgebra"), app(cst("WeylAlgebra"), bvar(0))),
    )
}
/// `OreExtension : Type → Type → Type`
///
/// The Ore extension R\[x; σ, δ\]: polynomials in x over R with twisted multiplication
/// x·r = σ(r)·x + δ(r), where σ is an endomorphism and δ is a σ-derivation.
pub fn ore_extension_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `OreCondition : ∀ (R : Type), IntegralDomain R → Prop`
///
/// The (right) Ore condition: for all a, b ∈ R with b ≠ 0,
/// there exist a', b' with b' ≠ 0 such that ab' = ba'.
/// This condition is equivalent to R having a right ring of fractions.
pub fn ore_condition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(app(cst("IntegralDomain"), bvar(0)), prop()),
    )
}
/// `SkewPolynomialRing : ∀ (R : Type) (σ : R → R), Type`
///
/// Skew polynomial ring R\[x; σ\] with rule x·r = σ(r)·x.
/// Special case of Ore extension with δ = 0.
pub fn skew_polynomial_ring_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        arrow(arrow(bvar(0), bvar(1)), type0()),
    )
}
/// `MoritaEquivalent : Type → Type → Prop`
///
/// Two rings R and S are Morita equivalent if their categories of modules
/// Mod-R and Mod-S are equivalent as categories.
/// R and Mₙ(R) are always Morita equivalent.
pub fn morita_equivalent_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `MoritaContext : Type → Type → Type`
///
/// A Morita context (R, S, P, Q, φ, ψ) witnessing Morita equivalence:
/// R-S-bimodule P, S-R-bimodule Q with balanced pairings.
pub fn morita_context_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `MatrixMoritaEquiv : ∀ (R : Type) (n : Nat), MoritaEquivalent R (Matrix R n n)`
///
/// Morita: R and M_n(R) are always Morita equivalent for any n ≥ 1.
pub fn matrix_morita_equiv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                cst("MoritaEquivalent"),
                bvar(1),
                app2(cst("MatrixRing"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `MoritaInvariant : ∀ (R S : Type), MoritaEquivalent R S → Prop`
///
/// A ring-theoretic property is Morita invariant if it is preserved under Morita equivalence.
/// Examples: being simple, semisimple, Noetherian, von Neumann regular.
pub fn morita_invariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            type0(),
            arrow(app2(cst("MoritaEquivalent"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `GroupAlgebra : Type → Type → Type`
///
/// The group algebra k\[G\]: the free k-module on G with multiplication
/// extending the group law linearly.
pub fn group_algebra_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `GroupRepresentation : ∀ (G k V : Type), IsGroup G → IsField k → IsVectorSpace k V → Prop`
///
/// A k-linear representation of G: a group homomorphism ρ : G → GL(V).
pub fn group_representation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "k",
            type0(),
            pi(
                BinderInfo::Default,
                "V",
                type0(),
                arrow(
                    app(cst("IsGroup"), bvar(2)),
                    arrow(
                        app(cst("IsField"), bvar(2)),
                        arrow(app2(cst("IsVectorSpace"), bvar(3), bvar(1)), prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `MaschkeTheorem : ∀ (G k : Type), IsFiniteGroup G → IsField k →
///     ¬ (Char k ∣ |G|) → IsSemisimple (GroupAlgebra k G)`
///
/// Maschke's theorem: k\[G\] is semisimple iff char k ∤ |G|.
pub fn maschke_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "k",
            type0(),
            arrow(
                app(cst("IsFiniteGroup"), bvar(1)),
                arrow(app(cst("IsField"), bvar(1)), prop()),
            ),
        ),
    )
}
/// `CharacterTheory : ∀ (G : Type), IsFiniteGroup G → Type`
///
/// The character ring of a finite group G: formal sums of characters χᵢ = Tr(ρᵢ).
pub fn character_theory_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("IsFiniteGroup"), bvar(0)), type0()),
    )
}
/// `KoszulAlgebra : Type → Prop`
///
/// A graded algebra A is Koszul if its minimal free resolution has a particularly
/// simple form — the Koszul complex. Koszul duality exchanges A with A^!.
pub fn koszul_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `KoszulDual : Type → Type`
///
/// The Koszul dual A^! of a Koszul algebra A.
/// For a polynomial ring k\[x₁,...,xₙ\], the Koszul dual is the exterior algebra Λ(y₁,...,yₙ).
pub fn koszul_dual_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AssociatedGradedAlgebra : ∀ (A : Type), FilteredAlgebraStr A → Type`
///
/// gr(A) = ⊕ₙ Fₙ(A)/F_{n-1}(A) as a graded algebra.
pub fn associated_graded_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("FilteredAlgebraStr"), bvar(0)), type0()),
    )
}
/// `GradedCommutativity : ∀ (A : Type), GradedAlgebraStr A → Prop`
///
/// A graded algebra is graded-commutative (supercommutative) if
/// ab = (−1)^{|a||b|} ba for homogeneous elements a, b.
pub fn graded_commutativity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(app(cst("GradedAlgebraStr"), bvar(0)), prop()),
    )
}
/// `AzumayaAlgebra : ∀ (R : Type), Type`
///
/// An Azumaya algebra over a commutative ring R: an R-algebra A that is
/// locally (étale-locally) a matrix algebra — generalises central simple algebras.
pub fn azumaya_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BrauerGroup : Type → Type`
///
/// The Brauer group Br(R) of a field or ring R:
/// Morita equivalence classes of Azumaya R-algebras under ⊗_R.
pub fn brauer_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CentralSimpleAlgebra : ∀ (k : Type), IsField k → Type`
///
/// A finite-dimensional k-algebra A that is central (Z(A) = k) and simple.
pub fn central_simple_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        type0(),
        arrow(app(cst("IsField"), bvar(0)), type0()),
    )
}
/// `BrauerGroupPeriodicity : ∀ (k : Type), IsField k → Nat`
///
/// The period of an element in the Brauer group: the order of \[A\] in Br(k).
/// Tsen's theorem: Br(k) = 0 for algebraically closed k.
pub fn brauer_period_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        type0(),
        arrow(app(cst("IsField"), bvar(0)), nat_ty()),
    )
}
/// `DGAlgebra : Type → Prop`
///
/// A differential graded algebra (DGA): a graded algebra (A, ·) with a
/// differential d of degree +1 (or −1) satisfying d² = 0 and the Leibniz rule
/// d(ab) = d(a)b + (−1)^{|a|} a d(b).
pub fn dg_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DGAlgebraMorphism : ∀ (A B : Type), DGAlgebra A → DGAlgebra B → Type`
///
/// A morphism of DGAs: a degree-0 algebra map commuting with differentials.
pub fn dg_algebra_morphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(
                app(cst("DGAlgebra"), bvar(1)),
                arrow(app(cst("DGAlgebra"), bvar(1)), type0()),
            ),
        ),
    )
}
/// `AInfinityAlgebra : Type → Prop`
///
/// An A_∞-algebra: a graded vector space A with higher multiplications
/// mₙ : A^⊗n → A of degree 2−n satisfying the Stasheff identities Σ m∘m = 0.
/// Dg-associativity is A_∞ with m_n = 0 for n ≥ 3.
pub fn a_infinity_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Operad : Type → Prop`
///
/// A (symmetric) operad P: a collection P(n) of n-ary operations with
/// symmetric group actions and composition maps satisfying associativity and unit.
pub fn operad_ty() -> Expr {
    arrow(type0(), prop())
}
/// `OperadAlgebra : ∀ (P A : Type), Operad P → Type`
///
/// A P-algebra: a vector space A together with operations P(n) ⊗ A^⊗n → A
/// compatible with the operad structure.
pub fn operad_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app(cst("Operad"), bvar(1)), type0()),
        ),
    )
}
/// `BarConstruction : ∀ (P : Type), Operad P → Type`
///
/// The bar construction B(P) of an operad P produces a cooperad,
/// fundamental in computing operad cohomology and Koszul duality for operads.
pub fn bar_construction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(app(cst("Operad"), bvar(0)), type0()),
    )
}
/// `CobarConstruction : ∀ (C : Type), Cooperad C → Type`
///
/// The cobar construction Ω(C) of a cooperad C produces an operad.
/// Bar-cobar adjunction: B ⊣ Ω, fundamental for Koszul duality of operads.
pub fn cobar_construction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        type0(),
        arrow(app(cst("Cooperad"), bvar(0)), type0()),
    )
}
/// `EnrichedCategory : Type → Type → Prop`
///
/// A category C enriched over a monoidal category V:
/// hom-sets are replaced by hom-objects Hom_C(X,Y) ∈ V,
/// with composition V-morphisms and unit V-morphisms.
pub fn enriched_category_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `InfinityCategory : Type → Prop`
///
/// An (∞,1)-category (quasi-category): a simplicial set satisfying the
/// inner horn filling condition (Joyal). Equivalences are invertible 1-cells.
pub fn infinity_category_ty() -> Expr {
    arrow(type0(), prop())
}
/// `StableInfinityCategory : Type → Prop`
///
/// A stable ∞-category: an ∞-category with finite limits and colimits
/// in which the suspension functor is an equivalence. Captures derived categories.
pub fn stable_infinity_category_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SymmetricMonoidalInftyCategory : Type → Prop`
///
/// A symmetric monoidal ∞-category: an ∞-category equipped with a
/// symmetric monoidal structure (commutative algebra object in Cat_∞).
pub fn symmetric_monoidal_infty_cat_ty() -> Expr {
    arrow(type0(), prop())
}
/// `EInfinityAlgebra : ∀ (C : Type), SymmetricMonoidalInftyCategory C → Type → Prop`
///
/// An E_∞-algebra in a symmetric monoidal ∞-category:
/// a commutative algebra up to all higher coherences.
/// Spectra with E_∞-structure give commutative ring spectra.
pub fn e_infinity_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app(cst("SymmetricMonoidalInftyCategory"), bvar(1)), prop()),
        ),
    )
}
/// `SymplecticForm : ∀ (V : Type), IsVectorSpace k V → Prop`
///
/// A symplectic form on V: a non-degenerate alternating bilinear form ω : V×V → k.
pub fn symplectic_form_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(app(cst("IsVectorSpace"), bvar(0)), prop()),
    )
}
/// `SymplecticModule : Type → Prop`
///
/// A symplectic module (V, ω): a free module of even rank equipped with
/// a non-degenerate skew-symmetric bilinear form.
pub fn symplectic_module_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SymplecticGroup : ∀ (V : Type), SymplecticModule V → Type`
///
/// The symplectic group Sp(V, ω): the group of linear automorphisms preserving ω.
/// |Sp(2n, 𝔽_q)| = q^{n²} ∏_{i=1}^{n} (q^{2i} − 1).
pub fn symplectic_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(app(cst("SymplecticModule"), bvar(0)), type0()),
    )
}
/// `WittGroup : Type → Type`
///
/// The Witt group W(R) of a commutative ring R:
/// equivalence classes of non-degenerate symmetric bilinear forms under metabolic cancellation.
pub fn witt_group_ty() -> Expr {
    arrow(type0(), type0())
}
