//! Definitional equality unfolds ONE definition at a time (Lean's lazy δ
//! reduction) instead of bringing both sides to full weak head normal form
//! first.
//!
//! The kernel used to start every comparison by reducing both sides to full
//! weak head normal form, unfolding every definition at the head. A proof of
//! the GRH export compares two terms that both mention `2000000 • x` in ℝ;
//! reducing them evaluated two million nested `Real.add`s before anything
//! was compared, which overflowed the stack or spent the whole budget —
//! where Lean's kernel, which unfolds the side with the greater definitional
//! height one step at a time and compares after each step, checks the proof
//! in a second. These tests pin the Lean behaviour on a chain of
//! definitions longer than the reducer's step cap (10,000): reaching the
//! bottom of the chain is impossible within the cap, so a comparison that
//! tried would fail and record a reduction limit.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal,
};
use oxilean_kernel::{
    fuel, BinderInfo, Environment, Expr, Level, Name, Node, ReducibilityHint, TypeChecker,
};

/// Longer than the reducer's chain cap.
const CHAIN: u32 = 20_000;

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn def(env: &mut Environment, name: &str, ty: Expr, value: Expr, height: u32) {
    let ci = ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        value,
        hints: ReducibilityHint::Regular(height),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str(name)],
    });
    if let Err(e) = env.add_constant(ci) {
        panic!("{name}: {e:?}");
    }
}

fn axiom(env: &mut Environment, name: &str, ty: Expr) {
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    });
    if let Err(e) = env.add_constant(ci) {
        panic!("{name}: {e:?}");
    }
}

/// `T : Type`, `c : T`, `d0 := c`, `d(i+1) := d i` (height `i + 1`), and two
/// identity functions `wrap1`, `wrap2 : T → T` of height 1, plus `id2 : T → T`
/// of height 2 defined as `fun x => wrap1 x`.
fn env() -> Environment {
    let mut env = Environment::new();
    axiom(&mut env, "T", Expr::Sort(Level::succ(Level::zero())));
    axiom(&mut env, "c", cnst("T"));
    def(&mut env, "d0", cnst("T"), cnst("c"), 1);
    for i in 1..=CHAIN {
        def(
            &mut env,
            &format!("d{i}"),
            cnst("T"),
            cnst(&format!("d{}", i - 1)),
            i + 1,
        );
    }
    let t_to_t = Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(cnst("T")),
        Node::new(cnst("T")),
    );
    let identity = Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(cnst("T")),
        Node::new(Expr::BVar(0)),
    );
    def(&mut env, "wrap1", t_to_t.clone(), identity.clone(), 1);
    def(&mut env, "wrap2", t_to_t.clone(), identity, 1);
    let via_wrap1 = Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(cnst("T")),
        Node::new(app(cnst("wrap1"), Expr::BVar(0))),
    );
    def(&mut env, "id2", t_to_t, via_wrap1, 2);
    env
}

fn top() -> Expr {
    cnst(&format!("d{CHAIN}"))
}

#[test]
fn two_definitions_of_equal_height_meet_after_one_unfolding_each() {
    let env = env();
    fuel::set_budget(Some(1 << 24));
    let mut tc = TypeChecker::new(&env);
    // `wrap1 dN =?= wrap2 dN`: different heads of the same height — unfold
    // both, and the two sides are the same term. Bringing them to weak head
    // normal form first walks the whole chain twice.
    assert!(tc.is_def_eq(&app(cnst("wrap1"), top()), &app(cnst("wrap2"), top())));
    assert!(
        !fuel::reduction_limit_hit(),
        "the comparison must not have walked the chain"
    );
    fuel::set_budget(None);
}

#[test]
fn the_higher_definition_is_unfolded_first() {
    let env = env();
    fuel::set_budget(Some(1 << 24));
    let mut tc = TypeChecker::new(&env);
    // `id2 dN =?= wrap1 dN`: `id2` is higher, so it is unfolded (to
    // `wrap1 dN`) and the sides meet — `wrap1` and the chain stay folded.
    assert!(tc.is_def_eq(&app(cnst("id2"), top()), &app(cnst("wrap1"), top())));
    // `dN =?= d(N-1)`: `dN` is higher; one unfolding makes them equal.
    let below = cnst(&format!("d{}", CHAIN - 1));
    assert!(tc.is_def_eq(&top(), &below));
    assert!(tc.is_def_eq(&below, &top()));
    assert!(!fuel::reduction_limit_hit());
    fuel::set_budget(None);
}

#[test]
fn unequal_terms_are_still_told_apart() {
    let mut env = env();
    axiom(&mut env, "c'", cnst("T"));
    def(&mut env, "e0", cnst("T"), cnst("c'"), 1);
    fuel::set_budget(Some(1 << 24));
    let mut tc = TypeChecker::new(&env);
    // `d1` and `e0` bottom out in different axioms.
    assert!(!tc.is_def_eq(&cnst("d1"), &cnst("e0")));
    assert!(!tc.is_def_eq(&app(cnst("wrap1"), cnst("c")), &cnst("c'")));
    // And a short chain still reaches the bottom when it has to.
    assert!(tc.is_def_eq(&cnst("d5"), &cnst("c")));
    assert!(tc.is_def_eq(&cnst("c"), &app(cnst("id2"), cnst("d3"))));
    fuel::set_budget(None);
}
