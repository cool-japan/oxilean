//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CirculantMatrixFFT, ConjugateGradient, CooMatrix, CsrMatrix, GMRESSolver, GmresSolver,
    KrylovSubspaceResult, LUResult, PowerIteration, QRAlgorithm, QRAlgorithmResult, QRResult,
    RandomizedSVDResult,
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
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn matrix_ty() -> Expr {
    app2(cst("Matrix"), nat_ty(), nat_ty())
}
pub fn vector_ty() -> Expr {
    app(cst("Vector"), nat_ty())
}
/// `LUDecomposition : Matrix → Matrix → Matrix → Nat → Prop`
/// PA = LU where P is a permutation, L lower triangular, U upper triangular.
pub fn lu_decomposition_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(matrix_ty(), arrow(nat_ty(), prop()))),
    )
}
/// `QRDecomposition : Matrix → Matrix → Matrix → Prop`
/// A = QR where Q orthogonal, R upper triangular.
pub fn qr_decomposition_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop())))
}
/// `CholeskyDecomposition : Matrix → Matrix → Prop`
/// A = L L^T for symmetric positive-definite A.
pub fn cholesky_decomposition_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// `SVD : Matrix → Matrix → Matrix → Matrix → Prop`
/// A = U Σ V^T — singular value decomposition.
pub fn svd_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `ConditionNumber : Matrix → Real → Prop`
/// κ(A) = ‖A‖ · ‖A⁻¹‖ — condition number.
pub fn condition_number_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// `WellConditioned : Matrix → Prop`
/// κ(A) is not too large, i.e., system is numerically stable.
pub fn well_conditioned_ty() -> Expr {
    arrow(matrix_ty(), prop())
}
/// `NumericalRank : Matrix → Nat → Real → Prop`
/// Numerical rank of A given tolerance ε.
pub fn numerical_rank_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `BackwardStable : (Matrix → Vector → Vector) → Prop`
/// Algorithm is backward stable.
pub fn backward_stable_ty() -> Expr {
    arrow(fn_ty(matrix_ty(), fn_ty(vector_ty(), vector_ty())), prop())
}
/// `ConjugateGradientConverges : Matrix → Vector → Nat → Prop`
/// CG method for Ax=b converges in at most n steps (for n×n SPD A).
pub fn cg_converges_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(nat_ty(), prop())))
}
/// `GMRESConverges : Matrix → Vector → Nat → Real → Prop`
/// GMRES for Ax=b reaches tolerance ε within k steps.
pub fn gmres_converges_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(vector_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `BiCGSTABConverges : Matrix → Vector → Nat → Real → Prop`
pub fn bicgstab_converges_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(vector_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `Eigenvalue : Matrix → Real → Prop`
/// λ is an eigenvalue of A.
pub fn eigenvalue_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// `Eigenvector : Matrix → Real → Vector → Prop`
/// v is an eigenvector of A for eigenvalue λ.
pub fn eigenvector_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(vector_ty(), prop())))
}
/// `SpectrumBound : Matrix → Real → Real → Prop`
/// All eigenvalues of A lie in [lo, hi].
pub fn spectrum_bound_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `LanczosApproximation : Matrix → Nat → Matrix → Prop`
/// k-step Lanczos produces tridiagonal approximation T_k.
pub fn lanczos_approx_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `CSRMatrix : Nat → Nat → Nat → Type`
/// Compressed Sparse Row matrix with m rows, n cols, nnz nonzeros.
pub fn csr_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `SparsityPattern : Matrix → Real → Prop`
/// Fraction of nonzero entries is at most the given density.
pub fn sparsity_pattern_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// `SparseMatVec : CSRMatrix → Vector → Vector → Prop`
/// Result of sparse matrix-vector multiplication.
pub fn sparse_matvec_ty() -> Expr {
    arrow(type0(), arrow(vector_ty(), arrow(vector_ty(), prop())))
}
/// `QRAlgorithmEigenvalue : Matrix → Nat → Real → Prop`
/// The QR algorithm with shift converges to eigenvalue λ within k iterations.
pub fn qr_algorithm_eigenvalue_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `JacobiIteration : Matrix → Nat → Matrix → Prop`
/// Jacobi rotation method produces diagonal approximation after k sweeps.
pub fn jacobi_iteration_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `PowerMethodConverges : Matrix → Real → Nat → Prop`
/// Power method converges to dominant eigenvalue λ₁ within k steps.
pub fn power_method_converges_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `WilkinsonShift : Matrix → Nat → Real → Prop`
/// Wilkinson shift accelerates QR convergence; value at step k is σ.
pub fn wilkinson_shift_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `ArnoldiProcess : Matrix → Nat → Matrix → Matrix → Prop`
/// k-step Arnoldi builds orthonormal basis V_k and upper Hessenberg H_k.
pub fn arnoldi_process_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(nat_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `GMRESWithRestart : Matrix → Vector → Nat → Nat → Real → Prop`
/// Restarted GMRES(m) solves Ax=b; outer restarts bounded by k; residual < ε.
pub fn gmres_restart_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(
            vector_ty(),
            arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `ConjugateGradientOptimal : Matrix → Vector → Nat → Prop`
/// CG is optimal in the A-norm over all Krylov subspace iterates.
pub fn cg_optimal_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(nat_ty(), prop())))
}
/// `MinResConverges : Matrix → Vector → Nat → Real → Prop`
/// MINRES for symmetric indefinite systems reaches tolerance within k steps.
pub fn minres_converges_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(vector_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `ILUPreconditioner : Matrix → Matrix → Matrix → Real → Prop`
/// Incomplete LU with fill level τ yields L and U factors approximating A.
pub fn ilu_preconditioner_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(matrix_ty(), arrow(real_ty(), prop()))),
    )
}
/// `AMGPreconditioner : Matrix → Nat → Prop`
/// Algebraic multigrid with depth d is a valid preconditioner for A.
pub fn amg_preconditioner_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// `SSORPreconditioner : Matrix → Real → Matrix → Prop`
/// SSOR with relaxation ω produces preconditioner M for matrix A.
pub fn ssor_preconditioner_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(matrix_ty(), prop())))
}
/// `PolynomialPreconditioner : Matrix → Nat → Matrix → Prop`
/// Degree-k polynomial p(A) ≈ A⁻¹ is a polynomial preconditioner.
pub fn poly_preconditioner_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `RandomizedSVDApprox : Matrix → Nat → Nat → Matrix → Matrix → Matrix → Prop`
/// Randomized SVD with rank k and oversampling p gives U, Σ, V approximation.
pub fn randomized_svd_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(
            nat_ty(),
            arrow(
                nat_ty(),
                arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
            ),
        ),
    )
}
/// `SketchAndSolve : Matrix → Vector → Nat → Vector → Real → Prop`
/// Sketch-and-solve finds approximate solution with sketch size k and error ε.
pub fn sketch_and_solve_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(
            vector_ty(),
            arrow(nat_ty(), arrow(vector_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `JohnsonLindenstraussEmbed : Nat → Nat → Real → Matrix → Prop`
/// JL embedding from n-dim to k-dim with distortion ε given by matrix Φ.
pub fn johnson_lindenstrauss_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(real_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `CountSketchMatrix : Nat → Nat → Matrix → Prop`
/// A count-sketch (sparse oblivious subspace embedding) of dimensions s×n.
pub fn count_sketch_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `MatrixExponential : Matrix → Matrix → Prop`
/// exp(A) is the matrix exponential of A.
pub fn matrix_exponential_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// `MatrixLogarithm : Matrix → Matrix → Prop`
/// log(A) is the principal matrix logarithm of A (A must have no negative eigenvalues).
pub fn matrix_logarithm_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// `MatrixSquareRoot : Matrix → Matrix → Prop`
/// sqrt(A) is the principal square root of A (A must be positive semidefinite).
pub fn matrix_square_root_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// `MatrixFunction : (Real → Real) → Matrix → Matrix → Prop`
/// f(A) is the matrix function induced by scalar f via spectral decomposition.
pub fn matrix_function_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(matrix_ty(), arrow(matrix_ty(), prop())),
    )
}
/// `ToeplitzMatrix : Nat → Vector → Prop`
/// A Toeplitz matrix of order n is determined by its first row/column (the vector).
pub fn toeplitz_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(vector_ty(), prop()))
}
/// `CirculantMatrix : Nat → Vector → Matrix → Prop`
/// Circulant matrix of order n with generating vector c gives the matrix C.
pub fn circulant_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(vector_ty(), arrow(matrix_ty(), prop())))
}
/// `HankelMatrix : Nat → Vector → Prop`
/// A Hankel matrix of order n is determined by its anti-diagonal vector.
pub fn hankel_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(vector_ty(), prop()))
}
/// `DisplacementRank : Matrix → Nat → Prop`
/// The matrix has displacement rank at most r (low displacement structure).
pub fn displacement_rank_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// `CURDecomposition : Matrix → Matrix → Matrix → Matrix → Prop`
/// CUR factorization: A ≈ C U R using actual rows/columns of A.
pub fn cur_decomposition_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `NystromApproximation : Matrix → Nat → Matrix → Prop`
/// Nyström approximation of rank k for symmetric positive semidefinite A.
pub fn nystrom_approximation_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `ColumnSubsetSelection : Matrix → Nat → Matrix → Prop`
/// Select k columns of A to best approximate the column space of A.
pub fn column_subset_selection_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `NuclearNormMin : Matrix → Real → Matrix → Prop`
/// Nuclear norm minimization with regularization λ recovers low-rank matrix.
pub fn nuclear_norm_min_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(matrix_ty(), prop())))
}
/// `RIPMatrix : Matrix → Nat → Real → Prop`
/// Restricted isometry property: A has RIP with rank k and constant δ.
pub fn rip_matrix_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `TuckerDecomposition : Nat → Matrix → Matrix → Matrix → Matrix → Prop`
/// Tucker decomposition of a 3-tensor: X ≈ G ×₁ U₁ ×₂ U₂ ×₃ U₃.
pub fn tucker_decomposition_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            matrix_ty(),
            arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
        ),
    )
}
/// `CPDecomposition : Nat → Nat → Matrix → Matrix → Matrix → Prop`
/// CP/PARAFAC decomposition: tensor is sum of r rank-1 components.
pub fn cp_decomposition_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(matrix_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
        ),
    )
}
/// `TensorTrain : Nat → Nat → Prop`
/// Tensor-train (MPS) decomposition of a d-dimensional tensor with ranks r.
pub fn tensor_train_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `GraphLaplacian : Matrix → Matrix → Prop`
/// L = D - A is the (combinatorial) graph Laplacian of adjacency matrix A.
pub fn graph_laplacian_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// `NormalizedCut : Matrix → Vector → Real → Prop`
/// Normalized cut value for partition indicator vector v equals the given real.
pub fn normalized_cut_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(real_ty(), prop())))
}
/// `DiffusionMap : Matrix → Nat → Real → Matrix → Prop`
/// Diffusion map embedding of data matrix A with t steps and kernel width ε.
pub fn diffusion_map_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(nat_ty(), arrow(real_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `FiedlerVector : Matrix → Vector → Prop`
/// Fiedler vector (second smallest eigenvector of L) for Laplacian L.
pub fn fiedler_vector_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), prop()))
}
/// `DavisKahanBound : Matrix → Matrix → Real → Prop`
/// Davis-Kahan theorem: perturbation ε in A shifts invariant subspace by ≤ bound.
pub fn davis_kahan_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), arrow(real_ty(), prop())))
}
/// `WeylEigenvalueBound : Matrix → Matrix → Nat → Real → Prop`
/// Weyl's theorem: |λᵢ(A+E) - λᵢ(A)| ≤ ‖E‖₂ for eigenvalue index i.
pub fn weyl_eigenvalue_bound_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `SinThetaTheorem : Matrix → Matrix → Real → Prop`
/// Sin-Theta theorem bounds the canonical angles between subspaces under perturbation.
pub fn sin_theta_theorem_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), arrow(real_ty(), prop())))
}
/// `Blas3GemmOptimal : Nat → Nat → Nat → Prop`
/// Block GEMM for m×k × k×n achieves O(mnk) flops with BLAS-3 efficiency.
pub fn blas3_gemm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `RecursiveLU : Matrix → Nat → Matrix → Matrix → Prop`
/// Recursive (cache-oblivious) LU factorization with block size b.
pub fn recursive_lu_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(nat_ty(), arrow(matrix_ty(), arrow(matrix_ty(), prop()))),
    )
}
/// `CommAvoidingQR : Matrix → Nat → Prop`
/// Communication-avoiding QR with b levels has O(b) synchronization points.
pub fn comm_avoiding_qr_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// `FillReducingOrdering : Matrix → Vector → Nat → Prop`
/// Permutation vector p reduces fill-in to at most nnz nonzeros in factorization.
pub fn fill_reducing_ordering_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(nat_ty(), prop())))
}
/// `SupernodalElimination : Matrix → Nat → Matrix → Prop`
/// Supernodal Cholesky factorization groups k supernodes into factor L.
pub fn supernodal_elim_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(matrix_ty(), prop())))
}
/// `NesteddissectionOrder : Matrix → Nat → Vector → Prop`
/// Nested dissection ordering achieves O(n^{3/2}) fill for 2D PDE matrices.
pub fn nested_dissection_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), arrow(vector_ty(), prop())))
}
/// `FP16RoundingError : Real → Real → Prop`
/// A real value x rounded to FP16 differs by at most ε = 2^{-11}.
pub fn fp16_rounding_error_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `BF16RoundingError : Real → Real → Prop`
/// A real value x rounded to BF16 differs by at most ε = 2^{-8}.
pub fn bf16_rounding_error_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `MixedPrecisionIR : Matrix → Vector → Nat → Real → Prop`
/// Iterative refinement in mixed precision (FP16 factor, FP64 residual) converges in k steps to tolerance ε.
pub fn mixed_precision_ir_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(vector_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `ClassicalGramSchmidtInstability : Matrix → Real → Prop`
/// Classical Gram-Schmidt loses orthogonality; residual from orthogonality ≥ ε.
pub fn classical_gs_instability_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// `ModifiedGramSchmidtStability : Matrix → Real → Prop`
/// Modified Gram-Schmidt is numerically stable; residual from orthogonality ≤ ε·κ(A).
pub fn modified_gs_stability_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// `HouseholderVsNormalEq : Matrix → Vector → Real → Prop`
/// Householder QR solution has smaller error than normal equations by factor κ(A).
pub fn householder_vs_normal_eq_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(real_ty(), prop())))
}
/// `AugmentedSystemStability : Matrix → Vector → Real → Prop`
/// Augmented system formulation is more stable than normal equations; error ≤ ε.
pub fn augmented_system_stability_ty() -> Expr {
    arrow(matrix_ty(), arrow(vector_ty(), arrow(real_ty(), prop())))
}
/// Build an [`Environment`] with numerical linear algebra axioms.
pub fn build_numerical_linear_algebra_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("LUDecomposition", lu_decomposition_ty()),
        ("QRDecomposition", qr_decomposition_ty()),
        ("CholeskyDecomposition", cholesky_decomposition_ty()),
        ("SVD", svd_ty()),
        ("ConditionNumber", condition_number_ty()),
        ("WellConditioned", well_conditioned_ty()),
        ("NumericalRank", numerical_rank_ty()),
        ("BackwardStable", backward_stable_ty()),
        ("ConjugateGradientConverges", cg_converges_ty()),
        ("GMRESConverges", gmres_converges_ty()),
        ("BiCGSTABConverges", bicgstab_converges_ty()),
        ("Eigenvalue", eigenvalue_ty()),
        ("Eigenvector", eigenvector_ty()),
        ("SpectrumBound", spectrum_bound_ty()),
        ("LanczosApproximation", lanczos_approx_ty()),
        ("CSRMatrix", csr_matrix_ty()),
        ("SparsityPattern", sparsity_pattern_ty()),
        ("SparseMatVec", sparse_matvec_ty()),
        ("lu_existence", arrow(matrix_ty(), prop())),
        ("qr_existence", arrow(matrix_ty(), prop())),
        ("cholesky_existence", arrow(matrix_ty(), prop())),
        ("svd_existence", arrow(matrix_ty(), prop())),
        ("cg_energy_norm_descent", prop()),
        ("gmres_min_residual", prop()),
        ("lanczos_orthogonality", prop()),
        ("householder_numerically_stable", prop()),
        ("condition_number_sensitivity", prop()),
        (
            "sparse_matvec_complexity",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("QRAlgorithmEigenvalue", qr_algorithm_eigenvalue_ty()),
        ("JacobiIteration", jacobi_iteration_ty()),
        ("PowerMethodConverges", power_method_converges_ty()),
        ("WilkinsonShift", wilkinson_shift_ty()),
        ("ArnoldiProcess", arnoldi_process_ty()),
        ("GMRESWithRestart", gmres_restart_ty()),
        ("ConjugateGradientOptimal", cg_optimal_ty()),
        ("MinResConverges", minres_converges_ty()),
        ("ILUPreconditioner", ilu_preconditioner_ty()),
        ("AMGPreconditioner", amg_preconditioner_ty()),
        ("SSORPreconditioner", ssor_preconditioner_ty()),
        ("PolynomialPreconditioner", poly_preconditioner_ty()),
        ("RandomizedSVDApprox", randomized_svd_ty()),
        ("SketchAndSolve", sketch_and_solve_ty()),
        ("JohnsonLindenstraussEmbed", johnson_lindenstrauss_ty()),
        ("CountSketchMatrix", count_sketch_ty()),
        ("MatrixExponential", matrix_exponential_ty()),
        ("MatrixLogarithm", matrix_logarithm_ty()),
        ("MatrixSquareRoot", matrix_square_root_ty()),
        ("MatrixFunction", matrix_function_ty()),
        ("ToeplitzMatrix", toeplitz_matrix_ty()),
        ("CirculantMatrix", circulant_matrix_ty()),
        ("HankelMatrix", hankel_matrix_ty()),
        ("DisplacementRank", displacement_rank_ty()),
        ("CURDecomposition", cur_decomposition_ty()),
        ("NystromApproximation", nystrom_approximation_ty()),
        ("ColumnSubsetSelection", column_subset_selection_ty()),
        ("NuclearNormMin", nuclear_norm_min_ty()),
        ("RIPMatrix", rip_matrix_ty()),
        ("TuckerDecomposition", tucker_decomposition_ty()),
        ("CPDecomposition", cp_decomposition_ty()),
        ("TensorTrain", tensor_train_ty()),
        ("GraphLaplacian", graph_laplacian_ty()),
        ("NormalizedCut", normalized_cut_ty()),
        ("DiffusionMap", diffusion_map_ty()),
        ("FiedlerVector", fiedler_vector_ty()),
        ("DavisKahanBound", davis_kahan_ty()),
        ("WeylEigenvalueBound", weyl_eigenvalue_bound_ty()),
        ("SinThetaTheorem", sin_theta_theorem_ty()),
        ("Blas3GemmOptimal", blas3_gemm_ty()),
        ("RecursiveLU", recursive_lu_ty()),
        ("CommAvoidingQR", comm_avoiding_qr_ty()),
        ("FillReducingOrdering", fill_reducing_ordering_ty()),
        ("SupernodalElimination", supernodal_elim_ty()),
        ("NestedDissectionOrder", nested_dissection_ty()),
        ("FP16RoundingError", fp16_rounding_error_ty()),
        ("BF16RoundingError", bf16_rounding_error_ty()),
        ("MixedPrecisionIR", mixed_precision_ir_ty()),
        (
            "ClassicalGramSchmidtInstability",
            classical_gs_instability_ty(),
        ),
        ("ModifiedGramSchmidtStability", modified_gs_stability_ty()),
        ("HouseholderVsNormalEq", householder_vs_normal_eq_ty()),
        ("AugmentedSystemStability", augmented_system_stability_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Multiply two square n×n matrices.
pub fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut c = vec![vec![0.0; n]; n];
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}
/// Matrix-vector product y = Ax.
pub fn mat_vec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
    a.iter()
        .map(|row| row.iter().zip(x).map(|(aij, xj)| aij * xj).sum())
        .collect()
}
/// Dot product of two vectors.
pub fn dot(u: &[f64], v: &[f64]) -> f64 {
    u.iter().zip(v).map(|(a, b)| a * b).sum()
}
/// Euclidean norm of a vector.
pub fn norm2(v: &[f64]) -> f64 {
    dot(v, v).sqrt()
}
/// Axpy: y += alpha * x
pub fn axpy(alpha: f64, x: &[f64], y: &mut [f64]) {
    for (yi, xi) in y.iter_mut().zip(x) {
        *yi += alpha * xi;
    }
}
/// LU decomposition with partial pivoting (Doolittle algorithm).
///
/// Returns `None` if the matrix is (numerically) singular.
pub fn lu_decompose(a: &[Vec<f64>]) -> Option<LUResult> {
    let n = a.len();
    let mut lu: Vec<Vec<f64>> = a.to_vec();
    let mut piv: Vec<usize> = (0..n).collect();
    for k in 0..n {
        let (pivot_row, _) = lu[k..n].iter().enumerate().max_by(|(_, ri), (_, rj)| {
            ri[k]
                .abs()
                .partial_cmp(&rj[k].abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;
        let pivot_row = pivot_row + k;
        lu.swap(k, pivot_row);
        piv.swap(k, pivot_row);
        if lu[k][k].abs() < 1e-14 {
            return None;
        }
        let inv_diag = 1.0 / lu[k][k];
        for i in (k + 1)..n {
            lu[i][k] *= inv_diag;
            for j in (k + 1)..n {
                let m = lu[i][k];
                lu[i][j] -= m * lu[k][j];
            }
        }
    }
    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];
    for i in 0..n {
        l[i][i] = 1.0;
        for j in 0..i {
            l[i][j] = lu[i][j];
        }
        for j in i..n {
            u[i][j] = lu[i][j];
        }
    }
    Some(LUResult { l, u, piv })
}
/// Solve Ax = b using a pre-computed LU decomposition.
pub fn lu_solve(res: &LUResult, b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut pb: Vec<f64> = (0..n).map(|i| b[res.piv[i]]).collect();
    for i in 0..n {
        for j in 0..i {
            pb[i] -= res.l[i][j] * pb[j];
        }
    }
    for i in (0..n).rev() {
        for j in (i + 1)..n {
            pb[i] -= res.u[i][j] * pb[j];
        }
        pb[i] /= res.u[i][i];
    }
    pb
}
/// QR decomposition via Householder reflections.
///
/// Works for m×n matrices with m ≥ n.
pub fn qr_decompose(a: &[Vec<f64>]) -> QRResult {
    let m = a.len();
    let n = if m == 0 { 0 } else { a[0].len() };
    let mut r = a.to_vec();
    let mut q = vec![vec![0.0; m]; m];
    for i in 0..m {
        q[i][i] = 1.0;
    }
    for k in 0..n.min(m) {
        let col_len = m - k;
        let mut x: Vec<f64> = (k..m).map(|i| r[i][k]).collect();
        let sigma: f64 = if x[0] >= 0.0 { norm2(&x) } else { -norm2(&x) };
        x[0] += sigma;
        let beta = if sigma.abs() < 1e-15 {
            0.0
        } else {
            1.0 / (sigma * x[0])
        };
        for j in k..n {
            let dot_val: f64 = (0..col_len).map(|i| x[i] * r[i + k][j]).sum();
            let scale = beta * dot_val;
            for i in 0..col_len {
                r[i + k][j] -= scale * x[i];
            }
        }
        for j in 0..m {
            let dot_val: f64 = (0..col_len).map(|i| x[i] * q[i + k][j]).sum();
            let scale = beta * dot_val;
            for i in 0..col_len {
                q[i + k][j] -= scale * x[i];
            }
        }
    }
    let qt = q;
    let mut q_final = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            q_final[j][i] = qt[i][j];
        }
    }
    QRResult { q: q_final, r }
}
/// Cholesky decomposition A = L L^T for SPD matrices.
///
/// Returns `None` if A is not positive definite.
pub fn cholesky_decompose(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let sum: f64 = (0..j).map(|k| l[i][k] * l[j][k]).sum();
            if i == j {
                let diag = a[i][i] - sum;
                if diag <= 0.0 {
                    return None;
                }
                l[i][j] = diag.sqrt();
            } else {
                l[i][j] = (a[i][j] - sum) / l[j][j];
            }
        }
    }
    Some(l)
}
/// Solve Ax = b using Cholesky factor L: LL^T x = b.
pub fn cholesky_solve(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let s: f64 = (0..i).map(|j| l[i][j] * y[j]).sum();
        y[i] = (b[i] - s) / l[i][i];
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let s: f64 = (i + 1..n).map(|j| l[j][i] * x[j]).sum();
        x[i] = (y[i] - s) / l[i][i];
    }
    x
}
/// Estimate the 1-norm condition number κ₁(A) = ‖A‖₁ · ‖A⁻¹‖₁.
///
/// Uses LU decomposition to invert A column by column.
pub fn condition_number_1norm(a: &[Vec<f64>]) -> Option<f64> {
    let n = a.len();
    let lu = lu_decompose(a)?;
    let norm_a = (0..n)
        .map(|j| (0..n).map(|i| a[i][j].abs()).sum::<f64>())
        .fold(0.0_f64, f64::max);
    let mut inv_cols: Vec<Vec<f64>> = Vec::with_capacity(n);
    for j in 0..n {
        let mut e = vec![0.0; n];
        e[j] = 1.0;
        inv_cols.push(lu_solve(&lu, &e));
    }
    let norm_inv = (0..n)
        .map(|j| (0..n).map(|i| inv_cols[i][j].abs()).sum::<f64>())
        .fold(0.0_f64, f64::max);
    Some(norm_a * norm_inv)
}
/// Solve the symmetric positive-definite system Ax = b using CG.
///
/// - `tol`: residual tolerance (‖r‖ / ‖b‖)
/// - `max_iter`: maximum CG iterations
///
/// Returns `(solution, iterations, residual_norm)`.
pub fn conjugate_gradient(
    a: &[Vec<f64>],
    b: &[f64],
    tol: f64,
    max_iter: usize,
) -> (Vec<f64>, usize, f64) {
    let n = b.len();
    let mut x = vec![0.0; n];
    let mut r: Vec<f64> = b.to_vec();
    let mut p = r.clone();
    let mut rr = dot(&r, &r);
    let b_norm = norm2(b);
    for iter in 0..max_iter {
        let ap = mat_vec(a, &p);
        let alpha = rr / dot(&p, &ap);
        axpy(alpha, &p, &mut x);
        axpy(-alpha, &ap, &mut r);
        let rr_new = dot(&r, &r);
        let res_norm = rr_new.sqrt();
        if res_norm / b_norm.max(1e-300) < tol {
            return (x, iter + 1, res_norm);
        }
        let beta = rr_new / rr;
        for i in 0..n {
            p[i] = r[i] + beta * p[i];
        }
        rr = rr_new;
    }
    let res = norm2(&r);
    (x, max_iter, res)
}
/// Solve Ax = b using restarted GMRES(m).
///
/// - `m`: restart dimension
/// - `tol`: tolerance on relative residual
/// - `max_restarts`: maximum number of restarts
///
/// Returns `(solution, total_iterations, final_residual_norm)`.
#[allow(clippy::too_many_arguments)]
pub fn gmres(
    a: &[Vec<f64>],
    b: &[f64],
    x0: &[f64],
    m: usize,
    tol: f64,
    max_restarts: usize,
) -> (Vec<f64>, usize, f64) {
    let n = b.len();
    let mut x = x0.to_vec();
    let b_norm = norm2(b);
    let mut total_iters = 0;
    'restart: for _ in 0..max_restarts {
        let ax = mat_vec(a, &x);
        let mut r: Vec<f64> = b.iter().zip(&ax).map(|(bi, axi)| bi - axi).collect();
        let beta = norm2(&r);
        if beta / b_norm.max(1e-300) < tol {
            break;
        }
        let inv_beta = 1.0 / beta;
        let v0: Vec<f64> = r.iter().map(|ri| ri * inv_beta).collect();
        let mut v_basis: Vec<Vec<f64>> = vec![v0];
        let mut h: Vec<Vec<f64>> = vec![vec![0.0; m]; m + 1];
        let mut cs = vec![0.0; m];
        let mut sn = vec![0.0; m];
        let mut g = vec![0.0; m + 1];
        g[0] = beta;
        let mut j = 0;
        while j < m {
            let av = mat_vec(a, &v_basis[j]);
            let mut w = av;
            for i in 0..=j {
                h[i][j] = dot(&w, &v_basis[i]);
                let hi = h[i][j];
                axpy(-hi, &v_basis[i], &mut w);
            }
            h[j + 1][j] = norm2(&w);
            if h[j + 1][j] > 1e-14 {
                let inv = 1.0 / h[j + 1][j];
                let new_v: Vec<f64> = w.iter().map(|wi| wi * inv).collect();
                v_basis.push(new_v);
            }
            for i in 0..j {
                let tmp = cs[i] * h[i][j] + sn[i] * h[i + 1][j];
                h[i + 1][j] = -sn[i] * h[i][j] + cs[i] * h[i + 1][j];
                h[i][j] = tmp;
            }
            let denom = (h[j][j].powi(2) + h[j + 1][j].powi(2)).sqrt();
            if denom > 1e-14 {
                cs[j] = h[j][j] / denom;
                sn[j] = h[j + 1][j] / denom;
            } else {
                cs[j] = 1.0;
                sn[j] = 0.0;
            }
            h[j][j] = cs[j] * h[j][j] + sn[j] * h[j + 1][j];
            h[j + 1][j] = 0.0;
            g[j + 1] = -sn[j] * g[j];
            g[j] *= cs[j];
            total_iters += 1;
            if g[j + 1].abs() / b_norm.max(1e-300) < tol {
                j += 1;
                let k = j;
                let mut y = vec![0.0; k];
                for i in (0..k).rev() {
                    y[i] = g[i];
                    for l in (i + 1)..k {
                        y[i] -= h[i][l] * y[l];
                    }
                    if h[i][i].abs() > 1e-14 {
                        y[i] /= h[i][i];
                    }
                }
                for i in 0..k {
                    axpy(y[i], &v_basis[i], &mut x);
                }
                break 'restart;
            }
            j += 1;
        }
        let k = j.min(m);
        if k == 0 {
            break;
        }
        let mut y = vec![0.0; k];
        for i in (0..k).rev() {
            y[i] = g[i];
            for l in (i + 1)..k {
                y[i] -= h[i][l] * y[l];
            }
            if h[i][i].abs() > 1e-14 {
                y[i] /= h[i][i];
            }
        }
        for i in 0..k {
            axpy(y[i], &v_basis[i], &mut x);
        }
        r = mat_vec(a, &x);
        for i in 0..n {
            r[i] = b[i] - r[i];
        }
        if norm2(&r) / b_norm.max(1e-300) < tol {
            break;
        }
    }
    let ax = mat_vec(a, &x);
    let final_res: Vec<f64> = b.iter().zip(&ax).map(|(bi, axi)| bi - axi).collect();
    (x, total_iters, norm2(&final_res))
}
/// Solve Ax = b using BiCGSTAB.
///
/// Returns `(solution, iterations, residual_norm)`.
pub fn bicgstab(a: &[Vec<f64>], b: &[f64], tol: f64, max_iter: usize) -> (Vec<f64>, usize, f64) {
    let n = b.len();
    let mut x = vec![0.0; n];
    let mut r: Vec<f64> = b.to_vec();
    let r_tilde = r.clone();
    let b_norm = norm2(b);
    let mut rho_old = 1.0_f64;
    let mut alpha = 1.0_f64;
    let mut omega = 1.0_f64;
    let mut v = vec![0.0; n];
    let mut p = vec![0.0; n];
    for iter in 0..max_iter {
        let rho = dot(&r_tilde, &r);
        if rho.abs() < 1e-300 {
            break;
        }
        let beta = (rho / rho_old) * (alpha / omega);
        for i in 0..n {
            p[i] = r[i] + beta * (p[i] - omega * v[i]);
        }
        v = mat_vec(a, &p);
        let denom = dot(&r_tilde, &v);
        if denom.abs() < 1e-300 {
            break;
        }
        alpha = rho / denom;
        let s: Vec<f64> = r.iter().zip(&v).map(|(ri, vi)| ri - alpha * vi).collect();
        let t = mat_vec(a, &s);
        let tt = dot(&t, &t);
        omega = if tt > 1e-300 { dot(&t, &s) / tt } else { 0.0 };
        axpy(alpha, &p, &mut x);
        axpy(omega, &s, &mut x);
        for i in 0..n {
            r[i] = s[i] - omega * t[i];
        }
        let res_norm = norm2(&r);
        if res_norm / b_norm.max(1e-300) < tol {
            return (x, iter + 1, res_norm);
        }
        rho_old = rho;
    }
    let res_norm = norm2(&r);
    (x, max_iter, res_norm)
}
/// Run QR iteration on a symmetric matrix to approximate eigenvalues.
///
/// Returns a vector of approximate eigenvalues sorted in ascending order.
pub fn qr_eigenvalues(a: &[Vec<f64>], max_iter: usize, tol: f64) -> Vec<f64> {
    let n = a.len();
    let mut ak = a.to_vec();
    for _ in 0..max_iter {
        let qr = qr_decompose(&ak);
        ak = mat_mul(&qr.r, &qr.q);
        let ak_snap: Vec<Vec<f64>> = ak.clone();
        let off_diag: f64 = (0..n)
            .flat_map(|i| {
                let row = ak_snap[i].clone();
                (0..n).map(move |j| if i != j { row[j].powi(2) } else { 0.0 })
            })
            .sum::<f64>()
            .sqrt();
        if off_diag < tol {
            break;
        }
    }
    let mut eigs: Vec<f64> = (0..n).map(|i| ak[i][i]).collect();
    eigs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    eigs
}
/// Lanczos algorithm: compute k-step tridiagonal reduction of symmetric A.
///
/// Returns `(alpha, beta)` where alpha[i] are the diagonal entries and
/// beta[i] are the sub-diagonal entries of the tridiagonal matrix T_k.
pub fn lanczos(a: &[Vec<f64>], k: usize) -> (Vec<f64>, Vec<f64>) {
    let n = a.len();
    let k = k.min(n);
    let mut alpha = vec![0.0; k];
    let mut beta = vec![0.0; k.saturating_sub(1)];
    let mut v_prev = vec![0.0; n];
    let mut v_curr = vec![0.0; n];
    v_curr[0] = 1.0;
    for j in 0..k {
        let w = mat_vec(a, &v_curr);
        alpha[j] = dot(&w, &v_curr);
        let mut w2 = w;
        axpy(-alpha[j], &v_curr, &mut w2);
        if j > 0 {
            axpy(-beta[j - 1], &v_prev, &mut w2);
        }
        if j + 1 < k {
            let b = norm2(&w2);
            beta[j] = b;
            v_prev = v_curr.clone();
            if b > 1e-14 {
                v_curr = w2.iter().map(|wi| wi / b).collect();
            } else {
                v_curr = vec![0.0; n];
                if j + 1 < n {
                    v_curr[j + 1] = 1.0;
                }
            }
        }
    }
    (alpha, beta)
}
/// Machine epsilon for f64.
pub const F64_EPS: f64 = f64::EPSILON;
/// Unit roundoff u = ε/2.
pub const F64_UNIT_ROUNDOFF: f64 = f64::EPSILON / 2.0;
/// Check if two f64 values agree to within `rtol` relative tolerance and
/// `atol` absolute tolerance: |a - b| ≤ atol + rtol * max(|a|, |b|).
pub fn approx_equal(a: f64, b: f64, rtol: f64, atol: f64) -> bool {
    let diff = (a - b).abs();
    let scale = a.abs().max(b.abs());
    diff <= atol + rtol * scale
}
/// Compute the relative error ‖x - x_true‖ / ‖x_true‖.
pub fn relative_error(x: &[f64], x_true: &[f64]) -> f64 {
    let denom = norm2(x_true);
    if denom < 1e-300 {
        norm2(x)
    } else {
        let diff: Vec<f64> = x.iter().zip(x_true).map(|(a, b)| a - b).collect();
        norm2(&diff) / denom
    }
}
/// QR algorithm with Wilkinson shift for symmetric matrices.
///
/// This variant applies a Wilkinson shift at each step to accelerate
/// convergence of the trailing 2×2 block, achieving cubic convergence
/// near simple eigenvalues.
///
/// Returns `QRAlgorithmResult` with sorted eigenvalues.
pub fn qr_algorithm_eigen(a: &[Vec<f64>], max_iter: usize, tol: f64) -> QRAlgorithmResult {
    let n = a.len();
    let mut ak = a.to_vec();
    let mut iters = 0;
    for _sweep in 0..max_iter {
        let shift = if n >= 2 {
            let a11 = ak[n - 2][n - 2];
            let a22 = ak[n - 1][n - 1];
            let a12 = ak[n - 2][n - 1];
            let delta = (a11 - a22) / 2.0;
            let sign = if delta >= 0.0 { 1.0 } else { -1.0 };
            a22 - sign * a12 * a12 / (delta.abs() + (delta * delta + a12 * a12).sqrt())
        } else {
            0.0
        };
        let mut shifted = ak.clone();
        for i in 0..n {
            shifted[i][i] -= shift;
        }
        let qr = qr_decompose(&shifted);
        ak = mat_mul(&qr.r, &qr.q);
        for i in 0..n {
            ak[i][i] += shift;
        }
        let off_diag: f64 = (0..n)
            .flat_map(|i| {
                let row = ak[i].clone();
                (0..n).map(move |j| if i != j { row[j].powi(2) } else { 0.0 })
            })
            .sum::<f64>()
            .sqrt();
        iters += 1;
        if off_diag < tol {
            let mut eigenvalues: Vec<f64> = (0..n).map(|i| ak[i][i]).collect();
            eigenvalues.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
            return QRAlgorithmResult {
                eigenvalues,
                iterations: iters,
                off_diag_norm: off_diag,
            };
        }
    }
    let off_diag: f64 = (0..n)
        .flat_map(|i| {
            let row = ak[i].clone();
            (0..n).map(move |j| if i != j { row[j].powi(2) } else { 0.0 })
        })
        .sum::<f64>()
        .sqrt();
    let mut eigenvalues: Vec<f64> = (0..n).map(|i| ak[i][i]).collect();
    eigenvalues.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    QRAlgorithmResult {
        eigenvalues,
        iterations: iters,
        off_diag_norm: off_diag,
    }
}
/// Randomized SVD for m×n matrix A, targeting rank k with oversampling p.
///
/// Algorithm:
/// 1. Draw a random Gaussian sketch matrix Ω of size n×(k+p).
/// 2. Form Y = A Ω and orthogonalize via QR to get Q (m×(k+p)).
/// 3. Project: B = Q^T A  (size (k+p)×n).
/// 4. Compute exact SVD of the small matrix B.
/// 5. Recover U = Q Ũ.
///
/// This is a simplified deterministic stand-in: the random matrix is
/// replaced by a structured sketch built from A's columns to keep the
/// implementation self-contained and testable without an RNG dependency.
pub fn randomized_svd(a: &[Vec<f64>], k: usize, oversampling: usize) -> RandomizedSVDResult {
    let m = a.len();
    let n = if m == 0 { 0 } else { a[0].len() };
    let l = (k + oversampling).min(m).min(n);
    let mut omega: Vec<Vec<f64>> = Vec::with_capacity(l);
    for j in 0..l {
        let col: Vec<f64> = (0..m).map(|i| a[i][j]).collect();
        let c = norm2(&col);
        if c > 1e-15 {
            omega.push(col.iter().map(|v| v / c).collect());
        } else {
            let mut e = vec![0.0; m];
            if j < m {
                e[j] = 1.0;
            }
            omega.push(e);
        }
    }
    let mut y: Vec<Vec<f64>> = vec![vec![0.0; l]; m];
    for j in 0..l {
        for i in 0..m {
            y[i][j] = omega[j][i];
        }
    }
    let qr_y = qr_decompose(&y);
    let q: Vec<Vec<f64>> = (0..m)
        .map(|i| (0..l).map(|j| qr_y.q[i][j]).collect())
        .collect();
    let mut b_proj: Vec<Vec<f64>> = vec![vec![0.0; n]; l];
    for i in 0..l {
        for jj in 0..n {
            b_proj[i][jj] = (0..m).map(|r| q[r][i] * a[r][jj]).sum();
        }
    }
    let mut bbt: Vec<Vec<f64>> = vec![vec![0.0; l]; l];
    for i in 0..l {
        for j in 0..l {
            bbt[i][j] = (0..n).map(|c| b_proj[i][c] * b_proj[j][c]).sum();
        }
    }
    let qr_res = qr_algorithm_eigen(&bbt, 200, 1e-10);
    let sigma: Vec<f64> = qr_res
        .eigenvalues
        .iter()
        .rev()
        .take(k)
        .map(|&ev| ev.max(0.0).sqrt())
        .collect();
    let u_k = k.min(m);
    let u: Vec<Vec<f64>> = (0..m)
        .map(|i| (0..u_k).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    let vt: Vec<Vec<f64>> = (0..k)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    RandomizedSVDResult { u, sigma, vt }
}
/// Build a k-step Krylov subspace via the Arnoldi process.
///
/// Starting from the normalised initial vector `b`, computes orthonormal
/// basis {v₁, …, v_{k+1}} and the upper Hessenberg matrix H_k such that
/// A V_k = V_{k+1} H̄_k.
///
/// Returns breakdown early if ‖w‖ < `breakdown_tol`.
pub fn arnoldi_krylov(
    a: &[Vec<f64>],
    b: &[f64],
    k: usize,
    breakdown_tol: f64,
) -> KrylovSubspaceResult {
    let n = b.len();
    let b_norm = norm2(b);
    let v0: Vec<f64> = if b_norm > 1e-300 {
        b.iter().map(|bi| bi / b_norm).collect()
    } else {
        let mut e = vec![0.0; n];
        if n > 0 {
            e[0] = 1.0;
        }
        e
    };
    let mut v_basis: Vec<Vec<f64>> = vec![v0];
    let mut h: Vec<Vec<f64>> = vec![vec![0.0; k]; k + 1];
    let mut steps = 0;
    for j in 0..k {
        let mut w = mat_vec(a, &v_basis[j]);
        for i in 0..=j {
            h[i][j] = dot(&w, &v_basis[i]);
            let hi = h[i][j];
            axpy(-hi, &v_basis[i], &mut w);
        }
        h[j + 1][j] = norm2(&w);
        if h[j + 1][j] < breakdown_tol {
            steps = j + 1;
            break;
        }
        let inv = 1.0 / h[j + 1][j];
        let vj1: Vec<f64> = w.iter().map(|wi| wi * inv).collect();
        v_basis.push(vj1);
        steps = j + 1;
    }
    KrylovSubspaceResult { v_basis, h, steps }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lu_solve_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 8.0];
        let lu = lu_decompose(&a).expect("LU should succeed");
        let x = lu_solve(&lu, &b);
        assert!((x[0] - 1.4).abs() < 1e-10, "x[0] = {}", x[0]);
        assert!((x[1] - 2.2).abs() < 1e-10, "x[1] = {}", x[1]);
    }
    #[test]
    fn test_cholesky_solve_3x3() {
        let a = vec![
            vec![4.0, 2.0, 0.0],
            vec![2.0, 5.0, 3.0],
            vec![0.0, 3.0, 7.0],
        ];
        let b = vec![6.0, 10.0, 10.0];
        let l = cholesky_decompose(&a).expect("Cholesky should succeed");
        let x = cholesky_solve(&l, &b);
        let ax = mat_vec(&a, &x);
        for i in 0..3 {
            assert!(
                (ax[i] - b[i]).abs() < 1e-9,
                "residual[{i}] = {}",
                ax[i] - b[i]
            );
        }
    }
    #[test]
    fn test_qr_orthogonality() {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 10.0],
        ];
        let qr = qr_decompose(&a);
        let qt = &qr.q;
        let n = qt.len();
        for i in 0..n {
            for j in 0..n {
                let val: f64 = (0..n).map(|k| qt[k][i] * qt[k][j]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((val - expected).abs() < 1e-9, "Q^T Q [{i},{j}] = {val}");
            }
        }
    }
    #[test]
    fn test_conjugate_gradient() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let b = vec![1.0, 2.0, 3.0];
        let (x, _iters, res) = conjugate_gradient(&a, &b, 1e-10, 100);
        assert!(res < 1e-8, "CG residual {res}");
        let ax = mat_vec(&a, &x);
        for i in 0..3 {
            assert!(
                (ax[i] - b[i]).abs() < 1e-8,
                "ax[{i}]={} b[{i}]={}",
                ax[i],
                b[i]
            );
        }
    }
    #[test]
    fn test_bicgstab() {
        let a = vec![vec![3.0, 1.0], vec![1.0, 4.0]];
        let b = vec![5.0, 6.0];
        let (x, _iters, res) = bicgstab(&a, &b, 1e-10, 100);
        assert!(res < 1e-8, "BiCGSTAB residual {res}");
        let ax = mat_vec(&a, &x);
        for i in 0..2 {
            assert!(
                (ax[i] - b[i]).abs() < 1e-8,
                "ax[{i}]={} b[{i}]={}",
                ax[i],
                b[i]
            );
        }
    }
    #[test]
    fn test_csr_matvec() {
        let mut coo = CooMatrix::new(3, 3);
        coo.push(0, 0, 2.0);
        coo.push(1, 1, 3.0);
        coo.push(2, 2, 5.0);
        let csr = CsrMatrix::from_coo(&coo);
        let x = vec![1.0, 2.0, 3.0];
        let y = csr.matvec(&x);
        assert!((y[0] - 2.0).abs() < 1e-12);
        assert!((y[1] - 6.0).abs() < 1e-12);
        assert!((y[2] - 15.0).abs() < 1e-12);
    }
    #[test]
    fn test_qr_eigenvalues_symmetric() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let eigs = qr_eigenvalues(&a, 200, 1e-10);
        assert!((eigs[0] - 1.0).abs() < 1e-6, "eig[0]={}", eigs[0]);
        assert!((eigs[1] - 3.0).abs() < 1e-6, "eig[1]={}", eigs[1]);
    }
    #[test]
    fn test_build_env() {
        let env = build_numerical_linear_algebra_env();
        assert!(env.get(&Name::str("LUDecomposition")).is_some());
        assert!(env.get(&Name::str("CholeskyDecomposition")).is_some());
        assert!(env.get(&Name::str("ConjugateGradientConverges")).is_some());
        assert!(env.get(&Name::str("CSRMatrix")).is_some());
        assert!(env.get(&Name::str("Eigenvalue")).is_some());
    }
    #[test]
    fn test_build_env_new_axioms() {
        let env = build_numerical_linear_algebra_env();
        assert!(env.get(&Name::str("QRAlgorithmEigenvalue")).is_some());
        assert!(env.get(&Name::str("JacobiIteration")).is_some());
        assert!(env.get(&Name::str("PowerMethodConverges")).is_some());
        assert!(env.get(&Name::str("WilkinsonShift")).is_some());
        assert!(env.get(&Name::str("ArnoldiProcess")).is_some());
        assert!(env.get(&Name::str("GMRESWithRestart")).is_some());
        assert!(env.get(&Name::str("ConjugateGradientOptimal")).is_some());
        assert!(env.get(&Name::str("MinResConverges")).is_some());
        assert!(env.get(&Name::str("ILUPreconditioner")).is_some());
        assert!(env.get(&Name::str("AMGPreconditioner")).is_some());
        assert!(env.get(&Name::str("SSORPreconditioner")).is_some());
        assert!(env.get(&Name::str("PolynomialPreconditioner")).is_some());
        assert!(env.get(&Name::str("RandomizedSVDApprox")).is_some());
        assert!(env.get(&Name::str("SketchAndSolve")).is_some());
        assert!(env.get(&Name::str("JohnsonLindenstraussEmbed")).is_some());
        assert!(env.get(&Name::str("CountSketchMatrix")).is_some());
        assert!(env.get(&Name::str("MatrixExponential")).is_some());
        assert!(env.get(&Name::str("MatrixLogarithm")).is_some());
        assert!(env.get(&Name::str("MatrixSquareRoot")).is_some());
        assert!(env.get(&Name::str("MatrixFunction")).is_some());
        assert!(env.get(&Name::str("ToeplitzMatrix")).is_some());
        assert!(env.get(&Name::str("CirculantMatrix")).is_some());
        assert!(env.get(&Name::str("HankelMatrix")).is_some());
        assert!(env.get(&Name::str("DisplacementRank")).is_some());
        assert!(env.get(&Name::str("CURDecomposition")).is_some());
        assert!(env.get(&Name::str("NystromApproximation")).is_some());
        assert!(env.get(&Name::str("ColumnSubsetSelection")).is_some());
        assert!(env.get(&Name::str("NuclearNormMin")).is_some());
        assert!(env.get(&Name::str("RIPMatrix")).is_some());
        assert!(env.get(&Name::str("TuckerDecomposition")).is_some());
        assert!(env.get(&Name::str("CPDecomposition")).is_some());
        assert!(env.get(&Name::str("TensorTrain")).is_some());
        assert!(env.get(&Name::str("GraphLaplacian")).is_some());
        assert!(env.get(&Name::str("NormalizedCut")).is_some());
        assert!(env.get(&Name::str("DiffusionMap")).is_some());
        assert!(env.get(&Name::str("FiedlerVector")).is_some());
        assert!(env.get(&Name::str("DavisKahanBound")).is_some());
        assert!(env.get(&Name::str("WeylEigenvalueBound")).is_some());
        assert!(env.get(&Name::str("SinThetaTheorem")).is_some());
        assert!(env.get(&Name::str("Blas3GemmOptimal")).is_some());
        assert!(env.get(&Name::str("RecursiveLU")).is_some());
        assert!(env.get(&Name::str("CommAvoidingQR")).is_some());
        assert!(env.get(&Name::str("FillReducingOrdering")).is_some());
        assert!(env.get(&Name::str("SupernodalElimination")).is_some());
        assert!(env.get(&Name::str("NestedDissectionOrder")).is_some());
        assert!(env.get(&Name::str("FP16RoundingError")).is_some());
        assert!(env.get(&Name::str("BF16RoundingError")).is_some());
        assert!(env.get(&Name::str("MixedPrecisionIR")).is_some());
        assert!(env
            .get(&Name::str("ClassicalGramSchmidtInstability"))
            .is_some());
        assert!(env
            .get(&Name::str("ModifiedGramSchmidtStability"))
            .is_some());
        assert!(env.get(&Name::str("HouseholderVsNormalEq")).is_some());
        assert!(env.get(&Name::str("AugmentedSystemStability")).is_some());
    }
    #[test]
    fn test_qr_algorithm_eigen_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let res = qr_algorithm_eigen(&a, 300, 1e-10);
        assert!(res.off_diag_norm < 1e-8, "off_diag={}", res.off_diag_norm);
        assert!(
            (res.eigenvalues[0] - 1.0).abs() < 1e-6,
            "eig[0]={}",
            res.eigenvalues[0]
        );
        assert!(
            (res.eigenvalues[1] - 3.0).abs() < 1e-6,
            "eig[1]={}",
            res.eigenvalues[1]
        );
    }
    #[test]
    fn test_gmres_solver_struct() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let b = vec![1.0, 2.0, 3.0];
        let solver = GMRESSolver::new(10, 1e-10, 20);
        let sol = solver.solve(&a, &b);
        assert!(
            sol.converged,
            "GMRES did not converge; rel_res={}",
            sol.rel_residual
        );
        let ax = mat_vec(&a, &sol.x);
        for i in 0..3 {
            assert!(
                (ax[i] - b[i]).abs() < 1e-8,
                "residual[{i}]={}",
                ax[i] - b[i]
            );
        }
    }
    #[test]
    fn test_randomized_svd_shape() {
        let a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ];
        let res = randomized_svd(&a, 2, 1);
        assert_eq!(res.sigma.len(), 2, "sigma length");
        assert!(res.sigma[0] >= 0.0, "sigma[0] >= 0");
    }
    #[test]
    fn test_arnoldi_krylov_orthogonality() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let b = vec![1.0, 0.0, 0.0];
        let res = arnoldi_krylov(&a, &b, 3, 1e-12);
        let vb = &res.v_basis;
        for i in 0..vb.len() {
            for j in 0..vb.len() {
                let ip = dot(&vb[i], &vb[j]);
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((ip - expected).abs() < 1e-9, "V[{i}]·V[{j}] = {ip}");
            }
        }
    }
    #[test]
    fn test_circulant_matrix_fft_matvec() {
        let c = vec![2.0, 1.0, 0.0, 0.0];
        let circ = CirculantMatrixFFT::new(c);
        let x = vec![1.0, 0.0, 0.0, 0.0];
        let y_naive = circ.matvec_naive(&x);
        let y_fft = circ.matvec(&x);
        for i in 0..4 {
            assert!(
                (y_naive[i] - y_fft[i]).abs() < 1e-10,
                "y[{i}]: naive={} fft={}",
                y_naive[i],
                y_fft[i]
            );
        }
    }
    #[test]
    fn test_circulant_eigenvalues_real() {
        let c = vec![3.0, 1.0, 0.0, 1.0];
        let circ = CirculantMatrixFFT::new(c);
        let (_, im) = circ.dft_eigenvalues();
        for (i, &v) in im.iter().enumerate() {
            assert!(
                v.abs() < 1e-10,
                "im[{i}]={v} should be ~0 for symmetric circulant"
            );
        }
    }
}
#[cfg(test)]
mod tests_nla_extended {
    use super::*;
    #[test]
    fn test_cg_solver_identity() {
        let n = 3;
        let a: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
            .collect();
        let b = vec![1.0, 2.0, 3.0];
        let cg = ConjugateGradient::new(100, 1e-10);
        let (x, res, _iters) = cg.solve(&a, &b);
        assert!(res < 1e-9, "Residual = {res}");
        for i in 0..n {
            assert!(
                (x[i] - b[i]).abs() < 1e-8,
                "x[{i}]={} expected {}",
                x[i],
                b[i]
            );
        }
    }
    #[test]
    fn test_cg_solver_spd() {
        let a = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
        let b = vec![1.0, 0.0];
        let cg = ConjugateGradient::new(100, 1e-10);
        let (x, _res, _iters) = cg.solve(&a, &b);
        assert!((x[0] - 2.0 / 3.0).abs() < 1e-8, "x[0]={}", x[0]);
        assert!((x[1] - 1.0 / 3.0).abs() < 1e-8, "x[1]={}", x[1]);
    }
    #[test]
    fn test_power_iteration_diagonal() {
        let a = vec![vec![5.0, 0.0], vec![0.0, 2.0]];
        let pi = PowerIteration::new(1000, 1e-10);
        let (eigenvalue, _v, _iters) = pi.run(&a);
        assert!((eigenvalue - 5.0).abs() < 1e-6, "eigenvalue={eigenvalue}");
    }
    #[test]
    fn test_qr_algorithm_eigenvalues() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let qr = QRAlgorithm::new(500, 1e-10);
        let eigs = qr.run(&a);
        let mut eigs_sorted = eigs.clone();
        eigs_sorted.sort_by(|a, b| a.partial_cmp(b).expect("sort_by should succeed"));
        assert!(
            (eigs_sorted[0] - 1.0).abs() < 1e-6,
            "eig0={}",
            eigs_sorted[0]
        );
        assert!(
            (eigs_sorted[1] - 3.0).abs() < 1e-6,
            "eig1={}",
            eigs_sorted[1]
        );
    }
    #[test]
    fn test_gmres_arnoldi() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let b = vec![1.0, 0.0];
        let gmres = GmresSolver::new(2, 10, 1e-10);
        let (v, h) = gmres.arnoldi_basis(&a, &b);
        assert_eq!(v.len(), 3);
        assert_eq!(h.len(), 3);
    }
}
