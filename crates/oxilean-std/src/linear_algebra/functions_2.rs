//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// qz_decomp : ∀ (n : Nat) (A B : Matrix n n Real),
///   ∃ Q Z S T, Q * A * Z = S ∧ Q * B * Z = T ∧ IsUpperTriangular S ∧ IsUpperTriangular T
pub fn qz_decomp_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "A",
            app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
            pi(
                BinderInfo::Default,
                "B",
                app3(cst("Matrix"), bvar(1), bvar(1), cst("Real")),
                app(
                    cst("QZDecomposition"),
                    app2(cst("MatrixPencil.mk"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Add all advanced linear algebra axioms to the environment.
#[allow(dead_code)]
pub fn register_advanced_linear_algebra(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("TensorProduct", tensor_product_type_ty()),
        ("tensor_mk", tensor_mk_ty()),
        ("tensor_universal", tensor_universal_ty()),
        ("tensor_naturality", tensor_naturality_ty()),
        ("tensor_assoc", tensor_assoc_ty()),
        ("ExteriorAlgebra", exterior_algebra_type_ty()),
        ("exterior_wedge", exterior_wedge_ty()),
        ("exterior_graded", exterior_graded_ty()),
        ("exterior_anticomm", exterior_anticomm_ty()),
        ("SymmetricPower", symmetric_power_ty()),
        ("ExteriorPower", exterior_power_ty()),
        ("FreeModule", free_module_type_ty()),
        ("free_module_basis", free_module_basis_ty()),
        ("IsProjective", is_projective_ty()),
        ("IsInjective", is_injective_ty()),
        ("nakayama_lemma", nakayama_lemma_ty()),
        ("IsNormalOperator", is_normal_operator_ty()),
        ("spectral_decomposition", spectral_decomposition_ty()),
        ("functional_calculus", functional_calculus_ty()),
        ("spectrum", spectrum_ty()),
        ("spectral_radius", spectral_radius_ty()),
        ("CStarAlgebra", cstar_algebra_pred_ty()),
        ("VonNeumannAlgebra", von_neumann_algebra_pred_ty()),
        ("double_commutant", double_commutant_ty()),
        ("IsComplete", is_complete_ty()),
        ("BanachSpace", banach_space_pred_ty()),
        ("HilbertSpace", hilbert_space_pred_ty()),
        ("open_mapping_theorem", open_mapping_ty()),
        ("closed_graph_theorem", closed_graph_ty()),
        ("riesz_representation", riesz_representation_ty()),
        ("ConditionNumber", condition_number_ty()),
        ("IsBackwardStable", is_backward_stable_ty()),
        ("SingularValues", singular_values_ty()),
        ("svd_decomp", svd_decomp_ty()),
        ("FiniteField", finite_field_ty()),
        ("finite_field_order", finite_field_order_ty()),
        (
            "vector_space_fin_field_dim",
            vector_space_fin_field_dim_ty(),
        ),
        ("IsToeplitz", is_toeplitz_ty()),
        ("IsCirculant", is_circulant_ty()),
        ("IsCauchy", is_cauchy_ty()),
        ("circulant_diagonalizable", circulant_diagonalizable_ty()),
        ("StiefelManifold", stiefel_manifold_ty()),
        ("GrassmannManifold", grassmann_manifold_ty()),
        ("OrthogonalGroup", orthogonal_group_ty()),
        ("grassmann_is_homogeneous", grassmann_is_homogeneous_ty()),
        ("FredholmOperator", fredholm_operator_ty()),
        ("fredholm_index", fredholm_index_ty()),
        ("TraceClass", trace_class_ty()),
        ("operator_trace", operator_trace_ty()),
        (
            "linear_recurrence_solution",
            linear_recurrence_solution_ty(),
        ),
        ("companion_matrix", companion_matrix_ty()),
        (
            "cayley_hamilton_minimal_poly",
            cayley_hamilton_minimal_poly_ty(),
        ),
        ("sylvester_eq_solution", sylvester_eq_solution_ty()),
        ("lyapunov_eq", lyapunov_eq_ty()),
        ("MatrixPencil", matrix_pencil_ty()),
        ("pencil_eigenvalue", pencil_eigenvalue_ty()),
        ("qz_decomp", qz_decomp_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_zero() {
        let m = DenseMatrix::zero(3, 3);
        assert_eq!(m.trace(), 0.0);
        assert!(m.det().map(|d| d.abs() < 1e-12).unwrap_or(false));
    }
    #[test]
    fn test_matrix_identity() {
        let m = DenseMatrix::identity(3);
        assert!((m.trace() - 3.0).abs() < 1e-12);
        assert!((m.det().expect("det should succeed") - 1.0).abs() < 1e-12);
        assert!(m.is_symmetric());
    }
    #[test]
    fn test_matrix_mul_identity() {
        let id = DenseMatrix::identity(3);
        let a = DenseMatrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
            ],
        };
        let result = a.mul(&id).expect("mul should succeed");
        for i in 0..3 {
            for j in 0..3 {
                assert!((result.data[i][j] - a.data[i][j]).abs() < 1e-12);
            }
        }
    }
    #[test]
    fn test_matrix_det_2x2() {
        let m = DenseMatrix {
            rows: 2,
            cols: 2,
            data: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        };
        assert!((m.det().expect("det should succeed") - (-2.0)).abs() < 1e-12);
    }
    #[test]
    fn test_matrix_det_3x3() {
        let m = DenseMatrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![0.0, 1.0, 4.0],
                vec![5.0, 6.0, 0.0],
            ],
        };
        let d = m.det().expect("det should succeed");
        assert!((d - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_matrix_rank() {
        let m = DenseMatrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0],
            ],
        };
        assert_eq!(m.rank(), 2);
    }
    #[test]
    fn test_matrix_solve() {
        let a = DenseMatrix {
            rows: 2,
            cols: 2,
            data: vec![vec![2.0, 1.0], vec![1.0, 3.0]],
        };
        let b = vec![5.0, 10.0];
        let x = a.solve(&b).expect("solve should succeed");
        assert!((x[0] - 1.0).abs() < 1e-10);
        assert!((x[1] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_vector_dot() {
        let u = DenseVector {
            data: vec![1.0, 2.0, 3.0],
        };
        let v = DenseVector {
            data: vec![4.0, 5.0, 6.0],
        };
        assert!((u.dot(&v) - 32.0).abs() < 1e-12);
    }
    #[test]
    fn test_vector_norm() {
        let v = DenseVector {
            data: vec![3.0, 4.0],
        };
        assert!((v.norm() - 5.0).abs() < 1e-12);
    }
    #[test]
    fn test_vector_normalize() {
        let v = DenseVector {
            data: vec![3.0, 4.0],
        };
        let n = v.normalize().expect("normalize should succeed");
        assert!((n.norm() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_matrix_apply() {
        let id = DenseMatrix::identity(3);
        let v = DenseVector {
            data: vec![1.0, 2.0, 3.0],
        };
        let w = DenseVector::apply(&id, &v).expect("DenseVector::apply should succeed");
        for i in 0..3 {
            assert!((w.data[i] - v.data[i]).abs() < 1e-12);
        }
    }
    #[test]
    fn test_build_linear_algebra_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Add"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Mul"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Zero"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("One"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("CommRing"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Field"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Real"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Polynomial"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("InnerProductSpace"),
            univ_params: vec![],
            ty: arrow(type0(), arrow(type0(), type0())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("NormedAddCommGroup"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("MetricSpace"),
            univ_params: vec![],
            ty: arrow(type0(), type0()),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let result = build_linear_algebra_env(&mut env);
        assert!(
            result.is_ok(),
            "build_linear_algebra_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_register_advanced_linear_algebra() {
        let mut env = Environment::new();
        register_advanced_linear_algebra(&mut env);
    }
    #[test]
    fn test_sparse_matrix_matvec() {
        let entries = vec![(0, 0, 2.0), (1, 1, 3.0)];
        let a = SparseMatrix::from_coo(2, 2, &entries);
        assert_eq!(a.nnz(), 2);
        let y = a.matvec(&[1.0, 1.0]).expect("matvec should succeed");
        assert!((y[0] - 2.0).abs() < 1e-12);
        assert!((y[1] - 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_sparse_matrix_transpose() {
        let entries = vec![(0, 0, 1.0), (0, 1, 2.0), (1, 1, 3.0)];
        let a = SparseMatrix::from_coo(2, 2, &entries);
        let at = a.transpose();
        assert!((at.get(0, 0) - 1.0).abs() < 1e-12);
        assert!((at.get(1, 0) - 2.0).abs() < 1e-12);
        assert!((at.get(1, 1) - 3.0).abs() < 1e-12);
        assert!((at.get(0, 1)).abs() < 1e-12);
    }
    #[test]
    fn test_banded_matrix_matvec() {
        let mut b = BandedMatrix::zero(3, 3, 1, 1);
        b.set(0, 0, 2.0);
        b.set(0, 1, -1.0);
        b.set(1, 0, -1.0);
        b.set(1, 1, 2.0);
        b.set(1, 2, -1.0);
        b.set(2, 1, -1.0);
        b.set(2, 2, 2.0);
        let y = b.matvec(&[1.0, 0.0, 0.0]).expect("matvec should succeed");
        assert!((y[0] - 2.0).abs() < 1e-12);
        assert!((y[1] - (-1.0)).abs() < 1e-12);
        assert!((y[2]).abs() < 1e-12);
    }
    #[test]
    fn test_banded_matrix_diagonal() {
        let mut b = BandedMatrix::zero(3, 3, 1, 1);
        b.set(0, 0, 5.0);
        b.set(1, 1, 6.0);
        b.set(2, 2, 7.0);
        let d = b.diagonal();
        assert_eq!(d.len(), 3);
        assert!((d[0] - 5.0).abs() < 1e-12);
        assert!((d[1] - 6.0).abs() < 1e-12);
        assert!((d[2] - 7.0).abs() < 1e-12);
    }
    #[test]
    fn test_qr_decomposition_square() {
        let a = DenseMatrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![12.0, -51.0, 4.0],
                vec![6.0, 167.0, -68.0],
                vec![-4.0, 24.0, -41.0],
            ],
        };
        let qr = QRDecomposition::compute(&a).expect("QRDecomposition::compute should succeed");
        let qt = qr.q.transpose();
        let qtq = qt.mul(&qr.q).expect("mul should succeed");
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (qtq.data[i][j] - expected).abs() < 1e-8,
                    "QtQ[{}][{}] = {} ≠ {}",
                    i,
                    j,
                    qtq.data[i][j],
                    expected
                );
            }
        }
        for i in 0..3 {
            for j in 0..i {
                assert!(
                    qr.r.data[i][j].abs() < 1e-8,
                    "R[{}][{}] = {} should be 0",
                    i,
                    j,
                    qr.r.data[i][j]
                );
            }
        }
        let qr_prod = qr.q.mul(&qr.r).expect("mul should succeed");
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (qr_prod.data[i][j] - a.data[i][j]).abs() < 1e-6,
                    "QR[{}][{}] = {} ≠ A[{}][{}] = {}",
                    i,
                    j,
                    qr_prod.data[i][j],
                    i,
                    j,
                    a.data[i][j]
                );
            }
        }
    }
    #[test]
    fn test_qr_solve() {
        let a = DenseMatrix {
            rows: 3,
            cols: 2,
            data: vec![vec![1.0, 1.0], vec![1.0, 2.0], vec![1.0, 3.0]],
        };
        let b = vec![6.0, 5.0, 7.0];
        let qr = QRDecomposition::compute(&a).expect("QRDecomposition::compute should succeed");
        let x = qr.solve(&b).expect("solve should succeed");
        assert_eq!(x.len(), 2);
        let ax0 = a.data[0][0] * x[0] + a.data[0][1] * x[1];
        let ax1 = a.data[1][0] * x[0] + a.data[1][1] * x[1];
        let ax2 = a.data[2][0] * x[0] + a.data[2][1] * x[1];
        let residual = (ax0 - b[0]).powi(2) + (ax1 - b[1]).powi(2) + (ax2 - b[2]).powi(2);
        assert!(residual < 10.0, "residual = {}", residual);
    }
    #[test]
    fn test_lanczos_tridiagonal() {
        let a = DenseMatrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![4.0, 1.0, 0.0],
                vec![1.0, 3.0, 1.0],
                vec![0.0, 1.0, 2.0],
            ],
        };
        let v0 = DenseVector {
            data: vec![1.0, 0.0, 0.0],
        };
        let result =
            LanczosResult::compute(&a, &v0, 3).expect("LanczosResult::compute should succeed");
        assert!(!result.alpha.is_empty());
        let t = result.tridiagonal();
        assert!(t.is_symmetric());
    }
}
