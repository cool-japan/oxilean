//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Register all extended BitVec axioms into the environment.
///
/// This adds 35+ axiom builders covering:
/// - Shift laws (arithmetic, logical)
/// - Modular addition/multiplication with carry
/// - Signed/unsigned comparison laws
/// - Concatenation and extraction
/// - Zero/sign extension
/// - Fin isomorphism
/// - Nat and Int conversion round-trips
/// - Rotations
/// - Bit access (get/set)
/// - Population count
/// - Leading/trailing zero count
/// - Reversal and endianness
/// - SMT-LIB axioms
/// - Overflow behavior
/// - Bit tricks (isolate/clear lowest bit)
/// - SIMD-style vector axioms
#[allow(dead_code)]
#[allow(clippy::too_many_lines)]
pub fn register_bitvec_extended_axioms(env: &mut Environment) {
    let mut add = |name: &str, ty: Expr| {
        let _ = add_axiom(env, name, vec![], ty);
    };
    add("BitVec.arithShiftRight", bvx_ext_arith_shift_right_ty());
    add(
        "BitVec.shiftLeft_zero_amount",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.shiftLeft"), bvar(0), cst("Nat.zero")),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.shiftRight_zero_amount",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.shiftRight"), bvar(0), cst("Nat.zero")),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.arithShiftRight_zero_amount",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.arithShiftRight"), bvar(0), cst("Nat.zero")),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.shiftLeft_ge_width",
        bvx_ext_forall_bv_nat(|| {
            arrow(
                app3(
                    cst("Eq"),
                    bool_ty(),
                    app2(cst("Nat.ble"), bvar(2), bvar(0)),
                    cst("true"),
                ),
                mk_bv_eq(
                    bvar(2),
                    app2(cst("BitVec.shiftLeft"), bvar(1), bvar(0)),
                    app(cst("BitVec.zero"), bvar(2)),
                ),
            )
        }),
    );
    add(
        "BitVec.add_mod",
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
                    mk_nat_eq(
                        app(
                            cst("BitVec.toNat"),
                            app2(cst("BitVec.add"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Nat.mod"),
                            app2(
                                cst("Nat.add"),
                                app(cst("BitVec.toNat"), bvar(1)),
                                app(cst("BitVec.toNat"), bvar(0)),
                            ),
                            app2(
                                cst("Nat.pow"),
                                app(cst("Nat.succ"), app(cst("Nat.succ"), cst("Nat.zero"))),
                                bvar(2),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.add_neg_cancel",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.add"), bvar(0), app(cst("BitVec.neg"), bvar(0))),
                app(cst("BitVec.zero"), bvar(1)),
            )
        }),
    );
    add(
        "BitVec.mul_comm",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(cst("BitVec.mul"), bvar(1), bvar(0)),
                        app2(cst("BitVec.mul"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.mul_assoc",
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
                    pi(
                        BinderInfo::Default,
                        "c",
                        mk_bitvec(bvar(2)),
                        mk_bv_eq(
                            bvar(3),
                            app2(
                                cst("BitVec.mul"),
                                app2(cst("BitVec.mul"), bvar(2), bvar(1)),
                                bvar(0),
                            ),
                            app2(
                                cst("BitVec.mul"),
                                bvar(2),
                                app2(cst("BitVec.mul"), bvar(1), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.mul_mod",
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
                    mk_nat_eq(
                        app(
                            cst("BitVec.toNat"),
                            app2(cst("BitVec.mul"), bvar(1), bvar(0)),
                        ),
                        app2(
                            cst("Nat.mod"),
                            app2(
                                cst("Nat.mul"),
                                app(cst("BitVec.toNat"), bvar(1)),
                                app(cst("BitVec.toNat"), bvar(0)),
                            ),
                            app2(
                                cst("Nat.pow"),
                                app(cst("Nat.succ"), app(cst("Nat.succ"), cst("Nat.zero"))),
                                bvar(2),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.ult_irrefl",
        bvx_ext_forall_one(|| {
            mk_eq(
                bool_ty(),
                app2(cst("BitVec.ult"), bvar(0), bvar(0)),
                cst("false"),
            )
        }),
    );
    add(
        "BitVec.ule_refl",
        bvx_ext_forall_one(|| {
            mk_eq(
                bool_ty(),
                app2(cst("BitVec.ule"), bvar(0), bvar(0)),
                cst("true"),
            )
        }),
    );
    add(
        "BitVec.ult_ule",
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
                    arrow(
                        mk_eq(
                            bool_ty(),
                            app2(cst("BitVec.ult"), bvar(1), bvar(0)),
                            cst("true"),
                        ),
                        mk_eq(
                            bool_ty(),
                            app2(cst("BitVec.ule"), bvar(1), bvar(0)),
                            cst("true"),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.slt_irrefl",
        bvx_ext_forall_one(|| {
            mk_eq(
                bool_ty(),
                app2(cst("BitVec.slt"), bvar(0), bvar(0)),
                cst("false"),
            )
        }),
    );
    add(
        "BitVec.sle_refl",
        bvx_ext_forall_one(|| {
            mk_eq(
                bool_ty(),
                app2(cst("BitVec.sle"), bvar(0), bvar(0)),
                cst("true"),
            )
        }),
    );
    add(
        "BitVec.append_assoc",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Implicit,
                "m",
                nat_ty(),
                pi(
                    BinderInfo::Implicit,
                    "k",
                    nat_ty(),
                    pi(
                        BinderInfo::Default,
                        "a",
                        mk_bitvec(bvar(2)),
                        pi(
                            BinderInfo::Default,
                            "b",
                            mk_bitvec(bvar(2)),
                            pi(
                                BinderInfo::Default,
                                "c",
                                mk_bitvec(bvar(2)),
                                mk_bv_eq(
                                    app2(
                                        cst("Nat.add"),
                                        app2(cst("Nat.add"), bvar(5), bvar(4)),
                                        bvar(3),
                                    ),
                                    app2(
                                        cst("BitVec.append"),
                                        app2(cst("BitVec.append"), bvar(2), bvar(1)),
                                        bvar(0),
                                    ),
                                    app2(
                                        cst("BitVec.append"),
                                        bvar(2),
                                        app2(cst("BitVec.append"), bvar(1), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.extractLsb_full",
        bvx_ext_forall_nat_bv(|| {
            mk_bv_eq(
                bvar(1),
                app3(
                    cst("BitVec.extractLsb"),
                    app2(
                        cst("Nat.sub"),
                        bvar(1),
                        app(cst("Nat.succ"), cst("Nat.zero")),
                    ),
                    cst("Nat.zero"),
                    bvar(0),
                ),
                app2(cst("BitVec.setWidth"), bvar(1), bvar(0)),
            )
        }),
    );
    add("BitVec.zeroExtend", bvx_ext_zero_extend_ty());
    add(
        "BitVec.zeroExtend_ge_id",
        bvx_ext_forall_nat_bv(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.zeroExtend"), bvar(1), bvar(0)),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.signExtend_ge_id",
        bvx_ext_forall_nat_bv(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.signExtend"), bvar(1), bvar(0)),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.toFin",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            arrow(mk_bitvec(bvar(0)), bvx_ext_fin2n(bvar(0))),
        ),
    );
    add(
        "BitVec.ofFin",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            arrow(bvx_ext_fin2n(bvar(0)), mk_bitvec(bvar(0))),
        ),
    );
    add(
        "BitVec.toFin_ofFin",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "f",
                bvx_ext_fin2n(bvar(0)),
                mk_eq(
                    bvx_ext_fin2n(bvar(1)),
                    app(cst("BitVec.toFin"), app(cst("BitVec.ofFin"), bvar(0))),
                    bvar(0),
                ),
            ),
        ),
    );
    add(
        "BitVec.ofFin_toFin",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.ofFin"), app(cst("BitVec.toFin"), bvar(0))),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.toNat_lt",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "a",
                mk_bitvec(bvar(0)),
                mk_eq(
                    bool_ty(),
                    app2(
                        cst("Nat.lt"),
                        app(cst("BitVec.toNat"), bvar(0)),
                        app2(
                            cst("Nat.pow"),
                            app(cst("Nat.succ"), app(cst("Nat.succ"), cst("Nat.zero"))),
                            bvar(1),
                        ),
                    ),
                    cst("true"),
                ),
            ),
        ),
    );
    add(
        "BitVec.toInt_ofInt",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "i",
                int_ty(),
                mk_eq(
                    int_ty(),
                    app(
                        cst("BitVec.toInt"),
                        app2(cst("BitVec.ofInt"), bvar(1), bvar(0)),
                    ),
                    bvar(0),
                ),
            ),
        ),
    );
    add(
        "BitVec.rotateLeft_zero",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.rotateLeft"), bvar(0), cst("Nat.zero")),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.rotateRight_zero",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.rotateRight"), bvar(0), cst("Nat.zero")),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.rotateLeft_rotateRight_cancel",
        bvx_ext_forall_bv_nat(|| {
            mk_bv_eq(
                bvar(2),
                app2(
                    cst("BitVec.rotateLeft"),
                    app2(cst("BitVec.rotateRight"), bvar(1), bvar(0)),
                    bvar(0),
                ),
                bvar(1),
            )
        }),
    );
    add(
        "BitVec.rotateRight_rotateLeft_cancel",
        bvx_ext_forall_bv_nat(|| {
            mk_bv_eq(
                bvar(2),
                app2(
                    cst("BitVec.rotateRight"),
                    app2(cst("BitVec.rotateLeft"), bvar(1), bvar(0)),
                    bvar(0),
                ),
                bvar(1),
            )
        }),
    );
    add(
        "BitVec.setBit",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            arrow(
                mk_bitvec(bvar(0)),
                arrow(nat_ty(), arrow(bool_ty(), mk_bitvec(bvar(1)))),
            ),
        ),
    );
    add(
        "BitVec.getLsb_setBit_same",
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
                    "i",
                    nat_ty(),
                    pi(
                        BinderInfo::Default,
                        "v",
                        bool_ty(),
                        mk_eq(
                            bool_ty(),
                            app2(
                                cst("BitVec.getLsb"),
                                app3(cst("BitVec.setBit"), bvar(2), bvar(1), bvar(0)),
                                bvar(1),
                            ),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    );
    add("BitVec.popcount", bvx_ext_popcount_ty());
    add(
        "BitVec.popcount_zero",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(cst("BitVec.popcount"), app(cst("BitVec.zero"), bvar(0))),
                cst("Nat.zero"),
            ),
        ),
    );
    add(
        "BitVec.popcount_allOnes",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(cst("BitVec.popcount"), app(cst("BitVec.allOnes"), bvar(0))),
                bvar(0),
            ),
        ),
    );
    add(
        "BitVec.popcount_or_add",
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
                    mk_nat_eq(
                        app2(
                            cst("Nat.add"),
                            app(
                                cst("BitVec.popcount"),
                                app2(cst("BitVec.or"), bvar(1), bvar(0)),
                            ),
                            app(
                                cst("BitVec.popcount"),
                                app2(cst("BitVec.and"), bvar(1), bvar(0)),
                            ),
                        ),
                        app2(
                            cst("Nat.add"),
                            app(cst("BitVec.popcount"), bvar(1)),
                            app(cst("BitVec.popcount"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add("BitVec.countLeadingZeros", bvx_ext_count_zeros_ty());
    add("BitVec.countTrailingZeros", bvx_ext_count_zeros_ty());
    add(
        "BitVec.clz_zero",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(
                    cst("BitVec.countLeadingZeros"),
                    app(cst("BitVec.zero"), bvar(0)),
                ),
                bvar(0),
            ),
        ),
    );
    add(
        "BitVec.ctz_zero",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(
                    cst("BitVec.countTrailingZeros"),
                    app(cst("BitVec.zero"), bvar(0)),
                ),
                bvar(0),
            ),
        ),
    );
    add(
        "BitVec.clz_allOnes",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(
                    cst("BitVec.countLeadingZeros"),
                    app(cst("BitVec.allOnes"), bvar(0)),
                ),
                cst("Nat.zero"),
            ),
        ),
    );
    add("BitVec.reverse", bvx_ext_reverse_ty());
    add(
        "BitVec.reverse_reverse",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.reverse"), app(cst("BitVec.reverse"), bvar(0))),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.reverse_zero",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_bv_eq(
                bvar(0),
                app(cst("BitVec.reverse"), app(cst("BitVec.zero"), bvar(0))),
                app(cst("BitVec.zero"), bvar(0)),
            ),
        ),
    );
    add("BitVec.byteSwap", bitvec_unop_ty());
    add(
        "BitVec.byteSwap_byteSwap",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.byteSwap"), app(cst("BitVec.byteSwap"), bvar(0))),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.smt_bvadd_comm",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(cst("BitVec.add"), bvar(1), bvar(0)),
                        app2(cst("BitVec.add"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.smt_bvnot_bvadd_neg",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(
                    cst("BitVec.add"),
                    app(cst("BitVec.not"), bvar(0)),
                    app2(
                        cst("BitVec.ofNat"),
                        bvar(1),
                        app(cst("Nat.succ"), cst("Nat.zero")),
                    ),
                ),
                app(cst("BitVec.neg"), bvar(0)),
            )
        }),
    );
    add(
        "BitVec.smt_bvxor_comm",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(cst("BitVec.xor"), bvar(1), bvar(0)),
                        app2(cst("BitVec.xor"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.add_overflow_wrap",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_nat_eq(
                app(
                    cst("BitVec.toNat"),
                    app2(
                        cst("BitVec.add"),
                        app(cst("BitVec.allOnes"), bvar(0)),
                        app2(
                            cst("BitVec.ofNat"),
                            bvar(0),
                            app(cst("Nat.succ"), cst("Nat.zero")),
                        ),
                    ),
                ),
                cst("Nat.zero"),
            ),
        ),
    );
    add(
        "BitVec.sub_underflow_wrap",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_bv_eq(
                bvar(0),
                app2(
                    cst("BitVec.sub"),
                    app(cst("BitVec.zero"), bvar(0)),
                    app2(
                        cst("BitVec.ofNat"),
                        bvar(0),
                        app(cst("Nat.succ"), cst("Nat.zero")),
                    ),
                ),
                app(cst("BitVec.allOnes"), bvar(0)),
            ),
        ),
    );
    add("BitVec.isolateLowestBit", bitvec_unop_ty());
    add(
        "BitVec.isolateLowestBit_def",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.isolateLowestBit"), bvar(0)),
                app2(cst("BitVec.and"), bvar(0), app(cst("BitVec.neg"), bvar(0))),
            )
        }),
    );
    add("BitVec.clearLowestBit", bitvec_unop_ty());
    add(
        "BitVec.clearLowestBit_def",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app(cst("BitVec.clearLowestBit"), bvar(0)),
                app2(
                    cst("BitVec.and"),
                    bvar(0),
                    app2(
                        cst("BitVec.sub"),
                        bvar(0),
                        app2(
                            cst("BitVec.ofNat"),
                            bvar(1),
                            app(cst("Nat.succ"), cst("Nat.zero")),
                        ),
                    ),
                ),
            )
        }),
    );
    add(
        "BitVec.clearLowestBit_zero",
        bvx_ext_forall_one(|| {
            arrow(
                mk_bv_eq(bvar(1), bvar(0), app(cst("BitVec.zero"), bvar(1))),
                mk_bv_eq(
                    bvar(1),
                    app(cst("BitVec.clearLowestBit"), bvar(0)),
                    app(cst("BitVec.zero"), bvar(1)),
                ),
            )
        }),
    );
    add(
        "BitVec.simdAdd",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lanes",
                nat_ty(),
                arrow(
                    mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    arrow(
                        mk_bitvec(app2(cst("Nat.mul"), bvar(2), bvar(1))),
                        mk_bitvec(app2(cst("Nat.mul"), bvar(3), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.simdAnd",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lanes",
                nat_ty(),
                arrow(
                    mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    arrow(
                        mk_bitvec(app2(cst("Nat.mul"), bvar(2), bvar(1))),
                        mk_bitvec(app2(cst("Nat.mul"), bvar(3), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.simdOr",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lanes",
                nat_ty(),
                arrow(
                    mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    arrow(
                        mk_bitvec(app2(cst("Nat.mul"), bvar(2), bvar(1))),
                        mk_bitvec(app2(cst("Nat.mul"), bvar(3), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.simdXor",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lanes",
                nat_ty(),
                arrow(
                    mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    arrow(
                        mk_bitvec(app2(cst("Nat.mul"), bvar(2), bvar(1))),
                        mk_bitvec(app2(cst("Nat.mul"), bvar(3), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.simdAdd_comm",
        pi(
            BinderInfo::Implicit,
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "lanes",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "a",
                    mk_bitvec(app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    pi(
                        BinderInfo::Default,
                        "b",
                        mk_bitvec(app2(cst("Nat.mul"), bvar(2), bvar(1))),
                        mk_bv_eq(
                            app2(cst("Nat.mul"), bvar(3), bvar(2)),
                            app3(cst("BitVec.simdAdd"), bvar(2), bvar(1), bvar(0)),
                            app3(cst("BitVec.simdAdd"), bvar(2), bvar(0), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.xor_comm",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(cst("BitVec.xor"), bvar(1), bvar(0)),
                        app2(cst("BitVec.xor"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.xor_assoc",
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
                    pi(
                        BinderInfo::Default,
                        "c",
                        mk_bitvec(bvar(2)),
                        mk_bv_eq(
                            bvar(3),
                            app2(
                                cst("BitVec.xor"),
                                app2(cst("BitVec.xor"), bvar(2), bvar(1)),
                                bvar(0),
                            ),
                            app2(
                                cst("BitVec.xor"),
                                bvar(2),
                                app2(cst("BitVec.xor"), bvar(1), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.and_distrib_or",
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
                    pi(
                        BinderInfo::Default,
                        "c",
                        mk_bitvec(bvar(2)),
                        mk_bv_eq(
                            bvar(3),
                            app2(
                                cst("BitVec.and"),
                                bvar(2),
                                app2(cst("BitVec.or"), bvar(1), bvar(0)),
                            ),
                            app2(
                                cst("BitVec.or"),
                                app2(cst("BitVec.and"), bvar(2), bvar(1)),
                                app2(cst("BitVec.and"), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.or_distrib_and",
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
                    pi(
                        BinderInfo::Default,
                        "c",
                        mk_bitvec(bvar(2)),
                        mk_bv_eq(
                            bvar(3),
                            app2(
                                cst("BitVec.or"),
                                bvar(2),
                                app2(cst("BitVec.and"), bvar(1), bvar(0)),
                            ),
                            app2(
                                cst("BitVec.and"),
                                app2(cst("BitVec.or"), bvar(2), bvar(1)),
                                app2(cst("BitVec.or"), bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.and_or_absorb",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(
                            cst("BitVec.and"),
                            bvar(1),
                            app2(cst("BitVec.or"), bvar(1), bvar(0)),
                        ),
                        bvar(1),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.or_and_absorb",
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
                    mk_bv_eq(
                        bvar(2),
                        app2(
                            cst("BitVec.or"),
                            bvar(1),
                            app2(cst("BitVec.and"), bvar(1), bvar(0)),
                        ),
                        bvar(1),
                    ),
                ),
            ),
        ),
    );
    add(
        "BitVec.and_allOnes",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(
                    cst("BitVec.and"),
                    bvar(0),
                    app(cst("BitVec.allOnes"), bvar(1)),
                ),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.and_zero",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.and"), bvar(0), app(cst("BitVec.zero"), bvar(1))),
                app(cst("BitVec.zero"), bvar(1)),
            )
        }),
    );
    add(
        "BitVec.or_zero",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.or"), bvar(0), app(cst("BitVec.zero"), bvar(1))),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.or_allOnes",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(
                    cst("BitVec.or"),
                    bvar(0),
                    app(cst("BitVec.allOnes"), bvar(1)),
                ),
                app(cst("BitVec.allOnes"), bvar(1)),
            )
        }),
    );
    add(
        "BitVec.xor_zero",
        bvx_ext_forall_one(|| {
            mk_bv_eq(
                bvar(1),
                app2(cst("BitVec.xor"), bvar(0), app(cst("BitVec.zero"), bvar(1))),
                bvar(0),
            )
        }),
    );
    add(
        "BitVec.not_zero",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_bv_eq(
                bvar(0),
                app(cst("BitVec.not"), app(cst("BitVec.zero"), bvar(0))),
                app(cst("BitVec.allOnes"), bvar(0)),
            ),
        ),
    );
    add(
        "BitVec.not_allOnes",
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            mk_bv_eq(
                bvar(0),
                app(cst("BitVec.not"), app(cst("BitVec.allOnes"), bvar(0))),
                app(cst("BitVec.zero"), bvar(0)),
            ),
        ),
    );
    add(
        "BitVec.ext",
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
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "i",
                            nat_ty(),
                            mk_eq(
                                bool_ty(),
                                app2(cst("BitVec.getLsb"), bvar(2), bvar(0)),
                                app2(cst("BitVec.getLsb"), bvar(2), bvar(0)),
                            ),
                        ),
                        mk_bv_eq(bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    );
}
#[cfg(test)]
mod tests_ext {
    use super::*;
    fn setup_ext_env() -> Environment {
        let mut env = Environment::new();
        for (name, ty) in &[
            ("Nat", type1()),
            ("Int", type1()),
            ("Bool", type1()),
            ("Fin", type1()),
            ("Eq", prop()),
            ("Iff", prop()),
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .expect("operation should succeed");
        }
        for name in &[
            "Nat.zero", "Nat.succ", "Nat.add", "Nat.mul", "Nat.sub", "Nat.mod", "Nat.pow",
            "Nat.lt", "Nat.ble",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: nat_ty(),
            })
            .expect("operation should succeed");
        }
        for name in &["true", "false"] {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: bool_ty(),
            })
            .expect("operation should succeed");
        }
        build_bitvec_env(&mut env).expect("build_bitvec_env should succeed");
        register_bitvec_extended_axioms(&mut env);
        env
    }
    #[test]
    fn test_ext_arith_shift_right_present() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.arithShiftRight")));
    }
    #[test]
    fn test_ext_shift_zero_laws() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.shiftLeft_zero_amount")));
        assert!(env.contains(&Name::str("BitVec.shiftRight_zero_amount")));
        assert!(env.contains(&Name::str("BitVec.arithShiftRight_zero_amount")));
    }
    #[test]
    fn test_ext_add_mod_present() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.add_mod")));
    }
    #[test]
    fn test_ext_mul_laws() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.mul_comm")));
        assert!(env.contains(&Name::str("BitVec.mul_assoc")));
        assert!(env.contains(&Name::str("BitVec.mul_mod")));
    }
    #[test]
    fn test_ext_comparison_laws() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.ult_irrefl")));
        assert!(env.contains(&Name::str("BitVec.ule_refl")));
        assert!(env.contains(&Name::str("BitVec.slt_irrefl")));
        assert!(env.contains(&Name::str("BitVec.sle_refl")));
    }
    #[test]
    fn test_ext_concat_extract() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.append_assoc")));
        assert!(env.contains(&Name::str("BitVec.extractLsb_full")));
    }
    #[test]
    fn test_ext_zero_sign_extend() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.zeroExtend")));
        assert!(env.contains(&Name::str("BitVec.zeroExtend_ge_id")));
        assert!(env.contains(&Name::str("BitVec.signExtend_ge_id")));
    }
    #[test]
    fn test_ext_fin_iso() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.toFin")));
        assert!(env.contains(&Name::str("BitVec.ofFin")));
        assert!(env.contains(&Name::str("BitVec.toFin_ofFin")));
        assert!(env.contains(&Name::str("BitVec.ofFin_toFin")));
    }
    #[test]
    fn test_ext_nat_int_conv() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.toNat_lt")));
        assert!(env.contains(&Name::str("BitVec.toInt_ofInt")));
    }
    #[test]
    fn test_ext_rotation_laws() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.rotateLeft_zero")));
        assert!(env.contains(&Name::str("BitVec.rotateRight_zero")));
        assert!(env.contains(&Name::str("BitVec.rotateLeft_rotateRight_cancel")));
        assert!(env.contains(&Name::str("BitVec.rotateRight_rotateLeft_cancel")));
    }
    #[test]
    fn test_ext_bit_access() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.setBit")));
        assert!(env.contains(&Name::str("BitVec.getLsb_setBit_same")));
    }
    #[test]
    fn test_ext_popcount() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.popcount")));
        assert!(env.contains(&Name::str("BitVec.popcount_zero")));
        assert!(env.contains(&Name::str("BitVec.popcount_allOnes")));
        assert!(env.contains(&Name::str("BitVec.popcount_or_add")));
    }
    #[test]
    fn test_ext_count_zeros() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.countLeadingZeros")));
        assert!(env.contains(&Name::str("BitVec.countTrailingZeros")));
        assert!(env.contains(&Name::str("BitVec.clz_zero")));
        assert!(env.contains(&Name::str("BitVec.ctz_zero")));
        assert!(env.contains(&Name::str("BitVec.clz_allOnes")));
    }
    #[test]
    fn test_ext_reversal() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.reverse")));
        assert!(env.contains(&Name::str("BitVec.reverse_reverse")));
        assert!(env.contains(&Name::str("BitVec.reverse_zero")));
        assert!(env.contains(&Name::str("BitVec.byteSwap")));
        assert!(env.contains(&Name::str("BitVec.byteSwap_byteSwap")));
    }
    #[test]
    fn test_ext_smt_axioms() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.smt_bvadd_comm")));
        assert!(env.contains(&Name::str("BitVec.smt_bvnot_bvadd_neg")));
        assert!(env.contains(&Name::str("BitVec.smt_bvxor_comm")));
    }
    #[test]
    fn test_ext_overflow_behavior() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.add_overflow_wrap")));
        assert!(env.contains(&Name::str("BitVec.sub_underflow_wrap")));
    }
    #[test]
    fn test_ext_bit_tricks() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.isolateLowestBit")));
        assert!(env.contains(&Name::str("BitVec.isolateLowestBit_def")));
        assert!(env.contains(&Name::str("BitVec.clearLowestBit")));
        assert!(env.contains(&Name::str("BitVec.clearLowestBit_def")));
        assert!(env.contains(&Name::str("BitVec.clearLowestBit_zero")));
    }
    #[test]
    fn test_ext_simd() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.simdAdd")));
        assert!(env.contains(&Name::str("BitVec.simdAnd")));
        assert!(env.contains(&Name::str("BitVec.simdOr")));
        assert!(env.contains(&Name::str("BitVec.simdXor")));
        assert!(env.contains(&Name::str("BitVec.simdAdd_comm")));
    }
    #[test]
    fn test_ext_distributivity() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.and_distrib_or")));
        assert!(env.contains(&Name::str("BitVec.or_distrib_and")));
        assert!(env.contains(&Name::str("BitVec.xor_comm")));
        assert!(env.contains(&Name::str("BitVec.xor_assoc")));
    }
    #[test]
    fn test_ext_absorption() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.and_or_absorb")));
        assert!(env.contains(&Name::str("BitVec.or_and_absorb")));
    }
    #[test]
    fn test_ext_identity_annihilator() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.and_allOnes")));
        assert!(env.contains(&Name::str("BitVec.and_zero")));
        assert!(env.contains(&Name::str("BitVec.or_zero")));
        assert!(env.contains(&Name::str("BitVec.or_allOnes")));
        assert!(env.contains(&Name::str("BitVec.xor_zero")));
        assert!(env.contains(&Name::str("BitVec.not_zero")));
        assert!(env.contains(&Name::str("BitVec.not_allOnes")));
    }
    #[test]
    fn test_ext_extensionality() {
        let env = setup_ext_env();
        assert!(env.contains(&Name::str("BitVec.ext")));
    }
    #[test]
    fn test_bitvecfixed_struct_size() {
        let _bv: BitVecFixed<64> = BitVecFixed { data: 0xDEAD_BEEF };
        assert_eq!(std::mem::size_of::<BitVecFixed<64>>(), 8);
    }
}
