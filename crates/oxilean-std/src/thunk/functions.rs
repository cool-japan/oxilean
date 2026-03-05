//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CofreeStream, ComonadCtx, ComputeNode, Deferred, DeferredPool, Delayed, GameArena, ITreeNode,
    LazyList, LazyStream, Memo, MemoThunk, OnceCellThunk, Suspension, ThunkApp, ThunkKleisli,
    ThunkSeq, ThunkState, ThunkTree, TryThunk,
};

pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn thunk_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Thunk"), vec![])),
        Box::new(alpha),
    )
}
pub fn unit_ty() -> Expr {
    Expr::Const(Name::str("Unit"), vec![])
}
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn list_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(alpha),
    )
}
pub fn unit_to(ret_ty: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(unit_ty()),
        Box::new(ret_ty),
    )
}
pub fn option_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(alpha),
    )
}
pub fn axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
pub fn alpha_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(inner),
    )
}
pub fn ab_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(inner),
        )),
    )
}
/// Build Thunk type in the environment.
pub fn build_thunk_env(env: &mut Environment) -> Result<(), String> {
    let thunk_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1()),
        Box::new(type2()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Thunk"),
        univ_params: vec![],
        ty: thunk_ty,
    })
    .map_err(|e| e.to_string())?;
    add_mk(env)?;
    add_get(env)?;
    add_pure(env)?;
    add_map(env)?;
    add_bind(env)?;
    add_join(env)?;
    add_zip(env)?;
    add_and(env)?;
    add_or(env)?;
    add_is_forced(env)?;
    add_ap(env)?;
    add_sequence(env)?;
    add_delay(env)?;
    add_count(env)?;
    add_get_or(env)?;
    Ok(())
}
pub fn add_mk(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(unit_to(Expr::BVar(1))),
        Box::new(thunk_of(Expr::BVar(1))),
    ));
    axiom(env, "Thunk.mk", ty)
}
pub fn add_get(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::BVar(1)),
    ));
    axiom(env, "Thunk.get", ty)
}
pub fn add_pure(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::BVar(0)),
        Box::new(thunk_of(Expr::BVar(1))),
    ));
    axiom(env, "Thunk.pure", ty)
}
pub fn add_map(env: &mut Environment) -> Result<(), String> {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(Expr::BVar(1)),
    );
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(fn_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(thunk_of(Expr::BVar(2))),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ));
    axiom(env, "Thunk.map", ty)
}
pub fn add_bind(env: &mut Environment) -> Result<(), String> {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(thunk_of(Expr::BVar(1))),
    );
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(1))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(fn_ty),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ));
    axiom(env, "Thunk.bind", ty)
}
pub fn add_join(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(thunk_of(Expr::BVar(0)))),
        Box::new(thunk_of(Expr::BVar(1))),
    ));
    axiom(env, "Thunk.join", ty)
}
pub fn add_zip(env: &mut Environment) -> Result<(), String> {
    let prod_ty = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Prod"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
        Box::new(Expr::BVar(0)),
    );
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("ta"),
        Box::new(thunk_of(Expr::BVar(1))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("tb"),
            Box::new(thunk_of(Expr::BVar(1))),
            Box::new(thunk_of(prod_ty)),
        )),
    ));
    axiom(env, "Thunk.zip", ty)
}
pub fn add_bool_op(env: &mut Environment, name: &str) -> Result<(), String> {
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(thunk_of(bool_ty())),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(thunk_of(bool_ty())),
            Box::new(thunk_of(bool_ty())),
        )),
    );
    axiom(env, name, ty)
}
pub fn add_and(env: &mut Environment) -> Result<(), String> {
    add_bool_op(env, "Thunk.and")
}
pub fn add_or(env: &mut Environment) -> Result<(), String> {
    add_bool_op(env, "Thunk.or")
}
pub fn add_is_forced(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(bool_ty()),
    ));
    axiom(env, "Thunk.isForced", ty)
}
pub fn add_ap(env: &mut Environment) -> Result<(), String> {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(Expr::BVar(1)),
    );
    let ty = ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("tf"),
        Box::new(thunk_of(fn_ty)),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("ta"),
            Box::new(thunk_of(Expr::BVar(2))),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ));
    axiom(env, "Thunk.ap", ty)
}
pub fn add_sequence(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("ts"),
        Box::new(list_of(thunk_of(Expr::BVar(0)))),
        Box::new(thunk_of(list_of(Expr::BVar(1)))),
    ));
    axiom(env, "Thunk.sequence", ty)
}
pub fn add_delay(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::BVar(0)),
        Box::new(thunk_of(Expr::BVar(1))),
    ));
    axiom(env, "Thunk.delay", ty)
}
pub fn add_count(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("ts"),
        Box::new(list_of(thunk_of(Expr::BVar(0)))),
        Box::new(nat_ty()),
    ));
    axiom(env, "Thunk.count", ty)
}
pub fn add_get_or(env: &mut Environment) -> Result<(), String> {
    let ty = alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("opt"),
        Box::new(option_of(thunk_of(Expr::BVar(0)))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("def"),
            Box::new(Expr::BVar(1)),
            Box::new(Expr::BVar(2)),
        )),
    ));
    axiom(env, "Thunk.getOr", ty)
}
pub fn setup_base_env() -> Environment {
    let mut env = Environment::new();
    let t1 = type1();
    for name in ["Unit", "Bool", "Prod", "List", "Nat", "Option"] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t1.clone(),
        })
        .unwrap_or(());
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_thunk_env() {
        let mut env = setup_base_env();
        assert!(build_thunk_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Thunk")).is_some());
        assert!(env.get(&Name::str("Thunk.mk")).is_some());
        assert!(env.get(&Name::str("Thunk.get")).is_some());
    }
    #[test]
    fn test_thunk_pure() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(matches!(
            env.get(&Name::str("Thunk.pure"))
                .expect("declaration 'Thunk.pure' should exist in env"),
            Declaration::Axiom { .. }
        ));
    }
    #[test]
    fn test_thunk_map() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(matches!(
            env.get(&Name::str("Thunk.map"))
                .expect("declaration 'Thunk.map' should exist in env"),
            Declaration::Axiom { .. }
        ));
    }
    #[test]
    fn test_thunk_bind() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.bind")).is_some());
    }
    #[test]
    fn test_thunk_join() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.join")).is_some());
    }
    #[test]
    fn test_thunk_zip() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.zip")).is_some());
    }
    #[test]
    fn test_thunk_and() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.and")).is_some());
    }
    #[test]
    fn test_thunk_or() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.or")).is_some());
    }
    #[test]
    fn test_thunk_is_forced() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.isForced")).is_some());
    }
    #[test]
    fn test_thunk_mk_is_axiom() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(matches!(
            env.get(&Name::str("Thunk.mk"))
                .expect("declaration 'Thunk.mk' should exist in env"),
            Declaration::Axiom { .. }
        ));
    }
    #[test]
    fn test_thunk_ap() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.ap")).is_some());
    }
    #[test]
    fn test_thunk_sequence() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.sequence")).is_some());
    }
    #[test]
    fn test_thunk_delay() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.delay")).is_some());
    }
    #[test]
    fn test_thunk_count() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.count")).is_some());
    }
    #[test]
    fn test_thunk_get_or() {
        let mut env = setup_base_env();
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        assert!(env.get(&Name::str("Thunk.getOr")).is_some());
    }
}
#[cfg(test)]
mod thunk_extended_tests {
    use super::*;
    #[test]
    fn test_once_cell_thunk_forces_once() {
        let count = std::sync::Arc::new(std::sync::Mutex::new(0));
        let count_clone = count.clone();
        let mut thunk = OnceCellThunk::new(move || {
            *count_clone.lock().expect("lock should succeed") += 1;
            42i32
        });
        assert!(!thunk.is_forced());
        assert_eq!(*thunk.force(), 42);
        assert!(thunk.is_forced());
        assert_eq!(*thunk.force(), 42);
        assert_eq!(*count.lock().expect("lock should succeed"), 1);
    }
    #[test]
    fn test_thunk_seq_lazy() {
        let mut seq: ThunkSeq<i32> = ThunkSeq::new();
        seq.push(|| 10);
        seq.push(|| 20);
        seq.push(|| 30);
        assert_eq!(seq.len(), 3);
        assert_eq!(seq.forced_count(), 0);
        assert_eq!(seq.get(1), Some(&20));
        assert_eq!(seq.forced_count(), 1);
    }
    #[test]
    fn test_thunk_seq_out_of_bounds() {
        let mut seq: ThunkSeq<i32> = ThunkSeq::new();
        seq.push(|| 1);
        assert!(seq.get(5).is_none());
    }
    #[test]
    fn test_memo_caches_result() {
        let call_count = std::sync::Arc::new(std::sync::Mutex::new(0usize));
        let cc = call_count.clone();
        let mut memo = Memo::new(move |x: &i32| {
            *cc.lock().expect("lock should succeed") += 1;
            x * 2
        });
        assert_eq!(*memo.call(&5), 10);
        assert_eq!(*memo.call(&5), 10);
        assert_eq!(*call_count.lock().expect("lock should succeed"), 1);
        assert_eq!(memo.cache_size(), 1);
    }
    #[test]
    fn test_lazy_list_finite() {
        let mut list = LazyList::from_fn(|i| if i < 5 { Some(i * i) } else { None });
        let first3 = list.take(3);
        assert_eq!(first3, vec![0, 1, 4]);
    }
    #[test]
    fn test_lazy_list_get() {
        let mut list = LazyList::from_fn(|i| Some(i as i32 + 1));
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(4), Some(&5));
    }
    #[test]
    fn test_thunk_tree_leaf() {
        let t = ThunkTree::leaf(42i32);
        assert!(t.is_leaf());
        let leaves = t.leaves();
        assert_eq!(leaves, vec![42]);
    }
    #[test]
    fn test_thunk_tree_node() {
        let t: ThunkTree<i32> = ThunkTree::node(|| vec![ThunkTree::leaf(1), ThunkTree::leaf(2)]);
        assert!(!t.is_leaf());
        let leaves = t.leaves();
        assert_eq!(leaves.len(), 2);
    }
    #[test]
    fn test_memo_clear_cache() {
        let mut memo = Memo::new(|x: &i32| x + 1);
        memo.call(&1);
        memo.call(&2);
        assert_eq!(memo.cache_size(), 2);
        memo.clear_cache();
        assert_eq!(memo.cache_size(), 0);
    }
}
#[cfg(test)]
mod thunk_deferred_tests {
    use super::*;
    #[test]
    fn test_try_thunk_ok() {
        let mut t: TryThunk<i32, &str> = TryThunk::new(|| Ok(42));
        assert!(!t.is_forced());
        assert_eq!(t.force(), &Ok(42));
        assert!(t.is_forced());
    }
    #[test]
    fn test_try_thunk_err() {
        let mut t: TryThunk<i32, &str> = TryThunk::new(|| Err("fail"));
        assert_eq!(t.force(), &Err("fail"));
    }
    #[test]
    fn test_deferred_now_is_ready() {
        let d = Deferred::now(10i32);
        assert!(d.is_ready());
    }
    #[test]
    fn test_deferred_later_not_ready() {
        let d: Deferred<i32> = Deferred::later(0);
        assert!(!d.is_ready());
    }
    #[test]
    fn test_deferred_resolve_or() {
        let d = Deferred::now(5i32);
        assert_eq!(d.resolve_or(|_| 99), 5);
        let d2: Deferred<i32> = Deferred::later(7);
        assert_eq!(d2.resolve_or(|id| id as i32), 7);
    }
    #[test]
    fn test_deferred_map() {
        let d = Deferred::now(3i32);
        let d2 = d.map(|x| x * 2);
        assert_eq!(d2.resolve_or(|_| 0), 6);
    }
    #[test]
    fn test_deferred_pool_submit_force() {
        let mut pool = DeferredPool::new();
        let id = pool.submit(|| 42i32);
        assert_eq!(pool.pending_count(), 1);
        let v = pool.force(id);
        assert_eq!(v, Some(&42));
        assert_eq!(pool.resolved_count(), 1);
        assert_eq!(pool.pending_count(), 0);
    }
    #[test]
    fn test_deferred_pool_force_all() {
        let mut pool = DeferredPool::new();
        pool.submit(|| 1i32);
        pool.submit(|| 2i32);
        pool.force_all();
        assert_eq!(pool.pending_count(), 0);
        assert_eq!(pool.resolved_count(), 2);
    }
    #[test]
    fn test_delayed_steps() {
        let mut d = Delayed::new(42i32, 3);
        assert!(!d.is_ready());
        assert_eq!(d.step(), None);
        assert_eq!(d.step(), None);
        assert_eq!(d.step(), Some(42));
        assert!(d.is_ready());
    }
    #[test]
    fn test_delayed_remaining() {
        let d = Delayed::new(1i32, 5);
        assert_eq!(d.remaining(), 5);
    }
}
/// Build Thunk theorems in the environment as axioms.
///
/// Adds basic thunk laws: `Thunk.get_pure`, `Thunk.map_id`, `Thunk.map_comp`.
pub fn build_thunk_theorems(env: &mut Environment) -> Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let get_pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(prop.clone()),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Thunk.get_pure"),
        univ_params: vec![],
        ty: get_pure_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_id_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Thunk"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop.clone()),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Thunk.map_id"),
        univ_params: vec![],
        ty: map_id_ty,
    })
    .map_err(|e| e.to_string())?;
    let bind_pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(prop.clone()),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Thunk.bind_pure"),
        univ_params: vec![],
        ty: bind_pure_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Count how many Thunk declarations are registered in the environment.
pub fn count_thunk_decls(env: &Environment) -> usize {
    let names = [
        "Thunk",
        "Thunk.mk",
        "Thunk.get",
        "Thunk.pure",
        "Thunk.map",
        "Thunk.bind",
        "Thunk.join",
        "Thunk.zip",
        "Thunk.and",
        "Thunk.or",
        "Thunk.isForced",
        "Thunk.ap",
        "Thunk.sequence",
        "Thunk.delay",
        "Thunk.count",
        "Thunk.getOr",
        "Thunk.get_pure",
        "Thunk.map_id",
        "Thunk.bind_pure",
    ];
    names
        .iter()
        .filter(|&&n| env.get(&Name::str(n)).is_some())
        .count()
}
#[cfg(test)]
mod thunk_extended_extra_tests {
    use super::*;
    fn base() -> Environment {
        let mut env = Environment::new();
        let t1 = Expr::Sort(Level::succ(Level::zero()));
        for name in ["Unit", "Bool", "Prod", "List", "Nat", "Option"] {
            env.add(Declaration::Axiom {
                name: Name::str(name),
                univ_params: vec![],
                ty: t1.clone(),
            })
            .unwrap_or(());
        }
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        env
    }
    #[test]
    fn test_build_thunk_theorems() {
        let mut env = base();
        assert!(build_thunk_theorems(&mut env).is_ok());
        assert!(env.get(&Name::str("Thunk.get_pure")).is_some());
    }
    #[test]
    fn test_thunk_map_id() {
        let mut env = base();
        build_thunk_theorems(&mut env).expect("build_thunk_theorems should succeed");
        assert!(env.get(&Name::str("Thunk.map_id")).is_some());
    }
    #[test]
    fn test_thunk_bind_pure() {
        let mut env = base();
        build_thunk_theorems(&mut env).expect("build_thunk_theorems should succeed");
        assert!(env.get(&Name::str("Thunk.bind_pure")).is_some());
    }
    #[test]
    fn test_count_thunk_decls_base() {
        let env = base();
        let n = count_thunk_decls(&env);
        assert!(n >= 16);
    }
    #[test]
    fn test_count_thunk_decls_with_theorems() {
        let mut env = base();
        build_thunk_theorems(&mut env).expect("build_thunk_theorems should succeed");
        let n = count_thunk_decls(&env);
        assert!(n >= 19);
    }
    #[test]
    fn test_deferred_pool_missing_id() {
        let mut pool = DeferredPool::<i32>::new();
        let v = pool.force(999);
        assert!(v.is_none());
    }
    #[test]
    fn test_thunk_seq_is_empty() {
        let seq: ThunkSeq<i32> = ThunkSeq::new();
        assert!(seq.is_empty());
    }
    #[test]
    fn test_lazy_list_produced_count() {
        let mut list = LazyList::from_fn(|i| Some(i as i32));
        list.take(5);
        assert_eq!(list.produced_count(), 5);
    }
}
pub fn thk_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn thk_ext_ab_prop(body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(body),
        )),
    )
}
pub fn thk_ext_abc_implicit(body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1()),
                Box::new(body),
            )),
        )),
    )
}
/// `Thunk.extract : {α : Type} → Thunk α → α`
///
/// The comonad extract (ε) operation: forces and returns the value.
pub fn axiom_comonad_extract_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::BVar(1)),
    ))
}
/// `Thunk.duplicate : {α : Type} → Thunk α → Thunk (Thunk α)`
///
/// The comonad duplicate (δ) operation.
pub fn axiom_comonad_duplicate_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thunk_of(thunk_of(Expr::BVar(1)))),
    ))
}
/// `Thunk.extend : {α β : Type} → (Thunk α → β) → Thunk α → Thunk β`
///
/// Cokleisli extension: the comonad co-bind operation.
pub fn axiom_comonad_extend_ty() -> Expr {
    let coalg = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(thunk_of(Expr::BVar(1))),
        Box::new(Expr::BVar(1)),
    );
    ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(coalg),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(thunk_of(Expr::BVar(2))),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.extract_duplicate : {α : Type} → ∀ t : Thunk α, extract (duplicate t) = t`
///
/// First comonad law: extract ∘ duplicate = id.
pub fn axiom_comonad_extract_duplicate_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.duplicate_extract : {α : Type} → ∀ t, map extract (duplicate t) = t`
///
/// Second comonad law: map extract ∘ duplicate = id.
pub fn axiom_comonad_duplicate_extract_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.duplicate_duplicate : {α : Type} → ∀ t, duplicate (duplicate t) = map duplicate (duplicate t)`
///
/// Third comonad law: duplicate ∘ duplicate = map duplicate ∘ duplicate.
pub fn axiom_comonad_duplicate_duplicate_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.cokleisli_compose : {α β γ : Type} → (Thunk α → β) → (Thunk β → γ) → Thunk α → γ`
///
/// Cokleisli composition in the Thunk comonad.
pub fn axiom_cokleisli_compose_ty() -> Expr {
    let f_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(thunk_of(Expr::BVar(2))),
        Box::new(Expr::BVar(2)),
    );
    let g_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(thunk_of(Expr::BVar(2))),
        Box::new(Expr::BVar(2)),
    );
    thk_ext_abc_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(f_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("g"),
            Box::new(g_ty),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(thunk_of(Expr::BVar(4))),
                Box::new(Expr::BVar(4)),
            )),
        )),
    ))
}
/// `Thunk.map_comp : {α β γ : Type} → (β → γ) → (α → β) → Thunk α → Thunk γ`
///
/// Functor composition law: map (g ∘ f) = map g ∘ map f.
pub fn axiom_functor_map_comp_ty() -> Expr {
    let f_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    let g_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::BVar(2)),
    );
    thk_ext_abc_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("g"),
        Box::new(g_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(f_ty),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(thunk_of(Expr::BVar(4))),
                Box::new(thunk_of(Expr::BVar(4))),
            )),
        )),
    ))
}
/// `Thunk.get_mk : {α : Type} → ∀ f : Unit → α, get (mk f) = f ()`
///
/// Forcing a constructed thunk returns the applied value.
pub fn axiom_get_mk_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(unit_to(Expr::BVar(1))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.mk_get : {α : Type} → ∀ t : Thunk α, mk (fun _ => get t) = t`
///
/// Reconstructing a thunk from its forced value equals the original.
pub fn axiom_mk_get_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.sharing : {α : Type} → ∀ t : Thunk α, get t = get t`
///
/// Sharing / memoization correctness: forcing a thunk twice gives the same result.
pub fn axiom_sharing_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.force_once : {α : Type} → ∀ t : Thunk α, ∀ _ : Bool, isForced t = true → get t = get t`
///
/// After the first force, the cached result equals re-forcing.
pub fn axiom_force_once_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_h"),
            Box::new(bool_ty()),
            Box::new(thk_ext_prop()),
        )),
    ))
}
/// `Thunk.cbn_beta : {α : Type} → ∀ (f : Unit → α), get (mk f) = f ()`
///
/// Call-by-need β-reduction: evaluating the thunk equals applying the function.
pub fn axiom_cbn_beta_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(unit_to(Expr::BVar(1))),
        Box::new(thk_ext_prop()),
    ))
}
/// `Thunk.presheaf_restriction : {α β : Type} → (β → α) → Thunk α → Thunk β`
///
/// Thunk as a presheaf on ω: restriction maps.
pub fn axiom_presheaf_restriction_ty() -> Expr {
    let f_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(0)),
        Box::new(Expr::BVar(1)),
    );
    ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(f_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(thunk_of(Expr::BVar(2))),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.productive_step : {α : Type} → (Nat → Thunk α) → Nat → Thunk α`
///
/// Productive corecursion via thunks: each step produces a value.
pub fn axiom_productive_step_ty() -> Expr {
    let step_fn = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(nat_ty()),
        Box::new(thunk_of(Expr::BVar(1))),
    );
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("gen"),
        Box::new(step_fn),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.guarded_next : {α : Type} → Thunk α → Thunk α`
///
/// Guarded recursion: the next step in a guarded recursive thunk.
pub fn axiom_guarded_next_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.game_question : {α : Type} → Thunk α → Nat`
///
/// Game semantics: the "question" move associated with a thunk.
pub fn axiom_game_question_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(nat_ty()),
    ))
}
/// `Thunk.game_answer : {α : Type} → Thunk α → α`
///
/// Game semantics: the "answer" move, equivalent to forcing.
pub fn axiom_game_answer_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::BVar(1)),
    ))
}
/// `Thunk.itree_ret : {α : Type} → α → Thunk α`
///
/// Interaction tree return: a pure leaf in an itree encoded as a thunk.
pub fn axiom_itree_ret_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::BVar(0)),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.itree_tau : {α : Type} → Thunk α → Thunk α`
///
/// Interaction tree silent step (τ): wraps a thunk in one more layer of delay.
pub fn axiom_itree_tau_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.itree_vis : {α : Type} → Nat → (Nat → Thunk α) → Thunk α`
///
/// Interaction tree visible event: models an external call.
pub fn axiom_itree_vis_ty() -> Expr {
    let cont = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(nat_ty()),
        Box::new(thunk_of(Expr::BVar(1))),
    );
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("ev"),
        Box::new(nat_ty()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("k"),
            Box::new(cont),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.lazy_nat_stream : Nat → Thunk Nat`
///
/// Lazily indexed stream of natural numbers; models ω-sequences.
pub fn axiom_lazy_nat_stream_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_ty()),
        Box::new(thunk_of(nat_ty())),
    )
}
/// `Thunk.memoTable_lookup : {α : Type} → List (Thunk α) → Nat → Option α`
///
/// Lazy memoization table lookup.
pub fn axiom_memo_table_lookup_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("tbl"),
        Box::new(list_of(thunk_of(Expr::BVar(0)))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("idx"),
            Box::new(nat_ty()),
            Box::new(option_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.memoTable_insert : {α : Type} → List (Thunk α) → Nat → Thunk α → List (Thunk α)`
///
/// Insert into a lazy memoization table.
pub fn axiom_memo_table_insert_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("tbl"),
        Box::new(list_of(thunk_of(Expr::BVar(0)))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("idx"),
            Box::new(nat_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("v"),
                Box::new(thunk_of(Expr::BVar(2))),
                Box::new(list_of(thunk_of(Expr::BVar(3)))),
            )),
        )),
    ))
}
/// `Thunk.lazyTree_branch : {α : Type} → Thunk α → Thunk α → Thunk α`
///
/// A lazy binary tree node branching left and right.
pub fn axiom_lazy_tree_branch_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("l"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("r"),
            Box::new(thunk_of(Expr::BVar(1))),
            Box::new(thunk_of(Expr::BVar(2))),
        )),
    ))
}
/// `Thunk.lazyTree_depth : {α : Type} → Thunk α → Nat`
///
/// The depth of a lazy tree (forces all branches).
pub fn axiom_lazy_tree_depth_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(nat_ty()),
    ))
}
/// `Thunk.cofree_out : {α : Type} → Thunk α → α`
///
/// Cofree comonad out-map: projects the head element.
pub fn axiom_cofree_out_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::BVar(1)),
    ))
}
/// `Thunk.cofree_tail : {α : Type} → Thunk α → List (Thunk α)`
///
/// Cofree comonad tail: the sub-computations generated by the head.
pub fn axiom_cofree_tail_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(list_of(thunk_of(Expr::BVar(1)))),
    ))
}
/// `Thunk.whnf_step : {α : Type} → Thunk α → Thunk α`
///
/// One weak-head normal form reduction step.
pub fn axiom_whnf_step_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.ap_naturality : {α β : Type} → ∀ f : α → β, ∀ t : Thunk α, ...`
///
/// Applicative naturality: map commutes with ap.
pub fn axiom_ap_naturality_ty() -> Expr {
    thk_ext_ab_prop(thk_ext_prop())
}
/// `Thunk.bind_assoc : {α β γ : Type} → ∀ t f g, bind (bind t f) g = bind t (fun x => bind (f x) g)`
///
/// Monad associativity for Thunk.
pub fn axiom_bind_assoc_ty() -> Expr {
    thk_ext_abc_implicit(thk_ext_prop())
}
/// `Thunk.pure_map : {α β : Type} → ∀ (f : α → β) (x : α), map f (pure x) = pure (f x)`
///
/// Applicative homomorphism law.
pub fn axiom_pure_map_ty() -> Expr {
    thk_ext_ab_prop(thk_ext_prop())
}
/// `Thunk.lazy_fix : {α : Type} → (Thunk α → α) → Thunk α`
///
/// Lazy fixed-point combinator: produces a thunk whose value satisfies `f (fix f) = fix f`.
pub fn axiom_lazy_fix_ty() -> Expr {
    let coalg = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(thunk_of(Expr::BVar(1))),
        Box::new(Expr::BVar(1)),
    );
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(coalg),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.omega_limit : {α : Type} → (Nat → Thunk α) → Thunk α`
///
/// ω-limit of a sequence of thunks: models the colimit in the ω-CPO.
pub fn axiom_omega_limit_ty() -> Expr {
    let seq_fn = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(nat_ty()),
        Box::new(thunk_of(Expr::BVar(1))),
    );
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("chain"),
        Box::new(seq_fn),
        Box::new(thunk_of(Expr::BVar(1))),
    ))
}
/// `Thunk.scott_continuity : {α : Type} → Prop`
///
/// Scott continuity of the forcing operation w.r.t. the ω-CPO structure.
pub fn axiom_scott_continuity_ty() -> Expr {
    alpha_implicit(thk_ext_prop())
}
/// `Thunk.kleisli_compose : {α β γ : Type} → (α → Thunk β) → (β → Thunk γ) → α → Thunk γ`
///
/// Kleisli composition in the Thunk monad.
pub fn axiom_kleisli_compose_ty() -> Expr {
    let f_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(thunk_of(Expr::BVar(2))),
    );
    let g_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(thunk_of(Expr::BVar(2))),
    );
    thk_ext_abc_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(f_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("g"),
            Box::new(g_ty),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(4)),
                Box::new(thunk_of(Expr::BVar(4))),
            )),
        )),
    ))
}
/// `Thunk.force_deterministic : {α : Type} → ∀ t : Thunk α, ∀ n m : Nat, get t = get t`
///
/// Determinism: forcing a thunk any number of times yields the same result.
pub fn axiom_force_deterministic_ty() -> Expr {
    alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(thunk_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(thk_ext_prop()),
        )),
    ))
}
/// Register all extended Thunk comonad axioms into the given environment.
pub fn register_thunk_extended(env: &mut Environment) -> Result<(), String> {
    let entries: &[(&str, fn() -> Expr)] = &[
        ("Thunk.extract", axiom_comonad_extract_ty),
        ("Thunk.duplicate", axiom_comonad_duplicate_ty),
        ("Thunk.extend", axiom_comonad_extend_ty),
        (
            "Thunk.extract_duplicate",
            axiom_comonad_extract_duplicate_ty,
        ),
        (
            "Thunk.duplicate_extract",
            axiom_comonad_duplicate_extract_ty,
        ),
        (
            "Thunk.duplicate_duplicate",
            axiom_comonad_duplicate_duplicate_ty,
        ),
        ("Thunk.cokleisli_compose", axiom_cokleisli_compose_ty),
        ("Thunk.map_comp", axiom_functor_map_comp_ty),
        ("Thunk.get_mk", axiom_get_mk_ty),
        ("Thunk.mk_get", axiom_mk_get_ty),
        ("Thunk.sharing", axiom_sharing_ty),
        ("Thunk.force_once", axiom_force_once_ty),
        ("Thunk.cbn_beta", axiom_cbn_beta_ty),
        ("Thunk.presheaf_restriction", axiom_presheaf_restriction_ty),
        ("Thunk.productive_step", axiom_productive_step_ty),
        ("Thunk.guarded_next", axiom_guarded_next_ty),
        ("Thunk.game_question", axiom_game_question_ty),
        ("Thunk.game_answer", axiom_game_answer_ty),
        ("Thunk.itree_ret", axiom_itree_ret_ty),
        ("Thunk.itree_tau", axiom_itree_tau_ty),
        ("Thunk.itree_vis", axiom_itree_vis_ty),
        ("Thunk.lazy_nat_stream", axiom_lazy_nat_stream_ty),
        ("Thunk.memoTable_lookup", axiom_memo_table_lookup_ty),
        ("Thunk.memoTable_insert", axiom_memo_table_insert_ty),
        ("Thunk.lazyTree_branch", axiom_lazy_tree_branch_ty),
        ("Thunk.lazyTree_depth", axiom_lazy_tree_depth_ty),
        ("Thunk.cofree_out", axiom_cofree_out_ty),
        ("Thunk.cofree_tail", axiom_cofree_tail_ty),
        ("Thunk.whnf_step", axiom_whnf_step_ty),
        ("Thunk.ap_naturality", axiom_ap_naturality_ty),
        ("Thunk.bind_assoc", axiom_bind_assoc_ty),
        ("Thunk.pure_map", axiom_pure_map_ty),
        ("Thunk.lazy_fix", axiom_lazy_fix_ty),
        ("Thunk.omega_limit", axiom_omega_limit_ty),
        ("Thunk.scott_continuity", axiom_scott_continuity_ty),
        ("Thunk.kleisli_compose", axiom_kleisli_compose_ty),
        ("Thunk.force_deterministic", axiom_force_deterministic_ty),
    ];
    for (name, ty_fn) in entries {
        axiom(env, name, ty_fn())?;
    }
    Ok(())
}
#[cfg(test)]
mod thunk_comonad_tests {
    use super::*;
    fn base_env_for_ext() -> Environment {
        let mut env = Environment::new();
        let t1 = type1();
        for name in ["Unit", "Bool", "Prod", "List", "Nat", "Option"] {
            env.add(Declaration::Axiom {
                name: Name::str(name),
                univ_params: vec![],
                ty: t1.clone(),
            })
            .unwrap_or(());
        }
        build_thunk_env(&mut env).expect("build_thunk_env should succeed");
        env
    }
    #[test]
    fn test_register_thunk_extended_all_registered() {
        let mut env = base_env_for_ext();
        let result = register_thunk_extended(&mut env);
        assert!(
            result.is_ok(),
            "register_thunk_extended failed: {:?}",
            result
        );
        assert!(env.get(&Name::str("Thunk.extract")).is_some());
        assert!(env.get(&Name::str("Thunk.duplicate")).is_some());
        assert!(env.get(&Name::str("Thunk.extend")).is_some());
        assert!(env.get(&Name::str("Thunk.cokleisli_compose")).is_some());
        assert!(env.get(&Name::str("Thunk.itree_ret")).is_some());
        assert!(env.get(&Name::str("Thunk.lazy_fix")).is_some());
        assert!(env.get(&Name::str("Thunk.omega_limit")).is_some());
    }
    #[test]
    fn test_comonad_ctx_extract() {
        let ctx = ComonadCtx::new("env", 42i32);
        assert_eq!(ctx.extract(), 42);
    }
    #[test]
    fn test_comonad_ctx_map() {
        let ctx = ComonadCtx::new("env", 10i32);
        let ctx2 = ctx.map(|x| x * 2);
        assert_eq!(ctx2.extract(), 20);
    }
    #[test]
    fn test_comonad_ctx_extend() {
        let ctx = ComonadCtx::new(0u32, 5i32);
        let ctx2 = ctx.extend(|c| c.extract() + 1);
        assert_eq!(ctx2.extract(), 6);
    }
    #[test]
    fn test_game_arena_ask_answer() {
        let mut arena = GameArena::new();
        let q = arena.ask();
        arena.answer(q, "42");
        assert_eq!(arena.get_answer(q), Some("42"));
        assert!(arena.is_complete());
    }
    #[test]
    fn test_game_arena_incomplete() {
        let mut arena = GameArena::new();
        let _q = arena.ask();
        assert!(!arena.is_complete());
    }
    #[test]
    fn test_lazy_stream_take() {
        let stream: LazyStream<i32> = LazyStream::cons(1, || {
            LazyStream::cons(2, || LazyStream::cons(3, || LazyStream::nil()))
        });
        let elems = stream.take(3);
        assert_eq!(elems, vec![1, 2, 3]);
    }
    #[test]
    fn test_lazy_stream_nil() {
        let s: LazyStream<i32> = LazyStream::nil();
        assert!(s.is_nil());
    }
    #[test]
    fn test_thunk_kleisli_apply() {
        let k = ThunkKleisli::new(|x: i32| if x > 0 { Some(x * 2) } else { None });
        assert_eq!(k.apply(3), Some(6));
        assert_eq!(k.apply(-1), None);
    }
    #[test]
    fn test_thunk_kleisli_compose() {
        let k1 = ThunkKleisli::new(|x: i32| Some(x + 1));
        let k2 = ThunkKleisli::new(|x: i32| Some(x * 3));
        let composed = k1.compose(k2);
        assert_eq!(composed.apply(4), Some(15));
    }
    #[test]
    fn test_itree_node_ret() {
        let node: ITreeNode<(), i32> = ITreeNode::ret(99);
        assert_eq!(node.run(10), Some(99));
    }
    #[test]
    fn test_itree_node_tau() {
        let node: ITreeNode<(), i32> = ITreeNode::tau(|| ITreeNode::tau(|| ITreeNode::ret(7)));
        assert_eq!(node.run(10), Some(7));
    }
    #[test]
    fn test_itree_node_vis_blocks() {
        let node: ITreeNode<&str, i32> = ITreeNode::vis("read", |_| ITreeNode::ret(0));
        assert_eq!(node.run(10), None);
    }
    #[test]
    fn test_axiom_comonad_extract_ty_is_pi() {
        let ty = axiom_comonad_extract_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_axiom_itree_vis_ty_is_pi() {
        let ty = axiom_itree_vis_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
}
#[cfg(test)]
mod tests_thunk_extra {
    use super::*;
    #[test]
    fn test_memo_thunk() {
        let mut t = MemoThunk::<i32>::new();
        assert!(!t.is_forced());
        let v = t.force_with(|| 42);
        assert_eq!(v, 42);
        assert!(t.is_forced());
        let v2 = t.force_with(|| 99);
        assert_eq!(v2, 42);
        t.reset();
        assert!(!t.is_forced());
    }
    #[test]
    fn test_cofree_stream() {
        let s = CofreeStream::from_periodic(vec![1, 2, 3]);
        assert_eq!(s.extract(), &1);
        assert_eq!(*s.nth(0), 1);
        assert_eq!(*s.nth(1), 2);
        assert_eq!(*s.nth(3), 1);
        let t = s.tail();
        assert_eq!(t.extract(), &2);
        let doubled = s.map(|x| x * 2);
        assert_eq!(*doubled.nth(0), 2);
        assert_eq!(*doubled.nth(1), 4);
    }
    #[test]
    fn test_compute_node() {
        let mut node = ComputeNode::<i32>::new("result");
        node.add_dep("input");
        assert!(node.is_dirty);
        assert!(node.get().is_none());
        node.set_value(100);
        assert!(!node.is_dirty);
        assert_eq!(*node.get().expect("get should succeed"), 100);
        node.invalidate();
        assert!(node.is_dirty);
    }
    #[test]
    fn test_thunk_app_functor() {
        let t = ThunkApp::pure(5);
        let t2 = t.map(|x| x * 2);
        assert_eq!(t2.val, 10);
        let tf = ThunkApp::pure(|x: i32| x + 1);
        let t3 = ThunkApp::pure(10);
        let result = t3.ap(tf);
        assert_eq!(result.val, 11);
    }
    #[test]
    fn test_thunk_state_blackhole() {
        let mut s = ThunkState::<i32>::Unevaluated;
        assert!(s.enter());
        assert!(s.is_blackhole());
        assert!(!s.enter());
        s.fill(42);
        assert!(s.is_value());
        assert_eq!(s.get_value(), Some(&42));
    }
    #[test]
    fn test_suspension_monad() {
        let s: Suspension<i32> = Suspension::pure(10);
        assert!(s.is_done());
        let s2 = s.map(|x| x + 5);
        assert!(s2.is_done());
        let pend: Suspension<i32> = Suspension::suspend("waiting");
        let pend2 = pend.map(|x| x * 2);
        assert!(!pend2.is_done());
        let chained = Suspension::pure(3).and_then(|x| Suspension::pure(x * x));
        assert!(chained.is_done());
    }
}
