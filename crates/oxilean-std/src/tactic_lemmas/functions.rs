//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{SimpLemmaEntry, SimpTheorems};

/// Build a function application `f a`.
#[allow(dead_code)]
pub(super) fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build a function application `f a b`.
#[allow(dead_code)]
pub(super) fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
/// Build a function application `f a b c`.
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
/// Build `Pi (name : dom), body` with given binder info.
#[allow(dead_code)]
pub(super) fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
/// Build a non-dependent arrow `A -> B`.
#[allow(dead_code)]
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
/// Build a lambda `fun (name : dom) => body`.
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
/// Named constant with no universe levels.
#[allow(dead_code)]
pub(super) fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
/// Named constant with universe levels.
#[allow(dead_code)]
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
/// Prop: `Sort 0`.
#[allow(dead_code)]
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type 1: `Sort 1`.
#[allow(dead_code)]
pub(super) fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Sort u (universe parameter).
#[allow(dead_code)]
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
/// Bound variable.
#[allow(dead_code)]
pub(super) fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// Nat type.
#[allow(dead_code)]
pub(super) fn nat_ty() -> Expr {
    cst("Nat")
}
/// Bool type.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// Build `Iff a b` (a ↔ b).
#[allow(dead_code)]
pub fn mk_iff(a: Expr, b: Expr) -> Expr {
    app2(cst("Iff"), a, b)
}
/// Build `And a b` (a ∧ b).
#[allow(dead_code)]
pub(super) fn mk_and(a: Expr, b: Expr) -> Expr {
    app2(cst("And"), a, b)
}
/// Build `Or a b` (a ∨ b).
#[allow(dead_code)]
pub(super) fn mk_or(a: Expr, b: Expr) -> Expr {
    app2(cst("Or"), a, b)
}
/// Build `Not a` (¬a).
#[allow(dead_code)]
pub(super) fn mk_not(a: Expr) -> Expr {
    app(cst("Not"), a)
}
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub(super) fn mk_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("Eq"), ty, a, b)
}
/// Build `Eq @{} Nat a b`.
#[allow(dead_code)]
pub fn mk_nat_eq(a: Expr, b: Expr) -> Expr {
    mk_eq(nat_ty(), a, b)
}
/// Build `Eq @{} Bool a b`.
#[allow(dead_code)]
pub(super) fn mk_bool_eq(a: Expr, b: Expr) -> Expr {
    mk_eq(bool_ty(), a, b)
}
/// Build `Ne @{} ty a b` (as `Not (Eq ty a b)`).
#[allow(dead_code)]
pub fn mk_ne(ty: Expr, a: Expr, b: Expr) -> Expr {
    mk_not(mk_eq(ty, a, b))
}
/// Build `Nat.add a b`.
#[allow(dead_code)]
pub(super) fn nat_add(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.add"), a, b)
}
/// Build `Nat.mul a b`.
#[allow(dead_code)]
pub(super) fn nat_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.mul"), a, b)
}
/// Build `Nat.sub a b`.
#[allow(dead_code)]
pub fn nat_sub(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.sub"), a, b)
}
/// Build `Nat.succ n`.
#[allow(dead_code)]
pub fn nat_succ(n: Expr) -> Expr {
    app(cst("Nat.succ"), n)
}
/// Build `Nat.zero`.
#[allow(dead_code)]
pub fn nat_zero() -> Expr {
    cst("Nat.zero")
}
/// Build `Nat.lt a b`.
#[allow(dead_code)]
pub(super) fn nat_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.lt"), a, b)
}
/// Build `List @{u} ty`.
#[allow(dead_code)]
pub fn list_of(ty: Expr) -> Expr {
    app(cst("List"), ty)
}
/// Build `List.nil @{u} ty`.
#[allow(dead_code)]
pub fn list_nil(ty: Expr) -> Expr {
    app(cst("List.nil"), ty)
}
/// Build `List.append @{} ty a b`.
#[allow(dead_code)]
pub fn list_append(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("List.append"), ty, a, b)
}
/// Build `List.length @{} ty l`.
#[allow(dead_code)]
pub fn list_length(ty: Expr, l: Expr) -> Expr {
    app2(cst("List.length"), ty, l)
}
/// Build `List.map @{} ty_a ty_b f l`.
#[allow(dead_code)]
pub fn list_map(ty_a: Expr, ty_b: Expr, f: Expr, l: Expr) -> Expr {
    app(app3(cst("List.map"), ty_a, ty_b, f), l)
}
/// Build `List.reverse @{} ty l`.
#[allow(dead_code)]
pub fn list_reverse(ty: Expr, l: Expr) -> Expr {
    app2(cst("List.reverse"), ty, l)
}
/// Build `Bool.not b`.
#[allow(dead_code)]
pub fn bool_not(b: Expr) -> Expr {
    app(cst("Bool.not"), b)
}
/// Build `Bool.and a b`.
#[allow(dead_code)]
pub fn bool_and(a: Expr, b: Expr) -> Expr {
    app2(cst("Bool.and"), a, b)
}
/// Build `Bool.or a b`.
#[allow(dead_code)]
pub fn bool_or(a: Expr, b: Expr) -> Expr {
    app2(cst("Bool.or"), a, b)
}
/// Literal `Nat.zero` as a Nat literal 0.
#[allow(dead_code)]
pub fn nat_lit_zero() -> Expr {
    cst("Nat.zero")
}
/// Literal `Nat.succ Nat.zero` as 1.
#[allow(dead_code)]
pub fn nat_lit_one() -> Expr {
    nat_succ(nat_zero())
}
/// Build a `SimpLemmaEntry` from name, priority, and proof.
#[allow(dead_code)]
pub fn mk_simp_lemma(name: &str, priority: u64, proof: Expr) -> SimpLemmaEntry {
    SimpLemmaEntry::mk(Name::str(name), priority, proof)
}
#[allow(dead_code)]
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Register all built-in simp lemma entries into a `SimpTheorems` set.
///
/// The proof terms are just `Const` references to the corresponding axioms
/// that are registered in the environment.
#[allow(dead_code)]
pub fn register_default_simp_lemmas() -> SimpTheorems {
    let mut st = SimpTheorems::empty();
    let prop_lemmas = [
        "simp_true_and",
        "simp_and_true",
        "simp_false_and",
        "simp_and_false",
        "simp_true_or",
        "simp_or_true",
        "simp_false_or",
        "simp_or_false",
        "simp_not_true",
        "simp_not_false",
        "simp_not_not",
        "simp_iff_self",
        "simp_eq_self",
        "simp_true_implies",
        "simp_false_implies",
        "simp_and_self",
        "simp_or_self",
    ];
    for name in &prop_lemmas {
        st.add(mk_simp_lemma(name, 1000, cst(name)));
    }
    let nat_lemmas = [
        "simp_nat_zero_add",
        "simp_nat_add_zero",
        "simp_nat_zero_mul",
        "simp_nat_mul_zero",
        "simp_nat_one_mul",
        "simp_nat_mul_one",
        "simp_nat_succ_ne_zero",
        "simp_nat_sub_self",
    ];
    for name in &nat_lemmas {
        st.add(mk_simp_lemma(name, 900, cst(name)));
    }
    let bool_lemmas = [
        "simp_bool_not_not",
        "simp_bool_and_true",
        "simp_bool_and_false",
        "simp_bool_or_true",
        "simp_bool_or_false",
    ];
    for name in &bool_lemmas {
        st.add(mk_simp_lemma(name, 900, cst(name)));
    }
    let list_lemmas = [
        "simp_list_nil_append",
        "simp_list_append_nil",
        "simp_list_length_nil",
        "simp_list_map_nil",
        "simp_list_reverse_nil",
    ];
    for name in &list_lemmas {
        st.add(mk_simp_lemma(name, 800, cst(name)));
    }
    st
}
/// Build the tactic lemmas environment, adding simp lemmas, ext lemmas,
/// and norm_num lemmas as axioms.
///
/// Assumes `True`, `False`, `Not`, `And`, `Or`, `Iff`, `Eq`, `Nat`, `Bool`,
/// `List`, and related operations are already declared in `env`.
#[allow(clippy::too_many_lines)]
pub fn build_tactic_lemmas_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "SimpLemmaEntry", vec![], type1())?;
    let simp_entry_mk_ty = arrow(
        type1(),
        arrow(nat_ty(), arrow(type1(), cst("SimpLemmaEntry"))),
    );
    add_axiom(env, "SimpLemmaEntry.mk", vec![], simp_entry_mk_ty)?;
    add_axiom(env, "SimpTheorems", vec![], type1())?;
    add_axiom(env, "SimpTheorems.empty", vec![], cst("SimpTheorems"))?;
    let add_ty = arrow(
        cst("SimpTheorems"),
        arrow(cst("SimpLemmaEntry"), cst("SimpTheorems")),
    );
    add_axiom(env, "SimpTheorems.add", vec![], add_ty)?;
    let erase_ty = arrow(cst("SimpTheorems"), arrow(type1(), cst("SimpTheorems")));
    add_axiom(env, "SimpTheorems.erase", vec![], erase_ty)?;
    let simp_true_and_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_and(cst("True"), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_true_and", vec![], simp_true_and_ty)?;
    let simp_and_true_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_and(bvar(0), cst("True")), bvar(0)),
    );
    add_axiom(env, "simp_and_true", vec![], simp_and_true_ty)?;
    let simp_false_and_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_and(cst("False"), bvar(0)), cst("False")),
    );
    add_axiom(env, "simp_false_and", vec![], simp_false_and_ty)?;
    let simp_and_false_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_and(bvar(0), cst("False")), cst("False")),
    );
    add_axiom(env, "simp_and_false", vec![], simp_and_false_ty)?;
    let simp_true_or_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_or(cst("True"), bvar(0)), cst("True")),
    );
    add_axiom(env, "simp_true_or", vec![], simp_true_or_ty)?;
    let simp_or_true_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_or(bvar(0), cst("True")), cst("True")),
    );
    add_axiom(env, "simp_or_true", vec![], simp_or_true_ty)?;
    let simp_false_or_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_or(cst("False"), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_false_or", vec![], simp_false_or_ty)?;
    let simp_or_false_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_or(bvar(0), cst("False")), bvar(0)),
    );
    add_axiom(env, "simp_or_false", vec![], simp_or_false_ty)?;
    let simp_not_true_ty = mk_iff(mk_not(cst("True")), cst("False"));
    add_axiom(env, "simp_not_true", vec![], simp_not_true_ty)?;
    let simp_not_false_ty = mk_iff(mk_not(cst("False")), cst("True"));
    add_axiom(env, "simp_not_false", vec![], simp_not_false_ty)?;
    let simp_not_not_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_not(mk_not(bvar(0))), bvar(0)),
    );
    add_axiom(env, "simp_not_not", vec![], simp_not_not_ty)?;
    let simp_iff_self_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_iff(bvar(0), bvar(0)), cst("True")),
    );
    add_axiom(env, "simp_iff_self", vec![], simp_iff_self_ty)?;
    let simp_eq_self_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            mk_iff(mk_eq(bvar(1), bvar(0), bvar(0)), cst("True")),
        ),
    );
    add_axiom(env, "simp_eq_self", vec![Name::str("u")], simp_eq_self_ty)?;
    let simp_true_implies_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(arrow(cst("True"), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_true_implies", vec![], simp_true_implies_ty)?;
    let simp_false_implies_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(arrow(cst("False"), bvar(0)), cst("True")),
    );
    add_axiom(env, "simp_false_implies", vec![], simp_false_implies_ty)?;
    let simp_and_self_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_and(bvar(0), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_and_self", vec![], simp_and_self_ty)?;
    let simp_or_self_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_iff(mk_or(bvar(0), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_or_self", vec![], simp_or_self_ty)?;
    let simp_nat_zero_add_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_add(nat_zero(), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_nat_zero_add", vec![], simp_nat_zero_add_ty)?;
    let simp_nat_add_zero_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_add(bvar(0), nat_zero()), bvar(0)),
    );
    add_axiom(env, "simp_nat_add_zero", vec![], simp_nat_add_zero_ty)?;
    let simp_nat_zero_mul_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(nat_zero(), bvar(0)), nat_zero()),
    );
    add_axiom(env, "simp_nat_zero_mul", vec![], simp_nat_zero_mul_ty)?;
    let simp_nat_mul_zero_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(bvar(0), nat_zero()), nat_zero()),
    );
    add_axiom(env, "simp_nat_mul_zero", vec![], simp_nat_mul_zero_ty)?;
    let simp_nat_one_mul_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(nat_lit_one(), bvar(0)), bvar(0)),
    );
    add_axiom(env, "simp_nat_one_mul", vec![], simp_nat_one_mul_ty)?;
    let simp_nat_mul_one_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(bvar(0), nat_lit_one()), bvar(0)),
    );
    add_axiom(env, "simp_nat_mul_one", vec![], simp_nat_mul_one_ty)?;
    let simp_nat_succ_ne_zero_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_ne(nat_ty(), nat_succ(bvar(0)), nat_zero()),
    );
    add_axiom(
        env,
        "simp_nat_succ_ne_zero",
        vec![],
        simp_nat_succ_ne_zero_ty,
    )?;
    let simp_nat_sub_self_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_sub(bvar(0), bvar(0)), nat_zero()),
    );
    add_axiom(env, "simp_nat_sub_self", vec![], simp_nat_sub_self_ty)?;
    let simp_bool_not_not_ty = pi(
        BinderInfo::Default,
        "b",
        bool_ty(),
        mk_bool_eq(bool_not(bool_not(bvar(0))), bvar(0)),
    );
    add_axiom(env, "simp_bool_not_not", vec![], simp_bool_not_not_ty)?;
    let simp_bool_and_true_ty = pi(
        BinderInfo::Default,
        "b",
        bool_ty(),
        mk_bool_eq(bool_and(bvar(0), cst("true")), bvar(0)),
    );
    add_axiom(env, "simp_bool_and_true", vec![], simp_bool_and_true_ty)?;
    let simp_bool_and_false_ty = pi(
        BinderInfo::Default,
        "b",
        bool_ty(),
        mk_bool_eq(bool_and(bvar(0), cst("false")), cst("false")),
    );
    add_axiom(env, "simp_bool_and_false", vec![], simp_bool_and_false_ty)?;
    let simp_bool_or_true_ty = pi(
        BinderInfo::Default,
        "b",
        bool_ty(),
        mk_bool_eq(bool_or(bvar(0), cst("true")), cst("true")),
    );
    add_axiom(env, "simp_bool_or_true", vec![], simp_bool_or_true_ty)?;
    let simp_bool_or_false_ty = pi(
        BinderInfo::Default,
        "b",
        bool_ty(),
        mk_bool_eq(bool_or(bvar(0), cst("false")), bvar(0)),
    );
    add_axiom(env, "simp_bool_or_false", vec![], simp_bool_or_false_ty)?;
    let simp_list_nil_append_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "l",
            list_of(bvar(0)),
            mk_eq(
                list_of(bvar(1)),
                list_append(bvar(1), list_nil(bvar(1)), bvar(0)),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "simp_list_nil_append", vec![], simp_list_nil_append_ty)?;
    let simp_list_append_nil_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "l",
            list_of(bvar(0)),
            mk_eq(
                list_of(bvar(1)),
                list_append(bvar(1), bvar(0), list_nil(bvar(1))),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "simp_list_append_nil", vec![], simp_list_append_nil_ty)?;
    let simp_list_length_nil_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        mk_nat_eq(list_length(bvar(0), list_nil(bvar(0))), nat_zero()),
    );
    add_axiom(env, "simp_list_length_nil", vec![], simp_list_length_nil_ty)?;
    let simp_list_map_nil_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                mk_eq(
                    list_of(bvar(1)),
                    list_map(bvar(2), bvar(1), bvar(0), list_nil(bvar(2))),
                    list_nil(bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "simp_list_map_nil", vec![], simp_list_map_nil_ty)?;
    let simp_list_reverse_nil_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        mk_eq(
            list_of(bvar(0)),
            list_reverse(bvar(0), list_nil(bvar(0))),
            list_nil(bvar(0)),
        ),
    );
    add_axiom(
        env,
        "simp_list_reverse_nil",
        vec![],
        simp_list_reverse_nil_ty,
    )?;
    let ext_funext_ty = pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            Expr::Sort(Level::Param(Name::str("v"))),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(2), bvar(2)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            mk_eq(bvar(3), app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
                        ),
                        mk_eq(arrow(bvar(4), bvar(4)), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(
        env,
        "ext_funext",
        vec![Name::str("u"), Name::str("v")],
        ext_funext_ty,
    )?;
    let ext_propext_ty = pi(
        BinderInfo::Default,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "q",
            prop(),
            arrow(mk_iff(bvar(1), bvar(0)), mk_eq(prop(), bvar(1), bvar(0))),
        ),
    );
    add_axiom(env, "ext_propext", vec![], ext_propext_ty)?;
    let ext_array_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "a",
            app(cst("Array"), bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                app(cst("Array"), bvar(1)),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "i",
                        nat_ty(),
                        mk_eq(
                            bvar(3),
                            app3(cst("Array.get"), bvar(3), bvar(2), bvar(0)),
                            app3(cst("Array.get"), bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                    mk_eq(app(cst("Array"), bvar(2)), bvar(1), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "ext_array", vec![], ext_array_ty)?;
    let ext_set_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "s",
            app(cst("Set"), bvar(0)),
            pi(
                BinderInfo::Default,
                "t",
                app(cst("Set"), bvar(1)),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "x",
                        bvar(2),
                        mk_iff(
                            app3(cst("Set.mem"), bvar(3), bvar(0), bvar(2)),
                            app3(cst("Set.mem"), bvar(3), bvar(0), bvar(1)),
                        ),
                    ),
                    mk_eq(app(cst("Set"), bvar(2)), bvar(1), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "ext_set", vec![], ext_set_ty)?;
    let norm_num_zero_lt_succ_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        nat_lt(nat_zero(), nat_succ(bvar(0))),
    );
    add_axiom(
        env,
        "norm_num_zero_lt_succ",
        vec![],
        norm_num_zero_lt_succ_ty,
    )?;
    let norm_num_succ_ne_zero_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_ne(nat_ty(), nat_succ(bvar(0)), nat_zero()),
    );
    add_axiom(
        env,
        "norm_num_succ_ne_zero",
        vec![],
        norm_num_succ_ne_zero_ty,
    )?;
    let norm_num_add_comm_ty = pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            mk_nat_eq(nat_add(bvar(1), bvar(0)), nat_add(bvar(0), bvar(1))),
        ),
    );
    add_axiom(env, "norm_num_add_comm", vec![], norm_num_add_comm_ty)?;
    let norm_num_mul_comm_ty = pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            mk_nat_eq(nat_mul(bvar(1), bvar(0)), nat_mul(bvar(0), bvar(1))),
        ),
    );
    add_axiom(env, "norm_num_mul_comm", vec![], norm_num_mul_comm_ty)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Set up a minimal environment with prerequisite declarations.
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        for name in &["True", "False", "Not", "And", "Or", "Iff", "Eq"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: prop(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("True.intro"),
            univ_params: vec![],
            ty: cst("True"),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1(),
        })
        .expect("operation should succeed");
        for name in &[
            "Nat.zero", "Nat.succ", "Nat.add", "Nat.mul", "Nat.sub", "Nat.lt",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: nat_ty(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1(),
        })
        .expect("operation should succeed");
        for name in &["true", "false", "Bool.not", "Bool.and", "Bool.or"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: bool_ty(),
            })
            .expect("operation should succeed");
        }
        for name in &[
            "List",
            "List.nil",
            "List.append",
            "List.length",
            "List.map",
            "List.reverse",
            "Array",
            "Array.get",
            "Set",
            "Set.mem",
            "Membership.mem",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1(),
            })
            .expect("operation should succeed");
        }
        env
    }
    /// Build a full test environment.
    fn full_env() -> Environment {
        let mut env = setup_env();
        build_tactic_lemmas_env(&mut env).expect("build_tactic_lemmas_env should succeed");
        env
    }
    #[test]
    fn test_simp_lemma_entry_mk() {
        let entry = SimpLemmaEntry::mk(Name::str("test_lemma"), 100, cst("proof"));
        assert_eq!(entry.name, Name::str("test_lemma"));
        assert_eq!(entry.priority, 100);
    }
    #[test]
    fn test_mk_simp_lemma_helper() {
        let entry = mk_simp_lemma("my_lemma", 500, cst("pf"));
        assert_eq!(entry.name, Name::str("my_lemma"));
        assert_eq!(entry.priority, 500);
    }
    #[test]
    fn test_simp_theorems_empty() {
        let st = SimpTheorems::empty();
        assert!(st.is_empty());
        assert_eq!(st.len(), 0);
    }
    #[test]
    fn test_simp_theorems_add() {
        let mut st = SimpTheorems::empty();
        st.add(mk_simp_lemma("lemma1", 100, cst("p1")));
        st.add(mk_simp_lemma("lemma2", 200, cst("p2")));
        assert_eq!(st.len(), 2);
    }
    #[test]
    fn test_simp_theorems_priority_order() {
        let mut st = SimpTheorems::empty();
        st.add(mk_simp_lemma("low", 100, cst("p1")));
        st.add(mk_simp_lemma("high", 1000, cst("p2")));
        st.add(mk_simp_lemma("mid", 500, cst("p3")));
        assert_eq!(st.lemmas[0].name, Name::str("high"));
        assert_eq!(st.lemmas[1].name, Name::str("mid"));
        assert_eq!(st.lemmas[2].name, Name::str("low"));
    }
    #[test]
    fn test_simp_theorems_erase() {
        let mut st = SimpTheorems::empty();
        st.add(mk_simp_lemma("lemma1", 100, cst("p1")));
        st.add(mk_simp_lemma("lemma2", 200, cst("p2")));
        st.erase(&Name::str("lemma1"));
        assert_eq!(st.len(), 1);
        assert!(st.get(&Name::str("lemma1")).is_none());
        assert!(st.get(&Name::str("lemma2")).is_some());
    }
    #[test]
    fn test_simp_theorems_get() {
        let mut st = SimpTheorems::empty();
        st.add(mk_simp_lemma("target", 100, cst("pf")));
        let entry = st.get(&Name::str("target"));
        assert!(entry.is_some());
        assert_eq!(entry.expect("entry should be valid").priority, 100);
    }
    #[test]
    fn test_simp_theorems_get_missing() {
        let st = SimpTheorems::empty();
        assert!(st.get(&Name::str("nonexistent")).is_none());
    }
    #[test]
    fn test_simp_theorems_erase_nonexistent() {
        let mut st = SimpTheorems::empty();
        st.add(mk_simp_lemma("lemma1", 100, cst("p1")));
        st.erase(&Name::str("nonexistent"));
        assert_eq!(st.len(), 1);
    }
    #[test]
    fn test_simp_theorems_default() {
        let st = SimpTheorems::default();
        assert!(st.is_empty());
    }
    #[test]
    fn test_register_default_simp_lemmas_nonempty() {
        let st = register_default_simp_lemmas();
        assert!(!st.is_empty());
    }
    #[test]
    fn test_register_default_simp_lemmas_count() {
        let st = register_default_simp_lemmas();
        assert_eq!(st.len(), 35);
    }
    #[test]
    fn test_register_default_prop_lemmas_present() {
        let st = register_default_simp_lemmas();
        let prop_names = [
            "simp_true_and",
            "simp_and_true",
            "simp_false_and",
            "simp_and_false",
            "simp_true_or",
            "simp_or_true",
            "simp_false_or",
            "simp_or_false",
            "simp_not_true",
            "simp_not_false",
            "simp_not_not",
            "simp_iff_self",
            "simp_eq_self",
            "simp_true_implies",
            "simp_false_implies",
            "simp_and_self",
            "simp_or_self",
        ];
        for name in &prop_names {
            assert!(
                st.get(&Name::str(*name)).is_some(),
                "missing prop lemma: {}",
                name
            );
        }
    }
    #[test]
    fn test_register_default_nat_lemmas_present() {
        let st = register_default_simp_lemmas();
        let nat_names = [
            "simp_nat_zero_add",
            "simp_nat_add_zero",
            "simp_nat_zero_mul",
            "simp_nat_mul_zero",
            "simp_nat_one_mul",
            "simp_nat_mul_one",
            "simp_nat_succ_ne_zero",
            "simp_nat_sub_self",
        ];
        for name in &nat_names {
            assert!(
                st.get(&Name::str(*name)).is_some(),
                "missing nat lemma: {}",
                name
            );
        }
    }
    #[test]
    fn test_register_default_bool_lemmas_present() {
        let st = register_default_simp_lemmas();
        let bool_names = [
            "simp_bool_not_not",
            "simp_bool_and_true",
            "simp_bool_and_false",
            "simp_bool_or_true",
            "simp_bool_or_false",
        ];
        for name in &bool_names {
            assert!(
                st.get(&Name::str(*name)).is_some(),
                "missing bool lemma: {}",
                name
            );
        }
    }
    #[test]
    fn test_register_default_list_lemmas_present() {
        let st = register_default_simp_lemmas();
        let list_names = [
            "simp_list_nil_append",
            "simp_list_append_nil",
            "simp_list_length_nil",
            "simp_list_map_nil",
            "simp_list_reverse_nil",
        ];
        for name in &list_names {
            assert!(
                st.get(&Name::str(*name)).is_some(),
                "missing list lemma: {}",
                name
            );
        }
    }
    #[test]
    fn test_prop_lemma_priority() {
        let st = register_default_simp_lemmas();
        let entry = st
            .get(&Name::str("simp_true_and"))
            .expect("operation should succeed");
        assert_eq!(entry.priority, 1000);
    }
    #[test]
    fn test_nat_lemma_priority() {
        let st = register_default_simp_lemmas();
        let entry = st
            .get(&Name::str("simp_nat_zero_add"))
            .expect("operation should succeed");
        assert_eq!(entry.priority, 900);
    }
    #[test]
    fn test_bool_lemma_priority() {
        let st = register_default_simp_lemmas();
        let entry = st
            .get(&Name::str("simp_bool_not_not"))
            .expect("operation should succeed");
        assert_eq!(entry.priority, 900);
    }
    #[test]
    fn test_list_lemma_priority() {
        let st = register_default_simp_lemmas();
        let entry = st
            .get(&Name::str("simp_list_nil_append"))
            .expect("operation should succeed");
        assert_eq!(entry.priority, 800);
    }
    #[test]
    fn test_build_tactic_lemmas_env_succeeds() {
        let _ = full_env();
    }
    #[test]
    fn test_simp_infrastructure_present() {
        let env = full_env();
        let infra_names = [
            "SimpLemmaEntry",
            "SimpLemmaEntry.mk",
            "SimpTheorems",
            "SimpTheorems.empty",
            "SimpTheorems.add",
            "SimpTheorems.erase",
        ];
        for name in &infra_names {
            assert!(env.contains(&Name::str(*name)), "missing infra: {}", name);
        }
    }
    #[test]
    fn test_prop_simp_axioms_present() {
        let env = full_env();
        let prop_names = [
            "simp_true_and",
            "simp_and_true",
            "simp_false_and",
            "simp_and_false",
            "simp_true_or",
            "simp_or_true",
            "simp_false_or",
            "simp_or_false",
            "simp_not_true",
            "simp_not_false",
            "simp_not_not",
            "simp_iff_self",
            "simp_eq_self",
            "simp_true_implies",
            "simp_false_implies",
            "simp_and_self",
            "simp_or_self",
        ];
        for name in &prop_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing prop axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_nat_simp_axioms_present() {
        let env = full_env();
        let nat_names = [
            "simp_nat_zero_add",
            "simp_nat_add_zero",
            "simp_nat_zero_mul",
            "simp_nat_mul_zero",
            "simp_nat_one_mul",
            "simp_nat_mul_one",
            "simp_nat_succ_ne_zero",
            "simp_nat_sub_self",
        ];
        for name in &nat_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing nat axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_bool_simp_axioms_present() {
        let env = full_env();
        let bool_names = [
            "simp_bool_not_not",
            "simp_bool_and_true",
            "simp_bool_and_false",
            "simp_bool_or_true",
            "simp_bool_or_false",
        ];
        for name in &bool_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing bool axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_list_simp_axioms_present() {
        let env = full_env();
        let list_names = [
            "simp_list_nil_append",
            "simp_list_append_nil",
            "simp_list_length_nil",
            "simp_list_map_nil",
            "simp_list_reverse_nil",
        ];
        for name in &list_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing list axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_ext_axioms_present() {
        let env = full_env();
        let ext_names = ["ext_funext", "ext_propext", "ext_array", "ext_set"];
        for name in &ext_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing ext axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_norm_num_axioms_present() {
        let env = full_env();
        let nn_names = [
            "norm_num_zero_lt_succ",
            "norm_num_succ_ne_zero",
            "norm_num_add_comm",
            "norm_num_mul_comm",
        ];
        for name in &nn_names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing norm_num axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_simp_true_and_type_structure() {
        let env = full_env();
        let decl = env
            .get(&Name::str("simp_true_and"))
            .expect("declaration 'simp_true_and' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_simp_not_true_type_is_iff() {
        let env = full_env();
        let decl = env
            .get(&Name::str("simp_not_true"))
            .expect("declaration 'simp_not_true' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_app());
    }
    #[test]
    fn test_simp_nat_zero_add_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("simp_nat_zero_add"))
            .expect("declaration 'simp_nat_zero_add' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_simp_bool_not_not_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("simp_bool_not_not"))
            .expect("declaration 'simp_bool_not_not' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_ext_funext_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("ext_funext"))
            .expect("declaration 'ext_funext' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_ext_propext_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("ext_propext"))
            .expect("declaration 'ext_propext' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_norm_num_zero_lt_succ_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("norm_num_zero_lt_succ"))
            .expect("declaration 'norm_num_zero_lt_succ' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_norm_num_add_comm_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("norm_num_add_comm"))
            .expect("declaration 'norm_num_add_comm' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, _, cod) = ty {
            assert!(cod.is_pi());
        }
    }
    #[test]
    fn test_mk_iff_structure() {
        let e = mk_iff(cst("P"), cst("Q"));
        if let Expr::App(f, rhs) = &e {
            assert_eq!(**rhs, cst("Q"));
            if let Expr::App(g, lhs) = f.as_ref() {
                assert_eq!(**lhs, cst("P"));
                assert_eq!(**g, cst("Iff"));
            } else {
                panic!("expected inner App");
            }
        } else {
            panic!("expected outer App");
        }
    }
    #[test]
    fn test_mk_and_structure() {
        let e = mk_and(cst("A"), cst("B"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                assert_eq!(**g, cst("And"));
            } else {
                panic!("expected inner App");
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_or_structure() {
        let e = mk_or(cst("A"), cst("B"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                assert_eq!(**g, cst("Or"));
            } else {
                panic!("expected inner App");
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_not_structure() {
        let e = mk_not(cst("P"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("Not"));
            assert_eq!(**arg, cst("P"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_eq_structure() {
        let e = mk_eq(nat_ty(), cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_ne_structure() {
        let e = mk_ne(nat_ty(), cst("a"), cst("b"));
        if let Expr::App(f, _) = &e {
            assert_eq!(**f, cst("Not"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_nat_add_structure() {
        let e = nat_add(cst("x"), cst("y"));
        assert!(e.is_app());
    }
    #[test]
    fn test_nat_mul_structure() {
        let e = nat_mul(cst("x"), cst("y"));
        assert!(e.is_app());
    }
    #[test]
    fn test_nat_sub_structure() {
        let e = nat_sub(cst("x"), cst("y"));
        assert!(e.is_app());
    }
    #[test]
    fn test_nat_succ_structure() {
        let e = nat_succ(cst("n"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("Nat.succ"));
            assert_eq!(**arg, cst("n"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_list_nil_structure() {
        let e = list_nil(cst("α"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("List.nil"));
            assert_eq!(**arg, cst("α"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_bool_not_structure() {
        let e = bool_not(cst("b"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("Bool.not"));
            assert_eq!(**arg, cst("b"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_bool_and_structure() {
        let e = bool_and(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_total_declaration_count() {
        let env = full_env();
        let prereq_count = 27;
        assert!(
            env.len() >= prereq_count + 49,
            "expected at least {} declarations, got {}",
            prereq_count + 49,
            env.len()
        );
    }
    #[test]
    fn test_simp_eq_self_has_universe_param() {
        let env = full_env();
        let decl = env
            .get(&Name::str("simp_eq_self"))
            .expect("declaration 'simp_eq_self' should exist in env");
        assert!(!decl.univ_params().is_empty());
        assert_eq!(decl.univ_params()[0], Name::str("u"));
    }
    #[test]
    fn test_ext_funext_has_universe_params() {
        let env = full_env();
        let decl = env
            .get(&Name::str("ext_funext"))
            .expect("declaration 'ext_funext' should exist in env");
        assert_eq!(decl.univ_params().len(), 2);
    }
}
/// `Nat.add_zero : ∀ (n : ℕ), n + 0 = n`
#[allow(dead_code)]
pub fn tl2_ext_nat_add_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_add(bvar(0), nat_zero()), bvar(0)),
    )
}
/// `Nat.zero_add : ∀ (n : ℕ), 0 + n = n`
#[allow(dead_code)]
pub fn tl2_ext_nat_zero_add_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_add(nat_zero(), bvar(0)), bvar(0)),
    )
}
/// `Nat.add_comm : ∀ (m n : ℕ), m + n = n + m`
#[allow(dead_code)]
pub fn tl2_ext_nat_add_comm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(nat_add(bvar(1), bvar(0)), nat_add(bvar(0), bvar(1))),
        ),
    )
}
/// `Nat.add_assoc : ∀ (a b c : ℕ), (a + b) + c = a + (b + c)`
#[allow(dead_code)]
pub fn tl2_ext_nat_add_assoc_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                mk_nat_eq(
                    nat_add(nat_add(bvar(2), bvar(1)), bvar(0)),
                    nat_add(bvar(2), nat_add(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `Nat.mul_zero : ∀ (n : ℕ), n * 0 = 0`
#[allow(dead_code)]
pub fn tl2_ext_nat_mul_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(bvar(0), nat_zero()), nat_zero()),
    )
}
/// `Nat.zero_mul : ∀ (n : ℕ), 0 * n = 0`
#[allow(dead_code)]
pub fn tl2_ext_nat_zero_mul_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(nat_zero(), bvar(0)), nat_zero()),
    )
}
/// `Nat.mul_one : ∀ (n : ℕ), n * 1 = n`
#[allow(dead_code)]
pub fn tl2_ext_nat_mul_one_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(bvar(0), nat_lit_one()), bvar(0)),
    )
}
/// `Nat.one_mul : ∀ (n : ℕ), 1 * n = n`
#[allow(dead_code)]
pub fn tl2_ext_nat_one_mul_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_mul(nat_lit_one(), bvar(0)), bvar(0)),
    )
}
/// `Nat.mul_comm : ∀ (m n : ℕ), m * n = n * m`
#[allow(dead_code)]
pub fn tl2_ext_nat_mul_comm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(nat_mul(bvar(1), bvar(0)), nat_mul(bvar(0), bvar(1))),
        ),
    )
}
/// `Nat.mul_assoc : ∀ (a b c : ℕ), (a * b) * c = a * (b * c)`
#[allow(dead_code)]
pub fn tl2_ext_nat_mul_assoc_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                mk_nat_eq(
                    nat_mul(nat_mul(bvar(2), bvar(1)), bvar(0)),
                    nat_mul(bvar(2), nat_mul(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `Nat.sub_self : ∀ (n : ℕ), n - n = 0`
#[allow(dead_code)]
pub fn tl2_ext_nat_sub_self_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_sub(bvar(0), bvar(0)), nat_zero()),
    )
}
/// `Nat.sub_zero : ∀ (n : ℕ), n - 0 = n`
#[allow(dead_code)]
pub fn tl2_ext_nat_sub_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(nat_sub(bvar(0), nat_zero()), bvar(0)),
    )
}
/// `Nat.succ_ne_zero : ∀ (n : ℕ), Nat.succ n ≠ 0`
#[allow(dead_code)]
pub fn tl2_ext_nat_succ_ne_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_not(mk_nat_eq(nat_succ(bvar(0)), nat_zero())),
    )
}
/// `congr_arg : ∀ {α β : Sort u} (f : α → β) {a₁ a₂ : α}, a₁ = a₂ → f a₁ = f a₂`
#[allow(dead_code)]
pub fn tl2_ext_congr_arg_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "a1",
                    bvar(2),
                    pi(
                        BinderInfo::Implicit,
                        "a2",
                        bvar(3),
                        arrow(
                            mk_eq(bvar(4), bvar(1), bvar(0)),
                            mk_eq(bvar(4), app(bvar(2), bvar(1)), app(bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `congr_fun : ∀ {α β : Sort u} {f g : α → β}, f = g → ∀ (x : α), f x = g x`
#[allow(dead_code)]
pub fn tl2_ext_congr_fun_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "β",
            sort_u(),
            pi(
                BinderInfo::Implicit,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Implicit,
                    "g",
                    arrow(bvar(2), bvar(2)),
                    arrow(
                        mk_eq(arrow(bvar(3), bvar(3)), bvar(1), bvar(0)),
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(4),
                            mk_eq(bvar(5), app(bvar(3), bvar(0)), app(bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `eq_congr : ∀ {α : Sort u} {a b c d : α}, a = b → c = d → (a = c ↔ b = d)`
#[allow(dead_code)]
pub fn tl2_ext_eq_congr_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        sort_u(),
        pi(
            BinderInfo::Implicit,
            "a",
            bvar(0),
            pi(
                BinderInfo::Implicit,
                "b",
                bvar(1),
                pi(
                    BinderInfo::Implicit,
                    "c",
                    bvar(2),
                    pi(
                        BinderInfo::Implicit,
                        "d",
                        bvar(3),
                        arrow(
                            mk_eq(bvar(4), bvar(3), bvar(2)),
                            arrow(
                                mk_eq(bvar(5), bvar(3), bvar(2)),
                                mk_iff(
                                    mk_eq(bvar(6), bvar(5), bvar(3)),
                                    mk_eq(bvar(7), bvar(4), bvar(2)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Int` type constant
#[allow(dead_code)]
pub(super) fn int_ty() -> Expr {
    cst("Int")
}
/// `Int.add : Int → Int → Int`
#[allow(dead_code)]
pub(super) fn int_add(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.add"), a, b)
}
/// `Int.mul : Int → Int → Int`
#[allow(dead_code)]
pub(super) fn int_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.mul"), a, b)
}
/// `Int.neg : Int → Int`
#[allow(dead_code)]
pub(super) fn int_neg(n: Expr) -> Expr {
    app(cst("Int.neg"), n)
}
/// Helper: build `Eq Int a b`
#[allow(dead_code)]
pub(super) fn mk_int_eq(a: Expr, b: Expr) -> Expr {
    mk_eq(int_ty(), a, b)
}
/// Helper: `Int.le a b`
#[allow(dead_code)]
pub(super) fn int_le(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.le"), a, b)
}
/// Helper: `Int.lt a b`
#[allow(dead_code)]
pub fn int_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.lt"), a, b)
}
/// `Int.zero` constant
#[allow(dead_code)]
pub(super) fn int_zero() -> Expr {
    cst("Int.zero")
}
/// `omega_int_add_zero : ∀ (n : Int), n + 0 = n`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_add_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        int_ty(),
        mk_int_eq(int_add(bvar(0), int_zero()), bvar(0)),
    )
}
/// `omega_int_zero_add : ∀ (n : Int), 0 + n = n`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_zero_add_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        int_ty(),
        mk_int_eq(int_add(int_zero(), bvar(0)), bvar(0)),
    )
}
/// `omega_int_add_comm : ∀ (m n : Int), m + n = n + m`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_add_comm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        int_ty(),
        pi(
            BinderInfo::Default,
            "n",
            int_ty(),
            mk_int_eq(int_add(bvar(1), bvar(0)), int_add(bvar(0), bvar(1))),
        ),
    )
}
/// `omega_int_neg_add_cancel : ∀ (n : Int), -n + n = 0`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_neg_add_cancel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        int_ty(),
        mk_int_eq(int_add(int_neg(bvar(0)), bvar(0)), int_zero()),
    )
}
/// `omega_int_le_refl : ∀ (n : Int), n ≤ n`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_le_refl_ty() -> Expr {
    pi(BinderInfo::Default, "n", int_ty(), int_le(bvar(0), bvar(0)))
}
/// `omega_int_le_antisymm : ∀ (m n : Int), m ≤ n → n ≤ m → m = n`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_le_antisymm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        int_ty(),
        pi(
            BinderInfo::Default,
            "n",
            int_ty(),
            arrow(
                int_le(bvar(1), bvar(0)),
                arrow(int_le(bvar(0), bvar(1)), mk_int_eq(bvar(2), bvar(1))),
            ),
        ),
    )
}
/// `omega_int_lt_iff_add_one_le : ∀ (m n : Int), m < n ↔ m + 1 ≤ n`
#[allow(dead_code)]
pub fn tl2_ext_omega_int_lt_iff_add_one_le_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        int_ty(),
        pi(
            BinderInfo::Default,
            "n",
            int_ty(),
            mk_iff(
                int_lt(bvar(1), bvar(0)),
                int_le(int_add(bvar(1), cst("Int.one")), bvar(0)),
            ),
        ),
    )
}
/// `norm_num_add_eval : ∀ (a b c : Nat), a + b = c → Nat.add a b = c`
#[allow(dead_code)]
pub fn tl2_ext_norm_num_add_eval_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    mk_nat_eq(nat_add(bvar(2), bvar(1)), bvar(0)),
                    mk_nat_eq(nat_add(bvar(2), bvar(1)), bvar(0)),
                ),
            ),
        ),
    )
}
/// `norm_num_mul_eval : ∀ (a b c : Nat), a * b = c → Nat.mul a b = c`
#[allow(dead_code)]
pub fn tl2_ext_norm_num_mul_eval_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    mk_nat_eq(nat_mul(bvar(2), bvar(1)), bvar(0)),
                    mk_nat_eq(nat_mul(bvar(2), bvar(1)), bvar(0)),
                ),
            ),
        ),
    )
}
/// `norm_num_pow_zero : ∀ (n : Nat), n ^ 0 = 1`
#[allow(dead_code)]
pub fn tl2_ext_norm_num_pow_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        mk_nat_eq(app2(cst("Nat.pow"), bvar(0), nat_zero()), nat_lit_one()),
    )
}
/// `norm_num_pow_succ : ∀ (n k : Nat), n ^ (k+1) = n ^ k * n`
#[allow(dead_code)]
pub fn tl2_ext_norm_num_pow_succ_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            mk_nat_eq(
                app2(cst("Nat.pow"), bvar(1), nat_succ(bvar(0))),
                nat_mul(app2(cst("Nat.pow"), bvar(1), bvar(0)), bvar(1)),
            ),
        ),
    )
}
/// `ring_add_left_comm : ∀ (a b c : α), a + (b + c) = b + (a + c)` (for a semiring α)
#[allow(dead_code)]
pub fn tl2_ext_ring_add_left_comm_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "add",
            arrow(bvar(0), arrow(bvar(1), bvar(2))),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "c",
                        bvar(3),
                        mk_eq(
                            bvar(4),
                            app2(bvar(3), bvar(2), app2(bvar(3), bvar(1), bvar(0))),
                            app2(bvar(3), bvar(1), app2(bvar(3), bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
