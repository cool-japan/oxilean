//! Tests for the structural hash that `Name`, `Level` and every `Node` store
//! (see `crate::structural_hash`).
//!
//! What must hold, and is pinned here:
//!
//! * equal values have equal hashes — the only property correctness needs,
//!   since the hash is written by `Hash` and used as a negative test in `==`;
//! * every field `==` compares takes part, so the hash separates what the
//!   derived `Hash` separated (no systematic collisions);
//! * a `Node` caches exactly what its term computes, and hashes as that term
//!   (the `Borrow<Expr>` contract);
//! * hashing a term never walks it — a term with nested sharing, whose tree
//!   has 2^64 nodes, is hashed and used as a map key at once;
//! * `Name` and `Level` still write the same bytes to a `Hasher` and print
//!   the same `Debug` text as when they wrapped their kind directly.

use super::types::{BinderInfo, Expr, FVarId, Literal, Node};
use crate::bignat::BigNat;
use crate::{Level, LevelMVarId, Name};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

fn std_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn cnst(name: Name) -> Expr {
    Expr::Const(name, vec![])
}

/// Leaves that differ from one another in every way a leaf can: constructor,
/// index, id, name spelling (hierarchical / flat / numeric component), the
/// number, order and shape of universe levels, and literal kind and value.
fn leaves() -> Vec<Expr> {
    let u = Level::param(Name::str("u"));
    let v = Level::param(Name::str("v"));
    let one = Level::succ(Level::zero());
    vec![
        Expr::BVar(0),
        Expr::BVar(1),
        Expr::FVar(FVarId(0)),
        Expr::FVar(FVarId(1)),
        Expr::FVar(FVarId(1 << 62)),
        Expr::Sort(Level::zero()),
        Expr::Sort(one.clone()),
        Expr::Sort(u.clone()),
        Expr::Sort(Level::succ(u.clone())),
        Expr::Sort(Level::max(u.clone(), v.clone())),
        Expr::Sort(Level::max(v.clone(), u.clone())),
        Expr::Sort(Level::imax(u.clone(), v.clone())),
        Expr::Sort(Level::mvar(LevelMVarId(0))),
        Expr::Sort(Level::mvar(LevelMVarId(1))),
        cnst(Name::str("Nat")),
        cnst(Name::str("Nat").append_str("add")),
        cnst(Name::str("Nat.add")),
        cnst(Name::str("add").append_str("Nat")),
        cnst(Name::str("x").append_num(1)),
        cnst(Name::str("x").append_str("1")),
        cnst(Name::anonymous()),
        cnst(Name::anonymous().append_num(0)),
        Expr::Const(Name::str("List"), vec![Level::zero()]),
        Expr::Const(Name::str("List"), vec![one.clone()]),
        Expr::Const(Name::str("List"), vec![u.clone()]),
        Expr::Const(Name::str("P"), vec![u.clone(), v.clone()]),
        Expr::Const(Name::str("P"), vec![v.clone(), u.clone()]),
        Expr::Const(Name::str("P"), vec![u.clone(), u.clone()]),
        Expr::Const(Name::str("P"), vec![u]),
        Expr::Lit(Literal::nat(0)),
        Expr::Lit(Literal::nat(1)),
        Expr::Lit(Literal::Nat(BigNat::from(u64::MAX))),
        // 2^64: two limbs, the low one zero.
        Expr::Lit(Literal::Nat(
            BigNat::from(u64::MAX).add(&BigNat::from(1u64)),
        )),
        Expr::Lit(Literal::string("")),
        Expr::Lit(Literal::string("a")),
        Expr::Lit(Literal::string("a\0")),
        Expr::Lit(Literal::string("0")),
        Expr::Lit(Literal::string("abcdefgh")),
        Expr::Lit(Literal::string("abcdefghi")),
    ]
}

/// A family of pairwise-distinct terms covering every constructor, built
/// from scratch on each call (no allocation is shared between two calls).
fn family() -> Vec<Expr> {
    let leaves = leaves();
    let small: Vec<Expr> = leaves.iter().step_by(6).cloned().collect();
    let node = |e: &Expr| Node::new(e.clone());
    let mut terms = leaves.clone();
    for f in &leaves {
        for a in &leaves {
            terms.push(Expr::App(node(f), node(a)));
        }
    }
    let infos = [
        BinderInfo::Default,
        BinderInfo::Implicit,
        BinderInfo::StrictImplicit,
        BinderInfo::InstImplicit,
    ];
    for bi in infos {
        for binder in ["x", "y"] {
            for ty in &small {
                for body in &small {
                    terms.push(Expr::Lam(bi, Name::str(binder), node(ty), node(body)));
                    terms.push(Expr::Pi(bi, Name::str(binder), node(ty), node(body)));
                }
            }
        }
    }
    for binder in ["x", "y"] {
        for ty in &small {
            for val in &small {
                for body in &small {
                    terms.push(Expr::Let(
                        Name::str(binder),
                        node(ty),
                        node(val),
                        node(body),
                    ));
                }
            }
        }
    }
    for (structure, index) in [("S", 0u32), ("S", 1), ("T", 0)] {
        for inner in &leaves {
            terms.push(Expr::Proj(Name::str(structure), index, node(inner)));
        }
    }
    // One more level: applications and binders over composite terms.
    let composite: Vec<Expr> = terms
        .iter()
        .skip(leaves.len())
        .step_by(97)
        .cloned()
        .collect();
    for f in &composite {
        for a in &composite {
            terms.push(Expr::App(node(f), node(a)));
        }
        terms.push(Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            node(f),
            node(f),
        ));
    }
    terms
}

#[test]
fn equal_terms_hash_equal_and_distinct_terms_do_not_collide() {
    let first = family();
    let second = family();
    assert_eq!(first.len(), second.len());
    assert!(first.len() > 2000, "family has {} terms", first.len());
    let hashes: Vec<u64> = first.iter().map(Expr::structural_hash).collect();
    // Equal terms, built separately, have equal hashes — through the
    // structural hash and through `Hash`.
    for (i, (a, b)) in first.iter().zip(second.iter()).enumerate() {
        assert_eq!(a, b, "term {i} differs between two builds of the family");
        assert_eq!(hashes[i], b.structural_hash(), "term {i}: {a}");
        assert_eq!(std_hash(a), std_hash(b), "term {i}: {a}");
    }
    // Over the whole family, `==` and "same hash" coincide: no two distinct
    // terms collide, and no two equal terms are told apart.
    for i in 0..first.len() {
        for j in (i + 1)..first.len() {
            let equal = first[i] == second[j];
            assert_eq!(
                equal,
                hashes[i] == hashes[j],
                "terms {i} and {j}: equal = {equal}, hashes {:#x} / {:#x}",
                hashes[i],
                hashes[j]
            );
            assert!(!equal, "the family repeats a term at {i} and {j}");
        }
    }
}

#[test]
fn a_node_caches_what_its_term_computes_and_hashes_as_that_term() {
    let mut by_node: HashMap<Node, usize> = HashMap::new();
    let terms = family();
    for (i, term) in terms.iter().enumerate() {
        let node = Node::new(term.clone());
        assert_eq!(node.structural_hash(), term.structural_hash(), "term {i}");
        assert_eq!(node.range(), term.range(), "term {i}");
        assert_eq!(node.rebuild_cost(), term.rebuild_cost(), "term {i}");
        // `Borrow<Expr>`: a node and its term write the same hash.
        assert_eq!(std_hash(&node), std_hash(term), "term {i}");
        by_node.insert(node, i);
    }
    assert_eq!(by_node.len(), terms.len());
    // … so a map keyed by nodes can be asked with a bare term.
    for (i, term) in family().iter().enumerate() {
        assert_eq!(by_node.get(term), Some(&i), "term {i}: {term}");
    }
}

#[test]
fn node_equality_is_structural_whatever_the_allocation() {
    let term = Expr::App(
        Node::new(cnst(Name::str("f"))),
        Node::new(Expr::Lit(Literal::nat(3))),
    );
    let one = Node::new(term.clone());
    let same_allocation = one.clone();
    let other_allocation = Node::new(term.clone());
    assert!(Node::ptr_eq(&one, &same_allocation));
    assert_eq!(one.as_ptr(), same_allocation.as_ptr());
    assert!(!Node::ptr_eq(&one, &other_allocation));
    assert_ne!(one.as_ptr(), other_allocation.as_ptr());
    // Equal either way.
    assert_eq!(one, same_allocation);
    assert_eq!(one, other_allocation);
    // The address is that of the term the node dereferences to.
    assert!(std::ptr::eq(one.as_ptr(), &*one));
    let different = Node::new(Expr::App(
        Node::new(cnst(Name::str("f"))),
        Node::new(Expr::Lit(Literal::nat(4))),
    ));
    assert_ne!(one, different);
    assert_ne!(one.structural_hash(), different.structural_hash());
}

#[test]
fn a_term_with_nested_sharing_is_hashed_without_walking_it() {
    // x₀ = c, xₖ₊₁ = xₖ xₖ with both children the SAME node: 64 allocations,
    // a tree of 2^64 applications. Walking it — as `Hash` did through 0.1.3 —
    // would not finish; its hash is one read.
    let mut term = cnst(Name::str("c"));
    let mut hashes = vec![term.structural_hash()];
    for _ in 0..64 {
        let shared = Node::new(term);
        term = Expr::App(shared.clone(), shared);
        hashes.push(term.structural_hash());
    }
    let distinct: std::collections::HashSet<u64> = hashes.iter().copied().collect();
    assert_eq!(
        distinct.len(),
        hashes.len(),
        "two levels of the tower collide"
    );
    // The rebuild cost of the tree saturates; the hash does not care.
    assert_eq!(term.rebuild_cost(), crate::expr::REBUILD_COST_MAX);

    // As a map key: hashed on insertion and on lookup, compared by the
    // shared children's addresses.
    let mut cache: HashMap<Expr, u32> = HashMap::new();
    cache.insert(term.clone(), 7);
    assert_eq!(cache.get(&term), Some(&7));
    let mut pairs: std::collections::HashSet<(Expr, Expr)> = std::collections::HashSet::new();
    pairs.insert((term.clone(), cnst(Name::str("c"))));
    assert!(pairs.contains(&(term.clone(), cnst(Name::str("c")))));
    assert!(!pairs.contains(&(cnst(Name::str("c")), term.clone())));
    // A different tower of the same height is told apart by its hash alone.
    let mut other = cnst(Name::str("d"));
    for _ in 0..64 {
        let shared = Node::new(other);
        other = Expr::App(shared.clone(), shared);
    }
    assert_ne!(term.structural_hash(), other.structural_hash());
    assert_eq!(cache.get(&other), None);
    assert_ne!(Node::new(term), Node::new(other));
}

#[test]
fn name_and_level_hashes_follow_their_structure() {
    // Equal names built separately share a hash; a name differs from its
    // prefix, from the flat spelling, and a string from a numeric component.
    let nat_add = || Name::str("Nat").append_str("add");
    assert_eq!(nat_add(), nat_add());
    assert_eq!(nat_add().structural_hash(), nat_add().structural_hash());
    let names = [
        Name::anonymous(),
        Name::str("Nat"),
        nat_add(),
        Name::str("Nat.add"),
        Name::str("add").append_str("Nat"),
        Name::str("Nat").append_num(1),
        Name::str("Nat").append_str("1"),
        Name::anonymous().append_num(1),
        Name::str(""),
        Name::str("").append_str(""),
    ];
    for (i, a) in names.iter().enumerate() {
        for (j, b) in names.iter().enumerate() {
            assert_eq!(a == b, i == j, "names {i} and {j}: {a:?} / {b:?}");
            assert_eq!(
                a.structural_hash() == b.structural_hash(),
                i == j,
                "names {i} and {j}: {a:?} / {b:?}"
            );
        }
    }
    assert_eq!(Name::default(), Name::anonymous());
    assert_eq!(
        Name::default().structural_hash(),
        Name::anonymous().structural_hash()
    );

    let u = || Level::param(Name::str("u"));
    let v = || Level::param(Name::str("v"));
    let levels = [
        Level::zero(),
        Level::succ(Level::zero()),
        Level::succ(Level::succ(Level::zero())),
        u(),
        v(),
        Level::succ(u()),
        Level::max(u(), v()),
        Level::max(v(), u()),
        Level::imax(u(), v()),
        Level::imax(v(), u()),
        Level::max(u(), u()),
        Level::mvar(LevelMVarId(0)),
        Level::mvar(LevelMVarId(1)),
        Level::param(Name::str("0")),
    ];
    for (i, a) in levels.iter().enumerate() {
        for (j, b) in levels.iter().enumerate() {
            assert_eq!(a == b, i == j, "levels {i} and {j}: {a:?} / {b:?}");
            assert_eq!(
                a.structural_hash() == b.structural_hash(),
                i == j,
                "levels {i} and {j}: {a:?} / {b:?}"
            );
        }
    }
    assert_eq!(
        Level::max(u(), v()).structural_hash(),
        Level::max(u(), v()).structural_hash()
    );
}

/// `Name` as it was through 0.1.3: a newtype over the component kind, every
/// trait derived. Used to pin that the real `Name` still writes the same
/// bytes to a `Hasher` and the same `Debug` text.
#[derive(Hash, Debug)]
struct OldName(Rc<OldNameKind>);
#[derive(Hash, Debug)]
#[allow(dead_code)]
enum OldNameKind {
    Anonymous,
    Str(OldName, String),
    Num(OldName, u64),
}

/// `Level` as it was through 0.1.3 (see [`OldName`]).
#[derive(Hash, Debug)]
struct OldLevel(Rc<OldLevelKind>);
#[derive(Hash, Debug)]
#[allow(dead_code)]
enum OldLevelKind {
    Zero,
    Succ(OldLevel),
    Max(OldLevel, OldLevel),
    IMax(OldLevel, OldLevel),
    Param(OldName),
    MVar(LevelMVarId),
}

/// The `Debug` text of an [`OldName`] / [`OldLevel`], with the two type
/// names spelled as the real types spell them.
fn spelled_as_before(old_debug: &str) -> String {
    old_debug
        .replace("OldName", "Name")
        .replace("OldLevel", "Level")
}

fn old_name(name: &Name) -> OldName {
    OldName(Rc::new(match name.view() {
        crate::NameView::Anonymous => OldNameKind::Anonymous,
        crate::NameView::Str(parent, s) => OldNameKind::Str(old_name(parent), s.to_string()),
        crate::NameView::Num(parent, n) => OldNameKind::Num(old_name(parent), n),
    }))
}

fn old_level(level: &Level) -> OldLevel {
    OldLevel(Rc::new(match level.view() {
        crate::LevelView::Zero => OldLevelKind::Zero,
        crate::LevelView::Succ(l) => OldLevelKind::Succ(old_level(l)),
        crate::LevelView::Max(a, b) => OldLevelKind::Max(old_level(a), old_level(b)),
        crate::LevelView::IMax(a, b) => OldLevelKind::IMax(old_level(a), old_level(b)),
        crate::LevelView::Param(name) => OldLevelKind::Param(old_name(name)),
        crate::LevelView::MVar(id) => OldLevelKind::MVar(id),
    }))
}

#[test]
fn name_and_level_write_the_bytes_and_the_debug_text_they_always_did() {
    let names = [
        Name::anonymous(),
        Name::str("Nat"),
        Name::str("Nat").append_str("add").append_num(3),
        Name::str("_private").append_num(0).append_str("x"),
    ];
    for name in &names {
        let old = old_name(name);
        assert_eq!(std_hash(name), std_hash(&old), "{name:?}");
        assert_eq!(format!("{name:?}"), spelled_as_before(&format!("{old:?}")));
        assert_eq!(
            format!("{name:#?}"),
            spelled_as_before(&format!("{old:#?}"))
        );
    }
    assert_eq!(
        format!("{:?}", Name::str("Nat").append_num(1)),
        "Name(Num(Name(Str(Name(Anonymous), \"Nat\")), 1))"
    );

    let u = Level::param(Name::str("u"));
    let levels = [
        Level::zero(),
        Level::succ(Level::succ(Level::zero())),
        Level::max(u.clone(), Level::imax(Level::zero(), u.clone())),
        Level::succ(Level::mvar(LevelMVarId(4))),
    ];
    for level in &levels {
        let old = old_level(level);
        assert_eq!(std_hash(level), std_hash(&old), "{level:?}");
        assert_eq!(format!("{level:?}"), spelled_as_before(&format!("{old:?}")));
        assert_eq!(
            format!("{level:#?}"),
            spelled_as_before(&format!("{old:#?}"))
        );
    }
    assert_eq!(
        format!("{:?}", Level::succ(Level::zero())),
        "Level(Succ(Level(Zero)))"
    );
    // The `Debug` text of a term is unchanged with them.
    assert_eq!(
        format!(
            "{:?}",
            Expr::App(
                Node::new(Expr::Const(Name::str("f"), vec![Level::zero()])),
                Node::new(Expr::BVar(0)),
            )
        ),
        "App(Const(Name(Str(Name(Anonymous), \"f\")), [Level(Zero)]), BVar(0))"
    );
}
