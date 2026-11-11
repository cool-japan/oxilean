//! The mirrors of a checker's local context hold the variables in scope, and
//! only those (see `crate::local_types`).
//!
//! A `TypeChecker` mirrors the type of every local free variable into its
//! reducer, its def-eq checker and that checker's reducer. Through 0.1.3 an
//! entry was never removed; these tests pin that an entry now leaves all
//! three when its binder closes, that a variable still in scope keeps its
//! type meanwhile, and that a nested checker is still kept clear of the ids
//! of binders that have closed.

use super::types::TypeChecker;
use crate::def_eq::types::DEF_EQ_OPENED_FVAR_BASE;
use crate::{BinderInfo, Environment, Expr, FVarId, Level, LocalDecl, Name, Node};

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn lam(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(ty),
        Node::new(body),
    )
}

/// `fun (n₁ … nₖ : Prop) => #(k-1)` — the outermost variable, under `k`
/// binders named by `names`. Typeable in an empty environment.
fn nested(names: &[&str]) -> Expr {
    let outermost = u32::try_from(names.len().saturating_sub(1)).unwrap_or(u32::MAX);
    names
        .iter()
        .rev()
        .fold(Expr::BVar(outermost), |body, name| lam(name, prop(), body))
}

/// How many variables each of the three mirrors holds.
fn live_counts(tc: &TypeChecker<'_>) -> [usize; 3] {
    let (mirrors, _) = tc.local_mirrors();
    mirrors.map(|mirror| mirror.live().len())
}

#[test]
fn inferring_a_term_leaves_no_binder_in_the_mirrors() {
    let env = Environment::new();
    let mut tc = TypeChecker::new(&env);
    let term = nested(&["x", "y", "z"]);
    let ty = tc
        .infer_type(&term)
        .unwrap_or_else(|e| panic!("`fun x y z : Prop => x` must be typeable: {e:?}"));
    assert!(ty.is_pi(), "expected a Pi type, got {ty}");
    assert!(tc.local_ctx().is_empty());
    assert_eq!(live_counts(&tc), [0, 0, 0]);
    // The three binders were opened with ids 0, 1 and 2: closed, but on
    // record, as one run of ids.
    let (mirrors, _) = tc.local_mirrors();
    for mirror in mirrors {
        assert_eq!(mirror.max_own_id(), Some(2));
        assert_eq!(mirror.max_opened_id(), None);
        assert_eq!(mirror.closed_range_count(), 1);
        for id in 0..3 {
            assert_eq!(mirror.get(FVarId(id)), None);
        }
    }
}

#[test]
fn comparing_binders_leaves_no_opened_variable_in_the_mirrors() {
    let env = Environment::new();
    let mut tc = TypeChecker::new(&env);
    // Equal up to binder names, so the comparison is not syntactic: the
    // def-eq checker opens both binders.
    let lhs = nested(&["x", "y"]);
    let rhs = nested(&["a", "b"]);
    assert_ne!(lhs, rhs);
    assert!(tc.is_def_eq(&lhs, &rhs));
    assert_eq!(live_counts(&tc), [0, 0, 0]);
    let ([own_reducer, def_eq, def_eq_reducer], next_opened) = tc.local_mirrors();
    // The checker's own reducer never saw the opened variables.
    assert_eq!(own_reducer.max_opened_id(), None);
    // The def-eq checker opened two binders and closed both.
    for mirror in [def_eq, def_eq_reducer] {
        assert_eq!(mirror.max_opened_id(), Some(DEF_EQ_OPENED_FVAR_BASE + 1));
        assert_eq!(mirror.max_own_id(), None);
        assert_eq!(mirror.closed_range_count(), 1);
    }
    // Their ids are spent: the next binder gets a new one.
    assert_eq!(next_opened, DEF_EQ_OPENED_FVAR_BASE + 2);
}

#[test]
fn a_local_stays_in_the_mirrors_while_its_binder_is_open() {
    let env = Environment::new();
    let mut tc = TypeChecker::new(&env);
    let h = tc.fresh_fvar(Name::str("h"), prop());
    assert_eq!(live_counts(&tc), [1, 1, 1]);
    // Binders opened and closed above `h`, by inference and by def-eq.
    let over_h = lam("x", prop(), lam("y", prop(), Expr::FVar(h)));
    tc.infer_type(&over_h)
        .unwrap_or_else(|e| panic!("`fun x y : Prop => h` must be typeable: {e:?}"));
    assert!(tc.is_def_eq(&nested(&["x", "y"]), &nested(&["a", "b"])));
    // `h`, and only `h`, is still there, with its type.
    assert_eq!(live_counts(&tc), [1, 1, 1]);
    let (mirrors, _) = tc.local_mirrors();
    for mirror in mirrors {
        assert_eq!(mirror.get(h), Some(&prop()));
    }
    // Closing `h` empties the mirrors.
    let popped = tc.pop_local();
    assert_eq!(popped.map(|decl| decl.fvar), Some(h));
    assert_eq!(live_counts(&tc), [0, 0, 0]);
}

#[test]
fn a_nested_checker_mints_past_closed_binders() {
    let env = Environment::new();
    let mut tc = TypeChecker::new(&env);
    // Own range: ids 0, 1, 2 opened; 2 and 1 closed again.
    let outer = tc.fresh_fvar(Name::str("a"), prop());
    tc.fresh_fvar(Name::str("b"), prop());
    tc.fresh_fvar(Name::str("c"), prop());
    tc.pop_local();
    tc.pop_local();
    // Opened range: two binders opened and closed by the def-eq checker.
    assert!(tc.is_def_eq(&nested(&["x", "y"]), &nested(&["a", "b"])));

    let ([_, _, mirror], _) = tc.local_mirrors();
    assert_eq!(mirror.live().len(), 1);
    assert_eq!(mirror.get(outer), Some(&prop()));

    // Seeded from the variables in scope ALONE, a checker would hand out the
    // ids of the closed binders again, in both ranges.
    let mut from_live = TypeChecker::with_fvar_types(&env, mirror.live());
    assert_eq!(from_live.fresh_fvar(Name::str("z"), prop()), FVarId(1));
    assert_eq!(from_live.local_mirrors().1, DEF_EQ_OPENED_FVAR_BASE);

    // Seeded from the mirror, it mints past every id the mirror ever held —
    // the ids it got when the mirror kept every variable.
    let mut nested_tc = TypeChecker::with_local_types(&env, mirror);
    assert_eq!(nested_tc.local_mirrors().1, DEF_EQ_OPENED_FVAR_BASE + 2);
    assert_eq!(nested_tc.fresh_fvar(Name::str("z"), prop()), FVarId(3));
    // The variable in scope is typeable in it; the closed ones are not.
    assert_eq!(
        nested_tc.infer_type(&Expr::FVar(outer)).ok(),
        Some(prop()),
        "the seeded local must be typeable"
    );
    assert!(nested_tc.infer_type(&Expr::FVar(FVarId(1))).is_err());
}

#[test]
fn popping_one_of_two_declarations_of_a_variable_keeps_its_type() {
    let env = Environment::new();
    let mut tc = TypeChecker::new(&env);
    let x = FVarId(9);
    let decl = || LocalDecl {
        fvar: x,
        name: Name::str("x"),
        ty: prop(),
        val: None,
    };
    tc.push_local(decl());
    tc.push_local(decl());
    assert_eq!(live_counts(&tc), [1, 1, 1]);
    // One declaration of `x` is still in the context: it stays typed.
    tc.pop_local();
    assert_eq!(tc.local_ctx().len(), 1);
    assert_eq!(live_counts(&tc), [1, 1, 1]);
    // The last one closes it.
    tc.pop_local();
    assert_eq!(live_counts(&tc), [0, 0, 0]);
    // Popping an empty context is still `None`.
    assert!(tc.pop_local().is_none());
}
