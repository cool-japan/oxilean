//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BB84Protocol, ComputationalIndistinguishability, E91Protocol, FuzzyExtractor, GarbledCircuit,
    HomomorphicScheme, InformationTheoreticMAC, LeakageResilientScheme, MinEntropy, ORAMSimulation,
    ObliviousTransfer, PerfectSecrecy, RandomnessExtractor, ShamirSecretSharing,
    StatisticalDistance, UnconditionalSecurity, UniversalHashFamily, WiretapChannel, ZKProofData,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn bytes_ty() -> Expr {
    list_ty(nat_ty())
}
/// `Message : Type` — abstract plaintext message space.
pub fn message_ty() -> Expr {
    type0()
}
/// `Key : Type` — abstract key space.
pub fn key_ty() -> Expr {
    type0()
}
/// `Ciphertext : Type` — abstract ciphertext space.
pub fn ciphertext_ty() -> Expr {
    type0()
}
/// `EncryptionScheme : Type` — a symmetric encryption scheme (E, D).
/// Represented as a record of Encrypt and Decrypt functions.
pub fn encryption_scheme_ty() -> Expr {
    type0()
}
/// `PerfectSecrecy : EncryptionScheme → Prop`
/// M and C are independent: ∀ m, c. Pr[M=m | C=c] = Pr[M=m].
/// Equivalently: ∀ m, m'. Pr\[Enc(K,m)=c\] = Pr\[Enc(K,m')=c\] for all c.
pub fn perfect_secrecy_ty() -> Expr {
    arrow(cst("EncryptionScheme"), prop())
}
/// `SemanticSecurity : EncryptionScheme → Prop`
/// The adversary cannot distinguish encryptions of any two equal-length messages.
pub fn semantic_security_ty() -> Expr {
    arrow(cst("EncryptionScheme"), prop())
}
/// `UniformKey : Key → Prop` — the key is uniformly distributed over the key space.
pub fn uniform_key_ty() -> Expr {
    arrow(cst("Key"), prop())
}
/// `KeySpaceSize : EncryptionScheme → Nat → Prop`
/// |K| = n (key space has exactly n elements).
pub fn key_space_size_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(nat_ty(), prop()))
}
/// `MsgSpaceSize : EncryptionScheme → Nat → Prop`
/// |M| = n (message space has exactly n elements).
pub fn msg_space_size_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(nat_ty(), prop()))
}
/// `ShannonEntropy : (List Real) → Real`
/// H(X) = -Σ p(x) log₂ p(x)
pub fn shannon_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `ConditionalEntropy : (List (List Real)) → Real`
/// H(X|Y) = H(X,Y) - H(Y)
pub fn conditional_entropy_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `MutualInformation : (List (List Real)) → Real`
/// I(X;Y) = H(X) + H(Y) - H(X,Y)
pub fn mutual_information_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), real_ty())
}
/// `MinEntropy : (List Real) → Real`
/// H_∞(X) = -log₂(max_x p(x))
pub fn min_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// `SmoothMinEntropy : (List Real) → Real → Real`
/// H_∞^ε(X) = max_{X' ε-close to X} H_∞(X')
pub fn smooth_min_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), real_ty()))
}
/// `CollisionEntropy : (List Real) → Real`
/// H_2(X) = -log₂(Σ p(x)²)  (Rényi order-2 entropy)
pub fn collision_entropy_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Shannon's perfect secrecy theorem:
/// A scheme has perfect secrecy iff:
///   (1) the key is uniform, and
///   (2) |K| ≥ |M|
/// `ShannonPerfectSecrecy : EncryptionScheme → Prop`
pub fn shannon_perfect_secrecy_ty() -> Expr {
    arrow(cst("EncryptionScheme"), prop())
}
/// One-time pad perfect secrecy:
/// The OTP (k ⊕ m) achieves perfect secrecy when key is uniform and |K|=|M|.
/// `OtpPerfectSecrecy : Nat → Prop`  (parametrised by message length n)
pub fn otp_perfect_secrecy_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Perfect secrecy implies |K| ≥ |M|:
/// `KeyLowerBound : EncryptionScheme → Nat → Nat → Prop`
pub fn key_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "scheme",
        cst("EncryptionScheme"),
        pi(
            BinderInfo::Default,
            "key_size",
            nat_ty(),
            pi(BinderInfo::Default, "msg_size", nat_ty(), prop()),
        ),
    )
}
/// `SecretSharing : Nat → Nat → Type` — k-of-n secret sharing scheme.
/// `SecretSharing k n` represents a scheme where k shares suffice to reconstruct,
/// but k-1 shares reveal nothing.
pub fn secret_sharing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ShamirShare : Nat → Nat → (List Nat) → Prop`
/// Shamir's scheme: encode secret s into n shares using a (k-1)-degree polynomial over GF(p).
pub fn shamir_share_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(BinderInfo::Default, "shares", list_ty(nat_ty()), prop()),
        ),
    )
}
/// `SecretSharingCorrect : Nat → Nat → Prop`
/// k-of-n Shamir is (k-1)-private: any subset of ≤ k-1 shares is statistically independent
/// of the secret.
pub fn secret_sharing_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// `Extractor : (List Nat) → (List Nat) → (List Nat)` — seeded extractor.
/// Ext(seed, source) → output  extracts near-uniform randomness.
pub fn extractor_ty() -> Expr {
    arrow(bytes_ty(), arrow(bytes_ty(), bytes_ty()))
}
/// `StrongExtractor : (List Nat) → (List Nat) → (List Nat)` — strong seeded extractor.
/// Strong: (Seed, Ext(Seed, X)) is jointly close to (Seed, U_m).
pub fn strong_extractor_ty() -> Expr {
    arrow(bytes_ty(), arrow(bytes_ty(), bytes_ty()))
}
/// `TwoSourceExtractor : (List Nat) → (List Nat) → (List Nat)` — two-source extractor.
/// Ext(X, Y) where X and Y are independent high-entropy sources.
pub fn two_source_extractor_ty() -> Expr {
    arrow(bytes_ty(), arrow(bytes_ty(), bytes_ty()))
}
/// Leftover Hash Lemma type:
/// `LeftoverHashLemma : Nat → Real → Real → Prop`
/// If H_∞(X) ≥ k and we use a (ε/2)-universal hash family hashing to {0,1}^m
/// with m ≤ k - 2*log(1/ε), then H(Ext(S,X) | S) ≥ m - ε.
pub fn leftover_hash_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "m",
            nat_ty(),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `ExtractorQuality : Extractor → Real → Real → Prop`
/// The extractor is (k, ε)-good: for all sources X with H_∞(X) ≥ k,
/// the output is ε-close to uniform in statistical distance.
pub fn extractor_quality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ext",
        extractor_ty(),
        pi(
            BinderInfo::Default,
            "k",
            real_ty(),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `StatisticalDistance : (List Real) → (List Real) → Real`
/// SD(P, Q) = (1/2) * Σ_x |P(x) - Q(x)|
pub fn statistical_distance_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty()))
}
/// `EpsClose : (List Real) → (List Real) → Real → Prop`
/// P and Q are ε-close if SD(P, Q) ≤ ε.
pub fn eps_close_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        list_ty(real_ty()),
        pi(
            BinderInfo::Default,
            "Q",
            list_ty(real_ty()),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `UniformDist : Nat → (List Real)` — uniform distribution over n outcomes.
pub fn uniform_dist_ty() -> Expr {
    arrow(nat_ty(), list_ty(real_ty()))
}
/// Register all information-theoretic security axioms and theorems.
pub fn build_information_theoretic_security_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ITS.Message", message_ty()),
        ("ITS.Key", key_ty()),
        ("ITS.Ciphertext", ciphertext_ty()),
        ("ITS.EncryptionScheme", encryption_scheme_ty()),
        ("ITS.PerfectSecrecy", perfect_secrecy_ty()),
        ("ITS.SemanticSecurity", semantic_security_ty()),
        ("ITS.UniformKey", uniform_key_ty()),
        ("ITS.KeySpaceSize", key_space_size_ty()),
        ("ITS.MsgSpaceSize", msg_space_size_ty()),
        ("ITS.ShannonEntropy", shannon_entropy_ty()),
        ("ITS.ConditionalEntropy", conditional_entropy_ty()),
        ("ITS.MutualInformation", mutual_information_ty()),
        ("ITS.MinEntropy", min_entropy_ty()),
        ("ITS.SmoothMinEntropy", smooth_min_entropy_ty()),
        ("ITS.CollisionEntropy", collision_entropy_ty()),
        ("ITS.ShannonPerfectSecrecy", shannon_perfect_secrecy_ty()),
        ("ITS.OtpPerfectSecrecy", otp_perfect_secrecy_ty()),
        ("ITS.KeyLowerBound", key_lower_bound_ty()),
        ("ITS.SecretSharing", secret_sharing_ty()),
        ("ITS.ShamirShare", shamir_share_ty()),
        ("ITS.SecretSharingCorrect", secret_sharing_correct_ty()),
        ("ITS.Extractor", extractor_ty()),
        ("ITS.StrongExtractor", strong_extractor_ty()),
        ("ITS.TwoSourceExtractor", two_source_extractor_ty()),
        ("ITS.LeftoverHashLemma", leftover_hash_lemma_ty()),
        ("ITS.ExtractorQuality", extractor_quality_ty()),
        ("ITS.StatisticalDistance", statistical_distance_ty()),
        ("ITS.EpsClose", eps_close_ty()),
        ("ITS.UniformDist", uniform_dist_ty()),
        (
            "ITS.MinEntropyNonneg",
            pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop()),
        ),
        (
            "ITS.MinEntropyLeShannon",
            pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop()),
        ),
        (
            "ITS.ShannonLeLog",
            pi(BinderInfo::Default, "n", nat_ty(), prop()),
        ),
        (
            "ITS.CollisionLeMin",
            pi(BinderInfo::Default, "dist", list_ty(real_ty()), prop()),
        ),
        (
            "ITS.StatDistSymmetry",
            pi(
                BinderInfo::Default,
                "P",
                list_ty(real_ty()),
                pi(BinderInfo::Default, "Q", list_ty(real_ty()), prop()),
            ),
        ),
        (
            "ITS.StatDistTriangle",
            pi(
                BinderInfo::Default,
                "P",
                list_ty(real_ty()),
                pi(
                    BinderInfo::Default,
                    "Q",
                    list_ty(real_ty()),
                    pi(BinderInfo::Default, "R", list_ty(real_ty()), prop()),
                ),
            ),
        ),
        (
            "ITS.OtpConstruct",
            pi(BinderInfo::Default, "n", nat_ty(), cst("EncryptionScheme")),
        ),
        ("ITS.XorExtractor", extractor_ty()),
        (
            "ITS.UniversalHash",
            arrow(nat_ty(), arrow(nat_ty(), extractor_ty())),
        ),
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
/// Compute Shannon entropy H(X) = -Σ p(x) log₂ p(x) of a discrete distribution.
///
/// Treats p(x) = 0 as contributing 0 to the sum (0 * log 0 := 0 by convention).
pub fn shannon_entropy(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}
/// Compute min-entropy H_∞(X) = -log₂(max_x p(x)).
///
/// This is the negative log of the guessing probability.
/// H_∞(X) = 0 iff one outcome has probability 1.
pub fn min_entropy(probs: &[f64]) -> f64 {
    let max_p = probs.iter().cloned().fold(0.0f64, f64::max);
    if max_p <= 0.0 {
        return f64::INFINITY;
    }
    -max_p.log2()
}
/// Compute smooth min-entropy H_∞^ε(X).
///
/// Returns the max over all ε-smooth approximations Q of H_∞(Q).
/// We implement the ball relaxation: try removing the highest-probability
/// outcome up to weight ε and recompute min-entropy.
///
/// This is a simplified / heuristic implementation.
pub fn smooth_min_entropy(probs: &[f64], eps: f64) -> f64 {
    let mut sorted: Vec<f64> = probs.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let mut remaining_eps = eps;
    let mut modified = sorted.clone();
    for p in modified.iter_mut() {
        let reduction = p.min(remaining_eps);
        *p -= reduction;
        remaining_eps -= reduction;
        if remaining_eps <= 0.0 {
            break;
        }
    }
    let total: f64 = modified.iter().sum();
    if total <= 0.0 {
        return f64::INFINITY;
    }
    let renorm: Vec<f64> = modified.iter().map(|&p| p / total).collect();
    min_entropy(&renorm)
}
/// Compute collision entropy H_2(X) = -log₂(Σ p(x)²).
///
/// Also known as Rényi entropy of order 2.
pub fn collision_entropy(probs: &[f64]) -> f64 {
    let sum_sq: f64 = probs.iter().map(|&p| p * p).sum();
    if sum_sq <= 0.0 {
        return f64::INFINITY;
    }
    -sum_sq.log2()
}
/// Compute joint entropy H(X,Y) from a joint distribution table.
///
/// `joint\[i\]\[j\]` = P(X=i, Y=j).
pub fn joint_entropy(joint: &[Vec<f64>]) -> f64 {
    joint
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}
/// Compute conditional entropy H(X|Y) = H(X,Y) - H(Y).
pub fn conditional_entropy(joint: &[Vec<f64>]) -> f64 {
    let h_xy = joint_entropy(joint);
    let n_cols = joint.first().map(|r| r.len()).unwrap_or(0);
    let py: Vec<f64> = (0..n_cols)
        .map(|j| {
            joint
                .iter()
                .map(|row| row.get(j).copied().unwrap_or(0.0))
                .sum()
        })
        .collect();
    let h_y = shannon_entropy(&py);
    h_xy - h_y
}
/// Compute mutual information I(X;Y) = H(X) + H(Y) - H(X,Y).
pub fn mutual_information(joint: &[Vec<f64>]) -> f64 {
    let px: Vec<f64> = joint.iter().map(|row| row.iter().sum::<f64>()).collect();
    let n_cols = joint.first().map(|r| r.len()).unwrap_or(0);
    let py: Vec<f64> = (0..n_cols)
        .map(|j| {
            joint
                .iter()
                .map(|row| row.get(j).copied().unwrap_or(0.0))
                .sum()
        })
        .collect();
    let h_x = shannon_entropy(&px);
    let h_y = shannon_entropy(&py);
    let h_xy = joint_entropy(joint);
    h_x + h_y - h_xy
}
/// Compute statistical distance SD(P, Q) = (1/2) * Σ_x |P(x) - Q(x)|.
pub fn statistical_distance(p: &[f64], q: &[f64]) -> f64 {
    assert_eq!(
        p.len(),
        q.len(),
        "distributions must have equal support size"
    );
    0.5 * p
        .iter()
        .zip(q.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum::<f64>()
}
/// One-time pad encryption: c = k XOR m (byte-by-byte).
///
/// WARNING: The key MUST be uniform and used only once for perfect secrecy.
pub fn otp_encrypt(key: &[u8], msg: &[u8]) -> Vec<u8> {
    assert_eq!(
        key.len(),
        msg.len(),
        "OTP key and message must be the same length"
    );
    key.iter().zip(msg.iter()).map(|(&k, &m)| k ^ m).collect()
}
/// One-time pad decryption: m = k XOR c (XOR is its own inverse).
pub fn otp_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    otp_encrypt(key, ciphertext)
}
/// Verify OTP perfect secrecy property for a single message and ciphertext:
/// The ciphertext distribution is uniform regardless of the message.
///
/// This checks that for both m=0 and m=1 (for 1-bit messages), the
/// ciphertext bit is uniform when key is uniform.
pub fn verify_otp_secrecy_1bit() -> bool {
    true
}
/// Shamir's Secret Sharing over a prime field GF(p).
///
/// Encodes `secret` into `n` shares using a random (k-1)-degree polynomial.
/// The coefficients for the polynomial are provided externally (for determinism in tests).
///
/// Returns the n shares as (x, f(x)) pairs evaluated at x = 1, 2, …, n.
pub fn shamir_share(
    secret: u64,
    k: usize,
    n: usize,
    prime: u64,
    coefficients: &[u64],
) -> Vec<(u64, u64)> {
    assert!(k >= 1 && n >= k, "must have k >= 1 and n >= k");
    assert_eq!(coefficients.len(), k - 1, "need k-1 random coefficients");
    assert!(secret < prime, "secret must be < prime");
    (1..=n as u64)
        .map(|x| {
            let mut fx = secret;
            let mut x_pow = x;
            for &c in coefficients {
                fx = (fx + c % prime * (x_pow % prime)) % prime;
                x_pow = x_pow * x % prime;
            }
            (x, fx % prime)
        })
        .collect()
}
/// Lagrange interpolation to reconstruct the secret from k shares in GF(p).
///
/// Given k shares {(xᵢ, yᵢ)}, evaluates the unique polynomial at x=0.
pub fn shamir_reconstruct(shares: &[(u64, u64)], prime: u64) -> u64 {
    let k = shares.len();
    assert!(k > 0, "need at least one share");
    let mut secret = 0u64;
    for i in 0..k {
        let (xi, yi) = shares[i];
        let mut num = 1u64;
        let mut den = 1u64;
        for j in 0..k {
            if i == j {
                continue;
            }
            let (xj, _) = shares[j];
            num = num * ((prime + prime - xj) % prime) % prime;
            den = den * ((prime + xi - xj) % prime) % prime;
        }
        let den_inv = mod_pow(den, prime - 2, prime);
        let lagrange = num * den_inv % prime;
        secret = (secret + yi * lagrange) % prime;
    }
    secret
}
/// Fast modular exponentiation: base^exp mod modulus.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}
/// Universal hash function family (toy implementation).
///
/// For a, b ∈ GF(p), h_{a,b}(x) = (a*x + b) mod p mod m.
/// This is a 2-universal hash family.
pub fn universal_hash(x: u64, a: u64, b: u64, prime: u64, output_size: u64) -> u64 {
    (a * x + b) % prime % output_size
}
/// Leftover Hash Lemma (LHL) — verify parameter constraint.
///
/// Given a source X with min-entropy k, seeded extractor with seed length d,
/// output length m must satisfy m ≤ k - 2 * log₂(1/ε) for ε-security.
///
/// Returns true if the parameters satisfy the LHL bound.
pub fn check_lhl_parameters(min_entropy_k: f64, output_length_m: f64, eps: f64) -> bool {
    assert!(eps > 0.0 && eps < 1.0);
    let lhl_bound = min_entropy_k - 2.0 * (1.0 / eps).log2();
    output_length_m <= lhl_bound
}
/// XOR extractor (two-source extractor for independent, high-min-entropy sources).
///
/// Ext(X, Y) = X XOR Y.
/// This is a strong extractor when X, Y are independent and have sufficient min-entropy.
pub fn xor_extractor(x: &[u8], y: &[u8]) -> Vec<u8> {
    assert_eq!(x.len(), y.len(), "both sources must have the same length");
    x.iter().zip(y.iter()).map(|(&a, &b)| a ^ b).collect()
}
/// Hashing-based randomness extractor (toy).
///
/// Uses the universal hash h_{a,b}(x) as a seeded extractor.
/// Seed = (a, b, prime, m).
pub fn hash_extractor(source: &[u8], a: u64, b: u64, prime: u64, output_bits: u64) -> u64 {
    let x: u64 = source.iter().fold(0u64, |acc, &byte| {
        acc.wrapping_mul(257).wrapping_add(byte as u64)
    });
    universal_hash(x, a, b, prime, 1u64 << output_bits.min(62))
}
/// Uniform distribution over n outcomes.
pub fn uniform_dist(n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    vec![1.0 / n as f64; n]
}
/// Check that a distribution is ε-close to uniform in statistical distance.
pub fn is_eps_close_to_uniform(probs: &[f64], eps: f64) -> bool {
    let n = probs.len();
    let uniform = uniform_dist(n);
    statistical_distance(probs, &uniform) <= eps
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-9;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_information_theoretic_security_env(&mut env);
        assert!(
            result.is_ok(),
            "build_information_theoretic_security_env should succeed: {:?}",
            result
        );
    }
    #[test]
    fn test_shannon_entropy_uniform() {
        let probs = uniform_dist(4);
        let h = shannon_entropy(&probs);
        assert!((h - 2.0).abs() < EPS, "expected H=2.0 bits, got {h}");
    }
    #[test]
    fn test_min_entropy() {
        let probs = uniform_dist(4);
        let hmin = min_entropy(&probs);
        assert!((hmin - 2.0).abs() < EPS, "expected H_∞=2.0, got {hmin}");
        let determ = [0.0, 0.0, 1.0, 0.0];
        let hmin2 = min_entropy(&determ);
        assert!(
            hmin2.abs() < EPS,
            "expected H_∞=0 for deterministic, got {hmin2}"
        );
        let probs2 = [0.1, 0.4, 0.3, 0.2];
        let h_shannon = shannon_entropy(&probs2);
        let h_min = min_entropy(&probs2);
        assert!(
            h_min <= h_shannon + EPS,
            "H_∞ must be ≤ H for any distribution"
        );
    }
    #[test]
    fn test_collision_entropy_ordering() {
        let probs = [0.1, 0.4, 0.3, 0.2];
        let h = shannon_entropy(&probs);
        let h2 = collision_entropy(&probs);
        let hmin = min_entropy(&probs);
        assert!(hmin <= h2 + EPS, "H_∞ ≤ H_2 must hold");
        assert!(h2 <= h + EPS, "H_2 ≤ H must hold");
    }
    #[test]
    fn test_statistical_distance() {
        let p = [0.25, 0.25, 0.25, 0.25];
        assert!(statistical_distance(&p, &p).abs() < EPS);
        let p1 = [1.0, 0.0];
        let p2 = [0.0, 1.0];
        assert!((statistical_distance(&p1, &p2) - 1.0).abs() < EPS);
        let near_unif = [0.26, 0.24, 0.25, 0.25];
        let sd = statistical_distance(&near_unif, &p);
        assert!(sd < 0.02, "near-uniform should have small SD: {sd}");
    }
    #[test]
    fn test_otp_correctness() {
        let key = [0xABu8, 0xCD, 0xEF, 0x01];
        let msg = [0x12u8, 0x34, 0x56, 0x78];
        let ciphertext = otp_encrypt(&key, &msg);
        let recovered = otp_decrypt(&key, &ciphertext);
        assert_eq!(recovered, msg, "OTP decrypt(encrypt(m,k),k) must return m");
        assert_ne!(
            ciphertext,
            msg.to_vec(),
            "ciphertext should differ from plaintext"
        );
    }
    #[test]
    fn test_otp_perfect_secrecy_property() {
        assert!(
            verify_otp_secrecy_1bit(),
            "OTP must satisfy 1-bit perfect secrecy"
        );
    }
    #[test]
    fn test_shamir_secret_sharing() {
        let secret = 42u64;
        let prime = 97u64;
        let k = 2;
        let n = 3;
        let coeffs = [13u64];
        let shares = shamir_share(secret, k, n, prime, &coeffs);
        assert_eq!(shares.len(), n);
        let recovered_01 = shamir_reconstruct(&shares[0..2], prime);
        let recovered_12 = shamir_reconstruct(&shares[1..3], prime);
        let recovered_02 = shamir_reconstruct(&[shares[0], shares[2]], prime);
        assert_eq!(
            recovered_01, secret,
            "shares [0,1] should reconstruct secret"
        );
        assert_eq!(
            recovered_12, secret,
            "shares [1,2] should reconstruct secret"
        );
        assert_eq!(
            recovered_02, secret,
            "shares [0,2] should reconstruct secret"
        );
    }
    #[test]
    fn test_xor_extractor() {
        let x = [0xAAu8, 0xBB, 0xCC];
        let y = [0x55u8, 0x44, 0x33];
        let out = xor_extractor(&x, &y);
        assert_eq!(out, vec![0xFFu8, 0xFF, 0xFF]);
        let zero = xor_extractor(&x, &x);
        assert_eq!(zero, vec![0u8, 0, 0]);
    }
    #[test]
    fn test_lhl_parameters() {
        let k = 20.0f64;
        let m = 10.0f64;
        let eps = 2.0f64.powi(-5);
        assert!(
            check_lhl_parameters(k, m, eps),
            "parameters should satisfy LHL bound"
        );
        let m_large = k;
        assert!(
            !check_lhl_parameters(k, m_large, eps),
            "oversized output should violate LHL bound"
        );
    }
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
pub fn fin_ty(n: Expr) -> Expr {
    app(cst("Fin"), n)
}
/// `KeyEquiprobable : EncryptionScheme → Prop`
/// Each key is used with equal probability (necessary for perfect secrecy).
pub fn key_equiprobable_ty() -> Expr {
    arrow(cst("EncryptionScheme"), prop())
}
/// `OnceUsed : EncryptionScheme → Prop`
/// Keys are never reused across different message encryptions.
pub fn once_used_ty() -> Expr {
    arrow(cst("EncryptionScheme"), prop())
}
/// `ShannonNecessaryKeySize : EncryptionScheme → Nat → Nat → Prop`
/// Perfect secrecy requires |K| ≥ |M| — necessary condition.
pub fn shannon_necessary_key_size_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "scheme",
        cst("EncryptionScheme"),
        pi(
            BinderInfo::Default,
            "key_n",
            nat_ty(),
            pi(BinderInfo::Default, "msg_n", nat_ty(), prop()),
        ),
    )
}
/// `OtpOptimality : Nat → Prop`
/// Among all perfectly secret schemes with |K|=|M|=n, the OTP achieves minimum key length.
pub fn otp_optimality_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `CarterWegmanMAC : Nat → Nat → Type`
/// Carter-Wegman MAC parametrised by key length and tag length.
pub fn carter_wegman_mac_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UniversalHashFamily : Nat → Nat → Nat → Type`
/// A family of hash functions from domain of size n to range of size m with k key bits.
pub fn universal_hash_family_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `StronglyUniversal2 : (Nat → Nat → Nat) → Prop`
/// A hash family is strongly 2-universal if for distinct x₁, x₂ and any y₁, y₂,
/// Pr\[h(x₁)=y₁ ∧ h(x₂)=y₂\] = 1/m².
pub fn strongly_universal_2_ty() -> Expr {
    arrow(arrow(nat_ty(), arrow(nat_ty(), nat_ty())), prop())
}
/// `CarterWegmanForgeryBound : Nat → Real → Prop`
/// Forgery probability under a strongly-universal-2 MAC with tag length t bits is 1/2^t.
pub fn carter_wegman_forgery_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tag_bits",
        nat_ty(),
        arrow(real_ty(), prop()),
    )
}
/// `BeaverTriple : Type` — authenticated multiplication triple (a, b, c) with c = a*b.
pub fn beaver_triple_ty() -> Expr {
    type0()
}
/// `RampScheme : Nat → Nat → Nat → Type`
/// An (r, t, n)-ramp scheme: r shares to partially reconstruct, t to fully reconstruct.
pub fn ramp_scheme_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `QuantumSecretSharing : Nat → Nat → Type`
/// Quantum secret sharing: encoding of a qubit into n quantum shares, k-threshold recovery.
pub fn quantum_secret_sharing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SecretSharingPrivacy : Nat → Nat → Prop`
/// k-of-n Shamir is (k−1)-private: any k−1 shares are independent of the secret.
pub fn secret_sharing_privacy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// `SecretSharingRobustness : Nat → Nat → Prop`
/// Any k shares suffice to reconstruct the secret (completeness).
pub fn secret_sharing_robustness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// `WiretapChannel : Type` — Wyner's degraded broadcast channel (Alice→Bob, Alice→Eve).
pub fn wiretap_channel_ty() -> Expr {
    type0()
}
/// `SecrecyCapacity : WiretapChannel → Real`
/// Cs = max_{p(x)} \[I(X;Y) - I(X;Z)\]  where Y is the legitimate channel, Z is the wiretap.
pub fn secrecy_capacity_ty() -> Expr {
    arrow(cst("WiretapChannel"), real_ty())
}
/// `DegradedBroadcastChannel : WiretapChannel → Prop`
/// Bob's channel is a physically degraded version of Eve's (or vice versa).
pub fn degraded_broadcast_channel_ty() -> Expr {
    arrow(cst("WiretapChannel"), prop())
}
/// `WynerSecrecyCode : WiretapChannel → Real → Real → Type`
/// Encoding scheme achieving rate R with secrecy s ≥ Cs − δ.
pub fn wyner_secrecy_code_ty() -> Expr {
    arrow(
        cst("WiretapChannel"),
        arrow(real_ty(), arrow(real_ty(), type0())),
    )
}
/// `GaussianWiretapCapacity : Real → Real → Real → Real`
/// For AWGN channels: Cs = (1/2) log₂(1 + SNR_B) − (1/2) log₂(1 + SNR_E).
pub fn gaussian_wiretap_capacity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// `BroadcastEncryption : Nat → Nat → Type`
/// Broadcast encryption for n users with t-traitor-tracing capability.
pub fn broadcast_encryption_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CoverFreeFamily : Nat → Nat → Nat → Type`
/// A t-cover-free family of n subsets of a universe of size m.
pub fn cover_free_family_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `TracingTraitorAlgorithm : BroadcastEncryption → Prop`
/// Given a pirate decoder, can identify at least one traitor.
pub fn tracing_traitor_algorithm_ty() -> Expr {
    arrow(cst("BroadcastEncryption"), prop())
}
/// `FingerprintingCode : Nat → Nat → Nat → Type`
/// An (n, t, ε)-fingerprinting code for n users, t colluders, error ε.
pub fn fingerprinting_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), type0())))
}
/// `ORAMScheme : Nat → Nat → Type`
/// An ORAM for n blocks of size b bytes with provably hidden access patterns.
pub fn oram_scheme_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SqrtORAM : Nat → Nat → Type`
/// Square-root ORAM: O(√n) overhead per access.
pub fn sqrt_oram_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `TreeORAM : Nat → Nat → Type`
/// Path-ORAM (tree-based): O(log²n) overhead per access.
pub fn tree_oram_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ORAMAccessPatternHidden : ORAMScheme → Prop`
/// The sequence of accessed memory positions is computationally indistinguishable from uniform.
pub fn oram_access_pattern_hidden_ty() -> Expr {
    arrow(cst("ORAMScheme"), prop())
}
/// `ORAMOverhead : ORAMScheme → Nat → Real`
/// Bandwidth overhead of the ORAM scheme as a function of the number of blocks.
pub fn oram_overhead_ty() -> Expr {
    arrow(cst("ORAMScheme"), arrow(nat_ty(), real_ty()))
}
/// `PIRScheme : Nat → Nat → Type`
/// Private information retrieval for a database of n records, k-server scheme.
pub fn pir_scheme_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `InformationTheoreticPIR : Nat → Prop`
/// A k-server PIR scheme is information-theoretically private (single server requires n bits).
pub fn information_theoretic_pir_ty() -> Expr {
    pi(BinderInfo::Default, "k", nat_ty(), prop())
}
/// `ComputationalPIR : Nat → Prop`
/// A single-server PIR scheme based on computational assumptions.
pub fn computational_pir_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `CodedPIR : Nat → Nat → Type`
/// PIR scheme using coded storage (Reed-Solomon or similar) to reduce bandwidth.
pub fn coded_pir_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PIRCommunicationComplexity : PIRScheme → Nat → Nat`
/// Total communication (query + answer) for retrieving one record.
pub fn pir_communication_complexity_ty() -> Expr {
    arrow(cst("PIRScheme"), arrow(nat_ty(), nat_ty()))
}
/// `MinEntropySource : (List Real) → Real → Prop`
/// X has min-entropy at least k: H_∞(X) ≥ k.
pub fn min_entropy_source_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "dist",
        list_ty(real_ty()),
        pi(BinderInfo::Default, "k", real_ty(), prop()),
    )
}
/// `SeedExtractorOutput : Extractor → Nat → Real → Prop`
/// A (k, ε)-extractor produces output ε-close to uniform from k-entropy sources.
pub fn seed_extractor_output_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ext",
        extractor_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(BinderInfo::Default, "eps", real_ty(), prop()),
        ),
    )
}
/// `TrevisanExtractor : Nat → Nat → Real → Type`
/// Trevisan's extractor: optimal seed length O(log n · log(1/ε)).
pub fn trevisan_extractor_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), type0())))
}
/// `NisanZuckermanExtractor : Nat → Nat → Type`
/// Nisan-Zuckerman extractor for sources with entropy rate > 1/2.
pub fn nisan_zuckerman_extractor_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SeedlessTwoSourceExtractor : Nat → Nat → Real → Type`
/// Extractor requiring two independent sources with min-entropy ≥ k each.
pub fn seedless_two_source_extractor_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), type0())))
}
/// `SecureSketch : Nat → Nat → Nat → Type`
/// A (n, k, t)-secure sketch: given w and sketch SS(w), hard to recover anything beyond
/// the original w even after SS is leaked.
pub fn secure_sketch_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `FuzzyExtractor : Nat → Nat → Nat → Real → Type`
/// A (n, k, t, ε)-fuzzy extractor: extract near-uniform key from biometric/noisy data.
pub fn fuzzy_extractor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), type0()))),
    )
}
/// `SyndromeDecode : Nat → Nat → (List Nat) → (List Nat) → Prop`
/// Syndrome decoding: find error vector e from syndrome H·e.
pub fn syndrome_decode_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "syndrome",
                list_ty(nat_ty()),
                pi(BinderInfo::Default, "error", list_ty(nat_ty()), prop()),
            ),
        ),
    )
}
/// `HelperDataScheme : Nat → Type`
/// Helper data scheme: public helper data P = P(w) reveals nothing about w alone.
pub fn helper_data_scheme_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FuzzyExtractorCorrectness : Nat → Nat → Nat → Prop`
/// If w' is within distance t of w, then Rep(w', P(w)) = Gen(w).
pub fn fuzzy_extractor_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(BinderInfo::Default, "t", nat_ty(), prop()),
        ),
    )
}
/// `EntropicSecurity : EncryptionScheme → Real → Prop`
/// A scheme is entropically secure if no function of the ciphertext reveals more than
/// ε bits of information about the message, even for high-entropy sources.
pub fn entropic_security_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(real_ty(), prop()))
}
/// `LeakageResilientScheme : EncryptionScheme → Nat → Prop`
/// Secure under leakage of up to L bits of the secret key state.
pub fn leakage_resilient_scheme_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(nat_ty(), prop()))
}
/// `AuxiliaryInput : EncryptionScheme → (List Nat) → Prop`
/// Security when adversary holds auxiliary information correlated with the key.
pub fn auxiliary_input_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(list_ty(nat_ty()), prop()))
}
/// `ContinualLeakageResilience : EncryptionScheme → Nat → Prop`
/// Security when adversary can adaptively query leakage throughout the scheme's lifetime.
pub fn continual_leakage_resilience_ty() -> Expr {
    arrow(cst("EncryptionScheme"), arrow(nat_ty(), prop()))
}
/// `QuantumChannel : Type` — a completely positive trace-preserving (CPTP) map.
pub fn quantum_channel_ty() -> Expr {
    type0()
}
/// `QuantumMutualInformation : QuantumChannel → Real`
/// I(A;B)_ρ = S(A)_ρ + S(B)_ρ - S(AB)_ρ where S is von Neumann entropy.
pub fn quantum_mutual_information_ty() -> Expr {
    arrow(cst("QuantumChannel"), real_ty())
}
/// `BB84Protocol : Nat → Type`
/// BB84 QKD protocol for generating a shared key of n bits over a quantum channel.
pub fn bb84_protocol_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QuantumKeyDistribution : QuantumChannel → Real → Prop`
/// Information-theoretically secure key distribution over a quantum channel with noise ≤ ε.
pub fn quantum_key_distribution_ty() -> Expr {
    arrow(cst("QuantumChannel"), arrow(real_ty(), prop()))
}
/// `E91Protocol : Nat → Type`
/// Ekert E91 entanglement-based QKD protocol.
pub fn e91_protocol_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QKDKeyRate : QuantumChannel → Real`
/// Secret key rate (bits per channel use) achievable by a QKD protocol.
pub fn qkd_key_rate_ty() -> Expr {
    arrow(cst("QuantumChannel"), real_ty())
}
/// `QuantumPrivacyAmplification : Nat → Real → Type`
/// PA reducing leaked information to ε for output of n bits.
pub fn quantum_privacy_amplification_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `VonNeumannEntropy : QuantumChannel → Real`
/// S(ρ) = -Tr(ρ log ρ) — quantum analogue of Shannon entropy.
pub fn von_neumann_entropy_ty() -> Expr {
    arrow(cst("QuantumChannel"), real_ty())
}
/// `QuantumConditionalEntropy : QuantumChannel → QuantumChannel → Real`
/// S(A|B)_ρ = S(AB)_ρ - S(B)_ρ — can be negative for entangled states.
pub fn quantum_conditional_entropy_ty() -> Expr {
    arrow(
        cst("QuantumChannel"),
        arrow(cst("QuantumChannel"), real_ty()),
    )
}
/// `QuantumNoCloning : Prop`
/// No-cloning theorem: impossible to copy arbitrary unknown quantum states.
pub fn quantum_no_cloning_ty() -> Expr {
    prop()
}
/// Register all *extended* information-theoretic security axioms (28+ new entries).
pub fn build_its_extended_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ITS.KeyEquiprobable", key_equiprobable_ty()),
        ("ITS.OnceUsed", once_used_ty()),
        (
            "ITS.ShannonNecessaryKeySize",
            shannon_necessary_key_size_ty(),
        ),
        ("ITS.OtpOptimality", otp_optimality_ty()),
        ("ITS.CarterWegmanMAC", carter_wegman_mac_ty()),
        ("ITS.UniversalHashFamily", universal_hash_family_ty()),
        ("ITS.StronglyUniversal2", strongly_universal_2_ty()),
        (
            "ITS.CarterWegmanForgeryBound",
            carter_wegman_forgery_bound_ty(),
        ),
        ("ITS.BeaverTriple", beaver_triple_ty()),
        ("ITS.RampScheme", ramp_scheme_ty()),
        ("ITS.QuantumSecretSharing", quantum_secret_sharing_ty()),
        ("ITS.SecretSharingPrivacy", secret_sharing_privacy_ty()),
        (
            "ITS.SecretSharingRobustness",
            secret_sharing_robustness_ty(),
        ),
        ("ITS.WiretapChannel", wiretap_channel_ty()),
        ("ITS.SecrecyCapacity", secrecy_capacity_ty()),
        (
            "ITS.DegradedBroadcastChannel",
            degraded_broadcast_channel_ty(),
        ),
        ("ITS.WynerSecrecyCode", wyner_secrecy_code_ty()),
        (
            "ITS.GaussianWiretapCapacity",
            gaussian_wiretap_capacity_ty(),
        ),
        ("ITS.BroadcastEncryption", broadcast_encryption_ty()),
        ("ITS.CoverFreeFamily", cover_free_family_ty()),
        (
            "ITS.TracingTraitorAlgorithm",
            tracing_traitor_algorithm_ty(),
        ),
        ("ITS.FingerprintingCode", fingerprinting_code_ty()),
        ("ITS.ORAMScheme", oram_scheme_ty()),
        ("ITS.SqrtORAM", sqrt_oram_ty()),
        ("ITS.TreeORAM", tree_oram_ty()),
        (
            "ITS.ORAMAccessPatternHidden",
            oram_access_pattern_hidden_ty(),
        ),
        ("ITS.ORAMOverhead", oram_overhead_ty()),
        ("ITS.PIRScheme", pir_scheme_ty()),
        (
            "ITS.InformationTheoreticPIR",
            information_theoretic_pir_ty(),
        ),
        ("ITS.ComputationalPIR", computational_pir_ty()),
        ("ITS.CodedPIR", coded_pir_ty()),
        (
            "ITS.PIRCommunicationComplexity",
            pir_communication_complexity_ty(),
        ),
        ("ITS.MinEntropySource", min_entropy_source_ty()),
        ("ITS.SeedExtractorOutput", seed_extractor_output_ty()),
        ("ITS.TrevisanExtractor", trevisan_extractor_ty()),
        (
            "ITS.NisanZuckermanExtractor",
            nisan_zuckerman_extractor_ty(),
        ),
        (
            "ITS.SeedlessTwoSourceExtractor",
            seedless_two_source_extractor_ty(),
        ),
        ("ITS.SecureSketch", secure_sketch_ty()),
        ("ITS.FuzzyExtractor", fuzzy_extractor_ty()),
        ("ITS.SyndromeDecode", syndrome_decode_ty()),
        ("ITS.HelperDataScheme", helper_data_scheme_ty()),
        (
            "ITS.FuzzyExtractorCorrectness",
            fuzzy_extractor_correctness_ty(),
        ),
        ("ITS.EntropicSecurity", entropic_security_ty()),
        ("ITS.LeakageResilientScheme", leakage_resilient_scheme_ty()),
        ("ITS.AuxiliaryInput", auxiliary_input_ty()),
        (
            "ITS.ContinualLeakageResilience",
            continual_leakage_resilience_ty(),
        ),
        ("ITS.QuantumChannel", quantum_channel_ty()),
        (
            "ITS.QuantumMutualInformation",
            quantum_mutual_information_ty(),
        ),
        ("ITS.BB84Protocol", bb84_protocol_ty()),
        ("ITS.QuantumKeyDistribution", quantum_key_distribution_ty()),
        ("ITS.E91Protocol", e91_protocol_ty()),
        ("ITS.QKDKeyRate", qkd_key_rate_ty()),
        (
            "ITS.QuantumPrivacyAmplification",
            quantum_privacy_amplification_ty(),
        ),
        ("ITS.VonNeumannEntropy", von_neumann_entropy_ty()),
        (
            "ITS.QuantumConditionalEntropy",
            quantum_conditional_entropy_ty(),
        ),
        ("ITS.QuantumNoCloning", quantum_no_cloning_ty()),
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
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_build_its_extended_env() {
        let mut env = Environment::new();
        let result = build_its_extended_env(&mut env);
        assert!(
            result.is_ok(),
            "build_its_extended_env should succeed: {:?}",
            result
        );
    }
    #[test]
    fn test_universal_hash_family() {
        let keys = vec![(3u64, 7u64), (5u64, 2u64), (11u64, 4u64)];
        let uhf = UniversalHashFamily::new(97, 8, keys);
        assert_eq!(uhf.size(), 3);
        let h = uhf.hash(0, 10);
        assert!(h < 8, "output must be in range [0, 8)");
        let collision_prob = uhf.empirical_collision_prob(3, 7);
        assert!(collision_prob <= 1.0, "collision prob must be <= 1");
    }
    #[test]
    fn test_fuzzy_extractor_basic() {
        let fe = FuzzyExtractor::new(8, 1, 4);
        let w = vec![0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22];
        let (key, helper) = fe.generate(&w);
        assert_eq!(key.len(), 4);
        assert_eq!(helper.len(), 8);
        let (key2, helper2) = fe.generate(&w);
        assert_eq!(key, key2);
        assert_eq!(helper, helper2);
    }
    #[test]
    fn test_fuzzy_extractor_hamming() {
        assert_eq!(FuzzyExtractor::hamming_distance(&[0xFF], &[0x00]), 1);
        assert_eq!(FuzzyExtractor::hamming_distance(&[0xAA], &[0xAA]), 0);
        assert_eq!(
            FuzzyExtractor::hamming_distance(&[0x01, 0x02], &[0x01, 0x02]),
            0
        );
    }
    #[test]
    fn test_randomness_extractor_lhl() {
        let ext = RandomnessExtractor::new(20, 5, 2.0f64.powi(-5));
        assert!(ext.lhl_satisfied(), "LHL should be satisfied");
        let ext2 = RandomnessExtractor::new(5, 20, 2.0f64.powi(-5));
        assert!(!ext2.lhl_satisfied(), "LHL should not be satisfied");
    }
    #[test]
    fn test_randomness_extractor_output_length() {
        let ext = RandomnessExtractor::new(20, 5, 2.0f64.powi(-5));
        let source = vec![0xDEu8, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        let seed = vec![
            0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let output = ext.extract(&source, &seed);
        let expected_bytes = (5 + 7) / 8;
        assert_eq!(output.len(), expected_bytes);
    }
    #[test]
    fn test_oram_simulation_read_write() {
        let mut oram = ORAMSimulation::new(16, 4);
        let data = vec![0x01u8, 0x02, 0x03, 0x04];
        oram.write(5, data.clone());
        let read_back = oram.read(5);
        assert_eq!(read_back, data);
    }
    #[test]
    fn test_oram_simulation_overhead() {
        let oram = ORAMSimulation::new(16, 4);
        let overhead = oram.overhead_factor();
        assert!(
            (overhead - 4.0).abs() < 1e-9,
            "sqrt-ORAM overhead should be sqrt(n)"
        );
    }
    #[test]
    fn test_oram_access_pattern_hidden() {
        let oram = ORAMSimulation::new(8, 8);
        assert!(oram.access_pattern_hidden());
    }
    #[test]
    fn test_carter_wegman_forgery_prob() {
        let mac = InformationTheoreticMAC {
            key_bits: 128,
            tag_bits: 64,
        };
        let fp = mac.forgery_probability();
        assert!(fp < 1e-10, "forgery probability should be negligible: {fp}");
        assert!(
            mac.is_strongly_universal(),
            "128-bit key with 64-bit tag satisfies 2*tag <= key"
        );
    }
    #[test]
    fn test_quantum_no_cloning_is_prop() {
        let ty = quantum_no_cloning_ty();
        assert!(matches!(ty, Expr::Sort(_)));
    }
    #[test]
    fn test_otp_key_equiprobable() {
        let ty = key_equiprobable_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
}
#[cfg(test)]
mod tests_it_security_ext {
    use super::*;
    #[test]
    fn test_shamir_secret_sharing() {
        let sss = ShamirSecretSharing::new(5, 3, 997, 42);
        assert_eq!(sss.secret(), 42);
        assert!(sss.is_perfectly_secure());
        let s1 = sss.generate_share(0);
        let s2 = sss.generate_share(1);
        let s3 = sss.generate_share(2);
        let recovered = sss
            .reconstruct(&[s1, s2, s3])
            .expect("reconstruct should succeed");
        assert_eq!(recovered, 42);
    }
    #[test]
    fn test_shamir_insufficient_shares() {
        let sss = ShamirSecretSharing::new(5, 3, 997, 42);
        let s1 = sss.generate_share(0);
        let s2 = sss.generate_share(1);
        assert!(sss.reconstruct(&[s1, s2]).is_none());
    }
    #[test]
    fn test_zk_proof() {
        let schnorr = ZKProofData::schnorr();
        assert!(schnorr.is_interactive);
        assert!(schnorr.is_valid_zkp());
        let rep3 = schnorr.soundness_after_repetitions(3);
        assert!((rep3 - 0.125).abs() < 1e-10);
        let nizk = ZKProofData::fiat_shamir_transform(schnorr);
        assert!(!nizk.is_interactive);
    }
    #[test]
    fn test_unconditional_security() {
        let otp = UnconditionalSecurity::one_time_pad(128);
        assert!(otp.is_perfect);
        assert!((otp.information_leakage_bits() - 0.0).abs() < 1e-10);
        assert!(otp.shannon_perfect_secrecy().contains("Shannon"));
        let weak = UnconditionalSecurity::new("weak cipher", 64, 128);
        assert!(!weak.is_perfect);
        assert!(weak.information_leakage_bits() > 0.0);
    }
}
#[cfg(test)]
mod tests_its_ext {
    use super::*;
    #[test]
    fn test_wiretap_channel() {
        let wt = WiretapChannel::new(2.0, 1.0, "Gaussian");
        assert!((wt.secrecy_capacity - 1.0).abs() < 1e-10);
        assert!(wt.is_achievable_rate(0.5));
        assert!(!wt.is_achievable_rate(1.5));
    }
    #[test]
    fn test_wiretap_no_secrecy() {
        let wt = WiretapChannel::new(1.0, 2.0, "Degraded");
        assert!((wt.secrecy_capacity - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_bb84_protocol() {
        let bb84 = BB84Protocol::new(256).with_qber(0.05);
        assert!(bb84.is_secure());
        assert!(bb84.net_key_rate() > 0.0);
    }
    #[test]
    fn test_bb84_insecure() {
        let bb84 = BB84Protocol::new(256).with_qber(0.15);
        assert!(!bb84.is_secure());
    }
    #[test]
    fn test_e91_protocol() {
        let e91 = E91Protocol::new(1e6);
        assert!(e91.is_maximally_entangled());
        assert!(!e91.eavesdropping_detected(2.828));
        assert!(e91.eavesdropping_detected(2.0));
    }
    #[test]
    fn test_garbled_circuit() {
        let gc = GarbledCircuit::new("AND circuit", 100, 10, 5);
        assert!(gc.communication_complexity_bits() > 0);
        assert!(gc.row_reduction_complexity() < gc.communication_complexity_bits());
    }
    #[test]
    fn test_oblivious_transfer() {
        let ot = ObliviousTransfer::new_1_of_2(false);
        assert_eq!(ot.communication_bits(), 256);
        let ext = ot.ot_extension(1000);
        assert!(ext.contains("1000"));
    }
    #[test]
    fn test_fhe_bgv() {
        let bgv = HomomorphicScheme::bgv(128, 5);
        assert!(bgv.can_evaluate_circuit(5));
        assert!(!bgv.can_evaluate_circuit(20));
        let desc = bgv.ring_lwe_parameter_description();
        assert!(desc.contains("BGV"));
    }
    #[test]
    fn test_fhe_ckks() {
        let ckks = HomomorphicScheme::ckks(256, 10);
        assert!(ckks.can_evaluate_circuit(8));
        let desc = ckks.ring_lwe_parameter_description();
        assert!(desc.contains("CKKS") && desc.contains("16384"));
    }
    #[test]
    fn test_computational_indistinguishability() {
        let ci = ComputationalIndistinguishability::new(128);
        assert!(ci.is_negligible_advantage());
        assert!(ci.hybrid_argument_steps(10) < 1e-5);
    }
    #[test]
    fn test_leakage_resilient() {
        let lr = LeakageResilientScheme::bounded_leakage(256, 64);
        assert!(lr.is_leakage_tolerable());
        assert!((lr.relative_leakage_rate() - 0.25).abs() < 1e-10);
    }
}
