//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::env_builder::{app, bvar, pi, pi_implicit, pi_named, prop, sort, var, EnvBuilder};
use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    DecisionResult, EqBuilder, EqChain, EqRewriteRule, EqualityDatabase, EqualityWitness, PropEq,
    RewriteRuleDb, SetoidMorphism,
};

/// Build the `Eq.refl` proof for a given expression.
///
/// Returns `@Eq.refl α a`, which is the canonical reflexivity proof for
/// propositional equality in the Lean 4 kernel.
pub fn eq_refl(alpha: Expr, a: Expr) -> Expr {
    app(app(app(var("Eq.refl"), alpha), a.clone()), a)
}
/// Build `Eq.symm` applied to a given proof.
///
/// Given `h : a = b`, returns `@Eq.symm α a b h : b = a`.
pub fn eq_symm(alpha: Expr, a: Expr, b: Expr, h: Expr) -> Expr {
    app(
        app(app(app(app(var("Eq.symm"), alpha), a), b), h.clone()),
        h,
    )
}
/// Build `Eq.trans` applied to two proofs.
///
/// Given `h1 : a = b` and `h2 : b = c`, returns `@Eq.trans α a b c h1 h2`.
pub fn eq_trans(alpha: Expr, a: Expr, b: Expr, c: Expr, h1: Expr, h2: Expr) -> Expr {
    app(
        app(
            app(app(app(app(app(var("Eq.trans"), alpha), a), b), c), h1),
            h2.clone(),
        ),
        h2,
    )
}
/// Build `Eq.subst` applied to a proof.
///
/// Given `h : a = b` and `p : α → Prop` and `ha : p a`, returns `Eq.subst h ha`.
pub fn eq_subst(alpha: Expr, pred: Expr, a: Expr, b: Expr, h: Expr, ha: Expr) -> Expr {
    app(
        app(app(app(app(app(var("Eq.subst"), alpha), pred), a), b), h),
        ha,
    )
}
/// Build `congrArg` — congruence under function application.
///
/// Given `h : a = b`, returns `@congrArg α β a b f h : f a = f b`.
pub fn congr_arg(alpha: Expr, beta: Expr, a: Expr, b: Expr, f: Expr, h: Expr) -> Expr {
    app(
        app(app(app(app(app(var("congrArg"), alpha), beta), a), b), f),
        h,
    )
}
/// Build `congrFun` — congruence of function application.
///
/// Given `h : f = g`, returns `@congrFun α β f g h a : f a = g a`.
pub fn congr_fun(alpha: Expr, beta: Expr, f: Expr, g: Expr, h: Expr, a: Expr) -> Expr {
    app(
        app(app(app(app(app(var("congrFun"), alpha), beta), f), g), h),
        a,
    )
}
/// Build `HEq.refl` — the reflexivity proof for heterogeneous equality.
///
/// Returns `@HEq.refl α a : HEq a a`.
pub fn heq_refl(alpha: Expr, a: Expr) -> Expr {
    app(app(var("HEq.refl"), alpha), a)
}
/// Build `eq_of_heq` — recover homogeneous equality from heterogeneous.
///
/// Given `h : HEq a b` (where both have the same type), returns `h' : a = b`.
pub fn eq_of_heq(alpha: Expr, a: Expr, b: Expr, h: Expr) -> Expr {
    app(app(app(app(var("eq_of_heq"), alpha), a), b), h)
}
/// Build `heq_of_eq` — lift homogeneous equality to heterogeneous.
///
/// Given `h : a = b`, returns `@heq_of_eq α a b h : HEq a b`.
pub fn heq_of_eq(alpha: Expr, a: Expr, b: Expr, h: Expr) -> Expr {
    app(app(app(app(var("heq_of_eq"), alpha), a), b), h)
}
/// Build the `BEq` environment entry for a type.
///
/// Produces the `BEq` instance declaration using a boolean equality function
/// `beq_fn : α → α → Bool`.
pub fn build_beq_env(env: &mut EnvBuilder, type_name: &str, beq_fn: Expr) {
    let inst_name = format!("instBEq{type_name}");
    let ty = var(&format!("{type_name}.BEq"));
    let body = app(var("BEq.mk"), beq_fn);
    env.add_definition(Name::from_str(&inst_name), ty, body);
}
/// Build the `DecidableEq` environment entry for a type.
///
/// Produces a `DecidableEq` instance via a decision procedure `dec_fn`.
pub fn build_decidable_eq_env(env: &mut EnvBuilder, type_name: &str, dec_fn: Expr) {
    let inst_name = format!("instDecidableEq{type_name}");
    let ty = pi(var(type_name), pi(var(type_name), var("Prop")));
    let body = app(var("DecidableEq.mk"), dec_fn);
    env.add_definition(Name::from_str(&inst_name), ty, body);
}
/// Build the standard `HEq` environment entries.
///
/// Registers `HEq`, `HEq.refl`, `HEq.symm`, `HEq.trans`, and `heq_of_eq`
/// into the given builder.
pub fn build_heq_env(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("HEq"), sort(1));
    env.add_axiom(Name::from_str("HEq.refl"), sort(1));
    env.add_axiom(Name::from_str("HEq.symm"), sort(1));
    env.add_axiom(Name::from_str("HEq.trans"), sort(1));
    env.add_axiom(Name::from_str("heq_of_eq"), sort(1));
    env.add_axiom(Name::from_str("eq_of_heq"), sort(1));
}
/// Decidable equality at the Rust/meta level.
///
/// This mirrors the Lean 4 `DecidableEq` typeclass but operates on Rust types
/// used inside the OxiLean implementation.
pub trait DecidableEq: PartialEq {
    /// Returns `true` if `self == other`.
    fn decide_eq(&self, other: &Self) -> bool {
        self == other
    }
    /// Returns `Some(())` if equal, `None` if not.
    fn witness_eq(&self, other: &Self) -> Option<()> {
        if self == other {
            Some(())
        } else {
            None
        }
    }
}
impl DecidableEq for u8 {}
impl DecidableEq for u16 {}
impl DecidableEq for u32 {}
impl DecidableEq for u64 {}
impl DecidableEq for usize {}
impl DecidableEq for i8 {}
impl DecidableEq for i16 {}
impl DecidableEq for i32 {}
impl DecidableEq for i64 {}
impl DecidableEq for isize {}
impl DecidableEq for bool {}
impl DecidableEq for char {}
impl DecidableEq for String {}
impl DecidableEq for str {}
impl<T: PartialEq> DecidableEq for Vec<T> {}
impl<T: PartialEq> DecidableEq for Option<T> {}
impl<A: PartialEq, B: PartialEq> DecidableEq for (A, B) {}
/// Check whether two `Expr` nodes are alpha-equal (ignoring binder names).
///
/// This is a syntactic check only; it does not perform beta/eta reduction.
pub fn structural_eq(a: &Expr, b: &Expr) -> bool {
    a == b
}
/// Check whether two `Name`s are definitionally equal as strings.
pub fn name_eq(a: &Name, b: &Name) -> bool {
    a == b
}
/// Check equality of a list of expressions pairwise.
///
/// Returns `true` iff both slices have the same length and all pairs are
/// structurally equal.
pub fn exprs_eq(xs: &[Expr], ys: &[Expr]) -> bool {
    xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| x == y)
}
/// Decide equality of two `u32` values, returning a `DecisionResult`.
pub fn decide_u32_eq(a: u32, b: u32) -> DecisionResult<()> {
    if a == b {
        DecisionResult::IsTrue(())
    } else {
        DecisionResult::IsFalse(format!("{a} ≠ {b}"))
    }
}
/// Decide equality of two string slices, returning a `DecisionResult`.
pub fn decide_str_eq(a: &str, b: &str) -> DecisionResult<()> {
    if a == b {
        DecisionResult::IsTrue(())
    } else {
        DecisionResult::IsFalse(format!("{a:?} ≠ {b:?}"))
    }
}
/// Decide equality of two `Name` values.
pub fn decide_name_eq(a: &Name, b: &Name) -> DecisionResult<()> {
    if a == b {
        DecisionResult::IsTrue(())
    } else {
        DecisionResult::IsFalse(format!("{a} ≠ {b}"))
    }
}
/// A setoid: a type equipped with an equivalence relation.
///
/// This is the Rust-level analogue of Lean 4's `Setoid` typeclass,
/// used for quotiented types and proof-irrelevant equality.
pub trait Setoid {
    /// The equivalence relation.
    fn equiv(&self, other: &Self) -> bool;
    /// Reflexivity of the equivalence.
    fn refl(&self) -> bool {
        self.equiv(self)
    }
    /// Symmetry: if `self ~ other` then `other ~ self`.
    fn symm(&self, other: &Self) -> bool {
        if self.equiv(other) {
            other.equiv(self)
        } else {
            true
        }
    }
}
impl<T: PartialEq> Setoid for T {
    fn equiv(&self, other: &Self) -> bool {
        self == other
    }
}
/// Apply congruence: if `f = g` and `a = b` then `f a = g b`.
///
/// At the Rust meta-level this is simply function application, but this
/// function makes the intent explicit.
pub fn congr<A, B>(f: impl Fn(A) -> B, a: A) -> B {
    f(a)
}
/// Build a proof of `a = a` (reflexivity) at the meta level.
///
/// This is a no-op on the Rust side but provides a uniform API.
pub fn refl<T: PartialEq>(a: T) -> EqualityWitness<T> {
    EqualityWitness { value: a }
}
/// Substitute equals for equals at the Rust meta level.
///
/// Given a witness `a = b` and a value `pa : P a`, returns `pa` interpreted
/// as `P b`. Since Rust is monomorphic, this is just the identity.
pub fn subst<T, P>(_witness: &EqualityWitness<T>, pa: P) -> P {
    pa
}
/// Determine whether an expression is syntactically an equality proposition.
///
/// Returns `Some((ty, lhs, rhs))` if the expression has the form `@Eq ty lhs rhs`,
/// otherwise `None`.
pub fn as_eq(e: &Expr) -> Option<(Expr, Expr, Expr)> {
    match e {
        Expr::App(f, rhs) => match f.as_ref() {
            Expr::App(g, lhs) => match g.as_ref() {
                Expr::App(h, ty) => match h.as_ref() {
                    Expr::Const(n, _) if n.to_string() == "Eq" => Some((
                        ty.as_ref().clone(),
                        lhs.as_ref().clone(),
                        rhs.as_ref().clone(),
                    )),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
/// Determine whether an expression is a heterogeneous equality (`HEq`).
pub fn as_heq(e: &Expr) -> Option<(Expr, Expr, Expr, Expr)> {
    match e {
        Expr::App(f, rhs) => match f.as_ref() {
            Expr::App(g, lhs) => match g.as_ref() {
                Expr::App(h, rhs_ty) => match h.as_ref() {
                    Expr::App(i, lhs_ty) => match i.as_ref() {
                        Expr::Const(n, _) if n.to_string() == "HEq" => Some((
                            lhs_ty.as_ref().clone(),
                            lhs.as_ref().clone(),
                            rhs_ty.as_ref().clone(),
                            rhs.as_ref().clone(),
                        )),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
/// Build an `Eq` expression `@Eq ty lhs rhs`.
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    app(app(app(var("Eq"), ty), lhs), rhs)
}
/// Build an `HEq` expression `@HEq lhs_ty lhs rhs_ty rhs`.
pub fn mk_heq(lhs_ty: Expr, lhs: Expr, rhs_ty: Expr, rhs: Expr) -> Expr {
    app(app(app(app(var("HEq"), lhs_ty), lhs), rhs_ty), rhs)
}
/// Register the standard `Eq` and associated declarations into an environment.
///
/// This includes `Eq`, `Eq.refl`, `Eq.symm`, `Eq.trans`, `Eq.subst`,
/// `congrArg`, `congrFun`, and `congr`.
pub fn build_eq_env(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Eq"), sort(1));
    env.add_axiom(
        Name::from_str("Eq.refl"),
        pi_implicit(
            "α",
            sort(1),
            pi_named(
                "a",
                bvar(0),
                app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("Eq.symm"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "a",
                bvar(0),
                pi_implicit(
                    "b",
                    bvar(1),
                    pi(
                        app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                        app(app(app(var("Eq"), bvar(3)), bvar(1)), bvar(2)),
                    ),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("Eq.trans"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "a",
                bvar(0),
                pi_implicit(
                    "b",
                    bvar(1),
                    pi_implicit(
                        "c",
                        bvar(2),
                        pi(
                            app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                            pi(
                                app(app(app(var("Eq"), bvar(4)), bvar(2)), bvar(1)),
                                app(app(app(var("Eq"), bvar(5)), bvar(4)), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("Eq.subst"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "motive",
                pi(bvar(0), prop()),
                pi_implicit(
                    "a",
                    bvar(1),
                    pi_implicit(
                        "b",
                        bvar(2),
                        pi(
                            app(app(app(var("Eq"), bvar(3)), bvar(1)), bvar(0)),
                            pi(app(bvar(3), bvar(2)), app(bvar(4), bvar(2))),
                        ),
                    ),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("congrArg"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "β",
                sort(1),
                pi_named(
                    "f",
                    pi(bvar(1), bvar(1)),
                    pi_implicit(
                        "a",
                        bvar(2),
                        pi_implicit(
                            "b",
                            bvar(3),
                            pi(
                                app(app(app(var("Eq"), bvar(4)), bvar(1)), bvar(0)),
                                app(
                                    app(app(var("Eq"), bvar(4)), app(bvar(3), bvar(2))),
                                    app(bvar(3), bvar(1)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("congrFun"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "β",
                sort(1),
                pi_implicit(
                    "f",
                    pi(bvar(1), bvar(1)),
                    pi_implicit(
                        "g",
                        pi(bvar(2), bvar(2)),
                        pi(
                            app(app(app(var("Eq"), pi(bvar(3), bvar(3))), bvar(1)), bvar(0)),
                            pi_named(
                                "a",
                                bvar(4),
                                app(
                                    app(app(var("Eq"), bvar(4)), app(bvar(3), bvar(0))),
                                    app(bvar(2), bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("congr"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "β",
                sort(1),
                pi_implicit(
                    "f",
                    pi(bvar(1), bvar(1)),
                    pi_implicit(
                        "g",
                        pi(bvar(2), bvar(2)),
                        pi(
                            app(app(app(var("Eq"), pi(bvar(3), bvar(3))), bvar(1)), bvar(0)),
                            pi_implicit(
                                "a",
                                bvar(4),
                                pi_implicit(
                                    "b",
                                    bvar(5),
                                    pi(
                                        app(app(app(var("Eq"), bvar(6)), bvar(1)), bvar(0)),
                                        app(
                                            app(app(var("Eq"), bvar(6)), app(bvar(5), bvar(2))),
                                            app(bvar(4), bvar(1)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
}
/// Register `Eq.mpr` and related transport lemmas.
pub fn build_eq_mpr_env(env: &mut EnvBuilder) {
    env.add_axiom(
        Name::from_str("Eq.mpr"),
        pi_implicit(
            "α",
            prop(),
            pi_implicit(
                "β",
                prop(),
                pi(
                    app(app(app(var("Eq"), prop()), bvar(1)), bvar(0)),
                    pi(bvar(1), bvar(3)),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("Eq.mp"),
        pi_implicit(
            "α",
            prop(),
            pi_implicit(
                "β",
                prop(),
                pi(
                    app(app(app(var("Eq"), prop()), bvar(1)), bvar(0)),
                    pi(bvar(2), bvar(2)),
                ),
            ),
        ),
    );
    env.add_axiom(
        Name::from_str("id.def"),
        pi_implicit(
            "α",
            sort(1),
            pi_implicit(
                "a",
                bvar(0),
                app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
            ),
        ),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prop_eq_refl() {
        let ty = var("Nat");
        let a = var("x");
        let eq = PropEq::refl(ty, a.clone());
        assert!(eq.is_refl());
        assert_eq!(eq.lhs, a);
        assert_eq!(eq.rhs, a);
    }
    #[test]
    fn test_prop_eq_symm() {
        let ty = var("Nat");
        let a = var("a");
        let b = var("b");
        let eq = PropEq::new(ty.clone(), a.clone(), b.clone());
        let sym = eq.symm();
        assert_eq!(sym.lhs, b);
        assert_eq!(sym.rhs, a);
    }
    #[test]
    fn test_prop_eq_trans() {
        let ty = var("Nat");
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let e1 = PropEq::new(ty.clone(), a.clone(), b.clone());
        let e2 = PropEq::new(ty.clone(), b.clone(), c.clone());
        let e3 = e1.trans(e2).expect("trans should succeed");
        assert_eq!(e3.lhs, a);
        assert_eq!(e3.rhs, c);
    }
    #[test]
    fn test_prop_eq_trans_mismatch() {
        let ty = var("Nat");
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let e1 = PropEq::new(ty.clone(), a.clone(), b.clone());
        let e2 = PropEq::new(ty.clone(), c.clone(), a.clone());
        assert!(e1.trans(e2).is_none());
    }
    #[test]
    fn test_eq_chain_collapse() {
        let ty = var("Nat");
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let mut chain = EqChain::new(ty.clone());
        chain.push(PropEq::new(ty.clone(), a.clone(), b.clone()));
        chain.push(PropEq::new(ty.clone(), b.clone(), c.clone()));
        let collapsed = chain.collapse().expect("collapse should succeed");
        assert_eq!(collapsed.lhs, a);
        assert_eq!(collapsed.rhs, c);
    }
    #[test]
    fn test_eq_chain_empty() {
        let chain = EqChain::new(var("Nat"));
        assert!(chain.is_empty());
        assert!(chain.collapse().is_none());
    }
    #[test]
    fn test_decision_result_and() {
        let a: DecisionResult<()> = DecisionResult::IsTrue(());
        let b: DecisionResult<()> = DecisionResult::IsTrue(());
        let ab = a.and(b);
        assert!(ab.is_true());
    }
    #[test]
    fn test_decision_result_and_false() {
        let a: DecisionResult<()> = DecisionResult::IsTrue(());
        let b: DecisionResult<()> = DecisionResult::IsFalse("no".to_string());
        let ab = a.and(b);
        assert!(ab.is_false());
    }
    #[test]
    fn test_decision_result_or() {
        let a: DecisionResult<()> = DecisionResult::IsFalse("no".to_string());
        let b: DecisionResult<()> = DecisionResult::IsTrue(());
        let ab = a.or(b);
        assert!(ab.is_true());
    }
    #[test]
    fn test_equality_database_lookup() {
        let mut db = EqualityDatabase::new();
        let a = Name::from_str("a");
        let b = Name::from_str("b");
        db.register(a.clone(), b.clone(), var("proof_ab"));
        let found = db.lookup(&a, &b);
        assert!(found.is_some());
    }
    #[test]
    fn test_equality_database_symm() {
        let mut db = EqualityDatabase::new();
        let a = Name::from_str("a");
        let b = Name::from_str("b");
        db.register(a.clone(), b.clone(), var("proof_ab"));
        let found = db.lookup(&b, &a);
        assert!(found.is_some());
    }
    #[test]
    fn test_decide_str_eq() {
        assert!(decide_str_eq("hello", "hello").is_true());
        assert!(decide_str_eq("hello", "world").is_false());
    }
    #[test]
    fn test_decide_u32_eq() {
        assert!(decide_u32_eq(42, 42).is_true());
        assert!(decide_u32_eq(42, 43).is_false());
    }
    #[test]
    fn test_equality_witness() {
        let w = EqualityWitness::try_new(&42u32, &42u32);
        assert!(w.is_some());
        let w = EqualityWitness::try_new(&1u32, &2u32);
        assert!(w.is_none());
    }
    #[test]
    fn test_exprs_eq() {
        let a = vec![var("x"), var("y")];
        let b = vec![var("x"), var("y")];
        assert!(exprs_eq(&a, &b));
        let c = vec![var("x"), var("z")];
        assert!(!exprs_eq(&a, &c));
    }
    #[test]
    fn test_mk_eq_and_as_eq() {
        let ty = var("Nat");
        let lhs = var("a");
        let rhs = var("b");
        let eq_expr = mk_eq(ty.clone(), lhs.clone(), rhs.clone());
        let parsed = as_eq(&eq_expr);
        assert!(parsed.is_some());
        let (t, l, r) = parsed.expect("parsed should be valid");
        assert_eq!(t, ty);
        assert_eq!(l, lhs);
        assert_eq!(r, rhs);
    }
    #[test]
    fn test_equality_database_len() {
        let mut db = EqualityDatabase::new();
        assert_eq!(db.len(), 0);
        assert!(db.is_empty());
        db.register(Name::from_str("a"), Name::from_str("b"), var("p"));
        assert_eq!(db.len(), 1);
        assert!(!db.is_empty());
    }
    #[test]
    fn test_eq_chain_len() {
        let ty = var("Nat");
        let a = var("a");
        let b = var("b");
        let c = var("c");
        let mut chain = EqChain::new(ty.clone());
        chain.push(PropEq::new(ty.clone(), a.clone(), b.clone()));
        chain.push(PropEq::new(ty.clone(), b.clone(), c.clone()));
        assert_eq!(chain.len(), 2);
    }
}
/// Check pairwise equality of two vectors of expressions.
pub fn exprs_eq_pairwise(a: &[oxilean_kernel::Expr], b: &[oxilean_kernel::Expr]) -> Vec<bool> {
    if a.len() != b.len() {
        return vec![];
    }
    a.iter().zip(b.iter()).map(|(x, y)| x == y).collect()
}
/// Count the number of positions where two expression slices differ.
pub fn count_diffs(a: &[oxilean_kernel::Expr], b: &[oxilean_kernel::Expr]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}
/// Check if two expression slices are equal up to a permutation.
///
/// This is O(n²) and is only suitable for small slices.
pub fn exprs_eq_mod_permutation(a: &[oxilean_kernel::Expr], b: &[oxilean_kernel::Expr]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut used = vec![false; b.len()];
    'outer: for ea in a {
        for (j, eb) in b.iter().enumerate() {
            if !used[j] && ea == eb {
                used[j] = true;
                continue 'outer;
            }
        }
        return false;
    }
    true
}
/// Extensional equality: two functions are equal if they agree on all inputs.
///
/// At the Rust meta level, this is implemented via a finite test set.
pub fn extensionally_equal<A, B: PartialEq>(
    f: impl Fn(&A) -> B,
    g: impl Fn(&A) -> B,
    test_points: &[A],
) -> bool {
    test_points.iter().all(|x| f(x) == g(x))
}
/// Leibniz equality witness: if `a = b` and `P a` holds, then `P b` holds.
///
/// This function demonstrates the Leibniz substitution principle at the
/// meta level by applying the predicate to both sides.
pub fn leibniz_subst<T: PartialEq, P>(_a: &T, _b: &T, witness: &EqualityWitness<T>, pa: P) -> P {
    let _ = witness;
    pa
}
/// Check if an expression is a reflexivity application `@Eq.refl α a`.
///
/// Returns `Some(a)` if so, `None` otherwise.
pub fn is_refl_proof(e: &oxilean_kernel::Expr) -> Option<oxilean_kernel::Expr> {
    match e {
        oxilean_kernel::Expr::App(f, a) => match f.as_ref() {
            oxilean_kernel::Expr::App(g, _alpha) => match g.as_ref() {
                oxilean_kernel::Expr::Const(n, _) if n.to_string() == "Eq.refl" => {
                    Some(a.as_ref().clone())
                }
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
#[cfg(test)]
mod eq_extended_tests {
    use super::*;
    fn v(s: &str) -> oxilean_kernel::Expr {
        var(s)
    }
    fn n(s: &str) -> oxilean_kernel::Name {
        oxilean_kernel::Name::from_str(s)
    }
    #[test]
    fn test_exprs_eq_pairwise_same() {
        let a = vec![v("x"), v("y")];
        let b = vec![v("x"), v("y")];
        let result = exprs_eq_pairwise(&a, &b);
        assert!(result.iter().all(|&b| b));
    }
    #[test]
    fn test_exprs_eq_pairwise_diff() {
        let a = vec![v("x"), v("y")];
        let b = vec![v("x"), v("z")];
        let result = exprs_eq_pairwise(&a, &b);
        assert!(result[0]);
        assert!(!result[1]);
    }
    #[test]
    fn test_exprs_eq_pairwise_length_mismatch() {
        let result = exprs_eq_pairwise(&[v("x")], &[v("x"), v("y")]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_count_diffs() {
        let a = vec![v("x"), v("y"), v("z")];
        let b = vec![v("x"), v("a"), v("z")];
        assert_eq!(count_diffs(&a, &b), 1);
    }
    #[test]
    fn test_exprs_eq_mod_permutation_same() {
        let a = vec![v("x"), v("y")];
        let b = vec![v("y"), v("x")];
        assert!(exprs_eq_mod_permutation(&a, &b));
    }
    #[test]
    fn test_exprs_eq_mod_permutation_diff() {
        let a = vec![v("x"), v("y")];
        let b = vec![v("x"), v("z")];
        assert!(!exprs_eq_mod_permutation(&a, &b));
    }
    #[test]
    fn test_eq_builder_empty() {
        let b = EqBuilder::start(v("Nat"), v("a"));
        assert_eq!(b.num_steps(), 0);
        assert!(b.build().is_none());
    }
    #[test]
    fn test_eq_builder_one_step() {
        let builder = EqBuilder::start(v("Nat"), v("a")).step(v("b"), v("proof_ab"));
        let eq = builder.build();
        assert!(eq.is_some());
    }
    #[test]
    fn test_extensionally_equal() {
        let f = |x: &u32| x * 2;
        let g = |x: &u32| x + x;
        let pts = [0u32, 1, 2, 3, 4];
        assert!(extensionally_equal(f, g, &pts));
    }
    #[test]
    fn test_extensionally_not_equal() {
        let f = |x: &u32| x + 1;
        let g = |x: &u32| x * 2;
        let pts = [0u32, 1, 2];
        assert!(!extensionally_equal(f, g, &pts));
    }
    #[test]
    fn test_leibniz_subst() {
        let w = EqualityWitness { value: 42u32 };
        let pa = "result";
        let result = leibniz_subst(&42u32, &42u32, &w, pa);
        assert_eq!(result, "result");
    }
    #[test]
    fn test_eq_rewrite_rule_apply() {
        let rule = EqRewriteRule::new(n("r"), v("a"), v("b"));
        assert!(rule.matches(&v("a")));
        assert_eq!(rule.apply(&v("a")), Some(v("b")));
        assert!(rule.apply(&v("c")).is_none());
    }
    #[test]
    fn test_eq_rewrite_rule_reversed() {
        let rule = EqRewriteRule::new(n("r"), v("a"), v("b")).make_reversible();
        let rev = rule.reversed().expect("reversed should succeed");
        assert_eq!(rev.lhs, v("b"));
        assert_eq!(rev.rhs, v("a"));
    }
    #[test]
    fn test_eq_rewrite_rule_not_reversible() {
        let rule = EqRewriteRule::new(n("r"), v("a"), v("b"));
        assert!(rule.reversed().is_none());
    }
    #[test]
    fn test_rewrite_rule_db_add_find() {
        let mut db = RewriteRuleDb::new();
        db.add(EqRewriteRule::new(n("r1"), v("x"), v("y")));
        let found = db.find_match(&v("x"));
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").rhs, v("y"));
    }
    #[test]
    fn test_rewrite_rule_db_apply_all() {
        let mut db = RewriteRuleDb::new();
        db.add(EqRewriteRule::new(n("r1"), v("x"), v("y")));
        db.add(EqRewriteRule::new(n("r2"), v("x"), v("z")));
        let results = db.apply_all(&v("x"));
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_rewrite_rule_db_remove() {
        let mut db = RewriteRuleDb::new();
        db.add(EqRewriteRule::new(n("r1"), v("x"), v("y")));
        db.remove(&n("r1"));
        assert!(db.is_empty());
    }
    #[test]
    fn test_is_refl_proof_none() {
        let e = v("not_refl");
        assert!(is_refl_proof(&e).is_none());
    }
}
pub fn eq_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn eq_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    eq_ext_app(eq_ext_app(f, a), b)
}
pub fn eq_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn eq_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn eq_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn eq_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn eq_ext_nat_ty() -> Expr {
    eq_ext_cst("Nat")
}
pub fn eq_ext_bool_ty() -> Expr {
    eq_ext_cst("Bool")
}
pub fn eq_ext_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::Anonymous,
        Box::new(dom),
        Box::new(cod),
    )
}
/// Build the type of the Eq-class reflexivity law:
/// `∀ {α : Type} (a : α), a = a`.
pub fn mk_eq_class_refl_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
        ),
    )
}
/// Build the type of the Eq-class symmetry law:
/// `∀ {α : Type} {a b : α}, a = b → b = a`.
pub fn mk_eq_class_symm_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi(
                    app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                    app(app(app(var("Eq"), bvar(3)), bvar(1)), bvar(2)),
                ),
            ),
        ),
    )
}
/// Build the type of the Eq-class transitivity law:
/// `∀ {α : Type} {a b c : α}, a = b → b = c → a = c`.
pub fn mk_eq_class_trans_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_implicit(
                    "c",
                    bvar(2),
                    pi(
                        app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                        pi(
                            app(app(app(var("Eq"), bvar(4)), bvar(2)), bvar(1)),
                            app(app(app(var("Eq"), bvar(5)), bvar(4)), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the BEq/Eq consistency axiom:
/// `∀ {α : Type} {a b : α}, (BEq.beq a b = true) → a = b`.
pub fn mk_beq_eq_consistency_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi(
                    app(
                        app(
                            app(var("Eq"), eq_ext_bool_ty()),
                            app(app(var("BEq.beq"), bvar(1)), bvar(0)),
                        ),
                        var("Bool.true"),
                    ),
                    app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                ),
            ),
        ),
    )
}
/// Build the type of the Nat equality decidability axiom:
/// `∀ (a b : Nat), Decidable (a = b)`.
pub fn mk_nat_decidable_eq_ty() -> Expr {
    pi_named(
        "a",
        eq_ext_nat_ty(),
        pi_named(
            "b",
            eq_ext_nat_ty(),
            app(
                var("Decidable"),
                app(app(app(var("Eq"), eq_ext_nat_ty()), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the Bool equality decidability axiom.
pub fn mk_bool_decidable_eq_ty() -> Expr {
    pi_named(
        "a",
        eq_ext_bool_ty(),
        pi_named(
            "b",
            eq_ext_bool_ty(),
            app(
                var("Decidable"),
                app(app(app(var("Eq"), eq_ext_bool_ty()), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the Char decidable equality axiom.
pub fn mk_char_decidable_eq_ty() -> Expr {
    pi_named(
        "a",
        eq_ext_cst("Char"),
        pi_named(
            "b",
            eq_ext_cst("Char"),
            app(
                var("Decidable"),
                app(app(app(var("Eq"), eq_ext_cst("Char")), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the Float equality axiom.
pub fn mk_float_eq_decidable_ty() -> Expr {
    pi_named(
        "a",
        eq_ext_cst("Float"),
        pi_named(
            "b",
            eq_ext_cst("Float"),
            app(
                var("Decidable"),
                app(app(app(var("Eq"), eq_ext_cst("Float")), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the Int decidable equality axiom.
pub fn mk_int_decidable_eq_ty() -> Expr {
    pi_named(
        "a",
        eq_ext_cst("Int"),
        pi_named(
            "b",
            eq_ext_cst("Int"),
            app(
                var("Decidable"),
                app(app(app(var("Eq"), eq_ext_cst("Int")), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the List equality axiom:
/// `∀ {α : Type} (xs ys : List α), Decidable (xs = ys)`.
pub fn mk_list_decidable_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "xs",
            app(var("List"), bvar(0)),
            pi_named(
                "ys",
                app(var("List"), bvar(1)),
                app(
                    var("Decidable"),
                    app(
                        app(app(var("Eq"), app(var("List"), bvar(2))), bvar(1)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Option decidable equality axiom.
pub fn mk_option_decidable_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "x",
            app(var("Option"), bvar(0)),
            pi_named(
                "y",
                app(var("Option"), bvar(1)),
                app(
                    var("Decidable"),
                    app(
                        app(app(var("Eq"), app(var("Option"), bvar(2))), bvar(1)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Pair (Prod) decidable equality axiom.
pub fn mk_pair_decidable_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_named(
                "p",
                app(app(var("Prod"), bvar(1)), bvar(0)),
                pi_named(
                    "q",
                    app(app(var("Prod"), bvar(2)), bvar(1)),
                    app(
                        var("Decidable"),
                        app(
                            app(
                                app(var("Eq"), app(app(var("Prod"), bvar(3)), bvar(2))),
                                bvar(1),
                            ),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Leibniz equality axiom:
/// `∀ {α : Type} (a b : α), (∀ (P : α → Prop), P a → P b) → a = b`.
pub fn mk_leibniz_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            pi_named(
                "b",
                bvar(1),
                pi(
                    pi_named(
                        "P",
                        pi(bvar(2), eq_ext_prop()),
                        pi(app(bvar(0), bvar(2)), app(bvar(1), bvar(2))),
                    ),
                    app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                ),
            ),
        ),
    )
}
/// Build the type of the Leibniz substitution direction:
/// `∀ {α : Type} {a b : α}, a = b → ∀ (P : α → Prop), P a → P b`.
pub fn mk_leibniz_subst_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi(
                    app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                    pi_named(
                        "P",
                        pi(bvar(3), eq_ext_prop()),
                        pi(app(bvar(0), bvar(3)), app(bvar(1), bvar(2))),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the equality reflection axiom:
/// Same as Leibniz substitution: `∀ {α} {a b : α}, a = b → ∀ P, P a → P b`.
pub fn mk_eq_reflection_ty() -> Expr {
    mk_leibniz_subst_ty()
}
/// Build the type of Streicher's K axiom:
/// `∀ {α : Type} {a : α} (p : a = a), p = Eq.refl a`.
pub fn mk_k_axiom_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_named(
                "p",
                app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
                app(
                    app(
                        app(
                            var("Eq"),
                            app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(1)),
                        ),
                        bvar(0),
                    ),
                    app(app(var("Eq.refl"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    )
}
/// Build the type of the UIP axiom:
/// `∀ {α : Type} {a b : α} (p q : a = b), p = q`.
pub fn mk_uip_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_named(
                    "p",
                    app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                    pi_named(
                        "q",
                        app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                        app(
                            app(
                                app(
                                    var("Eq"),
                                    app(app(app(var("Eq"), bvar(4)), bvar(3)), bvar(2)),
                                ),
                                bvar(1),
                            ),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the J axiom (Martin-Löf path induction).
pub fn mk_j_axiom_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_named(
                "P",
                pi_named(
                    "b",
                    bvar(1),
                    pi(
                        app(app(app(var("Eq"), bvar(2)), bvar(2)), bvar(0)),
                        eq_ext_prop(),
                    ),
                ),
                pi(
                    app(
                        app(bvar(0), bvar(1)),
                        app(app(var("Eq.refl"), bvar(2)), bvar(1)),
                    ),
                    pi_implicit(
                        "b",
                        bvar(3),
                        pi_named(
                            "h",
                            app(app(app(var("Eq"), bvar(4)), bvar(4)), bvar(0)),
                            app(app(bvar(3), bvar(1)), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the HEq introduction rule:
/// `∀ {α : Type} (a : α), HEq a a`.
pub fn mk_heq_intro_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            app(
                app(app(app(var("HEq"), bvar(1)), bvar(0)), bvar(1)),
                bvar(0),
            ),
        ),
    )
}
/// Build the type of HEq type-equality consequence:
/// `∀ {α β : Type} {a : α} {b : β}, HEq a b → α = β`.
pub fn mk_heq_type_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_implicit(
                "a",
                bvar(1),
                pi_implicit(
                    "b",
                    bvar(1),
                    pi(
                        app(
                            app(app(app(var("HEq"), bvar(3)), bvar(1)), bvar(2)),
                            bvar(0),
                        ),
                        app(app(app(var("Eq"), sort(1)), bvar(4)), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the general substitution axiom:
/// `∀ {α : Type} {a b : α} (P : α → Type), a = b → P a → P b`.
pub fn mk_subst_axiom_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_named(
                    "P",
                    pi(bvar(2), sort(1)),
                    pi(
                        app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                        pi(app(bvar(1), bvar(3)), app(bvar(2), bvar(3))),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the general congruence axiom:
/// `∀ {α β : Type} (f : α → β) {a b : α}, a = b → f a = f b`.
pub fn mk_cong_axiom_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_named(
                "f",
                pi(bvar(1), bvar(1)),
                pi_implicit(
                    "a",
                    bvar(2),
                    pi_implicit(
                        "b",
                        bvar(3),
                        pi(
                            app(app(app(var("Eq"), bvar(4)), bvar(1)), bvar(0)),
                            app(
                                app(app(var("Eq"), bvar(4)), app(bvar(2), bvar(2))),
                                app(bvar(2), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the functional extensionality axiom:
/// `∀ {α β : Type} (f g : α → β), (∀ x, f x = g x) → f = g`.
pub fn mk_funext_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_named(
                "f",
                pi(bvar(1), bvar(1)),
                pi_named(
                    "g",
                    pi(bvar(2), bvar(2)),
                    pi(
                        pi_named(
                            "x",
                            bvar(3),
                            app(
                                app(app(var("Eq"), bvar(3)), app(bvar(2), bvar(0))),
                                app(bvar(1), bvar(0)),
                            ),
                        ),
                        app(app(app(var("Eq"), pi(bvar(4), bvar(4))), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of propositional extensionality:
/// `∀ (P Q : Prop), (P ↔ Q) → P = Q`.
pub fn mk_propext_ty() -> Expr {
    pi_named(
        "P",
        eq_ext_prop(),
        pi_named(
            "Q",
            eq_ext_prop(),
            pi(
                app(app(var("And"), pi(bvar(1), bvar(1))), pi(bvar(1), bvar(2))),
                app(app(app(var("Eq"), eq_ext_prop()), bvar(2)), bvar(1)),
            ),
        ),
    )
}
/// Build the type of the quotient soundness axiom.
pub fn mk_quotient_sound_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "r",
            pi(bvar(0), pi(bvar(1), eq_ext_prop())),
            pi_named(
                "a",
                bvar(1),
                pi_named(
                    "b",
                    bvar(2),
                    pi(
                        app(app(bvar(2), bvar(1)), bvar(0)),
                        app(
                            app(
                                app(var("Eq"), app(var("Quotient"), bvar(4))),
                                app(app(var("Quotient.mk"), bvar(4)), bvar(2)),
                            ),
                            app(app(var("Quotient.mk"), bvar(5)), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the bisimulation-equality axiom.
pub fn mk_bisim_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "R",
            pi(bvar(0), pi(bvar(1), eq_ext_prop())),
            pi(
                app(var("Bisimulation"), bvar(0)),
                pi_named(
                    "a",
                    bvar(2),
                    pi_named(
                        "b",
                        bvar(3),
                        pi(
                            app(app(bvar(3), bvar(1)), bvar(0)),
                            app(app(app(var("Eq"), bvar(5)), bvar(2)), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the observational equality axiom.
pub fn mk_obs_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            pi_named(
                "b",
                bvar(1),
                pi(
                    pi_named(
                        "P",
                        pi(bvar(2), eq_ext_prop()),
                        app(
                            app(var("And"), pi(app(bvar(0), bvar(2)), app(bvar(1), bvar(2)))),
                            pi(app(bvar(1), bvar(2)), app(bvar(0), bvar(3))),
                        ),
                    ),
                    app(app(app(var("Eq"), bvar(3)), bvar(2)), bvar(1)),
                ),
            ),
        ),
    )
}
/// Build the type of the setoid construction axiom.
pub fn mk_setoid_ax_ty() -> Expr {
    pi_named(
        "α",
        sort(1),
        pi_named(
            "r",
            pi(bvar(0), pi(bvar(1), eq_ext_prop())),
            pi(
                app(var("IsEquivalence"), bvar(0)),
                app(var("Setoid"), bvar(2)),
            ),
        ),
    )
}
/// Build the type of the setoid morphism axiom.
pub fn mk_setoid_morphism_ax_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_named(
                "f",
                pi(bvar(1), bvar(1)),
                pi(
                    app(var("Respects"), bvar(0)),
                    app(var("SetoidMorphism"), bvar(1)),
                ),
            ),
        ),
    )
}
/// Build the type of the groupoid path concatenation (alias for trans).
pub fn mk_path_concat_ty() -> Expr {
    mk_eq_class_trans_ty()
}
/// Build the type of the groupoid path inversion (alias for symm).
pub fn mk_path_inv_ty() -> Expr {
    mk_eq_class_symm_ty()
}
/// Build the type of the definitional equality (reflexivity in MLTT).
pub fn mk_def_eq_mltt_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            app(app(app(var("Eq"), bvar(1)), bvar(0)), bvar(0)),
        ),
    )
}
/// Build the type of the general DecidableEq instance.
pub fn mk_decidable_eq_instance_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            pi_named(
                "b",
                bvar(1),
                app(
                    var("Decidable"),
                    app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
                ),
            ),
        ),
    )
}
/// Build the type of the homotopy equivalence axiom.
pub fn mk_homotopy_equiv_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi(
                app(app(var("HomotopyEquiv"), bvar(1)), bvar(0)),
                app(var("Nonempty"), app(app(var("Equiv"), bvar(2)), bvar(1))),
            ),
        ),
    )
}
/// Build the type of the congruence closure axiom.
pub fn mk_cong_closure_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "f",
            pi(bvar(0), bvar(0)),
            pi_named(
                "a",
                bvar(1),
                pi_named(
                    "b",
                    bvar(2),
                    pi(
                        app(app(app(var("Eq"), bvar(3)), bvar(1)), bvar(0)),
                        app(
                            app(app(var("Eq"), bvar(4)), app(bvar(2), bvar(2))),
                            app(bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Subsingleton equality axiom.
pub fn mk_subsingleton_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "a",
            bvar(0),
            pi_named(
                "b",
                bvar(1),
                app(app(app(var("Eq"), bvar(2)), bvar(1)), bvar(0)),
            ),
        ),
    )
}
/// Build the type of the Sigma equality (fst projection equality).
pub fn mk_sigma_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "s",
            app(var("Sigma"), bvar(0)),
            pi_named(
                "t",
                app(var("Sigma"), bvar(1)),
                pi(
                    app(
                        app(app(var("Eq"), app(var("Sigma"), bvar(2))), bvar(1)),
                        bvar(0),
                    ),
                    app(
                        app(app(var("Eq"), bvar(3)), app(var("Sigma.fst"), bvar(2))),
                        app(var("Sigma.fst"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Subtype equality axiom.
pub fn mk_subtype_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_named(
            "s",
            app(var("Subtype"), bvar(0)),
            pi_named(
                "t",
                app(var("Subtype"), bvar(1)),
                pi(
                    app(
                        app(app(var("Eq"), bvar(2)), app(var("Subtype.val"), bvar(1))),
                        app(var("Subtype.val"), bvar(0)),
                    ),
                    app(
                        app(app(var("Eq"), app(var("Subtype"), bvar(3))), bvar(2)),
                        bvar(1),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the function equality pointwise axiom.
pub fn mk_fun_eq_pointwise_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_implicit(
                "f",
                pi(bvar(1), bvar(1)),
                pi_implicit(
                    "g",
                    pi(bvar(2), bvar(2)),
                    pi(
                        app(app(app(var("Eq"), pi(bvar(3), bvar(3))), bvar(1)), bvar(0)),
                        pi_named(
                            "x",
                            bvar(4),
                            app(
                                app(app(var("Eq"), bvar(4)), app(bvar(3), bvar(0))),
                                app(bvar(2), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Either/Sum decidable equality axiom.
pub fn mk_either_decidable_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "β",
            sort(1),
            pi_named(
                "x",
                app(app(var("Sum"), bvar(1)), bvar(0)),
                pi_named(
                    "y",
                    app(app(var("Sum"), bvar(2)), bvar(1)),
                    app(
                        var("Decidable"),
                        app(
                            app(
                                app(var("Eq"), app(app(var("Sum"), bvar(3)), bvar(2))),
                                bvar(1),
                            ),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of the Result (Ok/Err) decidable equality axiom.
pub fn mk_result_decidable_eq_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "ε",
            sort(1),
            pi_named(
                "x",
                app(app(var("Except"), bvar(1)), bvar(0)),
                pi_named(
                    "y",
                    app(app(var("Except"), bvar(2)), bvar(1)),
                    app(
                        var("Decidable"),
                        app(
                            app(
                                app(var("Eq"), app(app(var("Except"), bvar(3)), bvar(2))),
                                bvar(1),
                            ),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the type of `Eq.ndrec` (no-dependency recursor).
pub fn mk_eq_ndrec_ty() -> Expr {
    pi_implicit(
        "α",
        sort(1),
        pi_implicit(
            "a",
            bvar(0),
            pi_named(
                "P",
                pi(bvar(1), sort(1)),
                pi_named(
                    "ha",
                    app(bvar(0), bvar(1)),
                    pi_implicit(
                        "b",
                        bvar(3),
                        pi(
                            app(app(app(var("Eq"), bvar(4)), bvar(3)), bvar(0)),
                            app(bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
