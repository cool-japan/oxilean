//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CechCohomologyData, ComplexOfSheaves, DerivePushforward, FinitePresheaf, FiniteSite,
    FormalComplex, Germ, GlobalSectionsComputation, LeraySpectralSequence, MicrosupportData,
    OpenNode, PerverseSheafFilter, PresheafSheafification, SheafOnSite, SheafQuantization,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn ring_ty() -> Expr {
    cst("Ring")
}
pub fn space_ty() -> Expr {
    cst("TopologicalSpace")
}
pub fn category_ty() -> Expr {
    cst("Category")
}
pub fn set_of(ty: Expr) -> Expr {
    app(cst("Set"), ty)
}
/// `Presheaf : TopologicalSpace → Category → Type`
///
/// A presheaf F on a topological space X with values in a category C is a
/// contravariant functor from Open(X) to C: assigns to each open set U an
/// object F(U) and to each inclusion V ↪ U a restriction map ρ_{UV} : F(U) → F(V),
/// functorially.
pub fn presheaf_ty() -> Expr {
    arrow(space_ty(), arrow(category_ty(), type0()))
}
/// `Sheaf : TopologicalSpace → Category → Type`
///
/// A sheaf is a presheaf satisfying the gluing axiom: given a cover {Uᵢ} of U
/// and sections sᵢ ∈ F(Uᵢ) that agree on overlaps (sᵢ|_{Uᵢ∩Uⱼ} = sⱼ|_{Uᵢ∩Uⱼ}),
/// there exists a unique s ∈ F(U) with s|_{Uᵢ} = sᵢ for all i.
pub fn sheaf_ty() -> Expr {
    arrow(space_ty(), arrow(category_ty(), type0()))
}
/// `IsSheaf : Presheaf X C → Prop`
///
/// The sheaf condition (locality + gluing) for a presheaf on a topological space.
pub fn is_sheaf_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(app2(cst("Presheaf"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `ConstantSheaf : TopologicalSpace → Type → Sheaf`
///
/// The constant sheaf A_X associated to an abelian group A: sections over a
/// connected open set U are locally constant functions U → A.
pub fn constant_sheaf_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `StructureSheaf : TopologicalSpace → Sheaf`
///
/// The structure sheaf O_X of a ringed space (X, O_X): sections over U are
/// the ring of regular functions (or holomorphic functions, etc.) on U.
pub fn structure_sheaf_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// `SheafOfModules : Sheaf → Ring → Type`
///
/// A sheaf of O_X-modules: a sheaf F such that each F(U) is a module over O_X(U)
/// and the restriction maps are module homomorphisms.
pub fn sheaf_of_modules_ty() -> Expr {
    arrow(type0(), arrow(ring_ty(), type0()))
}
/// `Stalk : Presheaf X C → X → C`
///
/// The stalk F_x of a presheaf F at a point x ∈ X: the colimit (direct limit)
/// of F(U) over all open neighborhoods U of x, with germs as elements.
pub fn stalk_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(
                app2(cst("Presheaf"), bvar(1), bvar(0)),
                arrow(bvar(2), type0()),
            ),
        ),
    )
}
/// `Section : Presheaf X C → OpenSet X → Type`
///
/// A section of a presheaf F over an open set U: an element of F(U).
pub fn section_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(
                app2(cst("Presheaf"), bvar(1), bvar(0)),
                arrow(app(cst("OpenSet"), bvar(2)), type0()),
            ),
        ),
    )
}
/// `Germ : Presheaf X C → x : X → U ∋ x → F(U) → StalkAt x`
///
/// The germ of a section s ∈ F(U) at a point x ∈ U: the equivalence class
/// of (U, s) in the stalk F_x.
pub fn germ_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(
                app2(cst("Presheaf"), bvar(1), bvar(0)),
                arrow(bvar(2), type0()),
            ),
        ),
    )
}
/// `RestrictionMap : F(U) → (V ⊆ U) → F(V)`
///
/// The restriction (transition) map of a presheaf: given an inclusion V ↪ U,
/// a restriction morphism ρ_{UV} : F(U) → F(V).
pub fn restriction_map_ty() -> Expr {
    arrow(type0(), arrow(prop(), type0()))
}
/// `GlobalSection : Sheaf X C → Type`
///
/// The global sections functor Γ(X, F) = F(X): sections over the whole space.
pub fn global_section_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(app2(cst("Sheaf"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `Sheafification : Presheaf X C → Sheaf X C`
///
/// The associated sheaf (sheafification) functor: the universal sheaf aF
/// equipped with a morphism θ : F → aF such that any presheaf map F → G
/// (with G a sheaf) factors uniquely through aF.
pub fn sheafification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(
                app2(cst("Presheaf"), bvar(1), bvar(0)),
                app2(cst("Sheaf"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `SheafificationUnit : ∀ F : Presheaf, ∃ θ : F → aF, universal`
///
/// The unit of the sheafification adjunction: the natural transformation
/// θ : F → aF that is universal among morphisms from F to sheaves.
pub fn sheafification_unit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Presheaf"), bvar(1), bvar(0)),
                prop(),
            ),
        ),
    )
}
/// `SheafificationPreservesStalk : ∀ F x, Stalk(aF, x) ≅ Stalk(F, x)`
///
/// Sheafification does not change stalks: the stalk of aF at any point x
/// is naturally isomorphic to the stalk of F at x.
pub fn sheafification_preserves_stalk_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "F",
            app2(cst("Presheaf"), bvar(0), cst("Category")),
            arrow(bvar(1), prop()),
        ),
    )
}
/// `DirectImage : (X → Y continuous) → Sheaf X C → Sheaf Y C`
///
/// The direct image (pushforward) sheaf f_* F: for an open set V ⊆ Y,
/// (f_* F)(V) = F(f⁻¹(V)).
pub fn direct_image_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(
                    app2(cst("Sheaf"), bvar(2), category_ty()),
                    app2(cst("Sheaf"), bvar(2), category_ty()),
                ),
            ),
        ),
    )
}
/// `InverseImage : (X → Y continuous) → Sheaf Y C → Sheaf X C`
///
/// The inverse image (pullback) sheaf f⁻¹ G: the sheafification of the presheaf
/// U ↦ colim_{V ⊇ f(U)} G(V).
pub fn inverse_image_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(
                    app2(cst("Sheaf"), bvar(1), category_ty()),
                    app2(cst("Sheaf"), bvar(2), category_ty()),
                ),
            ),
        ),
    )
}
/// `DirectImageAdjunction : Hom(f⁻¹G, F) ≅ Hom(G, f_*F)`
///
/// The (f⁻¹, f_*) adjunction: there is a natural bijection between sheaf
/// morphisms f⁻¹G → F and G → f_*F.
pub fn direct_image_adjunction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(arrow(bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `ProperBaseChange : ∀ (f : X → Y proper) (g : Y' → Y), g* ∘ Rf_* ≅ Rf'_* ∘ g'*`
///
/// Proper base change theorem: for a Cartesian square with f proper, the
/// derived pushforward commutes with pullback along any base change.
pub fn proper_base_change_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(arrow(bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `SheafCohomology : Nat → TopologicalSpace → Sheaf X Ab → AbelianGroup`
///
/// The sheaf cohomology groups Hⁿ(X, F): derived functors of the global
/// sections functor Γ(X, ·).
pub fn sheaf_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), type0())))
}
/// `CechCohomology : Nat → TopologicalSpace → Sheaf X Ab → AbelianGroup`
///
/// Čech cohomology Ȟⁿ(X, F) with respect to an open cover: agrees with
/// sheaf cohomology for paracompact Hausdorff spaces.
pub fn cech_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(space_ty(), arrow(type0(), type0())))
}
/// `LongExactCohomology : 0 → F' → F → F'' → 0 →
///   ... → Hⁿ(F') → Hⁿ(F) → Hⁿ(F'') → H^{n+1}(F') → ...`
///
/// The long exact sequence in sheaf cohomology induced by a short exact
/// sequence of sheaves 0 → F' → F → F'' → 0.
pub fn long_exact_cohomology_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), prop())))
}
/// `FlabbyResolution : Sheaf X Ab → Type`
///
/// A flabby (flasque) resolution of a sheaf F: a resolution 0 → F → I⁰ → I¹ → ...
/// where each Iⁿ is flabby (all restriction maps are surjective). Used to
/// compute sheaf cohomology.
pub fn flabby_resolution_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GodementResolution : Sheaf X Ab → Type`
///
/// The canonical Godement resolution: the canonical flabby resolution
/// constructed using stalks and discontinuous sections.
pub fn godement_resolution_ty() -> Expr {
    arrow(type0(), type0())
}
/// `KunnethFormula : H*(X×Y, p*F⊗q*G) ≅ H*(X,F) ⊗ H*(Y,G)`
///
/// The Künneth formula for sheaf cohomology on a product space.
pub fn kunneth_formula_ty() -> Expr {
    arrow(space_ty(), arrow(space_ty(), arrow(type0(), prop())))
}
/// `DerivedCategory : TopologicalSpace → Category → Category`
///
/// The bounded derived category D^b(Sh(X)) of sheaves on X: objects are
/// bounded complexes of sheaves, morphisms are chain maps up to homotopy
/// with quasi-isomorphisms inverted.
pub fn derived_category_ty() -> Expr {
    arrow(space_ty(), arrow(category_ty(), category_ty()))
}
/// `DerivedPushforward : (X → Y) → D^b(Sh(X)) → D^b(Sh(Y))`
///
/// The derived pushforward Rf_*: the right derived functor of f_*, computed
/// using injective (or flabby) resolutions.
pub fn derived_pushforward_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(
                    app2(cst("DerivedCategory"), bvar(2), cst("Category")),
                    app2(cst("DerivedCategory"), bvar(2), cst("Category")),
                ),
            ),
        ),
    )
}
/// `DerivedPullback : (X → Y) → D^b(Sh(Y)) → D^b(Sh(X))`
///
/// The derived pullback Lf*: the left derived functor of f*.
pub fn derived_pullback_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(
                    app2(cst("DerivedCategory"), bvar(1), cst("Category")),
                    app2(cst("DerivedCategory"), bvar(2), cst("Category")),
                ),
            ),
        ),
    )
}
/// `VerdierDuality : D^b(Sh(X)) → D^b(Sh(X))`
///
/// The Verdier dualizing functor DX = RHom(-, ωX): sends F to the Verdier
/// dual DF, generalizing Poincaré duality.
pub fn verdier_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        arrow(
            app2(cst("DerivedCategory"), bvar(0), cst("Category")),
            app2(cst("DerivedCategory"), bvar(1), cst("Category")),
        ),
    )
}
/// `SixFunctors : (f: X→Y) → Functors on D^b(Sh)`
///
/// Grothendieck's six functor formalism: f_*, f_!, f*, f!, ⊗^L, RHom
/// satisfying the fundamental adjunctions and base change formulas.
pub fn six_functors_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(arrow(bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `DModule : Variety → Type`
///
/// A D-module on an algebraic variety X: a sheaf M of O_X-modules equipped
/// with a flat connection ∇ : M → M ⊗ Ω¹_X (i.e., a module over the sheaf D_X
/// of differential operators).
pub fn d_module_ty() -> Expr {
    arrow(type0(), type0())
}
/// `HolonomicDModule : DModule X → Prop`
///
/// A holonomic D-module: a D-module M whose characteristic variety char(M) ⊆ T*X
/// has dimension dim(X) (the minimum possible, by Bernstein's inequality).
pub fn holonomic_d_module_ty() -> Expr {
    arrow(cst("DModule"), prop())
}
/// `RiemannHilbert : DModuleCategory X ≃ PerverseSheafCategory X`
///
/// The Riemann-Hilbert correspondence (Kashiwara-Mebkhout): an equivalence
/// of triangulated categories between regular holonomic D-modules on a complex
/// manifold X and perverse sheaves on X.
pub fn riemann_hilbert_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `BernsteinSato : ∀ f : polynomial, ∃ b(s) ∈ ℤ[s], b(s) f^s = P(s) f^{s+1}`
///
/// The Bernstein-Sato polynomial (b-function): for any polynomial f,
/// there exists a non-zero polynomial b(s) and a differential operator P(s)
/// such that b(s) f^s = P(s, ∂) f^{s+1}; its roots encode singularities.
pub fn bernstein_sato_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PerverseSheaf : TopologicalSpace → Type`
///
/// A perverse sheaf on a stratified space X: an object of D^b(Sh(X)) satisfying
/// the perversity conditions (support and co-support conditions with respect to
/// a stratification).
pub fn perverse_sheaf_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// `IntersectionCohomology : TopologicalSpace → Perversity → AbelianGroup`
///
/// The intersection cohomology IH*(X; p) of a singular space X with perversity p:
/// the hypercohomology of the intersection complex IC(X), a perverse sheaf.
pub fn intersection_cohomology_ty() -> Expr {
    arrow(space_ty(), arrow(nat_ty(), type0()))
}
/// `BBDDecomposition : ∀ (f : X → Y proper semismall), Rf_* IC(X) ≅ ⊕ IC(Y_α)[shift]`
///
/// The Beilinson-Bernstein-Deligne decomposition theorem: the derived pushforward
/// of the intersection complex of a smooth variety along a proper map decomposes
/// as a direct sum of shifted intersection complexes on Y.
pub fn bbd_decomposition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        space_ty(),
        pi(
            BinderInfo::Default,
            "Y",
            space_ty(),
            arrow(arrow(bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `MiddlePerversity : Nat → Nat`
///
/// The middle perversity p(k) = ⌊(k-1)/2⌋ for a stratification with strata
/// of codimension k; the most commonly used perversity in intersection cohomology.
pub fn middle_perversity_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `PoincareDualityIC : IH^k(X) ≅ IH^{2n-k}(X) for compact oriented X of dim n`
///
/// Poincaré duality for intersection cohomology: on a compact, oriented
/// pseudomanifold X of real dimension 2n, there is a non-degenerate pairing
/// IH^k(X) ⊗ IH^{2n-k}(X) → ℤ.
pub fn poincare_duality_ic_ty() -> Expr {
    arrow(space_ty(), arrow(nat_ty(), prop()))
}
/// `EtaleCohomology : Nat → Scheme → AbelianSheaf → AbelianGroup`
///
/// The étale cohomology groups H^n_{ét}(X, F) of a scheme X with coefficients
/// in an étale sheaf F: the right derived functors of the étale global sections
/// functor Γ_{ét}(X, -).
pub fn etale_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), arrow(type0(), type0())))
}
/// `lAdicCohomology : Nat → Scheme → AbelianGroup`
///
/// The ℓ-adic cohomology H^n(X_{ét}, ℚ_ℓ): étale cohomology with coefficients
/// in the constant ℓ-adic sheaf, a fundamental tool in the Weil conjectures.
pub fn l_adic_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `WeilConjectures : ∀ X/F_q smooth proper, Z(X, T) is rational, satisfies FE, RH`
///
/// The Weil conjectures (proved by Deligne): the zeta function Z(X/F_q, T) of a
/// smooth projective variety over a finite field is rational, satisfies a
/// functional equation, and satisfies the Riemann Hypothesis (eigenvalues of
/// Frobenius have absolute value q^{k/2}).
pub fn weil_conjectures_ty() -> Expr {
    arrow(type0(), prop())
}
/// `EtaleFundamentalGroup : Scheme → Group`
///
/// The étale fundamental group π₁^{ét}(X, x̄): the automorphism group of the
/// fiber functor from finite étale covers of X to sets; generalizes the
/// profinite completion of the topological fundamental group.
pub fn etale_fundamental_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GrothendieckPurity : ∀ (i : Z ↪ X closed, codim r), H^k(X\Z) → H^{k-2r}(Z)`
///
/// The Gysin (purity) sequence in étale cohomology: for a closed immersion
/// i : Z ↪ X of codimension r, there is a long exact sequence involving the
/// Gysin map i_* and the localization sequence.
pub fn grothendieck_purity_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// `GrothendieckTopology : Category → Type`
///
/// A Grothendieck topology on a category C: assigns to each object U a
/// collection of covering sieves J(U), satisfying maximality, stability,
/// and transitivity axioms.
pub fn grothendieck_topology_ty() -> Expr {
    arrow(category_ty(), type0())
}
/// `Site : Type`
///
/// A site (C, J): a category C equipped with a Grothendieck topology J.
pub fn site_ty() -> Expr {
    type0()
}
/// `SheafOnSiteType : Site → Category → Type`
///
/// A sheaf on a site (C, J) with values in a category D: a presheaf F : C^op → D
/// satisfying the sheaf condition for every covering sieve in J.
pub fn sheaf_on_site_ty() -> Expr {
    arrow(site_ty(), arrow(category_ty(), type0()))
}
/// `DescentData : Site → Sheaf → Prop`
///
/// The descent datum condition: sections over a cover that agree on all
/// intersections (overlaps in the Grothendieck topology sense) descend
/// uniquely to a global section.
pub fn descent_data_ty() -> Expr {
    arrow(site_ty(), arrow(type0(), prop()))
}
/// `SheafificationOnSite : Presheaf Site D → Sheaf Site D`
///
/// The sheafification of a presheaf on a site: the universal sheaf associated
/// to a given presheaf, constructed via the plus-construction (two applications
/// of the sheaf-associated presheaf functor).
pub fn sheafification_on_site_ty() -> Expr {
    arrow(site_ty(), arrow(category_ty(), arrow(type0(), type0())))
}
/// `ToposOfSheaves : Site → Topos`
///
/// The topos of sheaves on a site: Sh(C, J) is a Grothendieck topos,
/// the fundamental example of a topos arising from a site.
pub fn topos_of_sheaves_ty() -> Expr {
    arrow(site_ty(), type0())
}
/// `TStructure : DerivedCategory → Prop`
///
/// A t-structure on a triangulated category D: a pair of subcategories
/// (D^{≤0}, D^{≥0}) satisfying orthogonality, stability, and truncation axioms.
pub fn t_structure_ty() -> Expr {
    arrow(category_ty(), prop())
}
/// `HeartOfTStructure : TStructure → AbelianCategory`
///
/// The heart of a t-structure: the abelian category D^{≤0} ∩ D^{≥0},
/// whose objects are "cohomologically concentrated in degree 0".
pub fn heart_of_t_structure_ty() -> Expr {
    arrow(prop(), category_ty())
}
/// `PerversityFunction : Nat → Int`
///
/// A perversity function p : ℕ → ℤ specifying the allowed cohomological
/// degrees for the support and co-support conditions in the definition of
/// perverse sheaves on a stratified space.
pub fn perversity_function_ty() -> Expr {
    arrow(nat_ty(), int_ty())
}
/// `BBDPurity : Prop → Prop`
///
/// Purity in the BBD sense: a perverse sheaf is pure of weight w if its
/// stalks and costalks are pure of the appropriate weight (relevant for
/// the decomposition theorem over finite fields).
pub fn bbd_purity_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ConstructibleSheaf : TopologicalSpace → Stratification → Type`
///
/// A constructible sheaf on a stratified space (X, S): a sheaf that is
/// locally constant on each stratum Sᵢ of the stratification S.
pub fn constructible_sheaf_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `CharacteristicVariety : DModule → Type`
///
/// The characteristic variety char(M) ⊆ T*X of a D-module M: the support
/// of its associated graded module under the filtration by order of
/// differential operators; measures the singularities of M.
pub fn characteristic_variety_ty() -> Expr {
    arrow(cst("DModule"), type0())
}
/// `LAdicSheaf : Scheme → Type`
///
/// An ℓ-adic sheaf on a scheme X: a projective system of constructible
/// ℤ/ℓⁿ-sheaves on X_{ét}, whose limit gives a ℚ_ℓ-sheaf.
pub fn l_adic_sheaf_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FrobeniusEndomorphism : Scheme → Endomorphism`
///
/// The Frobenius endomorphism Frob_q : X → X for a scheme over 𝔽_q:
/// the morphism given by raising all coordinates to the q-th power.
pub fn frobenius_endomorphism_ty() -> Expr {
    arrow(type0(), type0())
}
/// `WeightFiltration : LAdicSheaf → Type`
///
/// The weight filtration on an ℓ-adic sheaf: a filtration W_• such that
/// gr_k W is pure of weight k (eigenvalues of Frobenius have absolute value q^{k/2}).
pub fn weight_filtration_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Microsupport : Sheaf X Ab → Type`
///
/// The microsupport SS(F) ⊆ T*X of a sheaf F: the set of covectors (x, ξ)
/// such that F is not locally constant in the direction ξ at x.
/// Defined by Kashiwara-Schapira.
pub fn microsupport_ty() -> Expr {
    arrow(type0(), type0())
}
/// `MicrolocalPropagation : Microsupport → Prop`
///
/// The microlocal propagation theorem: if a sheaf F has empty microsupport
/// over a region, sections propagate across that region.
pub fn microlocal_propagation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `MicrolocalMorseInequality : Sheaf → Morse function → Prop`
///
/// Microlocal Morse inequalities: relate the Euler characteristic of sections
/// with support to the intersection of the microsupport with the conormal of
/// a Morse function's level sets.
pub fn microlocal_morse_inequality_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `FukayaCategory : SymplecticManifold → A∞Category`
///
/// The Fukaya category Fuk(M, ω) of a symplectic manifold (M, ω): an A∞-category
/// whose objects are Lagrangian submanifolds and morphisms are Floer chain complexes.
pub fn fukaya_category_ty() -> Expr {
    arrow(type0(), type0())
}
/// `HomologicalMirrorSymmetry : SymplecticManifold → ComplexManifold → Prop`
///
/// Homological mirror symmetry (Kontsevich): an equivalence of A∞-categories
/// Fuk(M, ω) ≃ D^b(Coh(M̌)) between the Fukaya category of M and the derived
/// category of coherent sheaves on the mirror M̌.
pub fn homological_mirror_symmetry_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `FloerCohomology : Lagrangian → Lagrangian → AbelianGroup`
///
/// Floer cohomology HF*(L₀, L₁) of two Lagrangian submanifolds: the cohomology
/// of the Floer chain complex, counting pseudo-holomorphic strips between L₀ and L₁.
pub fn floer_cohomology_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `FactorizationAlgebra : TopologicalSpace → Type`
///
/// A factorization algebra on a manifold M: a cosheaf-like structure F
/// assigning to each open set U an object F(U), with factorization maps
/// F(U₁) ⊗ F(U₂) → F(U₁ ∪ U₂) for disjoint U₁, U₂ ⊆ U.
pub fn factorization_algebra_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// `TopologicalFieldTheory : Nat → Type`
///
/// An n-dimensional topological field theory (Atiyah-Segal): a symmetric
/// monoidal functor Z : Cob_n → Vect assigning vector spaces to (n-1)-manifolds
/// and linear maps to n-dimensional cobordisms.
pub fn topological_field_theory_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CobordismHypothesis : Nat → Prop`
///
/// The cobordism hypothesis (Lurie): the (∞,n)-category of fully extended
/// framed n-dimensional TFTs is equivalent to the (∞,n)-groupoid of fully
/// dualizable objects in the target symmetric monoidal (∞,n)-category.
pub fn cobordism_hypothesis_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CondensedSet : Type`
///
/// A condensed set (Clausen-Scholze): a sheaf on the site of profinite sets
/// (with the coherent/Stone topology). Replaces topological spaces in a
/// better-behaved categorical framework.
pub fn condensed_set_ty() -> Expr {
    type0()
}
/// `CondensedAbelianGroup : Type`
///
/// A condensed abelian group: an abelian group object in the category of
/// condensed sets; the correct replacement for topological abelian groups
/// in liquid/solid mathematics.
pub fn condensed_abelian_group_ty() -> Expr {
    type0()
}
/// `SolidAbelianGroup : CondensedAbelianGroup → Prop`
///
/// A solid abelian group: a condensed abelian group M such that
/// Ext^i(ℤ[S], M) = 0 for all profinite sets S and i > 0.
pub fn solid_abelian_group_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PyknoticObject : Topos → Type`
///
/// A pyknotic object (Barwick-Haine): an object in a topos T with an
/// action of the profinite completion of symmetry groups; closely related
/// to condensed sets.
pub fn pyknotic_object_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Topos : Type`
///
/// A (Grothendieck) topos: a category equivalent to the category of sheaves
/// on some site; satisfies Giraud's axioms (exactness, generators, etc.).
pub fn topos_ty() -> Expr {
    type0()
}
/// `GeometricMorphism : Topos → Topos → Type`
///
/// A geometric morphism f : E → F between topoi: a pair of adjoint functors
/// (f*, f_*) with f* exact (preserving finite limits).
pub fn geometric_morphism_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `InfinityTopos : Type`
///
/// An (∞,1)-topos (Lurie): an (∞,1)-category satisfying the ∞-categorical
/// Giraud axioms, generalizing Grothendieck topoi to higher categorical settings.
pub fn infinity_topos_ty() -> Expr {
    type0()
}
/// `HigherSheaf : InfinityTopos → Nat → Type`
///
/// A higher sheaf (n-sheaf or ∞-sheaf) on an ∞-topos: a sheaf taking values
/// in n-groupoids (or ∞-groupoids), satisfying the ∞-categorical descent condition.
pub fn higher_sheaf_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `LurieHypercovering : InfinityTopos → Sheaf → Prop`
///
/// The hypercovering theorem in ∞-topoi: every ∞-sheaf can be computed as
/// the limit of a hypercovering, generalizing the Čech → sheaf cohomology result.
pub fn lurie_hypercovering_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `SheavesOnStratifiedSpace : TopologicalSpace → Stratification → Type`
///
/// The category of sheaves on a stratified space (X, S): locally constant sheaves
/// on each stratum, glued via exit path categories.
pub fn sheaves_on_stratified_space_ty() -> Expr {
    arrow(space_ty(), arrow(type0(), type0()))
}
/// `ExitPathCategory : StratifiedSpace → Category`
///
/// The exit path category of a stratified space: a topological category (or
/// (∞,1)-category) whose objects are points of X and morphisms are exit paths
/// from deeper to shallower strata; classifies constructible sheaves.
pub fn exit_path_category_ty() -> Expr {
    arrow(type0(), category_ty())
}
/// `LocallyConstantSheaf : TopologicalSpace → Type`
///
/// A locally constant sheaf (local system) on X: a sheaf F such that every
/// point has a neighborhood U where F|_U is constant.
pub fn locally_constant_sheaf_ty() -> Expr {
    arrow(space_ty(), type0())
}
/// Register all sheaf theory axioms into the kernel environment.
pub fn build_sheaf_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("TopologicalSpace", type0()),
        ("Category", type0()),
        ("OpenSet", arrow(space_ty(), type0())),
        ("Ring", type0()),
        ("Group", type0()),
        ("AbelianGroup", type0()),
        ("Set", arrow(type0(), type0())),
        ("DModule", type0()),
        ("Presheaf", presheaf_ty()),
        ("Sheaf", sheaf_ty()),
        ("IsSheaf", is_sheaf_ty()),
        ("ConstantSheaf", constant_sheaf_ty()),
        ("StructureSheaf", structure_sheaf_ty()),
        ("SheafOfModules", sheaf_of_modules_ty()),
        ("Stalk", stalk_ty()),
        ("Section", section_ty()),
        ("Germ", germ_ty()),
        ("RestrictionMap", restriction_map_ty()),
        ("GlobalSection", global_section_ty()),
        ("Sheafification", sheafification_ty()),
        ("sheafification_unit", sheafification_unit_ty()),
        (
            "sheafification_preserves_stalk",
            sheafification_preserves_stalk_ty(),
        ),
        ("DirectImage", direct_image_ty()),
        ("InverseImage", inverse_image_ty()),
        ("direct_image_adjunction", direct_image_adjunction_ty()),
        ("proper_base_change", proper_base_change_ty()),
        ("SheafCohomology", sheaf_cohomology_ty()),
        ("CechCohomology", cech_cohomology_ty()),
        ("long_exact_cohomology", long_exact_cohomology_ty()),
        ("FlabbyResolution", flabby_resolution_ty()),
        ("GodementResolution", godement_resolution_ty()),
        ("kunneth_formula", kunneth_formula_ty()),
        ("DerivedCategory", derived_category_ty()),
        ("DerivedPushforward", derived_pushforward_ty()),
        ("DerivedPullback", derived_pullback_ty()),
        ("VerdierDuality", verdier_duality_ty()),
        ("six_functors", six_functors_ty()),
        ("d_module", d_module_ty()),
        ("HolonomicDModule", holonomic_d_module_ty()),
        ("riemann_hilbert", riemann_hilbert_ty()),
        ("bernstein_sato", bernstein_sato_ty()),
        ("PerverseSheaf", perverse_sheaf_ty()),
        ("IntersectionCohomology", intersection_cohomology_ty()),
        ("bbd_decomposition", bbd_decomposition_ty()),
        ("MiddlePerversity", middle_perversity_ty()),
        ("poincare_duality_ic", poincare_duality_ic_ty()),
        ("EtaleCohomology", etale_cohomology_ty()),
        ("lAdicCohomology", l_adic_cohomology_ty()),
        ("weil_conjectures", weil_conjectures_ty()),
        ("EtaleFundamentalGroup", etale_fundamental_group_ty()),
        ("grothendieck_purity", grothendieck_purity_ty()),
        ("GrothendieckTopology", grothendieck_topology_ty()),
        ("Site", site_ty()),
        ("SheafOnSiteType", sheaf_on_site_ty()),
        ("DescentData", descent_data_ty()),
        ("sheafification_on_site", sheafification_on_site_ty()),
        ("ToposOfSheaves", topos_of_sheaves_ty()),
        ("TStructure", t_structure_ty()),
        ("HeartOfTStructure", heart_of_t_structure_ty()),
        ("PerversityFunction", perversity_function_ty()),
        ("BBDPurity", bbd_purity_ty()),
        ("ConstructibleSheaf", constructible_sheaf_ty()),
        ("CharacteristicVariety", characteristic_variety_ty()),
        ("LAdicSheaf", l_adic_sheaf_ty()),
        ("FrobeniusEndomorphism", frobenius_endomorphism_ty()),
        ("WeightFiltration", weight_filtration_ty()),
        ("Microsupport", microsupport_ty()),
        ("microlocal_propagation", microlocal_propagation_ty()),
        (
            "microlocal_morse_inequality",
            microlocal_morse_inequality_ty(),
        ),
        ("FukayaCategory", fukaya_category_ty()),
        (
            "homological_mirror_symmetry",
            homological_mirror_symmetry_ty(),
        ),
        ("FloerCohomology", floer_cohomology_ty()),
        ("FactorizationAlgebra", factorization_algebra_ty()),
        ("TopologicalFieldTheory", topological_field_theory_ty()),
        ("cobordism_hypothesis", cobordism_hypothesis_ty()),
        ("CondensedSet", condensed_set_ty()),
        ("CondensedAbelianGroup", condensed_abelian_group_ty()),
        ("SolidAbelianGroup", solid_abelian_group_ty()),
        ("PyknoticObject", pyknotic_object_ty()),
        ("Topos", topos_ty()),
        ("GeometricMorphism", geometric_morphism_ty()),
        ("InfinityTopos", infinity_topos_ty()),
        ("HigherSheaf", higher_sheaf_ty()),
        ("lurie_hypercovering", lurie_hypercovering_ty()),
        ("SheavesOnStratifiedSpace", sheaves_on_stratified_space_ty()),
        ("ExitPathCategory", exit_path_category_ty()),
        ("LocallyConstantSheaf", locally_constant_sheaf_ty()),
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
/// Compute Čech H⁰ (global sections): the kernel of the Čech differential δ⁰.
///
/// Given sections over a cover {U, V} with overlap U∩V, the Čech 0-cocycles
/// are pairs (s_U, s_V) with ρ_U(s_U) = ρ_V(s_V) on the overlap.
///
/// Returns the dimension of H⁰ (number of compatible pairs basis elements).
pub fn cech_h0_dimension(presheaf: &FinitePresheaf, u: usize, v: usize, uv: usize) -> usize {
    let dim_u = presheaf.section_dims[u];
    let dim_v = presheaf.section_dims[v];
    let dim_uv = presheaf.section_dims[uv];
    let _ = dim_uv;
    let mut compatible = 0;
    for i in 0..dim_u {
        let mut eu = vec![0i64; dim_u];
        eu[i] = 1;
        for j in 0..dim_v {
            let mut ev = vec![0i64; dim_v];
            ev[j] = 1;
            if presheaf.check_compatibility(u, v, uv, &eu, &ev) {
                compatible += 1;
            }
        }
    }
    compatible
}
/// Compute the Euler characteristic of a stratified space from its
/// intersection cohomology Betti numbers.
///
/// χ = Σ (-1)^k dim IH^k(X).
pub fn intersection_euler_characteristic(betti_numbers: &[usize]) -> i64 {
    betti_numbers
        .iter()
        .enumerate()
        .map(|(k, &b)| if k % 2 == 0 { b as i64 } else { -(b as i64) })
        .sum()
}
/// Check Poincaré duality for intersection cohomology: IH^k ≅ IH^{2n-k}.
///
/// Returns true if the Betti numbers are symmetric: betti[k] = betti[2n-k].
pub fn check_poincare_duality_ic(betti_numbers: &[usize]) -> bool {
    let total = betti_numbers.len();
    for k in 0..total {
        let dual_k = if total > k { total - 1 - k } else { 0 };
        if k < dual_k && betti_numbers[k] != betti_numbers[dual_k] {
            return false;
        }
    }
    true
}
/// The middle perversity: p(k) = ⌊(k-1)/2⌋ for codimension k ≥ 1.
pub fn middle_perversity(codim: u32) -> u32 {
    if codim == 0 {
        0
    } else {
        (codim - 1) / 2
    }
}
/// Check whether a formal complex F satisfies the support condition for middle perversity.
///
/// The support condition: H^k(F|_{Sᵢ}) = 0 for k > -codim(Sᵢ) + p(codim(Sᵢ)).
/// Here we check whether the complex vanishes above the threshold degree.
pub fn check_support_condition(complex: &FormalComplex, codim: u32) -> bool {
    let threshold = -(codim as i32) + middle_perversity(codim) as i32;
    for (i, &d) in complex.cohomology.iter().enumerate() {
        let k = complex.min_degree + i as i32;
        if d > 0 && k > threshold {
            return false;
        }
    }
    true
}
/// Compute the E₂ page of the Leray spectral sequence for f : X → Y.
///
/// E₂^{p,q} = H^p(Y, R^q f_* F): given the higher direct images R^q f_* F
/// as formal complexes on Y, compute the total E₂ contributions.
///
/// Returns the total cohomology groups if the spectral sequence degenerates at E₂.
pub fn leray_e2_degeneration(
    base_cohomology: &[usize],
    higher_direct_images: &[Vec<usize>],
) -> Vec<usize> {
    if higher_direct_images.is_empty() {
        return base_cohomology.to_vec();
    }
    let n_base = base_cohomology.len();
    let n_fiber = higher_direct_images.len();
    let total_len = n_base + n_fiber - 1;
    let mut result = vec![0usize; total_len];
    for (p, &hp) in base_cohomology.iter().enumerate() {
        for (q, hq_vec) in higher_direct_images.iter().enumerate() {
            let hq = if p < hq_vec.len() { hq_vec[p] } else { 0 };
            let k = p + q;
            if k < total_len {
                result[k] += hp * hq;
            }
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_sheaf_theory_env_populates() {
        let mut env = Environment::new();
        build_sheaf_theory_env(&mut env);
        assert!(env.get(&Name::str("Sheaf")).is_some());
        assert!(env.get(&Name::str("Presheaf")).is_some());
        assert!(env.get(&Name::str("Sheafification")).is_some());
        assert!(env.get(&Name::str("SheafCohomology")).is_some());
        assert!(env.get(&Name::str("VerdierDuality")).is_some());
        assert!(env.get(&Name::str("riemann_hilbert")).is_some());
        assert!(env.get(&Name::str("weil_conjectures")).is_some());
        assert!(env.get(&Name::str("bbd_decomposition")).is_some());
        assert!(env.get(&Name::str("EtaleCohomology")).is_some());
    }
    #[test]
    fn test_finite_presheaf_restriction() {
        let nodes = vec![
            OpenNode::new(0, "U"),
            OpenNode::new(1, "V"),
            OpenNode::new(2, "U∩V"),
        ];
        let dims = vec![1, 1, 1];
        let mut psh = FinitePresheaf::new(nodes, dims);
        psh.add_restriction(0, 2, vec![vec![1]]);
        psh.add_restriction(1, 2, vec![vec![1]]);
        assert!(psh.check_compatibility(0, 1, 2, &[2], &[2]));
        assert!(!psh.check_compatibility(0, 1, 2, &[1], &[2]));
    }
    #[test]
    fn test_cech_h0_dimension() {
        let nodes = vec![
            OpenNode::new(0, "U"),
            OpenNode::new(1, "V"),
            OpenNode::new(2, "U∩V"),
        ];
        let dims = vec![1, 1, 1];
        let mut psh = FinitePresheaf::new(nodes, dims);
        psh.add_restriction(0, 2, vec![vec![1]]);
        psh.add_restriction(1, 2, vec![vec![1]]);
        let dim = cech_h0_dimension(&psh, 0, 1, 2);
        assert_eq!(dim, 1, "H0 should be 1-dimensional for constant sheaf");
    }
    #[test]
    fn test_formal_complex_shift() {
        let c = FormalComplex::new(0, vec![1, 0, 1]);
        assert_eq!(c.euler_characteristic(), 2);
        let shifted = c.shift(1);
        assert_eq!(shifted.min_degree, -1);
        assert_eq!(shifted.euler_characteristic(), -2);
    }
    #[test]
    fn test_formal_complex_lowest_degree() {
        let c = FormalComplex::new(-2, vec![0, 0, 1, 2]);
        assert_eq!(c.lowest_nonzero_degree(), Some(0));
        let empty = FormalComplex::new(0, vec![0, 0, 0]);
        assert_eq!(empty.lowest_nonzero_degree(), None);
    }
    #[test]
    fn test_intersection_euler_characteristic() {
        let betti = [1, 0, 1, 0, 1];
        assert_eq!(intersection_euler_characteristic(&betti), 3);
    }
    #[test]
    fn test_poincare_duality_ic() {
        assert!(check_poincare_duality_ic(&[1, 0, 1, 0, 1]));
        assert!(!check_poincare_duality_ic(&[1, 2, 1, 0, 1]));
    }
    #[test]
    fn test_middle_perversity() {
        assert_eq!(middle_perversity(0), 0);
        assert_eq!(middle_perversity(1), 0);
        assert_eq!(middle_perversity(2), 0);
        assert_eq!(middle_perversity(3), 1);
        assert_eq!(middle_perversity(4), 1);
        assert_eq!(middle_perversity(5), 2);
    }
    #[test]
    fn test_axiom_types_wellformed() {
        let ty1 = presheaf_ty();
        assert!(matches!(ty1, Expr::Pi(_, _, _, _)));
        let ty2 = sheaf_cohomology_ty();
        assert!(matches!(ty2, Expr::Pi(_, _, _, _)));
        let ty3 = verdier_duality_ty();
        assert!(matches!(ty3, Expr::Pi(_, _, _, _)));
        let ty4 = etale_cohomology_ty();
        assert!(matches!(ty4, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_extended_axiom_registration() {
        let mut env = Environment::new();
        build_sheaf_theory_env(&mut env);
        assert!(env.get(&Name::str("GrothendieckTopology")).is_some());
        assert!(env.get(&Name::str("SheafOnSiteType")).is_some());
        assert!(env.get(&Name::str("ToposOfSheaves")).is_some());
        assert!(env.get(&Name::str("TStructure")).is_some());
        assert!(env.get(&Name::str("ConstructibleSheaf")).is_some());
        assert!(env.get(&Name::str("LAdicSheaf")).is_some());
        assert!(env.get(&Name::str("Microsupport")).is_some());
        assert!(env.get(&Name::str("FukayaCategory")).is_some());
        assert!(env.get(&Name::str("homological_mirror_symmetry")).is_some());
        assert!(env.get(&Name::str("FactorizationAlgebra")).is_some());
        assert!(env.get(&Name::str("cobordism_hypothesis")).is_some());
        assert!(env.get(&Name::str("CondensedSet")).is_some());
        assert!(env.get(&Name::str("SolidAbelianGroup")).is_some());
        assert!(env.get(&Name::str("InfinityTopos")).is_some());
        assert!(env.get(&Name::str("lurie_hypercovering")).is_some());
        assert!(env.get(&Name::str("ExitPathCategory")).is_some());
        assert!(env.get(&Name::str("LocallyConstantSheaf")).is_some());
    }
    #[test]
    fn test_sheaf_on_site_basic() {
        let mut site = FiniteSite::new(3);
        site.add_morphism(0, 2);
        site.add_morphism(1, 2);
        site.add_cover(0, vec![0]);
        site.add_cover(1, vec![1]);
        site.add_cover(2, vec![0, 1]);
        let section_dims = vec![1, 1, 1];
        let mut sheaf = SheafOnSite::new(site, section_dims);
        sheaf.add_restriction(0, 2, vec![vec![1]]);
        sheaf.add_restriction(1, 2, vec![vec![1]]);
        assert_eq!(sheaf.total_covers(), 3);
        let r = sheaf.restrict(0, 2, &[3]);
        assert_eq!(r, Some(vec![3]));
    }
    #[test]
    fn test_sheaf_on_site_condition() {
        let mut site = FiniteSite::new(3);
        site.add_morphism(0, 2);
        site.add_morphism(1, 2);
        let section_dims = vec![1, 1, 1];
        let mut sheaf = SheafOnSite::new(site, section_dims);
        sheaf.add_restriction(0, 2, vec![vec![1]]);
        sheaf.add_restriction(1, 2, vec![vec![1]]);
        let sections = vec![vec![5i64], vec![5i64]];
        assert!(sheaf.check_sheaf_condition(&[0, 1], &sections));
        let sections2 = vec![vec![5i64], vec![6i64]];
        assert!(!sheaf.check_sheaf_condition(&[0, 1], &sections2));
    }
    #[test]
    fn test_presheaf_sheafification() {
        let nodes = vec![
            OpenNode::new(0, "U"),
            OpenNode::new(1, "V"),
            OpenNode::new(2, "UV"),
        ];
        let dims = vec![1, 1, 1];
        let mut psh = FinitePresheaf::new(nodes, dims);
        psh.add_restriction(0, 2, vec![vec![1]]);
        psh.add_restriction(1, 2, vec![vec![1]]);
        let mut sheafifier = PresheafSheafification::new(psh);
        assert_eq!(sheafifier.iterations, 0);
        sheafifier.apply_plus();
        assert_eq!(sheafifier.iterations, 1);
        sheafifier.apply_plus();
        assert_eq!(sheafifier.iterations, 2);
        assert_eq!(sheafifier.sheafified_dim(0), 1);
        assert_eq!(sheafifier.sheafified_dim(2), 1);
        assert!(sheafifier.is_sheaf_for_mv_cover(0, 1, 2));
    }
    #[test]
    fn test_derive_pushforward_kunneth() {
        let source = FormalComplex::new(0, vec![1, 2, 1]);
        let target = FormalComplex::new(0, vec![1, 1]);
        let fiber = FormalComplex::new(0, vec![1, 1]);
        let dpf = DerivePushforward::new(source, target, fiber);
        let kunneth = dpf.kunneth_cohomology();
        assert_eq!(kunneth.cohomology, vec![1, 2, 1]);
        assert!(dpf.verify_euler_multiplicativity());
    }
    #[test]
    fn test_perverse_sheaf_filter() {
        let supp = FormalComplex::new(-3, vec![1]);
        let cosupp = FormalComplex::new(-1, vec![1]);
        let filter = PerverseSheafFilter::new(vec![2], vec![supp], vec![cosupp]);
        assert!(filter.satisfies_support_condition());
        assert!(filter.satisfies_cosupport_condition());
        assert!(filter.is_perverse());
        assert_eq!(filter.num_perverse_strata(), 1);
    }
    #[test]
    fn test_leray_e2_degeneration() {
        let base = vec![1usize, 1];
        let higher_direct_images = vec![vec![1, 1]];
        let result = leray_e2_degeneration(&base, &higher_direct_images);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 1);
    }
    #[test]
    fn test_extended_axiom_types_wellformed() {
        let ty1 = grothendieck_topology_ty();
        assert!(matches!(ty1, Expr::Pi(_, _, _, _)));
        let ty2 = fukaya_category_ty();
        assert!(matches!(ty2, Expr::Pi(_, _, _, _)));
        let ty3 = condensed_set_ty();
        assert!(matches!(ty3, Expr::Sort(_)));
        let ty4 = infinity_topos_ty();
        assert!(matches!(ty4, Expr::Sort(_)));
        let ty5 = cobordism_hypothesis_ty();
        assert!(matches!(ty5, Expr::Pi(_, _, _, _)));
        let ty6 = exit_path_category_ty();
        assert!(matches!(ty6, Expr::Pi(_, _, _, _)));
    }
}
#[cfg(test)]
mod tests_sheaf_ext {
    use super::*;
    #[test]
    fn test_cech_cohomology() {
        let mut cd =
            CechCohomologyData::new(vec!["U1".to_string(), "U2".to_string()], "constant sheaf Z");
        cd.add_section(0, 1.0);
        cd.add_cocycle(0, 1, 1.0);
        assert!(cd.is_acyclic_cover());
        assert_eq!(cd.h0_dimension(), 1);
    }
    #[test]
    fn test_complex_of_sheaves() {
        let mut cs = ComplexOfSheaves::new("IC_X").bounded();
        cs.add_cohomology_degree(-2);
        cs.add_cohomology_degree(0);
        assert_eq!(cs.amplitude(), Some((-2, 0)));
        assert!(!cs.is_single_sheaf());
        assert!(cs.check_perversity_condition(2));
    }
    #[test]
    fn test_leray_spectral_sequence() {
        let mut lss = LeraySpectralSequence::new("S^2", "S^3", "Z").set_degenerates_at_e2();
        lss.set_e2(0, 0, 1);
        lss.set_e2(0, 1, 1);
        assert_eq!(lss.total_cohomology_rank(0), 1);
        assert_eq!(lss.total_cohomology_rank(1), 1);
    }
    #[test]
    fn test_microsupport() {
        let ms = MicrosupportData::new("L_x", "T*_x X")
            .lagrangian()
            .with_char_variety("char(M)");
        assert!(ms.is_lagrangian);
        assert!(ms.involutivity_theorem().contains("Lagrangian"));
    }
    #[test]
    fn test_cocycle_condition() {
        let mut cd = CechCohomologyData::new(
            vec!["U1".to_string(), "U2".to_string(), "U3".to_string()],
            "line bundle",
        );
        cd.add_cocycle(0, 1, 2.0);
        cd.add_cocycle(1, 2, 3.0);
        cd.add_cocycle(0, 2, 6.0);
        assert!(cd.satisfies_cocycle_condition(1e-10));
    }
}
#[cfg(test)]
mod tests_sheaf_ext2 {
    use super::*;
    #[test]
    fn test_sheaf_quantization() {
        let sq = SheafQuantization::new("T*_x X", "delta_x")
            .unique()
            .with_local_system_rank(1);
        assert!(sq.is_unique);
        assert!(sq.quantization_condition().contains("T*_x X"));
    }
    #[test]
    fn test_global_sections() {
        let mut gs = GlobalSectionsComputation::new(3, 3, 1);
        gs.set_vertex(0, 1.0);
        gs.set_vertex(1, 1.0);
        gs.set_vertex(2, 1.0);
        gs.add_edge_restriction(0, 1, 1.0);
        gs.add_edge_restriction(1, 2, 1.0);
        assert!(gs.is_global_section(1e-10));
        assert_eq!(gs.euler_characteristic(), 1);
    }
    #[test]
    fn test_global_sections_not() {
        let mut gs = GlobalSectionsComputation::new(2, 1, 0);
        gs.set_vertex(0, 1.0);
        gs.set_vertex(1, 2.0);
        gs.add_edge_restriction(0, 1, 1.5);
        assert!(!gs.is_global_section(1e-10));
    }
}
