//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    FloatInterval, IntervalScheduler, KrawczykSolver, RangeMinSegTree, ScheduledJob,
    ValidatedInterval,
};

/// Prop: `Sort 0`.
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type 1: `Sort 1`.
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Type 2: `Sort 2`.
#[allow(dead_code)]
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Nat type constant.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
/// Bool type constant.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
/// Unit type constant.
#[allow(dead_code)]
pub fn unit_ty() -> Expr {
    Expr::Const(Name::str("Unit"), vec![])
}
/// Range type constant.
#[allow(dead_code)]
pub fn range_ty() -> Expr {
    Expr::Const(Name::str("Range"), vec![])
}
/// RangeIterator type constant.
#[allow(dead_code)]
pub fn range_iterator_ty() -> Expr {
    Expr::Const(Name::str("RangeIterator"), vec![])
}
/// `List` applied to a type argument.
#[allow(dead_code)]
pub fn list_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("List"), vec![]), elem_ty)
}
/// `Array` applied to a type argument.
#[allow(dead_code)]
pub fn array_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Array"), vec![]), elem_ty)
}
/// `Option` applied to a type argument.
#[allow(dead_code)]
pub fn option_of(elem_ty: Expr) -> Expr {
    app(Expr::Const(Name::str("Option"), vec![]), elem_ty)
}
/// `Prod` applied to two type arguments.
#[allow(dead_code)]
pub fn prod_of(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Prod"), vec![]), a, b)
}
/// `Monad` applied to a type constructor.
#[allow(dead_code)]
pub fn monad_of(m: Expr) -> Expr {
    app(Expr::Const(Name::str("Monad"), vec![]), m)
}
/// `Iff` applied to two propositions.
#[allow(dead_code)]
pub fn iff(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Iff"), vec![]), a, b)
}
/// Build a non-dependent arrow `A -> B`.
#[allow(dead_code)]
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Function application `f a`.
#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Function application `f a b`.
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
/// Function application `f a b c`.
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
/// Function application `f a b c d`.
#[allow(dead_code)]
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
/// An implicit Pi binder.
#[allow(dead_code)]
pub fn implicit_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// A default (explicit) Pi binder.
#[allow(dead_code)]
pub fn default_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// An instance-implicit Pi binder.
#[allow(dead_code)]
pub fn inst_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::InstImplicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn eq_expr(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(Expr::Const(Name::str("Eq"), vec![]), ty, a, b)
}
/// Shorthand to add an axiom to env.
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
/// `Type → Type` (kind of a type constructor like a monad).
#[allow(dead_code)]
pub fn type_to_type() -> Expr {
    arrow(type1(), type1())
}
/// Build `And a b`.
#[allow(dead_code)]
pub fn and_prop(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("And"), vec![]), a, b)
}
/// Build `Nat.le a b`.
#[allow(dead_code)]
pub fn le_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.le"), vec![]), a, b)
}
/// Build `Nat.lt a b`.
#[allow(dead_code)]
pub fn lt_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.lt"), vec![]), a, b)
}
/// Build `Nat.sub a b`.
#[allow(dead_code)]
pub fn sub_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.sub"), vec![]), a, b)
}
/// Build `Nat.mod a b`.
#[allow(dead_code)]
pub fn mod_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.mod"), vec![]), a, b)
}
/// Build `Nat.ge a b` (= Nat.le b a).
#[allow(dead_code)]
pub fn ge_nat(a: Expr, b: Expr) -> Expr {
    le_nat(b, a)
}
/// Build `BEq.beq a b = Bool.true`.
#[allow(dead_code)]
pub fn eq_true(e: Expr) -> Expr {
    eq_expr(bool_ty(), e, Expr::Const(Name::str("Bool.true"), vec![]))
}
/// Build the Range type, iteration protocols, operations, and theorems.
///
/// Assumes that `Nat`, `Bool`, `Unit`, `List`, `Array`, `Option`, `Prod`,
/// `Monad`, `Eq`, `Iff`, `And` are already declared or referenced by name.
pub fn build_range_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Range", vec![], type1())?;
    let range_mk_ty = default_pi(
        "start",
        nat_ty(),
        default_pi("stop", nat_ty(), default_pi("step", nat_ty(), range_ty())),
    );
    add_axiom(env, "Range.mk", vec![], range_mk_ty)?;
    let range_start_ty = arrow(range_ty(), nat_ty());
    add_axiom(env, "Range.start", vec![], range_start_ty)?;
    let range_stop_ty = arrow(range_ty(), nat_ty());
    add_axiom(env, "Range.stop", vec![], range_stop_ty)?;
    let range_step_ty = arrow(range_ty(), nat_ty());
    add_axiom(env, "Range.step", vec![], range_step_ty)?;
    let of_nat_ty = default_pi("start", nat_ty(), default_pi("stop", nat_ty(), range_ty()));
    add_axiom(env, "Range.ofNat", vec![], of_nat_ty)?;
    let single_ty = arrow(nat_ty(), range_ty());
    add_axiom(env, "Range.single", vec![], single_ty)?;
    let with_step_ty = default_pi(
        "start",
        nat_ty(),
        default_pi("stop", nat_ty(), default_pi("step", nat_ty(), range_ty())),
    );
    add_axiom(env, "Range.withStep", vec![], with_step_ty)?;
    let for_in_ty = default_pi(
        "m",
        type_to_type(),
        default_pi("α", type1(), default_pi("β", type1(), type2())),
    );
    add_axiom(env, "ForIn", vec![], for_in_ty)?;
    let for_in_method_ty = implicit_pi(
        "m",
        type_to_type(),
        implicit_pi(
            "α",
            type1(),
            implicit_pi(
                "β",
                type1(),
                inst_pi(
                    "_inst",
                    app3(
                        Expr::Const(Name::str("ForIn"), vec![]),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                    default_pi(
                        "container",
                        Expr::BVar(2),
                        default_pi(
                            "init",
                            Expr::BVar(2),
                            default_pi(
                                "body",
                                arrow(Expr::BVar(3), app(Expr::BVar(6), Expr::BVar(4))),
                                app(Expr::BVar(6), Expr::BVar(4)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "ForIn.forIn", vec![], for_in_method_ty)?;
    let inst_for_in_range_ty = app3(
        Expr::Const(Name::str("ForIn"), vec![]),
        Expr::Const(Name::str("Id"), vec![]),
        range_ty(),
        nat_ty(),
    );
    add_axiom(env, "instForInRangeNat", vec![], inst_for_in_range_ty)?;
    let iterator_ty = default_pi("α", type1(), default_pi("σ", type1(), type2()));
    add_axiom(env, "Iterator", vec![], iterator_ty)?;
    let next_ty = implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "σ",
            type1(),
            inst_pi(
                "_inst",
                app2(
                    Expr::Const(Name::str("Iterator"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                default_pi(
                    "state",
                    Expr::BVar(1),
                    option_of(prod_of(Expr::BVar(3), Expr::BVar(2))),
                ),
            ),
        ),
    );
    add_axiom(env, "Iterator.next", vec![], next_ty)?;
    let has_next_ty = implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "σ",
            type1(),
            inst_pi(
                "_inst",
                app2(
                    Expr::Const(Name::str("Iterator"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                default_pi("state", Expr::BVar(1), bool_ty()),
            ),
        ),
    );
    add_axiom(env, "Iterator.hasNext", vec![], has_next_ty)?;
    add_axiom(env, "RangeIterator", vec![], type1())?;
    let inst_iter_ty = app2(
        Expr::Const(Name::str("Iterator"), vec![]),
        nat_ty(),
        range_iterator_ty(),
    );
    add_axiom(env, "instIteratorNatRangeIterator", vec![], inst_iter_ty)?;
    let is_empty_ty = arrow(range_ty(), bool_ty());
    add_axiom(env, "Range.isEmpty", vec![], is_empty_ty)?;
    let size_ty = arrow(range_ty(), nat_ty());
    add_axiom(env, "Range.size", vec![], size_ty)?;
    let contains_ty = default_pi("r", range_ty(), default_pi("n", nat_ty(), bool_ty()));
    add_axiom(env, "Range.contains", vec![], contains_ty)?;
    let to_list_ty = arrow(range_ty(), list_of(nat_ty()));
    add_axiom(env, "Range.toList", vec![], to_list_ty)?;
    let for_m_ty = implicit_pi(
        "m",
        type_to_type(),
        inst_pi(
            "_inst",
            monad_of(Expr::BVar(0)),
            default_pi(
                "r",
                range_ty(),
                default_pi(
                    "body",
                    arrow(nat_ty(), app(Expr::BVar(3), unit_ty())),
                    app(Expr::BVar(3), unit_ty()),
                ),
            ),
        ),
    );
    add_axiom(env, "Range.forM", vec![], for_m_ty)?;
    let foldl_ty = implicit_pi(
        "β",
        type1(),
        default_pi(
            "f",
            arrow(Expr::BVar(0), arrow(nat_ty(), Expr::BVar(1))),
            default_pi(
                "init",
                Expr::BVar(1),
                default_pi("r", range_ty(), Expr::BVar(3)),
            ),
        ),
    );
    add_axiom(env, "Range.foldl", vec![], foldl_ty)?;
    let all_ty = default_pi(
        "r",
        range_ty(),
        default_pi("pred", arrow(nat_ty(), bool_ty()), bool_ty()),
    );
    add_axiom(env, "Range.all", vec![], all_ty)?;
    let any_ty = default_pi(
        "r",
        range_ty(),
        default_pi("pred", arrow(nat_ty(), bool_ty()), bool_ty()),
    );
    add_axiom(env, "Range.any", vec![], any_ty)?;
    let array_range_ty = arrow(nat_ty(), array_of(nat_ty()));
    add_axiom(env, "Array.range", vec![], array_range_ty)?;
    let list_range_ty = arrow(nat_ty(), list_of(nat_ty()));
    add_axiom(env, "List.range", vec![], list_range_ty)?;
    let list_iota_ty = arrow(nat_ty(), list_of(nat_ty()));
    add_axiom(env, "List.iota", vec![], list_iota_ty)?;
    let enum_from_ty = implicit_pi(
        "α",
        type1(),
        default_pi(
            "start",
            nat_ty(),
            default_pi(
                "l",
                list_of(Expr::BVar(1)),
                list_of(prod_of(nat_ty(), Expr::BVar(2))),
            ),
        ),
    );
    add_axiom(env, "List.enumFrom", vec![], enum_from_ty)?;
    let size_mk_ty = default_pi(
        "start",
        nat_ty(),
        default_pi(
            "stop",
            nat_ty(),
            eq_expr(
                nat_ty(),
                app(
                    Expr::Const(Name::str("Range.size"), vec![]),
                    app3(
                        Expr::Const(Name::str("Range.mk"), vec![]),
                        Expr::BVar(1),
                        Expr::BVar(0),
                        Expr::Const(Name::str("Nat.one"), vec![]),
                    ),
                ),
                sub_nat(Expr::BVar(0), Expr::BVar(1)),
            ),
        ),
    );
    add_axiom(env, "Range.size_mk", vec![], size_mk_ty)?;
    let contains_iff_ty = default_pi(
        "start",
        nat_ty(),
        default_pi(
            "stop",
            nat_ty(),
            default_pi(
                "step",
                nat_ty(),
                default_pi(
                    "n",
                    nat_ty(),
                    iff(
                        eq_true(app2(
                            Expr::Const(Name::str("Range.contains"), vec![]),
                            app3(
                                Expr::Const(Name::str("Range.mk"), vec![]),
                                Expr::BVar(3),
                                Expr::BVar(2),
                                Expr::BVar(1),
                            ),
                            Expr::BVar(0),
                        )),
                        and_prop(
                            le_nat(Expr::BVar(3), Expr::BVar(0)),
                            and_prop(
                                lt_nat(Expr::BVar(0), Expr::BVar(2)),
                                eq_expr(
                                    nat_ty(),
                                    mod_nat(sub_nat(Expr::BVar(0), Expr::BVar(3)), Expr::BVar(1)),
                                    Expr::Const(Name::str("Nat.zero"), vec![]),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "Range.contains_iff", vec![], contains_iff_ty)?;
    let to_list_length_ty = default_pi(
        "r",
        range_ty(),
        eq_expr(
            nat_ty(),
            app(
                Expr::Const(Name::str("List.length"), vec![]),
                app(
                    Expr::Const(Name::str("Range.toList"), vec![]),
                    Expr::BVar(0),
                ),
            ),
            app(Expr::Const(Name::str("Range.size"), vec![]), Expr::BVar(0)),
        ),
    );
    add_axiom(env, "Range.toList_length", vec![], to_list_length_ty)?;
    let is_empty_iff_ty = default_pi(
        "r",
        range_ty(),
        iff(
            eq_true(app(
                Expr::Const(Name::str("Range.isEmpty"), vec![]),
                Expr::BVar(0),
            )),
            ge_nat(
                app(Expr::Const(Name::str("Range.start"), vec![]), Expr::BVar(0)),
                app(Expr::Const(Name::str("Range.stop"), vec![]), Expr::BVar(0)),
            ),
        ),
    );
    add_axiom(env, "Range.isEmpty_iff", vec![], is_empty_iff_ty)?;
    let range_length_ty = default_pi(
        "n",
        nat_ty(),
        eq_expr(
            nat_ty(),
            app(
                Expr::Const(Name::str("List.length"), vec![]),
                app(Expr::Const(Name::str("List.range"), vec![]), Expr::BVar(0)),
            ),
            Expr::BVar(0),
        ),
    );
    add_axiom(env, "List.range_length", vec![], range_length_ty)?;
    let iota_length_ty = default_pi(
        "n",
        nat_ty(),
        eq_expr(
            nat_ty(),
            app(
                Expr::Const(Name::str("List.length"), vec![]),
                app(Expr::Const(Name::str("List.iota"), vec![]), Expr::BVar(0)),
            ),
            Expr::BVar(0),
        ),
    );
    add_axiom(env, "List.iota_length", vec![], iota_length_ty)?;
    Ok(())
}
/// Build `Range.mk start stop step`.
#[allow(dead_code)]
pub fn mk_range(start: Expr, stop: Expr, step: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("Range.mk"), vec![]),
        start,
        stop,
        step,
    )
}
/// Build `Range.ofNat start stop`.
#[allow(dead_code)]
pub fn mk_range_of_nat(start: Expr, stop: Expr) -> Expr {
    app2(Expr::Const(Name::str("Range.ofNat"), vec![]), start, stop)
}
/// Build `Range.single n`.
#[allow(dead_code)]
pub fn mk_range_single(n: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.single"), vec![]), n)
}
/// Build `ForIn.forIn container init body`.
#[allow(dead_code)]
pub fn mk_for_in(container: Expr, init: Expr, body: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("ForIn.forIn"), vec![]),
        container,
        init,
        body,
    )
}
/// Build `Range.toList r`.
#[allow(dead_code)]
pub fn mk_range_to_list(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.toList"), vec![]), r)
}
/// Build `Range.size r`.
#[allow(dead_code)]
pub fn mk_range_size(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.size"), vec![]), r)
}
/// Build `List.range n`.
#[allow(dead_code)]
pub fn mk_list_range(n: Expr) -> Expr {
    app(Expr::Const(Name::str("List.range"), vec![]), n)
}
/// Build `Array.range n`.
#[allow(dead_code)]
pub fn mk_array_range(n: Expr) -> Expr {
    app(Expr::Const(Name::str("Array.range"), vec![]), n)
}
/// Build `List.iota n`.
#[allow(dead_code)]
pub fn mk_list_iota(n: Expr) -> Expr {
    app(Expr::Const(Name::str("List.iota"), vec![]), n)
}
/// Build `Range.isEmpty r`.
#[allow(dead_code)]
pub fn mk_range_is_empty(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.isEmpty"), vec![]), r)
}
/// Build `Range.contains r n`.
#[allow(dead_code)]
pub fn mk_range_contains(r: Expr, n: Expr) -> Expr {
    app2(Expr::Const(Name::str("Range.contains"), vec![]), r, n)
}
/// Build `Range.foldl f init r`.
#[allow(dead_code)]
pub fn mk_range_foldl(f: Expr, init: Expr, r: Expr) -> Expr {
    app3(Expr::Const(Name::str("Range.foldl"), vec![]), f, init, r)
}
/// Build `Range.all r pred`.
#[allow(dead_code)]
pub fn mk_range_all(r: Expr, pred: Expr) -> Expr {
    app2(Expr::Const(Name::str("Range.all"), vec![]), r, pred)
}
/// Build `Range.any r pred`.
#[allow(dead_code)]
pub fn mk_range_any(r: Expr, pred: Expr) -> Expr {
    app2(Expr::Const(Name::str("Range.any"), vec![]), r, pred)
}
/// Build `Range.start r`.
#[allow(dead_code)]
pub fn mk_range_start(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.start"), vec![]), r)
}
/// Build `Range.stop r`.
#[allow(dead_code)]
pub fn mk_range_stop(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.stop"), vec![]), r)
}
/// Build `Range.step r`.
#[allow(dead_code)]
pub fn mk_range_step(r: Expr) -> Expr {
    app(Expr::Const(Name::str("Range.step"), vec![]), r)
}
/// Build `Range.withStep start stop step`.
#[allow(dead_code)]
pub fn mk_range_with_step(start: Expr, stop: Expr, step: Expr) -> Expr {
    app3(
        Expr::Const(Name::str("Range.withStep"), vec![]),
        start,
        stop,
        step,
    )
}
/// Build `List.enumFrom start l`.
#[allow(dead_code)]
pub fn mk_list_enum_from(start: Expr, l: Expr) -> Expr {
    app2(Expr::Const(Name::str("List.enumFrom"), vec![]), start, l)
}
/// Build `Range.forM r body`.
#[allow(dead_code)]
pub fn mk_range_for_m(r: Expr, body: Expr) -> Expr {
    app2(Expr::Const(Name::str("Range.forM"), vec![]), r, body)
}
/// Build `Iterator.next state`.
#[allow(dead_code)]
pub fn mk_iterator_next(state: Expr) -> Expr {
    app(Expr::Const(Name::str("Iterator.next"), vec![]), state)
}
/// Build `Iterator.hasNext state`.
#[allow(dead_code)]
pub fn mk_iterator_has_next(state: Expr) -> Expr {
    app(Expr::Const(Name::str("Iterator.hasNext"), vec![]), state)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_env() -> Environment {
        let mut env = Environment::new();
        build_range_env(&mut env).expect("build_range_env should succeed");
        env
    }
    #[test]
    fn test_build_range_env_succeeds() {
        let mut env = Environment::new();
        assert!(build_range_env(&mut env).is_ok());
    }
    #[test]
    fn test_range_type_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range")).is_some());
    }
    #[test]
    fn test_range_mk_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.mk")).is_some());
    }
    #[test]
    fn test_range_start_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.start")).is_some());
    }
    #[test]
    fn test_range_stop_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.stop")).is_some());
    }
    #[test]
    fn test_range_step_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.step")).is_some());
    }
    #[test]
    fn test_range_of_nat_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.ofNat")).is_some());
    }
    #[test]
    fn test_range_single_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.single")).is_some());
    }
    #[test]
    fn test_range_with_step_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.withStep")).is_some());
    }
    #[test]
    fn test_for_in_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("ForIn")).is_some());
    }
    #[test]
    fn test_for_in_method_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("ForIn.forIn")).is_some());
    }
    #[test]
    fn test_inst_for_in_range_nat_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("instForInRangeNat")).is_some());
    }
    #[test]
    fn test_iterator_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Iterator")).is_some());
    }
    #[test]
    fn test_iterator_next_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Iterator.next")).is_some());
    }
    #[test]
    fn test_iterator_has_next_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Iterator.hasNext")).is_some());
    }
    #[test]
    fn test_range_iterator_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("RangeIterator")).is_some());
    }
    #[test]
    fn test_inst_iterator_nat_range_exists() {
        let env = make_env();
        assert!(env
            .get(&Name::str("instIteratorNatRangeIterator"))
            .is_some());
    }
    #[test]
    fn test_range_is_empty_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.isEmpty")).is_some());
    }
    #[test]
    fn test_range_size_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.size")).is_some());
    }
    #[test]
    fn test_range_contains_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.contains")).is_some());
    }
    #[test]
    fn test_range_to_list_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.toList")).is_some());
    }
    #[test]
    fn test_range_for_m_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.forM")).is_some());
    }
    #[test]
    fn test_range_foldl_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.foldl")).is_some());
    }
    #[test]
    fn test_range_all_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.all")).is_some());
    }
    #[test]
    fn test_range_any_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.any")).is_some());
    }
    #[test]
    fn test_array_range_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Array.range")).is_some());
    }
    #[test]
    fn test_list_range_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("List.range")).is_some());
    }
    #[test]
    fn test_list_iota_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("List.iota")).is_some());
    }
    #[test]
    fn test_list_enum_from_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("List.enumFrom")).is_some());
    }
    #[test]
    fn test_range_size_mk_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.size_mk")).is_some());
    }
    #[test]
    fn test_range_contains_iff_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.contains_iff")).is_some());
    }
    #[test]
    fn test_range_to_list_length_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.toList_length")).is_some());
    }
    #[test]
    fn test_range_is_empty_iff_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("Range.isEmpty_iff")).is_some());
    }
    #[test]
    fn test_list_range_length_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("List.range_length")).is_some());
    }
    #[test]
    fn test_list_iota_length_exists() {
        let env = make_env();
        assert!(env.get(&Name::str("List.iota_length")).is_some());
    }
    #[test]
    fn test_mk_range() {
        let r = mk_range(
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
            Expr::Const(Name::str("c"), vec![]),
        );
        assert!(matches!(r, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_range_of_nat() {
        let r = mk_range_of_nat(
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        );
        assert!(matches!(r, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_range_single() {
        let r = mk_range_single(Expr::Const(Name::str("n"), vec![]));
        assert!(matches!(r, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_for_in() {
        let f = mk_for_in(
            Expr::Const(Name::str("c"), vec![]),
            Expr::Const(Name::str("i"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        );
        assert!(matches!(f, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_range_to_list() {
        let l = mk_range_to_list(Expr::Const(Name::str("r"), vec![]));
        assert!(matches!(l, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_range_size() {
        let s = mk_range_size(Expr::Const(Name::str("r"), vec![]));
        assert!(matches!(s, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_list_range() {
        let l = mk_list_range(Expr::Const(Name::str("n"), vec![]));
        assert!(matches!(l, Expr::App(_, _)));
    }
    #[test]
    fn test_all_declarations_are_axioms() {
        let env = make_env();
        for name in [
            "Range",
            "Range.mk",
            "Range.start",
            "Range.stop",
            "Range.step",
            "Range.ofNat",
            "Range.single",
            "Range.withStep",
            "ForIn",
            "ForIn.forIn",
            "instForInRangeNat",
            "Iterator",
            "Iterator.next",
            "Iterator.hasNext",
            "RangeIterator",
            "instIteratorNatRangeIterator",
            "Range.isEmpty",
            "Range.size",
            "Range.contains",
            "Range.toList",
            "Range.forM",
            "Range.foldl",
            "Range.all",
            "Range.any",
            "Array.range",
            "List.range",
            "List.iota",
            "List.enumFrom",
        ] {
            let decl = env.get(&Name::str(name)).expect("operation should succeed");
            assert!(
                matches!(decl, Declaration::Axiom { .. }),
                "{} should be an axiom",
                name
            );
        }
    }
    #[test]
    fn test_env_declaration_count() {
        let env = make_env();
        assert!(env.len() >= 30);
    }
    #[test]
    fn test_range_mk_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Range.mk"))
            .expect("declaration 'Range.mk' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_range_foldl_type_is_pi() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Range.foldl"))
            .expect("declaration 'Range.foldl' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_range_type_is_sort() {
        let env = make_env();
        let decl = env
            .get(&Name::str("Range"))
            .expect("declaration 'Range' should exist in env");
        assert!(decl.ty().is_sort());
    }
    #[test]
    fn test_range_iterator_type_is_sort() {
        let env = make_env();
        let decl = env
            .get(&Name::str("RangeIterator"))
            .expect("declaration 'RangeIterator' should exist in env");
        assert!(decl.ty().is_sort());
    }
}
pub fn rng_ext_type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn rng_ext_type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn rng_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn rng_ext_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(cod),
    )
}
pub fn rng_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn rng_ext_ipi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn rng_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn rng_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    rng_ext_app(rng_ext_app(f, a), b)
}
pub fn rng_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    rng_ext_app(rng_ext_app2(f, a, b), c)
}
pub fn rng_ext_nat() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn rng_ext_float() -> Expr {
    Expr::Const(Name::str("Float"), vec![])
}
pub fn rng_ext_bool() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
pub fn rng_ext_interval() -> Expr {
    Expr::Const(Name::str("Interval"), vec![])
}
pub fn rng_ext_interval_of(ty: Expr) -> Expr {
    rng_ext_app(Expr::Const(Name::str("Interval"), vec![]), ty)
}
pub fn rng_ext_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    rng_ext_app3(Expr::Const(Name::str("Eq"), vec![]), ty, a, b)
}
pub fn rng_ext_and(a: Expr, b: Expr) -> Expr {
    rng_ext_app2(Expr::Const(Name::str("And"), vec![]), a, b)
}
pub fn rng_ext_iff(a: Expr, b: Expr) -> Expr {
    rng_ext_app2(Expr::Const(Name::str("Iff"), vec![]), a, b)
}
pub fn rng_ext_le(a: Expr, b: Expr) -> Expr {
    rng_ext_app2(Expr::Const(Name::str("LE.le"), vec![]), a, b)
}
pub fn rng_ext_lt(a: Expr, b: Expr) -> Expr {
    rng_ext_app2(Expr::Const(Name::str("LT.lt"), vec![]), a, b)
}
pub fn rng_ext_add_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// `Interval : Type → Type` — a closed interval type parameterized by a numeric type.
pub fn axiom_interval_type_ty() -> Expr {
    rng_ext_arrow(rng_ext_type1(), rng_ext_type1())
}
/// `Interval.mk : {α : Type} → α → α → Interval α` — construct from lo and hi.
pub fn axiom_interval_mk_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        rng_ext_pi(
            "lo",
            Expr::BVar(0),
            rng_ext_pi("hi", Expr::BVar(1), rng_ext_interval_of(Expr::BVar(2))),
        ),
    )
}
/// `Interval.lo : {α : Type} → Interval α → α`.
pub fn axiom_interval_lo_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        rng_ext_arrow(rng_ext_interval_of(Expr::BVar(0)), Expr::BVar(0)),
    )
}
/// `Interval.hi : {α : Type} → Interval α → α`.
pub fn axiom_interval_hi_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        rng_ext_arrow(rng_ext_interval_of(Expr::BVar(0)), Expr::BVar(0)),
    )
}
/// `Interval.valid : {α : Type} → [Ord α] → Interval α → Prop` — lo ≤ hi.
pub fn axiom_interval_valid_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(rng_ext_app(
                Expr::Const(Name::str("Ord"), vec![]),
                Expr::BVar(0),
            )),
            Box::new(rng_ext_arrow(
                rng_ext_interval_of(Expr::BVar(1)),
                rng_ext_prop(),
            )),
        ),
    )
}
/// `Interval.contains : {α : Type} → [Ord α] → Interval α → α → Prop`.
pub fn axiom_interval_contains_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(rng_ext_app(
                Expr::Const(Name::str("Ord"), vec![]),
                Expr::BVar(0),
            )),
            Box::new(rng_ext_pi(
                "iv",
                rng_ext_interval_of(Expr::BVar(1)),
                rng_ext_pi("x", Expr::BVar(2), rng_ext_prop()),
            )),
        ),
    )
}
/// `Interval.add : Interval Float → Interval Float → Interval Float` — Moore addition.
pub fn axiom_interval_add_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.sub : Interval Float → Interval Float → Interval Float` — Moore subtraction.
pub fn axiom_interval_sub_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.mul : Interval Float → Interval Float → Interval Float` — Moore multiplication.
pub fn axiom_interval_mul_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.div : Interval Float → Interval Float → Interval Float` — Moore division (partial).
pub fn axiom_interval_div_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.width : Interval Float → Float` — hi - lo.
pub fn axiom_interval_width_ty() -> Expr {
    rng_ext_arrow(rng_ext_interval_of(rng_ext_float()), rng_ext_float())
}
/// `Interval.midpoint : Interval Float → Float` — (lo + hi) / 2.
pub fn axiom_interval_midpoint_ty() -> Expr {
    rng_ext_arrow(rng_ext_interval_of(rng_ext_float()), rng_ext_float())
}
/// `Interval.intersect : Interval Float → Interval Float → Option (Interval Float)`.
pub fn axiom_interval_intersect_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_app(
                Expr::Const(Name::str("Option"), vec![]),
                rng_ext_interval_of(rng_ext_float()),
            ),
        ),
    )
}
/// `Interval.hull : Interval Float → Interval Float → Interval Float` — smallest enclosing interval.
pub fn axiom_interval_hull_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.subset : Interval Float → Interval Float → Prop`.
pub fn axiom_interval_subset_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_arrow(rng_ext_interval_of(rng_ext_float()), rng_ext_prop()),
    )
}
/// `MooreArithmetic.inclusion_monotone : Prop` — inclusion monotonicity of Moore arithmetic.
pub fn axiom_moore_inclusion_monotone_ty() -> Expr {
    rng_ext_prop()
}
/// `MooreArithmetic.fundamental_theorem : Prop` — fundamental theorem of interval arithmetic.
pub fn axiom_moore_fundamental_theorem_ty() -> Expr {
    rng_ext_prop()
}
/// `IntervalNewton.step : (Float → Float) → Interval Float → Interval Float → Interval Float`.
pub fn axiom_interval_newton_step_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_arrow(rng_ext_float(), rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_arrow(
                rng_ext_interval_of(rng_ext_float()),
                rng_ext_interval_of(rng_ext_float()),
            ),
        ),
    )
}
/// `IntervalNewton.converges : Prop` — the interval Newton method converges quadratically.
pub fn axiom_interval_newton_converges_ty() -> Expr {
    rng_ext_prop()
}
/// `Krawczyk.operator : (Float → Float) → Interval Float → Interval Float`.
pub fn axiom_krawczyk_operator_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_arrow(rng_ext_float(), rng_ext_float()),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Krawczyk.enclosure_theorem : Prop` — Krawczyk's theorem for root enclosure.
pub fn axiom_krawczyk_enclosure_ty() -> Expr {
    rng_ext_prop()
}
/// `GaussSeidel.interval_step : Nat → Interval Float → Interval Float`.
pub fn axiom_gauss_seidel_interval_step_ty() -> Expr {
    rng_ext_pi(
        "n",
        rng_ext_nat(),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `ValidatedNumerics.enclosure : (Float → Float) → Interval Float → Prop`.
pub fn axiom_validated_numerics_enclosure_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_arrow(rng_ext_float(), rng_ext_float()),
        rng_ext_arrow(rng_ext_interval_of(rng_ext_float()), rng_ext_prop()),
    )
}
/// `IntervalPresheaf : Type 2` — intervals as a presheaf on the reals.
pub fn axiom_interval_presheaf_ty() -> Expr {
    rng_ext_type2()
}
/// `IntervalValuedProbability : Type` — interval-valued probability measure.
pub fn axiom_interval_valued_probability_ty() -> Expr {
    rng_ext_type1()
}
/// `IntervalValuedProbability.measure : IntervalValuedProbability → Interval Float`.
pub fn axiom_ivp_measure_ty() -> Expr {
    rng_ext_arrow(
        Expr::Const(Name::str("IntervalValuedProbability"), vec![]),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `ModalInterval : Type` — modal interval arithmetic (proper and improper intervals).
pub fn axiom_modal_interval_ty() -> Expr {
    rng_ext_type1()
}
/// `ModalInterval.dual : ModalInterval → ModalInterval` — dual of a modal interval.
pub fn axiom_modal_interval_dual_ty() -> Expr {
    rng_ext_arrow(
        Expr::Const(Name::str("ModalInterval"), vec![]),
        Expr::Const(Name::str("ModalInterval"), vec![]),
    )
}
/// `IntervalLinearSystem.solve : Nat → (Interval Float) → Option (Interval Float)`.
pub fn axiom_interval_linear_system_solve_ty() -> Expr {
    rng_ext_pi(
        "n",
        rng_ext_nat(),
        rng_ext_arrow(
            rng_ext_interval_of(rng_ext_float()),
            rng_ext_app(
                Expr::Const(Name::str("Option"), vec![]),
                rng_ext_interval_of(rng_ext_float()),
            ),
        ),
    )
}
/// `DempsterShafer.belief : DempsterShafer → Interval Float`.
pub fn axiom_dempster_shafer_belief_ty() -> Expr {
    rng_ext_arrow(
        Expr::Const(Name::str("DempsterShafer"), vec![]),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `DempsterShafer.plausibility : DempsterShafer → Interval Float`.
pub fn axiom_dempster_shafer_plausibility_ty() -> Expr {
    rng_ext_arrow(
        Expr::Const(Name::str("DempsterShafer"), vec![]),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `FuzzyInterval : Type` — fuzzy interval for soft membership.
pub fn axiom_fuzzy_interval_ty() -> Expr {
    rng_ext_type1()
}
/// `FuzzyInterval.alpha_cut : FuzzyInterval → Float → Interval Float`.
pub fn axiom_fuzzy_interval_alpha_cut_ty() -> Expr {
    rng_ext_arrow(
        Expr::Const(Name::str("FuzzyInterval"), vec![]),
        rng_ext_arrow(rng_ext_float(), rng_ext_interval_of(rng_ext_float())),
    )
}
/// `SegmentTree : Type → Nat → Type` — segment tree for interval range queries.
pub fn axiom_segment_tree_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_type1(),
        rng_ext_arrow(rng_ext_nat(), rng_ext_type1()),
    )
}
/// `SegmentTree.query : {α : Type} → {n : Nat} → SegmentTree α n → Nat → Nat → α`.
pub fn axiom_segment_tree_query_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        rng_ext_ipi(
            "n",
            rng_ext_nat(),
            rng_ext_pi(
                "tree",
                rng_ext_app2(
                    Expr::Const(Name::str("SegmentTree"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                rng_ext_pi(
                    "l",
                    rng_ext_nat(),
                    rng_ext_pi("r", rng_ext_nat(), Expr::BVar(4)),
                ),
            ),
        ),
    )
}
/// `SegmentTree.update : {α : Type} → {n : Nat} → SegmentTree α n → Nat → α → SegmentTree α n`.
pub fn axiom_segment_tree_update_ty() -> Expr {
    rng_ext_ipi(
        "α",
        rng_ext_type1(),
        rng_ext_ipi(
            "n",
            rng_ext_nat(),
            rng_ext_pi(
                "tree",
                rng_ext_app2(
                    Expr::Const(Name::str("SegmentTree"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                rng_ext_pi(
                    "i",
                    rng_ext_nat(),
                    rng_ext_pi(
                        "val",
                        Expr::BVar(3),
                        rng_ext_app2(
                            Expr::Const(Name::str("SegmentTree"), vec![]),
                            Expr::BVar(4),
                            Expr::BVar(3),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `IntervalScheduling.WeightedJob : Type` — weighted job for interval scheduling.
pub fn axiom_weighted_job_ty() -> Expr {
    rng_ext_type1()
}
/// `IntervalScheduling.optimalSchedule : List WeightedJob → List WeightedJob`.
pub fn axiom_interval_scheduling_optimal_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_app(
            Expr::Const(Name::str("List"), vec![]),
            Expr::Const(Name::str("WeightedJob"), vec![]),
        ),
        rng_ext_app(
            Expr::Const(Name::str("List"), vec![]),
            Expr::Const(Name::str("WeightedJob"), vec![]),
        ),
    )
}
/// `IntervalScheduling.dpCorrect : Prop` — DP solution correctness for WIS.
pub fn axiom_interval_scheduling_dp_correct_ty() -> Expr {
    rng_ext_prop()
}
/// `Range.foldl_correct : Prop` — foldl over ranges is correct with respect to List.foldl.
pub fn axiom_range_foldl_correct_ty() -> Expr {
    rng_ext_prop()
}
/// `Range.all_iff : Prop` — `Range.all r p = true ↔ ∀ n ∈ r, p n = true`.
pub fn axiom_range_all_iff_ty() -> Expr {
    rng_ext_prop()
}
/// `Range.any_iff : Prop` — `Range.any r p = true ↔ ∃ n ∈ r, p n = true`.
pub fn axiom_range_any_iff_ty() -> Expr {
    rng_ext_prop()
}
/// `IntervalArithmetic.correctRounding : Prop` — interval operations round outward correctly.
pub fn axiom_interval_correct_rounding_ty() -> Expr {
    rng_ext_prop()
}
/// `IntervalArithmetic.subtraction_anticlosure : Prop` — subtraction may expand intervals.
pub fn axiom_interval_subtraction_anticlosure_ty() -> Expr {
    rng_ext_prop()
}
/// `Interval.pow : Interval Float → Nat → Interval Float`.
pub fn axiom_interval_pow_ty() -> Expr {
    rng_ext_pi(
        "iv",
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_pi("n", rng_ext_nat(), rng_ext_interval_of(rng_ext_float())),
    )
}
/// `Interval.sqrt : Interval Float → Option (Interval Float)`.
pub fn axiom_interval_sqrt_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_app(
            Expr::Const(Name::str("Option"), vec![]),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.exp : Interval Float → Interval Float`.
pub fn axiom_interval_exp_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `Interval.log : Interval Float → Option (Interval Float)` — log on positive intervals.
pub fn axiom_interval_log_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_app(
            Expr::Const(Name::str("Option"), vec![]),
            rng_ext_interval_of(rng_ext_float()),
        ),
    )
}
/// `Interval.sin : Interval Float → Interval Float`.
pub fn axiom_interval_sin_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `Interval.cos : Interval Float → Interval Float`.
pub fn axiom_interval_cos_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `Interval.abs : Interval Float → Interval Float`.
pub fn axiom_interval_abs_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// `Interval.neg : Interval Float → Interval Float`.
pub fn axiom_interval_neg_ty() -> Expr {
    rng_ext_arrow(
        rng_ext_interval_of(rng_ext_float()),
        rng_ext_interval_of(rng_ext_float()),
    )
}
/// Register all extended Range/Interval axioms into the environment.
pub fn register_range_extended(env: &mut Environment) -> Result<(), String> {
    rng_ext_add_axiom(env, "Interval", axiom_interval_type_ty())?;
    rng_ext_add_axiom(env, "Interval.mk", axiom_interval_mk_ty())?;
    rng_ext_add_axiom(env, "Interval.lo", axiom_interval_lo_ty())?;
    rng_ext_add_axiom(env, "Interval.hi", axiom_interval_hi_ty())?;
    rng_ext_add_axiom(env, "Interval.valid", axiom_interval_valid_ty())?;
    rng_ext_add_axiom(env, "Interval.contains", axiom_interval_contains_ty())?;
    rng_ext_add_axiom(env, "Interval.add", axiom_interval_add_ty())?;
    rng_ext_add_axiom(env, "Interval.sub", axiom_interval_sub_ty())?;
    rng_ext_add_axiom(env, "Interval.mul", axiom_interval_mul_ty())?;
    rng_ext_add_axiom(env, "Interval.div", axiom_interval_div_ty())?;
    rng_ext_add_axiom(env, "Interval.width", axiom_interval_width_ty())?;
    rng_ext_add_axiom(env, "Interval.midpoint", axiom_interval_midpoint_ty())?;
    rng_ext_add_axiom(env, "Interval.intersect", axiom_interval_intersect_ty())?;
    rng_ext_add_axiom(env, "Interval.hull", axiom_interval_hull_ty())?;
    rng_ext_add_axiom(env, "Interval.subset", axiom_interval_subset_ty())?;
    rng_ext_add_axiom(
        env,
        "MooreArithmetic.inclusion_monotone",
        axiom_moore_inclusion_monotone_ty(),
    )?;
    rng_ext_add_axiom(
        env,
        "MooreArithmetic.fundamental_theorem",
        axiom_moore_fundamental_theorem_ty(),
    )?;
    rng_ext_add_axiom(env, "IntervalNewton.step", axiom_interval_newton_step_ty())?;
    rng_ext_add_axiom(
        env,
        "IntervalNewton.converges",
        axiom_interval_newton_converges_ty(),
    )?;
    rng_ext_add_axiom(env, "Krawczyk.operator", axiom_krawczyk_operator_ty())?;
    rng_ext_add_axiom(
        env,
        "Krawczyk.enclosure_theorem",
        axiom_krawczyk_enclosure_ty(),
    )?;
    rng_ext_add_axiom(
        env,
        "ValidatedNumerics.enclosure",
        axiom_validated_numerics_enclosure_ty(),
    )?;
    rng_ext_add_axiom(env, "IntervalPresheaf", axiom_interval_presheaf_ty())?;
    rng_ext_add_axiom(
        env,
        "IntervalValuedProbability",
        axiom_interval_valued_probability_ty(),
    )?;
    rng_ext_add_axiom(
        env,
        "IntervalValuedProbability.measure",
        axiom_ivp_measure_ty(),
    )?;
    rng_ext_add_axiom(env, "ModalInterval", axiom_modal_interval_ty())?;
    rng_ext_add_axiom(env, "ModalInterval.dual", axiom_modal_interval_dual_ty())?;
    rng_ext_add_axiom(env, "FuzzyInterval", axiom_fuzzy_interval_ty())?;
    rng_ext_add_axiom(
        env,
        "FuzzyInterval.alpha_cut",
        axiom_fuzzy_interval_alpha_cut_ty(),
    )?;
    rng_ext_add_axiom(env, "SegmentTree", axiom_segment_tree_ty())?;
    rng_ext_add_axiom(env, "WeightedJob", axiom_weighted_job_ty())?;
    rng_ext_add_axiom(
        env,
        "IntervalScheduling.dpCorrect",
        axiom_interval_scheduling_dp_correct_ty(),
    )?;
    rng_ext_add_axiom(env, "Range.foldl_correct", axiom_range_foldl_correct_ty())?;
    rng_ext_add_axiom(env, "Range.all_iff", axiom_range_all_iff_ty())?;
    rng_ext_add_axiom(env, "Range.any_iff", axiom_range_any_iff_ty())?;
    rng_ext_add_axiom(
        env,
        "IntervalArithmetic.correctRounding",
        axiom_interval_correct_rounding_ty(),
    )?;
    rng_ext_add_axiom(env, "Interval.pow", axiom_interval_pow_ty())?;
    rng_ext_add_axiom(env, "Interval.sqrt", axiom_interval_sqrt_ty())?;
    rng_ext_add_axiom(env, "Interval.exp", axiom_interval_exp_ty())?;
    rng_ext_add_axiom(env, "Interval.log", axiom_interval_log_ty())?;
    rng_ext_add_axiom(env, "Interval.sin", axiom_interval_sin_ty())?;
    rng_ext_add_axiom(env, "Interval.cos", axiom_interval_cos_ty())?;
    rng_ext_add_axiom(env, "Interval.abs", axiom_interval_abs_ty())?;
    rng_ext_add_axiom(env, "Interval.neg", axiom_interval_neg_ty())?;
    Ok(())
}
#[cfg(test)]
mod range_extended_tests {
    use super::*;
    #[test]
    fn test_register_range_extended_succeeds() {
        let mut env = Environment::new();
        build_range_env(&mut env).expect("build_range_env should succeed");
        assert!(register_range_extended(&mut env).is_ok());
    }
    #[test]
    fn test_register_range_extended_axioms_present() {
        let mut env = Environment::new();
        build_range_env(&mut env).expect("build_range_env should succeed");
        register_range_extended(&mut env).expect("unwrap should succeed");
        assert!(env.get(&Name::str("Interval")).is_some());
        assert!(env.get(&Name::str("Interval.add")).is_some());
        assert!(env
            .get(&Name::str("MooreArithmetic.fundamental_theorem"))
            .is_some());
        assert!(env.get(&Name::str("SegmentTree")).is_some());
        assert!(env.get(&Name::str("FuzzyInterval")).is_some());
        assert!(env.get(&Name::str("ModalInterval")).is_some());
    }
    #[test]
    fn test_float_interval_new() {
        let iv = FloatInterval::new(1.0, 3.0);
        assert_eq!(iv.lo(), 1.0);
        assert_eq!(iv.hi(), 3.0);
        assert_eq!(iv.width(), 2.0);
        assert_eq!(iv.midpoint(), 2.0);
    }
    #[test]
    fn test_float_interval_contains() {
        let iv = FloatInterval::new(0.0, 1.0);
        assert!(iv.contains(0.5));
        assert!(iv.contains(0.0));
        assert!(iv.contains(1.0));
        assert!(!iv.contains(-0.1));
        assert!(!iv.contains(1.1));
    }
    #[test]
    fn test_float_interval_add() {
        let a = FloatInterval::new(1.0, 2.0);
        let b = FloatInterval::new(3.0, 4.0);
        let c = a.add(b);
        assert_eq!(c.lo(), 4.0);
        assert_eq!(c.hi(), 6.0);
    }
    #[test]
    fn test_float_interval_sub() {
        let a = FloatInterval::new(1.0, 3.0);
        let b = FloatInterval::new(1.0, 2.0);
        let c = a.sub(b);
        assert_eq!(c.lo(), -1.0);
        assert_eq!(c.hi(), 2.0);
    }
    #[test]
    fn test_float_interval_mul() {
        let a = FloatInterval::new(2.0, 3.0);
        let b = FloatInterval::new(4.0, 5.0);
        let c = a.mul(b);
        assert_eq!(c.lo(), 8.0);
        assert_eq!(c.hi(), 15.0);
    }
    #[test]
    fn test_float_interval_neg() {
        let a = FloatInterval::new(1.0, 3.0);
        let b = a.neg();
        assert_eq!(b.lo(), -3.0);
        assert_eq!(b.hi(), -1.0);
    }
    #[test]
    fn test_float_interval_intersect() {
        let a = FloatInterval::new(1.0, 5.0);
        let b = FloatInterval::new(3.0, 8.0);
        let c = a.intersect(b).expect("intersect should succeed");
        assert_eq!(c.lo(), 3.0);
        assert_eq!(c.hi(), 5.0);
    }
    #[test]
    fn test_float_interval_disjoint() {
        let a = FloatInterval::new(1.0, 2.0);
        let b = FloatInterval::new(3.0, 4.0);
        assert!(a.intersect(b).is_none());
    }
    #[test]
    fn test_float_interval_hull() {
        let a = FloatInterval::new(1.0, 3.0);
        let b = FloatInterval::new(2.0, 5.0);
        let h = a.hull(b);
        assert_eq!(h.lo(), 1.0);
        assert_eq!(h.hi(), 5.0);
    }
    #[test]
    fn test_float_interval_abs_positive() {
        let a = FloatInterval::new(2.0, 5.0);
        let b = a.abs();
        assert_eq!(b.lo(), 2.0);
        assert_eq!(b.hi(), 5.0);
    }
    #[test]
    fn test_float_interval_abs_negative() {
        let a = FloatInterval::new(-5.0, -2.0);
        let b = a.abs();
        assert_eq!(b.lo(), 2.0);
        assert_eq!(b.hi(), 5.0);
    }
    #[test]
    fn test_float_interval_abs_mixed() {
        let a = FloatInterval::new(-3.0, 5.0);
        let b = a.abs();
        assert_eq!(b.lo(), 0.0);
        assert_eq!(b.hi(), 5.0);
    }
    #[test]
    fn test_float_interval_subset() {
        let a = FloatInterval::new(2.0, 4.0);
        let b = FloatInterval::new(1.0, 5.0);
        assert!(a.is_subset_of(b));
        assert!(!b.is_subset_of(a));
    }
    #[test]
    fn test_scheduled_job_overlaps() {
        let j1 = ScheduledJob::new(0, 0, 5, 1);
        let j2 = ScheduledJob::new(1, 3, 8, 1);
        let j3 = ScheduledJob::new(2, 5, 10, 1);
        assert!(j1.overlaps(&j2));
        assert!(!j1.overlaps(&j3));
    }
    #[test]
    fn test_interval_scheduler_basic() {
        let jobs = vec![
            ScheduledJob::new(0, 0, 3, 3),
            ScheduledJob::new(1, 1, 4, 2),
            ScheduledJob::new(2, 3, 6, 4),
        ];
        let mut sched = IntervalScheduler::new(jobs);
        let w = sched.max_weight_schedule();
        assert!(w >= 4);
    }
    #[test]
    fn test_interval_scheduler_no_overlap() {
        let jobs = vec![
            ScheduledJob::new(0, 0, 2, 1),
            ScheduledJob::new(1, 2, 4, 2),
            ScheduledJob::new(2, 4, 6, 3),
        ];
        let mut sched = IntervalScheduler::new(jobs);
        let w = sched.max_weight_schedule();
        assert_eq!(w, 6);
    }
    #[test]
    fn test_seg_tree_build_query() {
        let values = vec![5i64, 3, 7, 1, 9, 2];
        let tree = RangeMinSegTree::build(&values);
        assert_eq!(tree.query_min(0, 5), Some(1));
        assert_eq!(tree.query_min(0, 2), Some(3));
        assert_eq!(tree.query_min(3, 5), Some(1));
    }
    #[test]
    fn test_seg_tree_update() {
        let values = vec![5i64, 3, 7, 1, 9, 2];
        let mut tree = RangeMinSegTree::build(&values);
        tree.update(3, 10);
        assert_eq!(tree.query_min(0, 5), Some(2));
    }
    #[test]
    fn test_seg_tree_empty() {
        let tree = RangeMinSegTree::build(&[]);
        assert!(tree.is_empty());
        assert_eq!(tree.query_min(0, 0), None);
    }
    #[test]
    fn test_krawczyk_finds_root_of_linear() {
        let solver = KrawczykSolver::new(50, 1e-10);
        let x0 = FloatInterval::new(1.0, 3.0);
        let result = solver.solve(x0, |x| x - 2.0, |_iv| FloatInterval::new(1.0, 1.0));
        assert!(result.is_some());
        let root_iv = result.expect("result should be valid");
        assert!(root_iv.contains(2.0));
    }
    #[test]
    fn test_validated_interval_verified() {
        let iv = FloatInterval::new(1.0, 2.0);
        let vi = ValidatedInterval::verified(iv, "test");
        assert!(vi.is_verified());
        assert_eq!(vi.certificate(), "test");
    }
    #[test]
    fn test_validated_interval_add() {
        let a = ValidatedInterval::verified(FloatInterval::new(1.0, 2.0), "a");
        let b = ValidatedInterval::verified(FloatInterval::new(3.0, 4.0), "b");
        let c = a.add(&b);
        assert!(c.is_verified());
        assert_eq!(c.interval().lo(), 4.0);
        assert_eq!(c.interval().hi(), 6.0);
    }
    #[test]
    fn test_validated_interval_unverified_propagates() {
        let a = ValidatedInterval::verified(FloatInterval::new(1.0, 2.0), "a");
        let b = ValidatedInterval::unverified(FloatInterval::new(3.0, 4.0));
        let c = a.add(&b);
        assert!(!c.is_verified());
    }
    #[test]
    fn test_axiom_type_builders_interval() {
        let _ = axiom_interval_type_ty();
        let _ = axiom_interval_mk_ty();
        let _ = axiom_interval_add_ty();
        let _ = axiom_interval_mul_ty();
        let _ = axiom_interval_width_ty();
        let _ = axiom_interval_midpoint_ty();
        let _ = axiom_interval_newton_step_ty();
        let _ = axiom_krawczyk_operator_ty();
        let _ = axiom_segment_tree_query_ty();
        let _ = axiom_interval_scheduling_optimal_ty();
    }
}
