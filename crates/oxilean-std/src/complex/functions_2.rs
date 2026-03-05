//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::f64::consts::PI;

use super::types::*;

/// Argument principle: number of zeros minus poles inside contour.
#[allow(dead_code)]
pub fn argument_principle_zeros_minus_poles(
    contour_winding: i32,
    n_zeros: i32,
    n_poles: i32,
) -> bool {
    contour_winding == n_zeros - n_poles
}
#[cfg(test)]
mod tests_complex_extra {
    use super::*;
    #[test]
    fn test_c64_arithmetic() {
        let a = C64::new(1.0, 2.0);
        let b = C64::new(3.0, 4.0);
        let sum = a.add(&b);
        assert!((sum.re - 4.0).abs() < 1e-12);
        assert!((sum.im - 6.0).abs() < 1e-12);
        let prod = a.mul(&b);
        assert!((prod.re - (-5.0)).abs() < 1e-12);
        assert!((prod.im - 10.0).abs() < 1e-12);
    }
    #[test]
    fn test_c64_modulus_argument() {
        let z = C64::new(1.0, 1.0);
        assert!((z.modulus() - std::f64::consts::SQRT_2).abs() < 1e-12);
        assert!((z.argument() - std::f64::consts::PI / 4.0).abs() < 1e-12);
    }
    #[test]
    fn test_c64_exp() {
        let pi_i = C64::new(0.0, std::f64::consts::PI);
        let result = pi_i.exp();
        assert!((result.re + 1.0).abs() < 1e-12);
        assert!(result.im.abs() < 1e-12);
    }
    #[test]
    fn test_c64_sqrt() {
        let z = C64::new(-1.0, 0.0);
        let s = z.sqrt_principal();
        assert!(s.re.abs() < 1e-12);
        assert!((s.im - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_mobius_identity() {
        let id = MobiusMap::identity();
        let z = C64::new(2.0, 3.0);
        let result = id.apply(z).expect("apply should succeed");
        assert!((result.re - z.re).abs() < 1e-12);
        assert!((result.im - z.im).abs() < 1e-12);
    }
    #[test]
    fn test_mobius_compose() {
        let id = MobiusMap::identity();
        let comp = id.compose(&id);
        let z = C64::new(1.0, 1.0);
        let r1 = id.apply(z).expect("apply should succeed");
        let r2 = comp.apply(z).expect("apply should succeed");
        assert!((r1.re - r2.re).abs() < 1e-9);
    }
    #[test]
    fn test_conformal_map() {
        let m = ConformalMap::upper_half_to_disk();
        assert_eq!(m.source_domain, "UpperHalfPlane");
        assert!(m.is_biholomorphic());
    }
}
