//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use oxilean_kernel::{
    BinderInfo as HmBI, Declaration as HmDecl, Expr as HmExpr, Level as HmLevel, Name as HmName,
};

use super::types::{
    AssocMap, BiMap, FreqMap, IndexedMap, IntervalMap, LruMap, MultiMap, PersistentMap, StackMap,
};

/// Build HashMap type in the environment.
pub fn build_hashmap_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let hashmap_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(type2.clone()),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap"),
        univ_params: vec![],
        ty: hashmap_ty,
    })
    .map_err(|e| e.to_string())?;
    let empty_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::BVar(0)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap.empty"),
        univ_params: vec![],
        ty: empty_ty,
    })
    .map_err(|e| e.to_string())?;
    let insert_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("k"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("v"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("m"),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                                Box::new(Expr::BVar(4)),
                            )),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap.insert"),
        univ_params: vec![],
        ty: insert_ty,
    })
    .map_err(|e| e.to_string())?;
    let lookup_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("k"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("m"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Option"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap.lookup"),
        univ_params: vec![],
        ty: lookup_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// A map from `Name` to `Expr`, backed by an association list.
pub type NameExprMap = AssocMap<Name, Expr>;
/// A map from `String` to arbitrary values, backed by an association list.
pub type StringMap<V> = AssocMap<String, V>;
/// Build a `NameExprMap` from an iterator of `(Name, Expr)` pairs.
pub fn name_expr_map_from_iter(iter: impl IntoIterator<Item = (Name, Expr)>) -> NameExprMap {
    let mut map = NameExprMap::new();
    for (k, v) in iter {
        map.insert(k, v);
    }
    map
}
/// Build `HashMap.delete : {K V : Type} → K → HashMap K V → HashMap K V`.
pub fn build_hashmap_delete(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let delete_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("k"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("m"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap.delete"),
        univ_params: vec![],
        ty: delete_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `HashMap.size : {K V : Type} → HashMap K V → Nat`.
pub fn build_hashmap_size(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let size_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("K"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("V"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("m"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("HashMap"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("HashMap.size"),
        univ_params: vec![],
        ty: size_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build all HashMap operations into the environment.
pub fn build_hashmap_extended(env: &mut Environment) -> Result<(), String> {
    build_hashmap_env(env)?;
    build_hashmap_delete(env)?;
    build_hashmap_size(env)?;
    Ok(())
}
/// Count entries in an AssocMap matching a predicate.
pub fn count_matching<K: PartialEq + Clone, V: Clone>(
    map: &AssocMap<K, V>,
    pred: impl Fn(&K, &V) -> bool,
) -> usize {
    map.iter().filter(|(k, v)| pred(k, v)).count()
}
/// Convert a Vec of pairs into an AssocMap.
pub fn assoc_map_from_vec<K: PartialEq + Clone, V: Clone>(pairs: Vec<(K, V)>) -> AssocMap<K, V> {
    let mut map = AssocMap::new();
    for (k, v) in pairs {
        map.insert(k, v);
    }
    map
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        let option_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(type2),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Option"),
            univ_params: vec![],
            ty: option_ty,
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_hashmap_env() {
        let mut env = setup_env();
        assert!(build_hashmap_env(&mut env).is_ok());
        assert!(env.get(&Name::str("HashMap")).is_some());
        assert!(env.get(&Name::str("HashMap.empty")).is_some());
    }
    #[test]
    fn test_hashmap_insert() {
        let mut env = setup_env();
        build_hashmap_env(&mut env).expect("build_hashmap_env should succeed");
        let decl = env
            .get(&Name::str("HashMap.insert"))
            .expect("declaration 'HashMap.insert' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_hashmap_lookup() {
        let mut env = setup_env();
        build_hashmap_env(&mut env).expect("build_hashmap_env should succeed");
        let decl = env
            .get(&Name::str("HashMap.lookup"))
            .expect("declaration 'HashMap.lookup' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_assoc_map_insert_and_get() {
        let mut map: AssocMap<String, u32> = AssocMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        assert_eq!(map.get(&"a".to_string()), Some(&1));
        assert_eq!(map.get(&"b".to_string()), Some(&2));
        assert_eq!(map.get(&"c".to_string()), None);
    }
    #[test]
    fn test_assoc_map_overwrite() {
        let mut map: AssocMap<String, u32> = AssocMap::new();
        map.insert("x".to_string(), 10);
        map.insert("x".to_string(), 20);
        assert_eq!(map.get(&"x".to_string()), Some(&20));
        assert_eq!(map.len(), 1);
    }
    #[test]
    fn test_assoc_map_remove() {
        let mut map: AssocMap<String, u32> = AssocMap::new();
        map.insert("a".to_string(), 1);
        let removed = map.remove(&"a".to_string());
        assert_eq!(removed, Some(1));
        assert!(map.is_empty());
    }
    #[test]
    fn test_assoc_map_merge() {
        let mut m1: AssocMap<String, u32> = AssocMap::new();
        m1.insert("a".to_string(), 1);
        let mut m2: AssocMap<String, u32> = AssocMap::new();
        m2.insert("b".to_string(), 2);
        m2.insert("a".to_string(), 99);
        m1.merge(&m2);
        assert_eq!(m1.get(&"a".to_string()), Some(&99));
        assert_eq!(m1.get(&"b".to_string()), Some(&2));
    }
    #[test]
    fn test_assoc_map_retain() {
        let mut map: AssocMap<String, u32> = AssocMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        map.retain(|_, v| *v > 1);
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key(&"a".to_string()));
    }
    #[test]
    fn test_persistent_map_insert_get() {
        let m0: PersistentMap<i32, &str> = PersistentMap::empty();
        let m1 = m0.insert(1, "one");
        let m2 = m1.insert(2, "two");
        assert_eq!(m1.get(&1), Some(&"one"));
        assert_eq!(m1.get(&2), None);
        assert_eq!(m2.get(&1), Some(&"one"));
        assert_eq!(m2.get(&2), Some(&"two"));
    }
    #[test]
    fn test_persistent_map_sorted_output() {
        let m: PersistentMap<i32, i32> = PersistentMap::empty()
            .insert(3, 30)
            .insert(1, 10)
            .insert(2, 20);
        let sorted = m.to_sorted_vec();
        assert_eq!(sorted, vec![(1, 10), (2, 20), (3, 30)]);
    }
    #[test]
    fn test_persistent_map_overwrite() {
        let m = PersistentMap::empty().insert(1, "old").insert(1, "new");
        assert_eq!(m.get(&1), Some(&"new"));
        assert_eq!(m.len(), 1);
    }
    #[test]
    fn test_multi_map_insert_get() {
        let mut mm: MultiMap<String, u32> = MultiMap::new();
        mm.insert("k".to_string(), 1);
        mm.insert("k".to_string(), 2);
        mm.insert("k".to_string(), 3);
        assert_eq!(mm.get(&"k".to_string()), &[1, 2, 3]);
        assert_eq!(mm.total_count(), 3);
        assert_eq!(mm.key_count(), 1);
    }
    #[test]
    fn test_multi_map_remove() {
        let mut mm: MultiMap<String, u32> = MultiMap::new();
        mm.insert("a".to_string(), 10);
        let vals = mm.remove(&"a".to_string());
        assert_eq!(vals, vec![10]);
        assert!(mm.is_empty());
    }
    #[test]
    fn test_name_expr_map() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let map = name_expr_map_from_iter(vec![
            (Name::str("x"), nat.clone()),
            (Name::str("y"), nat.clone()),
        ]);
        assert_eq!(map.get(&Name::str("x")), Some(&nat));
        assert_eq!(map.len(), 2);
    }
    #[test]
    fn test_build_hashmap_delete() {
        let mut env = setup_env();
        build_hashmap_env(&mut env).expect("build_hashmap_env should succeed");
        assert!(build_hashmap_delete(&mut env).is_ok());
        assert!(env.get(&Name::str("HashMap.delete")).is_some());
    }
    #[test]
    fn test_build_hashmap_extended() {
        let mut env = setup_env();
        assert!(build_hashmap_extended(&mut env).is_ok());
        assert!(env.get(&Name::str("HashMap.delete")).is_some());
        assert!(env.get(&Name::str("HashMap.size")).is_some());
    }
    #[test]
    fn test_assoc_map_keys_values() {
        let mut map: AssocMap<i32, i32> = AssocMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let keys: Vec<i32> = map.keys().copied().collect();
        let vals: Vec<i32> = map.values().copied().collect();
        assert_eq!(keys, vec![1, 2]);
        assert_eq!(vals, vec![10, 20]);
    }
    #[test]
    fn test_bimap_insert_lookup() {
        let mut m: BiMap<String, u32> = BiMap::new();
        m.insert("a".to_string(), 1);
        m.insert("b".to_string(), 2);
        assert_eq!(m.get_by_key(&"a".to_string()), Some(&1));
        assert_eq!(m.get_by_val(&2), Some(&"b".to_string()));
    }
    #[test]
    fn test_bimap_evicts_on_collision() {
        let mut m: BiMap<String, u32> = BiMap::new();
        m.insert("a".to_string(), 1);
        m.insert("b".to_string(), 1);
        assert!(m.get_by_key(&"a".to_string()).is_none());
    }
    #[test]
    fn test_lrumap_eviction() {
        let mut m: LruMap<u32, u32> = LruMap::new(2);
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);
        assert!(m.peek(&1).is_none());
        assert_eq!(m.peek(&3), Some(&30));
    }
    #[test]
    fn test_stackmap_shadowing() {
        let mut m: StackMap<String, u32> = StackMap::new();
        m.insert("x".to_string(), 1);
        m.push_scope();
        m.insert("x".to_string(), 2);
        assert_eq!(m.get(&"x".to_string()), Some(&2));
        m.pop_scope();
        assert_eq!(m.get(&"x".to_string()), Some(&1));
    }
    #[test]
    fn test_freq_map_record() {
        let mut f: FreqMap<String> = FreqMap::new();
        f.record("a".to_string());
        f.record("a".to_string());
        f.record("b".to_string());
        assert_eq!(f.count(&"a".to_string()), 2);
        assert_eq!(f.count(&"b".to_string()), 1);
        assert_eq!(f.total_count(), 3);
    }
    #[test]
    fn test_freq_map_most_common() {
        let mut f: FreqMap<u32> = FreqMap::new();
        f.record(1);
        f.record(2);
        f.record(2);
        f.record(2);
        let (k, c) = f.most_common().expect("most_common should succeed");
        assert_eq!(*k, 2);
        assert_eq!(c, 3);
    }
    #[test]
    fn test_interval_map_query() {
        let mut m: IntervalMap<u32, &str> = IntervalMap::new();
        m.insert(0, 10, "a");
        m.insert(5, 15, "b");
        m.insert(20, 30, "c");
        let results = m.query(&7);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&&"a"));
        assert!(results.contains(&&"b"));
    }
    #[test]
    fn test_interval_map_no_match() {
        let mut m: IntervalMap<u32, &str> = IntervalMap::new();
        m.insert(0, 5, "a");
        assert!(m.query(&10).is_empty());
    }
    #[test]
    fn test_indexed_map_insert_get() {
        let mut m: IndexedMap<String> = IndexedMap::new();
        m.insert(0, "hello".to_string());
        m.insert(5, "world".to_string());
        assert_eq!(m.get(0), Some(&"hello".to_string()));
        assert_eq!(m.get(5), Some(&"world".to_string()));
        assert!(m.get(3).is_none());
        assert_eq!(m.len(), 2);
    }
    #[test]
    fn test_indexed_map_remove() {
        let mut m: IndexedMap<u32> = IndexedMap::new();
        m.insert(0, 42);
        let old = m.remove(0);
        assert_eq!(old, Some(42));
        assert!(m.is_empty());
    }
    #[test]
    fn test_indexed_map_iter() {
        let mut m: IndexedMap<u32> = IndexedMap::new();
        m.insert(0, 10);
        m.insert(2, 30);
        let pairs: Vec<_> = m.iter().collect();
        assert_eq!(pairs.len(), 2);
    }
    #[test]
    fn test_count_matching() {
        let mut m: AssocMap<u32, u32> = AssocMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 5);
        assert_eq!(count_matching(&m, |_, v| *v >= 10), 2);
    }
    #[test]
    fn test_assoc_map_from_vec() {
        let m = assoc_map_from_vec(vec![("a", 1u32), ("b", 2), ("a", 99)]);
        assert_eq!(m.get(&"a"), Some(&99));
        assert_eq!(m.len(), 2);
    }
}
/// Prop sort.
#[allow(dead_code)]
pub fn hm_prop() -> HmExpr {
    HmExpr::Sort(HmLevel::zero())
}
/// Type₁ sort.
#[allow(dead_code)]
pub fn hm_type1() -> HmExpr {
    HmExpr::Sort(HmLevel::succ(HmLevel::zero()))
}
/// Type₂ sort.
#[allow(dead_code)]
pub fn hm_type2() -> HmExpr {
    HmExpr::Sort(HmLevel::succ(HmLevel::succ(HmLevel::zero())))
}
/// Named constant.
#[allow(dead_code)]
pub fn hm_cst(s: &str) -> HmExpr {
    HmExpr::Const(HmName::str(s), vec![])
}
/// Bound variable.
#[allow(dead_code)]
pub fn hm_bvar(n: u32) -> HmExpr {
    HmExpr::BVar(n)
}
/// Application `f a`.
#[allow(dead_code)]
pub fn hm_app(f: HmExpr, a: HmExpr) -> HmExpr {
    HmExpr::App(Box::new(f), Box::new(a))
}
/// Application `f a b`.
#[allow(dead_code)]
pub fn hm_app2(f: HmExpr, a: HmExpr, b: HmExpr) -> HmExpr {
    hm_app(hm_app(f, a), b)
}
/// Application `f a b c`.
#[allow(dead_code)]
pub fn hm_app3(f: HmExpr, a: HmExpr, b: HmExpr, c: HmExpr) -> HmExpr {
    hm_app(hm_app2(f, a, b), c)
}
/// Pi binder.
#[allow(dead_code)]
pub fn hm_pi(bi: HmBI, name: &str, dom: HmExpr, body: HmExpr) -> HmExpr {
    HmExpr::Pi(bi, HmName::str(name), Box::new(dom), Box::new(body))
}
/// Non-dependent arrow `A → B`.
#[allow(dead_code)]
pub fn hm_arrow(a: HmExpr, b: HmExpr) -> HmExpr {
    hm_pi(HmBI::Default, "_", a, b)
}
/// `Eq ty a b`.
#[allow(dead_code)]
pub fn hm_eq(ty: HmExpr, a: HmExpr, b: HmExpr) -> HmExpr {
    hm_app3(hm_cst("Eq"), ty, a, b)
}
/// `HashMap K V`.
#[allow(dead_code)]
pub fn hm_ty(k: HmExpr, v: HmExpr) -> HmExpr {
    hm_app2(hm_cst("HashMap"), k, v)
}
/// `Option V`.
#[allow(dead_code)]
pub fn hm_option(v: HmExpr) -> HmExpr {
    hm_app(hm_cst("Option"), v)
}
/// `Nat`.
#[allow(dead_code)]
pub fn hm_nat() -> HmExpr {
    hm_cst("Nat")
}
/// `Bool`.
#[allow(dead_code)]
pub fn hm_bool() -> HmExpr {
    hm_cst("Bool")
}
/// `List A`.
#[allow(dead_code)]
pub fn hm_list(a: HmExpr) -> HmExpr {
    hm_app(hm_cst("List"), a)
}
/// `∀ {K V : Type}, HashMap K V → ...`  quantifier over a single map.
#[allow(dead_code)]
pub fn hm_ext_forall_map(body: HmExpr) -> HmExpr {
    hm_pi(
        HmBI::Implicit,
        "K",
        hm_type1(),
        hm_pi(
            HmBI::Implicit,
            "V",
            hm_type1(),
            hm_pi(HmBI::Default, "m", hm_ty(hm_bvar(1), hm_bvar(0)), body),
        ),
    )
}
/// `∀ {K V : Type}, HashMap K V → HashMap K V → ...`
#[allow(dead_code)]
pub fn hm_ext_forall_two_maps(body: HmExpr) -> HmExpr {
    hm_pi(
        HmBI::Implicit,
        "K",
        hm_type1(),
        hm_pi(
            HmBI::Implicit,
            "V",
            hm_type1(),
            hm_pi(
                HmBI::Default,
                "m1",
                hm_ty(hm_bvar(1), hm_bvar(0)),
                hm_pi(HmBI::Default, "m2", hm_ty(hm_bvar(2), hm_bvar(1)), body),
            ),
        ),
    )
}
/// Equality of two `HashMap K V` values.
#[allow(dead_code)]
pub fn hm_ext_map_eq(k: HmExpr, v: HmExpr, a: HmExpr, b: HmExpr) -> HmExpr {
    hm_eq(hm_ty(k, v), a, b)
}
/// Build `HashMap.get m k = Option.some v` equality.
#[allow(dead_code)]
pub fn hm_ext_get_eq_some(m: HmExpr, k: HmExpr, v: HmExpr, v_ty: HmExpr) -> HmExpr {
    hm_eq(
        hm_option(v_ty),
        hm_app2(hm_cst("HashMap.get"), m, k),
        hm_app(hm_cst("Option.some"), v),
    )
}
/// Build `HashMap.get m k = Option.none`.
#[allow(dead_code)]
pub fn hm_ext_get_eq_none(m: HmExpr, k: HmExpr, v_ty: HmExpr) -> HmExpr {
    hm_eq(
        hm_option(v_ty),
        hm_app2(hm_cst("HashMap.get"), m, k),
        hm_cst("Option.none"),
    )
}
/// Standard `∀ {K V} (m : HashMap K V) (k : K) (v : V)` prefix.
#[allow(dead_code)]
pub fn hm_ext_forall_map_k_v(body: HmExpr) -> HmExpr {
    hm_pi(
        HmBI::Implicit,
        "K",
        hm_type1(),
        hm_pi(
            HmBI::Implicit,
            "V",
            hm_type1(),
            hm_pi(
                HmBI::Default,
                "m",
                hm_ty(hm_bvar(1), hm_bvar(0)),
                hm_pi(
                    HmBI::Default,
                    "k",
                    hm_bvar(2),
                    hm_pi(HmBI::Default, "v", hm_bvar(2), body),
                ),
            ),
        ),
    )
}
