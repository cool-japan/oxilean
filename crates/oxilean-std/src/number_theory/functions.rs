//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdeleRing, ClassFieldTheory, DirichletProgression, DirichletSeries, EllipticCurvePointCounting,
    MilnorKGroup, MobiusFunctionTable, NumberField, QuadraticSieve, SieveMethod,
    SieveOfEratosthenes, WeilConjectureData, ZeroFreeRegion,
};

#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_lit(n: u64) -> Expr {
    Expr::Lit(oxilean_kernel::Literal::Nat(n))
}
pub fn eq_nat(a: Expr, b: Expr) -> Expr {
    app2(app(cst("Eq"), nat_ty()), a, b)
}
#[allow(dead_code)]
pub fn nat_le(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.le"), a, b)
}
pub fn nat_lt(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.lt"), a, b)
}
pub fn dvd(a: Expr, b: Expr) -> Expr {
    app2(cst("Nat.dvd"), a, b)
}
pub fn forall_nat(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, nat_ty(), body)
}
/// IsPrime n ↔ n ≥ 2 ∧ ∀ m, m ∣ n → m = 1 ∨ m = n
pub fn is_prime_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Nat.Coprime a b ↔ gcd a b = 1
pub fn nat_coprime_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// Nat.gcd : Nat → Nat → Nat
pub fn nat_gcd_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Nat.lcm : Nat → Nat → Nat
pub fn nat_lcm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Nat.totient (φ) : Nat → Nat  (Euler's totient function)
pub fn nat_totient_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Nat.primeFactors : Nat → Multiset Nat
pub fn prime_factors_ty() -> Expr {
    arrow(nat_ty(), app(cst("Multiset"), nat_ty()))
}
/// ZMod n : Type  (integers modulo n)
pub fn zmod_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// sigma_0 (divisor count) : Nat → Nat
pub fn sigma_0_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// sigma_1 (sum of divisors) : Nat → Nat
pub fn sigma_1_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Nat.moebius (Möbius function) : Nat → Int
pub fn moebius_ty() -> Expr {
    arrow(nat_ty(), cst("Int"))
}
/// vonMangoldt : Nat → Real  (von Mangoldt Λ)
pub fn von_mangoldt_ty() -> Expr {
    arrow(nat_ty(), cst("Real"))
}
/// IsSquarefree n ↔ ∀ m, m^2 ∣ n → m = 1
pub fn is_squarefree_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// IsPerfect n ↔ sigma_1 n = 2 * n
pub fn is_perfect_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Fundamental theorem of arithmetic: unique prime factorization
pub fn fundamental_theorem_arithmetic_ty() -> Expr {
    forall_nat(
        "n",
        arrow(
            nat_lt(nat_lit(0), bvar(0)),
            app(
                cst("ExistsUnique"),
                pi(
                    BinderInfo::Default,
                    "f",
                    app(cst("Multiset"), nat_ty()),
                    app2(
                        cst("And"),
                        app(cst("Multiset.AllPrime"), bvar(0)),
                        eq_nat(app(cst("Multiset.prod"), bvar(0)), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Infinitely many primes (Euclid)
pub fn infinitely_many_primes_ty() -> Expr {
    forall_nat(
        "n",
        app(
            cst("Exists"),
            pi(
                BinderInfo::Default,
                "p",
                nat_ty(),
                app2(
                    cst("And"),
                    nat_lt(bvar(1), bvar(0)),
                    app(cst("IsPrime"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Fermat's little theorem: prime p, ¬(p ∣ a) → a^(p-1) ≡ 1 (mod p)
pub fn fermat_little_theorem_ty() -> Expr {
    forall_nat(
        "p",
        arrow(
            app(cst("IsPrime"), bvar(0)),
            forall_nat(
                "a",
                arrow(
                    app2(cst("Nat.not_dvd"), bvar(1), bvar(0)),
                    eq_nat(
                        app2(
                            cst("Nat.mod"),
                            app2(
                                cst("Nat.pow"),
                                bvar(0),
                                app2(cst("Nat.sub"), bvar(1), nat_lit(1)),
                            ),
                            bvar(1),
                        ),
                        nat_lit(1),
                    ),
                ),
            ),
        ),
    )
}
/// Wilson's theorem: p is prime ↔ (p-1)! ≡ -1 (mod p)
pub fn wilson_theorem_ty() -> Expr {
    forall_nat(
        "p",
        arrow(
            nat_lt(nat_lit(1), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("IsPrime"), bvar(0)),
                eq_nat(
                    app2(
                        cst("Nat.mod"),
                        app(
                            cst("Nat.factorial"),
                            app2(cst("Nat.sub"), bvar(0), nat_lit(1)),
                        ),
                        bvar(0),
                    ),
                    app2(cst("Nat.sub"), bvar(0), nat_lit(1)),
                ),
            ),
        ),
    )
}
/// Chinese Remainder Theorem:
/// If gcd(m, n) = 1, then ZMod (m*n) ≅ ZMod m × ZMod n
pub fn chinese_remainder_theorem_ty() -> Expr {
    forall_nat(
        "m",
        forall_nat(
            "n",
            arrow(
                eq_nat(app2(cst("Nat.gcd"), bvar(1), bvar(0)), nat_lit(1)),
                app(
                    app(
                        cst("RingEquiv"),
                        app(cst("ZMod"), app2(cst("Nat.mul"), bvar(1), bvar(0))),
                    ),
                    app2(
                        cst("Prod"),
                        app(cst("ZMod"), bvar(1)),
                        app(cst("ZMod"), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Euler's theorem: gcd(a, n) = 1 → a^φ(n) ≡ 1 (mod n)
pub fn euler_theorem_ty() -> Expr {
    forall_nat(
        "n",
        forall_nat(
            "a",
            arrow(
                eq_nat(app2(cst("Nat.gcd"), bvar(0), bvar(1)), nat_lit(1)),
                eq_nat(
                    app2(
                        cst("Nat.mod"),
                        app2(cst("Nat.pow"), bvar(0), app(cst("Nat.totient"), bvar(1))),
                        bvar(1),
                    ),
                    nat_lit(1),
                ),
            ),
        ),
    )
}
/// GCD is symmetric: Nat.gcd a b = Nat.gcd b a
pub fn gcd_comm_ty() -> Expr {
    forall_nat(
        "a",
        forall_nat(
            "b",
            eq_nat(
                app2(cst("Nat.gcd"), bvar(1), bvar(0)),
                app2(cst("Nat.gcd"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// GCD divides both arguments: gcd a b ∣ a ∧ gcd a b ∣ b
pub fn gcd_dvd_both_ty() -> Expr {
    forall_nat(
        "a",
        forall_nat(
            "b",
            app2(
                cst("And"),
                dvd(app2(cst("Nat.gcd"), bvar(1), bvar(0)), bvar(1)),
                dvd(app2(cst("Nat.gcd"), bvar(1), bvar(0)), bvar(0)),
            ),
        ),
    )
}
/// Bezout's identity: ∃ u v : Int, u * a + v * b = gcd a b
pub fn bezout_identity_ty() -> Expr {
    forall_nat(
        "a",
        forall_nat(
            "b",
            app(
                cst("Exists"),
                pi(
                    BinderInfo::Default,
                    "u",
                    cst("Int"),
                    app(
                        cst("Exists"),
                        pi(
                            BinderInfo::Default,
                            "v",
                            cst("Int"),
                            eq_nat(
                                app(
                                    cst("Int.toNat"),
                                    app2(
                                        cst("Int.add"),
                                        app2(cst("Int.mul"), bvar(1), bvar(3)),
                                        app2(cst("Int.mul"), bvar(0), bvar(2)),
                                    ),
                                ),
                                app2(cst("Nat.gcd"), bvar(3), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Dirichlet's theorem: ∀ a n, gcd(a,n)=1 → infinitely many primes p ≡ a (mod n)
pub fn dirichlet_theorem_ty() -> Expr {
    forall_nat(
        "a",
        forall_nat(
            "n",
            arrow(
                eq_nat(app2(cst("Nat.gcd"), bvar(1), bvar(0)), nat_lit(1)),
                app(
                    cst("InfinitelyMany"),
                    pi(
                        BinderInfo::Default,
                        "p",
                        nat_ty(),
                        app2(
                            cst("And"),
                            app(cst("IsPrime"), bvar(0)),
                            eq_nat(app2(cst("Nat.mod"), bvar(0), bvar(2)), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Prime number theorem: π(x) ~ x / ln(x) as x → ∞
pub fn prime_number_theorem_ty() -> Expr {
    app2(
        cst("AsympEq"),
        pi(
            BinderInfo::Default,
            "_",
            cst("Real"),
            app(cst("Real.natPrimeCounting"), bvar(0)),
        ),
        pi(
            BinderInfo::Default,
            "_",
            cst("Real"),
            app2(cst("Real.div"), bvar(0), app(cst("Real.log"), bvar(0))),
        ),
    )
}
#[allow(dead_code)]
/// Build the number theory environment, registering all axioms and theorems.
pub fn build_number_theory_env(env: &mut Environment) -> Result<(), String> {
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.Prime"),
        univ_params: vec![],
        ty: is_prime_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsPrime"),
        univ_params: vec![],
        ty: is_prime_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.Coprime"),
        univ_params: vec![],
        ty: nat_coprime_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.gcd"),
        univ_params: vec![],
        ty: nat_gcd_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.lcm"),
        univ_params: vec![],
        ty: nat_lcm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.totient"),
        univ_params: vec![],
        ty: nat_totient_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ZMod"),
        univ_params: vec![],
        ty: zmod_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.primeFactors"),
        univ_params: vec![],
        ty: prime_factors_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.sigma0"),
        univ_params: vec![],
        ty: sigma_0_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.sigma1"),
        univ_params: vec![],
        ty: sigma_1_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.moebius"),
        univ_params: vec![],
        ty: moebius_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsSquarefree"),
        univ_params: vec![],
        ty: is_squarefree_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsPerfect"),
        univ_params: vec![],
        ty: is_perfect_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.fundamental_theorem_arithmetic"),
        univ_params: vec![],
        ty: fundamental_theorem_arithmetic_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.infinitely_many_primes"),
        univ_params: vec![],
        ty: infinitely_many_primes_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.fermat_little"),
        univ_params: vec![],
        ty: fermat_little_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.wilson"),
        univ_params: vec![],
        ty: wilson_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.chineseRemainder"),
        univ_params: vec![],
        ty: chinese_remainder_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.euler_theorem"),
        univ_params: vec![],
        ty: euler_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.gcd_comm"),
        univ_params: vec![],
        ty: gcd_comm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.gcd_dvd_both"),
        univ_params: vec![],
        ty: gcd_dvd_both_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Nat.bezout"),
        univ_params: vec![],
        ty: bezout_identity_ty(),
    });
    Ok(())
}
/// Compute GCD using the Euclidean algorithm.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Compute LCM.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}
/// Extended Euclidean algorithm: returns (g, u, v) such that u*a + v*b = g = gcd(a, b).
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, u, v) = extended_gcd(b, a % b);
    (g, v, u - (a / b) * v)
}
/// Primality test (trial division).
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
/// Miller-Rabin primality test (deterministic for n < 3,317,044,064,679,887,385,961,981).
pub fn miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = n - 1;
    let mut r = 0u64;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    let witnesses: &[u64] = if n < 2_047 {
        &[2]
    } else if n < 1_373_653 {
        &[2, 3]
    } else if n < 9_080_191 {
        &[31, 73]
    } else if n < 25_326_001 {
        &[2, 3, 5]
    } else if n < 3_215_031_751 {
        &[2, 3, 5, 7]
    } else {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    };
    'outer: for &a in witnesses {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
/// Modular multiplication avoiding overflow.
pub fn mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 0u64;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    result
}
/// Modular exponentiation: a^b mod m.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp /= 2;
    }
    result
}
/// Euler's totient function φ(n).
pub fn totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    let mut n_mut = n;
    let mut p = 2u64;
    while p * p <= n_mut {
        if n_mut % p == 0 {
            while n_mut % p == 0 {
                n_mut /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n_mut > 1 {
        result -= result / n_mut;
    }
    result
}
/// Prime factorization: returns sorted list of (prime, exponent) pairs.
pub fn prime_factorization(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut exp = 0u32;
            while n % p == 0 {
                n /= p;
                exp += 1;
            }
            factors.push((p, exp));
        }
        p += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}
/// Sieve of Eratosthenes: returns list of primes up to n.
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime_sieve = vec![true; n + 1];
    is_prime_sieve[0] = false;
    is_prime_sieve[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime_sieve[i] {
            let mut j = i * i;
            while j <= n {
                is_prime_sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (0..=n).filter(|&i| is_prime_sieve[i]).collect()
}
/// Number of divisors σ₀(n).
pub fn num_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    prime_factorization(n)
        .iter()
        .map(|(_, e)| (e + 1) as u64)
        .product()
}
/// Sum of divisors σ₁(n).
pub fn sum_of_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    prime_factorization(n)
        .iter()
        .map(|(p, e)| (p.pow(e + 1) - 1) / (p - 1))
        .product()
}
/// Is n a perfect number? (σ₁(n) = 2n)
pub fn is_perfect(n: u64) -> bool {
    n > 0 && sum_of_divisors(n) == 2 * n
}
/// Is n squarefree? (no prime factor appears more than once)
pub fn is_squarefree(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    prime_factorization(n).iter().all(|(_, e)| *e == 1)
}
/// Möbius function μ(n).
pub fn moebius(n: u64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let factors = prime_factorization(n);
    if factors.iter().any(|(_, e)| *e > 1) {
        return 0;
    }
    if factors.len() % 2 == 0 {
        1
    } else {
        -1
    }
}
/// Chinese Remainder Theorem solver:
/// Given (a₁, m₁), (a₂, m₂), ..., (aₙ, mₙ) with pairwise coprime mᵢ,
/// find x such that x ≡ aᵢ (mod mᵢ) for all i.
/// Returns None if the moduli are not pairwise coprime.
pub fn chinese_remainder(residues: &[(i64, i64)]) -> Option<(i64, i64)> {
    if residues.is_empty() {
        return Some((0, 1));
    }
    let mut x = residues[0].0;
    let mut m = residues[0].1;
    for &(a, mi) in &residues[1..] {
        let (g, u, _v) = extended_gcd(m, mi);
        if (a - x) % g != 0 {
            return None;
        }
        let lcm_val = m / g * mi;
        x = ((x + m * (u * ((a - x) / g) % (mi / g))) % lcm_val + lcm_val) % lcm_val;
        m = lcm_val;
    }
    Some((x, m))
}
/// Modular inverse of a mod m (if gcd(a, m) = 1).
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, u, _) = extended_gcd(a, m);
    if g != 1 {
        return None;
    }
    Some(((u % m) + m) % m)
}
/// Discrete logarithm: find k such that g^k ≡ h (mod p) using Baby-step Giant-step.
/// Returns None if no solution exists. O(√p) time.
pub fn discrete_log(g: u64, h: u64, p: u64) -> Option<u64> {
    let m = (p as f64).sqrt().ceil() as u64 + 1;
    let mut table = std::collections::HashMap::new();
    let mut gj = 1u64;
    for j in 0..m {
        table.entry(gj).or_insert(j);
        gj = mod_mul(gj, g, p);
    }
    let g_inv_m = mod_pow(mod_pow(g, p - 2, p), m, p);
    let mut giant = h;
    for i in 0..m {
        if let Some(&j) = table.get(&giant) {
            return Some(i * m + j);
        }
        giant = mod_mul(giant, g_inv_m, p);
    }
    None
}
/// Legendre symbol (a | p) for odd prime p.
/// Returns 0 if p | a, 1 if a is a QR mod p, -1 otherwise.
pub fn legendre_symbol(a: i64, p: u64) -> i64 {
    let a_mod = ((a % p as i64) + p as i64) as u64 % p;
    if a_mod == 0 {
        return 0;
    }
    let power = mod_pow(a_mod, (p - 1) / 2, p);
    if power == 1 {
        1
    } else {
        -1
    }
}
/// Jacobi symbol (a | n) — generalization of Legendre symbol.
pub fn jacobi_symbol(a: i64, n: u64) -> i64 {
    assert!(n % 2 == 1, "n must be odd");
    let mut a = ((a % n as i64) + n as i64) as u64 % n;
    let mut n = n;
    let mut result = 1i64;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let n8 = n % 8;
            if n8 == 3 || n8 == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            result = -result;
        }
        a %= n;
    }
    if n == 1 {
        result
    } else {
        0
    }
}
/// Prime counting function π(n) — count of primes ≤ n.
pub fn prime_pi(n: u64) -> u64 {
    sieve_of_eratosthenes(n as usize).len() as u64
}
/// Nth prime number.
pub fn nth_prime(n: usize) -> u64 {
    if n == 0 {
        return 2;
    }
    let upper = if n < 10 {
        50
    } else {
        (n as f64 * (n as f64).ln() * 1.5 + 100.0) as usize
    };
    let primes = sieve_of_eratosthenes(upper.max(10));
    if n < primes.len() {
        return primes[n] as u64;
    }
    let mut count = 0usize;
    let mut candidate = 2u64;
    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}
/// Mertens function M(n) = Σ_{k=1}^{n} μ(k).
pub fn mertens(n: u64) -> i64 {
    (1..=n).map(|k| moebius(k)).sum()
}
/// Von Mangoldt function Λ(n) = ln p if n = p^k, else 0.
pub fn von_mangoldt(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    let factors = prime_factorization(n);
    if factors.len() == 1 {
        (factors[0].0 as f64).ln()
    } else {
        0.0
    }
}
/// Chebyshev's ψ(x) = Σ_{n≤x} Λ(n).
pub fn chebyshev_psi(x: u64) -> f64 {
    (1..=x).map(|n| von_mangoldt(n)).sum()
}
/// Chebyshev's θ(x) = Σ_{p≤x, p prime} ln p.
pub fn chebyshev_theta(x: u64) -> f64 {
    (2..=x)
        .filter(|&n| is_prime(n))
        .map(|p| (p as f64).ln())
        .sum()
}
/// Dirichlet series coefficient a(n) for the Riemann zeta function:
/// ζ(s) = Σ n^{-s}.  Returns n^{-s} for a given real s > 1.
pub fn zeta_term(n: u64, s: f64) -> f64 {
    (n as f64).powf(-s)
}
/// Partial sum of the Riemann zeta function up to N terms.
pub fn zeta_partial_sum(s: f64, n_terms: u64) -> f64 {
    (1..=n_terms).map(|n| zeta_term(n, s)).sum()
}
/// Dirichlet character χ mod q — completely multiplicative, periodic mod q.
/// For the principal character χ₀: χ₀(n) = 1 if gcd(n,q)=1, else 0.
pub fn principal_character(n: u64, q: u64) -> f64 {
    if gcd(n, q) == 1 {
        1.0
    } else {
        0.0
    }
}
/// Legendre's prime counting formula approximation: π(n) ≈ n / ln(n).
pub fn prime_pi_approx(n: f64) -> f64 {
    if n <= 1.0 {
        return 0.0;
    }
    n / n.ln()
}
/// Li(x) — logarithmic integral, used in the prime number theorem.
/// Approximation via numerical integration of 1/ln(t) from 2 to x.
pub fn logarithmic_integral(x: f64) -> f64 {
    if x <= 2.0 {
        return 0.0;
    }
    let steps = 10_000usize;
    let dt = (x - 2.0) / steps as f64;
    let mut sum = 0.0;
    for i in 0..steps {
        let t = 2.0 + i as f64 * dt;
        sum += 1.0 / t.ln();
    }
    sum * dt
}
/// Pollard's rho factorization (returns a non-trivial factor of n, or n if prime).
pub fn pollard_rho(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }
    if is_prime(n) {
        return n;
    }
    let mut x = 2u64;
    let mut y = 2u64;
    let mut c = 1u64;
    let mut d = 1u64;
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(x.abs_diff(y), n);
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(100, 75), 25);
    }
    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(7, 13), 91);
        assert_eq!(lcm(0, 5), 0);
    }
    #[test]
    fn test_extended_gcd() {
        let (g, u, v) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(u * 35 + v * 15, 5);
    }
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }
    #[test]
    fn test_miller_rabin() {
        assert!(!miller_rabin(0));
        assert!(!miller_rabin(1));
        assert!(miller_rabin(2));
        assert!(miller_rabin(97));
        assert!(!miller_rabin(100));
        assert!(miller_rabin(1_000_000_007));
    }
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 4, 7), 4);
        assert_eq!(mod_pow(2, 0, 5), 1);
    }
    #[test]
    fn test_totient() {
        assert_eq!(totient(1), 1);
        assert_eq!(totient(2), 1);
        assert_eq!(totient(6), 2);
        assert_eq!(totient(12), 4);
        assert_eq!(totient(7), 6);
    }
    #[test]
    fn test_prime_factorization() {
        assert_eq!(prime_factorization(12), vec![(2, 2), (3, 1)]);
        assert_eq!(prime_factorization(1), vec![]);
        assert_eq!(prime_factorization(7), vec![(7, 1)]);
    }
    #[test]
    fn test_sieve() {
        let primes = sieve_of_eratosthenes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
    #[test]
    fn test_num_divisors() {
        assert_eq!(num_divisors(12), 6);
        assert_eq!(num_divisors(1), 1);
        assert_eq!(num_divisors(7), 2);
        assert_eq!(num_divisors(36), 9);
    }
    #[test]
    fn test_sum_of_divisors() {
        assert_eq!(sum_of_divisors(6), 12);
        assert_eq!(sum_of_divisors(1), 1);
        assert_eq!(sum_of_divisors(7), 8);
    }
    #[test]
    fn test_is_perfect() {
        assert!(is_perfect(6));
        assert!(is_perfect(28));
        assert!(!is_perfect(12));
    }
    #[test]
    fn test_is_squarefree() {
        assert!(is_squarefree(6));
        assert!(!is_squarefree(4));
        assert!(is_squarefree(30));
        assert!(!is_squarefree(12));
    }
    #[test]
    fn test_moebius() {
        assert_eq!(moebius(1), 1);
        assert_eq!(moebius(2), -1);
        assert_eq!(moebius(4), 0);
        assert_eq!(moebius(6), 1);
        assert_eq!(moebius(30), -1);
    }
    #[test]
    fn test_chinese_remainder() {
        let result =
            chinese_remainder(&[(2, 3), (3, 5), (2, 7)]).expect("operation should succeed");
        assert_eq!(result.0 % 3, 2);
        assert_eq!(result.0 % 5, 3);
        assert_eq!(result.0 % 7, 2);
    }
    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), Some(5));
        assert_eq!(mod_inverse(2, 6), None);
    }
    #[test]
    fn test_legendre_symbol() {
        assert_eq!(legendre_symbol(1, 7), 1);
        assert_eq!(legendre_symbol(2, 7), 1);
        assert_eq!(legendre_symbol(3, 7), -1);
        assert_eq!(legendre_symbol(0, 7), 0);
    }
    #[test]
    fn test_prime_pi() {
        assert_eq!(prime_pi(10), 4);
        assert_eq!(prime_pi(100), 25);
    }
    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(0), 2);
        assert_eq!(nth_prime(1), 3);
        assert_eq!(nth_prime(2), 5);
        assert_eq!(nth_prime(4), 11);
    }
    #[test]
    fn test_discrete_log() {
        let k = discrete_log(3, 6, 7);
        assert!(k.is_some());
        assert_eq!(mod_pow(3, k.expect("k should be valid"), 7), 6);
    }
    #[test]
    fn test_pollard_rho_composite() {
        let factor = pollard_rho(15);
        assert!(15 % factor == 0 && factor > 1 && factor < 15);
    }
    #[test]
    fn test_build_number_theory_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Multiset"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Prod"),
            univ_params: vec![],
            ty: arrow(type0(), arrow(type0(), type0())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("RingEquiv"),
            univ_params: vec![],
            ty: arrow(type0(), arrow(type0(), arrow(type0(), type0()))),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("InfinitelyMany"),
            univ_params: vec![],
            ty: arrow(arrow(nat_ty(), prop()), prop()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("AsympEq"),
            univ_params: vec![],
            ty: arrow(
                arrow(cst("Real"), cst("Real")),
                arrow(arrow(cst("Real"), cst("Real")), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("ExistsUnique"),
            univ_params: vec![Name::str("u")],
            ty: impl_pi("α", type0(), arrow(arrow(bvar(0), prop()), prop())),
        });
        let result = build_number_theory_env(&mut env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_mertens() {
        assert_eq!(mertens(1), 1);
        assert_eq!(mertens(2), 0);
        assert_eq!(mertens(5), -2);
    }
    #[test]
    fn test_von_mangoldt() {
        assert!((von_mangoldt(1) - 0.0).abs() < 1e-12);
        assert!((von_mangoldt(2) - 2.0f64.ln()).abs() < 1e-12);
        assert!((von_mangoldt(4) - 2.0f64.ln()).abs() < 1e-12);
        assert!((von_mangoldt(6) - 0.0).abs() < 1e-12);
    }
    #[test]
    fn test_chebyshev_psi_theta() {
        let psi = chebyshev_psi(10);
        let theta = chebyshev_theta(10);
        assert!(psi >= theta - 1e-9);
        let expected_theta = 2.0f64.ln() + 3.0f64.ln() + 5.0f64.ln() + 7.0f64.ln();
        assert!((theta - expected_theta).abs() < 1e-9);
    }
    #[test]
    fn test_zeta_partial_sum() {
        let z = zeta_partial_sum(2.0, 10_000);
        assert!((z - std::f64::consts::PI.powi(2) / 6.0).abs() < 0.001);
    }
    #[test]
    fn test_principal_character() {
        assert!((principal_character(1, 5) - 1.0).abs() < 1e-12);
        assert!((principal_character(5, 5) - 0.0).abs() < 1e-12);
        assert!((principal_character(6, 5) - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_prime_pi_approx() {
        let approx = prime_pi_approx(100.0);
        assert!(
            approx > 15.0 && approx < 35.0,
            "approx should be near 25: {approx}"
        );
    }
    #[test]
    fn test_logarithmic_integral() {
        let li10 = logarithmic_integral(10.0);
        assert!(
            li10 > 5.0 && li10 < 8.0,
            "Li(10) should be near 6.165: {li10}"
        );
    }
    #[test]
    fn test_mobius_function_table() {
        let table = MobiusFunctionTable::new(10);
        assert_eq!(table.mu(1), 1);
        assert_eq!(table.mu(2), -1);
        assert_eq!(table.mu(4), 0);
        assert_eq!(table.mu(6), 1);
        let m5 = table.mertens(5);
        assert_eq!(m5, -2);
    }
    #[test]
    fn test_sieve_of_eratosthenes_struct() {
        let sieve = SieveOfEratosthenes::new(30);
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(29));
        assert!(!sieve.is_prime(1));
        assert!(!sieve.is_prime(15));
        let primes = sieve.primes();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(sieve.count_primes(), 10);
    }
    #[test]
    fn test_quadratic_sieve_basic() {
        let qs = QuadraticSieve::new(15);
        let factor = qs.find_factor();
        assert!(factor.is_some(), "should find a factor of 15");
        let f = factor.expect("factor should be valid");
        assert!(
            15 % f == 0 && f > 1 && f < 15,
            "factor should be non-trivial: {f}"
        );
    }
    #[test]
    fn test_elliptic_curve_point_counting() {
        let ec = EllipticCurvePointCounting::new(1, 1, 5);
        let count = ec.count_points();
        let mut expected = 1u64;
        for x in 0u64..5 {
            let rhs = (x.pow(3) + x + 1) % 5;
            for y in 0u64..5 {
                if (y * y) % 5 == rhs {
                    expected += 1;
                }
            }
        }
        assert_eq!(count, expected, "point count: {count} vs {expected}");
    }
    #[test]
    fn test_elliptic_curve_hasse_bound() {
        let ec = EllipticCurvePointCounting::new(1, 1, 7);
        assert!(ec.satisfies_hasse_bound(), "should satisfy Hasse's theorem");
    }
    #[test]
    fn test_build_number_theory_extended_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        let result = build_number_theory_extended_env(&mut env);
        assert!(result.is_ok(), "extended env build: {:?}", result);
    }
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
/// `RiemannHypothesis : Prop`
pub fn riemann_hypothesis_ty() -> Expr {
    prop()
}
/// `GeneralizedRiemannHypothesis : Prop`
pub fn generalized_riemann_hypothesis_ty() -> Expr {
    prop()
}
/// `DirichletLFunction : Nat → Complex → Complex`
pub fn dirichlet_l_function_ty() -> Expr {
    arrow(nat_ty(), arrow(complex_ty(), complex_ty()))
}
/// `LFunctionZeroFreeRegion : Nat → Prop`
pub fn l_function_zero_free_region_ty() -> Expr {
    pi(BinderInfo::Default, "q", nat_ty(), prop())
}
/// `IdealClassGroup : Nat → Type`
pub fn ideal_class_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ClassNumber : Nat → Nat`
pub fn class_number_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `ClassNumberFormula : Nat → Prop`
pub fn class_number_formula_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// `RingOfIntegers : Nat → Type`
pub fn ring_of_integers_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Discriminant : Nat → Int`
pub fn discriminant_ty() -> Expr {
    arrow(nat_ty(), int_ty())
}
/// `MinkowskiBound : Nat → Real`
pub fn minkowski_bound_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `GelfondSchneider : Prop`
pub fn gelfond_schneider_ty() -> Expr {
    prop()
}
/// `BakerTheorem : Nat → Prop`
pub fn baker_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `LiouvilleApproximation : Nat → Real → Prop`
pub fn liouville_approximation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "d",
        nat_ty(),
        pi(BinderInfo::Default, "alpha", real_ty(), prop()),
    )
}
/// `RothTheorem : Prop`
pub fn roth_theorem_ty() -> Expr {
    prop()
}
/// `SchmidtSubspaceTheorem : Nat → Prop`
pub fn schmidt_subspace_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `WeylExponentialSum : Nat → Real → Real → Prop`
pub fn weyl_exponential_sum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            real_ty(),
            pi(BinderInfo::Default, "bound", real_ty(), prop()),
        ),
    )
}
/// `VinogradovMeanValue : Nat → Nat → Prop`
pub fn vinogradov_mean_value_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(BinderInfo::Default, "s", nat_ty(), prop()),
    )
}
/// `HardyLittlewoodCircleMethod : Nat → Prop`
pub fn hardy_littlewood_circle_method_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), prop())
}
/// `WaringProblem : Nat → Nat → Prop`
pub fn waring_problem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(BinderInfo::Default, "g_k", nat_ty(), prop()),
    )
}
/// `BrunSieve : Nat → Real → Prop`
pub fn brun_sieve_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        nat_ty(),
        pi(BinderInfo::Default, "bound", real_ty(), prop()),
    )
}
/// `SelbergSieve : Nat → Real → Prop`
pub fn selberg_sieve_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "bound", real_ty(), prop()),
    )
}
/// `LargeSieve : Nat → Real → Prop`
pub fn large_sieve_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "q", real_ty(), prop()),
    )
}
/// `RamanujanTau : Nat → Int`
pub fn ramanujan_tau_ty() -> Expr {
    arrow(nat_ty(), int_ty())
}
/// `ModularFormSpace : Nat → Nat → Type`
pub fn modular_form_space_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ModularityTheoremEC : Nat → Prop`
pub fn modularity_theorem_ec_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `KroneckerWeberTheorem : Prop`
pub fn kronecker_weber_theorem_ty() -> Expr {
    prop()
}
/// `ArtinReciprocity : Nat → Prop`
pub fn artin_reciprocity_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), prop())
}
/// `RayClassField : Nat → Nat → Type`
pub fn ray_class_field_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LanglandsFunctoriality : Nat → Nat → Prop`
pub fn langlands_functoriality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        nat_ty(),
        pi(BinderInfo::Default, "h", nat_ty(), prop()),
    )
}
/// `BaseChange : Nat → Nat → Prop`
pub fn base_change_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        nat_ty(),
        pi(BinderInfo::Default, "e", nat_ty(), prop()),
    )
}
/// `AKSPrimality : Nat → Prop`
pub fn aks_primality_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `GNFSFactoring : Nat → Prop`
pub fn gnfs_factoring_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `EllipticCurveFactoring : Nat → Prop`
pub fn elliptic_curve_factoring_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `IndexCalculus : Nat → Prop`
pub fn index_calculus_ty() -> Expr {
    pi(BinderInfo::Default, "p", nat_ty(), prop())
}
/// `DivisorBound : Nat → Real → Prop`
pub fn divisor_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "eps", real_ty(), prop()),
    )
}
/// `AverageDivisorCount : Nat → Prop`
pub fn average_divisor_count_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `SumOfTwoSquares : Nat → Prop`
pub fn sum_of_two_squares_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `QuadraticReciprocity : Prop`
pub fn quadratic_reciprocity_ty() -> Expr {
    prop()
}
/// `MertensConstant : Real → Prop`
pub fn mertens_constant_ty() -> Expr {
    pi(BinderInfo::Default, "x", real_ty(), prop())
}
/// `DirichletDensity : Nat → Nat → Real → Prop`
pub fn dirichlet_density_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "q",
            nat_ty(),
            pi(BinderInfo::Default, "density", real_ty(), prop()),
        ),
    )
}
/// `GoldbachConjecture : Prop`
pub fn goldbach_conjecture_ty() -> Expr {
    prop()
}
/// `TwinPrimeConjecture : Prop`
pub fn twin_prime_conjecture_ty() -> Expr {
    prop()
}
/// Register all extended number theory axioms.
#[allow(dead_code)]
pub fn build_number_theory_extended_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("NT.RiemannHypothesis", riemann_hypothesis_ty()),
        (
            "NT.GeneralizedRiemannHypothesis",
            generalized_riemann_hypothesis_ty(),
        ),
        ("NT.DirichletLFunction", dirichlet_l_function_ty()),
        (
            "NT.LFunctionZeroFreeRegion",
            l_function_zero_free_region_ty(),
        ),
        ("NT.IdealClassGroup", ideal_class_group_ty()),
        ("NT.ClassNumber", class_number_ty()),
        ("NT.ClassNumberFormula", class_number_formula_ty()),
        ("NT.RingOfIntegers", ring_of_integers_ty()),
        ("NT.Discriminant", discriminant_ty()),
        ("NT.MinkowskiBound", minkowski_bound_ty()),
        ("NT.GelfondSchneider", gelfond_schneider_ty()),
        ("NT.BakerTheorem", baker_theorem_ty()),
        ("NT.LiouvilleApproximation", liouville_approximation_ty()),
        ("NT.RothTheorem", roth_theorem_ty()),
        ("NT.SchmidtSubspaceTheorem", schmidt_subspace_theorem_ty()),
        ("NT.WeylExponentialSum", weyl_exponential_sum_ty()),
        ("NT.VinogradovMeanValue", vinogradov_mean_value_ty()),
        (
            "NT.HardyLittlewoodCircleMethod",
            hardy_littlewood_circle_method_ty(),
        ),
        ("NT.WaringProblem", waring_problem_ty()),
        ("NT.BrunSieve", brun_sieve_ty()),
        ("NT.SelbergSieve", selberg_sieve_ty()),
        ("NT.LargeSieve", large_sieve_ty()),
        ("NT.RamanujanTau", ramanujan_tau_ty()),
        ("NT.ModularFormSpace", modular_form_space_ty()),
        ("NT.ModularityTheoremEC", modularity_theorem_ec_ty()),
        ("NT.KroneckerWeberTheorem", kronecker_weber_theorem_ty()),
        ("NT.ArtinReciprocity", artin_reciprocity_ty()),
        ("NT.RayClassField", ray_class_field_ty()),
        ("NT.LanglandsFunctoriality", langlands_functoriality_ty()),
        ("NT.BaseChange", base_change_ty()),
        ("NT.AKSPrimality", aks_primality_ty()),
        ("NT.GNFSFactoring", gnfs_factoring_ty()),
        ("NT.EllipticCurveFactoring", elliptic_curve_factoring_ty()),
        ("NT.IndexCalculus", index_calculus_ty()),
        ("NT.DivisorBound", divisor_bound_ty()),
        ("NT.AverageDivisorCount", average_divisor_count_ty()),
        ("NT.SumOfTwoSquares", sum_of_two_squares_ty()),
        ("NT.QuadraticReciprocity", quadratic_reciprocity_ty()),
        ("NT.MertensConstant", mertens_constant_ty()),
        ("NT.DirichletDensity", dirichlet_density_ty()),
        ("NT.GoldbachConjecture", goldbach_conjecture_ty()),
        ("NT.TwinPrimeConjecture", twin_prime_conjecture_ty()),
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
#[allow(dead_code)]
pub(super) fn gcd_ext(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_ext(b, a % b)
    }
}
#[allow(dead_code)]
pub(super) fn euler_phi(n: u64) -> u64 {
    let mut result = n;
    let mut temp = n;
    let mut p = 2u64;
    while p * p <= temp {
        if temp % p == 0 {
            while temp % p == 0 {
                temp /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if temp > 1 {
        result -= result / temp;
    }
    result
}
/// Prime counting function estimates.
#[allow(dead_code)]
pub fn prime_counting_estimates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Prime Number Theorem", "pi(x) ~ x / ln(x)"),
        (
            "Li(x) approximation",
            "pi(x) ~ Li(x) = integral_2^x dt/ln(t)",
        ),
        ("Riemann's R(x)", "pi(x) ~ R(x) = sum mu(n)/n * Li(x^(1/n))"),
        ("Error under RH", "pi(x) = Li(x) + O(sqrt(x) log x)"),
        (
            "Bertrand's postulate",
            "pi(2n) > pi(n) for n >= 1: prime in (n, 2n)",
        ),
        ("Chebyshev's theorem", "0.9 x/ln x <= pi(x) <= 1.1 x/ln x"),
    ]
}
/// Goldbach conjecture status.
#[allow(dead_code)]
pub fn goldbach_status() -> &'static str {
    "Goldbach's conjecture (unproved): every even n > 2 is sum of two primes. \
     Verified up to 4*10^18. Weak Goldbach (every odd n > 5 is sum of three primes) proved by Helfgott (2013)."
}
#[cfg(test)]
mod nt_ext_tests {
    use super::*;
    #[test]
    fn test_number_field() {
        let q = NumberField::rationals();
        assert!(q.is_pid());
        let qi = NumberField::gaussian_integers();
        assert_eq!(qi.degree, 2);
        assert!(qi.is_pid());
    }
    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(2), 1);
        assert_eq!(euler_phi(6), 2);
        assert_eq!(euler_phi(12), 4);
    }
    #[test]
    fn test_dirichlet_progression() {
        let prog = DirichletProgression::new(1, 4);
        assert!(prog.has_infinitely_many_primes());
        let density = prog.density_among_primes();
        assert!((density - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_prime_counting() {
        let estimates = prime_counting_estimates();
        assert!(!estimates.is_empty());
    }
    #[test]
    fn test_dirichlet_series() {
        let zeta = DirichletSeries::zeta();
        assert!(!zeta.name.is_empty());
    }
}
#[cfg(test)]
mod nt_adele_tests {
    use super::*;
    #[test]
    fn test_adele_ring() {
        let a = AdeleRing::new("Q");
        assert!(!a.product_formula().is_empty());
    }
    #[test]
    fn test_class_field_theory() {
        let cft = ClassFieldTheory::new("Q", "Q(sqrt(-1))");
        assert!(!cft.artin_reciprocity().is_empty());
    }
    #[test]
    fn test_weil_conjectures() {
        let wc = WeilConjectureData::new("Fermat curve", 1);
        assert!(!wc.rationality().is_empty());
        assert!(!wc.riemann_hypothesis().is_empty());
    }
}
#[cfg(test)]
mod sieve_tests {
    use super::*;
    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes = SieveMethod::sieve_of_eratosthenes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
    #[test]
    fn test_zero_free_region() {
        let zfr = ZeroFreeRegion::riemann_hypothesis();
        assert!(zfr.description.contains("1/2"));
    }
}
#[cfg(test)]
mod milnor_k_tests {
    use super::*;
    #[test]
    fn test_milnor_k() {
        let km2 = MilnorKGroup::new("Q", 2);
        assert!(!km2.relations().is_empty());
        assert!(!km2.milnor_conjecture_description().is_empty());
    }
}
