//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

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
/// Build a function application `f a b c`.
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
/// Build `Pi (name : dom), body` with given binder info.
#[allow(dead_code)]
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
/// Build a non-dependent arrow `A -> B`.
#[allow(dead_code)]
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
/// Named constant with no universe levels.
#[allow(dead_code)]
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
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
/// Bound variable.
#[allow(dead_code)]
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// Nat type.
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// Int type.
#[allow(dead_code)]
pub fn int_ty() -> Expr {
    cst("Int")
}
/// Bool type.
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// Build `BitVec n` type expression.
#[allow(dead_code)]
pub fn mk_bitvec(n: Expr) -> Expr {
    app(cst("BitVec"), n)
}
/// Build `BitVec.ofNat n val`.
#[allow(dead_code)]
pub fn mk_bitvec_of_nat(n: Expr, val: Expr) -> Expr {
    app2(cst("BitVec.ofNat"), n, val)
}
/// Build `BitVec.zero n`.
#[allow(dead_code)]
pub fn mk_bitvec_zero(n: Expr) -> Expr {
    app(cst("BitVec.zero"), n)
}
/// Build `BitVec.and a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_and(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.and"), a, b)
}
/// Build `BitVec.or a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_or(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.or"), a, b)
}
/// Build `BitVec.xor a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_xor(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.xor"), a, b)
}
/// Build `BitVec.add a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_add(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.add"), a, b)
}
/// Build `BitVec.sub a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_sub(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.sub"), a, b)
}
/// Build `BitVec.mul a b` (implicit n).
#[allow(dead_code)]
pub fn mk_bitvec_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("BitVec.mul"), a, b)
}
/// Build `Eq @{} ty a b`.
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("Eq"), ty, a, b)
}
/// Build `Eq @{} (BitVec n) a b` where n is a bvar.
#[allow(dead_code)]
pub fn mk_bv_eq(n: Expr, a: Expr, b: Expr) -> Expr {
    mk_eq(mk_bitvec(n), a, b)
}
/// Build `Iff a b`.
#[allow(dead_code)]
pub fn mk_iff(a: Expr, b: Expr) -> Expr {
    app2(cst("Iff"), a, b)
}
/// Build `Eq @{} Nat a b`.
#[allow(dead_code)]
pub fn mk_nat_eq(a: Expr, b: Expr) -> Expr {
    mk_eq(nat_ty(), a, b)
}
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
/// `∀ {n : Nat}, BitVec n → BitVec n → BitVec n`
#[allow(dead_code)]
pub fn bitvec_binop_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(
            mk_bitvec(bvar(0)),
            arrow(mk_bitvec(bvar(1)), mk_bitvec(bvar(2))),
        ),
    )
}
/// `∀ {n : Nat}, BitVec n → BitVec n`
#[allow(dead_code)]
pub fn bitvec_unop_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), mk_bitvec(bvar(1))),
    )
}
/// `∀ {n : Nat}, BitVec n → Nat → BitVec n`
#[allow(dead_code)]
pub fn bitvec_shift_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), arrow(nat_ty(), mk_bitvec(bvar(1)))),
    )
}
/// `∀ {n : Nat}, BitVec n → BitVec n → Bool`
#[allow(dead_code)]
pub fn bitvec_cmp_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), arrow(mk_bitvec(bvar(1)), bool_ty())),
    )
}
/// `∀ {n : Nat}, BitVec n → Nat → Bool`
#[allow(dead_code)]
pub fn bitvec_getbit_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), arrow(nat_ty(), bool_ty())),
    )
}
/// Build the BitVec environment, adding the BitVec type, operations,
/// and theorems as axioms.
///
/// Assumes `Nat`, `Int`, `Bool`, `Eq`, and `Iff` are already declared in `env`.
#[allow(clippy::too_many_lines)]
pub fn build_bitvec_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "BitVec", vec![], arrow(nat_ty(), type1()))?;
    let of_nat_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(nat_ty(), mk_bitvec(bvar(0))),
    );
    add_axiom(env, "BitVec.ofNat", vec![], of_nat_ty)?;
    let to_nat_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), nat_ty()),
    );
    add_axiom(env, "BitVec.toNat", vec![], to_nat_ty)?;
    let of_bool_ty = arrow(bool_ty(), mk_bitvec(app(cst("Nat.succ"), cst("Nat.zero"))));
    add_axiom(env, "BitVec.ofBool", vec![], of_bool_ty)?;
    let zero_ty = pi(BinderInfo::Default, "n", nat_ty(), mk_bitvec(bvar(0)));
    add_axiom(env, "BitVec.zero", vec![], zero_ty)?;
    let all_ones_ty = pi(BinderInfo::Default, "n", nat_ty(), mk_bitvec(bvar(0)));
    add_axiom(env, "BitVec.allOnes", vec![], all_ones_ty)?;
    add_axiom(env, "BitVec.and", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.or", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.xor", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.not", vec![], bitvec_unop_ty())?;
    add_axiom(env, "BitVec.shiftLeft", vec![], bitvec_shift_ty())?;
    add_axiom(env, "BitVec.shiftRight", vec![], bitvec_shift_ty())?;
    add_axiom(env, "BitVec.add", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.sub", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.mul", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.neg", vec![], bitvec_unop_ty())?;
    add_axiom(env, "BitVec.udiv", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.umod", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.sdiv", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.smod", vec![], bitvec_binop_ty())?;
    add_axiom(env, "BitVec.ult", vec![], bitvec_cmp_ty())?;
    add_axiom(env, "BitVec.ule", vec![], bitvec_cmp_ty())?;
    add_axiom(env, "BitVec.slt", vec![], bitvec_cmp_ty())?;
    add_axiom(env, "BitVec.sle", vec![], bitvec_cmp_ty())?;
    add_axiom(env, "BitVec.beq", vec![], bitvec_cmp_ty())?;
    add_axiom(env, "BitVec.getLsb", vec![], bitvec_getbit_ty())?;
    add_axiom(env, "BitVec.getMsb", vec![], bitvec_getbit_ty())?;
    let set_width_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(mk_bitvec(bvar(1)), mk_bitvec(bvar(0))),
        ),
    );
    add_axiom(env, "BitVec.setWidth", vec![], set_width_ty)?;
    let append_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Implicit,
            "m",
            nat_ty(),
            arrow(
                mk_bitvec(bvar(1)),
                arrow(
                    mk_bitvec(bvar(1)),
                    mk_bitvec(app2(cst("Nat.add"), bvar(3), bvar(2))),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.append", vec![], append_ty)?;
    let extract_lsb_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "hi",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lo",
                nat_ty(),
                arrow(
                    mk_bitvec(bvar(2)),
                    mk_bitvec(app2(
                        cst("Nat.add"),
                        app2(cst("Nat.sub"), bvar(2), bvar(1)),
                        app(cst("Nat.succ"), cst("Nat.zero")),
                    )),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.extractLsb", vec![], extract_lsb_ty)?;
    let replicate_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            arrow(
                mk_bitvec(bvar(1)),
                mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(2))),
            ),
        ),
    );
    add_axiom(env, "BitVec.replicate", vec![], replicate_ty)?;
    add_axiom(env, "BitVec.rotateLeft", vec![], bitvec_shift_ty())?;
    add_axiom(env, "BitVec.rotateRight", vec![], bitvec_shift_ty())?;
    let sign_extend_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(mk_bitvec(bvar(1)), mk_bitvec(bvar(0))),
        ),
    );
    add_axiom(env, "BitVec.signExtend", vec![], sign_extend_ty)?;
    let to_int_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), int_ty()),
    );
    add_axiom(env, "BitVec.toInt", vec![], to_int_ty)?;
    let of_int_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(int_ty(), mk_bitvec(bvar(0))),
    );
    add_axiom(env, "BitVec.ofInt", vec![], of_int_ty)?;
    let add_comm_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                mk_bv_eq(
                    bvar(2),
                    app2(cst("BitVec.add"), bvar(1), bvar(0)),
                    app2(cst("BitVec.add"), bvar(0), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.add_comm", vec![], add_comm_ty)?;
    let add_assoc_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                pi(
                    BinderInfo::Default,
                    "c",
                    mk_bitvec(bvar(2)),
                    mk_bv_eq(
                        bvar(3),
                        app2(
                            cst("BitVec.add"),
                            app2(cst("BitVec.add"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("BitVec.add"),
                            bvar(2),
                            app2(cst("BitVec.add"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.add_assoc", vec![], add_assoc_ty)?;
    let add_zero_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.add"), bvar(0), app(cst("BitVec.zero"), bvar(1))),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "BitVec.add_zero", vec![], add_zero_ty)?;
    let zero_add_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.add"), app(cst("BitVec.zero"), bvar(1)), bvar(0)),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "BitVec.zero_add", vec![], zero_add_ty)?;
    let and_comm_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                mk_bv_eq(
                    bvar(2),
                    app2(cst("BitVec.and"), bvar(1), bvar(0)),
                    app2(cst("BitVec.and"), bvar(0), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.and_comm", vec![], and_comm_ty)?;
    let and_assoc_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                pi(
                    BinderInfo::Default,
                    "c",
                    mk_bitvec(bvar(2)),
                    mk_bv_eq(
                        bvar(3),
                        app2(
                            cst("BitVec.and"),
                            app2(cst("BitVec.and"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("BitVec.and"),
                            bvar(2),
                            app2(cst("BitVec.and"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.and_assoc", vec![], and_assoc_ty)?;
    let or_comm_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                mk_bv_eq(
                    bvar(2),
                    app2(cst("BitVec.or"), bvar(1), bvar(0)),
                    app2(cst("BitVec.or"), bvar(0), bvar(1)),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.or_comm", vec![], or_comm_ty)?;
    let or_assoc_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                pi(
                    BinderInfo::Default,
                    "c",
                    mk_bitvec(bvar(2)),
                    mk_bv_eq(
                        bvar(3),
                        app2(
                            cst("BitVec.or"),
                            app2(cst("BitVec.or"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("BitVec.or"),
                            bvar(2),
                            app2(cst("BitVec.or"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.or_assoc", vec![], or_assoc_ty)?;
    let xor_self_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.xor"), bvar(0), bvar(0)),
                app(cst("BitVec.zero"), bvar(1)),
            ),
        ),
    );
    add_axiom(env, "BitVec.xor_self", vec![], xor_self_ty)?;
    let and_self_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(bvar(1), app2(cst("BitVec.and"), bvar(0), bvar(0)), bvar(0)),
        ),
    );
    add_axiom(env, "BitVec.and_self", vec![], and_self_ty)?;
    let or_self_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(bvar(1), app2(cst("BitVec.or"), bvar(0), bvar(0)), bvar(0)),
        ),
    );
    add_axiom(env, "BitVec.or_self", vec![], or_self_ty)?;
    let not_not_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.not"), app(cst("BitVec.not"), bvar(0))),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "BitVec.not_not", vec![], not_not_ty)?;
    let demorgan_and_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                mk_bv_eq(
                    bvar(2),
                    app(cst("BitVec.not"), app2(cst("BitVec.and"), bvar(1), bvar(0))),
                    app2(
                        cst("BitVec.or"),
                        app(cst("BitVec.not"), bvar(1)),
                        app(cst("BitVec.not"), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.demorgan_and", vec![], demorgan_and_ty)?;
    let demorgan_or_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                mk_bv_eq(
                    bvar(2),
                    app(cst("BitVec.not"), app2(cst("BitVec.or"), bvar(1), bvar(0))),
                    app2(
                        cst("BitVec.and"),
                        app(cst("BitVec.not"), bvar(1)),
                        app(cst("BitVec.not"), bvar(0)),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.demorgan_or", vec![], demorgan_or_ty)?;
    let to_nat_of_nat_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "val",
            nat_ty(),
            mk_nat_eq(
                app(
                    cst("BitVec.toNat"),
                    app2(cst("BitVec.ofNat"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("Nat.mod"),
                    bvar(0),
                    app2(
                        cst("Nat.pow"),
                        app(cst("Nat.succ"), app(cst("Nat.succ"), cst("Nat.zero"))),
                        bvar(1),
                    ),
                ),
            ),
        ),
    );
    add_axiom(env, "BitVec.toNat_ofNat", vec![], to_nat_of_nat_ty)?;
    let of_nat_to_nat_ty = pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "v",
            mk_bitvec(bvar(0)),
            mk_bv_eq(
                bvar(1),
                app2(
                    cst("BitVec.ofNat"),
                    bvar(1),
                    app(cst("BitVec.toNat"), bvar(0)),
                ),
                bvar(0),
            ),
        ),
    );
    add_axiom(env, "BitVec.ofNat_toNat", vec![], of_nat_to_nat_ty)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Set up a minimal environment with prerequisite declarations.
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1(),
        })
        .expect("operation should succeed");
        for name in &[
            "Nat.zero", "Nat.succ", "Nat.add", "Nat.mul", "Nat.sub", "Nat.mod", "Nat.pow",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: nat_ty(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: type1(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1(),
        })
        .expect("operation should succeed");
        for name in &["true", "false"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: bool_ty(),
            })
            .expect("operation should succeed");
        }
        env.add(Declaration::Axiom {
            name: Name::str("Eq"),
            univ_params: vec![],
            ty: prop(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Iff"),
            univ_params: vec![],
            ty: prop(),
        })
        .expect("operation should succeed");
        env
    }
    /// Build a full test environment.
    fn full_env() -> Environment {
        let mut env = setup_env();
        build_bitvec_env(&mut env).expect("build_bitvec_env should succeed");
        env
    }
    #[test]
    fn test_build_bitvec_env_succeeds() {
        let _ = full_env();
    }
    #[test]
    fn test_bitvec_type_present() {
        let env = full_env();
        assert!(env.contains(&Name::str("BitVec")));
    }
    #[test]
    fn test_bitvec_constructors_present() {
        let env = full_env();
        let names = [
            "BitVec.ofNat",
            "BitVec.toNat",
            "BitVec.ofBool",
            "BitVec.zero",
            "BitVec.allOnes",
        ];
        for name in &names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing constructor: {}",
                name
            );
        }
    }
    #[test]
    fn test_bitvec_bitwise_ops_present() {
        let env = full_env();
        let names = [
            "BitVec.and",
            "BitVec.or",
            "BitVec.xor",
            "BitVec.not",
            "BitVec.shiftLeft",
            "BitVec.shiftRight",
        ];
        for name in &names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing bitwise op: {}",
                name
            );
        }
    }
    #[test]
    fn test_bitvec_arithmetic_ops_present() {
        let env = full_env();
        let names = [
            "BitVec.add",
            "BitVec.sub",
            "BitVec.mul",
            "BitVec.neg",
            "BitVec.udiv",
            "BitVec.umod",
            "BitVec.sdiv",
            "BitVec.smod",
        ];
        for name in &names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing arith op: {}",
                name
            );
        }
    }
    #[test]
    fn test_bitvec_comparison_ops_present() {
        let env = full_env();
        let names = [
            "BitVec.ult",
            "BitVec.ule",
            "BitVec.slt",
            "BitVec.sle",
            "BitVec.beq",
        ];
        for name in &names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing comparison: {}",
                name
            );
        }
    }
    #[test]
    fn test_bitvec_bit_ops_present() {
        let env = full_env();
        let names = [
            "BitVec.getLsb",
            "BitVec.getMsb",
            "BitVec.setWidth",
            "BitVec.append",
            "BitVec.extractLsb",
            "BitVec.replicate",
            "BitVec.rotateLeft",
            "BitVec.rotateRight",
            "BitVec.signExtend",
        ];
        for name in &names {
            assert!(env.contains(&Name::str(*name)), "missing bit op: {}", name);
        }
    }
    #[test]
    fn test_bitvec_conversion_present() {
        let env = full_env();
        let names = ["BitVec.toInt", "BitVec.ofInt"];
        for name in &names {
            assert!(
                env.contains(&Name::str(*name)),
                "missing conversion: {}",
                name
            );
        }
    }
    #[test]
    fn test_bitvec_theorems_present() {
        let env = full_env();
        let names = [
            "BitVec.add_comm",
            "BitVec.add_assoc",
            "BitVec.add_zero",
            "BitVec.zero_add",
            "BitVec.and_comm",
            "BitVec.and_assoc",
            "BitVec.or_comm",
            "BitVec.or_assoc",
            "BitVec.xor_self",
            "BitVec.and_self",
            "BitVec.or_self",
            "BitVec.not_not",
            "BitVec.demorgan_and",
            "BitVec.demorgan_or",
            "BitVec.toNat_ofNat",
            "BitVec.ofNat_toNat",
        ];
        for name in &names {
            assert!(env.contains(&Name::str(*name)), "missing theorem: {}", name);
        }
    }
    #[test]
    fn test_bitvec_type_is_arrow() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec"))
            .expect("declaration 'BitVec' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_of_nat_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.ofNat"))
            .expect("declaration 'BitVec.ofNat' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_and_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.and"))
            .expect("declaration 'BitVec.and' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(bi, _, dom, _) = ty {
            assert_eq!(*bi, BinderInfo::Implicit);
            assert_eq!(**dom, nat_ty());
        }
    }
    #[test]
    fn test_bitvec_add_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.add"))
            .expect("declaration 'BitVec.add' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_ult_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.ult"))
            .expect("declaration 'BitVec.ult' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_get_lsb_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.getLsb"))
            .expect("declaration 'BitVec.getLsb' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_to_int_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.toInt"))
            .expect("declaration 'BitVec.toInt' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_add_comm_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.add_comm"))
            .expect("declaration 'BitVec.add_comm' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_not_not_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.not_not"))
            .expect("declaration 'BitVec.not_not' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_append_type_is_pi() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.append"))
            .expect("declaration 'BitVec.append' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_mk_bitvec_structure() {
        let e = mk_bitvec(cst("n"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("BitVec"));
            assert_eq!(**arg, cst("n"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_bitvec_of_nat_structure() {
        let e = mk_bitvec_of_nat(cst("n"), cst("val"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_zero_structure() {
        let e = mk_bitvec_zero(cst("n"));
        if let Expr::App(f, arg) = &e {
            assert_eq!(**f, cst("BitVec.zero"));
            assert_eq!(**arg, cst("n"));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_bitvec_and_structure() {
        let e = mk_bitvec_and(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_or_structure() {
        let e = mk_bitvec_or(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_xor_structure() {
        let e = mk_bitvec_xor(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_add_structure() {
        let e = mk_bitvec_add(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_sub_structure() {
        let e = mk_bitvec_sub(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_mk_bitvec_mul_structure() {
        let e = mk_bitvec_mul(cst("a"), cst("b"));
        assert!(e.is_app());
    }
    #[test]
    fn test_total_declaration_count() {
        let env = full_env();
        let prereq_count = 13;
        assert!(
            env.len() >= prereq_count + 52,
            "expected at least {} declarations, got {}",
            prereq_count + 52,
            env.len()
        );
    }
    #[test]
    fn test_bitvec_set_width_type_structure() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.setWidth"))
            .expect("declaration 'BitVec.setWidth' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(bi, _, dom, _) = ty {
            assert_eq!(*bi, BinderInfo::Implicit);
            assert_eq!(**dom, nat_ty());
        }
    }
    #[test]
    fn test_bitvec_sign_extend_type_structure() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.signExtend"))
            .expect("declaration 'BitVec.signExtend' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
    }
    #[test]
    fn test_bitvec_of_bool_type_structure() {
        let env = full_env();
        let decl = env
            .get(&Name::str("BitVec.ofBool"))
            .expect("declaration 'BitVec.ofBool' should exist in env");
        let ty = decl.ty();
        assert!(ty.is_pi());
        if let Expr::Pi(_, _, dom, _) = ty {
            assert_eq!(**dom, bool_ty());
        }
    }
}
/// Build `∀ {n} (a b : BitVec n), P a b` — the standard two-bitvec quantifier.
#[allow(dead_code)]
pub fn bvx_ext_forall_two(body_builder: impl Fn() -> Expr) -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(BinderInfo::Default, "b", mk_bitvec(bvar(1)), body_builder()),
        ),
    )
}
/// Build `∀ {n} (a : BitVec n), P a`.
#[allow(dead_code)]
pub fn bvx_ext_forall_one(body_builder: impl Fn() -> Expr) -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "a", mk_bitvec(bvar(0)), body_builder()),
    )
}
/// Build `∀ {n} (a b c : BitVec n), P a b c`.
#[allow(dead_code)]
pub fn bvx_ext_forall_three(body_builder: impl Fn() -> Expr) -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(
                BinderInfo::Default,
                "b",
                mk_bitvec(bvar(1)),
                pi(BinderInfo::Default, "c", mk_bitvec(bvar(2)), body_builder()),
            ),
        ),
    )
}
/// Build `∀ {n} (a : BitVec n) (k : Nat), P a k`.
#[allow(dead_code)]
pub fn bvx_ext_forall_bv_nat(body_builder: impl Fn() -> Expr) -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            mk_bitvec(bvar(0)),
            pi(BinderInfo::Default, "k", nat_ty(), body_builder()),
        ),
    )
}
/// Build `∀ (n : Nat) (a : BitVec n), P n a`.
#[allow(dead_code)]
pub fn bvx_ext_forall_nat_bv(body_builder: impl Fn() -> Expr) -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "a", mk_bitvec(bvar(0)), body_builder()),
    )
}
/// BV equality shorthand for bvar indices `(a_idx, b_idx)` under width bvar `n_idx`.
#[allow(dead_code)]
pub fn bvx_ext_bveq(n_idx: u32, a_idx: u32, b_idx: u32) -> Expr {
    mk_bv_eq(bvar(n_idx), bvar(a_idx), bvar(b_idx))
}
/// Nat equality `Eq Nat lhs rhs`.
#[allow(dead_code)]
pub fn bvx_ext_nateq(lhs: Expr, rhs: Expr) -> Expr {
    mk_nat_eq(lhs, rhs)
}
/// Build `BitVec.arithShiftRight : {n : Nat} → BitVec n → Nat → BitVec n`.
#[allow(dead_code)]
pub fn bvx_ext_arith_shift_right_ty() -> Expr {
    bitvec_shift_ty()
}
/// Build type for `popcount : {n : Nat} → BitVec n → Nat`.
#[allow(dead_code)]
pub fn bvx_ext_popcount_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), nat_ty()),
    )
}
/// Build type for `clz/ctz : {n : Nat} → BitVec n → Nat`.
#[allow(dead_code)]
pub fn bvx_ext_count_zeros_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        arrow(mk_bitvec(bvar(0)), nat_ty()),
    )
}
/// Build type for `reverse : {n : Nat} → BitVec n → BitVec n`.
#[allow(dead_code)]
pub fn bvx_ext_reverse_ty() -> Expr {
    bitvec_unop_ty()
}
/// Build `∀ {n} (a b : BitVec n), a = b ↔ (∀ i < n, getLsb a i = getLsb b i)`.
/// Simplified as a Prop-valued type.
#[allow(dead_code)]
pub fn bvx_ext_ext_ty() -> Expr {
    bvx_ext_forall_two(|| bvx_ext_bveq(2, 1, 0))
}
/// Zero-extension type: `{n : Nat} → (m : Nat) → BitVec n → BitVec m`.
#[allow(dead_code)]
pub fn bvx_ext_zero_extend_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(mk_bitvec(bvar(1)), mk_bitvec(bvar(0))),
        ),
    )
}
/// Concatenation result type helper: `BitVec (Nat.add n m)`.
#[allow(dead_code)]
pub fn bvx_ext_concat_result(n_bvar: u32, m_bvar: u32) -> Expr {
    mk_bitvec(app2(cst("Nat.add"), bvar(n_bvar), bvar(m_bvar)))
}
/// Build `Fin (Nat.pow 2 n)` type expression.
#[allow(dead_code)]
pub fn bvx_ext_fin2n(n: Expr) -> Expr {
    app(
        cst("Fin"),
        app2(
            cst("Nat.pow"),
            app(cst("Nat.succ"), app(cst("Nat.succ"), cst("Nat.zero"))),
            n,
        ),
    )
}
/// Build `Int` constant.
#[allow(dead_code)]
pub fn bvx_ext_int() -> Expr {
    int_ty()
}
/// Build `Nat` constant.
#[allow(dead_code)]
pub fn bvx_ext_nat() -> Expr {
    nat_ty()
}
