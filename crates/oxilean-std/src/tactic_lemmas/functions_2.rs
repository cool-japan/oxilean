//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;

/// `ring_mul_left_comm : ∀ (a b c : α), a * (b * c) = b * (a * c)` (for a comm ring)
#[allow(dead_code)]
pub fn tl2_ext_ring_mul_left_comm_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "mul",
            arrow(bvar(0), arrow(bvar(1), bvar(2))),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "c",
                        bvar(3),
                        mk_eq(
                            bvar(4),
                            app2(bvar(3), bvar(2), app2(bvar(3), bvar(1), bvar(0))),
                            app2(bvar(3), bvar(1), app2(bvar(3), bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ring_add_zero : ∀ {α} (add : α→α→α) (zero : α) (a : α), add a zero = a`
#[allow(dead_code)]
pub fn tl2_ext_ring_add_zero_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "add",
            arrow(bvar(0), arrow(bvar(1), bvar(2))),
            pi(
                BinderInfo::Default,
                "zero",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(2),
                    mk_eq(bvar(3), app2(bvar(2), bvar(0), bvar(1)), bvar(0)),
                ),
            ),
        ),
    )
}
/// `ring_zero_add : ∀ {α} (add : α→α→α) (zero : α) (a : α), add zero a = a`
#[allow(dead_code)]
pub fn tl2_ext_ring_zero_add_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "add",
            arrow(bvar(0), arrow(bvar(1), bvar(2))),
            pi(
                BinderInfo::Default,
                "zero",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(2),
                    mk_eq(bvar(3), app2(bvar(2), bvar(1), bvar(0)), bvar(0)),
                ),
            ),
        ),
    )
}
/// `field_simp_div_self : ∀ {α} (inv : α→α) (mul : α→α→α) (one : α) (a : α),`
///   `a ≠ 0 → mul a (inv a) = one`
#[allow(dead_code)]
pub fn tl2_ext_field_div_self_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "inv",
            arrow(bvar(0), bvar(1)),
            pi(
                BinderInfo::Default,
                "mul",
                arrow(bvar(1), arrow(bvar(2), bvar(3))),
                pi(
                    BinderInfo::Default,
                    "one",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "zero",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            arrow(
                                mk_not(mk_eq(bvar(5), bvar(0), bvar(1))),
                                mk_eq(
                                    bvar(6),
                                    app2(bvar(3), bvar(0), app(bvar(4), bvar(0))),
                                    bvar(2),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `field_simp_inv_mul_cancel : ∀ {α} (inv mul : ...) (a : α), a ≠ 0 → inv a * a = one`
#[allow(dead_code)]
pub fn tl2_ext_field_inv_mul_cancel_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Default,
            "inv",
            arrow(bvar(0), bvar(1)),
            pi(
                BinderInfo::Default,
                "mul",
                arrow(bvar(1), arrow(bvar(2), bvar(3))),
                pi(
                    BinderInfo::Default,
                    "one",
                    bvar(2),
                    pi(
                        BinderInfo::Default,
                        "zero",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(4),
                            arrow(
                                mk_not(mk_eq(bvar(5), bvar(0), bvar(1))),
                                mk_eq(
                                    bvar(6),
                                    app2(bvar(3), app(bvar(4), bvar(0)), bvar(0)),
                                    bvar(2),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `linarith_le_trans : ∀ (a b c : Nat), a ≤ b → b ≤ c → a ≤ c`
#[allow(dead_code)]
pub fn tl2_ext_linarith_le_trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    nat_lt(bvar(2), bvar(1)),
                    arrow(nat_lt(bvar(1), bvar(0)), nat_lt(bvar(2), bvar(0))),
                ),
            ),
        ),
    )
}
/// `linarith_add_le_add : ∀ (a b c d : Nat), a ≤ b → c ≤ d → a + c ≤ b + d`
#[allow(dead_code)]
pub fn tl2_ext_linarith_add_le_add_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "d",
                    nat_ty(),
                    arrow(
                        nat_lt(bvar(3), bvar(2)),
                        arrow(
                            nat_lt(bvar(1), bvar(0)),
                            nat_lt(nat_add(bvar(3), bvar(1)), nat_add(bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `linarith_neg_le_neg : ∀ (a b : Int), a ≤ b → -b ≤ -a`
#[allow(dead_code)]
pub fn tl2_ext_linarith_neg_le_neg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        pi(
            BinderInfo::Default,
            "b",
            int_ty(),
            arrow(
                int_le(bvar(1), bvar(0)),
                int_le(int_neg(bvar(0)), int_neg(bvar(1))),
            ),
        ),
    )
}
/// `positivity_sq_nonneg : ∀ (a : Int), 0 ≤ a * a`
#[allow(dead_code)]
pub fn tl2_ext_positivity_sq_nonneg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        int_le(int_zero(), int_mul(bvar(0), bvar(0))),
    )
}
/// `positivity_add_nonneg : ∀ (a b : Int), 0 ≤ a → 0 ≤ b → 0 ≤ a + b`
#[allow(dead_code)]
pub fn tl2_ext_positivity_add_nonneg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        pi(
            BinderInfo::Default,
            "b",
            int_ty(),
            arrow(
                int_le(int_zero(), bvar(1)),
                arrow(
                    int_le(int_zero(), bvar(0)),
                    int_le(int_zero(), int_add(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `positivity_mul_nonneg : ∀ (a b : Int), 0 ≤ a → 0 ≤ b → 0 ≤ a * b`
#[allow(dead_code)]
pub fn tl2_ext_positivity_mul_nonneg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        pi(
            BinderInfo::Default,
            "b",
            int_ty(),
            arrow(
                int_le(int_zero(), bvar(1)),
                arrow(
                    int_le(int_zero(), bvar(0)),
                    int_le(int_zero(), int_mul(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `gcongr_add_left : ∀ (a b c : Nat), a ≤ b → a + c ≤ b + c`
#[allow(dead_code)]
pub fn tl2_ext_gcongr_add_left_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    nat_lt(bvar(2), bvar(1)),
                    nat_lt(nat_add(bvar(2), bvar(0)), nat_add(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `gcongr_add_right : ∀ (a b c : Nat), b ≤ c → a + b ≤ a + c`
#[allow(dead_code)]
pub fn tl2_ext_gcongr_add_right_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    nat_lt(bvar(1), bvar(0)),
                    nat_lt(nat_add(bvar(2), bvar(1)), nat_add(bvar(2), bvar(0))),
                ),
            ),
        ),
    )
}
/// `gcongr_mul_left : ∀ (a b c : Nat), a ≤ b → a * c ≤ b * c`
#[allow(dead_code)]
pub fn tl2_ext_gcongr_mul_left_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                arrow(
                    nat_lt(bvar(2), bvar(1)),
                    nat_lt(nat_mul(bvar(2), bvar(0)), nat_mul(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `aesop_and_intro : ∀ (p q : Prop), p → q → p ∧ q`
#[allow(dead_code)]
pub fn tl2_ext_aesop_and_intro_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "q",
            prop(),
            arrow(bvar(1), arrow(bvar(0), mk_and(bvar(3), bvar(2)))),
        ),
    )
}
/// `aesop_or_inl : ∀ (p q : Prop), p → p ∨ q`
#[allow(dead_code)]
pub fn tl2_ext_aesop_or_inl_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "q",
            prop(),
            arrow(bvar(1), mk_or(bvar(2), bvar(1))),
        ),
    )
}
/// `aesop_or_inr : ∀ (p q : Prop), q → p ∨ q`
#[allow(dead_code)]
pub fn tl2_ext_aesop_or_inr_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "q",
            prop(),
            arrow(bvar(0), mk_or(bvar(2), bvar(1))),
        ),
    )
}
/// `tauto_excluded_middle : ∀ (p : Prop), p ∨ ¬p`
#[allow(dead_code)]
pub fn tl2_ext_tauto_excluded_middle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        mk_or(bvar(0), mk_not(bvar(0))),
    )
}
/// `tauto_double_neg_elim : ∀ (p : Prop), ¬¬p → p`
#[allow(dead_code)]
pub fn tl2_ext_tauto_double_neg_elim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        arrow(mk_not(mk_not(bvar(0))), bvar(0)),
    )
}
/// `tauto_contrapositive : ∀ (p q : Prop), (p → q) → ¬q → ¬p`
#[allow(dead_code)]
pub fn tl2_ext_tauto_contrapositive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        pi(
            BinderInfo::Default,
            "q",
            prop(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(mk_not(bvar(0)), mk_not(bvar(1))),
            ),
        ),
    )
}
/// `mono_add : ∀ (a b c : Nat), a ≤ b → a + c ≤ b + c` (same as gcongr_add_left)
#[allow(dead_code)]
pub fn tl2_ext_mono_add_ty() -> Expr {
    tl2_ext_gcongr_add_left_ty()
}
/// `bound_le_refl : ∀ (n : Nat), n ≤ n`
#[allow(dead_code)]
pub fn tl2_ext_bound_le_refl_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), nat_lt(bvar(0), bvar(0)))
}
/// `decide_true : decide True = true`
#[allow(dead_code)]
pub fn tl2_ext_decide_true_ty() -> Expr {
    mk_bool_eq(app(cst("decide"), cst("True")), cst("true"))
}
/// `decide_false : decide False = false`
#[allow(dead_code)]
pub fn tl2_ext_decide_false_ty() -> Expr {
    mk_bool_eq(app(cst("decide"), cst("False")), cst("false"))
}
/// `native_decide_reflect : ∀ (p : Prop) \[Decidable p\], native_decide p = true → p`
#[allow(dead_code)]
pub fn tl2_ext_native_decide_reflect_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        prop(),
        arrow(
            mk_bool_eq(app(cst("native_decide"), bvar(0)), cst("true")),
            bvar(1),
        ),
    )
}
/// `polyrith_sub_self : ∀ (a : Int), a - a = 0`
#[allow(dead_code)]
pub fn tl2_ext_polyrith_sub_self_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        mk_int_eq(app2(cst("Int.sub"), bvar(0), bvar(0)), int_zero()),
    )
}
/// `polyrith_mul_sub : ∀ (a b c : Int), a * (b - c) = a * b - a * c`
#[allow(dead_code)]
pub fn tl2_ext_polyrith_mul_sub_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        pi(
            BinderInfo::Default,
            "b",
            int_ty(),
            pi(
                BinderInfo::Default,
                "c",
                int_ty(),
                mk_int_eq(
                    int_mul(bvar(2), app2(cst("Int.sub"), bvar(1), bvar(0))),
                    app2(
                        cst("Int.sub"),
                        int_mul(bvar(2), bvar(1)),
                        int_mul(bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `polyrith_sq_diff : ∀ (a b : Int), (a + b) * (a - b) = a * a - b * b`
#[allow(dead_code)]
pub fn tl2_ext_polyrith_sq_diff_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        int_ty(),
        pi(
            BinderInfo::Default,
            "b",
            int_ty(),
            mk_int_eq(
                int_mul(
                    int_add(bvar(1), bvar(0)),
                    app2(cst("Int.sub"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("Int.sub"),
                    int_mul(bvar(1), bvar(1)),
                    int_mul(bvar(0), bvar(0)),
                ),
            ),
        ),
    )
}
/// Register all extended tactic lemma axioms into `env`.
pub fn register_tactic_lemmas_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("tl2_nat_add_zero", tl2_ext_nat_add_zero_ty()),
        ("tl2_nat_zero_add", tl2_ext_nat_zero_add_ty()),
        ("tl2_nat_add_comm", tl2_ext_nat_add_comm_ty()),
        ("tl2_nat_add_assoc", tl2_ext_nat_add_assoc_ty()),
        ("tl2_nat_mul_zero", tl2_ext_nat_mul_zero_ty()),
        ("tl2_nat_zero_mul", tl2_ext_nat_zero_mul_ty()),
        ("tl2_nat_mul_one", tl2_ext_nat_mul_one_ty()),
        ("tl2_nat_one_mul", tl2_ext_nat_one_mul_ty()),
        ("tl2_nat_mul_comm", tl2_ext_nat_mul_comm_ty()),
        ("tl2_nat_mul_assoc", tl2_ext_nat_mul_assoc_ty()),
        ("tl2_nat_sub_self", tl2_ext_nat_sub_self_ty()),
        ("tl2_nat_sub_zero", tl2_ext_nat_sub_zero_ty()),
        ("tl2_nat_succ_ne_zero", tl2_ext_nat_succ_ne_zero_ty()),
        ("tl2_omega_int_add_zero", tl2_ext_omega_int_add_zero_ty()),
        ("tl2_omega_int_zero_add", tl2_ext_omega_int_zero_add_ty()),
        ("tl2_omega_int_add_comm", tl2_ext_omega_int_add_comm_ty()),
        (
            "tl2_omega_int_neg_add_cancel",
            tl2_ext_omega_int_neg_add_cancel_ty(),
        ),
        ("tl2_omega_int_le_refl", tl2_ext_omega_int_le_refl_ty()),
        (
            "tl2_omega_int_le_antisymm",
            tl2_ext_omega_int_le_antisymm_ty(),
        ),
        (
            "tl2_omega_int_lt_iff",
            tl2_ext_omega_int_lt_iff_add_one_le_ty(),
        ),
        ("tl2_norm_num_add_eval", tl2_ext_norm_num_add_eval_ty()),
        ("tl2_norm_num_mul_eval", tl2_ext_norm_num_mul_eval_ty()),
        ("tl2_norm_num_pow_zero", tl2_ext_norm_num_pow_zero_ty()),
        ("tl2_norm_num_pow_succ", tl2_ext_norm_num_pow_succ_ty()),
        ("tl2_ring_add_left_comm", tl2_ext_ring_add_left_comm_ty()),
        ("tl2_ring_mul_left_comm", tl2_ext_ring_mul_left_comm_ty()),
        ("tl2_ring_add_zero", tl2_ext_ring_add_zero_ty()),
        ("tl2_ring_zero_add", tl2_ext_ring_zero_add_ty()),
        ("tl2_linarith_le_trans", tl2_ext_linarith_le_trans_ty()),
        ("tl2_linarith_add_le_add", tl2_ext_linarith_add_le_add_ty()),
        ("tl2_linarith_neg_le_neg", tl2_ext_linarith_neg_le_neg_ty()),
        (
            "tl2_positivity_sq_nonneg",
            tl2_ext_positivity_sq_nonneg_ty(),
        ),
        (
            "tl2_positivity_add_nonneg",
            tl2_ext_positivity_add_nonneg_ty(),
        ),
        (
            "tl2_positivity_mul_nonneg",
            tl2_ext_positivity_mul_nonneg_ty(),
        ),
        ("tl2_gcongr_add_left", tl2_ext_gcongr_add_left_ty()),
        ("tl2_gcongr_add_right", tl2_ext_gcongr_add_right_ty()),
        ("tl2_gcongr_mul_left", tl2_ext_gcongr_mul_left_ty()),
        ("tl2_aesop_and_intro", tl2_ext_aesop_and_intro_ty()),
        ("tl2_aesop_or_inl", tl2_ext_aesop_or_inl_ty()),
        ("tl2_aesop_or_inr", tl2_ext_aesop_or_inr_ty()),
        ("tl2_tauto_excl_middle", tl2_ext_tauto_excluded_middle_ty()),
        ("tl2_tauto_dbl_neg_elim", tl2_ext_tauto_double_neg_elim_ty()),
        (
            "tl2_tauto_contrapositive",
            tl2_ext_tauto_contrapositive_ty(),
        ),
        ("tl2_decide_true", tl2_ext_decide_true_ty()),
        ("tl2_decide_false", tl2_ext_decide_false_ty()),
        (
            "tl2_native_decide_reflect",
            tl2_ext_native_decide_reflect_ty(),
        ),
        ("tl2_polyrith_sub_self", tl2_ext_polyrith_sub_self_ty()),
        ("tl2_polyrith_mul_sub", tl2_ext_polyrith_mul_sub_ty()),
        ("tl2_polyrith_sq_diff", tl2_ext_polyrith_sq_diff_ty()),
        ("tl2_congr_arg", tl2_ext_congr_arg_ty()),
        ("tl2_congr_fun", tl2_ext_congr_fun_ty()),
        ("tl2_eq_congr", tl2_ext_eq_congr_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("{e:?}"))?;
    }
    Ok(())
}
