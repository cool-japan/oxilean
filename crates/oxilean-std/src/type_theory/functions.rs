//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{HomotopyLevel, MlttTerm, STLCType};

pub(super) fn app(f: Expr, a: Expr) -> Expr {
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
/// `UniverseHierarchy : Type_0 : Type_1 : Type_2 : ...`
///
/// The universe hierarchy axiom — Type_n : Type_{n+1} for all n.
pub fn universe_hierarchy_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PiType : (A : Type) → (A → Type) → Type`
///
/// Dependent product type formation rule.
pub fn pi_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", arrow(bvar(0), type0()), type0()),
    )
}
/// `SigmaType : (A : Type) → (A → Type) → Type`
///
/// Dependent sum (Σ-type) formation rule.
pub fn sigma_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", arrow(bvar(0), type0()), type0()),
    )
}
/// `IdentityType : (A : Type) → A → A → Type`
///
/// Martin-Löf identity type (propositional equality).
pub fn identity_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(BinderInfo::Default, "b", bvar(1), type0()),
        ),
    )
}
/// `UnitType : Type`
///
/// The terminal type (1-type) with a single inhabitant.
pub fn unit_type_ty() -> Expr {
    type0()
}
/// `EmptyType : Type`
///
/// The initial type (0-type) with no inhabitants.
pub fn empty_type_ty() -> Expr {
    type0()
}
/// `WType : (A : Type) → (A → Type) → Type`
///
/// W-types: well-founded trees for inductive definitions.
pub fn w_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", arrow(bvar(0), type0()), type0()),
    )
}
/// `UnivalenceAxiom : (A B : Type) → (A ≃ B) ≃ (A = B)`
///
/// Voevodsky's univalence axiom.
pub fn univalence_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            app2(
                cst("Equiv"),
                app2(cst("Equiv"), bvar(1), bvar(0)),
                app3(cst("Id"), type0(), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `FunctionExtensionality : ∀ (A B : Type) (f g : A → B), (∀ x, f x = g x) → f = g`
///
/// Funext axiom.
pub fn function_extensionality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(2), bvar(1)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            app2(cst("Eq"), app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
                        ),
                        app2(cst("Eq"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `PropositionalExtensionality : ∀ (P Q : Prop), (P ↔ Q) → P = Q`
pub fn propositional_extensionality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "Q",
            prop(),
            arrow(
                app2(cst("Iff"), bvar(1), bvar(0)),
                app2(cst("Eq"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `HITsExistence : ∀ (spec : Type), HIT spec`
///
/// Existence of higher inductive types (circles, suspensions, pushouts).
pub fn hits_existence_ty() -> Expr {
    arrow(type0(), cst("HIT"))
}
/// `JEliminator : ∀ (A : Type) (C : ∀ (x y : A), x = y → Type)
///                  (d : ∀ x, C x x refl) (x y : A) (p : x = y), C x y p`
///
/// The J-rule: identity eliminator.
pub fn j_eliminator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            pi(
                BinderInfo::Default,
                "x",
                bvar(0),
                pi(
                    BinderInfo::Default,
                    "y",
                    bvar(1),
                    arrow(app3(cst("Id"), bvar(2), bvar(1), bvar(0)), type0()),
                ),
            ),
            pi(
                BinderInfo::Default,
                "d",
                pi(
                    BinderInfo::Default,
                    "x",
                    bvar(1),
                    app3(cst("JMotiveRefl"), bvar(2), bvar(0), bvar(0)),
                ),
                pi(
                    BinderInfo::Default,
                    "x",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "y",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "p",
                            app3(cst("Id"), bvar(4), bvar(1), bvar(0)),
                            app3(cst("JMotive"), bvar(5), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Transport : ∀ (A : Type) (P : A → Type) (a b : A), a = b → P a → P b`
///
/// Transport (covariant substitution) along paths.
pub fn transport_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "P",
            arrow(bvar(0), type0()),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    arrow(
                        app3(cst("Id"), bvar(3), bvar(1), bvar(0)),
                        arrow(app(bvar(3), bvar(1)), app(bvar(3), bvar(0))),
                    ),
                ),
            ),
        ),
    )
}
/// `Congr : ∀ (A B : Type) (f : A → B) (a b : A), a = b → f a = f b`
///
/// Congruence: functions respect equality.
pub fn congr_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "b",
                        bvar(3),
                        arrow(
                            app3(cst("Id"), bvar(4), bvar(1), bvar(0)),
                            app3(
                                cst("Id"),
                                bvar(4),
                                app(bvar(3), bvar(1)),
                                app(bvar(3), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Sym : ∀ (A : Type) (a b : A), a = b → b = a`
///
/// Symmetry of identity.
pub fn sym_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(
                BinderInfo::Default,
                "b",
                bvar(1),
                arrow(
                    app3(cst("Id"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("Id"), bvar(2), bvar(0), bvar(1)),
                ),
            ),
        ),
    )
}
/// `Trans : ∀ (A : Type) (a b c : A), a = b → b = c → a = c`
///
/// Transitivity of identity.
pub fn trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(
                BinderInfo::Default,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "c",
                    bvar(2),
                    arrow(
                        app3(cst("Id"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("Id"), bvar(3), bvar(1), bvar(0)),
                            app3(cst("Id"), bvar(3), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `UIPConsistent : ∀ (A : Type) (a b : A) (p q : a = b), p = q`
///
/// Uniqueness of Identity Proofs (consistent with K axiom).
pub fn uip_consistent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            pi(
                BinderInfo::Default,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "p",
                    app3(cst("Id"), bvar(2), bvar(1), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "q",
                        app3(cst("Id"), bvar(3), bvar(2), bvar(1)),
                        app3(
                            cst("Id"),
                            app3(cst("Id"), bvar(4), bvar(3), bvar(2)),
                            bvar(1),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `HomotopyLevel : (n : Nat) → Type → Prop`
///
/// Definition of being a homotopy n-type.
pub fn homotopy_level_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `Contractible : Type → Prop`
///
/// A type A is contractible if ∃ a : A, ∀ x : A, x = a.
pub fn contractible_ty() -> Expr {
    arrow(type0(), prop())
}
/// `STLCTypingVar : ∀ (Γ : Ctx) (x : Var) (T : STLCType), HasType Γ x T`
///
/// Variable typing rule for the simply typed lambda calculus (STLC).
pub fn stlc_typing_var_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "x",
            cst("Var"),
            pi(
                BinderInfo::Default,
                "T",
                cst("STLCType"),
                app3(cst("HasType"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `STLCTypingApp : ∀ (Γ : Ctx) (f a : Term) (S T : STLCType),
///     HasType Γ f (S → T) → HasType Γ a S → HasType Γ (App f a) T`
///
/// Application typing rule for STLC.
pub fn stlc_typing_app_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "f",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "a",
                cst("Term"),
                pi(
                    BinderInfo::Default,
                    "S",
                    cst("STLCType"),
                    pi(
                        BinderInfo::Default,
                        "T",
                        cst("STLCType"),
                        arrow(
                            app3(
                                cst("HasType"),
                                bvar(4),
                                bvar(3),
                                app2(cst("FunTy"), bvar(1), bvar(0)),
                            ),
                            arrow(
                                app3(cst("HasType"), bvar(4), bvar(2), bvar(1)),
                                app3(
                                    cst("HasType"),
                                    bvar(4),
                                    app2(cst("TmApp"), bvar(3), bvar(2)),
                                    bvar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `STLCTypingLam : ∀ (Γ : Ctx) (x : Var) (S T : STLCType) (b : Term),
///     HasType (Γ, x : S) b T → HasType Γ (Lam x S b) (S → T)`
///
/// Lambda abstraction typing rule for STLC.
pub fn stlc_typing_lam_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "x",
            cst("Var"),
            pi(
                BinderInfo::Default,
                "S",
                cst("STLCType"),
                pi(
                    BinderInfo::Default,
                    "T",
                    cst("STLCType"),
                    pi(
                        BinderInfo::Default,
                        "b",
                        cst("Term"),
                        arrow(
                            app3(
                                cst("HasType"),
                                app3(cst("CtxExtend"), bvar(4), bvar(3), bvar(2)),
                                bvar(0),
                                bvar(1),
                            ),
                            app3(
                                cst("HasType"),
                                bvar(4),
                                app3(cst("TmLam"), bvar(3), bvar(2), bvar(0)),
                                app2(cst("FunTy"), bvar(2), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `SystemFUniversalIntro : ∀ (Γ : Ctx) (α : TyVar) (t : Term) (T : FType),
///     HasTypeSF (Γ, α type) t T → HasTypeSF Γ (TyAbs α t) (Forall α T)`
///
/// System F type abstraction (introduction of universal quantification).
pub fn systemf_universal_intro_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "alpha",
            cst("TyVar"),
            pi(
                BinderInfo::Default,
                "t",
                cst("Term"),
                pi(
                    BinderInfo::Default,
                    "T",
                    cst("FType"),
                    arrow(
                        app3(
                            cst("HasTypeSF"),
                            app2(cst("CtxTyExtend"), bvar(3), bvar(2)),
                            bvar(1),
                            bvar(0),
                        ),
                        app3(
                            cst("HasTypeSF"),
                            bvar(3),
                            app2(cst("TyAbs"), bvar(2), bvar(1)),
                            app2(cst("Forall"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `SystemFUniversalElim : ∀ (Γ : Ctx) (t : Term) (α : TyVar) (T S : FType),
///     HasTypeSF Γ t (Forall α T) → HasTypeSF Γ (TyApp t S) (T[S/α])`
///
/// System F type application (elimination of universal quantification).
pub fn systemf_universal_elim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "alpha",
                cst("TyVar"),
                pi(
                    BinderInfo::Default,
                    "T",
                    cst("FType"),
                    pi(
                        BinderInfo::Default,
                        "S",
                        cst("FType"),
                        arrow(
                            app3(
                                cst("HasTypeSF"),
                                bvar(4),
                                bvar(3),
                                app2(cst("Forall"), bvar(2), bvar(1)),
                            ),
                            app3(
                                cst("HasTypeSF"),
                                bvar(4),
                                app2(cst("TyApp"), bvar(3), bvar(0)),
                                app3(cst("TySubst"), bvar(1), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Parametricity : ∀ (t : Term) (T : FType), WellTypedSF t T → Parametric t T`
///
/// Reynolds' parametricity theorem for System F.
pub fn parametricity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "T",
            cst("FType"),
            arrow(
                app2(cst("WellTypedSF"), bvar(1), bvar(0)),
                app2(cst("Parametric"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `SystemFOmegaKinding : ∀ (Δ : KindCtx) (T : FwType) (κ : Kind),
///     HasKind Δ T κ → WellKinded Δ T`
///
/// Kind-checking judgment for System Fω type operators.
pub fn systemfomega_kinding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Delta",
        cst("KindCtx"),
        pi(
            BinderInfo::Default,
            "T",
            cst("FwType"),
            pi(
                BinderInfo::Default,
                "kappa",
                cst("Kind"),
                arrow(
                    app3(cst("HasKind"), bvar(2), bvar(1), bvar(0)),
                    app2(cst("WellKinded"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// `TypeOperatorApp : ∀ (Δ : KindCtx) (F G : FwType) (κ₁ κ₂ : Kind),
///     HasKind Δ F (κ₁ ⇒ κ₂) → HasKind Δ G κ₁ → HasKind Δ (F G) κ₂`
///
/// Kind-level application in System Fω.
pub fn type_operator_app_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Delta",
        cst("KindCtx"),
        pi(
            BinderInfo::Default,
            "F",
            cst("FwType"),
            pi(
                BinderInfo::Default,
                "G",
                cst("FwType"),
                pi(
                    BinderInfo::Default,
                    "k1",
                    cst("Kind"),
                    pi(
                        BinderInfo::Default,
                        "k2",
                        cst("Kind"),
                        arrow(
                            app3(
                                cst("HasKind"),
                                bvar(4),
                                bvar(3),
                                app2(cst("KindArr"), bvar(1), bvar(0)),
                            ),
                            arrow(
                                app3(cst("HasKind"), bvar(4), bvar(2), bvar(1)),
                                app3(
                                    cst("HasKind"),
                                    bvar(4),
                                    app2(cst("TyAppFw"), bvar(3), bvar(2)),
                                    bvar(0),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `PTSAxiom : ∀ (s₁ s₂ : Sort), PTSAxioms s₁ s₂ → HasType EmptyCtx s₁ s₂`
///
/// Axiom rule of pure type systems (PTS).
pub fn pts_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s1",
        cst("Sort"),
        pi(
            BinderInfo::Default,
            "s2",
            cst("Sort"),
            arrow(
                app2(cst("PTSAxioms"), bvar(1), bvar(0)),
                app3(cst("HasType"), cst("EmptyCtx"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `PTSRule : ∀ (s₁ s₂ s₃ : Sort) (Γ : Ctx) (A B : Term),
///     PTSRules s₁ s₂ s₃ → HasType Γ A s₁ → HasType (Γ, x : A) B s₂
///     → HasType Γ (Π x : A. B) s₃`
///
/// Product formation rule of PTS (allows dependent products).
pub fn pts_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s1",
        cst("Sort"),
        pi(
            BinderInfo::Default,
            "s2",
            cst("Sort"),
            pi(
                BinderInfo::Default,
                "s3",
                cst("Sort"),
                pi(
                    BinderInfo::Default,
                    "Gamma",
                    cst("Ctx"),
                    pi(
                        BinderInfo::Default,
                        "A",
                        cst("Term"),
                        pi(
                            BinderInfo::Default,
                            "B",
                            cst("Term"),
                            arrow(
                                app3(cst("PTSRules"), bvar(5), bvar(4), bvar(3)),
                                arrow(
                                    app3(cst("HasType"), bvar(2), bvar(1), bvar(5)),
                                    arrow(
                                        app3(
                                            cst("HasType"),
                                            app2(cst("CtxExtend"), bvar(2), bvar(1)),
                                            bvar(1),
                                            bvar(4),
                                        ),
                                        app3(
                                            cst("HasType"),
                                            bvar(2),
                                            app2(cst("PiTm"), bvar(1), bvar(1)),
                                            bvar(3),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CoC_SortsProp : PTSAxioms Prop Type`
///
/// The Calculus of Constructions has `Prop : Type` as one of its axioms.
pub fn coc_sorts_prop_ty() -> Expr {
    app2(cst("PTSAxioms"), cst("Prop"), cst("Type"))
}
/// `CoC_PropImpredicative : PTSRules Prop Prop Prop`
///
/// CoC allows propositions to quantify over propositions (impredicativity of Prop).
pub fn coc_prop_impredicative_ty() -> Expr {
    app3(cst("PTSRules"), cst("Prop"), cst("Prop"), cst("Prop"))
}
/// `StrongNormalization : ∀ (t : Term) (T : Type), WellTyped t T → Normalizes t`
///
/// Strong normalization theorem: every well-typed term in STLC (and CoC) terminates.
pub fn strong_normalization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            arrow(
                app2(cst("WellTyped"), bvar(1), bvar(0)),
                app(cst("Normalizes"), bvar(1)),
            ),
        ),
    )
}
/// `SubjectReduction : ∀ (t t' : Term) (T : Type),
///     WellTyped t T → BetaStep t t' → WellTyped t' T`
///
/// Subject reduction (type preservation): beta-reduction preserves types.
pub fn subject_reduction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "t_prime",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(
                    app2(cst("WellTyped"), bvar(2), bvar(0)),
                    arrow(
                        app2(cst("BetaStep"), bvar(2), bvar(1)),
                        app2(cst("WellTyped"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `ProgressTheorem : ∀ (t : Term) (T : Type),
///     WellTyped t T → Value t ∨ ∃ t', BetaStep t t'`
///
/// Progress: a well-typed closed term is either a value or can step.
pub fn progress_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            arrow(
                app2(cst("WellTyped"), bvar(1), bvar(0)),
                app2(
                    cst("Or"),
                    app(cst("Value"), bvar(1)),
                    app(cst("Exists"), app(cst("BetaStep"), bvar(1))),
                ),
            ),
        ),
    )
}
/// `Confluence : ∀ (t s u : Term),
///     BetaRed t s → BetaRed t u → ∃ v, BetaRed s v ∧ BetaRed u v`
///
/// Church-Rosser (confluence) of beta-reduction.
pub fn confluence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "s",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "u",
                cst("Term"),
                arrow(
                    app2(cst("BetaRed"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("BetaRed"), bvar(2), bvar(0)),
                        app(
                            cst("Exists"),
                            app2(
                                cst("And"),
                                app2(cst("BetaRed"), bvar(1), cst("_v")),
                                app2(cst("BetaRed"), bvar(0), cst("_v")),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CurryHoward : ∀ (P : Prop) (t : Term), IsProof t P ↔ HasType t P`
///
/// The Curry-Howard isomorphism: propositions are types, proofs are programs.
pub fn curry_howard_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            app2(
                cst("Iff"),
                app2(cst("IsProof"), bvar(0), bvar(1)),
                app2(cst("HasType"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `PropsAsTypes : Prop → Type`
///
/// The embedding of propositions into types (Prop is a subtype of Type in CIC).
pub fn props_as_types_ty() -> Expr {
    arrow(prop(), type0())
}
/// `ProofsAsTerms : ∀ (P : Prop), P → Term`
///
/// Every proof of a proposition gives a term (the computational content).
pub fn proofs_as_terms_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        arrow(bvar(0), cst("Term")),
    )
}
/// `DeBruijnLift : ∀ (t : DBTerm) (c d : Nat), DBTerm`
///
/// De Bruijn index shifting (lifting): `↑_c^d t` shifts free variables in `t`
/// at depth ≥ c by d positions.
pub fn de_bruijn_lift_ty() -> Expr {
    arrow(
        cst("DBTerm"),
        arrow(nat_ty(), arrow(nat_ty(), cst("DBTerm"))),
    )
}
/// `DeBruijnSubst : ∀ (t s : DBTerm) (k : Nat), DBTerm`
///
/// De Bruijn simultaneous substitution: `t[s/k]` replaces index k in t with s.
pub fn de_bruijn_subst_ty() -> Expr {
    arrow(
        cst("DBTerm"),
        arrow(cst("DBTerm"), arrow(nat_ty(), cst("DBTerm"))),
    )
}
/// `HereditarySubstitution : ∀ (t s : Term) (k : Nat) (T : STLCType),
///     WellTyped t T → WellTyped s T → WellTyped (Subst t s k) T`
///
/// Hereditary substitution preserves types (used in normalization proofs).
pub fn hereditary_substitution_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        cst("Term"),
        pi(
            BinderInfo::Default,
            "s",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "T",
                    cst("STLCType"),
                    arrow(
                        app2(cst("WellTyped"), bvar(3), bvar(0)),
                        arrow(
                            app2(cst("WellTyped"), bvar(2), bvar(0)),
                            app2(
                                cst("WellTyped"),
                                app3(cst("Subst"), bvar(3), bvar(2), bvar(1)),
                                bvar(0),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `LFKind : ∀ (Σ : Signature) (Γ : Ctx) (K : LFKind), WellFormedKind Σ Γ K`
///
/// Well-formedness of kinds in LF (Logical Framework / Edinburgh LF).
pub fn lf_kind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Sigma",
        cst("Signature"),
        pi(
            BinderInfo::Default,
            "Gamma",
            cst("Ctx"),
            pi(
                BinderInfo::Default,
                "K",
                cst("LFKind"),
                app3(cst("WellFormedKind"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `LFTypeFamily : ∀ (Σ : Signature) (Γ : Ctx) (A : LFType) (K : LFKind),
///     HasKindLF Σ Γ A K → WellTypedFamily Σ Γ A`
///
/// Well-typedness of type families in LF.
pub fn lf_type_family_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Sigma",
        cst("Signature"),
        pi(
            BinderInfo::Default,
            "Gamma",
            cst("Ctx"),
            pi(
                BinderInfo::Default,
                "A",
                cst("LFType"),
                pi(
                    BinderInfo::Default,
                    "K",
                    cst("LFKind"),
                    arrow(
                        app4(cst("HasKindLF"), bvar(3), bvar(2), bvar(1), bvar(0)),
                        app3(cst("WellTypedFamily"), bvar(3), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `BiCheckMode : ∀ (Γ : Ctx) (t : Term) (T : Type), Checks Γ t T`
///
/// Checking mode of bidirectional type checking: verify t has type T.
pub fn bidir_check_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                app3(cst("Checks"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `BiSynthMode : ∀ (Γ : Ctx) (t : Term), ∃ T, Synths Γ t T`
///
/// Synthesis mode of bidirectional type checking: infer the type of t.
pub fn bidir_synth_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            app(cst("Exists"), app2(cst("Synths"), bvar(1), bvar(0))),
        ),
    )
}
/// `BiSubsumption : ∀ (Γ : Ctx) (t : Term) (T S : Type),
///     Synths Γ t T → T <: S → Checks Γ t S`
///
/// Subsumption rule for bidirectional type checking.
pub fn bidir_subsumption_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                pi(
                    BinderInfo::Default,
                    "S",
                    type0(),
                    arrow(
                        app3(cst("Synths"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app2(cst("Subtype"), bvar(1), bvar(0)),
                            app3(cst("Checks"), bvar(3), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `WTypeElim : ∀ (A : Type) (B : A → Type) (P : W A B → Type),
///     (∀ (a : A) (f : B a → W A B), (∀ b, P (f b)) → P (sup a f))
///     → ∀ (w : W A B), P w`
///
/// Elimination principle for W-types (well-founded trees).
pub fn w_type_elim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            arrow(bvar(0), type0()),
            pi(
                BinderInfo::Default,
                "P",
                arrow(app2(cst("WTy"), bvar(1), bvar(0)), type0()),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(2),
                        pi(
                            BinderInfo::Default,
                            "f",
                            arrow(app(bvar(2), bvar(0)), app2(cst("WTy"), bvar(3), bvar(2))),
                            arrow(
                                pi(
                                    BinderInfo::Default,
                                    "b",
                                    app(bvar(2), bvar(0)),
                                    app(bvar(3), app(bvar(1), bvar(0))),
                                ),
                                app(bvar(3), app2(cst("WTySup"), bvar(1), bvar(0))),
                            ),
                        ),
                    ),
                    pi(
                        BinderInfo::Default,
                        "w",
                        app2(cst("WTy"), bvar(2), bvar(1)),
                        app(bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `CoInductiveStream : ∀ (A : Type), Stream A → (A × Stream A)`
///
/// Coinductive stream destructor (corecursion principle).
pub fn coinductive_stream_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(
            app(cst("Stream"), bvar(0)),
            app2(cst("Prod"), bvar(0), app(cst("Stream"), bvar(0))),
        ),
    )
}
/// `CoinductionPrinciple : ∀ (A : Type) (R : Stream A → Stream A → Prop),
///     (∀ s t, R s t → HeadEq s t ∧ R (Tail s) (Tail t))
///     → ∀ s t, R s t → s = t`
///
/// Coinduction principle: bisimulation implies stream equality.
pub fn coinduction_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            arrow(
                app(cst("Stream"), bvar(0)),
                arrow(app(cst("Stream"), bvar(1)), prop()),
            ),
            arrow(
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Stream"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "t",
                        app(cst("Stream"), bvar(2)),
                        arrow(
                            app2(bvar(2), bvar(1), bvar(0)),
                            app2(
                                cst("And"),
                                app2(cst("HeadEq"), bvar(1), bvar(0)),
                                app2(
                                    bvar(2),
                                    app(cst("Tail"), bvar(1)),
                                    app(cst("Tail"), bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
                pi(
                    BinderInfo::Default,
                    "s",
                    app(cst("Stream"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "t",
                        app(cst("Stream"), bvar(2)),
                        arrow(
                            app2(bvar(2), bvar(1), bvar(0)),
                            app3(cst("Id"), app(cst("Stream"), bvar(3)), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `SessionTypeDuality : ∀ (S : SessionType), Dual (Dual S) = S`
///
/// Session type duality is an involution.
pub fn session_type_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("SessionType"),
        app3(
            cst("Id"),
            cst("SessionType"),
            app(cst("Dual"), app(cst("Dual"), bvar(0))),
            bvar(0),
        ),
    )
}
/// `LinearTypeConsumption : ∀ (A : LinType) (t : LinTerm),
///     LinearlyTyped t A → UsedExactlyOnce A t`
///
/// Linear typing ensures every resource is used exactly once.
pub fn linear_type_consumption_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("LinType"),
        pi(
            BinderInfo::Default,
            "t",
            cst("LinTerm"),
            arrow(
                app2(cst("LinearlyTyped"), bvar(0), bvar(1)),
                app2(cst("UsedExactlyOnce"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `SessionComposition : ∀ (S T : SessionType) (P Q : Process),
///     TypedProcess P S → TypedProcess Q (Dual S) → TypedProcess (P ∥ Q) T`
///
/// Session-typed parallel composition: P and Q communicate via complementary sessions.
pub fn session_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("SessionType"),
        pi(
            BinderInfo::Default,
            "T",
            cst("SessionType"),
            pi(
                BinderInfo::Default,
                "P",
                cst("Process"),
                pi(
                    BinderInfo::Default,
                    "Q",
                    cst("Process"),
                    arrow(
                        app2(cst("TypedProcess"), bvar(1), bvar(3)),
                        arrow(
                            app2(cst("TypedProcess"), bvar(1), app(cst("Dual"), bvar(3))),
                            app2(
                                cst("TypedProcess"),
                                app2(cst("Par"), bvar(2), bvar(1)),
                                bvar(2),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CICDefEq : ∀ (Γ : Ctx) (t s A : Term),
///     HasType Γ t A → HasType Γ s A → DefEqual Γ t s A → DefEqual Γ s t A`
///
/// Definitional equality in CIC is symmetric.
pub fn cic_def_eq_sym_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "s",
                cst("Term"),
                pi(
                    BinderInfo::Default,
                    "A",
                    cst("Term"),
                    arrow(
                        app3(cst("HasType"), bvar(3), bvar(2), bvar(0)),
                        arrow(
                            app3(cst("HasType"), bvar(3), bvar(1), bvar(0)),
                            arrow(
                                app4(cst("DefEqual"), bvar(3), bvar(2), bvar(1), bvar(0)),
                                app4(cst("DefEqual"), bvar(3), bvar(1), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `CICConversionRule : ∀ (Γ : Ctx) (t A B : Term),
///     HasType Γ t A → DefEqual Γ A B → HasType Γ t B`
///
/// Type conversion rule in CIC: if A and B are definitionally equal, then
/// a term of type A also has type B.
pub fn cic_conversion_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        cst("Ctx"),
        pi(
            BinderInfo::Default,
            "t",
            cst("Term"),
            pi(
                BinderInfo::Default,
                "A",
                cst("Term"),
                pi(
                    BinderInfo::Default,
                    "B",
                    cst("Term"),
                    arrow(
                        app3(cst("HasType"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("DefEqual"), bvar(3), bvar(1), bvar(0)),
                            app3(cst("HasType"), bvar(3), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
/// Register all type theory axioms and theorem stubs in the given environment.
pub fn build_type_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("UniverseHierarchy", universe_hierarchy_ty()),
        ("PiType", pi_type_ty()),
        ("SigmaType", sigma_type_ty()),
        ("IdentityType", identity_type_ty()),
        ("UnitType", unit_type_ty()),
        ("EmptyType", empty_type_ty()),
        ("WType", w_type_ty()),
        ("UnivalenceAxiom", univalence_axiom_ty()),
        ("FunctionExtensionality", function_extensionality_ty()),
        (
            "PropositionalExtensionality",
            propositional_extensionality_ty(),
        ),
        ("HITsExistence", hits_existence_ty()),
        (
            "Equiv",
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                pi(BinderInfo::Default, "B", type0(), type0()),
            ),
        ),
        (
            "Id",
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(0),
                    pi(BinderInfo::Default, "b", bvar(1), prop()),
                ),
            ),
        ),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("HIT", type1()),
        (
            "JMotiveRefl",
            pi(
                BinderInfo::Default,
                "C",
                type0(),
                pi(
                    BinderInfo::Default,
                    "x",
                    type0(),
                    pi(BinderInfo::Default, "y", type0(), type0()),
                ),
            ),
        ),
        (
            "JMotive",
            pi(
                BinderInfo::Default,
                "C",
                type0(),
                pi(
                    BinderInfo::Default,
                    "x",
                    type0(),
                    pi(BinderInfo::Default, "y", type0(), type0()),
                ),
            ),
        ),
        ("j_eliminator", j_eliminator_ty()),
        ("transport", transport_ty()),
        ("congr", congr_ty()),
        ("sym", sym_ty()),
        ("trans", trans_ty()),
        ("uip_consistent", uip_consistent_ty()),
        ("homotopy_level", homotopy_level_ty()),
        ("contractible", contractible_ty()),
        ("stlc_typing_var", stlc_typing_var_ty()),
        ("stlc_typing_app", stlc_typing_app_ty()),
        ("stlc_typing_lam", stlc_typing_lam_ty()),
        ("systemf_universal_intro", systemf_universal_intro_ty()),
        ("systemf_universal_elim", systemf_universal_elim_ty()),
        ("parametricity", parametricity_ty()),
        ("systemfomega_kinding", systemfomega_kinding_ty()),
        ("type_operator_app", type_operator_app_ty()),
        ("pts_axiom", pts_axiom_ty()),
        ("pts_rule", pts_rule_ty()),
        ("coc_sorts_prop", coc_sorts_prop_ty()),
        ("coc_prop_impredicative", coc_prop_impredicative_ty()),
        ("strong_normalization", strong_normalization_ty()),
        ("subject_reduction", subject_reduction_ty()),
        ("progress_theorem", progress_theorem_ty()),
        ("confluence", confluence_ty()),
        ("curry_howard", curry_howard_ty()),
        ("props_as_types", props_as_types_ty()),
        ("proofs_as_terms", proofs_as_terms_ty()),
        ("de_bruijn_lift", de_bruijn_lift_ty()),
        ("de_bruijn_subst", de_bruijn_subst_ty()),
        ("hereditary_substitution", hereditary_substitution_ty()),
        ("lf_kind", lf_kind_ty()),
        ("lf_type_family", lf_type_family_ty()),
        ("bidir_check", bidir_check_ty()),
        ("bidir_synth", bidir_synth_ty()),
        ("bidir_subsumption", bidir_subsumption_ty()),
        ("w_type_elim", w_type_elim_ty()),
        ("coinductive_stream", coinductive_stream_ty()),
        ("coinduction_principle", coinduction_principle_ty()),
        ("session_type_duality", session_type_duality_ty()),
        ("linear_type_consumption", linear_type_consumption_ty()),
        ("session_composition", session_composition_ty()),
        ("cic_def_eq_sym", cic_def_eq_sym_ty()),
        ("cic_conversion_rule", cic_conversion_rule_ty()),
        (
            "HasType",
            arrow(
                cst("Ctx"),
                arrow(cst("Term"), arrow(cst("STLCType"), prop())),
            ),
        ),
        (
            "HasTypeSF",
            arrow(cst("Ctx"), arrow(cst("Term"), arrow(cst("FType"), prop()))),
        ),
        (
            "FunTy",
            arrow(cst("STLCType"), arrow(cst("STLCType"), cst("STLCType"))),
        ),
        ("TmApp", arrow(cst("Term"), arrow(cst("Term"), cst("Term")))),
        (
            "TmLam",
            arrow(
                cst("Var"),
                arrow(cst("STLCType"), arrow(cst("Term"), cst("Term"))),
            ),
        ),
        (
            "CtxExtend",
            arrow(
                cst("Ctx"),
                arrow(cst("Var"), arrow(cst("STLCType"), cst("Ctx"))),
            ),
        ),
        (
            "CtxTyExtend",
            arrow(cst("Ctx"), arrow(cst("TyVar"), cst("Ctx"))),
        ),
        (
            "TyAbs",
            arrow(cst("TyVar"), arrow(cst("Term"), cst("Term"))),
        ),
        (
            "TyApp",
            arrow(cst("Term"), arrow(cst("FType"), cst("Term"))),
        ),
        (
            "Forall",
            arrow(cst("TyVar"), arrow(cst("FType"), cst("FType"))),
        ),
        (
            "TySubst",
            arrow(
                cst("FType"),
                arrow(cst("TyVar"), arrow(cst("FType"), cst("FType"))),
            ),
        ),
        (
            "WellTypedSF",
            arrow(cst("Term"), arrow(cst("FType"), prop())),
        ),
        (
            "Parametric",
            arrow(cst("Term"), arrow(cst("FType"), prop())),
        ),
        (
            "HasKind",
            arrow(
                cst("KindCtx"),
                arrow(cst("FwType"), arrow(cst("Kind"), prop())),
            ),
        ),
        (
            "WellKinded",
            arrow(cst("KindCtx"), arrow(cst("FwType"), prop())),
        ),
        (
            "KindArr",
            arrow(cst("Kind"), arrow(cst("Kind"), cst("Kind"))),
        ),
        (
            "TyAppFw",
            arrow(cst("FwType"), arrow(cst("FwType"), cst("FwType"))),
        ),
        ("PTSAxioms", arrow(cst("Sort"), arrow(cst("Sort"), prop()))),
        (
            "PTSRules",
            arrow(cst("Sort"), arrow(cst("Sort"), arrow(cst("Sort"), prop()))),
        ),
        ("EmptyCtx", cst("Ctx")),
        ("Prop", cst("Sort")),
        ("Type", cst("Sort")),
        ("PiTm", arrow(cst("Term"), arrow(cst("Term"), cst("Term")))),
        ("WellTyped", arrow(cst("Term"), arrow(type0(), prop()))),
        ("Normalizes", arrow(cst("Term"), prop())),
        ("BetaStep", arrow(cst("Term"), arrow(cst("Term"), prop()))),
        ("BetaRed", arrow(cst("Term"), arrow(cst("Term"), prop()))),
        ("Value", arrow(cst("Term"), prop())),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("IsProof", arrow(cst("Term"), arrow(prop(), prop()))),
        ("DBTerm", type0()),
        (
            "Subst",
            arrow(
                cst("Term"),
                arrow(cst("Term"), arrow(nat_ty(), cst("Term"))),
            ),
        ),
        ("Signature", type0()),
        ("LFKind", type0()),
        ("LFType", type0()),
        (
            "HasKindLF",
            arrow(
                cst("Signature"),
                arrow(
                    cst("Ctx"),
                    arrow(cst("LFType"), arrow(cst("LFKind"), prop())),
                ),
            ),
        ),
        (
            "WellFormedKind",
            arrow(
                cst("Signature"),
                arrow(cst("Ctx"), arrow(cst("LFKind"), prop())),
            ),
        ),
        (
            "WellTypedFamily",
            arrow(
                cst("Signature"),
                arrow(cst("Ctx"), arrow(cst("LFType"), prop())),
            ),
        ),
        (
            "Checks",
            arrow(cst("Ctx"), arrow(cst("Term"), arrow(type0(), prop()))),
        ),
        (
            "Synths",
            arrow(cst("Ctx"), arrow(cst("Term"), arrow(type0(), prop()))),
        ),
        ("Subtype", arrow(type0(), arrow(type0(), prop()))),
        (
            "WTy",
            arrow(type0(), arrow(arrow(type0(), type0()), type0())),
        ),
        (
            "WTySup",
            arrow(type0(), arrow(arrow(type0(), type0()), type0())),
        ),
        ("Stream", arrow(type0(), type0())),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        (
            "Tail",
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                arrow(app(cst("Stream"), bvar(0)), app(cst("Stream"), bvar(0))),
            ),
        ),
        (
            "HeadEq",
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                arrow(
                    app(cst("Stream"), bvar(0)),
                    arrow(app(cst("Stream"), bvar(0)), prop()),
                ),
            ),
        ),
        ("SessionType", type0()),
        ("Dual", arrow(cst("SessionType"), cst("SessionType"))),
        ("LinType", type0()),
        ("LinTerm", type0()),
        (
            "LinearlyTyped",
            arrow(cst("LinTerm"), arrow(cst("LinType"), prop())),
        ),
        (
            "UsedExactlyOnce",
            arrow(cst("LinType"), arrow(cst("LinTerm"), prop())),
        ),
        ("Process", type0()),
        (
            "TypedProcess",
            arrow(cst("Process"), arrow(cst("SessionType"), prop())),
        ),
        (
            "Par",
            arrow(cst("Process"), arrow(cst("Process"), cst("Process"))),
        ),
        (
            "DefEqual",
            arrow(
                cst("Ctx"),
                arrow(cst("Term"), arrow(cst("Term"), arrow(cst("Term"), prop()))),
            ),
        ),
        ("Ctx", type0()),
        ("Var", type0()),
        ("STLCType", type0()),
        ("FType", type0()),
        ("TyVar", type0()),
        ("KindCtx", type0()),
        ("FwType", type0()),
        ("Kind", type0()),
        ("Sort", type0()),
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
/// Substitute `replacement` for all free occurrences of `var_name` in `term`.
pub(super) fn subst_var(term: &MlttTerm, var_name: &str, replacement: &MlttTerm) -> MlttTerm {
    match term {
        MlttTerm::Var(n) => {
            if n == var_name {
                replacement.clone()
            } else {
                term.clone()
            }
        }
        MlttTerm::Lam { binder, body } => {
            if binder == var_name {
                term.clone()
            } else {
                MlttTerm::Lam {
                    binder: binder.clone(),
                    body: Box::new(subst_var(body, var_name, replacement)),
                }
            }
        }
        MlttTerm::Pi {
            binder,
            domain,
            codomain,
        } => {
            let new_domain = subst_var(domain, var_name, replacement);
            let new_codomain = if binder == var_name {
                *codomain.clone()
            } else {
                subst_var(codomain, var_name, replacement)
            };
            MlttTerm::Pi {
                binder: binder.clone(),
                domain: Box::new(new_domain),
                codomain: Box::new(new_codomain),
            }
        }
        MlttTerm::App(f, a) => MlttTerm::App(
            Box::new(subst_var(f, var_name, replacement)),
            Box::new(subst_var(a, var_name, replacement)),
        ),
        MlttTerm::Sigma { binder, fst, snd } => {
            let new_fst = subst_var(fst, var_name, replacement);
            let new_snd = if binder == var_name {
                *snd.clone()
            } else {
                subst_var(snd, var_name, replacement)
            };
            MlttTerm::Sigma {
                binder: binder.clone(),
                fst: Box::new(new_fst),
                snd: Box::new(new_snd),
            }
        }
        MlttTerm::Pair(a, b) => MlttTerm::Pair(
            Box::new(subst_var(a, var_name, replacement)),
            Box::new(subst_var(b, var_name, replacement)),
        ),
        MlttTerm::Fst(t) => MlttTerm::Fst(Box::new(subst_var(t, var_name, replacement))),
        MlttTerm::Snd(t) => MlttTerm::Snd(Box::new(subst_var(t, var_name, replacement))),
        MlttTerm::Id { ty, lhs, rhs } => MlttTerm::Id {
            ty: Box::new(subst_var(ty, var_name, replacement)),
            lhs: Box::new(subst_var(lhs, var_name, replacement)),
            rhs: Box::new(subst_var(rhs, var_name, replacement)),
        },
        MlttTerm::Refl(t) => MlttTerm::Refl(Box::new(subst_var(t, var_name, replacement))),
        MlttTerm::J { motive, base, path } => MlttTerm::J {
            motive: Box::new(subst_var(motive, var_name, replacement)),
            base: Box::new(subst_var(base, var_name, replacement)),
            path: Box::new(subst_var(path, var_name, replacement)),
        },
        MlttTerm::Succ(t) => MlttTerm::Succ(Box::new(subst_var(t, var_name, replacement))),
        MlttTerm::NatRec {
            motive,
            base,
            step,
            n,
        } => MlttTerm::NatRec {
            motive: Box::new(subst_var(motive, var_name, replacement)),
            base: Box::new(subst_var(base, var_name, replacement)),
            step: Box::new(subst_var(step, var_name, replacement)),
            n: Box::new(subst_var(n, var_name, replacement)),
        },
        MlttTerm::Abort(t) => MlttTerm::Abort(Box::new(subst_var(t, var_name, replacement))),
        MlttTerm::Type(_)
        | MlttTerm::Nat
        | MlttTerm::Zero
        | MlttTerm::Unit
        | MlttTerm::Star
        | MlttTerm::Empty => term.clone(),
    }
}
