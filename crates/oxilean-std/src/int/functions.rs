//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Literal, Name};

use super::types::{
    IntCrtSolver, IntExtContinuedFraction, IntExtEuclidResult, IntExtGaussian, IntExtPadicVal,
    IntExtSternBrocot,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn int_const() -> Expr {
    cst("Int")
}
pub fn nat_const() -> Expr {
    cst("Nat")
}
pub fn bool_const() -> Expr {
    cst("Bool")
}
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
/// Helper: Int → Int → Int
pub fn int_binop_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), int_const()),
    )
}
/// Helper: Int → Int → Prop
pub fn int_rel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), prop()),
    )
}
/// Helper: Int → Int → Bool
pub fn int_bool_binop_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), bool_const()),
    )
}
/// Helper: ∀ (a b : Int), Eq (lhs) (rhs) — generic for commutative theorems
pub fn forall2_int_eq(body: Expr) -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), body),
    )
}
/// Helper: ∀ (a b c : Int), body
pub fn forall3_int(body: Expr) -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(
            BinderInfo::Default,
            "b",
            int_const(),
            pi(BinderInfo::Default, "c", int_const(), body),
        ),
    )
}
/// Helper: ∀ (a : Int), body
pub fn forall1_int(body: Expr) -> Expr {
    pi(BinderInfo::Default, "a", int_const(), body)
}
/// Helper: Eq a b (on Int), where a and b are already complete expressions.
pub fn int_eq_expr(a: Expr, b: Expr) -> Expr {
    app3(
        cst_u("Eq", vec![Level::succ(Level::zero())]),
        int_const(),
        a,
        b,
    )
}
/// Helper: Eq a b (on Nat).
pub fn nat_eq_expr(a: Expr, b: Expr) -> Expr {
    app3(
        cst_u("Eq", vec![Level::succ(Level::zero())]),
        nat_const(),
        a,
        b,
    )
}
/// Build the integer environment containing Int type, operations, and theorems.
#[allow(clippy::too_many_lines)]
pub fn build_int_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Int", vec![], type1())?;
    let ofnat_ty = pi(BinderInfo::Default, "n", nat_const(), int_const());
    add_axiom(env, "Int.ofNat", vec![], ofnat_ty)?;
    let negsucc_ty = pi(BinderInfo::Default, "n", nat_const(), int_const());
    add_axiom(env, "Int.negSucc", vec![], negsucc_ty)?;
    add_axiom(env, "Int.add", vec![], int_binop_ty())?;
    add_axiom(env, "Int.mul", vec![], int_binop_ty())?;
    add_axiom(env, "Int.sub", vec![], int_binop_ty())?;
    let neg_ty = pi(BinderInfo::Default, "a", int_const(), int_const());
    add_axiom(env, "Int.neg", vec![], neg_ty)?;
    let abs_ty = pi(BinderInfo::Default, "a", int_const(), nat_const());
    add_axiom(env, "Int.abs", vec![], abs_ty)?;
    let sign_ty = pi(BinderInfo::Default, "a", int_const(), int_const());
    add_axiom(env, "Int.sign", vec![], sign_ty)?;
    add_axiom(env, "Int.div", vec![], int_binop_ty())?;
    add_axiom(env, "Int.mod", vec![], int_binop_ty())?;
    let gcd_ty = pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), nat_const()),
    );
    add_axiom(env, "Int.gcd", vec![], gcd_ty)?;
    add_axiom(env, "Int.le", vec![], int_rel_ty())?;
    add_axiom(env, "Int.lt", vec![], int_rel_ty())?;
    add_axiom(env, "Int.beq", vec![], int_bool_binop_ty())?;
    add_axiom(env, "Int.ble", vec![], int_bool_binop_ty())?;
    let rec_ty = pi(
        BinderInfo::Implicit,
        "C",
        pi(BinderInfo::Default, "_", int_const(), sort_u()),
        pi(
            BinderInfo::Default,
            "h_ofNat",
            pi(
                BinderInfo::Default,
                "n",
                nat_const(),
                app(bvar(1), app(cst("Int.ofNat"), bvar(0))),
            ),
            pi(
                BinderInfo::Default,
                "h_negSucc",
                pi(
                    BinderInfo::Default,
                    "n",
                    nat_const(),
                    app(bvar(2), app(cst("Int.negSucc"), bvar(0))),
                ),
                pi(BinderInfo::Default, "i", int_const(), app(bvar(3), bvar(0))),
            ),
        ),
    );
    add_axiom(env, "Int.rec", vec![Name::str("u")], rec_ty)?;
    let ofnat_zero_ty = int_eq_expr(
        app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
        app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
    );
    add_axiom(env, "Int.ofNat_zero", vec![], ofnat_zero_ty)?;
    let ofnat_succ_ty = pi(
        BinderInfo::Default,
        "n",
        nat_const(),
        int_eq_expr(
            app(cst("Int.ofNat"), app(cst("Nat.succ"), bvar(0))),
            app2(
                cst("Int.add"),
                app(cst("Int.ofNat"), bvar(0)),
                app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(1))),
            ),
        ),
    );
    add_axiom(env, "Int.ofNat_succ", vec![], ofnat_succ_ty)?;
    let add_comm_ty = forall2_int_eq(int_eq_expr(
        app2(cst("Int.add"), bvar(1), bvar(0)),
        app2(cst("Int.add"), bvar(0), bvar(1)),
    ));
    add_axiom(env, "Int.add_comm", vec![], add_comm_ty)?;
    let add_assoc_ty = forall3_int(int_eq_expr(
        app2(
            cst("Int.add"),
            app2(cst("Int.add"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.add"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), bvar(0)),
        ),
    ));
    add_axiom(env, "Int.add_assoc", vec![], add_assoc_ty)?;
    let int_zero = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)));
    let add_zero_ty = forall1_int(int_eq_expr(
        app2(cst("Int.add"), bvar(0), int_zero.clone()),
        bvar(0),
    ));
    add_axiom(env, "Int.add_zero", vec![], add_zero_ty)?;
    let zero_add_ty = forall1_int(int_eq_expr(
        app2(cst("Int.add"), int_zero.clone(), bvar(0)),
        bvar(0),
    ));
    add_axiom(env, "Int.zero_add", vec![], zero_add_ty)?;
    let add_neg_cancel_ty = forall1_int(int_eq_expr(
        app2(cst("Int.add"), bvar(0), app(cst("Int.neg"), bvar(0))),
        int_zero.clone(),
    ));
    add_axiom(env, "Int.add_neg_cancel", vec![], add_neg_cancel_ty)?;
    let neg_add_cancel_ty = forall1_int(int_eq_expr(
        app2(cst("Int.add"), app(cst("Int.neg"), bvar(0)), bvar(0)),
        int_zero.clone(),
    ));
    add_axiom(env, "Int.neg_add_cancel", vec![], neg_add_cancel_ty)?;
    let mul_comm_ty = forall2_int_eq(int_eq_expr(
        app2(cst("Int.mul"), bvar(1), bvar(0)),
        app2(cst("Int.mul"), bvar(0), bvar(1)),
    ));
    add_axiom(env, "Int.mul_comm", vec![], mul_comm_ty)?;
    let mul_assoc_ty = forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.mul"), bvar(1), bvar(0)),
        ),
    ));
    add_axiom(env, "Int.mul_assoc", vec![], mul_assoc_ty)?;
    let int_one = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(1)));
    let mul_one_ty = forall1_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(0), int_one.clone()),
        bvar(0),
    ));
    add_axiom(env, "Int.mul_one", vec![], mul_one_ty)?;
    let one_mul_ty = forall1_int(int_eq_expr(
        app2(cst("Int.mul"), int_one.clone(), bvar(0)),
        bvar(0),
    ));
    add_axiom(env, "Int.one_mul", vec![], one_mul_ty)?;
    let mul_zero_ty = forall1_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(0), int_zero.clone()),
        int_zero.clone(),
    ));
    add_axiom(env, "Int.mul_zero", vec![], mul_zero_ty)?;
    let zero_mul_ty = forall1_int(int_eq_expr(
        app2(cst("Int.mul"), int_zero.clone(), bvar(0)),
        int_zero.clone(),
    ));
    add_axiom(env, "Int.zero_mul", vec![], zero_mul_ty)?;
    let left_distrib_ty = forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), bvar(0)),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
        ),
    ));
    add_axiom(env, "Int.left_distrib", vec![], left_distrib_ty)?;
    let right_distrib_ty = forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            app2(cst("Int.add"), bvar(1), bvar(0)),
            bvar(2),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(1), bvar(2)),
            app2(cst("Int.mul"), bvar(0), bvar(2)),
        ),
    ));
    add_axiom(env, "Int.right_distrib", vec![], right_distrib_ty)?;
    let neg_neg_ty = forall1_int(int_eq_expr(
        app(cst("Int.neg"), app(cst("Int.neg"), bvar(0))),
        bvar(0),
    ));
    add_axiom(env, "Int.neg_neg", vec![], neg_neg_ty)?;
    let neg_zero_ty = int_eq_expr(app(cst("Int.neg"), int_zero.clone()), int_zero.clone());
    add_axiom(env, "Int.neg_zero", vec![], neg_zero_ty)?;
    let sub_self_ty = forall1_int(int_eq_expr(
        app2(cst("Int.sub"), bvar(0), bvar(0)),
        int_zero.clone(),
    ));
    add_axiom(env, "Int.sub_self", vec![], sub_self_ty)?;
    let le_refl_ty = forall1_int(app2(cst("Int.le"), bvar(0), bvar(0)));
    add_axiom(env, "Int.le_refl", vec![], le_refl_ty)?;
    let le_trans_ty = forall3_int(pi(
        BinderInfo::Default,
        "h1",
        app2(cst("Int.le"), bvar(2), bvar(1)),
        pi(
            BinderInfo::Default,
            "h2",
            app2(cst("Int.le"), bvar(2), bvar(1)),
            app2(cst("Int.le"), bvar(4), bvar(2)),
        ),
    ));
    add_axiom(env, "Int.le_trans", vec![], le_trans_ty)?;
    let le_antisymm_ty = forall2_int_eq(pi(
        BinderInfo::Default,
        "h1",
        app2(cst("Int.le"), bvar(1), bvar(0)),
        pi(
            BinderInfo::Default,
            "h2",
            app2(cst("Int.le"), bvar(1), bvar(2)),
            int_eq_expr(bvar(3), bvar(2)),
        ),
    ));
    add_axiom(env, "Int.le_antisymm", vec![], le_antisymm_ty)?;
    let lt_iff_ty = forall2_int_eq(app2(
        cst("Iff"),
        app2(cst("Int.lt"), bvar(1), bvar(0)),
        app2(
            cst("And"),
            app2(cst("Int.le"), bvar(1), bvar(0)),
            app(cst("Not"), app2(cst("Int.le"), bvar(0), bvar(1))),
        ),
    ));
    add_axiom(env, "Int.lt_iff_le_not_le", vec![], lt_iff_ty)?;
    let abs_neg_ty = forall1_int(nat_eq_expr(
        app(cst("Int.abs"), app(cst("Int.neg"), bvar(0))),
        app(cst("Int.abs"), bvar(0)),
    ));
    add_axiom(env, "Int.abs_neg", vec![], abs_neg_ty)?;
    let abs_mul_ty = forall2_int_eq(nat_eq_expr(
        app(cst("Int.abs"), app2(cst("Int.mul"), bvar(1), bvar(0))),
        app2(
            cst("Nat.mul"),
            app(cst("Int.abs"), bvar(1)),
            app(cst("Int.abs"), bvar(0)),
        ),
    ));
    add_axiom(env, "Int.abs_mul", vec![], abs_mul_ty)?;
    let ofnat_add_ty = pi(
        BinderInfo::Default,
        "m",
        nat_const(),
        pi(
            BinderInfo::Default,
            "n",
            nat_const(),
            int_eq_expr(
                app(cst("Int.ofNat"), app2(cst("Nat.add"), bvar(1), bvar(0))),
                app2(
                    cst("Int.add"),
                    app(cst("Int.ofNat"), bvar(1)),
                    app(cst("Int.ofNat"), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "Int.ofNat_add", vec![], ofnat_add_ty)?;
    let ofnat_mul_ty = pi(
        BinderInfo::Default,
        "m",
        nat_const(),
        pi(
            BinderInfo::Default,
            "n",
            nat_const(),
            int_eq_expr(
                app(cst("Int.ofNat"), app2(cst("Nat.mul"), bvar(1), bvar(0))),
                app2(
                    cst("Int.mul"),
                    app(cst("Int.ofNat"), bvar(1)),
                    app(cst("Int.ofNat"), bvar(0)),
                ),
            ),
        ),
    );
    add_axiom(env, "Int.ofNat_mul", vec![], ofnat_mul_ty)?;
    Ok(())
}
/// The Int type expression.
#[allow(dead_code)]
pub fn int_ty() -> Expr {
    cst("Int")
}
/// Int.ofNat n.
#[allow(dead_code)]
pub fn int_of_nat(n: Expr) -> Expr {
    app(cst("Int.ofNat"), n)
}
/// Int.negSucc n.
#[allow(dead_code)]
pub fn int_neg_succ(n: Expr) -> Expr {
    app(cst("Int.negSucc"), n)
}
/// Int.add a b.
#[allow(dead_code)]
pub fn int_add(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.add"), a, b)
}
/// Int.mul a b.
#[allow(dead_code)]
pub fn int_mul(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.mul"), a, b)
}
/// Int.sub a b.
#[allow(dead_code)]
pub fn int_sub(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.sub"), a, b)
}
/// Int.neg a.
#[allow(dead_code)]
pub fn int_neg(a: Expr) -> Expr {
    app(cst("Int.neg"), a)
}
/// Int.abs a.
#[allow(dead_code)]
pub fn int_abs(a: Expr) -> Expr {
    app(cst("Int.abs"), a)
}
/// Int.sign a.
#[allow(dead_code)]
pub fn int_sign(a: Expr) -> Expr {
    app(cst("Int.sign"), a)
}
/// Int.div a b.
#[allow(dead_code)]
pub fn int_div(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.div"), a, b)
}
/// Int.mod a b.
#[allow(dead_code)]
pub fn int_mod(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.mod"), a, b)
}
/// Int.gcd a b.
#[allow(dead_code)]
pub fn int_gcd(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.gcd"), a, b)
}
/// Int.le a b.
#[allow(dead_code)]
pub fn int_le(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.le"), a, b)
}
/// Int.lt a b.
#[allow(dead_code)]
pub fn int_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.lt"), a, b)
}
/// Int.beq a b.
#[allow(dead_code)]
pub fn int_beq(a: Expr, b: Expr) -> Expr {
    app2(cst("Int.beq"), a, b)
}
/// Create an integer literal expression.
/// Positive values and zero use Int.ofNat, negative values use Int.negSucc.
/// For negative n, negSucc encodes -(n+1), so -k becomes negSucc(k-1).
#[allow(dead_code)]
pub fn int_lit(n: i64) -> Expr {
    if n >= 0 {
        app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(n as u64)))
    } else {
        let k = (-n - 1) as u64;
        app(cst("Int.negSucc"), Expr::Lit(Literal::Nat(k)))
    }
}
/// Int.ofNat 0.
#[allow(dead_code)]
pub fn int_zero() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)))
}
/// Int.ofNat 1.
#[allow(dead_code)]
pub fn int_one() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(1)))
}
/// Eq a b on Int.
#[allow(dead_code)]
pub fn mk_int_eq(a: Expr, b: Expr) -> Expr {
    app3(
        cst_u("Eq", vec![Level::succ(Level::zero())]),
        int_const(),
        a,
        b,
    )
}
/// Int.lcm : Int → Int → Nat
#[allow(dead_code)]
pub fn axiom_int_lcm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "b", int_const(), nat_const()),
    )
}
/// Int.lcm_comm : ∀ a b, Eq (Int.lcm a b) (Int.lcm b a)
#[allow(dead_code)]
pub fn axiom_int_lcm_comm_ty() -> Expr {
    forall2_int_eq(nat_eq_expr(
        app2(cst("Int.lcm"), bvar(1), bvar(0)),
        app2(cst("Int.lcm"), bvar(0), bvar(1)),
    ))
}
/// Int.gcd_comm : ∀ a b, Eq (Int.gcd a b) (Int.gcd b a)
#[allow(dead_code)]
pub fn axiom_int_gcd_comm_ty() -> Expr {
    forall2_int_eq(nat_eq_expr(
        app2(cst("Int.gcd"), bvar(1), bvar(0)),
        app2(cst("Int.gcd"), bvar(0), bvar(1)),
    ))
}
/// Int.gcd_self : ∀ a, Eq (Int.gcd a a) (Int.abs a)
#[allow(dead_code)]
pub fn axiom_int_gcd_self_ty() -> Expr {
    forall1_int(nat_eq_expr(
        app2(cst("Int.gcd"), bvar(0), bvar(0)),
        app(cst("Int.abs"), bvar(0)),
    ))
}
/// Int.gcd_zero_right : ∀ a, Eq (Int.gcd a 0) (Int.abs a)
#[allow(dead_code)]
pub fn axiom_int_gcd_zero_right_ty() -> Expr {
    forall1_int(nat_eq_expr(
        app2(
            cst("Int.gcd"),
            bvar(0),
            app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
        ),
        app(cst("Int.abs"), bvar(0)),
    ))
}
/// Int.gcd_zero_left : ∀ a, Eq (Int.gcd 0 a) (Int.abs a)
#[allow(dead_code)]
pub fn axiom_int_gcd_zero_left_ty() -> Expr {
    forall1_int(nat_eq_expr(
        app2(
            cst("Int.gcd"),
            app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
            bvar(0),
        ),
        app(cst("Int.abs"), bvar(0)),
    ))
}
/// Int.dvd : Int → Int → Prop  (divisibility relation)
#[allow(dead_code)]
pub fn axiom_int_dvd_ty() -> Expr {
    int_rel_ty()
}
/// Int.dvd_refl : ∀ a, Int.dvd a a
#[allow(dead_code)]
pub fn axiom_int_dvd_refl_ty() -> Expr {
    forall1_int(app2(cst("Int.dvd"), bvar(0), bvar(0)))
}
/// Int.dvd_trans : ∀ a b c, Int.dvd a b → Int.dvd b c → Int.dvd a c
#[allow(dead_code)]
pub fn axiom_int_dvd_trans_ty() -> Expr {
    forall3_int(pi(
        BinderInfo::Default,
        "h1",
        app2(cst("Int.dvd"), bvar(2), bvar(1)),
        pi(
            BinderInfo::Default,
            "h2",
            app2(cst("Int.dvd"), bvar(2), bvar(1)),
            app2(cst("Int.dvd"), bvar(4), bvar(2)),
        ),
    ))
}
/// Int.dvd_antisymm : ∀ a b, Int.dvd a b → Int.dvd b a → Eq (Int.abs a) (Int.abs b)
#[allow(dead_code)]
pub fn axiom_int_dvd_antisymm_ty() -> Expr {
    forall2_int_eq(pi(
        BinderInfo::Default,
        "h1",
        app2(cst("Int.dvd"), bvar(1), bvar(0)),
        pi(
            BinderInfo::Default,
            "h2",
            app2(cst("Int.dvd"), bvar(1), bvar(2)),
            nat_eq_expr(app(cst("Int.abs"), bvar(3)), app(cst("Int.abs"), bvar(2))),
        ),
    ))
}
/// Int.bezout : ∀ a b, ∃ x y : Int, Eq (Int.add (Int.mul a x) (Int.mul b y)) (Int.ofNat (Int.gcd a b))
/// (Bezout's identity as a Prop)
#[allow(dead_code)]
pub fn axiom_int_bezout_ty() -> Expr {
    forall2_int_eq(app2(
        cst("Exists"),
        int_const(),
        pi(
            BinderInfo::Default,
            "x",
            int_const(),
            app2(
                cst("Exists"),
                int_const(),
                pi(
                    BinderInfo::Default,
                    "y",
                    int_const(),
                    int_eq_expr(
                        app2(
                            cst("Int.add"),
                            app2(cst("Int.mul"), bvar(3), bvar(1)),
                            app2(cst("Int.mul"), bvar(2), bvar(0)),
                        ),
                        app(cst("Int.ofNat"), app2(cst("Int.gcd"), bvar(3), bvar(2))),
                    ),
                ),
            ),
        ),
    ))
}
/// Int.emod_emod_of_dvd : ∀ a b c, Int.dvd b c → Eq (Int.mod (Int.mod a c) b) (Int.mod a b)
#[allow(dead_code)]
pub fn axiom_int_emod_emod_of_dvd_ty() -> Expr {
    forall3_int(pi(
        BinderInfo::Default,
        "h",
        app2(cst("Int.dvd"), bvar(1), bvar(0)),
        int_eq_expr(
            app2(
                cst("Int.mod"),
                app2(cst("Int.mod"), bvar(3), bvar(1)),
                bvar(2),
            ),
            app2(cst("Int.mod"), bvar(3), bvar(2)),
        ),
    ))
}
/// Int.div_add_mod : ∀ a b, Eq (Int.add (Int.mul (Int.div a b) b) (Int.mod a b)) a
#[allow(dead_code)]
pub fn axiom_int_div_add_mod_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(
            cst("Int.add"),
            app2(
                cst("Int.mul"),
                app2(cst("Int.div"), bvar(1), bvar(0)),
                bvar(0),
            ),
            app2(cst("Int.mod"), bvar(1), bvar(0)),
        ),
        bvar(1),
    ))
}
/// Int.neg_mul : ∀ a b, Eq (Int.mul (Int.neg a) b) (Int.neg (Int.mul a b))
#[allow(dead_code)]
pub fn axiom_int_neg_mul_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(cst("Int.mul"), app(cst("Int.neg"), bvar(1)), bvar(0)),
        app(cst("Int.neg"), app2(cst("Int.mul"), bvar(1), bvar(0))),
    ))
}
/// Int.mul_neg : ∀ a b, Eq (Int.mul a (Int.neg b)) (Int.neg (Int.mul a b))
#[allow(dead_code)]
pub fn axiom_int_mul_neg_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(cst("Int.mul"), bvar(1), app(cst("Int.neg"), bvar(0))),
        app(cst("Int.neg"), app2(cst("Int.mul"), bvar(1), bvar(0))),
    ))
}
/// Int.neg_mul_neg : ∀ a b, Eq (Int.mul (Int.neg a) (Int.neg b)) (Int.mul a b)
#[allow(dead_code)]
pub fn axiom_int_neg_mul_neg_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(
            cst("Int.mul"),
            app(cst("Int.neg"), bvar(1)),
            app(cst("Int.neg"), bvar(0)),
        ),
        app2(cst("Int.mul"), bvar(1), bvar(0)),
    ))
}
/// Int.sub_eq_add_neg : ∀ a b, Eq (Int.sub a b) (Int.add a (Int.neg b))
#[allow(dead_code)]
pub fn axiom_int_sub_eq_add_neg_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(cst("Int.sub"), bvar(1), bvar(0)),
        app2(cst("Int.add"), bvar(1), app(cst("Int.neg"), bvar(0))),
    ))
}
/// Int.add_sub_cancel : ∀ a b, Eq (Int.sub (Int.add a b) b) a
#[allow(dead_code)]
pub fn axiom_int_add_sub_cancel_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(
            cst("Int.sub"),
            app2(cst("Int.add"), bvar(1), bvar(0)),
            bvar(0),
        ),
        bvar(1),
    ))
}
/// Int.sub_add_cancel : ∀ a b, Eq (Int.add (Int.sub a b) b) a
#[allow(dead_code)]
pub fn axiom_int_sub_add_cancel_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app2(
            cst("Int.add"),
            app2(cst("Int.sub"), bvar(1), bvar(0)),
            bvar(0),
        ),
        bvar(1),
    ))
}
/// Int.neg_add : ∀ a b, Eq (Int.neg (Int.add a b)) (Int.add (Int.neg a) (Int.neg b))
#[allow(dead_code)]
pub fn axiom_int_neg_add_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app(cst("Int.neg"), app2(cst("Int.add"), bvar(1), bvar(0))),
        app2(
            cst("Int.add"),
            app(cst("Int.neg"), bvar(1)),
            app(cst("Int.neg"), bvar(0)),
        ),
    ))
}
/// Int.mul_add : ∀ a b c, Eq (Int.mul a (Int.add b c)) (Int.add (Int.mul a b) (Int.mul a c))
/// (alias for left_distrib, included for completeness)
#[allow(dead_code)]
pub fn axiom_int_mul_add_ty() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), bvar(0)),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
        ),
    ))
}
/// Int.add_mul : ∀ a b c, Eq (Int.mul (Int.add a b) c) (Int.add (Int.mul a c) (Int.mul b c))
#[allow(dead_code)]
pub fn axiom_int_add_mul_ty() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            app2(cst("Int.add"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
            app2(cst("Int.mul"), bvar(1), bvar(0)),
        ),
    ))
}
/// Int.mul_sub : ∀ a b c, Eq (Int.mul a (Int.sub b c)) (Int.sub (Int.mul a b) (Int.mul a c))
#[allow(dead_code)]
pub fn axiom_int_mul_sub_ty() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.sub"), bvar(1), bvar(0)),
        ),
        app2(
            cst("Int.sub"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
        ),
    ))
}
/// Int.abs_add : ∀ a b, Int.le (Int.ofNat (Int.abs (Int.add a b))) (Nat.add (Int.abs a) (Int.abs b))
/// (triangle inequality for integers as Prop: |a+b| ≤ |a| + |b|)
#[allow(dead_code)]
pub fn axiom_int_abs_add_ty() -> Expr {
    forall2_int_eq(app2(
        cst("Nat.le"),
        app(cst("Int.abs"), app2(cst("Int.add"), bvar(1), bvar(0))),
        app2(
            cst("Nat.add"),
            app(cst("Int.abs"), bvar(1)),
            app(cst("Int.abs"), bvar(0)),
        ),
    ))
}
/// Int.abs_nonneg : ∀ a, Nat.le 0 (Int.abs a)
#[allow(dead_code)]
pub fn axiom_int_abs_nonneg_ty() -> Expr {
    forall1_int(app2(
        cst("Nat.le"),
        Expr::Lit(Literal::Nat(0)),
        app(cst("Int.abs"), bvar(0)),
    ))
}
/// Int.prime : Int → Prop
#[allow(dead_code)]
pub fn axiom_int_prime_ty() -> Expr {
    pi(BinderInfo::Default, "p", int_const(), prop())
}
/// Int.prime_dvd_mul : ∀ p a b, Int.prime p → Int.dvd p (Int.mul a b)
///   → Or (Int.dvd p a) (Int.dvd p b)
#[allow(dead_code)]
pub fn axiom_int_prime_dvd_mul_ty() -> Expr {
    forall3_int(pi(
        BinderInfo::Default,
        "hp",
        app(cst("Int.prime"), bvar(2)),
        pi(
            BinderInfo::Default,
            "hdvd",
            app2(
                cst("Int.dvd"),
                bvar(3),
                app2(cst("Int.mul"), bvar(2), bvar(1)),
            ),
            app2(
                cst("Or"),
                app2(cst("Int.dvd"), bvar(4), bvar(3)),
                app2(cst("Int.dvd"), bvar(4), bvar(2)),
            ),
        ),
    ))
}
/// Int.coprime : Int → Int → Prop  (gcd = 1)
#[allow(dead_code)]
pub fn axiom_int_coprime_ty() -> Expr {
    int_rel_ty()
}
/// Int.coprime_comm : ∀ a b, Iff (Int.coprime a b) (Int.coprime b a)
#[allow(dead_code)]
pub fn axiom_int_coprime_comm_ty() -> Expr {
    forall2_int_eq(app2(
        cst("Iff"),
        app2(cst("Int.coprime"), bvar(1), bvar(0)),
        app2(cst("Int.coprime"), bvar(0), bvar(1)),
    ))
}
/// Int.chinese_remainder : ∀ a b m n, Int.coprime m n
///   → ∃ x, And (Eq (Int.mod x m) (Int.mod a m)) (Eq (Int.mod x n) (Int.mod b n))
#[allow(dead_code)]
pub fn axiom_int_chinese_remainder_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(
            BinderInfo::Default,
            "b",
            int_const(),
            pi(
                BinderInfo::Default,
                "m",
                int_const(),
                pi(
                    BinderInfo::Default,
                    "n",
                    int_const(),
                    pi(
                        BinderInfo::Default,
                        "hcop",
                        app2(cst("Int.coprime"), bvar(1), bvar(0)),
                        app2(
                            cst("Exists"),
                            int_const(),
                            pi(
                                BinderInfo::Default,
                                "x",
                                int_const(),
                                app2(
                                    cst("And"),
                                    int_eq_expr(
                                        app2(cst("Int.mod"), bvar(0), bvar(3)),
                                        app2(cst("Int.mod"), bvar(5), bvar(3)),
                                    ),
                                    int_eq_expr(
                                        app2(cst("Int.mod"), bvar(0), bvar(3)),
                                        app2(cst("Int.mod"), bvar(5), bvar(3)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Int.natAbs : Int → Nat
#[allow(dead_code)]
pub fn axiom_int_nat_abs_ty() -> Expr {
    pi(BinderInfo::Default, "a", int_const(), nat_const())
}
/// Int.natAbs_eq : ∀ a, Eq (Int.ofNat (Int.natAbs a)) (Int.abs a |> Int.ofNat)
#[allow(dead_code)]
pub fn axiom_int_nat_abs_eq_ty() -> Expr {
    forall1_int(nat_eq_expr(
        app(cst("Int.natAbs"), bvar(0)),
        app(cst("Int.abs"), bvar(0)),
    ))
}
/// Int.pow : Int → Nat → Int
#[allow(dead_code)]
pub fn axiom_int_pow_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_const(),
        pi(BinderInfo::Default, "n", nat_const(), int_const()),
    )
}
/// Int.pow_zero : ∀ a, Eq (Int.pow a 0) (Int.ofNat 1)
#[allow(dead_code)]
pub fn axiom_int_pow_zero_ty() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.pow"), bvar(0), Expr::Lit(Literal::Nat(0))),
        app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(1))),
    ))
}
/// Int.pow_succ : ∀ a n, Eq (Int.pow a (Nat.succ n)) (Int.mul (Int.pow a n) a)
#[allow(dead_code)]
pub fn axiom_int_pow_succ_ty() -> Expr {
    forall1_int(pi(
        BinderInfo::Default,
        "n",
        nat_const(),
        int_eq_expr(
            app2(cst("Int.pow"), bvar(1), app(cst("Nat.succ"), bvar(0))),
            app2(
                cst("Int.mul"),
                app2(cst("Int.pow"), bvar(1), bvar(0)),
                bvar(1),
            ),
        ),
    ))
}
/// Int.pow_add : ∀ a m n, Eq (Int.pow a (Nat.add m n)) (Int.mul (Int.pow a m) (Int.pow a n))
#[allow(dead_code)]
pub fn axiom_int_pow_add_ty() -> Expr {
    forall1_int(pi(
        BinderInfo::Default,
        "m",
        nat_const(),
        pi(
            BinderInfo::Default,
            "n",
            nat_const(),
            int_eq_expr(
                app2(
                    cst("Int.pow"),
                    bvar(2),
                    app2(cst("Nat.add"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("Int.mul"),
                    app2(cst("Int.pow"), bvar(2), bvar(1)),
                    app2(cst("Int.pow"), bvar(2), bvar(0)),
                ),
            ),
        ),
    ))
}
/// Int.sq_nonneg : ∀ a, Int.le (Int.ofNat 0) (Int.mul a a)
#[allow(dead_code)]
pub fn axiom_int_sq_nonneg_ty() -> Expr {
    forall1_int(app2(
        cst("Int.le"),
        app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
        app2(cst("Int.mul"), bvar(0), bvar(0)),
    ))
}
/// Int.le_add_right : ∀ a b, b ≥ 0 → Int.le a (Int.add a b)
#[allow(dead_code)]
pub fn axiom_int_le_add_right_ty() -> Expr {
    forall2_int_eq(pi(
        BinderInfo::Default,
        "hb",
        app2(
            cst("Int.le"),
            app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0))),
            bvar(0),
        ),
        app2(
            cst("Int.le"),
            bvar(2),
            app2(cst("Int.add"), bvar(2), bvar(1)),
        ),
    ))
}
/// Int.add_le_add_left : ∀ a b c, Int.le a b → Int.le (Int.add c a) (Int.add c b)
#[allow(dead_code)]
pub fn axiom_int_add_le_add_left_ty() -> Expr {
    forall3_int(pi(
        BinderInfo::Default,
        "h",
        app2(cst("Int.le"), bvar(2), bvar(1)),
        app2(
            cst("Int.le"),
            app2(cst("Int.add"), bvar(1), bvar(3)),
            app2(cst("Int.add"), bvar(1), bvar(2)),
        ),
    ))
}
/// Int.mul_pos : ∀ a b, Int.lt 0 a → Int.lt 0 b → Int.lt 0 (Int.mul a b)
#[allow(dead_code)]
pub fn axiom_int_mul_pos_ty() -> Expr {
    let zero = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)));
    forall2_int_eq(pi(
        BinderInfo::Default,
        "ha",
        app2(cst("Int.lt"), zero.clone(), bvar(1)),
        pi(
            BinderInfo::Default,
            "hb",
            app2(cst("Int.lt"), zero.clone(), bvar(1)),
            app2(cst("Int.lt"), zero, app2(cst("Int.mul"), bvar(3), bvar(2))),
        ),
    ))
}
/// Int.ediv_nonneg : ∀ a b, Int.le 0 a → Int.lt 0 b → Int.le 0 (Int.div a b)
#[allow(dead_code)]
pub fn axiom_int_ediv_nonneg_ty() -> Expr {
    let zero = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)));
    forall2_int_eq(pi(
        BinderInfo::Default,
        "ha",
        app2(cst("Int.le"), zero.clone(), bvar(1)),
        pi(
            BinderInfo::Default,
            "hb",
            app2(cst("Int.lt"), zero.clone(), bvar(1)),
            app2(cst("Int.le"), zero, app2(cst("Int.div"), bvar(3), bvar(2))),
        ),
    ))
}
/// Int.toNat : Int → Nat
#[allow(dead_code)]
pub fn axiom_int_to_nat_ty() -> Expr {
    pi(BinderInfo::Default, "a", int_const(), nat_const())
}
/// Int.toNat_ofNat : ∀ n, Eq (Int.toNat (Int.ofNat n)) n
#[allow(dead_code)]
pub fn axiom_int_to_nat_of_nat_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_const(),
        nat_eq_expr(
            app(cst("Int.toNat"), app(cst("Int.ofNat"), bvar(0))),
            bvar(0),
        ),
    )
}
/// Int.sign_mul : ∀ a b, Eq (Int.sign (Int.mul a b)) (Int.mul (Int.sign a) (Int.sign b))
#[allow(dead_code)]
pub fn axiom_int_sign_mul_ty() -> Expr {
    forall2_int_eq(int_eq_expr(
        app(cst("Int.sign"), app2(cst("Int.mul"), bvar(1), bvar(0))),
        app2(
            cst("Int.mul"),
            app(cst("Int.sign"), bvar(1)),
            app(cst("Int.sign"), bvar(0)),
        ),
    ))
}
/// Int.sign_neg : ∀ a, Eq (Int.sign (Int.neg a)) (Int.neg (Int.sign a))
#[allow(dead_code)]
pub fn axiom_int_sign_neg_ty() -> Expr {
    forall1_int(int_eq_expr(
        app(cst("Int.sign"), app(cst("Int.neg"), bvar(0))),
        app(cst("Int.neg"), app(cst("Int.sign"), bvar(0))),
    ))
}
/// Int.mod_nonneg : ∀ a b, Int.lt 0 b → Int.le 0 (Int.mod a b)
#[allow(dead_code)]
pub fn axiom_int_mod_nonneg_ty() -> Expr {
    let zero = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)));
    forall2_int_eq(pi(
        BinderInfo::Default,
        "hb",
        app2(cst("Int.lt"), zero.clone(), bvar(0)),
        app2(cst("Int.le"), zero, app2(cst("Int.mod"), bvar(2), bvar(1))),
    ))
}
/// Int.mod_lt : ∀ a b, Int.lt 0 b → Int.lt (Int.mod a b) b
#[allow(dead_code)]
pub fn axiom_int_mod_lt_ty() -> Expr {
    let zero = app(cst("Int.ofNat"), Expr::Lit(Literal::Nat(0)));
    forall2_int_eq(pi(
        BinderInfo::Default,
        "hb",
        app2(cst("Int.lt"), zero, bvar(0)),
        app2(
            cst("Int.lt"),
            app2(cst("Int.mod"), bvar(2), bvar(1)),
            bvar(1),
        ),
    ))
}
/// Register all extended integer axioms into the environment.
#[allow(clippy::too_many_lines)]
pub fn register_int_extended(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Int.lcm", vec![], axiom_int_lcm_ty())?;
    add_axiom(env, "Int.lcm_comm", vec![], axiom_int_lcm_comm_ty())?;
    add_axiom(env, "Int.gcd_comm", vec![], axiom_int_gcd_comm_ty())?;
    add_axiom(env, "Int.gcd_self", vec![], axiom_int_gcd_self_ty())?;
    add_axiom(
        env,
        "Int.gcd_zero_right",
        vec![],
        axiom_int_gcd_zero_right_ty(),
    )?;
    add_axiom(
        env,
        "Int.gcd_zero_left",
        vec![],
        axiom_int_gcd_zero_left_ty(),
    )?;
    add_axiom(env, "Int.dvd", vec![], axiom_int_dvd_ty())?;
    add_axiom(env, "Int.dvd_refl", vec![], axiom_int_dvd_refl_ty())?;
    add_axiom(env, "Int.dvd_trans", vec![], axiom_int_dvd_trans_ty())?;
    add_axiom(env, "Int.dvd_antisymm", vec![], axiom_int_dvd_antisymm_ty())?;
    add_axiom(env, "Int.bezout", vec![], axiom_int_bezout_ty())?;
    add_axiom(
        env,
        "Int.emod_emod_of_dvd",
        vec![],
        axiom_int_emod_emod_of_dvd_ty(),
    )?;
    add_axiom(env, "Int.div_add_mod", vec![], axiom_int_div_add_mod_ty())?;
    add_axiom(env, "Int.neg_mul", vec![], axiom_int_neg_mul_ty())?;
    add_axiom(env, "Int.mul_neg", vec![], axiom_int_mul_neg_ty())?;
    add_axiom(env, "Int.neg_mul_neg", vec![], axiom_int_neg_mul_neg_ty())?;
    add_axiom(
        env,
        "Int.sub_eq_add_neg",
        vec![],
        axiom_int_sub_eq_add_neg_ty(),
    )?;
    add_axiom(
        env,
        "Int.add_sub_cancel",
        vec![],
        axiom_int_add_sub_cancel_ty(),
    )?;
    add_axiom(
        env,
        "Int.sub_add_cancel",
        vec![],
        axiom_int_sub_add_cancel_ty(),
    )?;
    add_axiom(env, "Int.neg_add", vec![], axiom_int_neg_add_ty())?;
    add_axiom(env, "Int.mul_add", vec![], axiom_int_mul_add_ty())?;
    add_axiom(env, "Int.add_mul", vec![], axiom_int_add_mul_ty())?;
    add_axiom(env, "Int.mul_sub", vec![], axiom_int_mul_sub_ty())?;
    add_axiom(env, "Int.abs_add", vec![], axiom_int_abs_add_ty())?;
    add_axiom(env, "Int.abs_nonneg", vec![], axiom_int_abs_nonneg_ty())?;
    add_axiom(env, "Int.prime", vec![], axiom_int_prime_ty())?;
    add_axiom(
        env,
        "Int.prime_dvd_mul",
        vec![],
        axiom_int_prime_dvd_mul_ty(),
    )?;
    add_axiom(env, "Int.coprime", vec![], axiom_int_coprime_ty())?;
    add_axiom(env, "Int.coprime_comm", vec![], axiom_int_coprime_comm_ty())?;
    add_axiom(
        env,
        "Int.chinese_remainder",
        vec![],
        axiom_int_chinese_remainder_ty(),
    )?;
    add_axiom(env, "Int.natAbs", vec![], axiom_int_nat_abs_ty())?;
    add_axiom(env, "Int.natAbs_eq", vec![], axiom_int_nat_abs_eq_ty())?;
    add_axiom(env, "Int.pow", vec![], axiom_int_pow_ty())?;
    add_axiom(env, "Int.pow_zero", vec![], axiom_int_pow_zero_ty())?;
    add_axiom(env, "Int.pow_succ", vec![], axiom_int_pow_succ_ty())?;
    add_axiom(env, "Int.pow_add", vec![], axiom_int_pow_add_ty())?;
    add_axiom(env, "Int.sq_nonneg", vec![], axiom_int_sq_nonneg_ty())?;
    add_axiom(env, "Int.le_add_right", vec![], axiom_int_le_add_right_ty())?;
    add_axiom(
        env,
        "Int.add_le_add_left",
        vec![],
        axiom_int_add_le_add_left_ty(),
    )?;
    add_axiom(env, "Int.mul_pos", vec![], axiom_int_mul_pos_ty())?;
    add_axiom(env, "Int.ediv_nonneg", vec![], axiom_int_ediv_nonneg_ty())?;
    add_axiom(env, "Int.toNat", vec![], axiom_int_to_nat_ty())?;
    add_axiom(env, "Int.toNat_ofNat", vec![], axiom_int_to_nat_of_nat_ty())?;
    add_axiom(env, "Int.sign_mul", vec![], axiom_int_sign_mul_ty())?;
    add_axiom(env, "Int.sign_neg", vec![], axiom_int_sign_neg_ty())?;
    add_axiom(env, "Int.mod_nonneg", vec![], axiom_int_mod_nonneg_ty())?;
    add_axiom(env, "Int.mod_lt", vec![], axiom_int_mod_lt_ty())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_int_env() {
        let mut env = Environment::new();
        assert!(build_int_env(&mut env).is_ok());
        assert!(env.contains(&Name::str("Int")));
        assert!(env.contains(&Name::str("Int.ofNat")));
        assert!(env.contains(&Name::str("Int.negSucc")));
        assert!(env.contains(&Name::str("Int.add")));
        assert!(env.contains(&Name::str("Int.mul")));
        assert!(env.contains(&Name::str("Int.sub")));
        assert!(env.contains(&Name::str("Int.neg")));
        assert!(env.contains(&Name::str("Int.abs")));
        assert!(env.contains(&Name::str("Int.sign")));
        assert!(env.contains(&Name::str("Int.div")));
        assert!(env.contains(&Name::str("Int.mod")));
        assert!(env.contains(&Name::str("Int.gcd")));
        assert!(env.contains(&Name::str("Int.le")));
        assert!(env.contains(&Name::str("Int.lt")));
        assert!(env.contains(&Name::str("Int.beq")));
        assert!(env.contains(&Name::str("Int.ble")));
        assert!(env.contains(&Name::str("Int.rec")));
        assert!(env.contains(&Name::str("Int.ofNat_zero")));
        assert!(env.contains(&Name::str("Int.ofNat_succ")));
        assert!(env.contains(&Name::str("Int.add_comm")));
        assert!(env.contains(&Name::str("Int.add_assoc")));
        assert!(env.contains(&Name::str("Int.add_zero")));
        assert!(env.contains(&Name::str("Int.zero_add")));
        assert!(env.contains(&Name::str("Int.add_neg_cancel")));
        assert!(env.contains(&Name::str("Int.neg_add_cancel")));
        assert!(env.contains(&Name::str("Int.mul_comm")));
        assert!(env.contains(&Name::str("Int.mul_assoc")));
        assert!(env.contains(&Name::str("Int.mul_one")));
        assert!(env.contains(&Name::str("Int.one_mul")));
        assert!(env.contains(&Name::str("Int.mul_zero")));
        assert!(env.contains(&Name::str("Int.zero_mul")));
        assert!(env.contains(&Name::str("Int.left_distrib")));
        assert!(env.contains(&Name::str("Int.right_distrib")));
        assert!(env.contains(&Name::str("Int.neg_neg")));
        assert!(env.contains(&Name::str("Int.neg_zero")));
        assert!(env.contains(&Name::str("Int.sub_self")));
        assert!(env.contains(&Name::str("Int.le_refl")));
        assert!(env.contains(&Name::str("Int.le_trans")));
        assert!(env.contains(&Name::str("Int.le_antisymm")));
        assert!(env.contains(&Name::str("Int.lt_iff_le_not_le")));
        assert!(env.contains(&Name::str("Int.abs_neg")));
        assert!(env.contains(&Name::str("Int.abs_mul")));
        assert!(env.contains(&Name::str("Int.ofNat_add")));
        assert!(env.contains(&Name::str("Int.ofNat_mul")));
    }
    #[test]
    fn test_int_of_nat() {
        let e = int_of_nat(Expr::Lit(Literal::Nat(5)));
        assert!(matches!(e, Expr::App(_, _)));
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.ofNat"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(5))));
        }
    }
    #[test]
    fn test_int_neg_succ() {
        let e = int_neg_succ(Expr::Lit(Literal::Nat(3)));
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.negSucc"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(3))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_add() {
        let a = int_zero();
        let b = int_one();
        let e = int_add(a, b);
        assert!(matches!(e, Expr::App(_, _)));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Int.add"));
                }
            }
        }
    }
    #[test]
    fn test_int_mul() {
        let a = int_one();
        let b = int_lit(2);
        let e = int_mul(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_sub() {
        let a = int_lit(5);
        let b = int_lit(3);
        let e = int_sub(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_neg() {
        let a = int_one();
        let e = int_neg(a);
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.neg"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_abs() {
        let a = int_lit(-3);
        let e = int_abs(a);
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.abs"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_sign() {
        let a = int_one();
        let e = int_sign(a);
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.sign"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_div() {
        let a = int_lit(10);
        let b = int_lit(3);
        let e = int_div(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_mod() {
        let a = int_lit(10);
        let b = int_lit(3);
        let e = int_mod(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_gcd() {
        let a = int_lit(12);
        let b = int_lit(8);
        let e = int_gcd(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_le() {
        let a = int_zero();
        let b = int_one();
        let e = int_le(a, b);
        assert!(matches!(e, Expr::App(_, _)));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Int.le"));
                }
            }
        }
    }
    #[test]
    fn test_int_lt() {
        let a = int_zero();
        let b = int_one();
        let e = int_lt(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_beq() {
        let a = int_one();
        let b = int_one();
        let e = int_beq(a, b);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_int_lit_positive() {
        let e = int_lit(42);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.ofNat"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(42))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_lit_negative() {
        let e = int_lit(-5);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.negSucc"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(4))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_lit_zero() {
        let e = int_lit(0);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.ofNat"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(0))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_zero() {
        let e = int_zero();
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.ofNat"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(0))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_one() {
        let e = int_one();
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.ofNat"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(1))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_int_eq() {
        let a = int_one();
        let b = int_lit(2);
        let e = mk_int_eq(a, b);
        assert!(matches!(e, Expr::App(_, _)));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::App(h, ty) = g.as_ref() {
                    if let Expr::Const(n, _) = h.as_ref() {
                        assert_eq!(*n, Name::str("Eq"));
                    }
                    if let Expr::Const(tn, _) = ty.as_ref() {
                        assert_eq!(*tn, Name::str("Int"));
                    }
                }
            }
        }
    }
    #[test]
    fn test_int_lit_neg_one() {
        let e = int_lit(-1);
        if let Expr::App(f, arg) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Int.negSucc"));
            }
            assert!(matches!(arg.as_ref(), Expr::Lit(Literal::Nat(0))));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_ty() {
        let e = int_ty();
        if let Expr::Const(n, lvls) = &e {
            assert_eq!(*n, Name::str("Int"));
            assert!(lvls.is_empty());
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_int_add_structure() {
        let a = int_lit(3);
        let b = int_lit(4);
        let e = int_add(a, b);
        if let Expr::App(f, _rhs) = &e {
            if let Expr::App(g, _lhs) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Int.add"));
                } else {
                    panic!("expected Const for Int.add");
                }
            } else {
                panic!("expected nested App");
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_int_gcd_structure() {
        let a = int_lit(6);
        let b = int_lit(4);
        let e = int_gcd(a, b);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Int.gcd"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_int_eq_structure() {
        let a = int_zero();
        let b = int_zero();
        let e = mk_int_eq(a, b);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::App(h, _) = g.as_ref() {
                    if let Expr::Const(n, _) = h.as_ref() {
                        assert_eq!(*n, Name::str("Eq"));
                    }
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_axiom_int_lcm_ty() {
        let ty = axiom_int_lcm_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_int_dvd_ty() {
        let ty = axiom_int_dvd_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_int_bezout_ty() {
        let ty = axiom_int_bezout_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_int_pow_ty() {
        let ty = axiom_int_pow_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_register_int_extended() {
        let mut env = Environment::new();
        build_int_env(&mut env).expect("build_int_env should succeed");
        let nat_binop_ty = pi(
            BinderInfo::Default,
            "a",
            nat_const(),
            pi(BinderInfo::Default, "b", nat_const(), nat_const()),
        );
        let nat_rel_ty = pi(
            BinderInfo::Default,
            "a",
            nat_const(),
            pi(BinderInfo::Default, "b", nat_const(), prop()),
        );
        add_axiom(&mut env, "Nat.add", vec![], nat_binop_ty.clone()).ok();
        add_axiom(&mut env, "Nat.mul", vec![], nat_binop_ty).ok();
        add_axiom(&mut env, "Nat.le", vec![], nat_rel_ty).ok();
        add_axiom(&mut env, "Exists", vec![Name::str("u")], type1()).ok();
        add_axiom(&mut env, "And", vec![], prop()).ok();
        add_axiom(&mut env, "Or", vec![], prop()).ok();
        add_axiom(&mut env, "Iff", vec![], prop()).ok();
        add_axiom(
            &mut env,
            "Not",
            vec![],
            pi(BinderInfo::Default, "_", prop(), prop()),
        )
        .ok();
        let result = register_int_extended(&mut env);
        assert!(result.is_ok(), "register_int_extended failed: {:?}", result);
        assert!(env.contains(&Name::str("Int.lcm")));
        assert!(env.contains(&Name::str("Int.dvd")));
        assert!(env.contains(&Name::str("Int.bezout")));
        assert!(env.contains(&Name::str("Int.pow")));
        assert!(env.contains(&Name::str("Int.prime")));
        assert!(env.contains(&Name::str("Int.coprime")));
        assert!(env.contains(&Name::str("Int.toNat")));
    }
    #[test]
    fn test_int_ext_euclid_basic() {
        let r = IntExtEuclidResult::compute(12, 8);
        assert_eq!(r.gcd, 4);
        assert!(r.verify(12, 8));
    }
    #[test]
    fn test_int_ext_euclid_coprime() {
        let r = IntExtEuclidResult::compute(7, 11);
        assert_eq!(r.gcd, 1);
        assert!(r.is_coprime());
        assert!(r.verify(7, 11));
    }
    #[test]
    fn test_int_ext_euclid_modular_inverse() {
        let r = IntExtEuclidResult::compute(3, 7);
        let inv = r
            .modular_inverse(3, 7)
            .expect("modular_inverse should succeed");
        assert_eq!((3 * inv) % 7, 1);
    }
    #[test]
    fn test_int_ext_euclid_no_inverse() {
        let r = IntExtEuclidResult::compute(6, 4);
        assert!(r.modular_inverse(6, 4).is_none());
    }
    #[test]
    fn test_int_crt_solver_empty() {
        let solver = IntCrtSolver::new();
        assert_eq!(solver.solve(), Some((0, 1)));
        assert!(solver.is_empty());
    }
    #[test]
    fn test_int_crt_solver_single() {
        let mut solver = IntCrtSolver::new();
        solver.add_congruence(3, 7);
        let (x, m) = solver.solve().expect("solve should succeed");
        assert_eq!(x % 7, 3);
        assert_eq!(m, 7);
    }
    #[test]
    fn test_int_crt_solver_two_congruences() {
        let mut solver = IntCrtSolver::new();
        solver.add_congruence(2, 3);
        solver.add_congruence(3, 5);
        let (x, m) = solver.solve().expect("solve should succeed");
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
        assert_eq!(m, 15);
    }
    #[test]
    fn test_int_ext_padic_val_basic() {
        let v = IntExtPadicVal::new(2);
        assert_eq!(v.val(8), Some(3));
        assert_eq!(v.val(12), Some(2));
        assert_eq!(v.val(7), Some(0));
    }
    #[test]
    fn test_int_ext_padic_val_zero() {
        let v = IntExtPadicVal::new(2);
        assert!(v.val(0).is_none());
    }
    #[test]
    fn test_int_ext_padic_unit_part() {
        let v = IntExtPadicVal::new(3);
        assert_eq!(v.unit_part(18), Some(2));
    }
    #[test]
    fn test_int_ext_gaussian_mul() {
        let a = IntExtGaussian::new(3, 2);
        let b = IntExtGaussian::new(1, 4);
        let c = a.mul(b);
        assert_eq!(c.re, -5);
        assert_eq!(c.im, 14);
    }
    #[test]
    fn test_int_ext_gaussian_norm() {
        let a = IntExtGaussian::new(3, 4);
        assert_eq!(a.norm(), 25);
    }
    #[test]
    fn test_int_ext_gaussian_units() {
        for u in &IntExtGaussian::units() {
            assert!(u.is_unit());
        }
    }
    #[test]
    fn test_int_ext_gaussian_divides() {
        let a = IntExtGaussian::new(1, 1);
        let b = IntExtGaussian::new(3, 1);
        let two = IntExtGaussian::new(2, 0);
        assert!(a.divides(two));
        let _ = a.divides(b);
    }
    #[test]
    fn test_int_ext_cf_basic() {
        let cf = IntExtContinuedFraction::from_ratio(7, 3);
        assert_eq!(cf.coeffs[0], 2);
        let (p, q) = cf.to_ratio();
        assert_eq!(p * 3, q * 7 / 7 * 7);
        let _ = (p, q);
    }
    #[test]
    fn test_int_ext_cf_identity() {
        let cf = IntExtContinuedFraction::from_ratio(355, 113);
        let (p, q) = cf.to_ratio();
        assert_eq!(p, 355);
        assert_eq!(q, 113);
    }
    #[test]
    fn test_int_ext_cf_convergent() {
        let cf = IntExtContinuedFraction::from_ratio(355, 113);
        let cv = cf.convergent(0);
        assert!(cv.is_some());
    }
    #[test]
    fn test_int_ext_stern_brocot_root() {
        let r = IntExtSternBrocot::root();
        assert_eq!(r.p, 1);
        assert_eq!(r.q, 1);
    }
    #[test]
    fn test_int_ext_stern_brocot_children() {
        let r = IntExtSternBrocot::root();
        let l = r.left_child();
        let right = r.right_child();
        assert!(l.to_f64() < r.to_f64());
        assert!(right.to_f64() > r.to_f64());
    }
    #[test]
    fn test_int_ext_stern_brocot_mediant() {
        let a = IntExtSternBrocot { p: 1, q: 3 };
        let b = IntExtSternBrocot { p: 1, q: 2 };
        let m = a.mediant(b);
        assert_eq!(m.p, 2);
        assert_eq!(m.q, 5);
    }
    #[test]
    fn test_int_ext_euclid_zero_b() {
        let r = IntExtEuclidResult::compute(15, 0);
        assert_eq!(r.gcd, 15);
        assert!(r.verify(15, 0));
    }
    #[test]
    fn test_int_ext_euclid_negative() {
        let r = IntExtEuclidResult::compute(-12, 8);
        assert_eq!(r.gcd, 4);
        assert!(r.verify(-12, 8));
    }
    #[test]
    fn test_int_crt_solver_incompatible() {
        let mut solver = IntCrtSolver::new();
        solver.add_congruence(1, 2);
        solver.add_congruence(0, 4);
        assert!(solver.solve().is_none());
    }
    #[test]
    fn test_int_ext_padic_power() {
        let v = IntExtPadicVal::new(5);
        assert_eq!(v.power(3), 125);
    }
    #[test]
    fn test_int_ext_padic_divides() {
        let v = IntExtPadicVal::new(2);
        assert!(v.divides_to_val(8, 4));
        assert!(!v.divides_to_val(4, 8));
    }
    #[test]
    fn test_int_ext_gaussian_add() {
        let a = IntExtGaussian::new(3, 4);
        let b = IntExtGaussian::new(-1, 2);
        let c = a.add(b);
        assert_eq!(c.re, 2);
        assert_eq!(c.im, 6);
    }
    #[test]
    fn test_int_ext_gaussian_conj() {
        let a = IntExtGaussian::new(3, 4);
        let conj = a.conj();
        assert_eq!(conj.re, 3);
        assert_eq!(conj.im, -4);
        let prod = a.mul(conj);
        assert_eq!(prod.re, 25);
        assert_eq!(prod.im, 0);
    }
    #[test]
    fn test_int_ext_gaussian_display() {
        let a = IntExtGaussian::new(3, 4);
        assert_eq!(format!("{}", a), "3+4i");
        let b = IntExtGaussian::new(3, -4);
        assert_eq!(format!("{}", b), "3-4i");
    }
    #[test]
    fn test_int_ext_cf_depth() {
        let cf = IntExtContinuedFraction::from_ratio(22, 7);
        assert!(cf.depth() > 0);
    }
    #[test]
    fn test_int_ext_cf_convergents_sequence() {
        let cf = IntExtContinuedFraction {
            coeffs: vec![1, 1, 1, 1, 1],
        };
        let (p, q) = cf.convergent(4).expect("convergent should succeed");
        assert_eq!(p, 8);
        assert_eq!(q, 5);
    }
    #[test]
    fn test_int_ext_stern_brocot_approximate() {
        let approx = IntExtSternBrocot::approximate(1, 3, 20);
        let diff = (approx.to_f64() - 1.0 / 3.0).abs();
        assert!(diff < 0.1);
    }
}
