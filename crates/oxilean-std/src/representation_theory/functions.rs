//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Character, CharacterTable, CharacterTableEntry, DynkinDiagram, DynkinDiagramKind, GroupAlgebra,
    HighestWeightModule, InducedRepresentationData, LieRootSystem, QuantumGroupRep,
    RepresentationRing, RootSystem, SchurFunctor, Su2Representation, Weight, WeylGroupElement,
    YoungDiagram, YoungTableau,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
/// `Rep : Group → VectorSpace → Type` — type of linear representations.
///
/// A representation is a group homomorphism ρ : G → GL(V).
pub fn representation_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("VectorSpace"), type0()))
}
/// `RepHom : Rep G V → Rep G W → Type` — type of G-equivariant linear maps.
///
/// A morphism of representations (G-module map) φ : V → W satisfying φ(ρ(g)v) = ρ'(g)φ(v).
pub fn homomorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            pi(
                BinderInfo::Default,
                "W",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Rep"), bvar(2), bvar(1)),
                    arrow(app2(cst("Rep"), bvar(3), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `SubRep : Rep G V → Type` — type of subrepresentations (G-stable subspaces).
pub fn subrepresentation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(app2(cst("Rep"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `Irreducible : Rep G V → Prop` — predicate asserting a representation is irreducible.
///
/// A rep is irreducible (simple) if it has no proper nonzero subrepresentations.
pub fn irreducible_representation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(app2(cst("Rep"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `Character : Rep G V → G → Complex` — character map χ_ρ(g) = Tr(ρ(g)).
///
/// The character of a representation is the trace of the representing matrices.
pub fn character_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(
                app2(cst("Rep"), bvar(1), bvar(0)),
                arrow(bvar(1), cst("Complex")),
            ),
        ),
    )
}
/// Schur's Lemma: any G-module map between irreducible representations is either
/// zero or an isomorphism.
pub fn schur_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            pi(
                BinderInfo::Default,
                "W",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Irreducible"), bvar(2), bvar(1)),
                    arrow(app2(cst("Irreducible"), bvar(3), bvar(1)), prop()),
                ),
            ),
        ),
    )
}
/// Maschke's Theorem: for a finite group G with char(k) ∤ |G|, every representation
/// over k is completely reducible (semisimple).
pub fn maschke_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        pi(
            BinderInfo::Default,
            "k",
            cst("Field"),
            arrow(
                app2(
                    cst("NotDvd"),
                    app(cst("char"), bvar(0)),
                    app(cst("card"), bvar(1)),
                ),
                prop(),
            ),
        ),
    )
}
/// Character Orthogonality: characters of irreducible representations form an
/// orthonormal basis for the space of class functions on G.
pub fn character_orthogonality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        pi(
            BinderInfo::Default,
            "chi",
            app(cst("IrreducibleCharacter"), bvar(0)),
            pi(
                BinderInfo::Default,
                "psi",
                app(cst("IrreducibleCharacter"), bvar(1)),
                prop(),
            ),
        ),
    )
}
/// Burnside's Theorem: the number of irreducible representations of a finite group
/// equals the number of conjugacy classes.
pub fn burnside_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        app2(
            cst("Eq"),
            app(cst("card"), app(cst("IrreducibleReps"), bvar(0))),
            app(cst("card"), app(cst("ConjugacyClasses"), bvar(0))),
        ),
    )
}
/// `CompletelyReducible : Rep G V → Prop` — every subrepresentation has a complement.
///
/// A representation is completely reducible (semisimple) iff it decomposes as a
/// direct sum of irreducible subrepresentations.
pub fn completely_reducible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(app2(cst("Rep"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `DirectSumRep : Rep G V → Rep G W → Rep G (V ⊕ W)` — direct sum of representations.
///
/// The direct sum acts diagonally: (ρ ⊕ σ)(g)(v, w) = (ρ(g)v, σ(g)w).
pub fn direct_sum_rep_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            pi(
                BinderInfo::Default,
                "W",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Rep"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("Rep"), bvar(3), bvar(1)),
                        app2(
                            cst("Rep"),
                            bvar(4),
                            app2(cst("DirectSum"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `TensorProductRep : Rep G V → Rep G W → Rep G (V ⊗ W)` — tensor product of representations.
///
/// (ρ ⊗ σ)(g)(v ⊗ w) = ρ(g)v ⊗ σ(g)w.
pub fn tensor_product_rep_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            pi(
                BinderInfo::Default,
                "W",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Rep"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("Rep"), bvar(3), bvar(1)),
                        app2(
                            cst("Rep"),
                            bvar(4),
                            app2(cst("TensorProduct"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `DualRep : Rep G V → Rep G V*` — contragredient (dual) representation.
///
/// The dual rep acts on linear functionals: (ρ*(g)f)(v) = f(ρ(g^{-1})v).
pub fn dual_rep_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(
                app2(cst("Rep"), bvar(1), bvar(0)),
                app2(cst("Rep"), bvar(2), app(cst("Dual"), bvar(1))),
            ),
        ),
    )
}
/// `InducedRep : H ≤ G → Rep H V → Rep G (Ind_H^G V)` — induced representation.
///
/// Ind_H^G ρ = k[G] ⊗_{k[H]} V has dimension [G:H] · dim V.
pub fn induced_rep_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "H",
            cst("Group"),
            pi(
                BinderInfo::Default,
                "V",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Subgroup"), bvar(1), bvar(2)),
                    arrow(
                        app2(cst("Rep"), bvar(2), bvar(1)),
                        app2(
                            cst("Rep"),
                            bvar(4),
                            app3(cst("Ind"), bvar(3), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `RestrictedRep : H ≤ G → Rep G V → Rep H V` — restriction of a representation.
///
/// Res_H^G ρ = ρ|_H is the representation restricted to the subgroup H.
pub fn restricted_rep_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "H",
            cst("Group"),
            pi(
                BinderInfo::Default,
                "V",
                cst("VectorSpace"),
                arrow(
                    app2(cst("Subgroup"), bvar(1), bvar(2)),
                    arrow(
                        app2(cst("Rep"), bvar(3), bvar(1)),
                        app2(cst("Rep"), bvar(3), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Frobenius Reciprocity: Hom_G(Ind_H^G σ, ρ) ≅ Hom_H(σ, Res_H^G ρ).
///
/// `∀ (G H : Group) (σ : Rep H W) (ρ : Rep G V),
///    H ≤ G → FrobeniusAdj G H σ ρ`
pub fn frobenius_reciprocity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "H",
            cst("Group"),
            pi(
                BinderInfo::Default,
                "V",
                cst("VectorSpace"),
                pi(
                    BinderInfo::Default,
                    "W",
                    cst("VectorSpace"),
                    arrow(app2(cst("Subgroup"), bvar(2), bvar(3)), prop()),
                ),
            ),
        ),
    )
}
/// Mackey's Theorem: decomposition of Res_H^G Ind_K^G σ into double coset summands.
///
/// `∀ (G H K : Group) (σ : Rep K W), H ≤ G → K ≤ G → MackeyDecomp G H K σ`
pub fn mackey_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "H",
            cst("Group"),
            pi(
                BinderInfo::Default,
                "K",
                cst("Group"),
                arrow(
                    app2(cst("Subgroup"), bvar(1), bvar(2)),
                    arrow(app2(cst("Subgroup"), bvar(1), bvar(3)), prop()),
                ),
            ),
        ),
    )
}
/// `SymmetricRep : Nat → Rep (S_n) V` — the symmetric representation of S_n.
///
/// The permutation representation on n basis vectors.
pub fn symmetric_rep_ty() -> Expr {
    arrow(
        nat_ty(),
        app2(cst("Rep"), cst("SymmetricGroup"), cst("VectorSpace")),
    )
}
/// `AlternatingRep : Nat → Rep (A_n) V` — the alternating (sign) representation.
///
/// The one-dimensional representation where even permutations act by +1, odd by -1.
pub fn alternating_rep_ty() -> Expr {
    arrow(
        nat_ty(),
        app2(cst("Rep"), cst("AlternatingGroup"), cst("VectorSpace")),
    )
}
/// `YoungTableauShape : Partition → Type` — type of standard Young tableaux of given shape.
///
/// A Young tableau of shape λ = (λ₁ ≥ λ₂ ≥ … ≥ λ_k) is a filling of the Young diagram.
pub fn young_tableau_shape_ty() -> Expr {
    arrow(cst("Partition"), type0())
}
/// `SpechModule : Partition → Rep (S_n) V` — Specht module associated to a partition.
///
/// The Specht module S^λ is the irreducible representation of S_n indexed by λ ⊢ n.
pub fn specht_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("Partition"), bvar(0)),
            app2(
                cst("Rep"),
                app(cst("SymmetricGroup"), bvar(1)),
                cst("VectorSpace"),
            ),
        ),
    )
}
/// RSK Correspondence: a bijection between permutations in S_n and pairs (P, Q) of
/// standard Young tableaux of the same shape.
///
/// `∀ (n : Nat), RSKBijection (Perm n) (SYTPairs n)`
pub fn rsk_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Bijection"),
            app(cst("Perm"), bvar(0)),
            app(cst("SYTPairs"), bvar(0)),
        ),
    )
}
/// `SchurFunctor : Partition → (VectorSpace → VectorSpace)` — Schur functor S^λ.
///
/// Schur functors generalize symmetric and exterior powers; applied to ℂ^n give GL(n) reps.
pub fn schur_functor_ty() -> Expr {
    arrow(
        cst("Partition"),
        arrow(cst("VectorSpace"), cst("VectorSpace")),
    )
}
/// `RootSystem : Nat → Type` — a root system in ℝ^n.
///
/// A root system Φ ⊂ ℝ^n satisfies the integrality and reflection conditions.
pub fn root_system_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SimpleRoots : RootSystem n → List (Vec n)` — the simple root basis Δ ⊂ Φ.
///
/// Simple roots form a basis of the ambient space such that all positive roots
/// are non-negative integer combinations of simple roots.
pub fn simple_roots_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("RootSystem"), bvar(0)),
            app(cst("List"), app(cst("Vec"), bvar(1))),
        ),
    )
}
/// `CartanMatrix : RootSystem n → Matrix Int` — the Cartan matrix A_{ij} = 2⟨αᵢ,αⱼ⟩/⟨αⱼ,αⱼ⟩.
///
/// The Cartan matrix encodes the complete structure of a root system / semisimple Lie algebra.
pub fn cartan_matrix_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("RootSystem"), bvar(0)),
            app(cst("Matrix"), int_ty()),
        ),
    )
}
/// `DynkinDiagram : RootSystem n → Graph` — the Dynkin diagram classifying simple Lie algebras.
///
/// Nodes are simple roots; edges encode Cartan matrix entries.  Connected diagrams of
/// finite type: A_n, B_n, C_n, D_n, E_6, E_7, E_8, F_4, G_2.
pub fn dynkin_diagram_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("RootSystem"), bvar(0)), cst("Graph")),
    )
}
/// `WeylGroup : RootSystem n → Group` — the Weyl group W generated by simple reflections.
///
/// W is a finite Coxeter group acting faithfully on ℝ^n and on the weight lattice.
pub fn weyl_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(app(cst("RootSystem"), bvar(0)), cst("Group")),
    )
}
/// `LieAlgRep : LieAlgebra → VectorSpace → Type` — representation of a Lie algebra.
///
/// A Lie algebra representation is a homomorphism φ : 𝔤 → 𝔤𝔩(V) = End(V).
pub fn lie_alg_rep_ty() -> Expr {
    arrow(cst("LieAlgebra"), arrow(cst("VectorSpace"), type0()))
}
/// `HighestWeight : LieAlgRep g V → Weight` — the highest weight λ of a representation.
///
/// A weight λ is the highest if λ + αᵢ is not a weight for any simple root αᵢ.
pub fn highest_weight_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        cst("LieAlgebra"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(app2(cst("LieAlgRep"), bvar(1), bvar(0)), cst("Weight")),
        ),
    )
}
/// `VermaModule : LieAlgebra → Weight → LieAlgRep g V` — Verma module M(λ).
///
/// M(λ) = U(𝔤) ⊗_{U(𝔟)} ℂ_λ is a universal highest weight module.
pub fn verma_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        cst("LieAlgebra"),
        arrow(
            cst("Weight"),
            app2(cst("LieAlgRep"), bvar(1), cst("VectorSpace")),
        ),
    )
}
/// Weyl Character Formula: the character of the irreducible L(λ) is
///   ch L(λ) = (∑_{w ∈ W} (-1)^{l(w)} e^{w(λ+ρ)-ρ}) / (∏_{α > 0} (1 - e^{-α})).
///
/// `∀ (g : LieAlgebra) (λ : DominantWeight g), WeylCharFormula g λ`
pub fn weyl_character_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        cst("LieAlgebra"),
        arrow(app(cst("DominantWeight"), bvar(0)), prop()),
    )
}
/// `BGGCategoryO : LieAlgebra → Type` — BGG category 𝒪.
///
/// Category 𝒪 consists of finitely generated 𝔤-modules that are semisimple over 𝔥
/// and locally finite over 𝔲⁺.  It contains all Verma modules and irreducibles L(λ).
pub fn bgg_category_o_ty() -> Expr {
    arrow(cst("LieAlgebra"), type1())
}
/// `KazhdanLusztigPoly : WeylGroupElement → WeylGroupElement → Poly Int` — K-L polynomial P_{x,w}.
///
/// The Kazhdan-Lusztig polynomial P_{x,w}(q) encodes the multiplicities
/// [M(w·0) : L(x·0)] in category 𝒪.
pub fn kazhdan_lusztig_poly_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        cst("WeylGroup"),
        arrow(
            app(cst("WeylGroupElement"), bvar(0)),
            arrow(
                app(cst("WeylGroupElement"), bvar(1)),
                app(cst("Poly"), int_ty()),
            ),
        ),
    )
}
/// `CrystalBase : LieAlgRep g V → Type` — Kashiwara crystal base B(λ).
///
/// A crystal base (L, B) at q → 0 gives a combinatorial model for highest weight reps.
pub fn crystal_base_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        cst("LieAlgebra"),
        pi(
            BinderInfo::Default,
            "V",
            cst("VectorSpace"),
            arrow(app2(cst("LieAlgRep"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `QuantumGroupRep : QuantumGroup → VectorSpace → Type` — quantum group representation.
///
/// U_q(𝔤)-modules specialize to 𝔤-modules at q = 1 and exhibit crystal structure at q → 0.
pub fn quantum_group_rep_ty() -> Expr {
    arrow(cst("QuantumGroup"), arrow(cst("VectorSpace"), type0()))
}
/// Geometric Satake Correspondence: an equivalence of tensor categories
/// Rep(G^∨) ≃ Perv_{G[[t]]}(Gr_G) between representations of the Langlands dual group
/// and equivariant perverse sheaves on the affine Grassmannian.
///
/// `∀ (G : ReductiveGroup), GeomSatakeEquiv G (LanglandsDual G)`
pub fn geometric_satake_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("ReductiveGroup"),
        app2(
            cst("TensorCategoryEquiv"),
            app(cst("Rep"), app(cst("LanglandsDual"), bvar(0))),
            app(cst("PervSheaves"), app(cst("AffineGrassmannian"), bvar(0))),
        ),
    )
}
/// Langlands Correspondence (local, over a p-adic field): a bijection between
/// irreducible smooth representations of G(F) and L-parameters φ : W_F × SL_2 → G^∨.
///
/// `∀ (G : ReductiveGroup) (F : PAdicField), LanglandsLocal G F`
pub fn langlands_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("ReductiveGroup"),
        pi(
            BinderInfo::Default,
            "F",
            cst("PAdicField"),
            app2(
                cst("Bijection"),
                app2(cst("SmoothRep"), bvar(1), bvar(0)),
                app2(cst("LParam"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `WeilRepresentation : SymplecticGroup → Rep (Mp) V` — Weil (oscillator) representation.
///
/// The Weil representation of the metaplectic group Mp(2n) arises from the Heisenberg group
/// and plays a central role in theta correspondences.
pub fn weil_representation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Rep"),
            app(cst("MetaplecticGroup"), bvar(0)),
            cst("VectorSpace"),
        ),
    )
}
/// `ThetaCorrespondence : DualReductivePair → Bijection` — Howe theta correspondence.
///
/// Given a dual reductive pair (G, G') in Sp(2n), theta lifting provides a correspondence
/// between (a subset of) Rep(G) and Rep(G').
pub fn theta_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "pair",
        cst("DualReductivePair"),
        app2(
            cst("Bijection"),
            app(cst("Rep"), app(cst("FirstGroup"), bvar(0))),
            app(cst("Rep"), app(cst("SecondGroup"), bvar(0))),
        ),
    )
}
/// `GradedRep : Group → GradedVectorSpace → Type` — graded representation.
///
/// A Z-graded representation ρ : G → GL(V) where V = ⊕_n V_n and ρ preserves grading.
pub fn graded_rep_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("GradedVectorSpace"), type0()))
}
/// `CategoryRep : SmallCategory → Functor → Type` — functor representation of a category.
///
/// A representation of a small category C is a functor ρ : C → Vect_k.
pub fn category_rep_ty() -> Expr {
    arrow(cst("SmallCategory"), arrow(cst("Functor"), type0()))
}
/// Dimension Sum of Squares Formula: for a finite group G,
///   |G| = ∑_{i} (dim V_i)²   where the sum runs over all irreducible representations.
///
/// `∀ (G : FiniteGroup), card G = sum (map (fun χ => sq (dim χ)) (IrreducibleReps G))`
pub fn dim_sum_squares_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        app2(
            cst("Eq"),
            app(cst("card"), bvar(0)),
            app(
                cst("SumOfSquaredDims"),
                app(cst("IrreducibleReps"), bvar(0)),
            ),
        ),
    )
}
/// Second Orthogonality (column orthogonality): for columns of the character table,
///   ∑_{χ irred} χ(g) · χ̄(h) = |C_G(g)| · δ_{g ~ h}   (g ~ h means same conjugacy class).
///
/// `∀ (G : FiniteGroup) (g h : G), ColumnOrthogonality G g h`
pub fn column_orthogonality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        pi(
            BinderInfo::Default,
            "g",
            bvar(0),
            pi(BinderInfo::Default, "h", bvar(1), prop()),
        ),
    )
}
/// `SchurIndicator : IrreducibleCharacter G → Int` — Frobenius-Schur indicator.
///
/// ν₂(χ) = (1/|G|) ∑_{g ∈ G} χ(g²) ∈ {-1, 0, 1}, indicating whether the rep is
/// real, quaternionic, or complex.
pub fn schur_indicator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("FiniteGroup"),
        arrow(app(cst("IrreducibleCharacter"), bvar(0)), int_ty()),
    )
}
/// Populate an `Environment` with representation-theory axioms.
pub fn build_representation_theory_env(env: &mut Environment) {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("Group", type1),
        ("FiniteGroup", type1),
        ("VectorSpace", type1),
        ("Field", type1),
        ("LieAlgebra", type1),
        ("ReductiveGroup", type1),
        ("WeylGroup", type1),
        ("QuantumGroup", type1),
        ("SmallCategory", type1),
    ];
    for (name, mk_ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let base_types0: &[&str] = &[
        "Complex",
        "Weight",
        "Partition",
        "Graph",
        "PAdicField",
        "DualReductivePair",
        "GradedVectorSpace",
        "Functor",
    ];
    for name in base_types0 {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: type0(),
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("Rep", representation_ty),
        ("RepHom", homomorphism_ty),
        ("SubRep", subrepresentation_ty),
        ("Irreducible", irreducible_representation_ty),
        ("Character", character_ty),
        ("CompletelyReducible", completely_reducible_ty),
        ("DirectSumRep", direct_sum_rep_ty),
        ("TensorProductRep", tensor_product_rep_ty),
        ("DualRep", dual_rep_ty),
        ("InducedRep", induced_rep_ty),
        ("RestrictedRep", restricted_rep_ty),
        ("SymmetricRep", symmetric_rep_ty),
        ("AlternatingRep", alternating_rep_ty),
        ("YoungTableauShape", young_tableau_shape_ty),
        ("SpechtModule", specht_module_ty),
        ("SchurFunctor", schur_functor_ty),
        ("RootSystem", root_system_ty),
        ("SimpleRoots", simple_roots_ty),
        ("CartanMatrix", cartan_matrix_ty),
        ("DynkinDiagram", dynkin_diagram_ty),
        ("WeylGroupAxiom", weyl_group_ty),
        ("LieAlgRep", lie_alg_rep_ty),
        ("HighestWeight", highest_weight_ty),
        ("VermaModule", verma_module_ty),
        ("BGGCategoryO", bgg_category_o_ty),
        ("KazhdanLusztigPoly", kazhdan_lusztig_poly_ty),
        ("CrystalBase", crystal_base_ty),
        ("QuantumGroupRep", quantum_group_rep_ty),
        ("WeilRepresentation", weil_representation_ty),
        ("ThetaCorrespondence", theta_correspondence_ty),
        ("GradedRep", graded_rep_ty),
        ("CategoryRep", category_rep_ty),
        ("SchurIndicator", schur_indicator_ty),
    ];
    for (name, mk_ty) in type_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("schur_lemma", schur_lemma_ty),
        ("maschke_theorem", maschke_theorem_ty),
        ("character_orthogonality", character_orthogonality_ty),
        ("burnside_theorem", burnside_theorem_ty),
        ("frobenius_reciprocity", frobenius_reciprocity_ty),
        ("mackey_theorem", mackey_theorem_ty),
        ("rsk_correspondence", rsk_correspondence_ty),
        ("weyl_character_formula", weyl_character_formula_ty),
        ("geometric_satake", geometric_satake_ty),
        ("langlands_correspondence", langlands_correspondence_ty),
        ("dim_sum_squares", dim_sum_squares_ty),
        ("column_orthogonality", column_orthogonality_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
}
/// Backward-compatible type alias so existing code using `RepresentationTable` still compiles.
pub type RepresentationTable = CharacterTable;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_algebra_new_is_semisimple() {
        let a = GroupAlgebra::new("S3");
        assert_eq!(a.group_name, "S3");
        assert_eq!(a.field_char, 0);
        assert!(a.is_semisimple());
    }
    #[test]
    fn test_group_algebra_finite_char_not_semisimple() {
        let mut a = GroupAlgebra::new("Z2");
        a.field_char = 2;
        assert!(!a.is_semisimple());
    }
    #[test]
    fn test_group_algebra_display() {
        let a = GroupAlgebra::new("S4");
        assert!(a.to_string().contains("S4"));
    }
    #[test]
    fn test_character_dimension() {
        let mut chi = Character::new(6);
        chi.add_value(2.0);
        chi.add_value(1.0);
        chi.add_value(-1.0);
        assert_eq!(chi.dimension(), 2.0);
    }
    #[test]
    fn test_character_inner_product_self() {
        let mut chi = Character::new(3);
        chi.add_value(1.0);
        chi.add_value(1.0);
        chi.add_value(1.0);
        assert!((chi.inner_product(&chi) - 1.0).abs() < 1e-9);
        assert!(chi.is_irreducible());
    }
    #[test]
    fn test_character_orthogonality_two_irreducibles() {
        let mut chi1 = Character::new(6);
        for _ in 0..6 {
            chi1.add_value(1.0);
        }
        let mut chi2 = Character::new(6);
        for _ in 0..3 {
            chi2.add_value(1.0);
        }
        for _ in 0..3 {
            chi2.add_value(-1.0);
        }
        assert!((chi1.inner_product(&chi2)).abs() < 1e-9);
    }
    #[test]
    fn test_character_table_row_orthogonality() {
        let mut chi1 = Character::new(2);
        chi1.add_value(1.0);
        chi1.add_value(1.0);
        let mut chi2 = Character::new(2);
        chi2.add_value(1.0);
        chi2.add_value(-1.0);
        let mut table = CharacterTable::new(2);
        table.add_character(chi1);
        table.add_character(chi2);
        assert!(table.check_row_orthogonality());
    }
    #[test]
    fn test_character_table_dim_sum_squares() {
        let mut chi1 = Character::new(2);
        chi1.add_value(1.0);
        chi1.add_value(1.0);
        let mut chi2 = Character::new(2);
        chi2.add_value(1.0);
        chi2.add_value(-1.0);
        let mut table = CharacterTable::new(2);
        table.add_character(chi1);
        table.add_character(chi2);
        assert!(table.check_dim_sum_squares());
    }
    #[test]
    fn test_character_table_frobenius_schur() {
        let mut table = CharacterTable::new(2);
        let mut chi = Character::new(2);
        chi.add_value(1.0);
        chi.add_value(1.0);
        table.add_character(chi);
        assert_eq!(table.frobenius_schur_indicator(0), 1);
        assert_eq!(table.frobenius_schur_indicator(5), 0);
    }
    #[test]
    fn test_representation_table_num_irreducibles() {
        let mut table = RepresentationTable::new(4);
        let chi1 = Character::new(4);
        let chi2 = Character::new(4);
        table.add_character(chi1);
        table.add_character(chi2);
        assert_eq!(table.num_irreducibles(), 2);
    }
    #[test]
    fn test_representation_table_orthogonality() {
        let mut chi1 = Character::new(2);
        chi1.add_value(1.0);
        chi1.add_value(1.0);
        let mut chi2 = Character::new(2);
        chi2.add_value(1.0);
        chi2.add_value(-1.0);
        let mut table = RepresentationTable::new(2);
        table.add_character(chi1);
        table.add_character(chi2);
        assert!(table.check_row_orthogonality());
    }
    #[test]
    fn test_young_tableau_empty() {
        let t = YoungTableau::new();
        assert_eq!(t.size(), 0);
        assert!(t.is_standard());
    }
    #[test]
    fn test_young_tableau_insert_rsk() {
        let mut t = YoungTableau::new();
        for k in [1usize, 3, 2, 5, 4] {
            t.insert(k);
        }
        assert_eq!(t.size(), 5);
        assert!(t.is_standard());
    }
    #[test]
    fn test_young_tableau_shape() {
        let mut t = YoungTableau::new();
        for k in 1..=5usize {
            t.insert(k);
        }
        assert_eq!(t.shape(), vec![5]);
        assert!(t.is_standard());
    }
    #[test]
    fn test_young_tableau_display() {
        let mut t = YoungTableau::new();
        t.insert(1);
        t.insert(2);
        let s = format!("{}", t);
        assert!(s.contains('1'));
        assert!(s.contains('2'));
    }
    #[test]
    fn test_root_system_a2_simple_roots() {
        let rs = RootSystem::type_a(2);
        assert_eq!(rs.rank, 2);
        assert_eq!(rs.num_simple_roots(), 2);
        assert_eq!(rs.simple_roots[0], vec![1, -1, 0]);
        assert_eq!(rs.simple_roots[1], vec![0, 1, -1]);
    }
    #[test]
    fn test_root_system_a2_cartan_matrix() {
        let rs = RootSystem::type_a(2);
        let cm = rs.cartan_matrix();
        assert_eq!(cm[0][0], 2);
        assert_eq!(cm[0][1], -1);
        assert_eq!(cm[1][0], -1);
        assert_eq!(cm[1][1], 2);
    }
    #[test]
    fn test_root_system_b2() {
        let rs = RootSystem::type_b(2);
        assert_eq!(rs.rank, 2);
        assert_eq!(rs.simple_roots[0], vec![1, -1]);
        assert_eq!(rs.simple_roots[1], vec![0, 1]);
    }
    #[test]
    fn test_root_system_c2() {
        let rs = RootSystem::type_c(2);
        assert_eq!(rs.rank, 2);
        assert_eq!(rs.simple_roots[0], vec![1, -1]);
        assert_eq!(rs.simple_roots[1], vec![0, 2]);
    }
    #[test]
    fn test_root_system_d2() {
        let rs = RootSystem::type_d(2);
        assert_eq!(rs.rank, 2);
        assert_eq!(rs.simple_roots[0], vec![1, -1]);
        assert_eq!(rs.simple_roots[1], vec![1, 1]);
    }
    #[test]
    fn test_root_system_display() {
        let rs = RootSystem::type_a(3);
        let s = format!("{}", rs);
        assert!(s.contains("A(3)"));
        assert!(s.contains("rank 3"));
    }
    #[test]
    fn test_dynkin_diagram_a2_simply_laced() {
        let rs = RootSystem::type_a(2);
        let dd = DynkinDiagram::from_root_system(&rs);
        assert!(dd.is_simply_laced());
        assert_eq!(dd.kind, DynkinDiagramKind::A(2));
    }
    #[test]
    fn test_dynkin_diagram_b2_not_simply_laced() {
        let rs = RootSystem::type_b(2);
        let dd = DynkinDiagram::from_root_system(&rs);
        assert!(!dd.is_simply_laced());
        assert_eq!(dd.kind, DynkinDiagramKind::B(2));
    }
    #[test]
    fn test_dynkin_diagram_display() {
        let rs = RootSystem::type_a(2);
        let dd = DynkinDiagram::from_root_system(&rs);
        let s = format!("{}", dd);
        assert!(s.contains("simply_laced=true"));
    }
    #[test]
    fn test_weyl_group_element_identity() {
        let e = WeylGroupElement::identity(3);
        assert_eq!(e.length(), 0);
        assert_eq!(format!("{}", e), "e");
    }
    #[test]
    fn test_weyl_group_element_simple_reflection() {
        let s1 = WeylGroupElement::simple_reflection(3, 0);
        assert_eq!(s1.length(), 1);
        assert_eq!(format!("{}", s1), "s1");
    }
    #[test]
    fn test_weyl_group_element_multiply() {
        let s1 = WeylGroupElement::simple_reflection(3, 0);
        let s2 = WeylGroupElement::simple_reflection(3, 1);
        let w = s1.multiply(&s2);
        assert_eq!(w.length(), 2);
        assert_eq!(format!("{}", w), "s1s2");
    }
    #[test]
    fn test_weyl_group_act_on_weight() {
        let s1 = WeylGroupElement::simple_reflection(2, 0);
        let result = s1.act_on_weight_an(vec![1, 0]);
        assert_eq!(result, vec![-1, 1]);
    }
    #[test]
    fn test_weyl_group_bruhat_order() {
        let e = WeylGroupElement::identity(3);
        let s1 = WeylGroupElement::simple_reflection(3, 0);
        assert!(e.bruhat_leq(&s1));
        assert!(s1.bruhat_leq(&s1));
    }
    #[test]
    fn test_build_representation_theory_env() {
        let mut env = Environment::new();
        build_representation_theory_env(&mut env);
        assert!(env.get(&Name::str("Rep")).is_some());
        assert!(env.get(&Name::str("Irreducible")).is_some());
        assert!(env.get(&Name::str("schur_lemma")).is_some());
        assert!(env.get(&Name::str("maschke_theorem")).is_some());
        assert!(env.get(&Name::str("burnside_theorem")).is_some());
        assert!(env.get(&Name::str("character_orthogonality")).is_some());
        assert!(env.get(&Name::str("CompletelyReducible")).is_some());
        assert!(env.get(&Name::str("TensorProductRep")).is_some());
        assert!(env.get(&Name::str("InducedRep")).is_some());
        assert!(env.get(&Name::str("RestrictedRep")).is_some());
        assert!(env.get(&Name::str("VermaModule")).is_some());
        assert!(env.get(&Name::str("KazhdanLusztigPoly")).is_some());
        assert!(env.get(&Name::str("CrystalBase")).is_some());
        assert!(env.get(&Name::str("frobenius_reciprocity")).is_some());
        assert!(env.get(&Name::str("mackey_theorem")).is_some());
        assert!(env.get(&Name::str("rsk_correspondence")).is_some());
        assert!(env.get(&Name::str("weyl_character_formula")).is_some());
        assert!(env.get(&Name::str("geometric_satake")).is_some());
        assert!(env.get(&Name::str("langlands_correspondence")).is_some());
        assert!(env.get(&Name::str("dim_sum_squares")).is_some());
        assert!(env.get(&Name::str("column_orthogonality")).is_some());
    }
}
/// Kazhdan-Lusztig polynomial evaluation table.
#[allow(dead_code)]
pub fn kl_polynomial_examples() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("e", "e", "1"),
        ("s1", "e", "1"),
        ("s1 s2", "s1", "1"),
        ("s1 s2 s1", "s2", "q"),
        ("longest w0 (A3)", "e", "q^3"),
        ("w0 (A4)", "e", "q^6 + q^5 + ..."),
    ]
}
/// Character table of S_3.
#[allow(dead_code)]
pub fn s3_character_table() -> Vec<CharacterTableEntry> {
    vec![
        CharacterTableEntry::new("S3", "trivial", "e", 1),
        CharacterTableEntry::new("S3", "trivial", "(12)", 1),
        CharacterTableEntry::new("S3", "trivial", "(123)", 1),
        CharacterTableEntry::new("S3", "sign", "e", 1),
        CharacterTableEntry::new("S3", "sign", "(12)", -1),
        CharacterTableEntry::new("S3", "sign", "(123)", 1),
        CharacterTableEntry::new("S3", "standard", "e", 2),
        CharacterTableEntry::new("S3", "standard", "(12)", 0),
        CharacterTableEntry::new("S3", "standard", "(123)", -1),
    ]
}
#[cfg(test)]
mod rep_theory_ext_tests {
    use super::*;
    #[test]
    fn test_weight_dominant() {
        let w = Weight::new(vec![1, 0, 0]);
        assert!(w.dominant());
        let w2 = Weight::new(vec![-1, 1, 0]);
        assert!(!w2.dominant());
    }
    #[test]
    fn test_root_system_a2() {
        let a2 = LieRootSystem::a_n(2);
        assert_eq!(a2.num_positive_roots, 3);
        assert_eq!(a2.rank, 2);
        assert_eq!(a2.weyl_group_order(), 6);
    }
    #[test]
    fn test_su2_dimension() {
        let v2 = Su2Representation::spin(2);
        assert_eq!(v2.dimension(), 3);
        assert!(v2.is_integer_spin());
    }
    #[test]
    fn test_clebsch_gordan() {
        let v1 = Su2Representation::spin(2);
        let v2 = Su2Representation::spin(2);
        let decomp = v1.clebsch_gordan_decomposition(&v2);
        assert_eq!(decomp.len(), 3);
        let dims: u32 = decomp.iter().map(|r| r.dimension()).sum();
        assert_eq!(dims, 9);
    }
    #[test]
    fn test_young_diagram() {
        let yd = YoungDiagram::new(vec![3, 2, 1]);
        assert_eq!(yd.size(), 6);
        let dim = yd.dimension_by_hook_formula();
        assert_eq!(dim, 16);
    }
    #[test]
    fn test_young_conjugate() {
        let yd = YoungDiagram::new(vec![3, 2]);
        let conj = yd.conjugate();
        assert_eq!(conj.rows, vec![2, 2, 1]);
    }
    #[test]
    fn test_s3_character_table() {
        let ct = s3_character_table();
        assert_eq!(ct.len(), 9);
    }
}
#[cfg(test)]
mod rep_extra_tests {
    use super::*;
    #[test]
    fn test_highest_weight_module() {
        let hw = HighestWeightModule::new("sl3", Weight::new(vec![1, 0]));
        assert!(hw.is_finite_dimensional);
        assert!(hw.is_integrable());
    }
    #[test]
    fn test_induced_rep() {
        let ir = InducedRepresentationData::new("S4", "S3", 2, 4);
        assert_eq!(ir.induced_dimension, 8);
        assert!(!ir.frobenius_reciprocity_description().is_empty());
    }
    #[test]
    fn test_rep_ring() {
        let rr = RepresentationRing::new("SU(2)", vec!["V(0)", "V(1)"]);
        assert!(rr.is_commutative());
    }
}
#[cfg(test)]
mod quantum_group_rep_tests {
    use super::*;
    #[test]
    fn test_quantum_group_rep() {
        let qr = QuantumGroupRep::at_root_of_unity("A1", vec![1], 5);
        assert!(qr.is_at_root_of_unity);
        assert_eq!(qr.level, Some(5));
    }
    #[test]
    fn test_generic_rep() {
        let gr = QuantumGroupRep::generic("A2", vec![1, 0]);
        assert!(!gr.is_at_root_of_unity);
    }
}
