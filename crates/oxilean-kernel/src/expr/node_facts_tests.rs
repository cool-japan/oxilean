//! Tests for the facts a `Node` caches besides its hash: the three content
//! flags (free variable, universe parameter, universe metavariable) and the
//! tree size they share a word with.
//!
//! What must hold, and is pinned here:
//!
//! * each flag is exactly "a subterm of that kind occurs somewhere" — the
//!   walks that pass over a subterm on the strength of a flag (abstraction,
//!   universe instantiation, the hygiene check of a declaration, the
//!   closed-term check) are only right if it is;
//! * the flags and the size do not disturb each other in their shared word;
//! * the O(1) predicates of `expr_util` answer what walking the term answers;
//! * `is_shared` says whether an allocation has more than one owner.

use super::types::{BinderInfo, Expr, FVarId, Literal, Node, REBUILD_COST_MAX};
use crate::expr_util::{has_any_fvar, has_level_mvar, has_level_param, has_loose_bvars};
use crate::{Level, LevelMVarId, LevelView, Name};

fn lcg(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state >> 33
}

fn random_level(state: &mut u64, depth: u32) -> Level {
    if depth == 0 || lcg(state).is_multiple_of(3) {
        return match lcg(state) % 4 {
            0 => Level::param(Name::str("u")),
            1 => Level::mvar(LevelMVarId(lcg(state) % 3)),
            _ => Level::zero(),
        };
    }
    match lcg(state) % 3 {
        0 => Level::succ(random_level(state, depth - 1)),
        1 => Level::max(
            random_level(state, depth - 1),
            random_level(state, depth - 1),
        ),
        _ => Level::imax(
            random_level(state, depth - 1),
            random_level(state, depth - 1),
        ),
    }
}

/// A random term that reuses subterms it has already built.
fn random_dag(state: &mut u64, steps: usize) -> Expr {
    fn leaf(state: &mut u64) -> Expr {
        match lcg(state) % 7 {
            0 => Expr::BVar((lcg(state) % 3) as u32),
            1 => Expr::Sort(random_level(state, 2)),
            2 => Expr::FVar(FVarId(lcg(state) % 3)),
            3 => Expr::Lit(Literal::nat(lcg(state) % 3)),
            4 => Expr::Const(
                Name::str("d"),
                vec![random_level(state, 2), random_level(state, 1)],
            ),
            _ => Expr::Const(Name::str("c"), Vec::new()),
        }
    }
    let mut pool: Vec<Node> = (0..3).map(|_| Node::new(leaf(state))).collect();
    for _ in 0..steps {
        let pick = |state: &mut u64, pool: &Vec<Node>| -> Node {
            if lcg(state).is_multiple_of(3) {
                Node::new(leaf(state))
            } else {
                pool[(lcg(state) as usize) % pool.len()].clone()
            }
        };
        let node = match lcg(state) % 6 {
            0 | 1 => Expr::App(pick(state, &pool), pick(state, &pool)),
            2 => Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                pick(state, &pool),
                pick(state, &pool),
            ),
            3 => Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                pick(state, &pool),
                pick(state, &pool),
            ),
            4 => Expr::Let(
                Name::str("x"),
                pick(state, &pool),
                pick(state, &pool),
                pick(state, &pool),
            ),
            _ => Expr::Proj(Name::str("S"), 0, pick(state, &pool)),
        };
        pool.push(Node::new(node));
    }
    let last = pool.len() - 1;
    (*pool[last]).clone()
}

fn level_has(level: &Level, param: bool) -> bool {
    match level.view() {
        LevelView::Zero => false,
        LevelView::Param(_) => param,
        LevelView::MVar(_) => !param,
        LevelView::Succ(l) => level_has(l, param),
        LevelView::Max(a, b) | LevelView::IMax(a, b) => level_has(a, param) || level_has(b, param),
    }
}

/// Whether some subterm of `e` satisfies `leaf`, by walking every path.
fn any_leaf(e: &Expr, leaf: &dyn Fn(&Expr) -> bool) -> bool {
    match e {
        Expr::App(f, a) => any_leaf(f, leaf) || any_leaf(a, leaf),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => any_leaf(ty, leaf) || any_leaf(b, leaf),
        Expr::Let(_, ty, v, b) => any_leaf(ty, leaf) || any_leaf(v, leaf) || any_leaf(b, leaf),
        Expr::Proj(_, _, s) => any_leaf(s, leaf),
        other => leaf(other),
    }
}

fn walk_has_fvar(e: &Expr) -> bool {
    any_leaf(e, &|leaf| matches!(leaf, Expr::FVar(_)))
}

fn walk_has_level(e: &Expr, param: bool) -> bool {
    any_leaf(e, &|leaf| match leaf {
        Expr::Sort(l) => level_has(l, param),
        Expr::Const(_, ls) => ls.iter().any(|l| level_has(l, param)),
        _ => false,
    })
}

/// The loose-bound-variable test by walking, as `has_loose_bvars` was.
fn walk_has_loose_bvar(e: &Expr, depth: u32) -> bool {
    match e {
        Expr::BVar(n) => *n >= depth,
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => walk_has_loose_bvar(f, depth) || walk_has_loose_bvar(a, depth),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => {
            walk_has_loose_bvar(ty, depth) || walk_has_loose_bvar(b, depth + 1)
        }
        Expr::Let(_, ty, v, b) => {
            walk_has_loose_bvar(ty, depth)
                || walk_has_loose_bvar(v, depth)
                || walk_has_loose_bvar(b, depth + 1)
        }
        Expr::Proj(_, _, s) => walk_has_loose_bvar(s, depth),
    }
}

#[test]
fn the_flags_say_what_a_walk_of_the_term_finds() {
    let mut state = 0x0fac_7500_0000_0001_u64;
    let mut seen = [0usize; 8];
    for i in 0..3000 {
        let term = loop {
            let term = random_dag(&mut state, 9);
            if term.rebuild_cost() <= 20_000 {
                break term;
            }
        };
        let fvar = walk_has_fvar(&term);
        let param = walk_has_level(&term, true);
        let mvar = walk_has_level(&term, false);
        assert_eq!(term.has_fvar(), fvar, "term {i}: {term:?}");
        assert_eq!(term.has_level_param(), param, "term {i}: {term:?}");
        assert_eq!(term.has_level_mvar(), mvar, "term {i}: {term:?}");
        // The public predicates read the same flags.
        assert_eq!(has_any_fvar(&term), fvar);
        assert_eq!(has_level_param(&term), param);
        assert_eq!(has_level_mvar(&term), mvar);
        assert_eq!(
            has_loose_bvars(&term),
            walk_has_loose_bvar(&term, 0),
            "term {i}: {term:?}"
        );
        // A node caches what its term computes.
        let node = Node::new(term.clone());
        assert_eq!(node.has_fvar(), fvar);
        assert_eq!(node.has_level_param(), param);
        assert_eq!(node.rebuild_cost(), term.rebuild_cost());
        seen[usize::from(fvar) | usize::from(param) << 1 | usize::from(mvar) << 2] += 1;
    }
    // Every combination of the three flags occurred.
    assert!(seen.iter().all(|&count| count > 0), "{seen:?}");
}

/// `xₖ₊₁ = xₖ xₖ`, both children one allocation.
fn tower(leaf: Expr, height: u32) -> Expr {
    let mut term = leaf;
    for _ in 0..height {
        let shared = Node::new(term);
        term = Expr::App(shared.clone(), shared);
    }
    term
}

#[test]
fn a_saturated_size_leaves_the_flags_alone_and_the_flags_leave_the_size_alone() {
    let u = Level::param(Name::str("u"));
    let cases: [(Expr, [bool; 3]); 5] = [
        (Expr::Const(Name::str("c"), vec![]), [false, false, false]),
        (Expr::FVar(FVarId(3)), [true, false, false]),
        (Expr::Sort(u.clone()), [false, true, false]),
        (
            Expr::Sort(Level::mvar(LevelMVarId(1))),
            [false, false, true],
        ),
        (
            Expr::App(
                Node::new(Expr::FVar(FVarId(0))),
                Node::new(Expr::Const(
                    Name::str("d"),
                    vec![Level::max(u, Level::mvar(LevelMVarId(2)))],
                )),
            ),
            [true, true, true],
        ),
    ];
    for (leaf, [fvar, param, mvar]) in cases {
        // Below the saturation point the size is exact, whatever the flags.
        let small = tower(leaf.clone(), 3);
        assert_eq!(small.rebuild_cost(), 8 * leaf.rebuild_cost() + 7);
        // 2^60 leaves: the size is the saturated value — never more, so it
        // cannot spill into the flag bits — and the flags are the leaf's.
        let big = tower(leaf.clone(), 60);
        for term in [&small, &big] {
            assert_eq!(term.has_fvar(), fvar, "{leaf:?}");
            assert_eq!(term.has_level_param(), param, "{leaf:?}");
            assert_eq!(term.has_level_mvar(), mvar, "{leaf:?}");
        }
        assert_eq!(big.rebuild_cost(), REBUILD_COST_MAX);
        let node = Node::new(big);
        assert_eq!(node.rebuild_cost(), REBUILD_COST_MAX);
        assert_eq!(node.has_fvar(), fvar);
        assert_eq!(node.has_level_param(), param);
    }
}

/// The depth of a term by walking it: the number of nodes on its longest
/// path down to a leaf.
#[cfg(feature = "term-depth")]
fn walk_depth(e: &Expr) -> u32 {
    1 + match e {
        Expr::App(f, a) => walk_depth(f).max(walk_depth(a)),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => walk_depth(ty).max(walk_depth(b)),
        Expr::Let(_, ty, v, b) => walk_depth(ty).max(walk_depth(v)).max(walk_depth(b)),
        Expr::Proj(_, _, s) => walk_depth(s),
        _ => 0,
    }
}

#[cfg(feature = "term-depth")]
#[test]
fn the_cached_depth_is_the_longest_path_to_a_leaf() {
    let mut state = 0x0dee_9000_0000_0001_u64;
    let mut deepest = 0;
    for i in 0..3000 {
        let term = loop {
            let term = random_dag(&mut state, 9);
            if term.rebuild_cost() <= 20_000 {
                break term;
            }
        };
        let depth = walk_depth(&term);
        assert_eq!(term.depth(), depth, "term {i}: {term:?}");
        assert_eq!(Node::new(term).depth(), depth);
        deepest = deepest.max(depth);
    }
    assert!(deepest >= 6, "the terms were not all shallow: {deepest}");
    // A leaf is one level; universe levels do not count.
    assert_eq!(Expr::BVar(7).depth(), 1);
    assert_eq!(
        Expr::Sort(Level::succ(Level::succ(Level::zero()))).depth(),
        1
    );
    // A tower of shared applications is as deep as it is tall, however many
    // tree nodes it stands for.
    let big = tower(Expr::BVar(0), 60);
    assert_eq!(big.depth(), 61);
    assert_eq!(big.rebuild_cost(), REBUILD_COST_MAX);
    // A chain of binders and projections, one level each.
    let mut chain = Expr::Const(Name::str("c"), vec![]);
    for k in 0..1000u32 {
        chain = match k % 3 {
            0 => Expr::Proj(Name::str("S"), 0, Node::new(chain)),
            1 => Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Const(Name::str("c"), vec![])),
                Node::new(chain),
            ),
            _ => Expr::Let(
                Name::str("x"),
                Node::new(Expr::Const(Name::str("c"), vec![])),
                Node::new(chain),
                Node::new(Expr::BVar(0)),
            ),
        };
    }
    assert_eq!(chain.depth(), 1001);
}

#[cfg(feature = "term-depth")]
#[test]
fn building_a_node_over_the_depth_limit_spends_the_budget() {
    use crate::fuel;
    fuel::set_budget(Some(1_000_000));
    fuel::set_term_depth_limit(Some(40));
    // Forty levels: within the limit.
    let mut term = Expr::Const(Name::str("c"), vec![]);
    for _ in 0..39 {
        term = Expr::App(
            Node::new(Expr::Const(Name::str("g"), vec![])),
            Node::new(term),
        );
    }
    assert_eq!(term.depth(), 40);
    let node = Node::new(term);
    assert!(!fuel::term_depth_limit_hit() && !fuel::is_exhausted());
    // One more level, and the node that holds it is over the limit.
    let over = Expr::App(Node::new(Expr::Const(Name::str("g"), vec![])), node);
    assert_eq!(over.depth(), 41);
    assert!(
        !fuel::term_depth_limit_hit(),
        "a term held by value is not a node yet"
    );
    let _held = Node::new(over);
    assert!(fuel::term_depth_limit_hit() && fuel::is_exhausted());
    fuel::set_budget(None);
    fuel::set_term_depth_limit(None);
}

#[test]
fn is_shared_counts_the_owners_of_an_allocation() {
    let node = Node::new(Expr::BVar(0));
    assert!(!node.is_shared(), "one owner");
    let second = node.clone();
    assert!(node.is_shared() && second.is_shared(), "two owners");
    drop(second);
    assert!(!node.is_shared(), "one owner again");
    // Two parents in one term.
    let term = Expr::App(node.clone(), node.clone());
    let Expr::App(f, a) = &term else {
        panic!("an application");
    };
    assert!(f.is_shared() && a.is_shared());
    // An equal subterm in its own allocation is not the same allocation.
    let other = Node::new(Expr::BVar(0));
    assert!(!other.is_shared());
    assert!(!Node::ptr_eq(&other, &node));
}
