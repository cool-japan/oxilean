//! Tests of substitution, lifting and abstraction (`super::functions`).
//!
//! Two kinds:
//!
//! * differential tests against straightforward references that rebuild
//!   every node and share nothing — on random trees (the Stage E-2 guard)
//!   and on random terms with shared subterms, where one allocation is
//!   reached along several paths and at several binder depths;
//! * what the walks keep and what they charge: an untouched subterm is the
//!   allocation it was, a shared subterm is rebuilt once and stays shared,
//!   and the per-declaration budget is charged one unit per subterm reached.

use super::functions::{abstract_expr, instantiate, instantiate_rev_nodes};
use crate::{BinderInfo, Expr, FVarId, Level, Name, Node};

// --- Stage E-2 differential test: the structural-sharing skip in
// `instantiate`/`lift_loose_bvars` must produce results IDENTICAL to a
// straightforward no-skip reference, on many random OPEN terms. This is the
// guard the reverted Stage-D attempt lacked — a subtle skip bug there slid
// past every hand-written test and only surfaced as a corpus rejection.

/// Reference lift with NO range-skip.
fn lift_ref(e: &Expr, n: u32, depth: u32) -> Expr {
    if n == 0 {
        return e.clone();
    }
    match e {
        Expr::BVar(idx) => {
            if *idx >= depth {
                Expr::BVar(idx + n)
            } else {
                e.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => e.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(lift_ref(f, n, depth)),
            Node::new(lift_ref(a, n, depth)),
        ),
        Expr::Lam(bi, nm, ty, b) => Expr::Lam(
            *bi,
            nm.clone(),
            Node::new(lift_ref(ty, n, depth)),
            Node::new(lift_ref(b, n, depth + 1)),
        ),
        Expr::Pi(bi, nm, ty, b) => Expr::Pi(
            *bi,
            nm.clone(),
            Node::new(lift_ref(ty, n, depth)),
            Node::new(lift_ref(b, n, depth + 1)),
        ),
        Expr::Let(nm, ty, v, b) => Expr::Let(
            nm.clone(),
            Node::new(lift_ref(ty, n, depth)),
            Node::new(lift_ref(v, n, depth)),
            Node::new(lift_ref(b, n, depth + 1)),
        ),
        Expr::Proj(nm, i, x) => Expr::Proj(nm.clone(), *i, Node::new(lift_ref(x, n, depth))),
    }
}

/// Reference instantiate with NO range-skip.
fn inst_ref(expr: &Expr, arg: &Expr, depth: u32) -> Expr {
    match expr {
        Expr::BVar(k) => {
            if *k == depth {
                lift_ref(arg, depth, 0)
            } else if *k > depth {
                Expr::BVar(k - 1)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(inst_ref(f, arg, depth)),
            Node::new(inst_ref(a, arg, depth)),
        ),
        Expr::Lam(bi, nm, ty, b) => Expr::Lam(
            *bi,
            nm.clone(),
            Node::new(inst_ref(ty, arg, depth)),
            Node::new(inst_ref(b, arg, depth + 1)),
        ),
        Expr::Pi(bi, nm, ty, b) => Expr::Pi(
            *bi,
            nm.clone(),
            Node::new(inst_ref(ty, arg, depth)),
            Node::new(inst_ref(b, arg, depth + 1)),
        ),
        Expr::Let(nm, ty, v, b) => Expr::Let(
            nm.clone(),
            Node::new(inst_ref(ty, arg, depth)),
            Node::new(inst_ref(v, arg, depth)),
            Node::new(inst_ref(b, arg, depth + 1)),
        ),
        Expr::Proj(nm, i, x) => Expr::Proj(nm.clone(), *i, Node::new(inst_ref(x, arg, depth))),
    }
}

fn lcg(s: &mut u64) -> u64 {
    *s = s
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *s >> 33
}

/// Generate a random term with up to `fuel` depth, referencing up to
/// `binders` enclosing binders plus a few LOOSE bvars beyond them.
fn gen(s: &mut u64, fuel: u32, binders: u32) -> Expr {
    if fuel == 0 || lcg(s) % 100 < 35 {
        match lcg(s) % 4 {
            0 => Expr::BVar((lcg(s) as u32) % (binders + 3)),
            1 => Expr::Sort(Level::zero()),
            2 => Expr::FVar(FVarId(lcg(s) % 4)),
            _ => Expr::Const(Name::str("c"), Vec::new()),
        }
    } else {
        match lcg(s) % 5 {
            0 => Expr::App(
                Node::new(gen(s, fuel - 1, binders)),
                Node::new(gen(s, fuel - 1, binders)),
            ),
            1 => Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(gen(s, fuel - 1, binders)),
                Node::new(gen(s, fuel - 1, binders + 1)),
            ),
            2 => Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(gen(s, fuel - 1, binders)),
                Node::new(gen(s, fuel - 1, binders + 1)),
            ),
            3 => Expr::Let(
                Name::str("x"),
                Node::new(gen(s, fuel - 1, binders)),
                Node::new(gen(s, fuel - 1, binders)),
                Node::new(gen(s, fuel - 1, binders + 1)),
            ),
            _ => Expr::Proj(Name::str("S"), 0, Node::new(gen(s, fuel - 1, binders))),
        }
    }
}

#[test]
fn instantiate_skip_matches_reference() {
    let mut s: u64 = 0x1234_5678_9abc_def0;
    for _ in 0..3000 {
        let body = gen(&mut s, 5, 1);
        let arg = gen(&mut s, 4, 0);
        let got = instantiate(&body, &arg);
        let want = inst_ref(&body, &arg, 0);
        assert_eq!(
            got, want,
            "instantiate range-skip diverged from reference\nbody={body:?}\narg={arg:?}"
        );
    }
}

#[test]
fn lift_skip_matches_reference() {
    let mut s: u64 = 0xdead_beef_cafe_babe;
    for _ in 0..3000 {
        let e = gen(&mut s, 5, 0);
        let n = 1 + (lcg(&mut s) as u32 % 3);
        let off = lcg(&mut s) as u32 % 3;
        let got = crate::expr_util::lift_loose_bvars(&e, n, off);
        let want = lift_ref(&e, n, off);
        assert_eq!(
            got, want,
            "lift range-skip diverged from reference\ne={e:?} n={n} off={off}"
        );
    }
}

/// `Expr::range()` (the cached looseBVarRange) must equal a direct
/// recomputation from scratch.
#[test]
fn cached_range_matches_recomputation() {
    fn recompute(e: &Expr) -> u32 {
        match e {
            Expr::BVar(n) => n.saturating_add(1),
            Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
            Expr::App(f, a) => recompute(f).max(recompute(a)),
            Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => {
                recompute(ty).max(recompute(b).saturating_sub(1))
            }
            Expr::Let(_, ty, v, b) => recompute(ty)
                .max(recompute(v))
                .max(recompute(b).saturating_sub(1)),
            Expr::Proj(_, _, x) => recompute(x),
        }
    }
    let mut s: u64 = 0x0f0f_0f0f_1234_9999;
    for _ in 0..3000 {
        let e = gen(&mut s, 6, 0);
        assert_eq!(e.range(), recompute(&e), "cached range wrong for {e:?}");
    }
}

/// `Expr::rebuild_cost()` (cached) must equal a from-scratch recomputation:
/// it is the size of the term as a tree, which the caches read.
#[test]
fn cached_rebuild_cost_matches_recomputation() {
    fn recompute(e: &Expr) -> u32 {
        match e {
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 2,
            Expr::App(f, a) => 1u32
                .saturating_add(recompute(f))
                .saturating_add(recompute(a)),
            Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => 1u32
                .saturating_add(recompute(ty))
                .saturating_add(recompute(b)),
            Expr::Let(_, ty, v, b) => 1u32
                .saturating_add(recompute(ty))
                .saturating_add(recompute(v))
                .saturating_add(recompute(b)),
            Expr::Proj(_, _, x) => 1u32.saturating_add(recompute(x)),
        }
    }
    let mut s: u64 = 0xabcd_1234_5678_ef01;
    for _ in 0..3000 {
        let e = gen(&mut s, 6, 0);
        assert_eq!(
            e.rebuild_cost(),
            recompute(&e),
            "cached cost wrong for {e:?}"
        );
    }
}

// --- 0.1.4: the walks keep what a term shares, and charge for what they
// reach. The terms below are DAGs: a subterm is one allocation however
// many parents it has, and it is used under binders too, so one
// allocation is reached at several binder depths.

/// The largest term, as a tree, the tree-walking references below are
/// asked about (they visit every path).
const REFERENCE_MAX_COST: u32 = 50_000;

/// A random term that reuses subterms it has already built, small
/// enough as a tree for the references.
fn gen_dag(s: &mut u64, steps: usize) -> Expr {
    loop {
        let term = gen_dag_any_size(s, steps);
        if term.rebuild_cost() <= REFERENCE_MAX_COST {
            return term;
        }
    }
}

/// A random term that reuses subterms it has already built.
fn gen_dag_any_size(s: &mut u64, steps: usize) -> Expr {
    fn leaf(s: &mut u64) -> Expr {
        match lcg(s) % 5 {
            0 | 1 => Expr::BVar((lcg(s) % 5) as u32),
            2 => Expr::Sort(Level::zero()),
            3 => Expr::FVar(FVarId(lcg(s) % 4)),
            _ => Expr::Const(Name::str("c"), Vec::new()),
        }
    }
    let mut pool: Vec<Node> = (0..4).map(|_| Node::new(leaf(s))).collect();
    for _ in 0..steps {
        let pick = |s: &mut u64, pool: &Vec<Node>| -> Node {
            if lcg(s).is_multiple_of(5) {
                Node::new(leaf(s))
            } else {
                pool[(lcg(s) as usize) % pool.len()].clone()
            }
        };
        let node = match lcg(s) % 6 {
            0 | 1 => Expr::App(pick(s, &pool), pick(s, &pool)),
            2 => Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                pick(s, &pool),
                pick(s, &pool),
            ),
            3 => Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                pick(s, &pool),
                pick(s, &pool),
            ),
            4 => Expr::Let(
                Name::str("x"),
                pick(s, &pool),
                pick(s, &pool),
                pick(s, &pool),
            ),
            _ => Expr::Proj(Name::str("S"), 0, pick(s, &pool)),
        };
        pool.push(Node::new(node));
    }
    let last = pool.len() - 1;
    (*pool[last]).clone()
}

/// How many allocations a term is made of (its own root not counted).
fn allocations(e: &Expr) -> usize {
    fn visit<'a>(e: &'a Expr, seen: &mut std::collections::HashSet<*const Expr>) {
        let mut child = |node: &'a Node| {
            if seen.insert(node.as_ptr()) {
                visit(node, seen);
            }
        };
        match e {
            Expr::App(f, a) => {
                child(f);
                child(a);
            }
            Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => {
                child(ty);
                child(b);
            }
            Expr::Let(_, ty, v, b) => {
                child(ty);
                child(v);
                child(b);
            }
            Expr::Proj(_, _, x) => child(x),
            _ => {}
        }
    }
    let mut seen = std::collections::HashSet::new();
    visit(e, &mut seen);
    seen.len()
}

/// The fuel a closure charges.
fn fuel_of<T>(work: impl FnOnce() -> T) -> (T, u64) {
    crate::fuel::set_budget(Some(u64::MAX));
    let result = work();
    let used = crate::fuel::used();
    crate::fuel::set_budget(None);
    (result, used)
}

/// `xₖ₊₁ = xₖ xₖ` over `leaf`, both children one allocation: `height`
/// allocations (and the leaf) standing for a tree of `2^height` leaves.
fn tower(leaf: Expr, height: u32) -> Expr {
    let mut term = leaf;
    for _ in 0..height {
        let shared = Node::new(term);
        term = Expr::App(shared.clone(), shared);
    }
    term
}

#[test]
fn instantiate_matches_reference_on_shared_terms() {
    let mut s: u64 = 0x5eed_5eed_0000_0001;
    let mut rebuilt_something = 0;
    for _ in 0..2000 {
        let body = gen_dag(&mut s, 14);
        let arg = gen_dag(&mut s, 6);
        let got = instantiate(&body, &arg);
        let want = inst_ref(&body, &arg, 0);
        assert_eq!(got, want, "body={body:?}\narg={arg:?}");
        if body.range() > 0 {
            rebuilt_something += 1;
        }
    }
    assert!(rebuilt_something > 500, "{rebuilt_something} open bodies");
}

#[test]
fn lift_matches_reference_on_shared_terms() {
    let mut s: u64 = 0x5eed_5eed_0000_0002;
    for _ in 0..2000 {
        let e = gen_dag(&mut s, 14);
        let n = lcg(&mut s) as u32 % 4;
        let off = lcg(&mut s) as u32 % 3;
        let got = crate::expr_util::lift_loose_bvars(&e, n, off);
        let want = lift_ref(&e, n, off);
        assert_eq!(got, want, "e={e:?} n={n} off={off}");
    }
}

#[test]
fn several_arguments_at_once_equal_one_at_a_time() {
    // `(λx₁ … xₖ. body) a₁ … aₖ`, reduced by `instantiate_rev_nodes` in
    // one walk and by `k` nested `instantiate` calls — the arguments
    // have loose variables of their own, which must come out the same.
    let mut s: u64 = 0x5eed_5eed_0000_0003;
    for _ in 0..2000 {
        let k = 1 + (lcg(&mut s) as usize % 4);
        let body = gen_dag(&mut s, 12);
        let args: Vec<Node> = (0..k).map(|_| Node::new(gen_dag(&mut s, 5))).collect();
        let arg_refs: Vec<&Node> = args.iter().collect();
        let got = instantiate_rev_nodes(&body, &arg_refs);

        let mut want = body.clone();
        for _ in 0..k {
            want = Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Sort(Level::zero())),
                Node::new(want),
            );
        }
        for arg in &args {
            want = match want {
                Expr::Lam(_, _, _, inner) => instantiate(&inner, arg),
                other => panic!("a lambda for every argument, got {other:?}"),
            };
        }
        assert_eq!(got, want, "k={k}\nbody={body:?}\nargs={args:?}");
    }
    // No argument: nothing happens.
    let body = Expr::App(Node::new(Expr::BVar(0)), Node::new(Expr::BVar(1)));
    assert_eq!(instantiate_rev_nodes(&body, &[]), body);
}

// (The towers below are 20 to 22 levels: a few dozen allocations
// standing for millions of tree nodes. Tall enough that a walk of the
// tree is charged tens of thousands of times what a walk of the shared
// term is, and short enough that such a walk — a regression — still
// finishes and FAILS the assertion instead of hanging the test.)

#[test]
fn a_shared_subterm_is_rebuilt_once_and_stays_shared() {
    // 22 allocations standing for 2^22 occurrences of the variable.
    let body = tower(Expr::BVar(0), 22);
    let arg = Expr::Const(Name::str("c"), Vec::new());
    let (result, used) = fuel_of(|| instantiate(&body, &arg));
    assert!(used < 500, "a 22-level tower charged {used} units");
    // The same tower over the constant: both children of every level are
    // one allocation.
    let mut cur = &result;
    let mut levels = 0;
    while let Expr::App(f, a) = cur {
        assert!(Node::ptr_eq(f, a), "level {levels} was split in two");
        cur = f;
        levels += 1;
    }
    assert_eq!(levels, 22);
    assert_eq!(*cur, arg);
    assert_eq!(allocations(&result), 22);

    // Under binders too: the same shared subterm met at two depths is
    // rebuilt once per depth, not once per path.
    let shared = Node::new(tower(Expr::BVar(0), 20));
    let under = Expr::Lam(
        BinderInfo::Default,
        Name::str("y"),
        Node::new(Expr::Sort(Level::zero())),
        Node::new(Expr::App(shared.clone(), shared.clone())),
    );
    let body = Expr::App(shared.clone(), Node::new(under));
    let (result, used) = fuel_of(|| instantiate(&body, &Expr::FVar(FVarId(7))));
    assert!(used < 500, "two depths of a 20-level tower charged {used}");
    // At depth 0 the variable is the argument: the tower is rebuilt over
    // it, once. At depth 1 (under `y`) `#0` is `y` itself: the tower is
    // untouched there and stays the allocation it was.
    let Expr::App(outside, lambda) = &result else {
        panic!("an application, got {result:?}");
    };
    // (Compared once per shared pair: `==` would walk both towers as trees.)
    assert!(crate::term_cache::same_term(
        outside,
        &tower(Expr::FVar(FVarId(7)), 20)
    ));
    let Expr::Lam(_, _, _, inner) = &**lambda else {
        panic!("a lambda, got {lambda:?}");
    };
    let Expr::App(left, right) = &**inner else {
        panic!("an application, got {inner:?}");
    };
    assert!(Node::ptr_eq(left, &shared) && Node::ptr_eq(right, &shared));
    assert!(allocations(&result) < 80, "{}", allocations(&result));

    // The same variable under a binder: the tower is rebuilt once at
    // that depth too, and both uses of it there share the result.
    let deep = Node::new(tower(Expr::BVar(1), 20));
    let under = Expr::Lam(
        BinderInfo::Default,
        Name::str("y"),
        Node::new(Expr::Sort(Level::zero())),
        Node::new(Expr::App(deep.clone(), deep)),
    );
    let (result, used) = fuel_of(|| instantiate(&under, &Expr::FVar(FVarId(7))));
    assert!(used < 300, "a 20-level tower under a binder charged {used}");
    let Expr::Lam(_, _, _, inner) = &result else {
        panic!("a lambda, got {result:?}");
    };
    let Expr::App(left, right) = &**inner else {
        panic!("an application, got {inner:?}");
    };
    assert!(Node::ptr_eq(left, right));
    assert!(crate::term_cache::same_term(
        left,
        &tower(Expr::FVar(FVarId(7)), 20)
    ));
}

#[test]
fn untouched_subterms_are_the_allocations_they_were() {
    let closed = Node::new(tower(Expr::Const(Name::str("c"), Vec::new()), 22));
    let body = Expr::App(closed.clone(), Node::new(Expr::BVar(0)));
    let arg = Node::new(Expr::FVar(FVarId(3)));

    let result = instantiate(&body, &arg);
    let Expr::App(f, a) = &result else {
        panic!("an application, got {result:?}");
    };
    assert!(Node::ptr_eq(f, &closed), "the closed subterm was copied");
    assert_eq!(**a, *arg);

    // Held as an allocation, the argument itself goes in.
    let result = instantiate_rev_nodes(&body, &[&arg]);
    let Expr::App(f, a) = &result else {
        panic!("an application, got {result:?}");
    };
    assert!(Node::ptr_eq(f, &closed));
    assert!(Node::ptr_eq(a, &arg), "the argument was copied");

    // A closed body is returned as it is.
    assert_eq!(instantiate(&closed, &arg), *closed);
}

#[test]
fn fuel_is_one_unit_per_subterm_reached() {
    let c = || Expr::Const(Name::str("c"), Vec::new());
    // (c #0) (λ _ : Sort. #1 c)
    let body = Expr::App(
        Node::new(Expr::App(Node::new(c()), Node::new(Expr::BVar(0)))),
        Node::new(Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Node::new(Expr::Sort(Level::zero())),
            Node::new(Expr::App(Node::new(Expr::BVar(1)), Node::new(c()))),
        )),
    );
    let arg = Expr::FVar(FVarId(1));
    let (result, used) = fuel_of(|| instantiate(&body, &arg));
    assert_eq!(result, inst_ref(&body, &arg, 0));
    // The body (1); `c #0` (1), its `c` (1) and its variable (1, and 1
    // for the copy of the argument that replaces it); the lambda (1),
    // its type (1), its body (1), the variable (1, and 1 for the copy of
    // the argument under the binder), the `c` (1).
    assert_eq!(used, 11);

    // Deterministic: the same call charges the same.
    let (_, again) = fuel_of(|| instantiate(&body, &arg));
    assert_eq!(again, used);
}

#[test]
fn a_subterm_shared_unchanged_costs_one_unit_whatever_its_size() {
    // The closed subterm is up to 2^22 leaves as a tree. Through 0.1.4's
    // first performance pass sharing it was charged that size, which is
    // what put heavily shared proof terms over the budget.
    let c = Expr::Const(Name::str("c"), Vec::new());
    for height in [1, 10, 22] {
        let closed = Node::new(tower(c.clone(), height));
        let body = Expr::App(closed, Node::new(Expr::BVar(0)));
        let (_, used) = fuel_of(|| instantiate(&body, &c));
        // The body, the closed subterm, the variable, the argument.
        assert_eq!(used, 4, "height {height}");
        let (_, used) = fuel_of(|| crate::expr_util::lift_loose_bvars(&body, 3, 0));
        // The body, the closed subterm, the variable.
        assert_eq!(used, 3, "height {height}");
    }
}

/// Reference abstraction: every node rebuilt, nothing shared.
fn abstract_ref(e: &Expr, fvar: FVarId, depth: u32) -> Expr {
    match e {
        Expr::FVar(id) if *id == fvar => Expr::BVar(depth),
        Expr::BVar(n) if *n >= depth => Expr::BVar(*n + 1),
        Expr::BVar(_) | Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            e.clone()
        }
        Expr::App(f, a) => Expr::App(
            Node::new(abstract_ref(f, fvar, depth)),
            Node::new(abstract_ref(a, fvar, depth)),
        ),
        Expr::Lam(bi, nm, ty, b) => Expr::Lam(
            *bi,
            nm.clone(),
            Node::new(abstract_ref(ty, fvar, depth)),
            Node::new(abstract_ref(b, fvar, depth + 1)),
        ),
        Expr::Pi(bi, nm, ty, b) => Expr::Pi(
            *bi,
            nm.clone(),
            Node::new(abstract_ref(ty, fvar, depth)),
            Node::new(abstract_ref(b, fvar, depth + 1)),
        ),
        Expr::Let(nm, ty, v, b) => Expr::Let(
            nm.clone(),
            Node::new(abstract_ref(ty, fvar, depth)),
            Node::new(abstract_ref(v, fvar, depth)),
            Node::new(abstract_ref(b, fvar, depth + 1)),
        ),
        Expr::Proj(nm, i, x) => Expr::Proj(nm.clone(), *i, Node::new(abstract_ref(x, fvar, depth))),
    }
}

#[test]
fn abstract_matches_reference_on_shared_terms() {
    let mut s: u64 = 0x5eed_5eed_0000_0004;
    let mut changed = 0;
    for _ in 0..2000 {
        let e = gen_dag(&mut s, 14);
        let fvar = FVarId(lcg(&mut s) % 4);
        let got = abstract_expr(&e, fvar);
        let want = abstract_ref(&e, fvar, 0);
        assert_eq!(got, want, "e={e:?} fvar={fvar:?}");
        if got != e {
            changed += 1;
        }
    }
    assert!(changed > 500, "{changed} terms changed");
}

#[test]
fn abstraction_is_undone_by_instantiation() {
    // For a term with no loose bound variable — what the type checker
    // abstracts — opening the binder again gives the term back.
    let mut s: u64 = 0x5eed_5eed_0000_0005;
    let mut checked = 0;
    for _ in 0..4000 {
        let e = gen_dag(&mut s, 12);
        if e.range() > 0 {
            continue;
        }
        let fvar = FVarId(lcg(&mut s) % 4);
        let closed_over = abstract_expr(&e, fvar);
        assert_eq!(instantiate(&closed_over, &Expr::FVar(fvar)), e);
        checked += 1;
    }
    assert!(checked > 200, "{checked} closed terms");
}

#[test]
fn abstraction_leaves_what_it_does_not_change_in_place() {
    let x = FVarId(5);
    let other = Node::new(tower(Expr::FVar(FVarId(6)), 22));
    let closed = Node::new(tower(Expr::Const(Name::str("c"), Vec::new()), 22));
    let e = Expr::App(
        Node::new(Expr::App(other.clone(), closed.clone())),
        Node::new(Expr::FVar(x)),
    );
    let (result, used) = fuel_of(|| abstract_expr(&e, x));
    let Expr::App(left, right) = &result else {
        panic!("an application, got {result:?}");
    };
    assert_eq!(**right, Expr::BVar(0));
    let Expr::App(kept_other, kept_closed) = &**left else {
        panic!("an application, got {left:?}");
    };
    // Neither holds `x`: the tower of ANOTHER variable is walked (once
    // per allocation) and found unchanged; the closed one is not entered.
    assert!(Node::ptr_eq(kept_other, &other));
    assert!(Node::ptr_eq(kept_closed, &closed));
    assert!(used < 200, "charged {used} units");

    // A term the variable does not occur in comes back equal, and its
    // children are the same allocations.
    let untouched = Expr::App(other.clone(), closed.clone());
    let result = abstract_expr(&untouched, x);
    let Expr::App(f, a) = &result else {
        panic!("an application, got {result:?}");
    };
    assert!(Node::ptr_eq(f, &other) && Node::ptr_eq(a, &closed));
}

#[test]
fn an_index_that_cannot_be_raised_stays_loose() {
    // `u32::MAX` cannot be incremented. It must not wrap round to a small
    // index, which a binder inside the term could capture.
    let top = Expr::BVar(u32::MAX);
    assert_eq!(crate::expr_util::lift_loose_bvars(&top, 1, 0), top);
    assert_eq!(crate::expr_util::lift_loose_bvars(&top, u32::MAX, 0), top);
    assert_eq!(abstract_expr(&top, FVarId(0)), top);
    let under = Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(Expr::Sort(Level::zero())),
        Node::new(Expr::BVar(u32::MAX)),
    );
    let lifted = crate::expr_util::lift_loose_bvars(&under, 5, 0);
    assert_eq!(lifted, under);
}

#[test]
fn test_instantiate_simple() {
    let bvar0 = Expr::BVar(0);
    let fvar = Expr::FVar(FVarId(1));
    let result = instantiate(&bvar0, &fvar);
    assert_eq!(result, fvar);
}
#[test]
fn test_instantiate_shift_down() {
    let bvar1 = Expr::BVar(1);
    let arg = Expr::FVar(FVarId(1));
    let result = instantiate(&bvar1, &arg);
    assert_eq!(result, Expr::BVar(0));
}
#[test]
fn test_abstract_roundtrip() {
    let fvar_id = FVarId(42);
    let fvar = Expr::FVar(fvar_id);
    let abstracted = abstract_expr(&fvar, fvar_id);
    let back = instantiate(&abstracted, &fvar);
    assert_eq!(back, fvar);
}
#[test]
fn test_instantiate_app() {
    let app = Expr::App(Node::new(Expr::BVar(0)), Node::new(Expr::BVar(1)));
    let arg = Expr::FVar(FVarId(99));
    let result = instantiate(&app, &arg);
    match result {
        Expr::App(f, a) => {
            assert_eq!(*f, arg);
            assert_eq!(*a, Expr::BVar(0));
        }
        _ => panic!("Expected App"),
    }
}
