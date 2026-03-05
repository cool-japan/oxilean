//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name, ReducibilityHint};

use super::functions::*;
use super::types::EnvBuilder;

/// Add `RBMap` (ordered map).
#[allow(dead_code)]
pub fn add_rb_map(b: &mut EnvBuilder) {
    if !b.contains("Ord") {
        b.axiom("Ord", pi(type0(), prop()));
    }
    b.axiom("RBMap", pi(type0(), pi(type0(), type1())));
    b.axiom(
        "RBMap.empty",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi(
                    app(var("Ord"), bvar(1)),
                    app(app(var("RBMap"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "RBMap.insert",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi(
                    app(var("Ord"), bvar(1)),
                    pi(
                        bvar(2),
                        pi(
                            bvar(2),
                            pi(
                                app(app(var("RBMap"), bvar(4)), bvar(3)),
                                app(app(var("RBMap"), bvar(5)), bvar(4)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "RBMap.find",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi(
                    app(var("Ord"), bvar(1)),
                    pi(
                        bvar(2),
                        pi(
                            app(app(var("RBMap"), bvar(3)), bvar(2)),
                            app(var("Option"), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "RBMap.size",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("Ord"), bvar(1)),
                    pi(app(app(var("RBMap"), bvar(2)), bvar(1)), var("Nat")),
                ),
            ),
        ),
    );
    b.axiom(
        "RBMap.keys",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("Ord"), bvar(1)),
                    pi(
                        app(app(var("RBMap"), bvar(2)), bvar(1)),
                        app(var("List"), bvar(3)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "RBMap.values",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("Ord"), bvar(1)),
                    pi(
                        app(app(var("RBMap"), bvar(2)), bvar(1)),
                        app(var("List"), bvar(1)),
                    ),
                ),
            ),
        ),
    );
}
/// Add `Task` and concurrency primitives.
#[allow(dead_code)]
pub fn add_task(b: &mut EnvBuilder) {
    b.axiom("Task", pi(type0(), type0()));
    b.axiom(
        "Task.pure",
        pi_implicit("α", type0(), pi(bvar(0), app(var("Task"), bvar(1)))),
    );
    b.axiom(
        "Task.bind",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(var("Task"), bvar(1)),
                    pi(
                        pi(bvar(2), app(var("Task"), bvar(2))),
                        app(var("Task"), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Task.spawn",
        pi_implicit(
            "α",
            type0(),
            pi(pi(var("Unit"), bvar(1)), app(var("Task"), bvar(2))),
        ),
    );
    b.axiom(
        "Task.get",
        pi_implicit("α", type0(), pi(app(var("Task"), bvar(0)), bvar(1))),
    );
    b.axiom(
        "Task.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    pi(bvar(1), bvar(1)),
                    pi(app(var("Task"), bvar(2)), app(var("Task"), bvar(2))),
                ),
            ),
        ),
    );
    b.axiom(
        "Task.cancel",
        pi_implicit("α", type0(), pi(app(var("Task"), bvar(0)), var("Unit"))),
    );
}
/// Add `ST` (state thread) monad.
#[allow(dead_code)]
pub fn add_st_monad(b: &mut EnvBuilder) {
    b.axiom("ST", pi(type0(), pi(type0(), type0())));
    b.axiom("STRef", pi(type0(), pi(type0(), type0())));
    b.axiom(
        "ST.run",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi_implicit("s", type0(), app(app(var("ST"), bvar(1)), bvar(1))),
                bvar(1),
            ),
        ),
    );
    b.axiom(
        "STRef.new",
        pi_implicit(
            "s",
            type0(),
            pi_implicit(
                "α",
                type0(),
                pi(
                    bvar(0),
                    app(
                        app(var("ST"), bvar(2)),
                        app(app(var("STRef"), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "STRef.read",
        pi_implicit(
            "s",
            type0(),
            pi_implicit(
                "α",
                type0(),
                pi(
                    app(app(var("STRef"), bvar(1)), bvar(0)),
                    app(app(var("ST"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "STRef.write",
        pi_implicit(
            "s",
            type0(),
            pi_implicit(
                "α",
                type0(),
                pi(
                    app(app(var("STRef"), bvar(1)), bvar(0)),
                    pi(bvar(1), app(app(var("ST"), bvar(3)), var("Unit"))),
                ),
            ),
        ),
    );
    b.axiom(
        "STRef.modify",
        pi_implicit(
            "s",
            type0(),
            pi_implicit(
                "α",
                type0(),
                pi(
                    app(app(var("STRef"), bvar(1)), bvar(0)),
                    pi(
                        pi(bvar(1), bvar(2)),
                        app(app(var("ST"), bvar(3)), var("Unit")),
                    ),
                ),
            ),
        ),
    );
}
/// Register a simple unary type constructor: `Name : Type 0 → Type 0`.
#[allow(dead_code)]
pub fn add_unary_type_ctor(b: &mut EnvBuilder, name: &str) {
    b.axiom(name, pi(type0(), type0()));
}
/// Register a binary type constructor: `Name : Type 0 → Type 0 → Type 0`.
#[allow(dead_code)]
pub fn add_binary_type_ctor(b: &mut EnvBuilder, name: &str) {
    b.axiom(name, pi(type0(), pi(type0(), type0())));
}
/// Batch-register unary type constructors.
#[allow(dead_code)]
pub fn add_unary_type_ctors(b: &mut EnvBuilder, names: &[&str]) {
    for &n in names {
        add_unary_type_ctor(b, n);
    }
}
/// Batch-register binary type constructors.
#[allow(dead_code)]
pub fn add_binary_type_ctors(b: &mut EnvBuilder, names: &[&str]) {
    for &n in names {
        add_binary_type_ctor(b, n);
    }
}
/// Add `OfNat` / `OfScientific` numeric literal helpers.
#[allow(dead_code)]
pub fn add_numeric_literals(b: &mut EnvBuilder) {
    b.axiom("OfScientific", pi(type0(), prop()));
    b.axiom(
        "OfScientific.ofScientific",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("OfScientific"), bvar(0)),
                pi(var("Nat"), pi(var("Bool"), pi(var("Nat"), bvar(3)))),
            ),
        ),
    );
    b.axiom("OfNat", pi(type0(), pi(var("Nat"), prop())));
    b.axiom(
        "OfNat.ofNat",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(app(app(var("OfNat"), bvar(1)), bvar(0)), bvar(2)),
            ),
        ),
    );
}
/// Add `Decidable` type class.
#[allow(dead_code)]
pub fn add_decidable_ext(b: &mut EnvBuilder) {
    if !b.contains("Decidable") {
        b.axiom("Decidable", pi(prop(), type0()));
    }
    b.axiom(
        "Decidable.isTrue",
        pi_named("p", prop(), pi(bvar(0), app(var("Decidable"), bvar(1)))),
    );
    b.axiom(
        "Decidable.isFalse",
        pi_named(
            "p",
            prop(),
            pi(app(var("Not"), bvar(0)), app(var("Decidable"), bvar(1))),
        ),
    );
    b.axiom(
        "decide",
        pi_named("p", prop(), pi(app(var("Decidable"), bvar(0)), var("Bool"))),
    );
    b.axiom(
        "Decidable.decide",
        pi_named("p", prop(), pi(app(var("Decidable"), bvar(0)), var("Bool"))),
    );
    b.axiom(
        "instDecidableAnd",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(
                    app(var("Decidable"), bvar(1)),
                    pi(
                        app(var("Decidable"), bvar(1)),
                        app(var("Decidable"), app(app(var("And"), bvar(3)), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "instDecidableOr",
        pi_named(
            "p",
            prop(),
            pi_named(
                "q",
                prop(),
                pi(
                    app(var("Decidable"), bvar(1)),
                    pi(
                        app(var("Decidable"), bvar(1)),
                        app(var("Decidable"), app(app(var("Or"), bvar(3)), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "instDecidableNot",
        pi_named(
            "p",
            prop(),
            pi(
                app(var("Decidable"), bvar(0)),
                app(var("Decidable"), app(var("Not"), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "instDecidableEqNat",
        pi_named(
            "a",
            var("Nat"),
            pi_named(
                "b",
                var("Nat"),
                app(var("Decidable"), app(app(var("Eq"), bvar(1)), bvar(0))),
            ),
        ),
    );
    b.axiom(
        "instDecidableEqBool",
        pi_named(
            "a",
            var("Bool"),
            pi_named(
                "b",
                var("Bool"),
                app(var("Decidable"), app(app(var("Eq"), bvar(1)), bvar(0))),
            ),
        ),
    );
}
/// Add `Classical` logic axioms.
#[allow(dead_code)]
pub fn add_classical(b: &mut EnvBuilder) {
    b.axiom("Nonempty", pi(type0(), prop()));
    b.axiom(
        "Nonempty.intro",
        pi_implicit("α", type0(), pi(bvar(0), app(var("Nonempty"), bvar(1)))),
    );
    b.axiom(
        "Classical.em",
        pi_named(
            "p",
            prop(),
            app(app(var("Or"), bvar(0)), app(var("Not"), bvar(0))),
        ),
    );
    b.axiom(
        "Classical.choice",
        pi_implicit("α", type0(), pi(app(var("Nonempty"), bvar(0)), bvar(1))),
    );
    b.axiom(
        "Classical.propDecidable",
        pi_named("p", prop(), app(var("Decidable"), bvar(0))),
    );
    b.axiom(
        "Classical.byContradiction",
        pi_named(
            "p",
            prop(),
            pi(pi(app(var("Not"), bvar(0)), var("False")), bvar(1)),
        ),
    );
    b.axiom(
        "Classical.not_not",
        pi_named(
            "p",
            prop(),
            pi(app(var("Not"), app(var("Not"), bvar(0))), bvar(1)),
        ),
    );
    b.axiom(
        "Classical.axiomOfChoice",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                pi(bvar(0), type0()),
                pi(
                    pi_implicit("x", bvar(1), app(var("Nonempty"), app(bvar(1), bvar(0)))),
                    pi_named(
                        "f",
                        pi(bvar(2), app(bvar(2), bvar(1))),
                        pi_implicit(
                            "x",
                            bvar(3),
                            app(app(bvar(3), bvar(2)), app(bvar(1), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Build a complete standard environment with all core primitives.
#[allow(dead_code)]
pub fn build_full_std_env() -> EnvBuilder {
    let mut b = EnvBuilder::fresh();
    add_empty(&mut b);
    add_unit_full(&mut b);
    add_bool(&mut b);
    add_list(&mut b);
    add_prod(&mut b);
    add_array(&mut b);
    add_hashmap(&mut b);
    add_io(&mut b);
    add_eq(&mut b);
    add_prop_logic(&mut b);
    add_nat_arith(&mut b);
    add_int_arith(&mut b);
    add_float_ops(&mut b);
    add_string_ops(&mut b);
    add_char_ops(&mut b);
    add_option(&mut b);
    add_result(&mut b);
    add_ordering(&mut b);
    add_format(&mut b);
    add_fin(&mut b);
    add_sigma(&mut b);
    add_subtype(&mut b);
    add_well_founded(&mut b);
    add_type_classes(&mut b);
    add_decidable(&mut b);
    add_classical(&mut b);
    b
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_add_eq_extended() {
        let mut b = EnvBuilder::fresh();
        add_eq(&mut b);
        assert!(b.contains("Eq"));
        assert!(b.contains("Eq.refl"));
    }
    #[test]
    fn test_add_prop_logic() {
        let mut b = EnvBuilder::fresh();
        add_prop_logic(&mut b);
        assert!(b.contains("True"));
        assert!(b.contains("False"));
        assert!(b.contains("And"));
        assert!(b.contains("Or"));
        assert!(b.contains("Not"));
        assert!(b.contains("Iff"));
        assert!(b.contains("And.intro"));
        assert!(b.contains("And.left"));
        assert!(b.contains("And.right"));
    }
    #[test]
    fn test_add_nat_arith() {
        let mut b = EnvBuilder::fresh();
        add_nat_arith(&mut b);
        assert!(b.contains("Nat"));
        assert!(b.contains("Nat.zero"));
        assert!(b.contains("Nat.succ"));
        assert!(b.contains("Nat.add"));
        assert!(b.contains("Nat.mul"));
        assert!(b.contains("Nat.factorial"));
    }
    #[test]
    fn test_add_option() {
        let mut b = EnvBuilder::fresh();
        add_option(&mut b);
        assert!(b.contains("Option"));
        assert!(b.contains("Option.none"));
        assert!(b.contains("Option.some"));
        assert!(b.contains("Option.map"));
        assert!(b.contains("Option.bind"));
        assert!(b.contains("Option.isSome"));
    }
    #[test]
    fn test_add_result() {
        let mut b = EnvBuilder::fresh();
        add_result(&mut b);
        assert!(b.contains("Result"));
        assert!(b.contains("Result.ok"));
        assert!(b.contains("Result.err"));
        assert!(b.contains("Result.isOk"));
        assert!(b.contains("Result.isErr"));
    }
    #[test]
    fn test_add_vector() {
        let mut b = EnvBuilder::fresh();
        add_vector(&mut b);
        assert!(b.contains("Vector"));
        assert!(b.contains("Vector.nil"));
        assert!(b.contains("Vector.cons"));
        assert!(b.contains("Vector.head"));
        assert!(b.contains("Vector.tail"));
        assert!(b.contains("Vector.map"));
        assert!(b.contains("Vector.append"));
    }
    #[test]
    fn test_add_fin() {
        let mut b = EnvBuilder::fresh();
        add_nat_arith(&mut b);
        add_fin(&mut b);
        assert!(b.contains("Fin"));
        assert!(b.contains("Fin.mk"));
        assert!(b.contains("Fin.val"));
        assert!(b.contains("Fin.zero"));
    }
    #[test]
    fn test_add_sigma() {
        let mut b = EnvBuilder::fresh();
        add_sigma(&mut b);
        assert!(b.contains("Sigma"));
        assert!(b.contains("Sigma.mk"));
        assert!(b.contains("Sigma.fst"));
        assert!(b.contains("Sigma.snd"));
    }
    #[test]
    fn test_add_subtype() {
        let mut b = EnvBuilder::fresh();
        add_subtype(&mut b);
        assert!(b.contains("Subtype"));
        assert!(b.contains("Subtype.mk"));
        assert!(b.contains("Subtype.val"));
        assert!(b.contains("Subtype.property"));
    }
    #[test]
    fn test_add_ordering() {
        let mut b = EnvBuilder::fresh();
        add_ordering(&mut b);
        assert!(b.contains("Ordering"));
        assert!(b.contains("Ordering.lt"));
        assert!(b.contains("Ordering.eq"));
        assert!(b.contains("Ordering.gt"));
    }
    #[test]
    fn test_add_type_classes() {
        let mut b = EnvBuilder::fresh();
        add_type_classes(&mut b);
        assert!(b.contains("Functor"));
        assert!(b.contains("Applicative"));
        assert!(b.contains("Monad"));
        assert!(b.contains("Monad.pure"));
        assert!(b.contains("Monad.bind"));
        assert!(b.contains("Ord"));
        assert!(b.contains("BEq"));
        assert!(b.contains("Hashable"));
        assert!(b.contains("ToString"));
    }
    #[test]
    fn test_add_state_monad() {
        let mut b = EnvBuilder::fresh();
        add_state_monad(&mut b);
        assert!(b.contains("StateT"));
        assert!(b.contains("StateT.pure"));
        assert!(b.contains("StateT.bind"));
        assert!(b.contains("StateT.get"));
        assert!(b.contains("StateT.set"));
        assert!(b.contains("StateT.run"));
    }
    #[test]
    fn test_add_rb_tree() {
        let mut b = EnvBuilder::fresh();
        add_type_classes(&mut b);
        add_rb_tree(&mut b);
        assert!(b.contains("RBTree"));
        assert!(b.contains("RBTree.empty"));
        assert!(b.contains("RBTree.insert"));
        assert!(b.contains("RBTree.find"));
        assert!(b.contains("RBTree.contains"));
    }
    #[test]
    fn test_add_decidable() {
        let mut b = EnvBuilder::fresh();
        add_prop_logic(&mut b);
        add_decidable(&mut b);
        assert!(b.contains("Decidable"));
        assert!(b.contains("Decidable.isTrue"));
        assert!(b.contains("Decidable.isFalse"));
        assert!(b.contains("decide"));
        assert!(b.contains("instDecidableAnd"));
    }
    #[test]
    fn test_add_classical() {
        let mut b = EnvBuilder::fresh();
        add_prop_logic(&mut b);
        add_classical(&mut b);
        assert!(b.contains("Classical.em"));
        assert!(b.contains("Classical.choice"));
        assert!(b.contains("Classical.byContradiction"));
        assert!(b.contains("Nonempty"));
    }
    #[test]
    fn test_build_full_std_env() {
        let b = build_full_std_env();
        assert!(b.is_ok(), "full std env errors: {:?}", b.errors());
        assert!(b.contains("Nat"));
        assert!(b.contains("List"));
        assert!(b.contains("Option"));
        assert!(b.contains("And"));
        assert!(b.contains("Monad"));
    }
    #[test]
    fn test_pi_chain() {
        let ty = pi_chain(vec![var("Nat"), var("Nat")], var("Nat"));
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_app_n() {
        let f = var("f");
        let result = app_n(f, vec![var("a"), var("b"), var("c")]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_lam_ext() {
        let _l = lam_ext("x", var("Nat"), bvar(0));
    }
    #[test]
    fn test_sort_levels() {
        let _t0 = type0();
        let _t1 = type1();
        let _su = sort_u();
        let _sv = sort_v();
        let _s5 = sort(5);
    }
    #[test]
    fn test_add_float_ops() {
        let mut b = EnvBuilder::fresh();
        add_float_ops(&mut b);
        assert!(b.contains("Float"));
        assert!(b.contains("Float.sqrt"));
        assert!(b.contains("Float.exp"));
        assert!(b.contains("Float.sin"));
        assert!(b.contains("Float.cos"));
        assert!(b.contains("Float.isNaN"));
    }
    #[test]
    fn test_add_st_monad() {
        let mut b = EnvBuilder::fresh();
        add_st_monad(&mut b);
        assert!(b.contains("ST"));
        assert!(b.contains("STRef"));
        assert!(b.contains("ST.run"));
        assert!(b.contains("STRef.new"));
        assert!(b.contains("STRef.read"));
        assert!(b.contains("STRef.write"));
    }
    #[test]
    fn test_add_task() {
        let mut b = EnvBuilder::fresh();
        add_task(&mut b);
        assert!(b.contains("Task"));
        assert!(b.contains("Task.pure"));
        assert!(b.contains("Task.bind"));
        assert!(b.contains("Task.spawn"));
        assert!(b.contains("Task.get"));
    }
    #[test]
    fn test_add_format() {
        let mut b = EnvBuilder::fresh();
        add_format(&mut b);
        assert!(b.contains("Format"));
        assert!(b.contains("Format.nil"));
        assert!(b.contains("Format.text"));
        assert!(b.contains("Format.pretty"));
    }
    #[test]
    fn test_env_builder_errors() {
        let b = EnvBuilder::fresh();
        assert!(b.is_ok());
        assert!(b.errors().is_empty());
    }
    #[test]
    fn test_add_quotient() {
        let mut b = EnvBuilder::fresh();
        add_quotient(&mut b);
        assert!(b.contains("Setoid"));
        assert!(b.contains("Quotient"));
        assert!(b.contains("Quotient.mk"));
        assert!(b.contains("Quotient.lift"));
    }
    #[test]
    fn test_add_well_founded() {
        let mut b = EnvBuilder::fresh();
        add_well_founded(&mut b);
        assert!(b.contains("WellFounded"));
        assert!(b.contains("WellFounded.rec"));
        assert!(b.contains("Acc"));
        assert!(b.contains("Acc.intro"));
    }
}
/// Add `Sum` (coproduct / disjoint union) type with `.inl` and `.inr`.
#[allow(dead_code)]
pub fn add_sum(b: &mut EnvBuilder) {
    b.axiom("Sum", pi(type0(), pi(type0(), type0())));
    b.axiom(
        "Sum.inl",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(bvar(1), app(app(var("Sum"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Sum.inr",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(bvar(0), app(app(var("Sum"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Sum.elim",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi_implicit(
                    "γ",
                    type0(),
                    pi(
                        pi(bvar(2), bvar(2)),
                        pi(
                            pi(bvar(2), bvar(2)),
                            pi(app(app(var("Sum"), bvar(4)), bvar(3)), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Sum.isLeft",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(app(app(var("Sum"), bvar(1)), bvar(0)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "Sum.isRight",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(app(app(var("Sum"), bvar(1)), bvar(0)), var("Bool")),
            ),
        ),
    );
}
/// Add `Prod.fst` and `Prod.snd` accessors (extends `add_prod`).
#[allow(dead_code)]
pub fn add_prod_accessors(b: &mut EnvBuilder) {
    if !b.contains("Prod") {
        add_prod(b);
    }
    b.axiom(
        "Prod.fst",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(app(app(var("Prod"), bvar(1)), bvar(0)), bvar(2)),
            ),
        ),
    );
    b.axiom(
        "Prod.snd",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(app(app(var("Prod"), bvar(1)), bvar(0)), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "Prod.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi_implicit(
                    "γ",
                    type0(),
                    pi_implicit(
                        "δ",
                        type0(),
                        pi(
                            pi(bvar(3), bvar(3)),
                            pi(
                                pi(bvar(2), bvar(2)),
                                pi(
                                    app(app(var("Prod"), bvar(4)), bvar(3)),
                                    app(app(var("Prod"), bvar(4)), bvar(3)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Prod.swap",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(app(var("Prod"), bvar(1)), bvar(0)),
                    app(app(var("Prod"), bvar(1)), bvar(2)),
                ),
            ),
        ),
    );
}
/// Add extended list operations beyond `nil`/`cons`.
#[allow(dead_code)]
pub fn add_list_ext(b: &mut EnvBuilder) {
    if !b.contains("List") {
        add_list(b);
    }
    b.axiom(
        "List.length",
        pi_implicit("α", type0(), pi(app(var("List"), bvar(0)), var("Nat"))),
    );
    b.axiom(
        "List.append",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("List"), bvar(0)),
                pi(app(var("List"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.head",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("List"), bvar(0)), app(var("Option"), bvar(1))),
        ),
    );
    b.axiom(
        "List.tail",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("List"), bvar(0)), app(var("List"), bvar(1))),
        ),
    );
    b.axiom(
        "List.get",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("List"), bvar(0)),
                pi(var("Nat"), app(var("Option"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    pi(bvar(1), bvar(1)),
                    pi(app(var("List"), bvar(2)), app(var("List"), bvar(2))),
                ),
            ),
        ),
    );
    b.axiom(
        "List.filter",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("List"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.foldl",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    bvar(1),
                    pi(
                        pi(bvar(2), pi(bvar(3), bvar(3))),
                        pi(app(var("List"), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "List.foldr",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    bvar(1),
                    pi(
                        pi(bvar(2), pi(bvar(2), bvar(2))),
                        pi(app(var("List"), bvar(3)), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "List.reverse",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("List"), bvar(0)), app(var("List"), bvar(1))),
        ),
    );
    b.axiom(
        "List.zip",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(var("List"), bvar(1)),
                    pi(
                        app(var("List"), bvar(1)),
                        app(var("List"), app(app(var("Prod"), bvar(2)), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "List.join",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("List"), app(var("List"), bvar(0))),
                app(var("List"), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "List.any",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("List"), bvar(1)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "List.all",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("List"), bvar(1)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "List.find",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("List"), bvar(1)), app(var("Option"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.partition",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(
                    app(var("List"), bvar(1)),
                    app(
                        app(var("Prod"), app(var("List"), bvar(2))),
                        app(var("List"), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "List.take",
        pi_implicit(
            "α",
            type0(),
            pi(
                var("Nat"),
                pi(app(var("List"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.drop",
        pi_implicit(
            "α",
            type0(),
            pi(
                var("Nat"),
                pi(app(var("List"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "List.splitAt",
        pi_implicit(
            "α",
            type0(),
            pi(
                var("Nat"),
                pi(
                    app(var("List"), bvar(1)),
                    app(
                        app(var("Prod"), app(var("List"), bvar(2))),
                        app(var("List"), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "List.flatten",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("List"), app(var("List"), bvar(0))),
                app(var("List"), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "List.contains",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(bvar(1), pi(app(var("List"), bvar(2)), var("Bool"))),
            ),
        ),
    );
    b.axiom(
        "List.eraseFirst",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    bvar(1),
                    pi(app(var("List"), bvar(2)), app(var("List"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "List.enum",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("List"), bvar(0)),
                app(var("List"), app(app(var("Prod"), var("Nat")), bvar(1))),
            ),
        ),
    );
    b.axiom("List.range", pi(var("Nat"), app(var("List"), var("Nat"))));
    b.axiom("List.iota", pi(var("Nat"), app(var("List"), var("Nat"))));
    b.axiom(
        "List.replicate",
        pi_implicit(
            "α",
            type0(),
            pi(var("Nat"), pi(bvar(1), app(var("List"), bvar(2)))),
        ),
    );
}
/// Add `Array` extended operations.
#[allow(dead_code)]
pub fn add_array_ext(b: &mut EnvBuilder) {
    if !b.contains("Array") {
        add_array(b);
    }
    b.axiom(
        "Array.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    pi(bvar(1), bvar(1)),
                    pi(app(var("Array"), bvar(2)), app(var("Array"), bvar(2))),
                ),
            ),
        ),
    );
    b.axiom(
        "Array.filter",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("Array"), bvar(1)), app(var("Array"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Array.foldl",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    bvar(1),
                    pi(
                        pi(bvar(2), pi(bvar(3), bvar(3))),
                        pi(app(var("Array"), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Array.foldr",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    bvar(1),
                    pi(
                        pi(bvar(2), pi(bvar(2), bvar(2))),
                        pi(app(var("Array"), bvar(3)), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Array.set",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Array"), bvar(0)),
                pi(var("Nat"), pi(bvar(2), app(var("Array"), bvar(3)))),
            ),
        ),
    );
    b.axiom(
        "Array.modify",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Array"), bvar(0)),
                pi(
                    var("Nat"),
                    pi(pi(bvar(2), bvar(2)), app(var("Array"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Array.toList",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("Array"), bvar(0)), app(var("List"), bvar(1))),
        ),
    );
    b.axiom(
        "Array.ofList",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("List"), bvar(0)), app(var("Array"), bvar(1))),
        ),
    );
    b.axiom(
        "Array.append",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Array"), bvar(0)),
                pi(app(var("Array"), bvar(1)), app(var("Array"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Array.zip",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(var("Array"), bvar(1)),
                    pi(
                        app(var("Array"), bvar(1)),
                        app(var("Array"), app(app(var("Prod"), bvar(2)), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Array.any",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("Array"), bvar(1)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "Array.all",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("Array"), bvar(1)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "Array.find",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("Array"), bvar(1)), app(var("Option"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Array.reverse",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("Array"), bvar(0)), app(var("Array"), bvar(1))),
        ),
    );
    b.axiom(
        "Array.sort",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(app(var("Array"), bvar(1)), app(var("Array"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Array.deduplicate",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(app(var("Array"), bvar(1)), app(var("Array"), bvar(2))),
            ),
        ),
    );
}
/// Add `HashMap` extended operations.
#[allow(dead_code)]
pub fn add_hashmap_ext(b: &mut EnvBuilder) {
    if !b.contains("HashMap") {
        add_hashmap(b);
    }
    b.axiom(
        "HashMap.size",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(app(app(var("HashMap"), bvar(3)), bvar(2)), var("Nat")),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.toList",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            app(var("List"), app(app(var("Prod"), bvar(4)), bvar(3))),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.keys",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            app(var("List"), bvar(4)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.values",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            app(var("List"), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.contains",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            pi(bvar(4), var("Bool")),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.map",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "w",
                    type0(),
                    pi_implicit(
                        "_",
                        app(var("BEq"), bvar(2)),
                        pi_implicit(
                            "_",
                            app(var("Hashable"), bvar(3)),
                            pi(
                                pi(bvar(3), bvar(3)),
                                pi(
                                    app(app(var("HashMap"), bvar(5)), bvar(4)),
                                    app(app(var("HashMap"), bvar(6)), bvar(3)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.filter",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            pi(bvar(3), pi(bvar(3), var("Bool"))),
                            pi(
                                app(app(var("HashMap"), bvar(4)), bvar(3)),
                                app(app(var("HashMap"), bvar(5)), bvar(4)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.erase",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            pi(bvar(4), app(app(var("HashMap"), bvar(5)), bvar(4))),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "HashMap.merge",
        pi_implicit(
            "k",
            type0(),
            pi_implicit(
                "v",
                type0(),
                pi_implicit(
                    "_",
                    app(var("BEq"), bvar(1)),
                    pi_implicit(
                        "_",
                        app(var("Hashable"), bvar(2)),
                        pi(
                            app(app(var("HashMap"), bvar(3)), bvar(2)),
                            pi(
                                app(app(var("HashMap"), bvar(4)), bvar(3)),
                                app(app(var("HashMap"), bvar(5)), bvar(4)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add basic numeric casts and conversions.
#[allow(dead_code)]
pub fn add_numeric_casts(b: &mut EnvBuilder) {
    b.axiom(
        "Nat.cast",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("OfNat"), bvar(0)), pi(var("Nat"), bvar(2))),
        ),
    );
    b.axiom(
        "Int.cast",
        pi_implicit("α", type0(), pi(prop(), pi(var("Int"), bvar(2)))),
    );
    b.axiom(
        "Float.cast",
        pi_implicit("α", type0(), pi(prop(), pi(var("Float"), bvar(2)))),
    );
    b.axiom("Rat", type0());
    b.axiom("Rat.mk", pi(var("Int"), pi(var("Nat"), var("Rat"))));
    b.axiom("Rat.num", pi(var("Rat"), var("Int")));
    b.axiom("Rat.den", pi(var("Rat"), var("Nat")));
    b.axiom("Rat.add", pi(var("Rat"), pi(var("Rat"), var("Rat"))));
    b.axiom("Rat.sub", pi(var("Rat"), pi(var("Rat"), var("Rat"))));
    b.axiom("Rat.mul", pi(var("Rat"), pi(var("Rat"), var("Rat"))));
    b.axiom("Rat.div", pi(var("Rat"), pi(var("Rat"), var("Rat"))));
    b.axiom("Rat.neg", pi(var("Rat"), var("Rat")));
    b.axiom("Rat.inv", pi(var("Rat"), var("Rat")));
    b.axiom("Rat.lt", pi(var("Rat"), pi(var("Rat"), prop())));
    b.axiom("Rat.le", pi(var("Rat"), pi(var("Rat"), prop())));
    b.axiom("Rat.ofInt", pi(var("Int"), var("Rat")));
    b.axiom("Rat.ofNat", pi(var("Nat"), var("Rat")));
    b.axiom("Rat.toFloat", pi(var("Rat"), var("Float")));
}
/// Add `IO` extended operations.
#[allow(dead_code)]
pub fn add_io_ext(b: &mut EnvBuilder) {
    if !b.contains("IO") {
        add_io(b);
    }
    b.axiom(
        "IO.println",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("ToString"), bvar(0)),
                pi(bvar(1), app(var("IO"), var("Unit"))),
            ),
        ),
    );
    b.axiom("IO.readLine", app(var("IO"), var("String")));
    b.axiom(
        "IO.getArgs",
        app(var("IO"), app(var("List"), var("String"))),
    );
    b.axiom("IO.exitSuccess", app(var("IO"), var("Empty")));
    b.axiom(
        "IO.exitFailure",
        pi(var("Nat"), app(var("IO"), var("Empty"))),
    );
    b.axiom(
        "IO.throwError",
        pi_implicit("α", type0(), pi(var("String"), app(var("IO"), bvar(1)))),
    );
    b.axiom(
        "IO.catchError",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("IO"), bvar(0)),
                pi(
                    pi(var("String"), app(var("IO"), bvar(1))),
                    app(var("IO"), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom("IO.sleep", pi(var("Nat"), app(var("IO"), var("Unit"))));
    b.axiom(
        "IO.fileRead",
        pi(
            var("String"),
            app(
                var("IO"),
                app(app(var("Result"), var("String")), var("String")),
            ),
        ),
    );
    b.axiom(
        "IO.fileWrite",
        pi(
            var("String"),
            pi(
                var("String"),
                app(
                    var("IO"),
                    app(app(var("Result"), var("Unit")), var("String")),
                ),
            ),
        ),
    );
    b.axiom(
        "IO.fileExists",
        pi(var("String"), app(var("IO"), var("Bool"))),
    );
    b.axiom(
        "IO.envVar",
        pi(
            var("String"),
            app(var("IO"), app(var("Option"), var("String"))),
        ),
    );
}
/// Add `Nat` induction and recursion principles.
#[allow(dead_code)]
pub fn add_nat_induction(b: &mut EnvBuilder) {
    b.axiom(
        "Nat.rec",
        pi_implicit(
            "motive",
            pi(var("Nat"), type1()),
            pi(
                app(bvar(0), var("Nat.zero")),
                pi(
                    pi_named(
                        "n",
                        var("Nat"),
                        pi(
                            app(bvar(2), bvar(0)),
                            app(bvar(3), app(var("Nat.succ"), bvar(1))),
                        ),
                    ),
                    pi_named("n", var("Nat"), app(bvar(3), bvar(0))),
                ),
            ),
        ),
    );
    b.axiom(
        "Nat.casesOn",
        pi_implicit(
            "motive",
            pi(var("Nat"), type1()),
            pi_named(
                "n",
                var("Nat"),
                pi(
                    app(bvar(1), var("Nat.zero")),
                    pi(
                        pi_named("k", var("Nat"), app(bvar(3), app(var("Nat.succ"), bvar(0)))),
                        app(bvar(3), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Nat.strongInduction",
        pi_implicit(
            "motive",
            pi(var("Nat"), prop()),
            pi(
                pi_named(
                    "n",
                    var("Nat"),
                    pi(
                        pi_named(
                            "m",
                            var("Nat"),
                            pi(
                                app(app(var("Nat.lt"), bvar(0)), bvar(1)),
                                app(bvar(3), bvar(1)),
                            ),
                        ),
                        app(bvar(2), bvar(1)),
                    ),
                ),
                pi_named("n", var("Nat"), app(bvar(2), bvar(0))),
            ),
        ),
    );
    b.axiom(
        "Nat.div2Ind",
        pi_implicit(
            "motive",
            pi(var("Nat"), prop()),
            pi(
                app(bvar(0), var("Nat.zero")),
                pi(
                    pi_named(
                        "n",
                        var("Nat"),
                        pi(
                            app(
                                bvar(2),
                                app(
                                    app(var("Nat.div"), app(var("Nat.succ"), bvar(0))),
                                    var("Nat.zero"),
                                ),
                            ),
                            app(bvar(3), app(var("Nat.succ"), bvar(1))),
                        ),
                    ),
                    pi_named("n", var("Nat"), app(bvar(3), bvar(0))),
                ),
            ),
        ),
    );
}
