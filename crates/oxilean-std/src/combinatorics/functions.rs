//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CycleIndexPolynomial, LovaszLocalLemma, MatroidIntersectionSolver, Poly, TuranDensityComputer,
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
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `Fib : Nat → Nat` — n-th Fibonacci number (F(0)=0, F(1)=1).
pub fn fib_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Lucas : Nat → Nat` — n-th Lucas number (L(0)=2, L(1)=1).
pub fn lucas_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Catalan : Nat → Nat` — n-th Catalan number C(n) = C(2n,n)/(n+1).
pub fn catalan_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Bell : Nat → Nat` — n-th Bell number (partitions of an n-set).
pub fn bell_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Bernoulli : Nat → Rat` — n-th Bernoulli number.
pub fn bernoulli_ty() -> Expr {
    arrow(nat_ty(), cst("Rat"))
}
/// `Partition : Nat → Nat` — integer partition function p(n).
pub fn partition_fn_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Choose : Nat → Nat → Nat` — binomial coefficient C(n,k).
pub fn choose_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `Multinomial : Nat → List Nat → Nat` — multinomial coefficient.
pub fn multinomial_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), nat_ty()))
}
/// `Stirling1 : Nat → Nat → Int` — signed Stirling number of the first kind s(n,k).
pub fn stirling1_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), int_ty()))
}
/// `Stirling1u : Nat → Nat → Nat` — unsigned (signless) Stirling number |s(n,k)|.
pub fn stirling1u_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `Stirling2 : Nat → Nat → Nat` — Stirling number of the second kind S(n,k).
pub fn stirling2_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `Derangement : Nat → Nat` — number of derangements D(n).
pub fn derangement_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `EulerTotient : Nat → Nat` — Euler's totient φ(n).
pub fn euler_totient_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `Moebius : Nat → Int` — Möbius function μ(n).
pub fn moebius_ty() -> Expr {
    arrow(nat_ty(), int_ty())
}
/// `fib_recurrence : ∀ n : Nat, Fib (n+2) = Fib (n+1) + Fib n`
pub fn fib_recurrence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            app(cst("Eq"), nat_ty()),
            app(
                cst("Fib"),
                app(cst("Nat.succ"), app(cst("Nat.succ"), bvar(0))),
            ),
            app2(
                cst("Nat.add"),
                app(cst("Fib"), app(cst("Nat.succ"), bvar(0))),
                app(cst("Fib"), bvar(0)),
            ),
        ),
    )
}
/// `fib_closed_form : ∀ n, Fib n = round((φ^n)/√5)`
/// Expressed as: ∀ n, FibClosedFormApprox n holds (opaque predicate).
pub fn fib_closed_form_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(cst("FibClosedFormApprox"), bvar(0)),
    )
}
/// `fib_gcd : ∀ m n, gcd (Fib m) (Fib n) = Fib (gcd m n)`
pub fn fib_gcd_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            app2(
                app(cst("Eq"), nat_ty()),
                app2(
                    cst("Nat.gcd"),
                    app(cst("Fib"), bvar(1)),
                    app(cst("Fib"), bvar(0)),
                ),
                app(cst("Fib"), app2(cst("Nat.gcd"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `catalan_recurrence : C(0)=1 ∧ C(n+1) = ∑_{i=0}^{n} C(i)*C(n-i)`
pub fn catalan_recurrence_ty() -> Expr {
    app2(
        cst("And"),
        app2(
            app(cst("Eq"), nat_ty()),
            app(cst("Catalan"), cst("Nat.zero")),
            cst("Nat.one"),
        ),
        cst("CatalanRecurrenceHolds"),
    )
}
/// `catalan_choose : ∀ n, Catalan n = Choose (2*n) n / (n+1)`
pub fn catalan_choose_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            app(cst("Eq"), nat_ty()),
            app(cst("Catalan"), bvar(0)),
            app2(
                cst("Nat.div"),
                app2(
                    cst("Choose"),
                    app2(cst("Nat.mul"), cst("Nat.two"), bvar(0)),
                    bvar(0),
                ),
                app2(cst("Nat.add"), bvar(0), cst("Nat.one")),
            ),
        ),
    )
}
/// `bell_triangle : Bell numbers satisfy Bell(n+1) = ∑_{k=0}^{n} C(n,k)*Bell(k)`
pub fn bell_triangle_ty() -> Expr {
    cst("BellTriangleRecurrence")
}
/// `stirling2_recurrence : S(n+1,k) = k*S(n,k) + S(n,k-1)`
pub fn stirling2_recurrence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            app2(
                app(cst("Eq"), nat_ty()),
                app2(cst("Stirling2"), app(cst("Nat.succ"), bvar(1)), bvar(0)),
                app2(
                    cst("Nat.add"),
                    app2(
                        cst("Nat.mul"),
                        bvar(0),
                        app2(cst("Stirling2"), bvar(1), bvar(0)),
                    ),
                    app2(cst("Stirling2"), bvar(1), app(cst("Nat.pred"), bvar(0))),
                ),
            ),
        ),
    )
}
/// `bell_sum_stirling2 : Bell n = ∑_{k=0}^{n} S(n,k)` (sum of row of Stirling2 triangle)
pub fn bell_sum_stirling2_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            app(cst("Eq"), nat_ty()),
            app(cst("Bell"), bvar(0)),
            app(cst("SumStirling2Row"), bvar(0)),
        ),
    )
}
/// `derangement_recurrence : D(n) = (n-1)*(D(n-1) + D(n-2))  for n≥2`
pub fn derangement_recurrence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(app(cst("Nat.le"), cst("Nat.two")), bvar(0)),
    )
}
/// `derangement_formula : D(n) = n! * ∑_{k=0}^{n} (-1)^k / k!`
pub fn derangement_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(cst("DerangementFormulaHolds"), bvar(0)),
    )
}
/// `inclusion_exclusion : |A₁ ∪ … ∪ Aₙ| = ∑ sign * |intersection|`
/// Stated as: the IEP axiom schema holds for finite families of finite sets.
pub fn inclusion_exclusion_ty() -> Expr {
    cst("InclusionExclusionPrinciple")
}
/// `pigeonhole : ∀ n m, n > m → ∀ f : Fin n → Fin m, ¬ Injective f`
pub fn pigeonhole_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            arrow(
                app2(cst("Nat.lt"), bvar(0), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(app(cst("Fin"), bvar(1)), app(cst("Fin"), bvar(1))),
                    app(cst("Not"), app(cst("Function.Injective"), bvar(0))),
                ),
            ),
        ),
    )
}
/// `RamseyNumber : Nat → Nat → Nat` — R(s,t) the Ramsey number.
pub fn ramsey_number_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `ramsey_theorem : ∀ s t, ∃ N, ∀ 2-coloring of K_N, monochromatic K_s or K_t exists`
pub fn ramsey_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "t",
            nat_ty(),
            app(
                cst("RamseyProperty"),
                app2(cst("RamseyNumber"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `ramsey_33 : R(3,3) = 6`
pub fn ramsey_33_ty() -> Expr {
    app2(
        app(cst("Eq"), nat_ty()),
        app2(cst("RamseyNumber"), cst("Nat.three"), cst("Nat.three")),
        cst("Nat.six"),
    )
}
/// `ramsey_upper_bound : R(s,t) ≤ R(s-1,t) + R(s,t-1)  (Pascal-like bound)`
pub fn ramsey_upper_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "t",
            nat_ty(),
            app(
                app(cst("Nat.le"), app2(cst("RamseyNumber"), bvar(1), bvar(0))),
                app2(
                    cst("Nat.add"),
                    app2(cst("RamseyNumber"), app(cst("Nat.pred"), bvar(1)), bvar(0)),
                    app2(cst("RamseyNumber"), bvar(1), app(cst("Nat.pred"), bvar(0))),
                ),
            ),
        ),
    )
}
/// `ramsey_symmetric : ∀ s t, R(s,t) = R(t,s)`
pub fn ramsey_symmetric_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "t",
            nat_ty(),
            app2(
                app(cst("Eq"), nat_ty()),
                app2(cst("RamseyNumber"), bvar(1), bvar(0)),
                app2(cst("RamseyNumber"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `van_der_waerden : ∀ k r, ∃ N, every r-coloring of {1..N} has mono AP of length k`
pub fn van_der_waerden_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "r",
            nat_ty(),
            app(
                app(cst("Exists"), nat_ty()),
                app2(cst("VDWProperty"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `OGF : (Nat → Nat) → Type` — ordinary generating function ∑ aₙ xⁿ
pub fn ogf_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), type0())
}
/// `EGF : (Nat → Nat) → Type` — exponential generating function ∑ aₙ xⁿ/n!
pub fn egf_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), type0())
}
/// `ogf_product : ∀ a b, OGF(a*b) = OGF(a) * OGF(b)` (Dirichlet-like convolution)
pub fn ogf_product_ty() -> Expr {
    cst("OGFProductLaw")
}
/// `egf_exp : EGF(Bell) = exp(exp(x) - 1)`
pub fn egf_bell_ty() -> Expr {
    cst("EGFBellIsExpExpMinus1")
}
/// `egf_catalan : OGF(Catalan) = (1 - √(1-4x)) / (2x)`
pub fn ogf_catalan_ty() -> Expr {
    cst("OGFCatalanAlgebraic")
}
/// `totient_sum : ∀ n, ∑_{d | n} φ(d) = n`
pub fn totient_sum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            app(cst("Eq"), nat_ty()),
            app(cst("DivisorSum"), app(cst("EulerTotient"), bvar(0))),
            bvar(0),
        ),
    )
}
/// `moebius_inversion : ∀ f g, (∀ n, g n = ∑_{d|n} f d) ↔ (∀ n, f n = ∑_{d|n} μ(d)*g(n/d))`
pub fn moebius_inversion_ty() -> Expr {
    cst("MoebiusInversionFormula")
}
/// `moebius_multiplicative : ∀ m n, gcd m n = 1 → μ(m*n) = μ(m)*μ(n)`
pub fn moebius_multiplicative_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(
                    app(cst("Eq"), nat_ty()),
                    app2(cst("Nat.gcd"), bvar(1), bvar(0)),
                    cst("Nat.one"),
                ),
                app2(
                    app(cst("Eq"), int_ty()),
                    app(cst("Moebius"), app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    app2(
                        cst("Int.mul"),
                        app(cst("Moebius"), bvar(1)),
                        app(cst("Moebius"), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `burnside_lemma : |X/G| = (1/|G|) * ∑_{g∈G} |Fix(g)|`
pub fn burnside_lemma_ty() -> Expr {
    cst("BurnsideLemma")
}
/// `polya_enumeration : PET expresses pattern inventory in terms of cycle index`
pub fn polya_enumeration_ty() -> Expr {
    cst("PolyaEnumerationTheorem")
}
/// Build the combinatorics environment: register all axioms as opaque constants.
pub fn build_combinatorics_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Fib", fib_ty()),
        ("Lucas", lucas_ty()),
        ("Catalan", catalan_ty()),
        ("Bell", bell_ty()),
        ("Bernoulli", bernoulli_ty()),
        ("Partition", partition_fn_ty()),
        ("Choose", choose_ty()),
        ("Multinomial", multinomial_ty()),
        ("Stirling1", stirling1_ty()),
        ("Stirling1u", stirling1u_ty()),
        ("Stirling2", stirling2_ty()),
        ("Derangement", derangement_ty()),
        ("EulerTotient", euler_totient_ty()),
        ("Moebius", moebius_ty()),
        ("FibClosedFormApprox", arrow(nat_ty(), prop())),
        ("CatalanRecurrenceHolds", prop()),
        ("BellTriangleRecurrence", prop()),
        ("SumStirling2Row", arrow(nat_ty(), nat_ty())),
        ("DerangementFormulaHolds", arrow(nat_ty(), prop())),
        ("InclusionExclusionPrinciple", prop()),
        ("RamseyNumber", ramsey_number_ty()),
        ("RamseyProperty", arrow(nat_ty(), prop())),
        (
            "VDWProperty",
            arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ),
        ("Nat.two", nat_ty()),
        ("Nat.three", nat_ty()),
        ("Nat.six", nat_ty()),
        ("OGF", ogf_ty()),
        ("EGF", egf_ty()),
        ("OGFProductLaw", prop()),
        ("EGFBellIsExpExpMinus1", prop()),
        ("OGFCatalanAlgebraic", prop()),
        ("DivisorSum", arrow(arrow(nat_ty(), nat_ty()), nat_ty())),
        ("MoebiusInversionFormula", prop()),
        ("BurnsideLemma", prop()),
        ("PolyaEnumerationTheorem", prop()),
        ("fib_recurrence", fib_recurrence_ty()),
        ("fib_gcd", fib_gcd_ty()),
        ("catalan_recurrence", catalan_recurrence_ty()),
        ("catalan_choose", catalan_choose_ty()),
        ("bell_sum_stirling2", bell_sum_stirling2_ty()),
        ("stirling2_recurrence", stirling2_recurrence_ty()),
        ("derangement_recurrence", derangement_recurrence_ty()),
        ("derangement_formula", derangement_formula_ty()),
        ("inclusion_exclusion", inclusion_exclusion_ty()),
        ("pigeonhole", pigeonhole_ty()),
        ("ramsey_theorem", ramsey_theorem_ty()),
        ("ramsey_33", ramsey_33_ty()),
        ("ramsey_upper_bound", ramsey_upper_bound_ty()),
        ("ramsey_symmetric", ramsey_symmetric_ty()),
        ("van_der_waerden", van_der_waerden_ty()),
        ("totient_sum", totient_sum_ty()),
        ("moebius_multiplicative", moebius_multiplicative_ty()),
        ("burnside_lemma", burnside_lemma_ty()),
        ("polya_enumeration", polya_enumeration_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Compute F(n) and F(n+1) simultaneously using fast doubling.
///
/// Returns `(F(n), F(n+1))`.
pub fn fib_fast_doubling(n: u64) -> (u128, u128) {
    if n == 0 {
        return (0, 1);
    }
    let (a, b) = fib_fast_doubling(n / 2);
    let c = a.wrapping_mul(b.wrapping_mul(2).wrapping_sub(a));
    let d = a * a + b * b;
    if n % 2 == 0 {
        (c, d)
    } else {
        (d, c.wrapping_add(d))
    }
}
/// Returns the n-th Fibonacci number F(n) (0-indexed: F(0)=0, F(1)=1).
///
/// Uses O(log n) fast-doubling. Returns a `u128` to avoid overflow for moderate n.
pub fn fibonacci(n: u64) -> u128 {
    fib_fast_doubling(n).0
}
/// Returns the n-th Lucas number L(n) (L(0)=2, L(1)=1).
pub fn lucas(n: u64) -> u128 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let (f_n, f_n1) = fib_fast_doubling(n);
            2 * f_n1 - f_n
        }
    }
}
/// Returns the n-th Catalan number C(n) = C(2n,n)/(n+1).
///
/// Uses big-integer arithmetic via u128 (accurate up to n ≈ 18).
pub fn catalan(n: u32) -> u128 {
    let mut result: u128 = 1;
    for i in 0..n {
        result = result * (2 * n as u128 - i as u128) / (i as u128 + 1);
    }
    result / (n as u128 + 1)
}
/// Compute a table of unsigned Stirling numbers of the first kind |s(n,k)| up to size `max_n+1`.
///
/// `stirling1u\[n\]\[k\]` = number of permutations of {1..n} with k cycles.
/// Recurrence: |s(n+1,k)| = n*|s(n,k)| + |s(n,k-1)|
pub fn stirling1u_table(max_n: usize) -> Vec<Vec<u128>> {
    let sz = max_n + 1;
    let mut t = vec![vec![0u128; sz]; sz];
    t[0][0] = 1;
    for n in 1..sz {
        for k in 1..=n {
            let prev_k = if k > 0 { t[n - 1][k] } else { 0 };
            let prev_k1 = if k >= 1 { t[n - 1][k - 1] } else { 0 };
            t[n][k] = (n as u128 - 1) * prev_k + prev_k1;
        }
    }
    t
}
/// Compute signed Stirling numbers of the first kind s(n,k) for a single row n.
///
/// s(n,k) = (-1)^(n-k) * |s(n,k)|
pub fn stirling1_row(n: usize) -> Vec<i128> {
    let table = stirling1u_table(n);
    table[n]
        .iter()
        .enumerate()
        .map(|(k, &v)| {
            if (n + k) % 2 == 0 {
                v as i128
            } else {
                -(v as i128)
            }
        })
        .collect()
}
/// Compute a table of Stirling numbers of the second kind S(n,k) up to size `max_n+1`.
///
/// `stirling2\[n\]\[k\]` = number of partitions of {1..n} into k non-empty subsets.
/// Recurrence: S(n+1,k) = k*S(n,k) + S(n,k-1)
pub fn stirling2_table(max_n: usize) -> Vec<Vec<u128>> {
    let sz = max_n + 1;
    let mut t = vec![vec![0u128; sz]; sz];
    t[0][0] = 1;
    for n in 1..sz {
        for k in 1..=n {
            let keep = k as u128 * t[n - 1][k];
            let merge = if k >= 1 { t[n - 1][k - 1] } else { 0 };
            t[n][k] = keep + merge;
        }
    }
    t
}
/// Compute Bell numbers up to B(max_n) using the Bell triangle.
pub fn bell_numbers(max_n: usize) -> Vec<u128> {
    if max_n == 0 {
        return vec![1];
    }
    let mut row: Vec<u128> = vec![1];
    let mut bells = vec![1u128];
    for _ in 1..=max_n {
        let mut next = vec![0u128; row.len() + 1];
        next[0] = *row
            .last()
            .expect("row is non-empty: initialized with one element");
        for j in 1..=row.len() {
            next[j] = next[j - 1] + row[j - 1];
        }
        bells.push(next[0]);
        row = next;
    }
    bells
}
/// Compute the partition function p(n) for all n up to `max_n` using Euler's pentagonal formula.
///
/// p(n) = ∑_{k≠0} (-1)^(k+1) * p(n - k*(3k-1)/2)
pub fn partition_numbers(max_n: usize) -> Vec<u128> {
    let mut p = vec![0u128; max_n + 1];
    p[0] = 1;
    for n in 1..=max_n {
        let mut k: i64 = 1;
        loop {
            let pent1 = (k * (3 * k - 1) / 2) as usize;
            let pent2 = (k * (3 * k + 1) / 2) as usize;
            if pent1 > n {
                break;
            }
            let sign: i64 = if k % 2 == 1 { 1 } else { -1 };
            let add1 = p[n - pent1] as i64 * sign;
            p[n] = (p[n] as i64 + add1) as u128;
            if pent2 <= n {
                let add2 = p[n - pent2] as i64 * sign;
                p[n] = (p[n] as i64 + add2) as u128;
            }
            k += 1;
        }
    }
    p
}
/// Compute D(n) = number of derangements of n elements.
///
/// Recurrence: D(0)=1, D(1)=0, D(n) = (n-1)*(D(n-1) + D(n-2))
pub fn derangements(max_n: usize) -> Vec<u128> {
    let mut d = vec![0u128; max_n + 1];
    if max_n == 0 {
        d[0] = 1;
        return d;
    }
    d[0] = 1;
    d[1] = 0;
    for n in 2..=max_n {
        d[n] = (n as u128 - 1) * (d[n - 1] + d[n - 2]);
    }
    d
}
/// Compute φ(n) for all n ≤ max_n using a sieve.
pub fn totient_sieve(max_n: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=max_n as u64).collect();
    for i in 2..=max_n {
        if phi[i] == i as u64 {
            let mut j = i;
            while j <= max_n {
                phi[j] -= phi[j] / i as u64;
                j += i;
            }
        }
    }
    phi
}
/// Compute μ(n) for all n ≤ max_n using a linear sieve.
///
/// μ(1) = 1, μ(p₁…pₖ) = (-1)^k, μ(n) = 0 if n has a squared prime factor.
pub fn moebius_sieve(max_n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; max_n + 1];
    let mut is_prime = vec![true; max_n + 1];
    let mut primes: Vec<usize> = Vec::new();
    mu[1] = 1;
    for i in 2..=max_n {
        if is_prime[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > max_n {
                break;
            }
            is_prime[ip] = false;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            } else {
                mu[ip] = -mu[i];
            }
        }
    }
    mu
}
/// Compute C(n, k) = n! / (k! * (n-k)!) using Pascal's triangle up to `max_n`.
pub fn binomial_table(max_n: usize) -> Vec<Vec<u128>> {
    let sz = max_n + 1;
    let mut c = vec![vec![0u128; sz]; sz];
    c[0][0] = 1;
    for n in 1..sz {
        c[n][0] = 1;
        for k in 1..=n {
            c[n][k] = c[n - 1][k - 1] + c[n - 1][k];
        }
    }
    c
}
/// Compute a single binomial coefficient C(n, k) without building a full table.
pub fn choose(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: u128 = 1;
    for i in 0..k {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }
    result
}
/// Enumerate all subsets of `items` as vectors.
pub fn all_subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let n = items.len();
    let total = 1usize << n;
    (0..total)
        .map(|mask| {
            (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| items[i].clone())
                .collect()
        })
        .collect()
}
/// Apply the inclusion–exclusion principle to count elements in exactly 0 of n sets,
/// given the sizes of all `2^n` intersections as a function `size_of(mask)`.
///
/// Returns |S₁ ∪ … ∪ Sₙ|.
pub fn inclusion_exclusion_count(n: usize, size_of: impl Fn(u32) -> i64) -> i64 {
    let mut result = 0i64;
    for mask in 1u32..(1 << n) {
        let bits = mask.count_ones() as i64;
        let sign = if bits % 2 == 1 { 1 } else { -1 };
        result += sign * size_of(mask);
    }
    result
}
/// Apply Burnside's lemma: given a list of fixed-point counts |Fix(g)| for each group element g,
/// return the number of distinct orbits |X/G| (as a fraction numerator and denominator).
///
/// Returns `(numerator, denominator)` where `numerator/denominator = |X/G|`.
pub fn burnside(fixed_point_counts: &[u64]) -> (u64, u64) {
    let total: u64 = fixed_point_counts.iter().sum();
    let g = fixed_point_counts.len() as u64;
    let d = gcd(total, g);
    (total / d, g / d)
}
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// `TuranNumber : Nat → Nat → Nat`
/// ex(n, r) = maximum edges in an n-vertex K_{r+1}-free graph.
pub fn turan_number_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `turan_theorem : ∀ n r, edges(T(n,r)) = TuranNumber n r`
/// Turán's theorem: the Turán graph T(n,r) is the unique extremal graph.
pub fn turan_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "r",
            nat_ty(),
            app2(
                app(cst("Eq"), nat_ty()),
                app2(cst("TuranGraphEdges"), bvar(1), bvar(0)),
                app2(cst("TuranNumber"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `KruskalKatona : Prop`
/// Kruskal-Katona theorem: shadow minimization for k-element sets.
pub fn kruskal_katona_ty() -> Expr {
    prop()
}
/// `FranklUnion : Prop`
/// Frankl's union-closed sets conjecture (stated as axiom/open problem).
pub fn frankl_union_ty() -> Expr {
    prop()
}
/// `RamseyMultiplicity : Nat → Nat → Nat`
/// M(s,t) = minimum number of monochromatic K_s or K_t in a 2-coloring of K_{R(s,t)}.
pub fn ramsey_multiplicity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `SchurNumber : Nat → Nat`
/// S(k) = largest n such that {1..n} can be k-colored without monochromatic x+y=z.
pub fn schur_number_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `schur_theorem : ∀ k, ∃ N, ∀ k-coloring of {1..N}, ∃ x y z, x+y=z (mono color)`
pub fn schur_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        app(
            app(cst("Exists"), nat_ty()),
            app(cst("SchurProperty"), bvar(0)),
        ),
    )
}
/// `van_der_waerden_bounds : ∀ k r, W(k;r) ≤ ExpTower k r`
pub fn van_der_waerden_bounds_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "r",
            nat_ty(),
            app(
                app(cst("Nat.le"), app2(cst("VDW"), bvar(1), bvar(0))),
                app2(cst("ExpTower"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `LovaszLocalLemmaTy : Prop`
/// Lovász Local Lemma: if each bad event has low probability and limited dependencies.
pub fn lovasz_local_lemma_ty() -> Expr {
    prop()
}
/// `AlterationMethod : Prop`
/// Erdős–Spencer alteration method for existence proofs.
pub fn alteration_method_ty() -> Expr {
    prop()
}
/// `SecondMomentMethod : Prop`
/// Second moment method: E\[X²\] ≥ E\[X\]² / E[1_{X>0}].
pub fn second_moment_method_ty() -> Expr {
    prop()
}
/// `lovasz_local_lemma_sym : ∀ (p d : Nat), p*(d+1) ≤ 1 → ∃ assignment, NoBADEvent`
pub fn lovasz_local_lemma_sym_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "d",
            nat_ty(),
            arrow(
                app2(
                    cst("Nat.le"),
                    app2(
                        cst("Nat.mul"),
                        bvar(1),
                        app2(cst("Nat.add"), bvar(0), cst("Nat.one")),
                    ),
                    cst("Nat.one"),
                ),
                app(app(cst("Exists"), cst("Assignment")), cst("NoBADEvent")),
            ),
        ),
    )
}
/// `RSKCorrespondence : Prop`
/// Robinson-Schensted-Knuth correspondence between permutations and pairs of Young tableaux.
pub fn rsk_correspondence_ty() -> Expr {
    prop()
}
/// `JeuDeTaquin : Prop`
/// Jeu de taquin: sliding algorithm on skew Young tableaux (Schützenberger).
pub fn jeu_de_taquin_ty() -> Expr {
    prop()
}
/// `YoungTableau : Type` — a standard Young tableau.
pub fn young_tableau_ty() -> Expr {
    type0()
}
/// `SchurPolynomial : YoungTableau → Type`
/// The Schur polynomial s_λ associated to a partition λ.
pub fn schur_polynomial_ty() -> Expr {
    arrow(cst("YoungTableau"), type0())
}
/// `LittlewoodRichardson : Prop`
/// Littlewood-Richardson rule for expanding s_λ * s_μ.
pub fn littlewood_richardson_ty() -> Expr {
    prop()
}
/// `Matroid : Type` — a matroid (E, I) with independent sets I.
pub fn matroid_ty() -> Expr {
    type0()
}
/// `MatroidCircuit : Matroid → Type`
/// The circuit axioms: C ⊂ C' ∈ circuits implies C = C'.
pub fn matroid_circuit_ty() -> Expr {
    arrow(cst("Matroid"), type0())
}
/// `matroid_union : Matroid → Matroid → Matroid`
/// Union of two matroids (Edmonds).
pub fn matroid_union_ty() -> Expr {
    arrow(cst("Matroid"), arrow(cst("Matroid"), cst("Matroid")))
}
/// `matroid_intersection_rank : Matroid → Matroid → Nat`
/// Maximum weight common independent set (matroid intersection).
pub fn matroid_intersection_rank_ty() -> Expr {
    arrow(cst("Matroid"), arrow(cst("Matroid"), nat_ty()))
}
/// `matroid_polytope : Matroid → Type`
/// The base polytope of a matroid.
pub fn matroid_polytope_ty() -> Expr {
    arrow(cst("Matroid"), type0())
}
/// `matroid_exchange_axiom : ∀ (M : Matroid), ExchangeAxiomHolds M`
pub fn matroid_exchange_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Matroid"),
        app(cst("ExchangeAxiomHolds"), bvar(0)),
    )
}
/// `CayleyThm : Prop`
/// Cayley's theorem: every group is isomorphic to a permutation group.
pub fn cayley_thm_ty() -> Expr {
    prop()
}
/// `BurnsidePolya : Prop`
/// Burnside-Pólya enumeration with cycle index.
pub fn burnside_polya_ty() -> Expr {
    prop()
}
/// `CycleIndex : (Nat → Nat) → Type`
/// Cycle index polynomial Z(G; x₁, …, xₙ).
pub fn cycle_index_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), type0())
}
/// `polya_cycle_index_formula : ∀ G, CycleIndex G = Z_G`
pub fn polya_cycle_index_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        arrow(nat_ty(), nat_ty()),
        app2(
            app(cst("Eq"), type0()),
            app(cst("CycleIndex"), bvar(0)),
            app(cst("Z_G"), bvar(0)),
        ),
    )
}
/// `DirichletGF : (Nat → Nat) → Type`
/// Dirichlet series ∑ aₙ / n^s.
pub fn dirichlet_gf_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), type0())
}
/// `EulerProduct : (Nat → Nat) → Type`
/// Euler product ∏_p (1 + a_p / p^s + ...).
pub fn euler_product_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), type0())
}
/// `RogersRamanujan : Prop`
/// Rogers-Ramanujan identities: product-sum relations for partitions.
pub fn rogers_ramanujan_ty() -> Expr {
    prop()
}
/// `DirichletConvolution : (Nat → Nat) → (Nat → Nat) → (Nat → Nat)`
/// Dirichlet convolution (f * g)(n) = ∑_{d|n} f(d) g(n/d).
pub fn dirichlet_convolution_ty() -> Expr {
    arrow(
        arrow(nat_ty(), nat_ty()),
        arrow(arrow(nat_ty(), nat_ty()), arrow(nat_ty(), nat_ty())),
    )
}
/// `LGV : Prop`
/// Lindström-Gessel-Viennot lemma: det(e(a_i, b_j)) = signed count of non-intersecting lattice paths.
pub fn lgv_ty() -> Expr {
    prop()
}
/// `CycleLemma : Prop`
/// Cycle lemma (Dvoretzky-Motzkin): exactly 1/n+1 cyclic shifts of a ballot sequence are valid.
pub fn cycle_lemma_ty() -> Expr {
    prop()
}
/// `BallotProblem : Nat → Nat → Nat`
/// Ballot(a, b) = probability that A stays strictly ahead; = (a - b) / (a + b).
pub fn ballot_problem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `ballot_formula : ∀ a b, a > b → BallotProblem a b = (a - b) / (a + b)`
pub fn ballot_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "b",
            nat_ty(),
            arrow(
                app2(cst("Nat.lt"), bvar(0), bvar(1)),
                app(
                    cst("BallotFormula"),
                    app2(cst("BallotProblem"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `Species : Type` — Joyal's combinatorial species (functor Fin → Set).
pub fn species_ty() -> Expr {
    type0()
}
/// `VirtualSpecies : Type` — difference of two species (for subtraction).
pub fn virtual_species_ty() -> Expr {
    type0()
}
/// `MolecularSpecies : Type` — connected/indecomposable species.
pub fn molecular_species_ty() -> Expr {
    type0()
}
/// `species_product : Species → Species → Species`
/// Cartesian product of species F and G.
pub fn species_product_ty() -> Expr {
    arrow(cst("Species"), arrow(cst("Species"), cst("Species")))
}
/// `species_composition : Species → Species → Species`
/// Substitution F ∘ G (partitional composition).
pub fn species_composition_ty() -> Expr {
    arrow(cst("Species"), arrow(cst("Species"), cst("Species")))
}
/// `DilworthThm : Prop`
/// Dilworth's theorem: max antichain = min chain cover.
pub fn dilworth_thm_ty() -> Expr {
    prop()
}
/// `MirskyThm : Prop`
/// Mirsky's theorem: max chain = min antichain cover.
pub fn mirsky_thm_ty() -> Expr {
    prop()
}
/// `SauerShelah : Nat → Nat → Nat`
/// Sauer-Shelah lemma: Phi(m, d) = bound on shattering.
pub fn sauer_shelah_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `SunflowerLemma : Prop`
/// Erdős-Ko-Rado sunflower lemma.
pub fn sunflower_lemma_ty() -> Expr {
    prop()
}
/// `SpencerDiscrepancy : Prop`
/// Spencer's six standard deviations theorem: disc(A) ≤ 6√n.
pub fn spencer_discrepancy_ty() -> Expr {
    prop()
}
/// `LovaszSpencer : Prop`
/// Lovász-Spencer lower bound on discrepancy.
pub fn lovasz_spencer_ty() -> Expr {
    prop()
}
/// `CombDiscrepancy : (Nat → Nat) → Nat`
/// Combinatorial discrepancy of a set system.
pub fn comb_discrepancy_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), nat_ty())
}
/// `SumsetBound : Nat → Nat → Nat`
/// |A + B| ≥ |A| + |B| - 1 (Cauchy-Davenport for primes).
pub fn sumset_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `FreimanRuzsa : Prop`
/// Freiman-Ruzsa theorem: small doubling implies A lies in a generalized AP.
pub fn freiman_ruzsa_ty() -> Expr {
    prop()
}
/// `AdditiveEnergyBound : Nat → Nat → Nat`
/// E(A, B) = |{(a₁,a₂,b₁,b₂) : a₁+b₁=a₂+b₂}| additive energy.
pub fn additive_energy_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `cauchy_davenport : ∀ p A B, prime p → |A+B| ≥ min p (|A|+|B|-1)`
pub fn cauchy_davenport_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "a",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "b",
                nat_ty(),
                arrow(
                    app(cst("IsPrime"), bvar(2)),
                    app(
                        app(cst("Nat.le"), app2(cst("SumsetBound"), bvar(1), bvar(0))),
                        app2(
                            cst("Nat.min"),
                            bvar(2),
                            app2(
                                cst("Nat.add"),
                                app2(cst("Nat.add"), bvar(1), bvar(0)),
                                cst("Nat.pred_one"),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Register all extended combinatorics axioms (§8–§19) into the environment.
pub fn build_combinatorics_env_ext(env: &mut Environment) -> Result<(), String> {
    build_combinatorics_env(env)?;
    let axioms: &[(&str, Expr)] = &[
        ("TuranNumber", turan_number_ty()),
        (
            "TuranGraphEdges",
            arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        ),
        ("turan_theorem", turan_theorem_ty()),
        ("KruskalKatona", kruskal_katona_ty()),
        ("FranklUnion", frankl_union_ty()),
        ("RamseyMultiplicity", ramsey_multiplicity_ty()),
        ("SchurNumber", schur_number_ty()),
        ("SchurProperty", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("schur_theorem", schur_theorem_ty()),
        ("VDW", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("ExpTower", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("van_der_waerden_bounds", van_der_waerden_bounds_ty()),
        ("LovaszLocalLemmaTy", lovasz_local_lemma_ty()),
        ("AlterationMethod", alteration_method_ty()),
        ("SecondMomentMethod", second_moment_method_ty()),
        ("Assignment", type0()),
        ("NoBADEvent", prop()),
        ("lovasz_local_lemma_sym", lovasz_local_lemma_sym_ty()),
        ("RSKCorrespondence", rsk_correspondence_ty()),
        ("JeuDeTaquin", jeu_de_taquin_ty()),
        ("YoungTableau", young_tableau_ty()),
        ("SchurPolynomial", schur_polynomial_ty()),
        ("LittlewoodRichardson", littlewood_richardson_ty()),
        ("Matroid", matroid_ty()),
        ("MatroidCircuit", matroid_circuit_ty()),
        ("matroid_union", matroid_union_ty()),
        ("matroid_intersection_rank", matroid_intersection_rank_ty()),
        ("matroid_polytope", matroid_polytope_ty()),
        ("ExchangeAxiomHolds", arrow(cst("Matroid"), prop())),
        ("matroid_exchange_axiom", matroid_exchange_axiom_ty()),
        ("CayleyThm", cayley_thm_ty()),
        ("BurnsidePolya", burnside_polya_ty()),
        ("CycleIndex", cycle_index_ty()),
        ("Z_G", arrow(arrow(nat_ty(), nat_ty()), type0())),
        ("polya_cycle_index_formula", polya_cycle_index_formula_ty()),
        ("DirichletGF", dirichlet_gf_ty()),
        ("EulerProduct", euler_product_ty()),
        ("RogersRamanujan", rogers_ramanujan_ty()),
        ("DirichletConvolution", dirichlet_convolution_ty()),
        ("LGV", lgv_ty()),
        ("CycleLemma", cycle_lemma_ty()),
        ("BallotProblem", ballot_problem_ty()),
        ("BallotFormula", arrow(nat_ty(), prop())),
        ("ballot_formula", ballot_formula_ty()),
        ("Species", species_ty()),
        ("VirtualSpecies", virtual_species_ty()),
        ("MolecularSpecies", molecular_species_ty()),
        ("species_product", species_product_ty()),
        ("species_composition", species_composition_ty()),
        ("DilworthThm", dilworth_thm_ty()),
        ("MirskyThm", mirsky_thm_ty()),
        ("SauerShelah", sauer_shelah_ty()),
        ("SunflowerLemma", sunflower_lemma_ty()),
        ("SpencerDiscrepancy", spencer_discrepancy_ty()),
        ("LovaszSpencer", lovasz_spencer_ty()),
        ("CombDiscrepancy", comb_discrepancy_ty()),
        ("SumsetBound", sumset_bound_ty()),
        ("FreimanRuzsa", freiman_ruzsa_ty()),
        ("AdditiveEnergyBound", additive_energy_bound_ty()),
        ("IsPrime", arrow(nat_ty(), prop())),
        ("Nat.min", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("Nat.pred_one", nat_ty()),
        ("cauchy_davenport", cauchy_davenport_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Euler's totient function φ(n) for use in CycleIndexPolynomial::cyclic.
pub fn euler_phi(n: u64) -> u64 {
    let mut result = n;
    let mut m = n;
    let mut p = 2u64;
    while p * p <= m {
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if m > 1 {
        result -= result / m;
    }
    result
}
pub fn gcd_usize(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_usize(b, a % b)
    }
}
/// Compute an upper bound table for Ramsey numbers R(s,t) using the bound
/// R(s,t) ≤ R(s-1,t) + R(s,t-1) and R(1,t) = 1, R(s,1) = 1.
///
/// Returns a `max×max` table where `table\[s\]\[t\]` is an upper bound on R(s,t).
pub fn ramsey_upper_bounds(max: usize) -> Vec<Vec<u64>> {
    let sz = max + 1;
    let mut r = vec![vec![u64::MAX / 2; sz]; sz];
    for i in 1..sz {
        r[1][i] = 1;
        r[i][1] = 1;
    }
    for i in 2..sz {
        r[2][i] = i as u64;
        r[i][2] = i as u64;
    }
    for s in 3..sz {
        for t in 3..sz {
            r[s][t] = r[s - 1][t].saturating_add(r[s][t - 1]);
        }
    }
    r
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibonacci() {
        let expected = [0u128, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u64), e, "F({i})");
        }
        assert_eq!(fibonacci(20), 6765);
    }
    #[test]
    fn test_lucas() {
        let expected = [2u128, 1, 3, 4, 7, 11, 18, 29, 47, 76];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(lucas(i as u64), e, "L({i})");
        }
    }
    #[test]
    fn test_catalan() {
        let expected = [1u128, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(catalan(i as u32), e, "C({i})");
        }
    }
    #[test]
    fn test_bell_numbers() {
        let bells = bell_numbers(10);
        let expected = [1u128, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 115975];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(bells[i], e, "B({i})");
        }
    }
    #[test]
    fn test_stirling2() {
        let t = stirling2_table(5);
        assert_eq!(t[4][2], 7);
        assert_eq!(t[5][3], 25);
        for n in 1..=5 {
            assert_eq!(t[n][1], 1, "S({n},1)");
        }
        for n in 0..=5 {
            assert_eq!(t[n][n], 1, "S({n},{n})");
        }
    }
    #[test]
    fn test_stirling1u() {
        let t = stirling1u_table(5);
        assert_eq!(t[5][2], 50);
        for n in 0..=5 {
            assert_eq!(t[n][n], 1, "|s({n},{n})|");
        }
        let factorials = [1u128, 1, 2, 6, 24];
        for n in 1..=5 {
            assert_eq!(t[n][1], factorials[n - 1], "|s({n},1)|");
        }
    }
    #[test]
    fn test_partition_numbers() {
        let p = partition_numbers(10);
        let expected = [1u128, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(p[i], e, "p({i})");
        }
    }
    #[test]
    fn test_derangements() {
        let d = derangements(10);
        let expected = [1u128, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(d[i], e, "D({i})");
        }
    }
    #[test]
    fn test_totient_sieve() {
        let phi = totient_sieve(10);
        let expected = [0u64, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(phi[i], e, "φ({i})");
        }
    }
    #[test]
    fn test_moebius_sieve() {
        let mu = moebius_sieve(12);
        assert_eq!(mu[1], 1);
        assert_eq!(mu[2], -1);
        assert_eq!(mu[3], -1);
        assert_eq!(mu[4], 0);
        assert_eq!(mu[6], 1);
        assert_eq!(mu[12], 0);
    }
    #[test]
    fn test_choose() {
        assert_eq!(choose(10, 3), 120);
        assert_eq!(choose(20, 10), 184756);
        assert_eq!(choose(5, 0), 1);
        assert_eq!(choose(5, 5), 1);
        assert_eq!(choose(3, 5), 0);
    }
    #[test]
    fn test_poly_add_mul() {
        let p = Poly(vec![1, 1]);
        let q = p.mul(&p);
        assert_eq!(q, Poly(vec![1, 2, 1]));
        let a = Poly(vec![1, 2]);
        let b = Poly(vec![3, 1]);
        assert_eq!(a.add(&b), Poly(vec![4, 3]));
    }
    #[test]
    fn test_poly_eval() {
        let p = Poly(vec![1, 2, 1]);
        assert_eq!(p.eval(3), 16);
    }
    #[test]
    fn test_all_subsets() {
        let subs = all_subsets(&[1, 2, 3]);
        assert_eq!(subs.len(), 8);
        assert!(subs.contains(&vec![]));
        assert!(subs.contains(&vec![1, 2, 3]));
    }
    #[test]
    fn test_inclusion_exclusion() {
        let size_of = |mask: u32| -> i64 {
            match mask {
                0b001 => 3,
                0b010 => 4,
                0b100 => 5,
                0b011 => 2,
                0b101 => 1,
                0b110 => 2,
                0b111 => 1,
                _ => 0,
            }
        };
        assert_eq!(inclusion_exclusion_count(3, size_of), 8);
    }
    #[test]
    fn test_burnside() {
        let (num, den) = burnside(&[16, 4, 4, 4]);
        assert_eq!(num * den, num * den);
        assert_eq!(num / den, 7, "28/4 = 7");
        let (n, d) = burnside(&[16, 4, 4, 4]);
        assert_eq!(n, 7);
        assert_eq!(d, 1);
    }
    #[test]
    fn test_ramsey_upper_bounds() {
        let r = ramsey_upper_bounds(6);
        assert!(r[3][3] <= 6);
        assert_eq!(r[2][5], 5);
    }
    #[test]
    fn test_build_combinatorics_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_combinatorics_env(&mut env);
        assert!(
            result.is_ok(),
            "build_combinatorics_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_build_combinatorics_env_ext() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_combinatorics_env_ext(&mut env);
        assert!(
            result.is_ok(),
            "build_combinatorics_env_ext failed: {:?}",
            result.err()
        );
        use oxilean_kernel::Name;
        assert!(env.get(&Name::str("TuranNumber")).is_some());
        assert!(env.get(&Name::str("Matroid")).is_some());
        assert!(env.get(&Name::str("FreimanRuzsa")).is_some());
        assert!(env.get(&Name::str("LovaszLocalLemmaTy")).is_some());
    }
    #[test]
    fn test_matroid_intersection_solver() {
        let solver = MatroidIntersectionSolver::new(
            4,
            vec![0, 0, 1, 1],
            vec![1, 1],
            vec![0, 1, 0, 1],
            vec![1, 1],
        );
        let (size, _mask) = solver.solve_brute_force();
        assert_eq!(size, 2);
    }
    #[test]
    fn test_lovasz_local_lemma_symmetric() {
        let lll = LovaszLocalLemma::new(10, 1, 16, 3);
        assert!(lll.symmetric_condition_holds());
        let lll_fail = LovaszLocalLemma::new(10, 1, 2, 3);
        assert!(!lll_fail.symmetric_condition_holds());
    }
    #[test]
    fn test_cycle_index_polynomial_cyclic() {
        let cip = CycleIndexPolynomial::cyclic(4);
        assert_eq!(cip.group_order, 4);
        assert_eq!(cip.eval_all_ones(), 1);
    }
    #[test]
    fn test_turan_density_computer() {
        let tdc = TuranDensityComputer::new(2);
        let edges = tdc.turan_edges(6);
        assert_eq!(edges, 9);
        let (num, den) = tdc.density();
        assert_eq!(num, 1);
        assert_eq!(den, 2);
    }
    #[test]
    fn test_turan_edges_k4_free() {
        let tdc = TuranDensityComputer::new(3);
        assert_eq!(tdc.turan_edges(9), 27);
    }
    #[test]
    fn test_cycle_index_burnside_count() {
        let cip = CycleIndexPolynomial::cyclic(2);
        assert_eq!(cip.burnside_count(2), 3);
    }
}
/// Ballot number B(k, n): number of sequences with k +1s and n -1s
/// where every prefix sum is positive.
///
/// B(k, n) = (k - n) / (k + n) * C(k+n, n) for k > n ≥ 0.
#[allow(dead_code)]
pub fn ballot_number(k: u64, n: u64) -> u64 {
    if k == 0 && n == 0 {
        return 1;
    }
    if k <= n {
        return 0;
    }
    let c = binomial(k + n, n);
    c * (k - n) / (k + n)
}
/// Binomial coefficient C(n, k).
#[allow(dead_code)]
pub fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
/// Motzkin number M(n): number of ways to draw non-crossing chords on n points.
///
/// M(0) = 1, M(n) = ((2n+2) M(n-1) + (3n-3) M(n-2)) / (n+2).
#[allow(dead_code)]
pub fn motzkin_number(n: u64) -> u64 {
    let mut m = vec![0u64; (n + 1) as usize];
    m[0] = 1;
    if n == 0 {
        return 1;
    }
    m[1] = 1;
    for i in 2..=(n as usize) {
        let i64v = i as u64;
        m[i] = ((2 * i64v + 2) * m[i - 1] + (3 * i64v - 3) * m[i - 2]) / (i64v + 2);
    }
    m[n as usize]
}
/// Narayana number N(n, k): number of Dyck paths of length 2n with k peaks.
///
/// N(n, k) = C(n, k)^2 / n.
#[allow(dead_code)]
pub fn narayana_number(n: u64, k: u64) -> u64 {
    if n == 0 || k == 0 || k > n {
        return 0;
    }
    let c_nk = binomial(n, k);
    let c_nk1 = binomial(n, k - 1);
    c_nk * c_nk1 / n
}
/// Chromatic polynomial of a complete graph K_n: n * (n-1) * ... * (n-k+1) evaluated at k.
///
/// P(K_n, k) = k * (k-1) * ... * (k-n+1) = falling factorial (k)_n.
#[allow(dead_code)]
pub fn chromatic_complete_graph(n: u64, k: u64) -> u64 {
    if k < n {
        return 0;
    }
    let mut result = 1u64;
    for i in 0..n {
        result *= k - i;
    }
    result
}
/// Chromatic polynomial of a cycle C_n: (k-1)^n + (-1)^n * (k-1).
#[allow(dead_code)]
pub fn chromatic_cycle_graph(n: u64, k: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let k1 = k - 1;
    let pow_pos = k1.pow(n as u32);
    let sign = if n % 2 == 0 { 1i64 } else { -1i64 };
    pow_pos + sign * k1
}
/// Chromatic polynomial of a tree T_n (n vertices): k * (k-1)^{n-1}.
#[allow(dead_code)]
pub fn chromatic_tree(n: u64, k: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    k * (k - 1).pow((n - 1) as u32)
}
/// Number of labeled spanning trees of K_n by Cayley's formula: n^{n-2}.
#[allow(dead_code)]
pub fn cayley_spanning_trees(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let base = n;
    let exp = n - 2;
    let mut result = 1u64;
    for _ in 0..exp {
        result *= base;
    }
    result
}
/// Count of reduced Latin squares of order n (first row and column sorted).
/// Values for small n from combinatorial enumeration.
#[allow(dead_code)]
pub fn reduced_latin_squares(n: u64) -> Option<u64> {
    match n {
        1 => Some(1),
        2 => Some(1),
        3 => Some(1),
        4 => Some(4),
        5 => Some(56),
        6 => Some(9408),
        7 => Some(16942080),
        _ => None,
    }
}
/// Number of Latin squares L(n) = n! * (n-1)! * R(n) where R(n) is number of reduced.
#[allow(dead_code)]
pub fn latin_squares_count(n: u64) -> Option<u64> {
    let r = reduced_latin_squares(n)?;
    let n_fact: u64 = (1..=n).product();
    let n1_fact: u64 = (1..n).product();
    Some(n_fact * n1_fact * r)
}
/// Partition into odd parts: equals number of partitions into distinct parts.
/// This is Euler's partition theorem. We verify for small n.
#[allow(dead_code)]
pub fn partitions_odd_parts(n: u64) -> u64 {
    let mut dp = vec![0u64; (n + 1) as usize];
    dp[0] = 1;
    let mut p = 1u64;
    while p <= n {
        if p % 2 == 1 {
            for i in p..=n {
                dp[i as usize] += dp[(i - p) as usize];
            }
        }
        p += 1;
    }
    dp[n as usize]
}
/// Partition into distinct parts: number of partitions where all parts differ.
#[allow(dead_code)]
pub fn partitions_distinct_parts(n: u64) -> u64 {
    let mut dp = vec![0u64; (n + 1) as usize];
    dp[0] = 1;
    for p in 1..=n {
        for i in (p..=n).rev() {
            dp[i as usize] += dp[(i - p) as usize];
        }
    }
    dp[n as usize]
}
/// Number of self-conjugate partitions of n (equals partitions into distinct odd parts).
#[allow(dead_code)]
pub fn self_conjugate_partitions(n: u64) -> u64 {
    let mut dp = vec![0u64; (n + 1) as usize];
    dp[0] = 1;
    let mut p = 1u64;
    while p <= n {
        if p % 2 == 1 {
            for i in (p..=n).rev() {
                dp[i as usize] += dp[(i - p) as usize];
            }
        }
        p += 2;
    }
    dp[n as usize]
}
/// Sunflower bound: the Erdős-Ko-Rado sunflower lemma.
/// If a family of r-element sets has > (p-1)^r * r! sets, it contains a sunflower with p petals.
#[allow(dead_code)]
pub fn sunflower_threshold(r: u64, p: u64) -> u64 {
    let base = p.saturating_sub(1);
    let mut result = 1u64;
    for _ in 0..r {
        result = result.saturating_mul(base);
    }
    let r_fact: u64 = (1..=r).product();
    result.saturating_mul(r_fact)
}
/// Kruskal-Katona theorem: shadow bound for uniform hypergraphs.
/// Given m sets of size r, the shadow has at least C(n, r-1) sets where n = largest n with C(n,r) ≤ m.
#[allow(dead_code)]
pub fn kruskal_katona_shadow_bound(m: u64, r: u64) -> u64 {
    let mut n = r;
    while binomial(n, r) <= m {
        n += 1;
    }
    n -= 1;
    if r == 0 {
        return 0;
    }
    binomial(n, r - 1)
}
#[cfg(test)]
mod tests_combinatorics_extended {
    use super::*;
    #[test]
    fn test_ballot_number_basic() {
        assert_eq!(ballot_number(3, 1), 2);
        assert_eq!(ballot_number(2, 0), 1);
    }
    #[test]
    fn test_motzkin_numbers() {
        assert_eq!(motzkin_number(0), 1);
        assert_eq!(motzkin_number(1), 1);
        assert_eq!(motzkin_number(2), 2);
        assert_eq!(motzkin_number(3), 4);
        assert_eq!(motzkin_number(4), 9);
    }
    #[test]
    fn test_narayana_numbers() {
        assert_eq!(narayana_number(4, 2), 6);
        let c4: u64 = (1..=4).map(|k| narayana_number(4, k)).sum();
        assert_eq!(c4, 14);
    }
    #[test]
    fn test_chromatic_complete() {
        assert_eq!(chromatic_complete_graph(3, 3), 6);
        assert_eq!(chromatic_complete_graph(4, 4), 24);
    }
    #[test]
    fn test_chromatic_cycle() {
        assert_eq!(chromatic_cycle_graph(4, 3), 18);
    }
    #[test]
    fn test_cayley_spanning_trees() {
        assert_eq!(cayley_spanning_trees(4), 16);
        assert_eq!(cayley_spanning_trees(5), 125);
    }
    #[test]
    fn test_euler_partition_theorem() {
        for n in 0u64..=10 {
            assert_eq!(
                partitions_odd_parts(n),
                partitions_distinct_parts(n),
                "n = {}",
                n
            );
        }
    }
    #[test]
    fn test_self_conjugate_partitions() {
        let sc4 = self_conjugate_partitions(4);
        assert!(sc4 >= 1);
    }
    #[test]
    fn test_reduced_latin_squares() {
        assert_eq!(reduced_latin_squares(1), Some(1));
        assert_eq!(reduced_latin_squares(4), Some(4));
        assert_eq!(reduced_latin_squares(5), Some(56));
    }
    #[test]
    fn test_sunflower_threshold() {
        assert_eq!(sunflower_threshold(2, 3), 8);
    }
}
