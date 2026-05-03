//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BettiNumbers, BuchbergerAlgorithm, BuchbergerCriterion, GroebnerBasis, HilbertFunction,
    HilbertPolynomial, IdealMembership, MinimalFreeResolution, Monomial, MonomialOrder,
    NullstellensatzCertificate, Polynomial, ReducedGroebnerBasis, SyzygyModule, Term,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `MonomialOrder : Type` — a total order on monomials compatible with multiplication.
pub fn monomial_order_ty() -> Expr {
    type0()
}
/// `Monomial : ℕ → Type` — a monomial in n variables as an exponent vector.
/// `Monomial n` represents x_1^{a_1} · … · x_n^{a_n}.
pub fn monomial_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Polynomial : Type → ℕ → Type` — a polynomial over field F in n variables.
/// Represented as a finite formal sum of (Monomial n, F) pairs.
pub fn polynomial_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `GroebnerBasis : (I : Ideal) → Type` — a Gröbner basis for ideal I.
/// A finite set G ⊆ I such that LT(I) = ⟨LT(g) : g ∈ G⟩.
pub fn groebner_basis_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `SPolynomial : Polynomial → Polynomial → Polynomial` — the S-polynomial
/// S(f, g) = lcm(LM(f),LM(g))/LT(f) · f − lcm(LM(f),LM(g))/LT(g) · g.
pub fn s_polynomial_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(
                app2(cst("Polynomial"), bvar(1), nat_ty()),
                app2(cst("Polynomial"), bvar(2), nat_ty()),
            ),
        ),
    )
}
/// `IdealMembership : Polynomial → Ideal → Prop` — f ∈ I iff normal form w.r.t. G(I) is 0.
pub fn ideal_membership_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(cst("Ideal"), prop()),
        ),
    )
}
/// `HilbertFunction : Ideal → ℕ → ℕ` — H(I, t) = dim_k(R/I)_t,
/// the Hilbert function of the quotient ring.
pub fn hilbert_function_ty() -> Expr {
    arrow(cst("Ideal"), arrow(nat_ty(), nat_ty()))
}
/// `SyzygyModule : (s : ℕ) → Vec Polynomial s → Module` — the first syzygy module
/// Syz(f_1,…,f_s) = { (a_1,…,a_s) : ∑ a_i f_i = 0 }.
pub fn syzygy_module_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            nat_ty(),
            arrow(
                list_ty(app2(cst("Polynomial"), bvar(1), nat_ty())),
                cst("Module"),
            ),
        ),
    )
}
/// `BuchbergerCriterion : GroebnerBasis → Prop` — G is a Gröbner basis iff
/// every S-pair S(f, g) for f, g ∈ G reduces to 0 modulo G.
pub fn buchberger_criterion_ty() -> Expr {
    arrow(cst("GroebnerBasis"), prop())
}
/// `NullstellensatzCertificate : Polynomial → Ideal → Prop` — Hilbert's Nullstellensatz:
/// f ∈ √I iff ∃ k : ℕ, f^k ∈ I.
pub fn nullstellensatz_certificate_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(cst("Ideal"), prop()),
        ),
    )
}
/// `MinimalFreeResolution : Module → Type` — a minimal free resolution
/// 0 → F_n → … → F_1 → F_0 → M → 0.
pub fn minimal_free_resolution_ty() -> Expr {
    arrow(cst("Module"), type0())
}
/// `HilbertPolynomial : Ideal → Polynomial` — the Hilbert polynomial P(I, t)
/// such that H(I, t) = P(I, t) for all sufficiently large t.
pub fn hilbert_polynomial_ty() -> Expr {
    arrow(cst("Ideal"), app2(cst("Polynomial"), nat_ty(), nat_ty()))
}
/// Populate an OxiLean kernel `Environment` with Gröbner basis axioms.
pub fn build_groebner_bases_env(env: &mut Environment) {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("Ideal", || type0()),
        ("Module", || type1()),
        ("Field", || type1()),
        ("Ring", || type1()),
    ];
    for (name, mk_ty) in base_types {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("MonomialOrder", monomial_order_ty),
        ("Monomial", monomial_ty),
        ("Polynomial", polynomial_ty),
        ("GroebnerBasis", groebner_basis_ty),
        ("SPolynomial", s_polynomial_ty),
        ("IdealMembership", ideal_membership_ty),
        ("HilbertFunction", hilbert_function_ty),
        ("SyzygyModule", syzygy_module_ty),
        ("BuchbergerCriterion", buchberger_criterion_ty),
        ("NullstellensatzCertificate", nullstellensatz_certificate_ty),
        ("MinimalFreeResolution", minimal_free_resolution_ty),
        ("HilbertPolynomial", hilbert_polynomial_ty),
    ];
    for (name, mk_ty) in type_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
}
/// Greatest common divisor (Euclidean).
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Compute the S-polynomial of f and g:
/// S(f,g) = lcm(LM(f),LM(g))/LT(f) · f − lcm(LM(f),LM(g))/LT(g) · g.
pub fn s_polynomial(f: &Polynomial, g: &Polynomial) -> Polynomial {
    let zero = Polynomial::zero(f.nvars, f.order.clone());
    let lm_f = match f.leading_monomial() {
        Some(m) => m.clone(),
        None => return zero,
    };
    let lm_g = match g.leading_monomial() {
        Some(m) => m.clone(),
        None => return zero,
    };
    let (lc_f_num, lc_f_den) = f.leading_coeff().unwrap_or((1, 1));
    let (lc_g_num, lc_g_den) = g.leading_coeff().unwrap_or((1, 1));
    let lcm = lm_f.lcm(&lm_g);
    let gamma_f = lcm.div(&lm_f).unwrap_or_default();
    let gamma_g = lcm.div(&lm_g).unwrap_or_default();
    let f_scaled = f.mul_monomial(&gamma_f).scale(lc_f_den, lc_f_num);
    let g_scaled = g.mul_monomial(&gamma_g).scale(lc_g_den, lc_g_num);
    f_scaled.sub(&g_scaled)
}
/// Reduce polynomial `f` modulo a list of polynomials `divisors`.
///
/// Returns the remainder after multivariate division.
pub fn reduce(f: &Polynomial, divisors: &[Polynomial]) -> Polynomial {
    let mut remainder = Polynomial::zero(f.nvars, f.order.clone());
    let mut p = f.clone();
    while !p.is_zero() {
        let lm_p = p
            .leading_monomial()
            .expect("p is non-zero: checked by while loop condition")
            .clone();
        let lc_p = p
            .leading_coeff()
            .expect("p is non-zero: checked by while loop condition");
        let mut divided = false;
        for g in divisors {
            if let Some(lm_g) = g.leading_monomial() {
                if lm_g.divides(&lm_p) {
                    let quot_mon = lm_p.div(lm_g).unwrap_or_default();
                    let (lc_g_num, lc_g_den) = g.leading_coeff().unwrap_or((1, 1));
                    let scaled_g = g
                        .mul_monomial(&quot_mon)
                        .scale(lc_p.0 * lc_g_den, lc_p.1 * lc_g_num);
                    p = p.sub(&scaled_g);
                    divided = true;
                    break;
                }
            }
        }
        if !divided {
            let lt = p.terms[0].clone();
            let lt_poly = Polynomial {
                nvars: p.nvars,
                terms: vec![lt.clone()],
                order: p.order.clone(),
            };
            p = p.sub(&lt_poly);
            remainder = remainder.add(&Polynomial {
                nvars: remainder.nvars,
                terms: vec![lt],
                order: remainder.order.clone(),
            });
        }
    }
    remainder
}
/// Compute all monomials of total degree exactly `d` in `nvars` variables.
pub fn monomials_of_degree(nvars: usize, d: usize) -> Vec<Monomial> {
    if nvars == 0 {
        if d == 0 {
            return vec![Monomial::new(vec![])];
        }
        return vec![];
    }
    let mut result = Vec::new();
    fn gen(nvars: usize, d: usize, current: &mut Vec<u32>, result: &mut Vec<Monomial>) {
        if nvars == 1 {
            current.push(d as u32);
            result.push(Monomial::new(current.clone()));
            current.pop();
            return;
        }
        for e in 0..=d {
            current.push(e as u32);
            gen(nvars - 1, d - e, current, result);
            current.pop();
        }
    }
    gen(nvars, d, &mut Vec::new(), &mut result);
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    fn grlex(nvars: usize) -> MonomialOrder {
        let _ = nvars;
        MonomialOrder::GradedLex
    }
    fn poly_xy(nvars: usize) -> Polynomial {
        Polynomial {
            nvars,
            terms: vec![Term::new(Monomial::new(vec![1, 1]), 1)],
            order: MonomialOrder::GradedLex,
        }
    }
    fn poly_x2(nvars: usize) -> Polynomial {
        Polynomial {
            nvars,
            terms: vec![Term::new(Monomial::new(vec![2, 0]), 1)],
            order: MonomialOrder::GradedLex,
        }
    }
    #[test]
    fn test_monomial_degree() {
        let m = Monomial::new(vec![2, 3, 1]);
        assert_eq!(m.degree(), 6);
    }
    #[test]
    fn test_monomial_lcm() {
        let a = Monomial::new(vec![2, 1]);
        let b = Monomial::new(vec![1, 3]);
        let l = a.lcm(&b);
        assert_eq!(l.exponents, vec![2, 3]);
    }
    #[test]
    fn test_monomial_divides() {
        let a = Monomial::new(vec![1, 1]);
        let b = Monomial::new(vec![2, 3]);
        assert!(a.divides(&b));
        assert!(!b.divides(&a));
    }
    #[test]
    fn test_monomial_cmp_grlex() {
        let a = Monomial::new(vec![2, 0]);
        let b = Monomial::new(vec![1, 1]);
        assert_eq!(a.cmp_grlex(&b), std::cmp::Ordering::Greater);
    }
    #[test]
    fn test_polynomial_add() {
        let f = poly_xy(2);
        let g = poly_x2(2);
        let h = f.add(&g);
        assert_eq!(h.terms.len(), 2);
    }
    #[test]
    fn test_polynomial_is_zero() {
        let f = Polynomial::zero(2, MonomialOrder::Lex);
        assert!(f.is_zero());
    }
    #[test]
    fn test_s_polynomial_cancels() {
        let f = poly_x2(2);
        let g = poly_xy(2);
        let sp = s_polynomial(&f, &g);
        let lm_f_deg = f
            .leading_monomial()
            .expect("leading_monomial should succeed")
            .degree();
        let sp_deg = sp.leading_monomial().map_or(0, |m| m.degree());
        assert!(sp_deg < lm_f_deg || sp.is_zero());
    }
    #[test]
    fn test_buchberger_ideal_membership() {
        let order = MonomialOrder::GradedRevLex;
        let nvars = 3;
        let f1 = Polynomial {
            nvars,
            terms: vec![
                Term::new(Monomial::new(vec![2, 0, 0]), 1),
                Term::new(Monomial::new(vec![0, 1, 0]), -1),
            ],
            order: order.clone(),
        };
        let f2 = Polynomial {
            nvars,
            terms: vec![
                Term::new(Monomial::new(vec![3, 0, 0]), 1),
                Term::new(Monomial::new(vec![0, 0, 1]), -1),
            ],
            order: order.clone(),
        };
        let algo = BuchbergerAlgorithm::new(nvars, order.clone());
        let gb = algo.buchberger(vec![f1, f2]);
        let p_in = Polynomial {
            nvars,
            terms: vec![
                Term::new(Monomial::new(vec![0, 3, 0]), 1),
                Term::new(Monomial::new(vec![0, 0, 2]), -1),
            ],
            order: order.clone(),
        };
        let mem = IdealMembership::new(gb);
        assert!(mem.contains(&p_in));
    }
    #[test]
    fn test_buchberger_is_groebner_basis() {
        let order = MonomialOrder::GradedLex;
        let nvars = 2;
        let f = Polynomial {
            nvars,
            terms: vec![
                Term::new(Monomial::new(vec![2, 0]), 1),
                Term::new(Monomial::new(vec![0, 2]), 1),
                Term::new(Monomial::new(vec![0, 0]), -1),
            ],
            order: order.clone(),
        };
        let algo = BuchbergerAlgorithm::new(nvars, order.clone());
        let gb = algo.buchberger(vec![f]);
        assert!(gb.is_groebner_basis());
    }
    #[test]
    fn test_hilbert_function_projective_space() {
        let rgb = ReducedGroebnerBasis {
            generators: vec![],
            nvars: 3,
            order: MonomialOrder::GradedRevLex,
        };
        let hf = HilbertFunction::compute(&rgb, 4);
        assert_eq!(hf.at(0), 1);
        assert_eq!(hf.at(1), 3);
        assert_eq!(hf.at(2), 6);
    }
    #[test]
    fn test_syzygy_module_trivial() {
        let f = poly_x2(2);
        let sm = SyzygyModule::compute(&[f]);
        assert_eq!(sm.num_generators(), 0);
    }
    #[test]
    fn test_betti_numbers_regularity() {
        let mut b = BettiNumbers::new();
        b.set(0, 0, 1);
        b.set(1, 2, 2);
        b.set(2, 4, 1);
        assert_eq!(b.regularity(), Some(2));
    }
    #[test]
    fn test_build_groebner_env() {
        let mut env = Environment::new();
        build_groebner_bases_env(&mut env);
        assert!(env.get(&Name::str("Polynomial")).is_some());
        assert!(env.get(&Name::str("GroebnerBasis")).is_some());
        assert!(env.get(&Name::str("HilbertFunction")).is_some());
        assert!(env.get(&Name::str("SyzygyModule")).is_some());
        assert!(env.get(&Name::str("BuchbergerCriterion")).is_some());
    }
    #[test]
    fn test_monomial_display() {
        let m = Monomial::new(vec![2, 0, 1]);
        let s = format!("{}", m);
        assert!(s.contains("x_1"));
        assert!(s.contains("x_3"));
    }
    #[test]
    fn test_weight_order() {
        let w = MonomialOrder::Weight(vec![2, 1]);
        let a = Monomial::new(vec![1, 0]);
        let b = Monomial::new(vec![0, 3]);
        assert_eq!(a.cmp_with_order(&b, &w), std::cmp::Ordering::Less);
    }
    #[test]
    fn test_grlex_order() {
        let _ = grlex(2);
        let a = Monomial::new(vec![3, 0]);
        let b = Monomial::new(vec![2, 1]);
        assert_eq!(a.cmp_grlex(&b), std::cmp::Ordering::Greater);
    }
}
/// Build an `Environment` containing all Gröbner basis kernel axioms.
///
/// This is an alias for `build_groebner_bases_env` using the canonical name
/// expected by the OxiLean std module interface.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    build_groebner_bases_env(&mut env);
    env
}
/// `SPolynomialReductionToZero : GroebnerBasis → Prop`
/// — Buchberger criterion: G is a Gröbner basis iff every S-pair reduces to 0 mod G.
pub fn gb_ext_s_polynomial_reduction_to_zero_ty() -> Expr {
    arrow(cst("GroebnerBasis"), prop())
}
/// `CriticalPair : Polynomial → Polynomial → Type`
/// — a critical pair (f, g) producing an S-polynomial to be tested.
pub fn gb_ext_critical_pair_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(app2(cst("Polynomial"), bvar(1), nat_ty()), type0()),
        ),
    )
}
/// `GebauerMollerCriteria : GroebnerBasis → Prop`
/// — the Gebauer-Möller criteria for discarding unnecessary S-pairs in Buchberger.
pub fn gb_ext_gebauer_moller_criteria_ty() -> Expr {
    arrow(cst("GroebnerBasis"), prop())
}
/// `ProductCriterion : Polynomial → Polynomial → Prop`
/// — the product criterion: gcd(LM(f), LM(g)) = 1 implies S(f,g) reduces to 0.
pub fn gb_ext_product_criterion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(app2(cst("Polynomial"), bvar(1), nat_ty()), prop()),
        ),
    )
}
/// `ChainCriterion : Polynomial → Polynomial → Polynomial → Prop`
/// — the chain criterion: if h ∈ G with LM(h) | lcm(LM(f), LM(g)), discard S(f,g).
pub fn gb_ext_chain_criterion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(
                app2(cst("Polynomial"), bvar(1), nat_ty()),
                arrow(app2(cst("Polynomial"), bvar(2), nat_ty()), prop()),
            ),
        ),
    )
}
/// `FaugereF4Matrix : Type → Nat → Type`
/// — the Macaulay-style matrix used in Faugère's F4 algorithm.
pub fn gb_ext_faugere_f4_matrix_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `FaugereF4Reduction : Type → Nat → Type`
/// — the row reduction step in F4: reduces the Macaulay matrix to row echelon form.
pub fn gb_ext_faugere_f4_reduction_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `FaugereF4Algorithm : Type → GroebnerBasis`
/// — the F4 algorithm: Gröbner basis via simultaneous reduction of multiple S-pairs.
pub fn gb_ext_faugere_f4_algorithm_ty() -> Expr {
    arrow(type0(), cst("GroebnerBasis"))
}
/// `SignaturePolynomial : Polynomial → Type → Type`
/// — a signature-polynomial pair (sig, poly) as used in Faugère's F5 algorithm.
pub fn gb_ext_signature_polynomial_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(type0(), type0()),
        ),
    )
}
/// `FaugereF5Criterion : Type → Prop`
/// — the F5 criterion for discarding zero reductions (signature-based criterion).
pub fn gb_ext_faugere_f5_criterion_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FaugereF5Algorithm : Type → GroebnerBasis`
/// — the F5 algorithm: signature-based Gröbner basis with no redundant reductions.
pub fn gb_ext_faugere_f5_algorithm_ty() -> Expr {
    arrow(type0(), cst("GroebnerBasis"))
}
/// `ConstructiveNullstellensatz : Polynomial → Ideal → Nat → Prop`
/// — constructive Nullstellensatz: compute k and cofactors showing f^k ∈ I via Gröbner.
pub fn gb_ext_constructive_nullstellensatz_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(cst("Ideal"), arrow(nat_ty(), prop())),
        ),
    )
}
/// `WeakNullstellensatz : Ideal → Prop`
/// — the weak Nullstellensatz: I = (1) iff V(I) = ∅ over the algebraic closure.
pub fn gb_ext_weak_nullstellensatz_ty() -> Expr {
    arrow(cst("Ideal"), prop())
}
/// `RadicalMembership : Polynomial → Ideal → Prop`
/// — f ∈ √I tested via the Rabinowitsch trick: 1 ∈ (I, 1 - t·f).
pub fn gb_ext_radical_membership_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(cst("Ideal"), prop()),
        ),
    )
}
/// `AlgebraicVariety : Ideal → Type`
/// — the algebraic variety V(I) = {x ∈ k^n : f(x) = 0 for all f ∈ I}.
pub fn gb_ext_algebraic_variety_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `CoordinateRing : Ideal → Type`
/// — the coordinate ring k\[x\]/I of an affine variety V(I).
pub fn gb_ext_coordinate_ring_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `Dimension_Variety : Ideal → Nat`
/// — the Krull dimension of V(I) (= degree of Hilbert polynomial).
pub fn gb_ext_dimension_variety_ty() -> Expr {
    arrow(cst("Ideal"), nat_ty())
}
/// `Degree_Variety : Ideal → Nat`
/// — the degree of a projective variety (leading coefficient of Hilbert polynomial times dim!).
pub fn gb_ext_degree_variety_ty() -> Expr {
    arrow(cst("Ideal"), nat_ty())
}
/// `Resultant : Polynomial → Polynomial → Nat → Polynomial`
/// — the resultant Res_{x_i}(f, g) of two polynomials with respect to variable x_i.
pub fn gb_ext_resultant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(
                app2(cst("Polynomial"), bvar(1), nat_ty()),
                arrow(nat_ty(), app2(cst("Polynomial"), bvar(2), nat_ty())),
            ),
        ),
    )
}
/// `EliminationTheorem : Ideal → Nat → Prop`
/// — the Elimination theorem: the k-th elimination ideal I_k is generated by
/// polynomials in the Gröbner basis of I w.r.t. lex order involving only x_{k+1},…,x_n.
pub fn gb_ext_elimination_theorem_ty() -> Expr {
    arrow(cst("Ideal"), arrow(nat_ty(), prop()))
}
/// `ExtensionTheorem : Ideal → Nat → Prop`
/// — the Extension theorem: partial solutions of I_k extend to V(I) (with genericity conditions).
pub fn gb_ext_extension_theorem_ty() -> Expr {
    arrow(cst("Ideal"), arrow(nat_ty(), prop()))
}
/// `ImplicitizationProblemType : Nat → Nat → Type`
/// — the implicitization problem for a parametric map (t_1,…,t_k) → (x_1,…,x_n).
pub fn gb_ext_implicitization_problem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ImplicitEquation : Ideal → Type`
/// — the implicit equation(s) describing the closure of the image of a parametric map.
pub fn gb_ext_implicit_equation_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `ComprehensiveGroebnerBasis : Ideal → Type → Type`
/// — a comprehensive Gröbner basis (CGB) for a parametric ideal I(a) valid for all parameter values a.
pub fn gb_ext_comprehensive_groebner_basis_ty() -> Expr {
    arrow(cst("Ideal"), arrow(type0(), type0()))
}
/// `ParametricGroebnerBasis : Ideal → Type → Type`
/// — a parametric Gröbner system: a case split on parameter values giving a Gröbner basis.
pub fn gb_ext_parametric_groebner_basis_ty() -> Expr {
    arrow(cst("Ideal"), arrow(type0(), type0()))
}
/// `SAGBIBasis : Type → Type`
/// — a SAGBI (Subalgebra Analogue of Gröbner Bases for Ideals) basis for a subalgebra.
pub fn gb_ext_sagbi_basis_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SubductionAlgorithm : Polynomial → Type → Polynomial`
/// — the subduction algorithm for SAGBI: reduce a polynomial modulo a subalgebra.
pub fn gb_ext_subduction_algorithm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app2(cst("Polynomial"), bvar(0), nat_ty()),
            arrow(type0(), app2(cst("Polynomial"), bvar(1), nat_ty())),
        ),
    )
}
/// `JanetBasis : Ideal → Type`
/// — a Janet involutive basis for a polynomial ideal (triangular, complete).
pub fn gb_ext_janet_basis_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `PommaredBasis : Ideal → Type`
/// — a Pommaret involutive basis (uses a multiplicative variable decomposition).
pub fn gb_ext_pommaret_basis_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `InvolutiveDivision : Monomial → Type → Prop`
/// — an involutive division: assigns multiplicative variables to each monomial.
pub fn gb_ext_involutive_division_ty() -> Expr {
    arrow(cst("Monomial"), arrow(type0(), prop()))
}
/// `OrderIdeal : Ideal → Type`
/// — an order ideal O ⊆ T^n (closed under divisibility, disjoint from LT(I)).
pub fn gb_ext_order_ideal_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `BorderBasis : Ideal → Type → Type`
/// — a border basis G_O for an ideal I with respect to an order ideal O.
pub fn gb_ext_border_basis_ty() -> Expr {
    arrow(cst("Ideal"), arrow(type0(), type0()))
}
/// `BorderBasisCriterion : Type → Prop`
/// — the border basis criterion: G_O is a border basis iff it satisfies the neighbor condition.
pub fn gb_ext_border_basis_criterion_ty() -> Expr {
    arrow(type0(), prop())
}
/// `TropicalPolynomial : Nat → Type`
/// — a tropical polynomial: a polynomial over the tropical semiring (R ∪ {∞}, min, +).
pub fn gb_ext_tropical_polynomial_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TropicalVariety : Ideal → Type`
/// — the tropical variety Trop(I): the set of weight vectors w such that
/// the initial ideal in_w(I) contains no monomial.
pub fn gb_ext_tropical_variety_ty() -> Expr {
    arrow(cst("Ideal"), type0())
}
/// `TropicalGroebnerBasis : Ideal → Type → Type`
/// — a tropical Gröbner basis for an ideal I with respect to a weight vector w.
pub fn gb_ext_tropical_groebner_basis_ty() -> Expr {
    arrow(cst("Ideal"), arrow(type0(), type0()))
}
/// Register all extended Gröbner basis axioms into `env`.
pub fn register_groebner_bases_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "SPolynomialReductionToZero",
            gb_ext_s_polynomial_reduction_to_zero_ty(),
        ),
        ("CriticalPair", gb_ext_critical_pair_ty()),
        ("GebauerMollerCriteria", gb_ext_gebauer_moller_criteria_ty()),
        ("ProductCriterion", gb_ext_product_criterion_ty()),
        ("ChainCriterion", gb_ext_chain_criterion_ty()),
        ("FaugereF4Matrix", gb_ext_faugere_f4_matrix_ty()),
        ("FaugereF4Reduction", gb_ext_faugere_f4_reduction_ty()),
        ("FaugereF4Algorithm", gb_ext_faugere_f4_algorithm_ty()),
        ("SignaturePolynomial", gb_ext_signature_polynomial_ty()),
        ("FaugereF5Criterion", gb_ext_faugere_f5_criterion_ty()),
        ("FaugereF5Algorithm", gb_ext_faugere_f5_algorithm_ty()),
        (
            "ConstructiveNullstellensatz",
            gb_ext_constructive_nullstellensatz_ty(),
        ),
        ("WeakNullstellensatz", gb_ext_weak_nullstellensatz_ty()),
        ("RadicalMembership", gb_ext_radical_membership_ty()),
        ("AlgebraicVariety", gb_ext_algebraic_variety_ty()),
        ("CoordinateRing", gb_ext_coordinate_ring_ty()),
        ("Dimension_Variety", gb_ext_dimension_variety_ty()),
        ("Degree_Variety", gb_ext_degree_variety_ty()),
        ("Resultant", gb_ext_resultant_ty()),
        ("EliminationTheorem", gb_ext_elimination_theorem_ty()),
        ("ExtensionTheorem", gb_ext_extension_theorem_ty()),
        (
            "ImplicitizationProblemType",
            gb_ext_implicitization_problem_ty(),
        ),
        ("ImplicitEquation", gb_ext_implicit_equation_ty()),
        (
            "ComprehensiveGroebnerBasis",
            gb_ext_comprehensive_groebner_basis_ty(),
        ),
        (
            "ParametricGroebnerBasis",
            gb_ext_parametric_groebner_basis_ty(),
        ),
        ("SAGBIBasis", gb_ext_sagbi_basis_ty()),
        ("SubductionAlgorithm", gb_ext_subduction_algorithm_ty()),
        ("JanetBasis", gb_ext_janet_basis_ty()),
        ("PommaredBasis", gb_ext_pommaret_basis_ty()),
        ("InvolutiveDivision", gb_ext_involutive_division_ty()),
        ("OrderIdeal", gb_ext_order_ideal_ty()),
        ("BorderBasis", gb_ext_border_basis_ty()),
        ("BorderBasisCriterion", gb_ext_border_basis_criterion_ty()),
        ("TropicalPolynomial", gb_ext_tropical_polynomial_ty()),
        ("TropicalVariety", gb_ext_tropical_variety_ty()),
        ("TropicalGroebnerBasis", gb_ext_tropical_groebner_basis_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {}: {:?}", name, e))?;
    }
    Ok(())
}
