//! Environment builder functions

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name, ReducibilityHint};

use super::super::types::EnvBuilder;

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build an n-ary application from a function and argument list.
pub fn app_n(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}
/// Build a `Const` expression (constant with no universe levels).
pub fn var(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Build a bound variable expression at de Bruijn index `i`.
pub fn bvar(i: u32) -> Expr {
    Expr::BVar(i)
}
/// Build a `Sort(Level::zero())` (Prop).
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Build a `Sort(Level::succ^n(Level::zero()))` (Type n-1).
pub fn sort(n: u32) -> Expr {
    let mut l = Level::zero();
    for _ in 0..n {
        l = Level::succ(l);
    }
    Expr::Sort(l)
}
/// Alias for `sort(1)` — `Type 0`.
pub fn type0() -> Expr {
    sort(1)
}
/// Alias for `sort(2)` — `Type 1`.
pub fn type1() -> Expr {
    sort(2)
}
/// Build a non-dependent arrow type `dom → cod`.
pub fn pi(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(cod),
    )
}
/// Build a named non-dependent pi-type `(name : dom) → cod`.
pub fn pi_named(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}
/// Build an implicit pi `{name : dom} → cod`.
pub fn pi_implicit(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}
/// Build an instance-implicit pi `[name : dom] → cod`.
pub fn pi_inst(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::InstImplicit,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}
/// Build a `Lam` expression with an anonymous binder and `Prop` domain.
pub fn lam(body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(prop()),
        Box::new(body),
    )
}
/// Build a named lambda `fun (name : ty) => body`.
pub fn lam_named(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Build an n-ary arrow type `t1 → t2 → … → ret`.
pub fn arrows(mut tys: Vec<Expr>, ret: Expr) -> Expr {
    tys.reverse();
    tys.into_iter().fold(ret, |acc, t| pi(t, acc))
}
/// Add `Bool` with its constructors `Bool.true` and `Bool.false`.
pub fn add_bool(b: &mut EnvBuilder) {
    b.axiom("Bool", type0());
    b.axiom("Bool.true", var("Bool"));
    b.axiom("Bool.false", var("Bool"));
}
/// Add `Unit` with its constructor `Unit.unit`.
pub fn add_unit(b: &mut EnvBuilder) {
    b.axiom("Unit", type0());
    b.axiom("Unit.unit", var("Unit"));
}
/// Add `Nat` with `Nat.zero` and `Nat.succ`.
pub fn add_nat(b: &mut EnvBuilder) {
    b.axiom("Nat", type0());
    b.axiom("Nat.zero", var("Nat"));
    b.axiom("Nat.succ", pi(var("Nat"), var("Nat")));
}
/// Add `Int` with `Int.ofNat` and `Int.negSucc`.
pub fn add_int(b: &mut EnvBuilder) {
    if !b.contains("Nat") {
        add_nat(b);
    }
    b.axiom("Int", type0());
    b.axiom("Int.ofNat", pi(var("Nat"), var("Int")));
    b.axiom("Int.negSucc", pi(var("Nat"), var("Int")));
}
/// Add `Prod` (product type) with `Prod.mk`.
pub fn add_prod(b: &mut EnvBuilder) {
    b.axiom("Prod", pi(type0(), pi(type0(), type0())));
    let prod_mk_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            type0(),
            pi(
                bvar(1),
                pi(bvar(1), app(app(var("Prod"), bvar(3)), bvar(2))),
            ),
        ),
    );
    b.axiom("Prod.mk", prod_mk_ty);
}
/// Add `Option` with `Option.none` and `Option.some`.
pub fn add_option(b: &mut EnvBuilder) {
    b.axiom("Option", pi(type0(), type0()));
    let none_ty = pi_implicit("α", type0(), app(var("Option"), bvar(0)));
    b.axiom("Option.none", none_ty);
    let some_ty = pi_implicit("α", type0(), pi(bvar(0), app(var("Option"), bvar(1))));
    b.axiom("Option.some", some_ty);
    // Option.map : {α β : Type} → (α → β) → Option α → Option β
    let map_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            type0(),
            pi(
                pi(bvar(1), bvar(1)),
                pi(app(var("Option"), bvar(2)), app(var("Option"), bvar(2))),
            ),
        ),
    );
    b.axiom("Option.map", map_ty);
    // Option.bind : {α β : Type} → Option α → (α → Option β) → Option β
    let bind_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            type0(),
            pi(
                app(var("Option"), bvar(1)),
                pi(
                    pi(bvar(2), app(var("Option"), bvar(2))),
                    app(var("Option"), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom("Option.bind", bind_ty);
    // Option.isSome : {α : Type} → Option α → Bool
    let is_some_ty = pi_implicit("α", type0(), pi(app(var("Option"), bvar(0)), var("Bool")));
    b.axiom("Option.isSome", is_some_ty);
}
/// Add `List` with `List.nil` and `List.cons`.
pub fn add_list(b: &mut EnvBuilder) {
    b.axiom("List", pi(type0(), type0()));
    let nil_ty = pi_implicit("α", type0(), app(var("List"), bvar(0)));
    b.axiom("List.nil", nil_ty);
    let cons_ty = pi_implicit(
        "α",
        type0(),
        pi(
            bvar(0),
            pi(app(var("List"), bvar(1)), app(var("List"), bvar(2))),
        ),
    );
    b.axiom("List.cons", cons_ty);
}
/// Build a minimal standard prelude environment.
pub fn minimal_prelude() -> Result<Environment, String> {
    let mut b = EnvBuilder::fresh();
    add_bool(&mut b);
    add_unit(&mut b);
    add_empty(&mut b);
    add_nat(&mut b);
    add_prod(&mut b);
    add_option(&mut b);
    add_list(&mut b);
    b.finish()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_builder_axiom() {
        let mut b = EnvBuilder::fresh();
        b.axiom("Foo", type0());
        assert!(b.contains("Foo"));
        assert!(b.is_ok());
    }
    #[test]
    fn test_builder_def() {
        let mut b = EnvBuilder::fresh();
        b.axiom("Bool", type0());
        b.def(
            "myId",
            pi(var("Bool"), var("Bool")),
            lam_named("x", var("Bool"), bvar(0)),
        );
        assert!(b.is_ok());
    }
    #[test]
    fn test_builder_finish_ok() {
        let mut b = EnvBuilder::fresh();
        b.axiom("Nat", type0());
        let env = b.finish().expect("finish should succeed");
        assert!(env.get(&Name::str("Nat")).is_some());
    }
    #[test]
    fn test_add_bool() {
        let mut b = EnvBuilder::fresh();
        add_bool(&mut b);
        assert!(b.contains("Bool"));
        assert!(b.contains("Bool.true"));
        assert!(b.contains("Bool.false"));
    }
    #[test]
    fn test_add_nat() {
        let mut b = EnvBuilder::fresh();
        add_nat(&mut b);
        assert!(b.contains("Nat"));
        assert!(b.contains("Nat.zero"));
        assert!(b.contains("Nat.succ"));
    }
    #[test]
    fn test_add_list() {
        let mut b = EnvBuilder::fresh();
        add_list(&mut b);
        assert!(b.contains("List"));
        assert!(b.contains("List.nil"));
        assert!(b.contains("List.cons"));
    }
    #[test]
    fn test_add_option() {
        let mut b = EnvBuilder::fresh();
        add_option(&mut b);
        assert!(b.contains("Option"));
        assert!(b.contains("Option.none"));
        assert!(b.contains("Option.some"));
    }
    #[test]
    fn test_minimal_prelude() {
        let env = minimal_prelude().expect("prelude should build");
        for name in &[
            "Bool", "Nat", "Int", "Unit", "Empty", "List", "Option", "Prod",
        ] {
            let _ = env.get(&Name::str(*name));
        }
        assert!(env.get(&Name::str("Bool")).is_some());
        assert!(env.get(&Name::str("Nat")).is_some());
    }
    #[test]
    fn test_app_n() {
        let f = var("f");
        let result = app_n(f, vec![bvar(0), bvar(1)]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_arrows() {
        let ty = arrows(vec![type0(), type0()], type0());
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_sort_levels() {
        let p = prop();
        let t = type0();
        let t1 = type1();
        assert!(matches!(p, Expr::Sort(_)));
        assert!(matches!(t, Expr::Sort(_)));
        assert!(matches!(t1, Expr::Sort(_)));
    }
    #[test]
    fn test_builder_errors() {
        let mut b = EnvBuilder::fresh();
        b.axiom("X", type0());
        b.axiom("X", type0());
        assert!(!b.is_ok());
        assert!(!b.errors().is_empty());
    }
    #[test]
    fn test_add_int() {
        let mut b = EnvBuilder::fresh();
        add_nat(&mut b);
        add_int(&mut b);
        assert!(b.contains("Int"));
        assert!(b.contains("Int.ofNat"));
        assert!(b.contains("Int.negSucc"));
    }
}
/// Add equality `Eq : {α : Type} → α → α → Prop`.
#[allow(dead_code)]
pub fn add_eq(b: &mut EnvBuilder) {
    let eq_ty = pi_implicit("α", type0(), pi(bvar(0), pi(bvar(1), prop())));
    b.axiom("Eq", eq_ty);
    let refl_ty = pi_implicit(
        "α",
        type0(),
        pi(bvar(0), app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0))),
    );
    b.axiom("Eq.refl", refl_ty);
}
/// Add `And` (logical conjunction).
#[allow(dead_code)]
pub fn add_and(b: &mut EnvBuilder) {
    b.axiom("And", pi(prop(), pi(prop(), prop())));
    let intro_ty = pi(bvar(0), pi(bvar(1), app(app(var("And"), bvar(2)), bvar(1))));
    b.axiom("And.intro", intro_ty);
}
/// Add `Or` (logical disjunction).
#[allow(dead_code)]
pub fn add_or(b: &mut EnvBuilder) {
    b.axiom("Or", pi(prop(), pi(prop(), prop())));
    let inl_ty = pi(bvar(0), app(app(var("Or"), bvar(1)), bvar(0)));
    b.axiom("Or.inl", inl_ty);
    let inr_ty = pi(bvar(0), app(app(var("Or"), bvar(0)), bvar(1)));
    b.axiom("Or.inr", inr_ty);
}
/// Add `Not` (logical negation).
#[allow(dead_code)]
pub fn add_not(b: &mut EnvBuilder) {
    b.axiom("Not", pi(prop(), prop()));
}
/// Add `False` (empty type in Prop).
#[allow(dead_code)]
pub fn add_false(b: &mut EnvBuilder) {
    b.axiom("False", prop());
    let elim_ty = pi_implicit("P", prop(), pi(var("False"), bvar(1)));
    b.axiom("False.elim", elim_ty);
}
/// Add `True` (unit type in Prop).
#[allow(dead_code)]
pub fn add_true(b: &mut EnvBuilder) {
    b.axiom("True", prop());
    b.axiom("True.intro", var("True"));
}
/// Add `Iff` (logical biconditional).
#[allow(dead_code)]
pub fn add_iff(b: &mut EnvBuilder) {
    b.axiom("Iff", pi(prop(), pi(prop(), prop())));
    let intro_ty = pi(
        pi(bvar(0), bvar(1)),
        pi(pi(bvar(0), bvar(2)), app(app(var("Iff"), bvar(3)), bvar(2))),
    );
    b.axiom("Iff.intro", intro_ty);
}
/// Add `Exists` (existential quantifier).
#[allow(dead_code)]
pub fn add_exists(b: &mut EnvBuilder) {
    let exists_ty = pi_implicit("α", type0(), pi(pi(bvar(0), prop()), prop()));
    b.axiom("Exists", exists_ty);
    let intro_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "P",
            pi(bvar(0), prop()),
            pi(
                bvar(1),
                pi(app(bvar(1), bvar(0)), app(var("Exists"), bvar(2))),
            ),
        ),
    );
    b.axiom("Exists.intro", intro_ty);
}
/// Add `String` type.
#[allow(dead_code)]
pub fn add_string(b: &mut EnvBuilder) {
    b.axiom("String", type0());
}
/// Add `Char` type.
#[allow(dead_code)]
pub fn add_char(b: &mut EnvBuilder) {
    b.axiom("Char", type0());
}
/// Build a full logic prelude (Bool, Nat, Eq, And, Or, Not, True, False, Iff, Exists).
#[allow(dead_code)]
pub fn logic_prelude() -> Result<Environment, String> {
    let mut b = EnvBuilder::fresh();
    add_bool(&mut b);
    add_nat(&mut b);
    add_true(&mut b);
    add_false(&mut b);
    add_not(&mut b);
    add_and(&mut b);
    add_or(&mut b);
    add_iff(&mut b);
    add_eq(&mut b);
    add_exists(&mut b);
    b.finish()
}
/// Build the type of an identity function `{α : Type} → α → α`.
#[allow(dead_code)]
pub fn id_type() -> Expr {
    pi_implicit("α", type0(), pi(bvar(0), bvar(1)))
}
/// Build the type of a composition function
/// `{α β γ : Type} → (β → γ) → (α → β) → α → γ`.
#[allow(dead_code)]
pub fn compose_type() -> Expr {
    pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            type0(),
            pi_implicit(
                "γ",
                type0(),
                pi(
                    pi(bvar(1), bvar(2)),
                    pi(pi(bvar(2), bvar(2)), pi(bvar(3), bvar(2))),
                ),
            ),
        ),
    )
}
#[cfg(test)]
mod env_builder_extra_tests {
    use super::*;
    #[test]
    fn test_add_eq() {
        let mut b = EnvBuilder::fresh();
        add_eq(&mut b);
        assert!(b.contains("Eq"));
        assert!(b.contains("Eq.refl"));
    }
    #[test]
    fn test_add_and_or() {
        let mut b = EnvBuilder::fresh();
        add_and(&mut b);
        add_or(&mut b);
        assert!(b.contains("And"));
        assert!(b.contains("Or"));
        assert!(b.contains("Or.inl"));
        assert!(b.contains("Or.inr"));
    }
    #[test]
    fn test_add_not_false_true() {
        let mut b = EnvBuilder::fresh();
        add_not(&mut b);
        add_false(&mut b);
        add_true(&mut b);
        assert!(b.contains("Not"));
        assert!(b.contains("False"));
        assert!(b.contains("True"));
        assert!(b.contains("True.intro"));
        assert!(b.contains("False.elim"));
    }
    #[test]
    fn test_add_iff() {
        let mut b = EnvBuilder::fresh();
        add_iff(&mut b);
        assert!(b.contains("Iff"));
        assert!(b.contains("Iff.intro"));
    }
    #[test]
    fn test_add_exists() {
        let mut b = EnvBuilder::fresh();
        add_exists(&mut b);
        assert!(b.contains("Exists"));
        assert!(b.contains("Exists.intro"));
    }
    #[test]
    fn test_logic_prelude() {
        let env = logic_prelude().expect("logic prelude should build");
        for name in &["Bool", "Nat", "And", "Or", "Not", "Iff", "Eq", "Exists"] {
            assert!(env.get(&Name::str(*name)).is_some(), "missing {}", name);
        }
    }
    #[test]
    fn test_id_type() {
        let ty = id_type();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_builder_theorem() {
        let mut b = EnvBuilder::fresh();
        add_true(&mut b);
        b.theorem("my_thm", var("True"), var("True.intro"));
        assert!(b.is_ok());
    }
    #[test]
    fn test_add_string_char() {
        let mut b = EnvBuilder::fresh();
        add_string(&mut b);
        add_char(&mut b);
        assert!(b.contains("String"));
        assert!(b.contains("Char"));
    }
    #[test]
    fn test_pi_variants() {
        let p = pi_named("x", type0(), type0());
        assert!(matches!(p, Expr::Pi(BinderInfo::Default, _, _, _)));
        let imp = pi_implicit("y", type0(), type0());
        assert!(matches!(imp, Expr::Pi(BinderInfo::Implicit, _, _, _)));
        let inst = pi_inst("z", type0(), type0());
        assert!(matches!(inst, Expr::Pi(BinderInfo::InstImplicit, _, _, _)));
    }
}
/// Add `WellFounded` typeclass stub.
#[allow(dead_code)]
pub fn add_well_founded(b: &mut EnvBuilder) {
    let wf_ty = pi_implicit("α", type0(), pi(pi(bvar(0), pi(bvar(1), prop())), prop()));
    b.axiom("WellFounded", wf_ty);
    // WellFounded.rec : {α : Type} → {r : α → α → Prop} → {C : α → Type}
    //   → ((x : α) → ((y : α) → r y x → C y) → C x) → (w : WellFounded r) → (a : α) → C a
    // Simplified: {α : Type} → {r : α → α → Prop} → WellFounded r → α → Type
    let rec_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "r",
            pi(bvar(0), pi(bvar(1), prop())),
            pi(prop(), pi(bvar(2), type0())),
        ),
    );
    b.axiom("WellFounded.rec", rec_ty);
    // Acc : {α : Type} → (α → α → Prop) → α → Prop
    let acc_ty = pi_implicit(
        "α",
        type0(),
        pi(pi(bvar(0), pi(bvar(1), prop())), pi(bvar(1), prop())),
    );
    b.axiom("Acc", acc_ty);
    // Acc.intro : {α : Type} → {r : α → α → Prop} → (x : α) → ((y : α) → r y x → Acc r y) → Acc r x
    // Simplified: {α : Type} → {r : α → α → Prop} → α → Prop
    let acc_intro_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit("r", pi(bvar(0), pi(bvar(1), prop())), pi(bvar(1), prop())),
    );
    b.axiom("Acc.intro", acc_intro_ty);
}
/// Add `Decidable` typeclass stub.
#[allow(dead_code)]
pub fn add_decidable(b: &mut EnvBuilder) {
    b.axiom("Decidable", pi(prop(), type0()));
    let is_true_ty = pi_implicit("P", prop(), pi(bvar(0), app(var("Decidable"), bvar(1))));
    b.axiom("Decidable.isTrue", is_true_ty);
    let is_false_ty = pi_implicit(
        "P",
        prop(),
        pi(pi(bvar(0), var("False")), app(var("Decidable"), bvar(1))),
    );
    b.axiom("Decidable.isFalse", is_false_ty);
    // decide : {P : Prop} → [Decidable P] → Decidable P
    let decide_ty = pi_implicit("P", prop(), app(var("Decidable"), bvar(0)));
    b.axiom("decide", decide_ty);
    // instDecidableAnd : {P Q : Prop} → [Decidable P] → [Decidable Q] → Decidable (P ∧ Q)
    let and_ty = pi_implicit(
        "P",
        prop(),
        pi_implicit(
            "Q",
            prop(),
            pi(
                app(var("Decidable"), bvar(1)),
                pi(
                    app(var("Decidable"), bvar(1)),
                    app(var("Decidable"), app(app(var("And"), bvar(3)), bvar(2))),
                ),
            ),
        ),
    );
    b.axiom("instDecidableAnd", and_ty);
}
/// Add `Subtype` type.
#[allow(dead_code)]
pub fn add_subtype(b: &mut EnvBuilder) {
    let subtype_ty = pi_implicit("α", type0(), pi(pi(bvar(0), prop()), type0()));
    b.axiom("Subtype", subtype_ty);
    let mk_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "P",
            pi(bvar(0), prop()),
            pi(
                bvar(1),
                pi(app(bvar(1), bvar(0)), app(var("Subtype"), bvar(2))),
            ),
        ),
    );
    b.axiom("Subtype.mk", mk_ty);
    // Subtype.val : {α : Type} → {P : α → Prop} → Subtype P → α
    let val_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "P",
            pi(bvar(0), prop()),
            pi(app(var("Subtype"), bvar(0)), bvar(2)),
        ),
    );
    b.axiom("Subtype.val", val_ty);
    // Subtype.property : {α : Type} → {P : α → Prop} → (s : Subtype P) → P (Subtype.val s)
    // Simplified: Subtype P → Prop
    let prop_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "P",
            pi(bvar(0), prop()),
            pi(app(var("Subtype"), bvar(0)), prop()),
        ),
    );
    b.axiom("Subtype.property", prop_ty);
}
#[cfg(test)]
mod env_builder_final_tests {
    use super::*;
    #[test]
    fn test_add_well_founded() {
        let mut b = EnvBuilder::fresh();
        add_well_founded(&mut b);
        assert!(b.contains("WellFounded"));
    }
    #[test]
    fn test_add_decidable() {
        let mut b = EnvBuilder::fresh();
        add_false(&mut b);
        add_decidable(&mut b);
        assert!(b.contains("Decidable"));
        assert!(b.contains("Decidable.isTrue"));
        assert!(b.contains("Decidable.isFalse"));
    }
    #[test]
    fn test_add_subtype() {
        let mut b = EnvBuilder::fresh();
        add_subtype(&mut b);
        assert!(b.contains("Subtype"));
        assert!(b.contains("Subtype.mk"));
    }
}
/// Add `Functor` typeclass stub.
#[allow(dead_code)]
pub fn add_functor(b: &mut EnvBuilder) {
    b.axiom("Functor", pi(pi(type0(), type0()), type1()));
    let map_ty = pi_implicit(
        "F",
        pi(type0(), type0()),
        pi_inst(
            "_",
            app(var("Functor"), bvar(0)),
            pi_implicit(
                "α",
                type0(),
                pi_implicit(
                    "β",
                    type0(),
                    pi(
                        pi(bvar(1), bvar(1)),
                        pi(app(bvar(4), bvar(2)), app(bvar(5), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom("Functor.map", map_ty);
}
/// Add `Monad` typeclass stub.
#[allow(dead_code)]
pub fn add_monad(b: &mut EnvBuilder) {
    b.axiom("Monad", pi(pi(type0(), type0()), type1()));
    let pure_ty = pi_implicit(
        "M",
        pi(type0(), type0()),
        pi_inst(
            "_",
            app(var("Monad"), bvar(0)),
            pi_implicit("α", type0(), pi(bvar(0), app(bvar(3), bvar(1)))),
        ),
    );
    b.axiom("Monad.pure", pure_ty);
}
#[cfg(test)]
mod env_builder_monad_tests {
    use super::*;
    #[test]
    fn test_add_functor() {
        let mut b = EnvBuilder::fresh();
        add_functor(&mut b);
        assert!(b.contains("Functor"));
        assert!(b.contains("Functor.map"));
    }
    #[test]
    fn test_add_monad() {
        let mut b = EnvBuilder::fresh();
        add_monad(&mut b);
        assert!(b.contains("Monad"));
        assert!(b.contains("Monad.pure"));
    }
}
/// Add `Sigma` (dependent pair) type.
#[allow(dead_code)]
pub fn add_sigma(b: &mut EnvBuilder) {
    let sigma_ty = pi_implicit("α", type0(), pi(pi(bvar(0), type0()), type0()));
    b.axiom("Sigma", sigma_ty);
    let mk_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            pi(bvar(0), type0()),
            pi(
                bvar(1),
                pi(app(bvar(1), bvar(0)), app(var("Sigma"), bvar(2))),
            ),
        ),
    );
    b.axiom("Sigma.mk", mk_ty);
    let fst_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            pi(bvar(0), type0()),
            pi(app(var("Sigma"), bvar(0)), bvar(2)),
        ),
    );
    b.axiom("Sigma.fst", fst_ty);
    // Sigma.snd : {α : Type} → {β : α → Type} → (s : Sigma β) → β (Sigma.fst s)
    // Simplified type: Sigma β → Type (approximate)
    let snd_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            pi(bvar(0), type0()),
            pi(app(var("Sigma"), bvar(0)), type0()),
        ),
    );
    b.axiom("Sigma.snd", snd_ty);
}
/// Add `Empty` (the uninhabited type).
#[allow(dead_code)]
pub fn add_empty(b: &mut EnvBuilder) {
    b.axiom("Empty", type0());
    let elim_ty = pi_implicit("α", type0(), pi(var("Empty"), bvar(1)));
    b.axiom("Empty.elim", elim_ty);
}
/// Add `Unit` type with constructor.
#[allow(dead_code)]
pub fn add_unit_full(b: &mut EnvBuilder) {
    b.axiom("Unit", type0());
    b.axiom("Unit.star", var("Unit"));
    let rec_ty = pi_implicit("α", type0(), pi(bvar(0), pi(var("Unit"), bvar(2))));
    b.axiom("Unit.rec", rec_ty);
}
/// Add `Fin n` (finite type with n elements).
#[allow(dead_code)]
pub fn add_fin(b: &mut EnvBuilder) {
    b.axiom("Fin", pi(var("Nat"), type0()));
    let mk_ty = pi(var("Nat"), pi(var("Nat"), app(var("Fin"), bvar(1))));
    b.axiom("Fin.mk", mk_ty);
    let val_ty = pi_implicit("n", var("Nat"), pi(app(var("Fin"), bvar(0)), var("Nat")));
    b.axiom("Fin.val", val_ty);
    // Fin.zero : {n : Nat} → Fin (n + 1)
    let zero_ty = pi_implicit(
        "n",
        var("Nat"),
        app(
            var("Fin"),
            app(
                app(var("Nat.add"), bvar(0)),
                app(var("Nat.succ"), var("Nat.zero")),
            ),
        ),
    );
    b.axiom("Fin.zero", zero_ty);
}
/// Add `Array α` type (dynamic array).
#[allow(dead_code)]
pub fn add_array(b: &mut EnvBuilder) {
    b.axiom("Array", pi(type0(), type0()));
    let empty_ty = pi_implicit("α", type0(), app(var("Array"), bvar(0)));
    b.axiom("Array.empty", empty_ty);
    let push_ty = pi_implicit(
        "α",
        type0(),
        pi(
            app(var("Array"), bvar(0)),
            pi(bvar(1), app(var("Array"), bvar(2))),
        ),
    );
    b.axiom("Array.push", push_ty);
    let size_ty = pi_implicit("α", type0(), pi(app(var("Array"), bvar(0)), var("Nat")));
    b.axiom("Array.size", size_ty);
}
/// Add `HashMap K V` type.
#[allow(dead_code)]
pub fn add_hashmap(b: &mut EnvBuilder) {
    b.axiom("HashMap", pi(type0(), pi(type0(), type0())));
    let empty_ty = pi_implicit(
        "K",
        type0(),
        pi_implicit("V", type0(), app(app(var("HashMap"), bvar(1)), bvar(0))),
    );
    b.axiom("HashMap.empty", empty_ty);
    let insert_ty = pi_implicit(
        "K",
        type0(),
        pi_implicit(
            "V",
            type0(),
            pi(
                app(app(var("HashMap"), bvar(1)), bvar(0)),
                pi(
                    bvar(2),
                    pi(bvar(2), app(app(var("HashMap"), bvar(4)), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom("HashMap.insert", insert_ty);
}
/// Add `IO α` effect type.
#[allow(dead_code)]
pub fn add_io(b: &mut EnvBuilder) {
    b.axiom("IO", pi(type0(), type0()));
    let pure_ty = pi_implicit("α", type0(), pi(bvar(0), app(var("IO"), bvar(1))));
    b.axiom("IO.pure", pure_ty);
    let bind_ty = pi_implicit(
        "α",
        type0(),
        pi_implicit(
            "β",
            type0(),
            pi(
                app(var("IO"), bvar(1)),
                pi(
                    pi(bvar(2), app(var("IO"), bvar(2))),
                    app(var("IO"), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom("IO.bind", bind_ty);
}
#[cfg(test)]
mod env_builder_extended_tests {
    use super::*;
    #[test]
    fn test_add_sigma() {
        let mut b = EnvBuilder::fresh();
        add_sigma(&mut b);
        assert!(b.contains("Sigma"));
        assert!(b.contains("Sigma.mk"));
        assert!(b.contains("Sigma.fst"));
    }
    #[test]
    fn test_add_empty() {
        let mut b = EnvBuilder::fresh();
        add_empty(&mut b);
        assert!(b.contains("Empty"));
        assert!(b.contains("Empty.elim"));
    }
    #[test]
    fn test_add_unit_full() {
        let mut b = EnvBuilder::fresh();
        add_unit_full(&mut b);
        assert!(b.contains("Unit"));
        assert!(b.contains("Unit.star"));
        assert!(b.contains("Unit.rec"));
    }
    #[test]
    fn test_add_fin() {
        let mut b = EnvBuilder::fresh();
        b.axiom("Nat", type0());
        add_fin(&mut b);
        assert!(b.contains("Fin"));
        assert!(b.contains("Fin.mk"));
        assert!(b.contains("Fin.val"));
    }
    #[test]
    fn test_add_array() {
        let mut b = EnvBuilder::fresh();
        b.axiom("Nat", type0());
        add_array(&mut b);
        assert!(b.contains("Array"));
        assert!(b.contains("Array.empty"));
        assert!(b.contains("Array.push"));
        assert!(b.contains("Array.size"));
    }
    #[test]
    fn test_add_hashmap() {
        let mut b = EnvBuilder::fresh();
        add_hashmap(&mut b);
        assert!(b.contains("HashMap"));
        assert!(b.contains("HashMap.empty"));
        assert!(b.contains("HashMap.insert"));
    }
    #[test]
    fn test_add_io() {
        let mut b = EnvBuilder::fresh();
        add_io(&mut b);
        assert!(b.contains("IO"));
        assert!(b.contains("IO.pure"));
        assert!(b.contains("IO.bind"));
    }
    #[test]
    fn test_builder_is_ok_after_additions() {
        let mut b = EnvBuilder::fresh();
        add_empty(&mut b);
        add_unit_full(&mut b);
        assert!(b.is_ok());
    }
}
/// Build a lambda abstraction `fun (name : dom) => body` (extended version).
#[allow(dead_code)]
pub fn lam_ext(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
/// Build `Sort(Level::Param("u"))` — a polymorphic sort.
#[allow(dead_code)]
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
/// Build `Sort(Level::Param("v"))`.
#[allow(dead_code)]
pub fn sort_v() -> Expr {
    Expr::Sort(Level::Param(Name::str("v")))
}
/// Build a chain of non-dependent pi-types: A₁ → A₂ → … → ret.
#[allow(dead_code)]
pub fn pi_chain(domains: Vec<Expr>, ret: Expr) -> Expr {
    domains.into_iter().rev().fold(ret, |acc, dom| pi(dom, acc))
}
/// Build a `Const` with one universe parameter `u`.
#[allow(dead_code)]
pub fn var_u(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![Level::Param(Name::str("u"))])
}
/// Add propositional logic connectives (True, False, And, Or, Not, Iff, And.intro, etc.)
#[allow(dead_code)]
pub fn add_prop_logic(b: &mut EnvBuilder) {
    b.axiom("True", prop());
    b.axiom("False", prop());
    b.axiom("And", pi(prop(), pi(prop(), prop())));
    b.axiom("Or", pi(prop(), pi(prop(), prop())));
    b.axiom("Not", pi(prop(), prop()));
    b.axiom("Iff", pi(prop(), pi(prop(), prop())));
    b.axiom(
        "And.intro",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(bvar(1), pi(bvar(1), app(app(var("And"), bvar(3)), bvar(2)))),
            ),
        ),
    );
    b.axiom(
        "And.left",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(app(app(var("And"), bvar(1)), bvar(0)), bvar(2)),
            ),
        ),
    );
    b.axiom(
        "And.right",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(app(app(var("And"), bvar(1)), bvar(0)), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "Or.inl",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(bvar(1), app(app(var("Or"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Or.inr",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(bvar(0), app(app(var("Or"), bvar(2)), bvar(1))),
            ),
        ),
    );
}
