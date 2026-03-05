//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name, ReducibilityHint};

use super::functions::*;
use super::functions_2::*;
use super::types::EnvBuilder;

/// Add standard number-theoretic axioms.
#[allow(dead_code)]
pub fn add_number_theory_axioms(b: &mut EnvBuilder) {
    b.axiom("Nat.prime", pi(var("Nat"), prop()));
    b.axiom("Nat.dvd", pi(var("Nat"), pi(var("Nat"), prop())));
    b.axiom("Nat.coprime", pi(var("Nat"), pi(var("Nat"), prop())));
    b.axiom("Nat.totient", pi(var("Nat"), var("Nat")));
    b.axiom("Nat.primes", app(var("List"), var("Nat")));
    b.axiom("Nat.nextPrime", pi(var("Nat"), var("Nat")));
    b.axiom(
        "infinite_primes",
        pi_named(
            "n",
            var("Nat"),
            app(var("Nat.prime"), app(var("Nat.nextPrime"), bvar(0))),
        ),
    );
    b.axiom(
        "prime_factorization",
        pi_named(
            "n",
            var("Nat"),
            app(
                app(
                    var("Eq"),
                    app(
                        app(app(var("List.foldl"), var("Nat.mul")), var("Nat.zero")),
                        app(var("prime_factors"), bvar(0)),
                    ),
                ),
                bvar(0),
            ),
        ),
    );
    b.axiom(
        "prime_factors",
        pi(var("Nat"), app(var("List"), var("Nat"))),
    );
    b.axiom("Nat.sqrt", pi(var("Nat"), var("Nat")));
    b.axiom("Nat.log2", pi(var("Nat"), var("Nat")));
    b.axiom("Nat.pow2", pi(var("Nat"), var("Nat")));
    b.axiom("Int.prime", pi(var("Int"), prop()));
    b.axiom("Int.dvd", pi(var("Int"), pi(var("Int"), prop())));
    b.axiom("Int.gcd", pi(var("Int"), pi(var("Int"), var("Nat"))));
    b.axiom("Int.lcm", pi(var("Int"), pi(var("Int"), var("Nat"))));
    b.axiom("ChineseRemainder", prop());
    b.axiom("FermatLittleTheorem", prop());
    b.axiom("EulerTotientTheorem", prop());
    b.axiom("WilsonTheorem", prop());
}
/// Add `Classical.choice`-based existence uniqueness.
#[allow(dead_code)]
pub fn add_existence_uniqueness(b: &mut EnvBuilder) {
    b.axiom(
        "ExistsUnique",
        pi_implicit("α", type0(), pi(pi(bvar(0), prop()), prop())),
    );
    b.axiom(
        "ExistsUnique.intro",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "p",
                pi(bvar(0), prop()),
                pi_named(
                    "w",
                    bvar(1),
                    pi(
                        app(bvar(1), bvar(0)),
                        pi(
                            pi_named(
                                "y",
                                bvar(2),
                                pi(app(bvar(2), bvar(0)), app(app(var("Eq"), bvar(1)), bvar(3))),
                            ),
                            app(app(var("ExistsUnique"), bvar(4)), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "ExistsUnique.elim",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "p",
                pi(bvar(0), prop()),
                pi_implicit(
                    "q",
                    prop(),
                    pi(
                        app(app(var("ExistsUnique"), bvar(2)), bvar(1)),
                        pi(
                            pi_named(
                                "w",
                                bvar(3),
                                pi(
                                    app(bvar(3), bvar(0)),
                                    pi(
                                        pi_named(
                                            "y",
                                            bvar(4),
                                            pi(
                                                app(bvar(4), bvar(0)),
                                                app(app(var("Eq"), bvar(1)), bvar(3)),
                                            ),
                                        ),
                                        bvar(3),
                                    ),
                                ),
                            ),
                            bvar(2),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Add `Fin.succ_cast` and related lemmas.
#[allow(dead_code)]
pub fn add_fin_ops(b: &mut EnvBuilder) {
    if !b.contains("Fin") {
        add_fin(b);
    }
    b.axiom(
        "Fin.add",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), app(var("Fin"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Fin.mul",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), app(var("Fin"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Fin.mod",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), app(var("Fin"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Fin.div",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), app(var("Fin"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Fin.rev",
        pi_named(
            "n",
            var("Nat"),
            pi(app(var("Fin"), bvar(0)), app(var("Fin"), bvar(1))),
        ),
    );
    b.axiom(
        "Fin.toNat",
        pi_named("n", var("Nat"), pi(app(var("Fin"), bvar(0)), var("Nat"))),
    );
    b.axiom(
        "Fin.ofNat",
        pi_named("n", var("Nat"), pi(var("Nat"), app(var("Fin"), bvar(1)))),
    );
    b.axiom(
        "Fin.castSucc",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                app(var("Fin"), app(var("Nat.succ"), bvar(1))),
            ),
        ),
    );
    b.axiom(
        "Fin.lt",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), prop()),
            ),
        ),
    );
    b.axiom(
        "Fin.le",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), prop()),
            ),
        ),
    );
    b.axiom(
        "Fin.beq",
        pi_named(
            "n",
            var("Nat"),
            pi(
                app(var("Fin"), bvar(0)),
                pi(app(var("Fin"), bvar(1)), var("Bool")),
            ),
        ),
    );
}
/// Register common type algebra theorems as axioms.
#[allow(dead_code)]
pub fn add_type_algebra(b: &mut EnvBuilder) {
    b.axiom(
        "Prod.assoc_left",
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
                        app(
                            app(var("Prod"), app(app(var("Prod"), bvar(2)), bvar(1))),
                            bvar(0),
                        ),
                        app(
                            app(var("Prod"), bvar(3)),
                            app(app(var("Prod"), bvar(3)), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Prod.assoc_right",
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
                        app(
                            app(var("Prod"), bvar(2)),
                            app(app(var("Prod"), bvar(2)), bvar(1)),
                        ),
                        app(
                            app(var("Prod"), app(app(var("Prod"), bvar(3)), bvar(2))),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Sum.distrib_left",
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
                        app(
                            app(var("Prod"), bvar(2)),
                            app(app(var("Sum"), bvar(1)), bvar(0)),
                        ),
                        app(
                            app(var("Sum"), app(app(var("Prod"), bvar(3)), bvar(2))),
                            app(app(var("Prod"), bvar(3)), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.curry",
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
                        pi(app(app(var("Prod"), bvar(2)), bvar(1)), bvar(0)),
                        pi(bvar(3), pi(bvar(3), bvar(3))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.uncurry",
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
                        pi(bvar(2), pi(bvar(2), bvar(2))),
                        pi(app(app(var("Prod"), bvar(3)), bvar(2)), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.comp",
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
                        pi(bvar(1), bvar(1)),
                        pi(pi(bvar(2), bvar(2)), pi(bvar(3), bvar(3))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.id",
        pi_implicit("α", type0(), pi(bvar(0), bvar(1))),
    );
    b.axiom(
        "Function.const",
        pi_implicit(
            "α",
            type0(),
            pi_implicit("β", type0(), pi(bvar(1), pi(bvar(1), bvar(2)))),
        ),
    );
    b.axiom(
        "Function.flip",
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
                        pi(bvar(2), pi(bvar(2), bvar(2))),
                        pi(bvar(2), pi(bvar(3), bvar(3))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.on",
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
                        pi(bvar(1), pi(bvar(2), bvar(2))),
                        pi(pi(bvar(2), bvar(2)), pi(bvar(3), pi(bvar(4), bvar(4)))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Function.Injective",
        pi_implicit(
            "α",
            type0(),
            pi_implicit("β", type0(), pi(pi(bvar(1), bvar(1)), prop())),
        ),
    );
    b.axiom(
        "Function.Surjective",
        pi_implicit(
            "α",
            type0(),
            pi_implicit("β", type0(), pi(pi(bvar(1), bvar(1)), prop())),
        ),
    );
    b.axiom(
        "Function.Bijective",
        pi_implicit(
            "α",
            type0(),
            pi_implicit("β", type0(), pi(pi(bvar(1), bvar(1)), prop())),
        ),
    );
    b.axiom(
        "Function.LeftInverse",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(pi(bvar(1), bvar(1)), pi(pi(bvar(1), bvar(2)), prop())),
            ),
        ),
    );
    b.axiom(
        "Function.RightInverse",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(pi(bvar(1), bvar(1)), pi(pi(bvar(1), bvar(2)), prop())),
            ),
        ),
    );
}
/// Add `Finset` (finite sets) type and operations.
#[allow(dead_code)]
pub fn add_finset(b: &mut EnvBuilder) {
    b.axiom("Finset", pi(type0(), type0()));
    b.axiom(
        "Finset.empty",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("BEq"), bvar(0)), app(var("Finset"), bvar(1))),
        ),
    );
    b.axiom(
        "Finset.singleton",
        pi_implicit("α", type0(), pi(bvar(0), app(var("Finset"), bvar(1)))),
    );
    b.axiom(
        "Finset.insert",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    bvar(1),
                    pi(app(var("Finset"), bvar(2)), app(var("Finset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.union",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Finset"), bvar(1)),
                    pi(app(var("Finset"), bvar(2)), app(var("Finset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.inter",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Finset"), bvar(1)),
                    pi(app(var("Finset"), bvar(2)), app(var("Finset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.diff",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Finset"), bvar(1)),
                    pi(app(var("Finset"), bvar(2)), app(var("Finset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.mem",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(bvar(1), pi(app(var("Finset"), bvar(2)), prop())),
            ),
        ),
    );
    b.axiom(
        "Finset.card",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("BEq"), bvar(0)),
                pi(app(var("Finset"), bvar(1)), var("Nat")),
            ),
        ),
    );
    b.axiom(
        "Finset.toList",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("BEq"), bvar(0)),
                pi(app(var("Finset"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Finset.ofList",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(app(var("List"), bvar(1)), app(var("Finset"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Finset.image",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(var("BEq"), bvar(0)),
                    pi(
                        pi(bvar(2), bvar(1)),
                        pi(app(var("Finset"), bvar(3)), app(var("Finset"), bvar(2))),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.filter",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("BEq"), bvar(0)),
                pi(
                    pi(bvar(1), var("Bool")),
                    pi(app(var("Finset"), bvar(2)), app(var("Finset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.sum",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    var("Nat"),
                    pi(
                        pi(bvar(2), bvar(2)),
                        pi(app(var("Finset"), bvar(3)), var("Nat")),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Finset.range",
        pi(var("Nat"), app(var("Finset"), var("Nat"))),
    );
    b.axiom(
        "Finset.Icc",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(bvar(1), pi(bvar(2), app(var("Finset"), bvar(3)))),
            ),
        ),
    );
    b.axiom(
        "Finset.Ico",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(bvar(1), pi(bvar(2), app(var("Finset"), bvar(3)))),
            ),
        ),
    );
    b.axiom(
        "Finset.powerset",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "_",
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Finset"), bvar(1)),
                    app(var("Finset"), app(var("Finset"), bvar(2))),
                ),
            ),
        ),
    );
}
/// Add `Multiset` type and operations.
#[allow(dead_code)]
pub fn add_multiset(b: &mut EnvBuilder) {
    b.axiom("Multiset", pi(type0(), type0()));
    b.axiom(
        "Multiset.empty",
        pi_implicit("α", type0(), app(var("Multiset"), bvar(0))),
    );
    b.axiom(
        "Multiset.ofList",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("List"), bvar(0)), app(var("Multiset"), bvar(1))),
        ),
    );
    b.axiom(
        "Multiset.toList",
        pi_implicit(
            "α",
            type0(),
            pi(app(var("Multiset"), bvar(0)), app(var("List"), bvar(1))),
        ),
    );
    b.axiom(
        "Multiset.cons",
        pi_implicit(
            "α",
            type0(),
            pi(
                bvar(0),
                pi(app(var("Multiset"), bvar(1)), app(var("Multiset"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Multiset.add",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Multiset"), bvar(0)),
                pi(app(var("Multiset"), bvar(1)), app(var("Multiset"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Multiset.count",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(bvar(1), pi(app(var("Multiset"), bvar(2)), var("Nat"))),
            ),
        ),
    );
    b.axiom(
        "Multiset.card",
        pi_implicit("α", type0(), pi(app(var("Multiset"), bvar(0)), var("Nat"))),
    );
    b.axiom(
        "Multiset.mem",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(bvar(1), pi(app(var("Multiset"), bvar(2)), prop())),
            ),
        ),
    );
    b.axiom(
        "Multiset.map",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    pi(bvar(1), bvar(1)),
                    pi(app(var("Multiset"), bvar(2)), app(var("Multiset"), bvar(2))),
                ),
            ),
        ),
    );
    b.axiom(
        "Multiset.filter",
        pi_implicit(
            "α",
            type0(),
            pi(
                pi(bvar(0), var("Bool")),
                pi(app(var("Multiset"), bvar(1)), app(var("Multiset"), bvar(2))),
            ),
        ),
    );
    b.axiom(
        "Multiset.inter",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Multiset"), bvar(1)),
                    pi(app(var("Multiset"), bvar(2)), app(var("Multiset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Multiset.union",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("BEq"), bvar(0)),
                pi(
                    app(var("Multiset"), bvar(1)),
                    pi(app(var("Multiset"), bvar(2)), app(var("Multiset"), bvar(3))),
                ),
            ),
        ),
    );
    b.axiom(
        "Multiset.sum",
        pi_implicit(
            "α",
            type0(),
            pi(
                prop(),
                pi(
                    pi(bvar(1), prop()),
                    pi(app(var("Multiset"), bvar(2)), prop()),
                ),
            ),
        ),
    );
    b.axiom(
        "Multiset.sort",
        pi_implicit(
            "α",
            type0(),
            pi(
                app(var("Ord"), bvar(0)),
                pi(app(var("Multiset"), bvar(1)), app(var("List"), bvar(2))),
            ),
        ),
    );
}
/// Add `Equiv` (type equivalence / isomorphism).
#[allow(dead_code)]
pub fn add_equiv(b: &mut EnvBuilder) {
    b.axiom(
        "Equiv",
        pi_implicit("α", type0(), pi_implicit("β", type0(), type1())),
    );
    b.axiom(
        "Equiv.mk",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    pi(bvar(1), bvar(1)),
                    pi(
                        pi(bvar(1), bvar(2)),
                        pi(
                            pi_named(
                                "a",
                                bvar(3),
                                app(app(var("Eq"), app(bvar(2), app(bvar(2), bvar(0)))), bvar(0)),
                            ),
                            pi(
                                pi_named(
                                    "b",
                                    bvar(3),
                                    app(
                                        app(var("Eq"), app(bvar(3), app(bvar(3), bvar(0)))),
                                        bvar(0),
                                    ),
                                ),
                                app(app(var("Equiv"), bvar(5)), bvar(4)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.toFun",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(app(var("Equiv"), bvar(1)), bvar(0)),
                    pi(bvar(2), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.invFun",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(app(var("Equiv"), bvar(1)), bvar(0)),
                    pi(bvar(1), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.refl",
        pi_implicit("α", type0(), app(app(var("Equiv"), bvar(0)), bvar(0))),
    );
    b.axiom(
        "Equiv.symm",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi(
                    app(app(var("Equiv"), bvar(1)), bvar(0)),
                    app(app(var("Equiv"), bvar(1)), bvar(2)),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.trans",
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
                        app(app(var("Equiv"), bvar(2)), bvar(1)),
                        pi(
                            app(app(var("Equiv"), bvar(2)), bvar(1)),
                            app(app(var("Equiv"), bvar(3)), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.prodComm",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                app(
                    app(var("Equiv"), app(app(var("Prod"), bvar(1)), bvar(0))),
                    app(app(var("Prod"), bvar(0)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.sumComm",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                app(
                    app(var("Equiv"), app(app(var("Sum"), bvar(1)), bvar(0))),
                    app(app(var("Sum"), bvar(0)), bvar(1)),
                ),
            ),
        ),
    );
    b.axiom(
        "Equiv.sumProdLeft",
        pi_implicit(
            "α",
            type0(),
            pi_implicit(
                "β",
                type0(),
                pi_implicit(
                    "γ",
                    type0(),
                    app(
                        app(
                            var("Equiv"),
                            app(
                                app(var("Prod"), bvar(2)),
                                app(app(var("Sum"), bvar(1)), bvar(0)),
                            ),
                        ),
                        app(
                            app(var("Sum"), app(app(var("Prod"), bvar(3)), bvar(2))),
                            app(app(var("Prod"), bvar(3)), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    );
}
#[cfg(test)]
mod env_builder_extra_tests {
    use super::*;
    #[test]
    fn test_add_sum() {
        let mut b = EnvBuilder::fresh();
        add_sum(&mut b);
        assert!(b.contains("Sum"));
        assert!(b.contains("Sum.inl"));
        assert!(b.contains("Sum.inr"));
        assert!(b.contains("Sum.elim"));
    }
    #[test]
    fn test_add_prod_accessors() {
        let mut b = EnvBuilder::fresh();
        add_prod(&mut b);
        add_prod_accessors(&mut b);
        assert!(b.contains("Prod.fst"));
        assert!(b.contains("Prod.snd"));
        assert!(b.contains("Prod.map"));
        assert!(b.contains("Prod.swap"));
    }
    #[test]
    fn test_add_list_ext() {
        let mut b = EnvBuilder::fresh();
        add_list(&mut b);
        add_list_ext(&mut b);
        assert!(b.contains("List.length"));
        assert!(b.contains("List.append"));
        assert!(b.contains("List.map"));
        assert!(b.contains("List.filter"));
        assert!(b.contains("List.foldl"));
        assert!(b.contains("List.reverse"));
        assert!(b.contains("List.range"));
    }
    #[test]
    fn test_add_array_ext() {
        let mut b = EnvBuilder::fresh();
        add_array(&mut b);
        add_array_ext(&mut b);
        assert!(b.contains("Array.map"));
        assert!(b.contains("Array.filter"));
        assert!(b.contains("Array.foldl"));
        assert!(b.contains("Array.sort"));
        assert!(b.contains("Array.toList"));
    }
    #[test]
    fn test_add_io_ext() {
        let mut b = EnvBuilder::fresh();
        add_io(&mut b);
        add_io_ext(&mut b);
        assert!(b.contains("IO.println"));
        assert!(b.contains("IO.readLine"));
        assert!(b.contains("IO.fileRead"));
        assert!(b.contains("IO.fileWrite"));
    }
    #[test]
    fn test_add_nat_induction() {
        let mut b = EnvBuilder::fresh();
        add_nat(&mut b);
        add_nat_arith(&mut b);
        add_nat_induction(&mut b);
        assert!(b.contains("Nat.rec"));
        assert!(b.contains("Nat.casesOn"));
        assert!(b.contains("Nat.strongInduction"));
    }
    #[test]
    fn test_add_numeric_casts() {
        let mut b = EnvBuilder::fresh();
        add_nat(&mut b);
        add_int(&mut b);
        add_numeric_casts(&mut b);
        assert!(b.contains("Rat"));
        assert!(b.contains("Rat.mk"));
        assert!(b.contains("Rat.add"));
        assert!(b.contains("Rat.lt"));
    }
    #[test]
    fn test_add_equiv() {
        let mut b = EnvBuilder::fresh();
        add_prod(&mut b);
        add_sum(&mut b);
        add_equiv(&mut b);
        assert!(b.contains("Equiv"));
        assert!(b.contains("Equiv.mk"));
        assert!(b.contains("Equiv.refl"));
        assert!(b.contains("Equiv.symm"));
        assert!(b.contains("Equiv.trans"));
        assert!(b.contains("Equiv.prodComm"));
    }
    #[test]
    fn test_add_finset() {
        let mut b = EnvBuilder::fresh();
        add_type_classes(&mut b);
        add_nat(&mut b);
        add_nat_arith(&mut b);
        add_finset(&mut b);
        assert!(b.contains("Finset"));
        assert!(b.contains("Finset.empty"));
        assert!(b.contains("Finset.singleton"));
        assert!(b.contains("Finset.card"));
        assert!(b.contains("Finset.range"));
    }
    #[test]
    fn test_add_multiset() {
        let mut b = EnvBuilder::fresh();
        add_list(&mut b);
        add_type_classes(&mut b);
        add_multiset(&mut b);
        assert!(b.contains("Multiset"));
        assert!(b.contains("Multiset.empty"));
        assert!(b.contains("Multiset.card"));
        assert!(b.contains("Multiset.map"));
    }
    #[test]
    fn test_add_type_algebra() {
        let mut b = EnvBuilder::fresh();
        add_prod(&mut b);
        add_sum(&mut b);
        add_type_algebra(&mut b);
        assert!(b.contains("Prod.assoc_left"));
        assert!(b.contains("Function.curry"));
        assert!(b.contains("Function.uncurry"));
        assert!(b.contains("Function.comp"));
        assert!(b.contains("Function.id"));
        assert!(b.contains("Function.Injective"));
        assert!(b.contains("Function.Bijective"));
    }
    #[test]
    fn test_add_fin_ops() {
        let mut b = EnvBuilder::fresh();
        add_nat(&mut b);
        add_nat_arith(&mut b);
        add_fin(&mut b);
        add_fin_ops(&mut b);
        assert!(b.contains("Fin.add"));
        assert!(b.contains("Fin.mul"));
        assert!(b.contains("Fin.toNat"));
        assert!(b.contains("Fin.castSucc"));
    }
    #[test]
    fn test_lam_ext_works() {
        let l = lam_ext("x", var("Nat"), bvar(0));
        assert!(matches!(l, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_sort_u_v() {
        let _su = sort_u();
        let _sv = sort_v();
        assert!(matches!(_su, Expr::Sort(_)));
        assert!(matches!(_sv, Expr::Sort(_)));
    }
    #[test]
    fn test_pi_chain_empty() {
        let ty = pi_chain(vec![], var("Nat"));
        assert!(matches!(ty, Expr::Const(_, _)));
    }
    #[test]
    fn test_pi_chain_three() {
        let ty = pi_chain(vec![var("Nat"), var("Int"), var("Bool")], var("Float"));
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
}
