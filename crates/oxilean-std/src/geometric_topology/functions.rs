//! Functions for geometric topology: 3-manifold invariants, surgery, JSJ decomposition.

use super::types::{
    bezout_coefficients, DehnSurgery, HeegaardSplitting, HyperbolicPiece, JSJDecomposition,
    Manifold3, ManifoldPiece, Matrix2x2, SeifertFibered,
};

// ── Euler characteristic ──────────────────────────────────────────────────────

/// Compute the Euler characteristic χ of a 3-manifold.
///
/// For closed 3-manifolds χ = 0 by Poincaré duality (over ℝ).
/// For manifolds with boundary, χ = (1/2) χ(∂M) by the boundary formula,
/// but since we only track integer data we return 0 for closed manifolds and
/// (-boundary_components) as a heuristic for open manifolds.
pub fn euler_characteristic_3mfld(m: &Manifold3) -> i64 {
    if m.is_closed {
        // All closed odd-dimensional manifolds have χ = 0
        0
    } else {
        // Heuristic: each torus boundary contributes 0 to χ;
        // return negative boundary count as a signed marker
        -(m.boundary_components as i64)
    }
}

// ── Heegaard genus ────────────────────────────────────────────────────────────

/// Estimate the Heegaard genus of a 3-manifold.
///
/// Uses known values for standard manifolds and the genus field as a fallback.
pub fn heegaard_genus(m: &Manifold3) -> usize {
    match m.name.as_str() {
        "S^3" => 0,
        "S^2 x S^1" => 1,
        "T^3" => 3,
        "RP^3" => 1,
        "D^2 x S^1" => 1,
        _ => {
            // Fall back to genus field if set; otherwise use boundary_components as lower bound
            m.genus
                .map(|g| g as usize)
                .unwrap_or(m.boundary_components as usize)
        }
    }
}

/// Construct a Heegaard splitting for a 3-manifold.
pub fn make_heegaard_splitting(m: &Manifold3) -> HeegaardSplitting {
    HeegaardSplitting::new(heegaard_genus(m), m.name.clone())
}

// ── Primality ─────────────────────────────────────────────────────────────────

/// Heuristic: return true if the 3-manifold is (likely) irreducible / prime.
///
/// Uses a lookup table of known prime and composite manifolds.
pub fn is_prime_manifold(m: &Manifold3) -> bool {
    // Known composite manifolds
    let composite = ["S^2 x S^1", "RP^3 # RP^3"];
    if composite.contains(&m.name.as_str()) {
        return false;
    }
    // Known prime manifolds
    let prime = [
        "S^3",
        "T^3",
        "RP^3",
        "D^2 x S^1",
        "figure-eight knot complement",
        "trefoil knot complement",
        "L(5,2)",
        "L(7,2)",
        "L(7,3)",
    ];
    if prime.contains(&m.name.as_str()) {
        return true;
    }
    // General heuristic: closed orientable manifolds with finite π_1 are prime
    m.is_orientable && m.is_closed
}

// ── Seifert invariants ────────────────────────────────────────────────────────

/// Extract Seifert invariants (e0, \[(a_i, b_i)\]) from a Seifert-fibered space.
///
/// e0 is the rational Euler number's integer part (sum of b_i/a_i floors).
/// Returns (e0, list_of_exceptional_fiber_pairs).
pub fn seifert_invariants(sf: &SeifertFibered) -> (i64, Vec<(i64, i64)>) {
    // e0 is the integer part of the orbifold Euler characteristic contribution:
    // e0 = -sum_{i} floor(b_i / a_i)
    let e0: i64 = sf
        .exceptional_fibers
        .iter()
        .map(|&(a, b)| if a == 0 { 0 } else { -(b / a) })
        .sum();
    (e0, sf.exceptional_fibers.clone())
}

/// Compute the rational Euler number e(M) of a Seifert-fibered space.
///
/// e(M) = e0 + sum_i b_i/a_i  (as a rational number, returned as (num, denom)).
pub fn seifert_euler_number(sf: &SeifertFibered) -> (i64, i64) {
    let (e0, fibers) = seifert_invariants(sf);
    // Compute e0 + sum b_i/a_i as a fraction
    let mut num = e0;
    let mut denom: i64 = 1;
    for (a, b) in &fibers {
        if *a == 0 {
            continue;
        }
        // num/denom + b/a = (num*a + b*denom) / (denom*a)
        num = num * a + b * denom;
        denom *= a;
    }
    let g = gcd(num.unsigned_abs(), denom.unsigned_abs()) as i64;
    if g == 0 {
        (num, denom)
    } else {
        (num / g, denom / g)
    }
}

// ── Dehn surgery ──────────────────────────────────────────────────────────────

/// Compute a string description of H_1(M_K(p/q)) for (p/q)-surgery on knot K.
///
/// By the surgery formula, H_1(S^3_{p/q}(K)) ≅ Z/pZ when p ≠ 0, and Z when p = 0.
/// The Alexander polynomial det is used for the full Casson invariant but we
/// return a simplified description here.
pub fn surgery_formula_alexander(knot: &str, p: i64, q: i64) -> String {
    let _ = q; // slope denominator not needed for H_1 alone
    if p == 0 {
        format!("H_1(S^3_{{0/{q}}}({knot})) ≅ Z (infinite cyclic)")
    } else if p == 1 || p == -1 {
        format!("H_1(S^3_{{{p}/{q}}}({knot})) ≅ 0 (homology sphere)")
    } else {
        format!(
            "H_1(S^3_{{{p}/{q}}}({knot})) ≅ Z/{abs}Z",
            abs = p.unsigned_abs()
        )
    }
}

/// Compute (|H_1|, type) for the result of a Dehn surgery.
///
/// Returns `(order, kind)` where:
/// - `order = 0` means H_1 is infinite (free part),
/// - `order > 0` means H_1 is finite of that order,
/// - `kind = 0` means infinite cyclic (p=0), `kind = 1` means trivial (|p|=1), `kind = 2` means torsion.
pub fn dehn_surgery_homology(surgery: &DehnSurgery) -> (i64, i64) {
    let p = surgery.slope.0;
    if p == 0 {
        (0, 0) // infinite cyclic
    } else if p == 1 || p == -1 {
        (1, 1) // homology sphere, trivial H_1
    } else {
        (p.unsigned_abs() as i64, 2) // Z/|p|Z torsion
    }
}

// ── JSJ decomposition ─────────────────────────────────────────────────────────

/// Return a toy JSJ decomposition for a named manifold.
///
/// Only a small catalogue of examples is known; returns `None` for unknown manifolds.
pub fn jsj_decomposition(name: &str) -> Option<JSJDecomposition> {
    match name {
        "S^3" => {
            // S^3 is Seifert-fibered over S^2 with no exceptional fibers
            Some(JSJDecomposition::new(
                vec![ManifoldPiece::Seifert(SeifertFibered::new("S^2", vec![]))],
                vec![],
            ))
        }
        "T^3" => {
            // T^3 is a Seifert fibration over T^2
            Some(JSJDecomposition::new(
                vec![ManifoldPiece::Seifert(SeifertFibered::new("T^2", vec![]))],
                vec![],
            ))
        }
        "figure-eight knot complement" => {
            // Hyperbolic piece with 1 cusp
            Some(JSJDecomposition::new(
                vec![ManifoldPiece::Hyperbolic(HyperbolicPiece::new(
                    2.029_883_212_819_307_5,
                    1,
                ))],
                vec![],
            ))
        }
        "trefoil knot complement" => {
            // Seifert-fibered: base D^2 with one exceptional fiber (3,1)
            Some(JSJDecomposition::new(
                vec![ManifoldPiece::Seifert(SeifertFibered::new(
                    "D^2",
                    vec![(3, 1), (2, 1)],
                ))],
                vec![],
            ))
        }
        "cable space" => {
            // Cable space: Seifert over annulus
            Some(JSJDecomposition::new(
                vec![
                    ManifoldPiece::Seifert(SeifertFibered::new("A", vec![])),
                    ManifoldPiece::IBundle,
                ],
                vec![(0, 1, Matrix2x2::identity())],
            ))
        }
        _ => None,
    }
}

// ── Matrix operations ─────────────────────────────────────────────────────────

/// Multiply two 2×2 integer matrices.
pub fn matrix2x2_mul(a: &Matrix2x2, b: &Matrix2x2) -> Matrix2x2 {
    Matrix2x2::new(
        a.a * b.a + a.b * b.c,
        a.a * b.b + a.b * b.d,
        a.c * b.a + a.d * b.c,
        a.c * b.b + a.d * b.d,
    )
}

/// Check that a matrix is in SL(2,Z): det = ±1.
pub fn is_in_sl2z(m: &Matrix2x2) -> bool {
    let det = m.det();
    det == 1 || det == -1
}

/// Compute the inverse of a matrix in SL(2,Z).
///
/// Returns `None` if the matrix is not in SL(2,Z).
pub fn sl2z_inverse(m: &Matrix2x2) -> Option<Matrix2x2> {
    let det = m.det();
    if det == 1 {
        Some(Matrix2x2::new(m.d, -m.b, -m.c, m.a))
    } else if det == -1 {
        Some(Matrix2x2::new(-m.d, m.b, m.c, -m.a))
    } else {
        None
    }
}

// ── Thurston geometrization ───────────────────────────────────────────────────

/// Return the Thurston geometry type of a 3-manifold by name heuristic.
///
/// The eight Thurston geometries are:
/// S^3, E^3, H^3, S^2 x R, H^2 x R, SL2R, Nil, Sol.
pub fn thurston_geometrization_type(m: &Manifold3) -> &'static str {
    match m.name.as_str() {
        "S^3" | "RP^3" | "L(5,2)" | "L(7,2)" | "L(7,3)" => "S^3",
        "T^3" | "flat torus bundle" => "E^3",
        "S^2 x S^1" | "RP^3 # RP^3" => "S^2 x R",
        "figure-eight knot complement" | "hyperbolic" => "H^3",
        "trefoil knot complement" | "Seifert fibered" => "SL2R-tilde",
        "Nil manifold" | "nilmanifold" => "Nil",
        "solvmanifold" | "sol manifold" => "Sol",
        "H^2 x R bundle" => "H^2 x R",
        _ => {
            if m.is_orientable && m.is_closed {
                // Generic orientable closed: guess spherical or hyperbolic
                if m.genus.unwrap_or(0) == 0 {
                    "S^3"
                } else {
                    "H^3"
                }
            } else {
                "unknown"
            }
        }
    }
}

// ── Helper: GCD ──────────────────────────────────────────────────────────────

pub(super) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// ── Extended Euclidean (re-export for convenience) ────────────────────────────

/// Extended Euclidean: return (s,t) with a*s + b*t = gcd(|a|,|b|).
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (s, t) = bezout_coefficients(a, b);
    let g = a * s + b * t;
    (g, s, t)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{DehnSurgery, Manifold3, SeifertFibered};
    use super::*;

    #[test]
    fn test_euler_char_closed() {
        let s3 = Manifold3::s3();
        assert_eq!(euler_characteristic_3mfld(&s3), 0);
    }

    #[test]
    fn test_euler_char_open() {
        let st = Manifold3::solid_torus();
        assert_eq!(euler_characteristic_3mfld(&st), -1);
    }

    #[test]
    fn test_heegaard_genus_s3() {
        let s3 = Manifold3::s3();
        assert_eq!(heegaard_genus(&s3), 0);
    }

    #[test]
    fn test_heegaard_genus_t3() {
        let t3 = Manifold3::t3();
        assert_eq!(heegaard_genus(&t3), 3);
    }

    #[test]
    fn test_heegaard_genus_rp3() {
        let rp3 = Manifold3::rp3();
        assert_eq!(heegaard_genus(&rp3), 1);
    }

    #[test]
    fn test_make_heegaard_splitting() {
        let s3 = Manifold3::s3();
        let hs = make_heegaard_splitting(&s3);
        assert_eq!(hs.genus, 0);
        assert_eq!(hs.manifold, "S^3");
    }

    #[test]
    fn test_is_prime_manifold_s3() {
        assert!(is_prime_manifold(&Manifold3::s3()));
    }

    #[test]
    fn test_is_prime_manifold_t3() {
        assert!(is_prime_manifold(&Manifold3::t3()));
    }

    #[test]
    fn test_is_prime_manifold_composite() {
        let composite = Manifold3::new("S^2 x S^1", None, true, true, 0);
        assert!(!is_prime_manifold(&composite));
    }

    #[test]
    fn test_seifert_invariants_trefoil() {
        // Trefoil complement: D^2(3,1)(2,1)
        let sf = SeifertFibered::new("D^2", vec![(3, 1), (2, 1)]);
        let (e0, fibers) = seifert_invariants(&sf);
        assert_eq!(fibers.len(), 2);
        assert_eq!(e0, 0); // -floor(1/3) - floor(1/2) = 0
    }

    #[test]
    fn test_seifert_invariants_lens() {
        let sf = SeifertFibered::lens_space_seifert(5, 2);
        let (e0, fibers) = seifert_invariants(&sf);
        assert_eq!(fibers.len(), 2);
        let _ = e0;
    }

    #[test]
    fn test_surgery_formula_trivial() {
        let desc = surgery_formula_alexander("trefoil", 1, 0);
        assert!(desc.contains("homology sphere"));
    }

    #[test]
    fn test_surgery_formula_torsion() {
        let desc = surgery_formula_alexander("trefoil", 5, 1);
        assert!(desc.contains("Z/5Z"));
    }

    #[test]
    fn test_surgery_formula_zero_slope() {
        let desc = surgery_formula_alexander("trefoil", 0, 1);
        assert!(desc.contains('Z'));
    }

    #[test]
    fn test_dehn_surgery_homology_torsion() {
        let s = DehnSurgery::new("S^3", "trefoil", (5, 1));
        let (ord, kind) = dehn_surgery_homology(&s);
        assert_eq!(ord, 5);
        assert_eq!(kind, 2);
    }

    #[test]
    fn test_dehn_surgery_homology_sphere() {
        let s = DehnSurgery::new("S^3", "trefoil", (1, 0));
        let (ord, kind) = dehn_surgery_homology(&s);
        assert_eq!(ord, 1);
        assert_eq!(kind, 1);
    }

    #[test]
    fn test_dehn_surgery_homology_infinite() {
        let s = DehnSurgery::new("S^3", "trefoil", (0, 1));
        let (ord, kind) = dehn_surgery_homology(&s);
        assert_eq!(ord, 0);
        assert_eq!(kind, 0);
    }

    #[test]
    fn test_jsj_s3() {
        let jsj = jsj_decomposition("S^3");
        assert!(jsj.is_some());
        let jsj = jsj.unwrap();
        assert_eq!(jsj.num_pieces(), 1);
    }

    #[test]
    fn test_jsj_figure_eight() {
        let jsj = jsj_decomposition("figure-eight knot complement");
        assert!(jsj.is_some());
        let jsj = jsj.unwrap();
        assert_eq!(jsj.num_pieces(), 1);
        if let ManifoldPiece::Hyperbolic(hp) = &jsj.pieces[0] {
            assert!(hp.volume > 2.0);
            assert_eq!(hp.cusps, 1);
        } else {
            panic!("Expected hyperbolic piece");
        }
    }

    #[test]
    fn test_jsj_unknown() {
        assert!(jsj_decomposition("exotic manifold XYZ").is_none());
    }

    #[test]
    fn test_matrix_identity_det() {
        let id = Matrix2x2::identity();
        assert_eq!(id.det(), 1);
        assert!(is_in_sl2z(&id));
    }

    #[test]
    fn test_matrix_mul_identity() {
        let id = Matrix2x2::identity();
        let m = Matrix2x2::new(3, 1, 2, 1);
        let prod = matrix2x2_mul(&id, &m);
        assert_eq!(prod, m);
    }

    #[test]
    fn test_matrix_mul() {
        let a = Matrix2x2::new(1, 1, 0, 1); // shear
        let b = Matrix2x2::new(1, 0, 1, 1); // other shear
        let c = matrix2x2_mul(&a, &b);
        // [1 1][1 0] = [2 1]
        // [0 1][1 1]   [1 1]
        assert_eq!(c, Matrix2x2::new(2, 1, 1, 1));
    }

    #[test]
    fn test_is_in_sl2z_true() {
        let m = Matrix2x2::new(3, 1, 2, 1); // det = 3-2 = 1
        assert!(is_in_sl2z(&m));
    }

    #[test]
    fn test_is_in_sl2z_false() {
        let m = Matrix2x2::new(2, 0, 0, 2); // det = 4
        assert!(!is_in_sl2z(&m));
    }

    #[test]
    fn test_sl2z_inverse() {
        let m = Matrix2x2::new(3, 1, 2, 1);
        let inv = sl2z_inverse(&m);
        assert!(inv.is_some());
        let inv = inv.unwrap();
        let prod = matrix2x2_mul(&m, &inv);
        assert_eq!(prod, Matrix2x2::identity());
    }

    #[test]
    fn test_thurston_s3() {
        let s3 = Manifold3::s3();
        assert_eq!(thurston_geometrization_type(&s3), "S^3");
    }

    #[test]
    fn test_thurston_t3() {
        let t3 = Manifold3::t3();
        assert_eq!(thurston_geometrization_type(&t3), "E^3");
    }

    #[test]
    fn test_thurston_figure_eight() {
        let m = Manifold3::new("figure-eight knot complement", None, true, false, 1);
        assert_eq!(thurston_geometrization_type(&m), "H^3");
    }

    #[test]
    fn test_thurston_trefoil_complement() {
        let m = Manifold3::new("trefoil knot complement", None, true, false, 1);
        assert_eq!(thurston_geometrization_type(&m), "SL2R-tilde");
    }

    #[test]
    fn test_extended_gcd() {
        let (g, s, t) = extended_gcd(12, 8);
        assert_eq!(g, 4);
        assert_eq!(12 * s + 8 * t, 4);
    }

    #[test]
    fn test_bezout_coefficients() {
        let (s, t) = bezout_coefficients(3, 7);
        assert_eq!(3 * s + 7 * t, gcd(3, 7) as i64);
    }

    #[test]
    fn test_surgery_matrix_in_sl2z() {
        let m = Matrix2x2::surgery_matrix(5, 3);
        assert!(is_in_sl2z(&m));
    }

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 5), 5);
    }
}
