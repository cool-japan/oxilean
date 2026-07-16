//! In-kernel construction of the four `#QUOT` primitives.
//!
//! This module owns the canonical, universe-polymorphic types of `Quot`,
//! `Quot.mk`, `Quot.lift` and `Quot.ind`, mirroring Lean's
//! `src/kernel/quot.cpp`. The kernel *never* trusts a caller-supplied type for
//! these primitives: [`Environment::add_quot`] builds them here and installs
//! all four atomically, exactly once, and only after `Eq` is present with its
//! expected inductive shape (needed because `Quot.lift`'s soundness hypothesis
//! mentions `Eq`).
//!
//! The canonical types (with binder info per Lean):
//!
//! ```text
//! Quot      : {α : Sort u} → (α → α → Prop) → Sort u
//! Quot.mk   : {α : Sort u} → (r : α → α → Prop) → α → Quot r
//! Quot.lift : {α : Sort u} → {r : α → α → Prop} → {β : Sort v} →
//!               (f : α → β) → (∀ a b, r a b → f a = f b) → Quot r → β
//! Quot.ind  : {α : Sort u} → {r : α → α → Prop} → {β : Quot r → Prop} →
//!               (∀ a, β (Quot.mk r a)) → ∀ q : Quot r, β q
//! ```
//!
//! `Quot.sound` is **not** a `#QUOT` primitive: in Lean it is a plain `axiom`
//! declared in `Init.Prelude` and it arrives in exports as a regular axiom
//! record. Its canonical type is nevertheless owned by the kernel (see
//! [`canonical_quot_sound_type`]) so that a replayer can verify the exported
//! axiom instead of trusting it:
//!
//! ```text
//! Quot.sound : ∀ {α : Sort u} {r : α → α → Prop} {a b : α},
//!                r a b → Quot.mk r a = Quot.mk r b
//! ```

use crate::declaration::{ConstantInfo, ConstantVal, QuotKind, QuotVal};
use crate::env::EnvError;
use crate::Node;
use crate::{BinderInfo, Environment, Expr, Level, Name};
use std::rc::Rc;

/// Hierarchical name `Quot`.
fn name_quot() -> Name {
    Name::str("Quot")
}
/// Hierarchical name `Quot.mk`.
fn name_quot_mk() -> Name {
    Name::str("Quot").append_str("mk")
}
/// Hierarchical name `Quot.lift`.
fn name_quot_lift() -> Name {
    Name::str("Quot").append_str("lift")
}
/// Hierarchical name `Quot.ind`.
fn name_quot_ind() -> Name {
    Name::str("Quot").append_str("ind")
}
/// Hierarchical name `Eq`.
fn name_eq() -> Name {
    Name::str("Eq")
}

/// `Prop` = `Sort 0`.
fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// `Sort u` for a level parameter named `u`.
fn sort_param(u: &str) -> Expr {
    Expr::Sort(Level::param(Name::str(u)))
}
/// A non-dependent arrow `dom → cod` (a `Pi` whose body ignores the binder).
///
/// The caller must supply `cod` already shifted for the extra binder if it
/// mentions outer variables (here it never does, so no shift is needed).
fn arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Node::new(dom),
        Node::new(cod),
    )
}

/// The relation type `α → α → Prop`, where `α` is `Expr::BVar(alpha_idx)` in the
/// *current* context (i.e. the context in which this whole expression sits).
///
/// The two nested binders shift `α` outward by one each, so the caller passes
/// the index of `α` as seen from the position where the relation type appears.
fn rel_type(alpha_idx: u32) -> Expr {
    // Innermost `Prop` ignores both binders.
    let body = prop();
    // `b : α` — under one extra binder, α is `alpha_idx + 1`.
    let inner = Expr::Pi(
        BinderInfo::Default,
        Name::str("b"),
        Node::new(Expr::BVar(alpha_idx + 1)),
        Node::new(body),
    );
    // `a : α` — α is `alpha_idx` here.
    Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(Expr::BVar(alpha_idx)),
        Node::new(inner),
    )
}

/// The canonical type of `Quot`:
/// `{α : Sort u} → (α → α → Prop) → Sort u`.
fn quot_type() -> Expr {
    // Body of the `r` binder: `Sort u`. Level params are name-based, so no
    // de Bruijn shifting is needed for the sort.
    let body = sort_param("u");
    // `(r : α → α → Prop)` — α is BVar(0) at this depth.
    let r_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Node::new(rel_type(0)),
        Node::new(body),
    );
    // `{α : Sort u}`.
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_param("u")),
        Node::new(r_pi),
    )
}

/// `@Quot α r` where `α = BVar(alpha_idx)` and `r = BVar(r_idx)`.
fn quot_app(alpha_idx: u32, r_idx: u32) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(name_quot(), vec![Level::param(Name::str("u"))])),
            Node::new(Expr::BVar(alpha_idx)),
        )),
        Node::new(Expr::BVar(r_idx)),
    )
}

/// The canonical type of `Quot.mk`:
/// `{α : Sort u} → (r : α → α → Prop) → α → Quot r`.
fn quot_mk_type() -> Expr {
    // Context after binding α (idx grows as 0 on the outside):
    //   α (BVar increases inward), r, (element).
    // Innermost result `@Quot α r`. In the context of the element binder:
    //   element = BVar(0), r = BVar(1), α = BVar(2).
    let result = quot_app(2, 1);
    // `(a : α)` — the quotiented element. α is BVar(1) here.
    let elem_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(Expr::BVar(1)),
        Node::new(result),
    );
    // `(r : α → α → Prop)` — α is BVar(0) here.
    let r_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Node::new(rel_type(0)),
        Node::new(elem_pi),
    );
    // `{α : Sort u}`.
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_param("u")),
        Node::new(r_pi),
    )
}

/// `@Eq.{lvl} ty lhs rhs`.
fn eq_app(lvl: Level, ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(name_eq(), vec![lvl])),
                Node::new(ty),
            )),
            Node::new(lhs),
        )),
        Node::new(rhs),
    )
}

/// The canonical type of `Quot.lift`:
/// `{α : Sort u} → {r : α → α → Prop} → {β : Sort v} → (f : α → β) →
///    (∀ a b, r a b → f a = f b) → Quot r → β`.
fn quot_lift_type() -> Expr {
    let lvl_v = Level::param(Name::str("v"));
    // Binders, outermost → innermost: α, r, β, f, h, q.
    //
    // Build from the innermost result outward.
    //
    // Result type `β`. In the context of `q` binder:
    //   q = 0, h = 1, f = 2, β = 3, r = 4, α = 5.
    let result = Expr::BVar(3);
    // `Quot r` — the `q` binder domain. In the context *before* `q` (i.e. inside
    // the `h` binder body position): h = 0, f = 1, β = 2, r = 3, α = 4.
    let q_dom = quot_app(4, 3);
    let q_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("q"),
        Node::new(q_dom),
        Node::new(result),
    );
    // The soundness hypothesis `h : ∀ (a b : α), r a b → f a = f b`.
    //
    // In the context before `h`: f = 0, β = 1, r = 2, α = 3.
    // We add binders a, b, then the arrow `r a b → f a = f b`.
    //
    // In the context after binding a, b: b = 0, a = 1, f = 2, β = 3, r = 4,
    // α = 5.
    //
    // Domain `r a b`:
    let r_a_b = Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::BVar(4)), // r
            Node::new(Expr::BVar(1)), // a
        )),
        Node::new(Expr::BVar(0)), // b
    );
    // Codomain `f a = f b`. It sits *under* the non-dependent arrow binder
    // (`_ : r a b`), so every variable shifts by 1: _ = 0, b = 1, a = 2, f = 3,
    // β = 4, r = 5, α = 6.
    let eq_fa_fb = {
        let f_a = Expr::App(Node::new(Expr::BVar(3)), Node::new(Expr::BVar(2)));
        let f_b = Expr::App(Node::new(Expr::BVar(3)), Node::new(Expr::BVar(1)));
        eq_app(lvl_v.clone(), Expr::BVar(4), f_a, f_b)
    };
    let impl_arrow = arrow(r_a_b, eq_fa_fb);
    // `∀ (b : α), r a b → f a = f b`. b domain is α. In the context after
    // binding a: α = BVar(4) (α=3 before `a`, +1 for `a`).
    let b_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("b"),
        Node::new(Expr::BVar(4)),
        Node::new(impl_arrow),
    );
    // `∀ (a : α), ...`. a domain is α. Before `a`, α = BVar(3).
    let a_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(Expr::BVar(3)),
        Node::new(b_pi),
    );
    let h_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("h"),
        Node::new(a_pi),
        Node::new(q_pi),
    );
    // `(f : α → β)`. In the context before `f`: β = 0, r = 1, α = 2.
    // Domain `α → β`: α = BVar(2); β under the `α` binder of the arrow = BVar(1)
    // shifted by the arrow binder → β = BVar(1). Rebuild explicitly.
    let f_dom = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Node::new(Expr::BVar(2)), // α
        Node::new(Expr::BVar(1)), // β (shifted by the arrow binder)
    );
    let f_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Node::new(f_dom),
        Node::new(h_pi),
    );
    // `{β : Sort v}`. Before `β`: r = 0, α = 1.
    let beta_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("β"),
        Node::new(Expr::Sort(lvl_v)),
        Node::new(f_pi),
    );
    // `{r : α → α → Prop}`. Before `r`: α = 0.
    let r_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Node::new(rel_type(0)),
        Node::new(beta_pi),
    );
    // `{α : Sort u}`.
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_param("u")),
        Node::new(r_pi),
    )
}

/// The canonical type of `Quot.ind`:
/// `{α : Sort u} → {r : α → α → Prop} → {β : Quot r → Prop} →
///    (∀ a, β (Quot.mk r a)) → ∀ q : Quot r, β q`.
fn quot_ind_type() -> Expr {
    // Binders, outermost → innermost: α, r, β, m (minor premise), q.
    //
    // Result `β q`. In the context of `q`: q = 0, m = 1, β = 2, r = 3, α = 4.
    let result = Expr::App(Node::new(Expr::BVar(2)), Node::new(Expr::BVar(0)));
    // `q : Quot r`. Before `q`: m = 0, β = 1, r = 2, α = 3.
    let q_dom = quot_app(3, 2);
    let q_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("q"),
        Node::new(q_dom),
        Node::new(result),
    );
    // Minor premise `m : ∀ (a : α), β (Quot.mk r a)`.
    // Before `m`: β = 0, r = 1, α = 2.
    // Under the `a` binder: β = 1, r = 2, α = 3, a = 0.
    //
    // `Quot.mk α r a` = `@Quot.mk α r a`.
    let mk_app = Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(
                    name_quot_mk(),
                    vec![Level::param(Name::str("u"))],
                )),
                Node::new(Expr::BVar(3)), // α
            )),
            Node::new(Expr::BVar(2)), // r
        )),
        Node::new(Expr::BVar(0)), // a
    );
    // `β (Quot.mk α r a)` — β = BVar(1).
    let beta_mk = Expr::App(Node::new(Expr::BVar(1)), Node::new(mk_app));
    // `∀ (a : α), β (Quot.mk r a)`. a domain α = BVar(2) before `a`.
    let a_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(Expr::BVar(2)),
        Node::new(beta_mk),
    );
    let m_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Node::new(a_pi),
        Node::new(q_pi),
    );
    // `{β : Quot r → Prop}`. Before `β`: r = 0, α = 1.
    // The domain `Quot r → Prop` = `Pi(_ : Quot r, Prop)`. The `Quot r` is the
    // *domain* of the arrow, evaluated in the same context (NOT under the `_`
    // binder), so α = BVar(1), r = BVar(0). Only the codomain `Prop` sits under
    // the `_` binder (and it ignores it).
    let beta_dom = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Node::new(quot_app(1, 0)),
        Node::new(prop()),
    );
    let beta_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("β"),
        Node::new(beta_dom),
        Node::new(m_pi),
    );
    // `{r : α → α → Prop}`. Before `r`: α = 0.
    let r_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Node::new(rel_type(0)),
        Node::new(beta_pi),
    );
    // `{α : Sort u}`.
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_param("u")),
        Node::new(r_pi),
    )
}

/// The canonical universe-polymorphic type for a given quotient kind.
///
/// - [`QuotKind::Type`], [`QuotKind::Mk`] use only the level parameter `u`.
/// - [`QuotKind::Lift`] additionally uses `v`.
/// - [`QuotKind::Ind`] uses only `u`.
///
/// The returned type is closed (no loose bound variables) and is the exact
/// type the kernel installs / requires for the primitive.
pub fn canonical_quot_type(kind: QuotKind) -> Expr {
    match kind {
        QuotKind::Type => quot_type(),
        QuotKind::Mk => quot_mk_type(),
        QuotKind::Lift => quot_lift_type(),
        QuotKind::Ind => quot_ind_type(),
    }
}

/// The universe level parameters for a given quotient kind (in order).
pub fn canonical_quot_level_params(kind: QuotKind) -> Vec<Name> {
    match kind {
        QuotKind::Type | QuotKind::Mk | QuotKind::Ind => vec![Name::str("u")],
        QuotKind::Lift => vec![Name::str("u"), Name::str("v")],
    }
}

/// The hierarchical name `Quot.sound` of the quotient soundness axiom.
pub fn quot_sound_name() -> Name {
    Name::str("Quot").append_str("sound")
}

/// The canonical type of the quotient soundness **axiom** `Quot.sound`
/// (mirroring `Init.Prelude`):
///
/// ```text
/// ∀ {α : Sort u} {r : α → α → Prop} {a b : α},
///     r a b → Quot.mk r a = Quot.mk r b
/// ```
///
/// `Quot.sound` is *not* installed by [`Environment::add_quot`] — in Lean it is
/// an ordinary axiom, added after the `#QUOT` primitives. This constructor
/// exists so a replayer can validate the exported axiom's type (up to
/// definitional equality, after aligning the single level parameter) instead of
/// accepting an arbitrary assertion under the trusted name `Quot.sound`.
///
/// The returned type is closed and universe-polymorphic in
/// [`canonical_quot_sound_level_params`] (`[u]`).
pub fn canonical_quot_sound_type() -> Expr {
    let lvl_u = Level::param(Name::str("u"));
    // Binders, outermost → innermost: α, r, a, b, then the anonymous
    // hypothesis `_ : r a b`, with body `Quot.mk r a = Quot.mk r b`.
    //
    // Body context: _ = 0, b = 1, a = 2, r = 3, α = 4.
    // `@Quot α r : Sort u` — the `Eq` instance's type argument.
    let quot_alpha_r = quot_app(4, 3);
    // `@Quot.mk α r x` for x ∈ {a, b}.
    let mk = |x: u32| {
        Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(name_quot_mk(), vec![lvl_u.clone()])),
                    Node::new(Expr::BVar(4)), // α
                )),
                Node::new(Expr::BVar(3)), // r
            )),
            Node::new(Expr::BVar(x)),
        )
    };
    let (mk_a, mk_b) = (mk(2), mk(1));
    let body = eq_app(lvl_u, quot_alpha_r, mk_a, mk_b);
    // Hypothesis domain `r a b`. Context before the hypothesis binder:
    // b = 0, a = 1, r = 2, α = 3.
    let r_a_b = Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::BVar(2)), // r
            Node::new(Expr::BVar(1)), // a
        )),
        Node::new(Expr::BVar(0)), // b
    );
    let hyp_arrow = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Node::new(r_a_b),
        Node::new(body),
    );
    // `{b : α}` — before `b`: a = 0, r = 1, α = 2.
    let b_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("b"),
        Node::new(Expr::BVar(2)),
        Node::new(hyp_arrow),
    );
    // `{a : α}` — before `a`: r = 0, α = 1.
    let a_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Node::new(Expr::BVar(1)),
        Node::new(b_pi),
    );
    // `{r : α → α → Prop}` — before `r`: α = 0.
    let r_pi = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Node::new(rel_type(0)),
        Node::new(a_pi),
    );
    // `{α : Sort u}`.
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_param("u")),
        Node::new(r_pi),
    )
}

/// The universe level parameters of the canonical `Quot.sound` axiom (`[u]`).
pub fn canonical_quot_sound_level_params() -> Vec<Name> {
    vec![Name::str("u")]
}

/// Build the `QuotVal` for a canonical quotient primitive.
fn canonical_quot_val(name: Name, kind: QuotKind) -> QuotVal {
    QuotVal {
        common: ConstantVal {
            name,
            level_params: canonical_quot_level_params(kind),
            ty: canonical_quot_type(kind),
        },
        kind,
    }
}

/// Verify that `Eq` is present with its expected inductive shape:
/// a single-constructor inductive living in `Prop`, whose sole constructor is
/// registered and points back to `Eq`.
///
/// This mirrors Lean's `check_eq_type` precondition of `add_quot`: `Quot.lift`'s
/// soundness hypothesis is stated with `Eq`, so `Eq` must already exist.
///
/// The check is deliberately agnostic about the exact spelling of the
/// constructor name (`Eq.refl` may be encoded either as a flat single-atom
/// `Name` or as a hierarchical `Eq.refl`), and instead validates the structural
/// relationship (`ctor.induct == Eq`).
fn check_eq_present(env: &Environment) -> Result<(), EnvError> {
    let eq_name = name_eq();
    let iv = env.get_inductive_val(&eq_name).ok_or_else(|| {
        EnvError::InvalidQuotient("`Eq` must be declared before quotients".into())
    })?;
    // Lean's `Eq` is a `Prop`-valued inductive with exactly one constructor.
    if iv.ctors.len() != 1 {
        return Err(EnvError::InvalidQuotient(
            "`Eq` must be an inductive with exactly one constructor".into(),
        ));
    }
    if !iv.is_prop {
        return Err(EnvError::InvalidQuotient(
            "`Eq` must be a `Prop`-valued inductive".into(),
        ));
    }
    let ctor_name = &iv.ctors[0];
    let cv = env
        .get_constructor_val(ctor_name)
        .ok_or_else(|| EnvError::InvalidQuotient("`Eq`'s constructor must be registered".into()))?;
    if cv.induct != eq_name {
        return Err(EnvError::InvalidQuotient(
            "`Eq`'s constructor does not point back to `Eq`".into(),
        ));
    }
    Ok(())
}

impl Environment {
    /// Install the four `#QUOT` primitives (`Quot`, `Quot.mk`, `Quot.lift`,
    /// `Quot.ind`) with their canonical, kernel-constructed types.
    ///
    /// This is the trusted `#QUOT` entry point, mirroring Lean's
    /// `add_quot` in `src/kernel/quot.cpp`:
    ///
    /// - **Precondition:** `Eq` must already exist as an inductive of the
    ///   expected shape (one constructor `Eq.refl`), because `Quot.lift`'s
    ///   soundness hypothesis mentions `Eq`. A typed error is returned
    ///   otherwise.
    /// - All four types are constructed **in-kernel** (universe-polymorphic in
    ///   `u`, plus `v` for `Quot.lift`); the caller never supplies them.
    /// - The four constants are added atomically. If any name already exists,
    ///   nothing is installed.
    /// - A once-only [`Environment::quot_initialized`] flag is set; a second
    ///   `add_quot` is a typed error, and any other `ConstantInfo::Quotient`
    ///   addition is rejected by [`Environment::add_constant`].
    pub fn add_quot(&mut self) -> Result<(), EnvError> {
        if self.quot_initialized() {
            return Err(EnvError::InvalidQuotient(
                "quotients have already been initialized".into(),
            ));
        }
        check_eq_present(self)?;
        let decls = [
            (name_quot(), QuotKind::Type),
            (name_quot_mk(), QuotKind::Mk),
            (name_quot_lift(), QuotKind::Lift),
            (name_quot_ind(), QuotKind::Ind),
        ];
        // Atomicity: reject up-front if any target name is taken.
        for (name, _) in &decls {
            if self.contains(name) {
                return Err(EnvError::DuplicateDeclaration(name.clone()));
            }
        }
        for (name, kind) in decls {
            let qv = canonical_quot_val(name, kind);
            self.insert_quotient(ConstantInfo::Quotient(qv));
        }
        self.set_quot_initialized();
        Ok(())
    }
}
