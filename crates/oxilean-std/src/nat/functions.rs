//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, Level, Literal, Name,
};

use super::types::{ArithmeticFunctions, CollatzUtil, FibonacciUtil};

/// The Nat type expression: `Const("Nat", [])`.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
/// The Bool type expression: `Const("Bool", [])`.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
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
/// Build `Nat -> Nat -> Nat`.
#[allow(dead_code)]
pub fn nat_binop_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Build `Nat -> Nat -> Bool`.
#[allow(dead_code)]
pub fn nat_to_nat_to_bool() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), bool_ty()))
}
/// Build `Nat -> Nat -> Prop`.
#[allow(dead_code)]
pub fn nat_to_nat_to_prop() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// Build `Eq @{} Nat a b` (propositional equality on Nat).
#[allow(dead_code)]
pub fn eq_nat(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(nat_ty()),
            )),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Build a function application `f a`.
#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build a function application `f a b`.
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
/// Build `Nat.add a b`.
#[allow(dead_code)]
pub fn add_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.add"), vec![]), a, b)
}
/// Build `Nat.mul a b`.
#[allow(dead_code)]
pub fn mul_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.mul"), vec![]), a, b)
}
/// Build `Nat.sub a b`.
#[allow(dead_code)]
pub fn sub_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.sub"), vec![]), a, b)
}
/// Build `Nat.div a b`.
#[allow(dead_code)]
pub fn div_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.div"), vec![]), a, b)
}
/// Build `Nat.mod a b`.
#[allow(dead_code)]
pub fn mod_nat(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.mod"), vec![]), a, b)
}
/// Iff applied to two propositions.
#[allow(dead_code)]
pub fn iff(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Iff"), vec![]), a, b)
}
/// Build `forall (name : Nat), body`.
///
/// This is a Pi type with domain Nat.
/// Inside `body`, `BVar(0)` refers to the bound variable.
#[allow(dead_code)]
pub fn forall_nat(name: &str, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(nat_ty()),
        Box::new(body),
    )
}
/// Build an implication `a -> b` (non-dependent Pi).
#[allow(dead_code)]
pub fn implies(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Build the Nat type and all standard declarations, adding them to the
/// environment.
///
/// Requires that `Bool`, `Eq`, and `Iff` are already declared in `env`
/// for full functionality. If they are not present, the declarations that
/// reference them are still added — they simply reference those names
/// as `Const`.
pub fn build_nat_env(env: &mut Environment, _ind_env: &mut InductiveEnv) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("Nat"),
        univ_params: vec![],
        ty: type1(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.zero"),
        univ_params: vec![],
        ty: nat_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.succ"),
        univ_params: vec![],
        ty: arrow(nat_ty(), nat_ty()),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.add"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.mul"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.sub"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.pow"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.div"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.mod"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.gcd"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.beq"),
        univ_params: vec![],
        ty: nat_to_nat_to_bool(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.ble"),
        univ_params: vec![],
        ty: nat_to_nat_to_bool(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.le"),
        univ_params: vec![],
        ty: nat_to_nat_to_prop(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.lt"),
        univ_params: vec![],
        ty: nat_to_nat_to_prop(),
    })
    .map_err(|e| e.to_string())?;
    let u = Name::str("u");
    let sort_u = Expr::Sort(Level::Param(u.clone()));
    let c_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(nat_ty()),
        Box::new(sort_u),
    );
    let zero_case_ty = app(Expr::BVar(0), Expr::Const(Name::str("Nat.zero"), vec![]));
    let c_n_for_ih = app(Expr::BVar(2), Expr::BVar(0));
    let c_succ_n = app(
        Expr::BVar(3),
        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(1)),
    );
    let succ_case_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_ty()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("ih"),
            Box::new(c_n_for_ih),
            Box::new(c_succ_n),
        )),
    );
    let result_ty = app(Expr::BVar(3), Expr::BVar(0));
    let target = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_ty()),
        Box::new(result_ty),
    );
    let rec_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("C"),
        Box::new(c_ty),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("hz"),
            Box::new(zero_case_ty),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("hs"),
                Box::new(succ_case_ty),
                Box::new(target),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.rec"),
        univ_params: vec![u],
        ty: rec_ty,
    })
    .map_err(|e| e.to_string())?;
    macro_rules! add_axiom {
        ($name:expr, $ty:expr) => {
            env.add(Declaration::Axiom {
                name: Name::str($name),
                univ_params: vec![],
                ty: $ty,
            })
            .map_err(|e| e.to_string())?;
        };
    }
    add_axiom!(
        "Nat.zero_add",
        forall_nat(
            "n",
            eq_nat(
                add_nat(Expr::Const(Name::str("Nat.zero"), vec![]), Expr::BVar(0)),
                Expr::BVar(0),
            )
        )
    );
    add_axiom!(
        "Nat.add_zero",
        forall_nat(
            "n",
            eq_nat(
                add_nat(Expr::BVar(0), Expr::Const(Name::str("Nat.zero"), vec![])),
                Expr::BVar(0),
            )
        )
    );
    add_axiom!(
        "Nat.succ_add",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    add_nat(
                        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(1)),
                        Expr::BVar(0),
                    ),
                    app(
                        Expr::Const(Name::str("Nat.succ"), vec![]),
                        add_nat(Expr::BVar(1), Expr::BVar(0)),
                    ),
                )
            )
        )
    );
    add_axiom!(
        "Nat.add_succ",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    add_nat(
                        Expr::BVar(1),
                        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(0)),
                    ),
                    app(
                        Expr::Const(Name::str("Nat.succ"), vec![]),
                        add_nat(Expr::BVar(1), Expr::BVar(0)),
                    ),
                )
            )
        )
    );
    add_axiom!(
        "Nat.add_comm",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    add_nat(Expr::BVar(1), Expr::BVar(0)),
                    add_nat(Expr::BVar(0), Expr::BVar(1)),
                )
            )
        )
    );
    add_axiom!(
        "Nat.add_assoc",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    eq_nat(
                        add_nat(add_nat(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                        add_nat(Expr::BVar(2), add_nat(Expr::BVar(1), Expr::BVar(0))),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.mul_zero",
        forall_nat(
            "n",
            eq_nat(
                mul_nat(Expr::BVar(0), Expr::Const(Name::str("Nat.zero"), vec![])),
                Expr::Const(Name::str("Nat.zero"), vec![]),
            )
        )
    );
    add_axiom!(
        "Nat.zero_mul",
        forall_nat(
            "n",
            eq_nat(
                mul_nat(Expr::Const(Name::str("Nat.zero"), vec![]), Expr::BVar(0)),
                Expr::Const(Name::str("Nat.zero"), vec![]),
            )
        )
    );
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.mul_one",
            forall_nat("n", eq_nat(mul_nat(Expr::BVar(0), one), Expr::BVar(0)))
        );
    }
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.one_mul",
            forall_nat("n", eq_nat(mul_nat(one, Expr::BVar(0)), Expr::BVar(0)))
        );
    }
    add_axiom!(
        "Nat.mul_comm",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    mul_nat(Expr::BVar(1), Expr::BVar(0)),
                    mul_nat(Expr::BVar(0), Expr::BVar(1)),
                )
            )
        )
    );
    add_axiom!(
        "Nat.mul_assoc",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    eq_nat(
                        mul_nat(mul_nat(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                        mul_nat(Expr::BVar(2), mul_nat(Expr::BVar(1), Expr::BVar(0))),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.left_distrib",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    eq_nat(
                        mul_nat(Expr::BVar(2), add_nat(Expr::BVar(1), Expr::BVar(0))),
                        add_nat(
                            mul_nat(Expr::BVar(2), Expr::BVar(1)),
                            mul_nat(Expr::BVar(2), Expr::BVar(0)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.right_distrib",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    eq_nat(
                        mul_nat(add_nat(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(2)),
                        add_nat(
                            mul_nat(Expr::BVar(1), Expr::BVar(2)),
                            mul_nat(Expr::BVar(0), Expr::BVar(2)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.le_refl",
        forall_nat("n", le_nat(Expr::BVar(0), Expr::BVar(0)))
    );
    add_axiom!(
        "Nat.le_trans",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    implies(
                        le_nat(Expr::BVar(2), Expr::BVar(1)),
                        implies(
                            le_nat(Expr::BVar(2), Expr::BVar(1)),
                            le_nat(Expr::BVar(4), Expr::BVar(2)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.le_antisymm",
        forall_nat(
            "n",
            forall_nat(
                "m",
                implies(
                    le_nat(Expr::BVar(1), Expr::BVar(0)),
                    implies(
                        le_nat(Expr::BVar(1), Expr::BVar(2)),
                        eq_nat(Expr::BVar(3), Expr::BVar(2)),
                    ),
                )
            )
        )
    );
    add_axiom!(
        "Nat.zero_le",
        forall_nat(
            "n",
            le_nat(Expr::Const(Name::str("Nat.zero"), vec![]), Expr::BVar(0))
        )
    );
    add_axiom!(
        "Nat.succ_le_succ",
        forall_nat(
            "n",
            forall_nat(
                "m",
                implies(
                    le_nat(Expr::BVar(1), Expr::BVar(0)),
                    le_nat(
                        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(2)),
                        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(1)),
                    ),
                )
            )
        )
    );
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.lt_iff_add_one_le",
            forall_nat(
                "n",
                forall_nat(
                    "m",
                    iff(
                        lt_nat(Expr::BVar(1), Expr::BVar(0)),
                        le_nat(add_nat(Expr::BVar(1), one), Expr::BVar(0)),
                    )
                )
            )
        );
    }
    add_axiom!(
        "Nat.sub_self",
        forall_nat(
            "n",
            eq_nat(
                sub_nat(Expr::BVar(0), Expr::BVar(0)),
                Expr::Const(Name::str("Nat.zero"), vec![]),
            )
        )
    );
    add_axiom!(
        "Nat.add_sub_cancel",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    sub_nat(add_nat(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(0)),
                    Expr::BVar(1),
                )
            )
        )
    );
    add_axiom!(
        "Nat.div_add_mod",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    add_nat(
                        mul_nat(Expr::BVar(0), div_nat(Expr::BVar(1), Expr::BVar(0))),
                        mod_nat(Expr::BVar(1), Expr::BVar(0)),
                    ),
                    Expr::BVar(1),
                )
            )
        )
    );
    add_axiom!(
        "Nat.div_mul",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    div_nat(mul_nat(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(0)),
                    Expr::BVar(1),
                )
            )
        )
    );
    add_axiom!(
        "Nat.mod_eq_zero_of_dvd",
        forall_nat(
            "n",
            forall_nat(
                "m",
                implies(
                    app2(
                        Expr::Const(Name::str("Nat.dvd"), vec![]),
                        Expr::BVar(0),
                        Expr::BVar(1)
                    ),
                    eq_nat(
                        mod_nat(Expr::BVar(1), Expr::BVar(0)),
                        Expr::Const(Name::str("Nat.zero"), vec![]),
                    ),
                )
            )
        )
    );
    add_axiom!(
        "Nat.mod_add_mod",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    eq_nat(
                        mod_nat(add_nat(Expr::BVar(2), Expr::BVar(1)), Expr::BVar(0)),
                        mod_nat(
                            add_nat(
                                mod_nat(Expr::BVar(2), Expr::BVar(0)),
                                mod_nat(Expr::BVar(1), Expr::BVar(0)),
                            ),
                            Expr::BVar(0),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.mod_lt",
        forall_nat(
            "n",
            forall_nat(
                "m",
                implies(
                    lt_nat(Expr::Const(Name::str("Nat.zero"), vec![]), Expr::BVar(0)),
                    lt_nat(mod_nat(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(0),),
                )
            )
        )
    );
    add_axiom!(
        "Nat.gcd_comm",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.gcd"), vec![]),
                        Expr::BVar(1),
                        Expr::BVar(0)
                    ),
                    app2(
                        Expr::Const(Name::str("Nat.gcd"), vec![]),
                        Expr::BVar(0),
                        Expr::BVar(1)
                    ),
                )
            )
        )
    );
    add_axiom!(
        "Nat.gcd_zero_right",
        forall_nat(
            "n",
            eq_nat(
                app2(
                    Expr::Const(Name::str("Nat.gcd"), vec![]),
                    Expr::BVar(0),
                    Expr::Const(Name::str("Nat.zero"), vec![]),
                ),
                Expr::BVar(0),
            ),
        )
    );
    add_axiom!(
        "Nat.gcd_zero_left",
        forall_nat(
            "n",
            eq_nat(
                app2(
                    Expr::Const(Name::str("Nat.gcd"), vec![]),
                    Expr::Const(Name::str("Nat.zero"), vec![]),
                    Expr::BVar(0),
                ),
                Expr::BVar(0),
            ),
        )
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.lcm"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    add_axiom!(
        "Nat.lcm_comm",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.lcm"), vec![]),
                        Expr::BVar(1),
                        Expr::BVar(0)
                    ),
                    app2(
                        Expr::Const(Name::str("Nat.lcm"), vec![]),
                        Expr::BVar(0),
                        Expr::BVar(1)
                    ),
                )
            )
        )
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.coprime"),
        univ_params: vec![],
        ty: nat_to_nat_to_prop(),
    })
    .map_err(|e| e.to_string())?;
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.coprime_iff_gcd_eq_one",
            forall_nat(
                "n",
                forall_nat(
                    "m",
                    iff(
                        app2(
                            Expr::Const(Name::str("Nat.coprime"), vec![]),
                            Expr::BVar(1),
                            Expr::BVar(0)
                        ),
                        eq_nat(
                            app2(
                                Expr::Const(Name::str("Nat.gcd"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0)
                            ),
                            one,
                        ),
                    )
                )
            )
        );
    }
    env.add(Declaration::Axiom {
        name: Name::str("Nat.factorial"),
        univ_params: vec![],
        ty: arrow(nat_ty(), nat_ty()),
    })
    .map_err(|e| e.to_string())?;
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.factorial_zero",
            eq_nat(
                app(
                    Expr::Const(Name::str("Nat.factorial"), vec![]),
                    Expr::Const(Name::str("Nat.zero"), vec![])
                ),
                one,
            )
        );
    }
    add_axiom!(
        "Nat.factorial_succ",
        forall_nat(
            "n",
            eq_nat(
                app(
                    Expr::Const(Name::str("Nat.factorial"), vec![]),
                    app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(0)),
                ),
                mul_nat(
                    app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(0)),
                    app(
                        Expr::Const(Name::str("Nat.factorial"), vec![]),
                        Expr::BVar(0)
                    ),
                ),
            )
        )
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.choose"),
        univ_params: vec![],
        ty: nat_binop_ty(),
    })
    .map_err(|e| e.to_string())?;
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.choose_zero_right",
            forall_nat(
                "n",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.choose"), vec![]),
                        Expr::BVar(0),
                        Expr::Const(Name::str("Nat.zero"), vec![]),
                    ),
                    one,
                )
            )
        );
    }
    add_axiom!(
        "Nat.choose_zero_left",
        forall_nat(
            "k",
            implies(
                lt_nat(Expr::Const(Name::str("Nat.zero"), vec![]), Expr::BVar(0)),
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.choose"), vec![]),
                        Expr::Const(Name::str("Nat.zero"), vec![]),
                        Expr::BVar(0),
                    ),
                    Expr::Const(Name::str("Nat.zero"), vec![]),
                )
            )
        )
    );
    add_axiom!(
        "Nat.choose_symm",
        forall_nat(
            "n",
            forall_nat(
                "k",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.choose"), vec![]),
                        Expr::BVar(1),
                        Expr::BVar(0)
                    ),
                    app2(
                        Expr::Const(Name::str("Nat.choose"), vec![]),
                        Expr::BVar(1),
                        sub_nat(Expr::BVar(1), Expr::BVar(0)),
                    ),
                )
            )
        )
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.Prime"),
        univ_params: vec![],
        ty: arrow(nat_ty(), prop()),
    })
    .map_err(|e| e.to_string())?;
    add_axiom!(
        "Nat.Prime.eq_one_or_self_of_dvd",
        forall_nat(
            "p",
            forall_nat(
                "d",
                implies(
                    app(Expr::Const(Name::str("Nat.Prime"), vec![]), Expr::BVar(1)),
                    implies(
                        app2(
                            Expr::Const(Name::str("Nat.dvd"), vec![]),
                            Expr::BVar(0),
                            Expr::BVar(1)
                        ),
                        app2(
                            Expr::Const(Name::str("Or"), vec![]),
                            eq_nat(
                                Expr::BVar(0),
                                app(
                                    Expr::Const(Name::str("Nat.succ"), vec![]),
                                    Expr::Const(Name::str("Nat.zero"), vec![]),
                                ),
                            ),
                            eq_nat(Expr::BVar(0), Expr::BVar(1)),
                        ),
                    )
                )
            )
        )
    );
    env.add(Declaration::Axiom {
        name: Name::str("Nat.Decidable.eq"),
        univ_params: vec![],
        ty: forall_nat(
            "n",
            forall_nat(
                "m",
                app(
                    Expr::Const(Name::str("Decidable"), vec![]),
                    eq_nat(Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.Decidable.le"),
        univ_params: vec![],
        ty: forall_nat(
            "n",
            forall_nat(
                "m",
                app(
                    Expr::Const(Name::str("Decidable"), vec![]),
                    le_nat(Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Nat.Decidable.lt"),
        univ_params: vec![],
        ty: forall_nat(
            "n",
            forall_nat(
                "m",
                app(
                    Expr::Const(Name::str("Decidable"), vec![]),
                    lt_nat(Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    })
    .map_err(|e| e.to_string())?;
    add_axiom!(
        "Nat.mul_lt_mul_left",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    implies(
                        lt_nat(Expr::BVar(1), Expr::BVar(0)),
                        lt_nat(
                            mul_nat(Expr::BVar(2), Expr::BVar(1)),
                            mul_nat(Expr::BVar(2), Expr::BVar(0)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.mul_le_mul_left",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    implies(
                        le_nat(Expr::BVar(1), Expr::BVar(0)),
                        le_nat(
                            mul_nat(Expr::BVar(2), Expr::BVar(1)),
                            mul_nat(Expr::BVar(2), Expr::BVar(0)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.pow_succ",
        forall_nat(
            "n",
            forall_nat(
                "m",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.pow"), vec![]),
                        Expr::BVar(1),
                        app(Expr::Const(Name::str("Nat.succ"), vec![]), Expr::BVar(0)),
                    ),
                    mul_nat(
                        Expr::BVar(1),
                        app2(
                            Expr::Const(Name::str("Nat.pow"), vec![]),
                            Expr::BVar(1),
                            Expr::BVar(0)
                        ),
                    ),
                )
            )
        )
    );
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.pow_zero",
            forall_nat(
                "n",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.pow"), vec![]),
                        Expr::BVar(0),
                        Expr::Const(Name::str("Nat.zero"), vec![]),
                    ),
                    one,
                )
            )
        );
    }
    {
        let one = app(
            Expr::Const(Name::str("Nat.succ"), vec![]),
            Expr::Const(Name::str("Nat.zero"), vec![]),
        );
        add_axiom!(
            "Nat.pow_one",
            forall_nat(
                "n",
                eq_nat(
                    app2(
                        Expr::Const(Name::str("Nat.pow"), vec![]),
                        Expr::BVar(0),
                        one
                    ),
                    Expr::BVar(0),
                )
            )
        );
    }
    add_axiom!(
        "Nat.sub_le",
        forall_nat(
            "n",
            forall_nat(
                "m",
                le_nat(sub_nat(Expr::BVar(1), Expr::BVar(0)), Expr::BVar(1),)
            )
        )
    );
    add_axiom!(
        "Nat.sub_mono_right",
        forall_nat(
            "n",
            forall_nat(
                "m",
                forall_nat(
                    "k",
                    implies(
                        le_nat(Expr::BVar(1), Expr::BVar(0)),
                        le_nat(
                            sub_nat(Expr::BVar(2), Expr::BVar(0)),
                            sub_nat(Expr::BVar(2), Expr::BVar(1)),
                        ),
                    )
                )
            )
        )
    );
    add_axiom!(
        "Nat.add_le_add_left",
        forall_nat(
            "k",
            forall_nat(
                "m",
                forall_nat(
                    "n",
                    implies(
                        le_nat(Expr::BVar(1), Expr::BVar(0)),
                        le_nat(
                            add_nat(Expr::BVar(2), Expr::BVar(1)),
                            add_nat(Expr::BVar(2), Expr::BVar(0)),
                        ),
                    )
                )
            )
        )
    );
    Ok(())
}
/// Create a Nat literal expression.
pub fn nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}
/// Create a Nat.succ expression.
pub fn nat_succ(n: Expr) -> Expr {
    app(Expr::Const(Name::str("Nat.succ"), vec![]), n)
}
/// Create a Nat.zero expression.
pub fn nat_zero() -> Expr {
    Expr::Const(Name::str("Nat.zero"), vec![])
}
/// Create a `Nat.add a b` expression.
#[allow(dead_code)]
pub fn nat_add(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.add"), vec![]), a, b)
}
/// Create a `Nat.mul a b` expression.
#[allow(dead_code)]
pub fn nat_mul(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.mul"), vec![]), a, b)
}
/// Create a `Nat.sub a b` expression.
#[allow(dead_code)]
pub fn nat_sub(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.sub"), vec![]), a, b)
}
/// Create a `Nat.pow a b` expression.
#[allow(dead_code)]
pub fn nat_pow(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.pow"), vec![]), a, b)
}
/// Create a `Nat.div a b` expression.
#[allow(dead_code)]
pub fn nat_div(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.div"), vec![]), a, b)
}
/// Create a `Nat.mod a b` expression.
#[allow(dead_code)]
pub fn nat_mod(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.mod"), vec![]), a, b)
}
/// Create a `Nat.gcd a b` expression.
#[allow(dead_code)]
pub fn nat_gcd(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.gcd"), vec![]), a, b)
}
/// Create a `Nat.beq a b` expression.
#[allow(dead_code)]
pub fn nat_beq(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.beq"), vec![]), a, b)
}
/// Create a `Nat.le a b` expression.
#[allow(dead_code)]
pub fn nat_le(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.le"), vec![]), a, b)
}
/// Create a `Nat.lt a b` expression.
#[allow(dead_code)]
pub fn nat_lt(a: Expr, b: Expr) -> Expr {
    app2(Expr::Const(Name::str("Nat.lt"), vec![]), a, b)
}
/// Create a `@Nat.rec C hz hs n` expression.
///
/// - `motive` : the motive `C : Nat -> Sort u`
/// - `zero_case` : proof / value for the zero case
/// - `succ_case` : proof / value for the succ case
/// - `n` : the natural number to recurse on
#[allow(dead_code)]
pub fn nat_rec(motive: Expr, zero_case: Expr, succ_case: Expr, n: Expr) -> Expr {
    let rec = Expr::Const(Name::str("Nat.rec"), vec![]);
    app(app(app(app(rec, motive), zero_case), succ_case), n)
}
/// Create an `Eq Nat a b` expression (propositional equality on Nat).
#[allow(dead_code)]
pub fn mk_nat_eq(a: Expr, b: Expr) -> Expr {
    eq_nat(a, b)
}
/// Create a `Nat.le a b` expression (alias of [`nat_le`]).
#[allow(dead_code)]
pub fn mk_nat_le(a: Expr, b: Expr) -> Expr {
    nat_le(a, b)
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Set up an environment with Bool, Eq, Iff, and Nat.
    fn full_env() -> (Environment, InductiveEnv) {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("true"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Bool"), vec![]),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("false"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Bool"), vec![]),
        })
        .expect("operation should succeed");
        let eq_ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("\u{03b1}"),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Sort(Level::zero())),
                )),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: eq_ty,
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: arrow(prop(), arrow(prop(), prop())),
        })
        .expect("operation should succeed");
        build_nat_env(&mut env, &mut ind_env).expect("build_nat_env should succeed");
        (env, ind_env)
    }
    #[test]
    fn test_build_nat_env() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        assert!(build_nat_env(&mut env, &mut ind_env).is_ok());
        assert!(env.contains(&Name::str("Nat")));
        assert!(env.contains(&Name::str("Nat.zero")));
        assert!(env.contains(&Name::str("Nat.succ")));
    }
    #[test]
    fn test_build_nat_env_full() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat")));
        assert!(env.contains(&Name::str("Nat.zero")));
        assert!(env.contains(&Name::str("Nat.succ")));
        assert!(env.contains(&Name::str("Nat.add")));
        assert!(env.contains(&Name::str("Nat.mul")));
        assert!(env.contains(&Name::str("Nat.sub")));
        assert!(env.contains(&Name::str("Nat.pow")));
        assert!(env.contains(&Name::str("Nat.div")));
        assert!(env.contains(&Name::str("Nat.mod")));
        assert!(env.contains(&Name::str("Nat.gcd")));
        assert!(env.contains(&Name::str("Nat.beq")));
        assert!(env.contains(&Name::str("Nat.ble")));
        assert!(env.contains(&Name::str("Nat.le")));
        assert!(env.contains(&Name::str("Nat.lt")));
        assert!(env.contains(&Name::str("Nat.rec")));
    }
    #[test]
    fn test_nat_env_theorems_present() {
        let (env, _) = full_env();
        let theorem_names = [
            "Nat.zero_add",
            "Nat.add_zero",
            "Nat.succ_add",
            "Nat.add_succ",
            "Nat.add_comm",
            "Nat.add_assoc",
            "Nat.mul_zero",
            "Nat.zero_mul",
            "Nat.mul_one",
            "Nat.one_mul",
            "Nat.mul_comm",
            "Nat.mul_assoc",
            "Nat.left_distrib",
            "Nat.right_distrib",
            "Nat.le_refl",
            "Nat.le_trans",
            "Nat.le_antisymm",
            "Nat.zero_le",
            "Nat.succ_le_succ",
            "Nat.lt_iff_add_one_le",
            "Nat.sub_self",
            "Nat.add_sub_cancel",
            "Nat.div_add_mod",
        ];
        for name in &theorem_names {
            assert!(env.contains(&Name::str(*name)), "missing theorem: {}", name);
        }
    }
    #[test]
    fn test_nat_lit() {
        let expr = nat_lit(42);
        assert_eq!(expr, Expr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_nat_zero() {
        let expr = nat_zero();
        assert!(matches!(expr, Expr::Const(_, _)));
    }
    #[test]
    fn test_nat_succ_of_zero() {
        let expr = nat_succ(nat_zero());
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_add_expr() {
        let a = nat_lit(3);
        let b = nat_lit(4);
        let expr = nat_add(a, b);
        assert!(matches!(expr, Expr::App(_, _)));
        if let Expr::App(f, arg) = &expr {
            assert_eq!(**arg, Expr::Lit(Literal::Nat(4)));
            if let Expr::App(ff, farg) = f.as_ref() {
                assert_eq!(**farg, Expr::Lit(Literal::Nat(3)));
                assert!(matches!(ff.as_ref(), Expr::Const(_, _)));
            } else {
                panic!("expected nested App");
            }
        }
    }
    #[test]
    fn test_nat_mul_expr() {
        let expr = nat_mul(nat_lit(2), nat_lit(5));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_sub_expr() {
        let expr = nat_sub(nat_lit(10), nat_lit(3));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_pow_expr() {
        let expr = nat_pow(nat_lit(2), nat_lit(8));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_div_expr() {
        let expr = nat_div(nat_lit(10), nat_lit(3));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_mod_expr() {
        let expr = nat_mod(nat_lit(10), nat_lit(3));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_gcd_expr() {
        let expr = nat_gcd(nat_lit(12), nat_lit(8));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_beq_expr() {
        let expr = nat_beq(nat_lit(1), nat_lit(2));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_le_expr() {
        let expr = nat_le(nat_lit(0), nat_lit(5));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_lt_expr() {
        let expr = nat_lt(nat_lit(3), nat_lit(7));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_rec_expr() {
        let motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(nat_ty()),
            Box::new(nat_ty()),
        );
        let zero_case = nat_zero();
        let succ_case = Expr::Lam(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("ih"),
                Box::new(nat_ty()),
                Box::new(nat_succ(Expr::BVar(0))),
            )),
        );
        let n = nat_lit(5);
        let expr = nat_rec(motive, zero_case, succ_case, n);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_rec_env_type() {
        let (env, _) = full_env();
        let rec_decl = env
            .get(&Name::str("Nat.rec"))
            .expect("declaration 'Nat.rec' should exist in env");
        assert_eq!(rec_decl.univ_params().len(), 1);
        assert!(rec_decl.ty().is_pi());
    }
    #[test]
    fn test_mk_nat_eq() {
        let e = mk_nat_eq(nat_lit(1), nat_lit(2));
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_nat_le() {
        let e = mk_nat_le(nat_lit(0), nat_lit(10));
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_forall_nat() {
        let body = eq_nat(Expr::BVar(0), Expr::BVar(0));
        let ty = forall_nat("n", body);
        assert!(matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)));
        if let Expr::Pi(_, name, dom, _) = &ty {
            assert_eq!(*name, Name::str("n"));
            assert_eq!(**dom, nat_ty());
        }
    }
    #[test]
    fn test_implies() {
        let a = le_nat(nat_lit(0), nat_lit(1));
        let b = le_nat(nat_lit(0), nat_lit(2));
        let ty = implies(a, b);
        assert!(matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_zero_add_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.zero_add"))
            .expect("declaration 'Nat.zero_add' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_add_comm_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.add_comm"))
            .expect("declaration 'Nat.add_comm' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_add_assoc_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.add_assoc"))
            .expect("declaration 'Nat.add_assoc' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_mul_comm_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.mul_comm"))
            .expect("declaration 'Nat.mul_comm' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_le_trans_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.le_trans"))
            .expect("declaration 'Nat.le_trans' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_left_distrib_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.left_distrib"))
            .expect("declaration 'Nat.left_distrib' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_sub_self_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.sub_self"))
            .expect("declaration 'Nat.sub_self' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_div_add_mod_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.div_add_mod"))
            .expect("declaration 'Nat.div_add_mod' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_nat_add_is_axiom() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.add"))
            .expect("declaration 'Nat.add' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_nat_rec_is_axiom() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.rec"))
            .expect("declaration 'Nat.rec' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_nat_zero_add_is_axiom() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.zero_add"))
            .expect("declaration 'Nat.zero_add' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_nat_add_type_structure() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.add"))
            .expect("declaration 'Nat.add' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, dom, cod) = ty {
            assert_eq!(**dom, nat_ty());
            assert!(cod.is_pi());
            if let Expr::Pi(_, _, dom2, cod2) = cod.as_ref() {
                assert_eq!(**dom2, nat_ty());
                assert_eq!(**cod2, nat_ty());
            }
        }
    }
    #[test]
    fn test_nat_beq_type_structure() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.beq"))
            .expect("declaration 'Nat.beq' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, dom, cod) = ty {
            assert_eq!(**dom, nat_ty());
            assert!(cod.is_pi());
            if let Expr::Pi(_, _, dom2, cod2) = cod.as_ref() {
                assert_eq!(**dom2, nat_ty());
                assert_eq!(**cod2, bool_ty());
            }
        }
    }
    #[test]
    fn test_nat_le_type_structure() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.le"))
            .expect("declaration 'Nat.le' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, dom, cod) = ty {
            assert_eq!(**dom, nat_ty());
            assert!(cod.is_pi());
            if let Expr::Pi(_, _, dom2, cod2) = cod.as_ref() {
                assert_eq!(**dom2, nat_ty());
                assert_eq!(**cod2, prop());
            }
        }
    }
    #[test]
    fn test_complex_nat_expr() {
        let expr = nat_mul(
            nat_add(nat_lit(2), nat_lit(3)),
            nat_sub(nat_lit(4), nat_lit(1)),
        );
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_nested_succ() {
        let three = nat_succ(nat_succ(nat_succ(nat_zero())));
        assert!(matches!(three, Expr::App(_, _)));
    }
    #[test]
    fn test_nat_pow_gcd_combined() {
        let expr = nat_gcd(nat_pow(nat_lit(2), nat_lit(4)), nat_lit(12));
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_equality_expression_symmetry() {
        let e1 = mk_nat_eq(nat_lit(1), nat_lit(2));
        let e2 = mk_nat_eq(nat_lit(2), nat_lit(1));
        assert_ne!(e1, e2);
    }
    #[test]
    fn test_forall_nat_nested() {
        let body = eq_nat(
            add_nat(Expr::BVar(1), Expr::BVar(0)),
            add_nat(Expr::BVar(0), Expr::BVar(1)),
        );
        let ty = forall_nat("n", forall_nat("m", body));
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, _, cod) = &ty {
            assert!(cod.is_pi());
        }
    }
    #[test]
    fn test_implies_chain() {
        let a = le_nat(nat_lit(0), nat_lit(1));
        let b = le_nat(nat_lit(1), nat_lit(2));
        let c = le_nat(nat_lit(0), nat_lit(2));
        let ty = implies(a, implies(b, c));
        assert!(ty.is_pi());
    }
    #[test]
    fn test_env_declaration_count() {
        let (env, _) = full_env();
        assert!(env.len() >= 38);
    }
    #[test]
    fn test_div_mul_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.div_mul")));
    }
    #[test]
    fn test_mod_eq_zero_of_dvd_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.mod_eq_zero_of_dvd")));
    }
    #[test]
    fn test_mod_add_mod_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.mod_add_mod")));
    }
    #[test]
    fn test_mod_lt_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.mod_lt")));
    }
    #[test]
    fn test_gcd_comm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.gcd_comm")));
    }
    #[test]
    fn test_gcd_zero_right_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.gcd_zero_right")));
    }
    #[test]
    fn test_gcd_zero_left_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.gcd_zero_left")));
    }
    #[test]
    fn test_lcm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.lcm")));
    }
    #[test]
    fn test_lcm_comm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.lcm_comm")));
    }
    #[test]
    fn test_coprime_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.coprime")));
    }
    #[test]
    fn test_coprime_iff_gcd_eq_one_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.coprime_iff_gcd_eq_one")));
    }
    #[test]
    fn test_factorial_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.factorial")));
    }
    #[test]
    fn test_factorial_zero_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.factorial_zero")));
    }
    #[test]
    fn test_factorial_succ_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.factorial_succ")));
    }
    #[test]
    fn test_choose_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.choose")));
    }
    #[test]
    fn test_choose_zero_right_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.choose_zero_right")));
    }
    #[test]
    fn test_choose_zero_left_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.choose_zero_left")));
    }
    #[test]
    fn test_choose_symm_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.choose_symm")));
    }
    #[test]
    fn test_prime_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.Prime")));
    }
    #[test]
    fn test_prime_eq_one_or_self_of_dvd_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.Prime.eq_one_or_self_of_dvd")));
    }
    #[test]
    fn test_decidable_eq_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.Decidable.eq")));
    }
    #[test]
    fn test_decidable_le_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.Decidable.le")));
    }
    #[test]
    fn test_decidable_lt_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.Decidable.lt")));
    }
    #[test]
    fn test_mul_lt_mul_left_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.mul_lt_mul_left")));
    }
    #[test]
    fn test_mul_le_mul_left_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.mul_le_mul_left")));
    }
    #[test]
    fn test_pow_succ_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.pow_succ")));
    }
    #[test]
    fn test_pow_zero_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.pow_zero")));
    }
    #[test]
    fn test_pow_one_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.pow_one")));
    }
    #[test]
    fn test_sub_le_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.sub_le")));
    }
    #[test]
    fn test_sub_mono_right_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.sub_mono_right")));
    }
    #[test]
    fn test_add_le_add_left_present() {
        let (env, _) = full_env();
        assert!(env.contains(&Name::str("Nat.add_le_add_left")));
    }
    #[test]
    fn test_div_mul_type_is_pi() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.div_mul"))
            .expect("declaration 'Nat.div_mul' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_factorial_type_is_arrow() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.factorial"))
            .expect("declaration 'Nat.factorial' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_choose_type_is_arrow() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.choose"))
            .expect("declaration 'Nat.choose' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_prime_type_is_arrow() {
        let (env, _) = full_env();
        let decl = env
            .get(&Name::str("Nat.Prime"))
            .expect("declaration 'Nat.Prime' should exist in env");
        assert!(decl.ty().is_pi());
    }
    #[test]
    fn test_all_new_theorems_in_env() {
        let (env, _) = full_env();
        let new_theorems = [
            "Nat.div_mul",
            "Nat.mod_eq_zero_of_dvd",
            "Nat.mod_add_mod",
            "Nat.mod_lt",
            "Nat.gcd_comm",
            "Nat.gcd_zero_right",
            "Nat.gcd_zero_left",
            "Nat.lcm_comm",
            "Nat.coprime_iff_gcd_eq_one",
            "Nat.factorial_zero",
            "Nat.factorial_succ",
            "Nat.choose_zero_right",
            "Nat.choose_zero_left",
            "Nat.choose_symm",
            "Nat.Prime.eq_one_or_self_of_dvd",
            "Nat.mul_lt_mul_left",
            "Nat.mul_le_mul_left",
            "Nat.pow_succ",
            "Nat.pow_zero",
            "Nat.pow_one",
            "Nat.sub_le",
            "Nat.sub_mono_right",
            "Nat.add_le_add_left",
        ];
        for name in &new_theorems {
            assert!(env.contains(&Name::str(*name)), "missing theorem: {}", name);
        }
    }
}
#[cfg(test)]
mod extended_nat_tests {
    use super::*;
    #[test]
    fn test_fibonacci() {
        assert_eq!(FibonacciUtil::fib(0), 0);
        assert_eq!(FibonacciUtil::fib(1), 1);
        assert_eq!(FibonacciUtil::fib(10), 55);
        assert!(FibonacciUtil::is_fibonacci(55));
        assert!(!FibonacciUtil::is_fibonacci(56));
    }
    #[test]
    fn test_zeckendorf() {
        let z = FibonacciUtil::zeckendorf(11);
        assert!(!z.is_empty());
        assert_eq!(z.iter().sum::<u64>(), 11);
    }
    #[test]
    fn test_euler_totient() {
        assert_eq!(ArithmeticFunctions::euler_totient(1), 1);
        assert_eq!(ArithmeticFunctions::euler_totient(6), 2);
        assert_eq!(ArithmeticFunctions::euler_totient(9), 6);
    }
    #[test]
    fn test_mobius() {
        assert_eq!(ArithmeticFunctions::mobius(1), 1);
        assert_eq!(ArithmeticFunctions::mobius(4), 0);
        assert_eq!(ArithmeticFunctions::mobius(6), 1);
        assert_eq!(ArithmeticFunctions::mobius(2), -1);
    }
    #[test]
    fn test_divisor_functions() {
        assert_eq!(ArithmeticFunctions::num_divisors(6), 4);
        assert_eq!(ArithmeticFunctions::sum_of_divisors(6), 12);
    }
    #[test]
    fn test_collatz() {
        let seq = CollatzUtil::sequence(6);
        assert_eq!(seq[0], 6);
        assert_eq!(*seq.last().expect("last should succeed"), 1);
        let st = CollatzUtil::stopping_time(6);
        assert!(st.is_some());
    }
}
