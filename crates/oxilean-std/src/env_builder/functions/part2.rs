//! Environment builder functions

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name, ReducibilityHint};

use super::super::types::EnvBuilder;

use super::part1::*;

/// Add extended `Nat` arithmetic: add, mul, sub, div, mod, pow, gcd, lcm, factorial.
#[allow(dead_code)]
pub fn add_nat_arith(b: &mut EnvBuilder) {
    if !b.contains("Nat") {
        b.axiom("Nat", type0());
        b.axiom("Nat.zero", var("Nat"));
        b.axiom("Nat.succ", pi(var("Nat"), var("Nat")));
    }
    b.axiom("Nat.add", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.mul", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.sub", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.div", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.mod", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.pow", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.gcd", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.lcm", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.factorial", pi(var("Nat"), var("Nat")));
    b.axiom("Nat.lt", pi(var("Nat"), pi(var("Nat"), prop())));
    b.axiom("Nat.le", pi(var("Nat"), pi(var("Nat"), prop())));
    b.axiom("Nat.min", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.max", pi(var("Nat"), pi(var("Nat"), var("Nat"))));
    b.axiom("Nat.toInt", pi(var("Nat"), var("Int")));
    b.axiom("Nat.repr", pi(var("Nat"), var("String")));
}
/// Add `Int` arithmetic operations.
#[allow(dead_code)]
pub fn add_int_arith(b: &mut EnvBuilder) {
    if !b.contains("Int") {
        b.axiom("Int", type0());
    }
    b.axiom("Int.add", pi(var("Int"), pi(var("Int"), var("Int"))));
    b.axiom("Int.sub", pi(var("Int"), pi(var("Int"), var("Int"))));
    b.axiom("Int.mul", pi(var("Int"), pi(var("Int"), var("Int"))));
    b.axiom("Int.div", pi(var("Int"), pi(var("Int"), var("Int"))));
    b.axiom("Int.mod", pi(var("Int"), pi(var("Int"), var("Int"))));
    b.axiom("Int.neg", pi(var("Int"), var("Int")));
    b.axiom("Int.abs", pi(var("Int"), var("Nat")));
    b.axiom("Int.lt", pi(var("Int"), pi(var("Int"), prop())));
    b.axiom("Int.le", pi(var("Int"), pi(var("Int"), prop())));
    b.axiom("Int.gcd", pi(var("Int"), pi(var("Int"), var("Nat"))));
}
/// Add `Float` operations.
#[allow(dead_code)]
pub fn add_float_ops(b: &mut EnvBuilder) {
    b.axiom("Float", type0());
    b.axiom(
        "Float.add",
        pi(var("Float"), pi(var("Float"), var("Float"))),
    );
    b.axiom(
        "Float.sub",
        pi(var("Float"), pi(var("Float"), var("Float"))),
    );
    b.axiom(
        "Float.mul",
        pi(var("Float"), pi(var("Float"), var("Float"))),
    );
    b.axiom(
        "Float.div",
        pi(var("Float"), pi(var("Float"), var("Float"))),
    );
    b.axiom("Float.neg", pi(var("Float"), var("Float")));
    b.axiom("Float.abs", pi(var("Float"), var("Float")));
    b.axiom("Float.sqrt", pi(var("Float"), var("Float")));
    b.axiom("Float.exp", pi(var("Float"), var("Float")));
    b.axiom("Float.log", pi(var("Float"), var("Float")));
    b.axiom("Float.sin", pi(var("Float"), var("Float")));
    b.axiom("Float.cos", pi(var("Float"), var("Float")));
    b.axiom("Float.tan", pi(var("Float"), var("Float")));
    b.axiom(
        "Float.pow",
        pi(var("Float"), pi(var("Float"), var("Float"))),
    );
    b.axiom("Float.floor", pi(var("Float"), var("Float")));
    b.axiom("Float.ceil", pi(var("Float"), var("Float")));
    b.axiom("Float.round", pi(var("Float"), var("Float")));
    b.axiom("Float.lt", pi(var("Float"), pi(var("Float"), prop())));
    b.axiom("Float.le", pi(var("Float"), pi(var("Float"), prop())));
    b.axiom("Float.isNaN", pi(var("Float"), var("Bool")));
    b.axiom("Float.isInf", pi(var("Float"), var("Bool")));
    b.axiom("Float.ofNat", pi(var("Nat"), var("Float")));
}
/// Add `String` operations.
#[allow(dead_code)]
pub fn add_string_ops(b: &mut EnvBuilder) {
    if !b.contains("String") {
        b.axiom("String", type0());
    }
    b.axiom("String.empty", var("String"));
    b.axiom(
        "String.append",
        pi(var("String"), pi(var("String"), var("String"))),
    );
    b.axiom("String.length", pi(var("String"), var("Nat")));
    b.axiom("String.get", pi(var("String"), pi(var("Nat"), var("Char"))));
    b.axiom("String.toLower", pi(var("String"), var("String")));
    b.axiom("String.toUpper", pi(var("String"), var("String")));
    b.axiom("String.trim", pi(var("String"), var("String")));
    b.axiom(
        "String.beq",
        pi(var("String"), pi(var("String"), var("Bool"))),
    );
    b.axiom("String.lt", pi(var("String"), pi(var("String"), prop())));
    b.axiom("String.repr", pi(var("String"), var("String")));
}
/// Add `Char` operations.
#[allow(dead_code)]
pub fn add_char_ops(b: &mut EnvBuilder) {
    if !b.contains("Char") {
        b.axiom("Char", type0());
    }
    b.axiom("Char.val", pi(var("Char"), var("Nat")));
    b.axiom("Char.ofNat", pi(var("Nat"), var("Char")));
    b.axiom("Char.isAlpha", pi(var("Char"), var("Bool")));
    b.axiom("Char.isDigit", pi(var("Char"), var("Bool")));
    b.axiom("Char.isSpace", pi(var("Char"), var("Bool")));
    b.axiom("Char.isUpper", pi(var("Char"), var("Bool")));
    b.axiom("Char.isLower", pi(var("Char"), var("Bool")));
    b.axiom("Char.toUpper", pi(var("Char"), var("Char")));
    b.axiom("Char.toLower", pi(var("Char"), var("Char")));
    b.axiom("Char.beq", pi(var("Char"), pi(var("Char"), var("Bool"))));
}
/// Add `Result` type with `.ok`, `.err`, `.isOk`, `.isErr`.
#[allow(dead_code)]
pub fn add_result(b: &mut EnvBuilder) {
    b.axiom("Result", pi(type0(), pi(type0(), type0())));
    b.axiom(
        "Result.ok",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "ε",
                type0(),
                pi(bvar(1), app(app(var("Result"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Result.err",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "ε",
                type0(),
                pi(bvar(0), app(app(var("Result"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Result.isOk",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "ε",
                type0(),
                pi(app(app(var("Result"), bvar(1)), bvar(0)), var("Bool")),
            ),
        ),
    );
    b.axiom(
        "Result.isErr",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "ε",
                type0(),
                pi(app(app(var("Result"), bvar(1)), bvar(0)), var("Bool")),
            ),
        ),
    );
}
/// Add `StateT` monad transformer.
#[allow(dead_code)]
pub fn add_state_monad(b: &mut EnvBuilder) {
    b.axiom("StateT", pi(type0(), pi(type0(), pi(type0(), type0()))));
    b.axiom(
        "StateT.pure",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(0),
                        app(app(app(var("StateT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "StateT.bind",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi_implicit(
                        "β",
                        type0(),
                        pi(
                            app(app(app(var("StateT"), bvar(3)), bvar(2)), bvar(1)),
                            pi(
                                pi(
                                    bvar(2),
                                    app(app(app(var("StateT"), bvar(4)), bvar(3)), bvar(1)),
                                ),
                                app(app(app(var("StateT"), bvar(4)), bvar(3)), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "StateT.get",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                app(app(app(var("StateT"), bvar(1)), bvar(0)), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "StateT.set",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi(
                    bvar(1),
                    app(app(app(var("StateT"), bvar(2)), bvar(1)), var("Unit")),
                ),
            ),
        ),
    );
    b.axiom(
        "StateT.modify",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi(
                    pi(bvar(1), bvar(2)),
                    app(app(app(var("StateT"), bvar(2)), bvar(1)), var("Unit")),
                ),
            ),
        ),
    );
    b.axiom(
        "StateT.run",
        pi_implicit(
            "σ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        app(app(app(var("StateT"), bvar(2)), bvar(1)), bvar(0)),
                        pi(bvar(3), bvar(3)),
                    ),
                ),
            ),
        ),
    );
}
/// Add `ReaderT` monad transformer.
#[allow(dead_code)]
pub fn add_reader_monad(b: &mut EnvBuilder) {
    b.axiom("ReaderT", pi(type0(), pi(type0(), pi(type0(), type0()))));
    b.axiom(
        "ReaderT.pure",
        pi_implicit(
            "ρ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(0),
                        app(app(app(var("ReaderT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ReaderT.ask",
        pi_implicit(
            "ρ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                app(app(app(var("ReaderT"), bvar(1)), bvar(0)), bvar(1)),
            ),
        ),
    );
    b.axiom(
        "ReaderT.run",
        pi_implicit(
            "ρ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        app(app(app(var("ReaderT"), bvar(2)), bvar(1)), bvar(0)),
                        pi(bvar(3), bvar(3)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ReaderT.bind",
        pi_implicit(
            "ρ",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi_implicit(
                        "β",
                        type0(),
                        pi(
                            app(app(app(var("ReaderT"), bvar(3)), bvar(2)), bvar(1)),
                            pi(
                                pi(
                                    bvar(2),
                                    app(app(app(var("ReaderT"), bvar(4)), bvar(3)), bvar(1)),
                                ),
                                app(app(app(var("ReaderT"), bvar(4)), bvar(3)), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `WriterT` monad transformer.
#[allow(dead_code)]
pub fn add_writer_monad(b: &mut EnvBuilder) {
    b.axiom("WriterT", pi(type0(), pi(type0(), pi(type0(), type0()))));
    b.axiom(
        "WriterT.pure",
        pi_implicit(
            "ω",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(0),
                        app(app(app(var("WriterT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "WriterT.tell",
        pi_implicit(
            "ω",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi(
                    bvar(1),
                    app(app(app(var("WriterT"), bvar(2)), bvar(1)), var("Unit")),
                ),
            ),
        ),
    );
    b.axiom(
        "WriterT.listen",
        pi_implicit(
            "ω",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        app(app(app(var("WriterT"), bvar(2)), bvar(1)), bvar(0)),
                        app(
                            app(app(var("WriterT"), bvar(3)), bvar(2)),
                            app(app(var("Prod"), bvar(1)), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `ContT` continuation monad transformer.
#[allow(dead_code)]
pub fn add_cont_monad(b: &mut EnvBuilder) {
    b.axiom("ContT", pi(type0(), pi(type0(), pi(type0(), type0()))));
    b.axiom(
        "ContT.pure",
        pi_implicit(
            "r",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(0),
                        app(app(app(var("ContT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ContT.callCC",
        pi_implicit(
            "r",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        pi(
                            pi(
                                bvar(0),
                                app(app(app(var("ContT"), bvar(3)), bvar(2)), bvar(1)),
                            ),
                            app(app(app(var("ContT"), bvar(3)), bvar(2)), bvar(1)),
                        ),
                        app(app(app(var("ContT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
}
/// Add `ExceptT` monad transformer.
#[allow(dead_code)]
pub fn add_except_monad(b: &mut EnvBuilder) {
    b.axiom("ExceptT", pi(type0(), pi(type0(), pi(type0(), type0()))));
    b.axiom(
        "ExceptT.pure",
        pi_implicit(
            "ε",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(0),
                        app(app(app(var("ExceptT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ExceptT.throw",
        pi_implicit(
            "ε",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        bvar(2),
                        app(app(app(var("ExceptT"), bvar(3)), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ExceptT.catch",
        pi_implicit(
            "ε",
            type0(),
            pi_implicit(
                "m",
                type0(),
                pi_implicit(
                    "α",
                    type0(),
                    pi(
                        app(app(app(var("ExceptT"), bvar(2)), bvar(1)), bvar(0)),
                        pi(
                            pi(
                                bvar(3),
                                app(app(app(var("ExceptT"), bvar(4)), bvar(3)), bvar(2)),
                            ),
                            app(app(app(var("ExceptT"), bvar(4)), bvar(3)), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `Vector` (length-indexed array).
#[allow(dead_code)]
pub fn add_vector(b: &mut EnvBuilder) {
    b.axiom("Vector", pi(type0(), pi(var("Nat"), type0())));
    b.axiom(
        "Vector.nil",
        pi_implicit(
            "α",
            type0(),
            app(app(var("Vector"), bvar(0)), var("Nat.zero")),
        ),
    );
    b.axiom(
        "Vector.cons",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(
                    bvar(1),
                    pi(
                        app(app(var("Vector"), bvar(2)), bvar(1)),
                        app(app(var("Vector"), bvar(3)), app(var("Nat.succ"), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.head",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(
                    app(app(var("Vector"), bvar(1)), app(var("Nat.succ"), bvar(0))),
                    bvar(2),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.tail",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(
                    app(app(var("Vector"), bvar(1)), app(var("Nat.succ"), bvar(0))),
                    app(app(var("Vector"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.get",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(
                    app(app(var("Vector"), bvar(1)), bvar(0)),
                    pi(var("Nat"), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi_implicit(
                    "n",
                    var("Nat"),
                    pi(
                        pi(bvar(2), bvar(2)),
                        pi(
                            app(app(var("Vector"), bvar(3)), bvar(1)),
                            app(app(var("Vector"), bvar(3)), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.append",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "m",
                var("Nat"),
                pi_implicit(
                    "n",
                    var("Nat"),
                    pi(
                        app(app(var("Vector"), bvar(2)), bvar(1)),
                        pi(
                            app(app(var("Vector"), bvar(3)), bvar(1)),
                            app(
                                app(var("Vector"), bvar(4)),
                                app(app(var("Nat.add"), bvar(3)), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.reverse",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "n",
                var("Nat"),
                pi(
                    app(app(var("Vector"), bvar(1)), bvar(0)),
                    app(app(var("Vector"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "Vector.foldl",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi_implicit(
                    "n",
                    var("Nat"),
                    pi(
                        bvar(1),
                        pi(
                            pi(bvar(2), pi(bvar(3), bvar(3))),
                            pi(app(app(var("Vector"), bvar(3)), bvar(1)), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `Quotient` type.
#[allow(dead_code)]
pub fn add_quotient(b: &mut EnvBuilder) {
    b.axiom("Setoid", pi(type0(), type1()));
    b.axiom(
        "Quotient",
        pi_implicit("α", type0(), pi(app(var("Setoid"), bvar(0)), type0())),
    );
    b.axiom(
        "Quotient.mk",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "s",
                app(var("Setoid"), bvar(0)),
                pi(bvar(1), app(app(var("Quotient"), bvar(2)), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Quotient.lift",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "s",
                app(var("Setoid"), bvar(0)),
                pi_implicit(
                    "β",
                    type0(),
                    pi(
                        pi(bvar(2), bvar(1)),
                        pi(
                            pi_implicit(
                                "a",
                                bvar(3),
                                pi_implicit(
                                    "b",
                                    bvar(4),
                                    pi(
                                        prop(),
                                        app(
                                            app(var("Eq"), app(bvar(5), bvar(2))),
                                            app(bvar(5), bvar(1)),
                                        ),
                                    ),
                                ),
                            ),
                            pi(app(app(var("Quotient"), bvar(4)), bvar(3)), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Quotient.sound",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "s",
                app(var("Setoid"), bvar(0)),
                pi_implicit(
                    "a",
                    bvar(1),
                    pi_implicit(
                        "b",
                        bvar(2),
                        pi(
                            prop(),
                            app(
                                app(var("Eq"), app(app(var("Quotient.mk"), bvar(3)), bvar(2))),
                                app(app(var("Quotient.mk"), bvar(3)), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `Functor`, `Applicative`, `Monad`, `Ord`, `BEq`, `Hashable`, `ToString` type classes.
#[allow(dead_code)]
pub fn add_type_classes(b: &mut EnvBuilder) {
    if !b.contains("Functor") {
        add_functor(b);
    }
    if !b.contains("Monad") {
        add_monad(b);
    }
    // Monad.bind : {M : Type → Type} → [Monad M] → {α β : Type} → M α → (α → M β) → M β
    if !b.contains("Monad.bind") {
        let bind_ty = pi_implicit(
            "M",
            pi(type0(), type0()),
            pi_inst(
                "_",
                app(var("Monad"), bvar(0)),
                pi_implicit(
                    "α",
                    type0(),
                    pi_implicit(
                        "β",
                        type0(),
                        pi(
                            app(bvar(4), bvar(1)),
                            pi(pi(bvar(2), app(bvar(5), bvar(2))), app(bvar(5), bvar(2))),
                        ),
                    ),
                ),
            ),
        );
        b.axiom("Monad.bind", bind_ty);
    }
    b.axiom("Applicative", pi(pi(type0(), type0()), prop()));
    b.axiom("Foldable", pi(pi(type0(), type0()), prop()));
    b.axiom("Traversable", pi(pi(type0(), type0()), prop()));
    b.axiom("Ord", pi(type0(), prop()));
    b.axiom(
        "Ord.compare",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(bvar(1), pi(bvar(2), var("Ordering"))),
            ),
        ),
    );
    b.axiom("Hashable", pi(type0(), prop()));
    b.axiom(
        "Hashable.hash",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("Hashable"), bvar(0)), pi(bvar(1), var("Nat"))),
        ),
    );
    b.axiom("BEq", pi(type0(), prop()));
    b.axiom(
        "BEq.beq",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(bvar(1), pi(bvar(2), var("Bool"))),
            ),
        ),
    );
    b.axiom("ToString", pi(type0(), prop()));
    b.axiom(
        "ToString.toString",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("ToString"), bvar(0)), pi(bvar(1), var("String"))),
        ),
    );
    b.axiom("Repr", pi(type0(), prop()));
    if !b.contains("Format") {
        b.axiom("Format", type0());
    }
    b.axiom(
        "Repr.reprPrec",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Repr"), bvar(0)),
                pi(bvar(1), pi(var("Nat"), var("Format"))),
            ),
        ),
    );
}
/// Add `Ordering` type and helpers.
#[allow(dead_code)]
pub fn add_ordering(b: &mut EnvBuilder) {
    b.axiom("Ordering", type0());
    b.axiom("Ordering.lt", var("Ordering"));
    b.axiom("Ordering.eq", var("Ordering"));
    b.axiom("Ordering.gt", var("Ordering"));
    b.axiom("Ordering.swap", pi(var("Ordering"), var("Ordering")));
    b.axiom("Ordering.isLT", pi(var("Ordering"), var("Bool")));
    b.axiom("Ordering.isEQ", pi(var("Ordering"), var("Bool")));
    b.axiom("Ordering.isGT", pi(var("Ordering"), var("Bool")));
    b.axiom(
        "compareOfLessAndEq",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(bvar(1), pi(bvar(2), var("Ordering"))),
            ),
        ),
    );
}
/// Add `Format` pretty-printing combinators.
#[allow(dead_code)]
pub fn add_format(b: &mut EnvBuilder) {
    if !b.contains("Format") {
        b.axiom("Format", type0());
    }
    b.axiom("Format.nil", var("Format"));
    b.axiom("Format.text", pi(var("String"), var("Format")));
    b.axiom("Format.line", var("Format"));
    b.axiom(
        "Format.nest",
        pi(var("Nat"), pi(var("Format"), var("Format"))),
    );
    b.axiom("Format.group", pi(var("Format"), var("Format")));
    b.axiom(
        "Format.append",
        pi(var("Format"), pi(var("Format"), var("Format"))),
    );
    b.axiom(
        "Format.join",
        pi(app(var("List"), var("Format")), var("Format")),
    );
    b.axiom(
        "Format.pretty",
        pi(var("Format"), pi(var("Nat"), var("String"))),
    );
    b.axiom("Format.indentD", pi(var("Format"), var("Format")));
    b.axiom(
        "Format.bracket",
        pi(
            var("String"),
            pi(var("Format"), pi(var("String"), var("Format"))),
        ),
    );
}
/// Add `RBTree` (red-black tree).
#[allow(dead_code)]
pub fn add_rb_tree(b: &mut EnvBuilder) {
    if !b.contains("Ord") {
        b.axiom("Ord", pi(type0(), prop()));
    }
    b.axiom("RBTree", pi(type0(), type1()));
    b.axiom(
        "RBTree.empty",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("Ord"), bvar(0)), app(var("RBTree"), bvar(1))),
        ),
    );
    b.axiom(
        "RBTree.insert",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(
                    bvar(1),
                    pi(app(var("RBTree"), bvar(2)), app(var("RBTree"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "RBTree.find",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(
                    bvar(1),
                    pi(app(var("RBTree"), bvar(2)), app(var("Option"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "RBTree.contains",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(bvar(1), pi(app(var("RBTree"), bvar(2)), var("Bool"))),
            ),
        ),
    );
    b.axiom(
        "RBTree.erase",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(
                    bvar(1),
                    pi(app(var("RBTree"), bvar(2)), app(var("RBTree"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "RBTree.size",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("Ord"), bvar(0)),
                pi(app(var("RBTree"), bvar(1)), var("Nat")),
            ),
        ),
    );
    b.axiom(
        "RBTree.toList",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("Ord"), bvar(0)),
                pi(app(var("RBTree"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "RBTree.fromList",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(app(var("List"), bvar(1)), app(var("RBTree"), bvar(2))),
            ),
        ),
    );
}
