//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BoolReflect, DecidableCounter, Decision, DecisionChain, DecisionTable, EqDecision, FiniteSet,
    FnPred, Interval, LeDecision, NamedDecision, Not,
};

/// A type whose truth value is decidable.
///
/// Analogous to Lean 4's `Decidable` typeclass. Implementors produce a
/// `Decision` when asked.
pub trait Decidable {
    /// The proof type when the proposition holds.
    type Proof;
    /// Decide the proposition, returning the decision.
    fn decide(&self) -> Decision<Self::Proof>;
    /// Shorthand: returns `true` iff the proposition holds.
    fn is_decidably_true(&self) -> bool {
        self.decide().is_true()
    }
}
/// A decidable predicate `P : A → Prop`.
///
/// Given any `a : A`, can compute whether `P(a)` holds.
pub trait DecidablePred<A> {
    /// Decide whether the predicate holds for `a`.
    fn decide_pred(&self, a: &A) -> Decision<()>;
    /// Filter a slice, keeping elements satisfying the predicate.
    fn filter_slice<'a>(&self, xs: &'a [A]) -> Vec<&'a A> {
        xs.iter()
            .filter(|x| self.decide_pred(x).is_true())
            .collect()
    }
    /// Check whether any element of a slice satisfies the predicate.
    fn any_slice(&self, xs: &[A]) -> bool {
        xs.iter().any(|x| self.decide_pred(x).is_true())
    }
    /// Check whether all elements of a slice satisfy the predicate.
    fn all_slice(&self, xs: &[A]) -> bool {
        xs.iter().all(|x| self.decide_pred(x).is_true())
    }
    /// Count elements of a slice satisfying the predicate.
    fn count_slice(&self, xs: &[A]) -> usize {
        xs.iter().filter(|x| self.decide_pred(x).is_true()).count()
    }
}
/// A decidable binary relation `R : A → A → Prop`.
pub trait DecidableRel<A> {
    /// Decide whether `R(a, b)` holds.
    fn decide_rel(&self, a: &A, b: &A) -> Decision<()>;
    /// Check whether `R(a, b)` holds.
    fn holds(&self, a: &A, b: &A) -> bool {
        self.decide_rel(a, b).is_true()
    }
}
/// Decide equality of two `u32` values.
pub fn decide_nat_eq(a: u32, b: u32) -> Decision<()> {
    if a == b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{a} ≠ {b}"))
    }
}
/// Decide `a ≤ b` for `u32` values.
pub fn decide_nat_le(a: u32, b: u32) -> Decision<()> {
    if a <= b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{a} > {b}"))
    }
}
/// Decide `a < b` for `u32` values.
pub fn decide_nat_lt(a: u32, b: u32) -> Decision<()> {
    if a < b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{a} ≥ {b}"))
    }
}
/// Decide equality of two string slices.
pub fn decide_str_eq(a: &str, b: &str) -> Decision<()> {
    if a == b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{a:?} ≠ {b:?}"))
    }
}
/// Decide membership of `x` in a slice `xs`.
pub fn decide_mem<T: PartialEq>(x: &T, xs: &[T]) -> Decision<usize> {
    match xs.iter().position(|y| y == x) {
        Some(idx) => Decision::IsTrue(idx),
        None => Decision::IsFalse("not found".to_string()),
    }
}
/// Decide whether `a` and `b` are equal for any `PartialEq` type.
pub fn decide_eq<T: PartialEq>(a: &T, b: &T) -> Decision<()> {
    if a == b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("not equal".to_string())
    }
}
/// Decide `a ≤ b` for any `PartialOrd` type.
pub fn decide_le<T: PartialOrd>(a: &T, b: &T) -> Decision<()> {
    if a <= b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("not ≤".to_string())
    }
}
/// Decide `a < b` for any `PartialOrd` type.
pub fn decide_lt<T: PartialOrd>(a: &T, b: &T) -> Decision<()> {
    if a < b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("not <".to_string())
    }
}
/// Decide `P ∧ Q`.
pub fn decision_and<P, Q>(dp: Decision<P>, dq: Decision<Q>) -> Decision<(P, Q)> {
    dp.and(dq)
}
/// Decide `P ∨ Q` (left-biased).
pub fn decision_or<P>(dp: Decision<P>, dq: Decision<P>) -> Decision<P> {
    dp.or(dq)
}
/// Decide `¬P`.
pub fn decision_not<P: std::fmt::Debug>(dp: Decision<P>) -> Decision<String> {
    dp.negate()
}
/// Decide `P → Q` given a decision of `P` and a function producing a decision of `Q`.
pub fn decision_implies<P, Q>(dp: Decision<P>, f: impl FnOnce(P) -> Decision<Q>) -> Decision<Q> {
    dp.flat_map(f)
}
/// Decide a conjunction of a list of unit decisions.
pub fn decide_all(ds: impl IntoIterator<Item = Decision<()>>) -> Decision<()> {
    for d in ds {
        if d.is_false() {
            return d;
        }
    }
    Decision::IsTrue(())
}
/// Decide a disjunction of a list of unit decisions.
pub fn decide_any(ds: impl IntoIterator<Item = Decision<()>>) -> Decision<()> {
    let mut last = Decision::IsFalse("no alternatives".to_string());
    for d in ds {
        if d.is_true() {
            return d;
        }
        last = d;
    }
    last
}
/// Decide whether a `u32` lies in the range `[lo, hi)`.
pub fn decide_range(x: u32, lo: u32, hi: u32) -> Decision<()> {
    if x >= lo && x < hi {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{x} not in [{lo}, {hi})"))
    }
}
/// Decide whether a string slice starts with a prefix.
pub fn decide_starts_with(s: &str, prefix: &str) -> Decision<()> {
    if s.starts_with(prefix) {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{s:?} does not start with {prefix:?}"))
    }
}
/// Decide whether a string slice ends with a suffix.
pub fn decide_ends_with(s: &str, suffix: &str) -> Decision<()> {
    if s.ends_with(suffix) {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{s:?} does not end with {suffix:?}"))
    }
}
/// Decide whether a slice is non-empty.
pub fn decide_non_empty<T>(xs: &[T]) -> Decision<()> {
    if !xs.is_empty() {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("slice is empty".to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decision_is_true() {
        let d: Decision<()> = Decision::IsTrue(());
        assert!(d.is_true());
        assert!(!d.is_false());
    }
    #[test]
    fn test_decision_is_false() {
        let d: Decision<()> = Decision::IsFalse("no".to_string());
        assert!(d.is_false());
        assert!(!d.is_true());
    }
    #[test]
    fn test_decision_map() {
        let d: Decision<u32> = Decision::IsTrue(5);
        let d2 = d.map(|x| x * 2);
        match d2 {
            Decision::IsTrue(v) => assert_eq!(v, 10),
            _ => panic!("expected IsTrue variant"),
        };
    }
    #[test]
    fn test_decision_and_both_true() {
        let a: Decision<()> = Decision::IsTrue(());
        let b: Decision<()> = Decision::IsTrue(());
        assert!(a.and(b).is_true());
    }
    #[test]
    fn test_decision_and_one_false() {
        let a: Decision<()> = Decision::IsTrue(());
        let b: Decision<()> = Decision::IsFalse("no".to_string());
        assert!(a.and(b).is_false());
    }
    #[test]
    fn test_decision_or_first_true() {
        let a: Decision<()> = Decision::IsTrue(());
        let b: Decision<()> = Decision::IsFalse("no".to_string());
        assert!(a.or(b).is_true());
    }
    #[test]
    fn test_decide_nat_eq() {
        assert!(decide_nat_eq(5, 5).is_true());
        assert!(decide_nat_eq(5, 6).is_false());
    }
    #[test]
    fn test_decide_nat_le() {
        assert!(decide_nat_le(3, 5).is_true());
        assert!(decide_nat_le(5, 3).is_false());
    }
    #[test]
    fn test_decide_str_eq() {
        assert!(decide_str_eq("hello", "hello").is_true());
        assert!(decide_str_eq("hello", "world").is_false());
    }
    #[test]
    fn test_decide_mem() {
        let xs = vec![1u32, 2, 3];
        assert!(decide_mem(&2, &xs).is_true());
        assert!(decide_mem(&5, &xs).is_false());
    }
    #[test]
    fn test_finite_set_insert() {
        let mut s: FiniteSet<u32> = FiniteSet::new();
        assert!(s.insert(1));
        assert!(!s.insert(1));
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_finite_set_union() {
        let mut a: FiniteSet<u32> = FiniteSet::new();
        a.insert(1);
        a.insert(2);
        let mut b: FiniteSet<u32> = FiniteSet::new();
        b.insert(2);
        b.insert(3);
        let u = a.union(&b);
        assert_eq!(u.len(), 3);
        assert!(u.contains(&3));
    }
    #[test]
    fn test_finite_set_intersection() {
        let a: FiniteSet<u32> = [1, 2, 3].iter().copied().collect();
        let b: FiniteSet<u32> = [2, 3, 4].iter().copied().collect();
        let inter = a.intersection(&b);
        assert_eq!(inter.len(), 2);
        assert!(inter.contains(&2));
        assert!(inter.contains(&3));
    }
    #[test]
    fn test_finite_set_difference() {
        let a: FiniteSet<u32> = [1, 2, 3].iter().copied().collect();
        let b: FiniteSet<u32> = [2, 3].iter().copied().collect();
        let diff = a.difference(&b);
        assert_eq!(diff.len(), 1);
        assert!(diff.contains(&1));
    }
    #[test]
    fn test_finite_set_subset() {
        let a: FiniteSet<u32> = [1, 2].iter().copied().collect();
        let b: FiniteSet<u32> = [1, 2, 3].iter().copied().collect();
        assert!(a.is_subset(&b));
        assert!(!b.is_subset(&a));
    }
    #[test]
    fn test_bool_reflect() {
        let d_true: Decision<()> = Decision::IsTrue(());
        let d_false: Decision<()> = Decision::IsFalse("no".to_string());
        assert!(BoolReflect::from_decision(&d_true).to_bool());
        assert!(!BoolReflect::from_decision(&d_false).to_bool());
    }
    #[test]
    fn test_decision_table() {
        let mut table = DecisionTable::new();
        table.insert("prop_a", Decision::IsTrue(()));
        table.insert("prop_b", Decision::IsFalse("no".to_string()));
        assert!(table
            .lookup("prop_a")
            .expect("lookup should succeed")
            .is_true());
        assert!(table
            .lookup("prop_b")
            .expect("lookup should succeed")
            .is_false());
        assert!(table.lookup("prop_c").is_none());
    }
    #[test]
    fn test_decide_range() {
        assert!(decide_range(5, 0, 10).is_true());
        assert!(decide_range(10, 0, 10).is_false());
        assert!(decide_range(0, 0, 10).is_true());
    }
    #[test]
    fn test_decide_all() {
        let ds: Vec<Decision<()>> = vec![Decision::IsTrue(()), Decision::IsTrue(())];
        assert!(decide_all(ds).is_true());
        let ds2: Vec<Decision<()>> = vec![Decision::IsTrue(()), Decision::IsFalse("n".to_string())];
        assert!(decide_all(ds2).is_false());
    }
    #[test]
    fn test_decide_any() {
        let ds: Vec<Decision<()>> = vec![Decision::IsFalse("n".to_string()), Decision::IsTrue(())];
        assert!(decide_any(ds).is_true());
        let ds2: Vec<Decision<()>> = vec![
            Decision::IsFalse("a".to_string()),
            Decision::IsFalse("b".to_string()),
        ];
        assert!(decide_any(ds2).is_false());
    }
    #[test]
    fn test_fn_pred() {
        let pred = FnPred::new(|x: &u32| *x > 3);
        assert!(pred.decide_pred(&5).is_true());
        assert!(pred.decide_pred(&2).is_false());
        let xs = vec![1u32, 2, 3, 4, 5];
        let filtered = pred.filter_slice(&xs);
        assert_eq!(filtered.len(), 2);
    }
}
/// Decide equality for `Option<T>`.
pub fn decide_option_eq<T: PartialEq>(a: &Option<T>, b: &Option<T>) -> Decision<()> {
    if a == b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("options differ".to_string())
    }
}
/// Decide whether an `Option<T>` is `Some`.
pub fn decide_is_some<T>(a: &Option<T>) -> Decision<()> {
    if a.is_some() {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("is None".to_string())
    }
}
/// Decide whether an `Option<T>` is `None`.
pub fn decide_is_none<T>(a: &Option<T>) -> Decision<()> {
    if a.is_none() {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("is Some".to_string())
    }
}
/// Decide equality for slices of `PartialEq` elements.
pub fn decide_slice_eq<T: PartialEq>(a: &[T], b: &[T]) -> Decision<()> {
    if a == b {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("slices differ (len {} vs {})", a.len(), b.len()))
    }
}
/// Decide whether a slice has the given length.
pub fn decide_len<T>(xs: &[T], expected: usize) -> Decision<()> {
    if xs.len() == expected {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("length {} ≠ {expected}", xs.len()))
    }
}
/// Decide whether an integer is even.
pub fn decide_even(n: u32) -> Decision<()> {
    if n % 2 == 0 {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{n} is odd"))
    }
}
/// Decide whether an integer is odd.
pub fn decide_odd(n: u32) -> Decision<()> {
    if n % 2 == 1 {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{n} is even"))
    }
}
/// Decide whether `n` is divisible by `d`.
///
/// Returns `IsFalse` if `d == 0`.
pub fn decide_divisible(n: u32, d: u32) -> Decision<()> {
    if d == 0 {
        Decision::IsFalse("divisor is zero".to_string())
    } else if n % d == 0 {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{n} not divisible by {d}"))
    }
}
/// Lift a `Result<(), E>` into a `Decision<()>`.
pub fn decision_from_result<E: std::fmt::Display>(r: Result<(), E>) -> Decision<()> {
    match r {
        Ok(()) => Decision::IsTrue(()),
        Err(e) => Decision::IsFalse(e.to_string()),
    }
}
/// Convert a `Decision<()>` into a `Result<(), String>`.
pub fn decision_to_result(d: Decision<()>) -> Result<(), String> {
    match d {
        Decision::IsTrue(()) => Ok(()),
        Decision::IsFalse(msg) => Err(msg),
    }
}
/// Run a series of named decisions and report the results.
///
/// Returns a list of all named decisions, and whether all passed.
pub fn run_decisions(decisions: Vec<NamedDecision>) -> (Vec<NamedDecision>, bool) {
    let all_pass = decisions.iter().all(|d| d.is_true());
    (decisions, all_pass)
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_decide_even_odd() {
        assert!(decide_even(4).is_true());
        assert!(decide_odd(3).is_true());
        assert!(decide_even(3).is_false());
        assert!(decide_odd(4).is_false());
    }
    #[test]
    fn test_decide_divisible() {
        assert!(decide_divisible(12, 4).is_true());
        assert!(decide_divisible(12, 5).is_false());
        assert!(decide_divisible(12, 0).is_false());
    }
    #[test]
    fn test_decide_option() {
        let a: Option<u32> = Some(5);
        let b: Option<u32> = Some(5);
        assert!(decide_option_eq(&a, &b).is_true());
        assert!(decide_is_some(&a).is_true());
        assert!(decide_is_none::<u32>(&None).is_true());
    }
    #[test]
    fn test_decide_slice_eq() {
        let a = vec![1u32, 2, 3];
        let b = vec![1u32, 2, 3];
        let c = vec![1u32, 2];
        assert!(decide_slice_eq(&a, &b).is_true());
        assert!(decide_slice_eq(&a, &c).is_false());
    }
    #[test]
    fn test_named_decision() {
        let nd = NamedDecision::new("prop_a", Decision::IsTrue(()));
        assert!(nd.is_true());
        let s = nd.summary();
        assert!(s.contains("prop_a"));
    }
    #[test]
    fn test_decision_from_result() {
        let r: Result<(), String> = Ok(());
        assert!(decision_from_result(r).is_true());
        let r2: Result<(), String> = Err("err".to_string());
        assert!(decision_from_result(r2).is_false());
    }
    #[test]
    fn test_decision_to_result() {
        assert!(decision_to_result(Decision::IsTrue(())).is_ok());
        assert!(decision_to_result(Decision::IsFalse("no".to_string())).is_err());
    }
}
/// Build the standard Decidable environment declarations.
///
/// Registers `Decidable`, `DecidableEq`, and common instances for
/// `Nat`, `Bool`, `String`, `Unit`, `Char`, and compound types.
pub fn build_decidable_env(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let mut add = |name: &str, ty: Expr| -> Result<(), String> {
        match env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        }) {
            Ok(()) | Err(_) => Ok(()),
        }
    };
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let dec_eq_of = |ty: Expr| -> Expr { app(cst("DecidableEq"), ty) };
    add("Decidable", arr(prop(), type1()))?;
    add("DecidableEq", arr(type1(), type1()))?;
    let is_true_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(Expr::BVar(0), dec_of(Expr::BVar(1)))),
    );
    add("Decidable.isTrue", is_true_ty)?;
    let is_false_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(arr(Expr::BVar(0), cst("False")), dec_of(Expr::BVar(1)))),
    );
    add("Decidable.isFalse", is_false_ty)?;
    add("instDecidableTrue", dec_of(cst("True")))?;
    add("instDecidableFalse", dec_of(cst("False")))?;
    let inst_dec_not_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(dec_of(app(cst("Not"), Expr::BVar(1)))),
        )),
    );
    add("instDecidableNot", inst_dec_not_ty)?;
    let inst_dec_and_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("dp"),
                Box::new(dec_of(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("dq"),
                    Box::new(dec_of(Expr::BVar(1))),
                    Box::new(dec_of(app(app(cst("And"), Expr::BVar(3)), Expr::BVar(2)))),
                )),
            )),
        )),
    );
    add("instDecidableAnd", inst_dec_and_ty)?;
    let inst_dec_or_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("dp"),
                Box::new(dec_of(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("dq"),
                    Box::new(dec_of(Expr::BVar(1))),
                    Box::new(dec_of(app(app(cst("Or"), Expr::BVar(3)), Expr::BVar(2)))),
                )),
            )),
        )),
    );
    add("instDecidableOr", inst_dec_or_ty)?;
    add("instDecidableEqNat", dec_eq_of(cst("Nat")))?;
    add("instDecidableEqBool", dec_eq_of(cst("Bool")))?;
    add("instDecidableEqString", dec_eq_of(cst("String")))?;
    add("instDecidableEqUnit", dec_eq_of(cst("Unit")))?;
    add("instDecidableEqChar", dec_eq_of(cst("Char")))?;
    add("instDecidableEqInt", dec_eq_of(cst("Int")))?;
    let inst_dec_eq_opt_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_eq_of(Expr::BVar(0))),
            Box::new(dec_eq_of(app(cst("Option"), Expr::BVar(1)))),
        )),
    );
    add("instDecidableEqOption", inst_dec_eq_opt_ty)?;
    let inst_dec_eq_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_eq_of(Expr::BVar(0))),
            Box::new(dec_eq_of(app(cst("List"), Expr::BVar(1)))),
        )),
    );
    add("instDecidableEqList", inst_dec_eq_list_ty)?;
    Ok(())
}
/// Decide membership in a `FiniteSet<T>`.
pub fn decide_finite_set_mem<T: PartialEq>(x: &T, set: &FiniteSet<T>) -> Decision<()> {
    if set.contains(x) {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("not in set".to_string())
    }
}
/// Decide whether a `FiniteSet` is a subset of another.
pub fn decide_subset<T: PartialEq>(a: &FiniteSet<T>, b: &FiniteSet<T>) -> Decision<()> {
    if a.is_subset(b) {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("not a subset".to_string())
    }
}
/// Decide whether an interval `[lo, hi]` is non-empty.
pub fn decide_interval_non_empty(iv: &Interval) -> Decision<()> {
    if !iv.is_empty() {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse("interval is empty".to_string())
    }
}
/// Decide whether two intervals overlap.
pub fn decide_intervals_overlap(a: &Interval, b: &Interval) -> Decision<()> {
    if !a.intersect(b).is_empty() {
        Decision::IsTrue(())
    } else {
        Decision::IsFalse(format!("{} and {} do not overlap", a, b))
    }
}
#[cfg(test)]
mod decidable_extra_tests {
    use super::*;
    #[test]
    fn test_decidable_counter_record() {
        let mut counter = DecidableCounter::new();
        counter.record(&Decision::IsTrue(()));
        counter.record(&Decision::IsTrue(()));
        counter.record(&Decision::IsFalse("no".to_string()));
        assert_eq!(counter.true_count, 2);
        assert_eq!(counter.false_count, 1);
        assert_eq!(counter.total(), 3);
    }
    #[test]
    fn test_decidable_counter_true_rate() {
        let mut counter = DecidableCounter::new();
        counter.record(&Decision::IsTrue(()));
        counter.record(&Decision::IsTrue(()));
        assert!((counter.true_rate() - 100.0).abs() < 0.01);
    }
    #[test]
    fn test_decidable_counter_all_true() {
        let mut counter = DecidableCounter::new();
        counter.record(&Decision::IsTrue(()));
        assert!(counter.all_true());
        counter.record(&Decision::IsFalse("n".to_string()));
        assert!(!counter.all_true());
    }
    #[test]
    fn test_decidable_counter_reset() {
        let mut counter = DecidableCounter::new();
        counter.record(&Decision::IsTrue(()));
        counter.reset();
        assert_eq!(counter.total(), 0);
    }
    #[test]
    fn test_decision_chain_all_passed() {
        let chain = DecisionChain::new()
            .step("a", Decision::IsTrue(()))
            .step("b", Decision::IsTrue(()));
        assert!(chain.all_passed());
        assert_eq!(chain.passed_count(), 2);
        assert_eq!(chain.failed_count(), 0);
    }
    #[test]
    fn test_decision_chain_first_failure() {
        let chain = DecisionChain::new()
            .step("a", Decision::IsTrue(()))
            .step("b", Decision::IsFalse("no".to_string()))
            .step("c", Decision::IsTrue(()));
        assert!(!chain.all_passed());
        assert_eq!(chain.first_failure(), Some("b"));
    }
    #[test]
    fn test_decision_chain_empty() {
        let chain = DecisionChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);
    }
    #[test]
    fn test_interval_contains() {
        let iv = Interval::new(3, 7);
        assert!(iv.contains(3));
        assert!(iv.contains(7));
        assert!(iv.contains(5));
        assert!(!iv.contains(2));
        assert!(!iv.contains(8));
    }
    #[test]
    fn test_interval_is_empty() {
        let iv = Interval::new(5, 3);
        assert!(iv.is_empty());
        assert_eq!(iv.len(), 0);
    }
    #[test]
    fn test_interval_len() {
        let iv = Interval::new(2, 5);
        assert_eq!(iv.len(), 4);
    }
    #[test]
    fn test_interval_intersect() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 8);
        let inter = a.intersect(&b);
        assert_eq!(inter.lo, 3);
        assert_eq!(inter.hi, 5);
    }
    #[test]
    fn test_interval_union() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 8);
        let u = a.union(&b);
        assert_eq!(u.lo, 1);
        assert_eq!(u.hi, 8);
    }
    #[test]
    fn test_interval_decide_contains() {
        let iv = Interval::new(0, 10);
        assert!(iv.decide_contains(5).is_true());
        assert!(iv.decide_contains(11).is_false());
    }
    #[test]
    fn test_interval_display() {
        let iv = Interval::new(-1, 3);
        assert_eq!(format!("{}", iv), "[-1, 3]");
    }
    #[test]
    fn test_eq_decision_true() {
        let d = EqDecision::new(42u32, 42u32);
        assert!(d.decide().is_true());
        assert!(d.is_decidably_true());
    }
    #[test]
    fn test_eq_decision_false() {
        let d = EqDecision::new(1u32, 2u32);
        assert!(d.decide().is_false());
    }
    #[test]
    fn test_le_decision_true() {
        let d = LeDecision::new(3u32, 5u32);
        assert!(d.decide().is_true());
    }
    #[test]
    fn test_le_decision_equal() {
        let d = LeDecision::new(5u32, 5u32);
        assert!(d.decide().is_true());
    }
    #[test]
    fn test_le_decision_false() {
        let d = LeDecision::new(6u32, 5u32);
        assert!(d.decide().is_false());
    }
    #[test]
    fn test_decide_finite_set_mem() {
        let mut s: FiniteSet<u32> = FiniteSet::new();
        s.insert(7);
        assert!(decide_finite_set_mem(&7u32, &s).is_true());
        assert!(decide_finite_set_mem(&8u32, &s).is_false());
    }
    #[test]
    fn test_decide_subset() {
        let a: FiniteSet<u32> = [1, 2].iter().copied().collect();
        let b: FiniteSet<u32> = [1, 2, 3].iter().copied().collect();
        assert!(decide_subset(&a, &b).is_true());
        assert!(decide_subset(&b, &a).is_false());
    }
    #[test]
    fn test_decide_interval_non_empty() {
        let iv1 = Interval::new(1, 5);
        let iv2 = Interval::new(5, 3);
        assert!(decide_interval_non_empty(&iv1).is_true());
        assert!(decide_interval_non_empty(&iv2).is_false());
    }
    #[test]
    fn test_decide_intervals_overlap() {
        let a = Interval::new(1, 5);
        let b = Interval::new(4, 9);
        let c = Interval::new(6, 9);
        assert!(decide_intervals_overlap(&a, &b).is_true());
        assert!(decide_intervals_overlap(&a, &c).is_false());
    }
}
pub fn dcs_ext_decidable_typeclass(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let decide_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(cst("Bool")),
        )),
    );
    add("Decidable.decide", decide_ty)?;
    let decide_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(app(
                app(
                    cst("Iff"),
                    app(
                        app(cst("Eq"), app(cst("Decidable.decide"), Expr::BVar(1))),
                        cst("Bool.true"),
                    ),
                ),
                Expr::BVar(1),
            )),
        )),
    );
    add("Decidable.decide_eq_true_iff", decide_iff_ty)?;
    let by_contra_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(arr(
                arr(arr(Expr::BVar(1), cst("False")), cst("False")),
                Expr::BVar(1),
            )),
        )),
    );
    add("Decidable.byContradiction", by_contra_ty)?;
    let em_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(app(
                app(cst("Or"), Expr::BVar(1)),
                arr(Expr::BVar(1), cst("False")),
            )),
        )),
    );
    add("Decidable.em", em_ty)?;
    let _ = type1();
    Ok(())
}
pub fn dcs_ext_logical_connective_closure(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let dec_implies_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("hp"),
                Box::new(dec_of(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("hq"),
                    Box::new(dec_of(Expr::BVar(1))),
                    Box::new(dec_of(arr(Expr::BVar(3), Expr::BVar(2)))),
                )),
            )),
        )),
    );
    add("instDecidableImplies", dec_implies_ty)?;
    let dec_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("hp"),
                Box::new(dec_of(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("hq"),
                    Box::new(dec_of(Expr::BVar(1))),
                    Box::new(dec_of(app(app(cst("Iff"), Expr::BVar(3)), Expr::BVar(2)))),
                )),
            )),
        )),
    );
    add("instDecidableIff", dec_iff_ty)?;
    let dec_and_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(arr(
                dec_of(Expr::BVar(1)),
                arr(
                    dec_of(Expr::BVar(1)),
                    dec_of(app(app(cst("And"), Expr::BVar(3)), Expr::BVar(2))),
                ),
            )),
        )),
    );
    add("Decidable.and_of_dec", dec_and_ty)?;
    let dec_or_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(arr(
                dec_of(Expr::BVar(1)),
                arr(
                    dec_of(Expr::BVar(1)),
                    dec_of(app(app(cst("Or"), Expr::BVar(3)), Expr::BVar(2))),
                ),
            )),
        )),
    );
    add("Decidable.or_of_dec", dec_or_ty)?;
    let dec_not_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(
            dec_of(Expr::BVar(0)),
            dec_of(arr(Expr::BVar(1), cst("False"))),
        )),
    );
    add("Decidable.not_of_dec", dec_not_ty)?;
    let _ = type1();
    Ok(())
}
pub fn dcs_ext_decidable_eq_basic(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    add("instDecidableEqNat", app(cst("DecidableEq"), cst("Nat")))?;
    add("instDecidableEqInt", app(cst("DecidableEq"), cst("Int")))?;
    add("instDecidableEqBool", app(cst("DecidableEq"), cst("Bool")))?;
    add("instDecidableEqChar", app(cst("DecidableEq"), cst("Char")))?;
    add(
        "instDecidableEqString",
        app(cst("DecidableEq"), cst("String")),
    )?;
    add("instDecidableEqUnit", app(cst("DecidableEq"), cst("Unit")))?;
    let nat_dec_eq_ty = arr(
        cst("Nat"),
        arr(
            cst("Nat"),
            dec_of(app(app(cst("Eq"), Expr::BVar(1)), Expr::BVar(0))),
        ),
    );
    add("Nat.decEq", nat_dec_eq_ty)?;
    let int_dec_eq_ty = arr(
        cst("Int"),
        arr(
            cst("Int"),
            dec_of(app(app(cst("Eq"), Expr::BVar(1)), Expr::BVar(0))),
        ),
    );
    add("Int.decEq", int_dec_eq_ty)?;
    let bool_dec_eq_ty = arr(
        cst("Bool"),
        arr(
            cst("Bool"),
            dec_of(app(app(cst("Eq"), Expr::BVar(1)), Expr::BVar(0))),
        ),
    );
    add("Bool.decEq", bool_dec_eq_ty)?;
    let _ = (type1(), Bi::Default, Name::Anonymous);
    Ok(())
}
pub fn dcs_ext_decidable_eq_compound(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let _arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let dec_eq_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(app(cst("DecidableEq"), app(cst("List"), Expr::BVar(1)))),
        )),
    );
    add("instDecidableEqList", dec_eq_list_ty)?;
    let dec_eq_opt_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(app(cst("DecidableEq"), app(cst("Option"), Expr::BVar(1)))),
        )),
    );
    add("instDecidableEqOption", dec_eq_opt_ty)?;
    let dec_eq_prod_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("ha"),
                Box::new(app(cst("DecidableEq"), Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("hb"),
                    Box::new(app(cst("DecidableEq"), Expr::BVar(1))),
                    Box::new(app(
                        cst("DecidableEq"),
                        app(app(cst("Prod"), Expr::BVar(3)), Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    add("instDecidableEqProd", dec_eq_prod_ty)?;
    let dec_eq_sum_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("ha"),
                Box::new(app(cst("DecidableEq"), Expr::BVar(1))),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("hb"),
                    Box::new(app(cst("DecidableEq"), Expr::BVar(1))),
                    Box::new(app(
                        cst("DecidableEq"),
                        app(app(cst("Sum"), Expr::BVar(3)), Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    add("instDecidableEqSum", dec_eq_sum_ty)?;
    Ok(())
}
pub fn dcs_ext_linear_ordering(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    add("DecidableLinearOrder", arr(type1(), type1()))?;
    add(
        "instDecidableLinearOrderNat",
        app(cst("DecidableLinearOrder"), cst("Nat")),
    )?;
    add(
        "instDecidableLinearOrderInt",
        app(cst("DecidableLinearOrder"), cst("Int")),
    )?;
    let nat_dec_lt_ty = arr(
        cst("Nat"),
        arr(
            cst("Nat"),
            dec_of(app(app(cst("Nat.lt"), Expr::BVar(1)), Expr::BVar(0))),
        ),
    );
    add("Nat.decLt", nat_dec_lt_ty)?;
    let nat_dec_le_ty = arr(
        cst("Nat"),
        arr(
            cst("Nat"),
            dec_of(app(app(cst("Nat.le"), Expr::BVar(1)), Expr::BVar(0))),
        ),
    );
    add("Nat.decLe", nat_dec_le_ty)?;
    let compare_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableLinearOrder"), Expr::BVar(0))),
            Box::new(arr(Expr::BVar(1), arr(Expr::BVar(2), cst("Ordering")))),
        )),
    );
    add("Decidable.compare", compare_ty)?;
    let min_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableLinearOrder"), Expr::BVar(0))),
            Box::new(arr(Expr::BVar(1), arr(Expr::BVar(2), Expr::BVar(3)))),
        )),
    );
    add("Decidable.min", min_ty)?;
    let max_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableLinearOrder"), Expr::BVar(0))),
            Box::new(arr(Expr::BVar(1), arr(Expr::BVar(2), Expr::BVar(3)))),
        )),
    );
    add("Decidable.max", max_ty)?;
    Ok(())
}
pub fn dcs_ext_bounded_quantifiers(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let bool_ty = || -> Expr { cst("Bool") };
    let forall_finset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(app(cst("DecidableEq"), Expr::BVar(0))),
            Box::new(arr(
                arr(Expr::BVar(1), prop()),
                arr(
                    app(cst("Finset"), Expr::BVar(2)),
                    dec_of(Expr::Pi(
                        Bi::Default,
                        Name::str("x"),
                        Box::new(Expr::BVar(2)),
                        Box::new(arr(
                            app(app(cst("Finset.mem"), Expr::BVar(0)), Expr::BVar(2)),
                            app(Expr::BVar(2), Expr::BVar(0)),
                        )),
                    )),
                ),
            )),
        )),
    );
    add("Decidable.forallFinset", forall_finset_ty)?;
    let exists_finset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), prop()),
            arr(
                app(cst("Finset"), Expr::BVar(1)),
                dec_of(app(
                    app(cst("Exists"), Expr::BVar(1)),
                    arr(
                        app(cst("Finset.mem"), Expr::BVar(0)),
                        app(Expr::BVar(1), Expr::BVar(0)),
                    ),
                )),
            ),
        )),
    );
    add("Decidable.existsFinset", exists_finset_ty)?;
    let forall_range_ty = arr(
        cst("Nat"),
        arr(
            arr(cst("Nat"), bool_ty()),
            dec_of(Expr::Pi(
                Bi::Default,
                Name::str("i"),
                Box::new(cst("Nat")),
                Box::new(arr(
                    app(app(cst("Nat.lt"), Expr::BVar(0)), Expr::BVar(2)),
                    app(
                        app(cst("Eq"), app(Expr::BVar(1), Expr::BVar(0))),
                        cst("Bool.true"),
                    ),
                )),
            )),
        ),
    );
    add("Decidable.forallRange", forall_range_ty)?;
    let exists_range_ty = arr(
        cst("Nat"),
        arr(
            arr(cst("Nat"), bool_ty()),
            dec_of(app(
                app(cst("Exists"), cst("Nat")),
                app(
                    app(
                        cst("And"),
                        app(app(cst("Nat.lt"), Expr::BVar(0)), Expr::BVar(2)),
                    ),
                    app(
                        app(cst("Eq"), app(Expr::BVar(1), Expr::BVar(0))),
                        cst("Bool.true"),
                    ),
                ),
            )),
        ),
    );
    add("Decidable.existsRange", exists_range_ty)?;
    Ok(())
}
pub fn dcs_ext_lem_boolean_reflection(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let bool_ty = || -> Expr { cst("Bool") };
    let lem_dec_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(
            dec_of(Expr::BVar(0)),
            app(
                app(cst("Or"), Expr::BVar(1)),
                arr(Expr::BVar(1), cst("False")),
            ),
        )),
    );
    add("LEM_from_Decidable", lem_dec_ty)?;
    let reflect_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(arr(
                bool_ty(),
                app(
                    app(
                        cst("Iff"),
                        app(app(cst("Eq"), Expr::BVar(0)), cst("Bool.true")),
                    ),
                    Expr::BVar(2),
                ),
            )),
        )),
    );
    add("Bool.reflect", reflect_ty)?;
    let decide_reflect_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(dec_of(Expr::BVar(0))),
            Box::new(app(
                app(
                    cst("Iff"),
                    app(
                        app(cst("Eq"), app(cst("Decidable.decide"), Expr::BVar(1))),
                        cst("Bool.true"),
                    ),
                ),
                Expr::BVar(1),
            )),
        )),
    );
    add("Bool.decide_reflect", decide_reflect_ty)?;
    let prop_decidable_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(dec_of(Expr::BVar(0))),
    );
    add("Classical.propDecidable", prop_decidable_ty)?;
    let classical_em_ty = Expr::Pi(
        Bi::Default,
        Name::str("p"),
        Box::new(prop()),
        Box::new(app(
            app(cst("Or"), Expr::BVar(0)),
            arr(Expr::BVar(0), cst("False")),
        )),
    );
    add("Classical.em", classical_em_ty)?;
    let _ = type1();
    Ok(())
}
pub fn dcs_ext_semi_decidability(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    add("SemiDecidable", arr(prop(), type1()))?;
    let to_semi_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(
            dec_of(Expr::BVar(0)),
            app(cst("SemiDecidable"), Expr::BVar(1)),
        )),
    );
    add("Decidable.toSemiDecidable", to_semi_ty)?;
    let semi_and_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(arr(
                app(cst("SemiDecidable"), Expr::BVar(1)),
                arr(
                    app(cst("SemiDecidable"), Expr::BVar(1)),
                    app(
                        cst("SemiDecidable"),
                        app(app(cst("And"), Expr::BVar(3)), Expr::BVar(2)),
                    ),
                ),
            )),
        )),
    );
    add("SemiDecidable.and", semi_and_ty)?;
    let semi_or_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("q"),
            Box::new(prop()),
            Box::new(arr(
                app(cst("SemiDecidable"), Expr::BVar(1)),
                arr(
                    app(cst("SemiDecidable"), Expr::BVar(1)),
                    app(
                        cst("SemiDecidable"),
                        app(app(cst("Or"), Expr::BVar(3)), Expr::BVar(2)),
                    ),
                ),
            )),
        )),
    );
    add("SemiDecidable.or", semi_or_ty)?;
    let semi_nn_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(prop()),
        Box::new(arr(
            app(cst("SemiDecidable"), Expr::BVar(0)),
            app(
                cst("SemiDecidable"),
                arr(arr(Expr::BVar(1), cst("False")), cst("False")),
            ),
        )),
    );
    add("SemiDecidable.not_not", semi_nn_ty)?;
    Ok(())
}
pub fn dcs_ext_undecidability_halting(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let halting_ty = arr(
        arr(cst("Nat"), app(cst("Option"), cst("Nat"))),
        arr(cst("Nat"), prop()),
    );
    add("HaltingProblem", halting_ty)?;
    add("HaltingProblem.undecidable", prop())?;
    add("Rice.theorem", prop())?;
    add("Decidable.computability_implies_decidable", prop())?;
    let semi_dec_halting_ty = Expr::Pi(
        Bi::Default,
        Name::str("p"),
        Box::new(arr(cst("Nat"), app(cst("Option"), cst("Nat")))),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("n"),
            Box::new(cst("Nat")),
            Box::new(app(
                cst("SemiDecidable"),
                app(app(cst("HaltingProblem"), Expr::BVar(1)), Expr::BVar(0)),
            )),
        )),
    );
    add("HaltingProblem.semi_decidable", semi_dec_halting_ty)?;
    add("Rice.corollary_extensional", prop())?;
    let _ = (type1(), dec_of(cst("P")));
    Ok(())
}
pub fn dcs_ext_presburger_arithmetic(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    add("PresburgerFormula", type1())?;
    let pres_decide_ty = arr(cst("PresburgerFormula"), cst("Bool"));
    add("Presburger.decide", pres_decide_ty)?;
    let pres_sound_ty = arr(
        cst("PresburgerFormula"),
        arr(
            app(
                app(cst("Eq"), app(cst("Presburger.decide"), Expr::BVar(0))),
                cst("Bool.true"),
            ),
            app(cst("PresburgerFormula.holds"), Expr::BVar(1)),
        ),
    );
    add("Presburger.decide_sound", pres_sound_ty)?;
    let pres_complete_ty = arr(
        cst("PresburgerFormula"),
        arr(
            app(cst("PresburgerFormula.holds"), Expr::BVar(0)),
            app(
                app(cst("Eq"), app(cst("Presburger.decide"), Expr::BVar(1))),
                cst("Bool.true"),
            ),
        ),
    );
    add("Presburger.decide_complete", pres_complete_ty)?;
    let pres_decidable_ty = arr(
        cst("PresburgerFormula"),
        dec_of(app(cst("PresburgerFormula.holds"), Expr::BVar(0))),
    );
    add("Presburger.decidable", pres_decidable_ty)?;
    let add_comm_dec_ty = arr(
        cst("Nat"),
        arr(
            cst("Nat"),
            dec_of(app(
                app(
                    cst("Eq"),
                    app(app(cst("Nat.add"), Expr::BVar(1)), Expr::BVar(0)),
                ),
                app(app(cst("Nat.add"), Expr::BVar(0)), Expr::BVar(1)),
            )),
        ),
    );
    add("Nat.add_comm_decidable", add_comm_dec_ty)?;
    let _ = prop();
    Ok(())
}
pub fn dcs_ext_dpll_procedure(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let bool_ty = || -> Expr { cst("Bool") };
    add("CNFFormula", type1())?;
    let dpll_solve_ty = arr(cst("CNFFormula"), bool_ty());
    add("DPLL.solve", dpll_solve_ty)?;
    let dpll_sound_ty = arr(
        cst("CNFFormula"),
        arr(
            app(
                app(cst("Eq"), app(cst("DPLL.solve"), Expr::BVar(0))),
                cst("Bool.true"),
            ),
            app(cst("CNFFormula.satisfiable"), Expr::BVar(1)),
        ),
    );
    add("DPLL.sound", dpll_sound_ty)?;
    let dpll_complete_ty = arr(
        cst("CNFFormula"),
        arr(
            app(cst("CNFFormula.satisfiable"), Expr::BVar(0)),
            app(
                app(cst("Eq"), app(cst("DPLL.solve"), Expr::BVar(1))),
                cst("Bool.true"),
            ),
        ),
    );
    add("DPLL.complete", dpll_complete_ty)?;
    let dpll_dec_ty = arr(
        cst("CNFFormula"),
        dec_of(app(cst("CNFFormula.satisfiable"), Expr::BVar(0))),
    );
    add("DPLL.decidable_sat", dpll_dec_ty)?;
    let tautology_ty = arr(cst("CNFFormula"), cst("Prop"));
    add("Propositional.Tautology", tautology_ty)?;
    let dec_taut_ty = arr(
        cst("CNFFormula"),
        dec_of(app(cst("Propositional.Tautology"), Expr::BVar(0))),
    );
    add("Propositional.decidable_tautology", dec_taut_ty)?;
    Ok(())
}
pub fn dcs_ext_constructive_markov(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use super::functions::*;
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    use std::fmt;
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let dec_of = |p: Expr| -> Expr { app(cst("Decidable"), p) };
    let markov_ty = Expr::Pi(
        Bi::Default,
        Name::str("p"),
        Box::new(arr(cst("Nat"), cst("Bool"))),
        Box::new(arr(
            arr(
                arr(
                    app(
                        app(cst("Exists"), cst("Nat")),
                        app(
                            app(cst("Eq"), app(Expr::BVar(1), Expr::BVar(0))),
                            cst("Bool.true"),
                        ),
                    ),
                    cst("False"),
                ),
                cst("False"),
            ),
            app(
                app(cst("Exists"), cst("Nat")),
                app(
                    app(cst("Eq"), app(Expr::BVar(1), Expr::BVar(0))),
                    cst("Bool.true"),
                ),
            ),
        )),
    );
    add("Markov.principle", markov_ty)?;
    add("ChurchThesis", arr(arr(cst("Nat"), cst("Nat")), prop()))?;
    let church_ty = Expr::Pi(
        Bi::Default,
        Name::str("f"),
        Box::new(arr(cst("Nat"), cst("Nat"))),
        Box::new(app(
            app(cst("Exists"), cst("Nat")),
            app(app(cst("Computes"), Expr::BVar(0)), Expr::BVar(1)),
        )),
    );
    add("Church.thesis", church_ty)?;
    let cons_dec_finite_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            app(cst("Finset"), Expr::BVar(0)),
            arr(
                arr(Expr::BVar(1), prop()),
                dec_of(Expr::Pi(
                    Bi::Default,
                    Name::str("x"),
                    Box::new(Expr::BVar(1)),
                    Box::new(arr(
                        app(app(cst("Finset.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        app(Expr::BVar(2), Expr::BVar(0)),
                    )),
                )),
            ),
        )),
    );
    add("Constructive.decidable_finite", cons_dec_finite_ty)?;
    let witness_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("p"),
        Box::new(arr(cst("Nat"), prop())),
        Box::new(arr(
            app(
                app(cst("Exists"), cst("Nat")),
                app(Expr::BVar(1), Expr::BVar(0)),
            ),
            app(
                app(cst("Sigma"), cst("Nat")),
                app(Expr::BVar(1), Expr::BVar(0)),
            ),
        )),
    );
    add("Constructive.witness_extraction", witness_ty)?;
    Ok(())
}
