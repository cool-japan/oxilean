//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

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
#[allow(dead_code)]
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
#[allow(dead_code)]
pub fn app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    app(app4(f, a, b, c, d), e)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
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
/// Vec n α = Fin n → α  (a vector of n elements of type α)
pub fn vec_type() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "α", type0(), type0()),
    )
}
/// vec_zero : ∀ (n : Nat) (α : Type) [Zero α], Vec n α
/// The zero vector
pub fn vec_zero_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app2(cst("Zero"), cst("α"), cst("α")),
                app2(cst("Vec"), cst("n"), cst("α")),
            ),
        ),
    )
}
/// vec_add : ∀ (n : Nat) (α : Type) [Add α], Vec n α → Vec n α → Vec n α
pub fn vec_add_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("Add"), cst("α")),
                arrow(
                    app2(cst("Vec"), bvar(1), bvar(1)),
                    arrow(
                        app2(cst("Vec"), bvar(2), bvar(2)),
                        app2(cst("Vec"), bvar(3), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// vec_smul : ∀ (n : Nat) (α : Type) [Mul α], α → Vec n α → Vec n α
pub fn vec_smul_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("Mul"), cst("α")),
                arrow(
                    cst("α"),
                    arrow(
                        app2(cst("Vec"), bvar(2), bvar(2)),
                        app2(cst("Vec"), bvar(3), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// vec_dot : ∀ (n : Nat) (α : Type) [Add α] [Mul α] [Zero α], Vec n α → Vec n α → α
pub fn vec_dot_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app2(cst("Vec"), bvar(0), bvar(0)),
                arrow(app2(cst("Vec"), bvar(1), bvar(1)), bvar(1)),
            ),
        ),
    )
}
/// vec_norm_sq : ∀ (n : Nat) (α : Type) [Add α] [Mul α] [Zero α], Vec n α → α
pub fn vec_norm_sq_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(app2(cst("Vec"), bvar(0), bvar(0)), bvar(0)),
        ),
    )
}
/// Matrix : Nat → Nat → Type → Type
pub fn matrix_type() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            pi(BinderInfo::Default, "α", type0(), type0()),
        ),
    )
}
/// matrix_zero : ∀ (m n : Nat) (α : Type) [Zero α], Matrix m n α
pub fn matrix_zero_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "α",
                type0(),
                arrow(
                    app(cst("Zero"), bvar(0)),
                    app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// matrix_add : ∀ (m n : Nat) (α : Type) [Add α], Matrix m n α → Matrix m n α → Matrix m n α
pub fn matrix_add_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "α",
                type0(),
                arrow(
                    app(cst("Add"), bvar(0)),
                    arrow(
                        app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)),
                        arrow(
                            app3(cst("Matrix"), bvar(3), bvar(2), bvar(1)),
                            app3(cst("Matrix"), bvar(4), bvar(3), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// matrix_mul : ∀ (m n k : Nat) (α : Type) [Add α] [Mul α] [Zero α],
///              Matrix m n α → Matrix n k α → Matrix m k α
pub fn matrix_mul_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "k",
                nat_ty(),
                impl_pi(
                    "α",
                    type0(),
                    arrow(
                        app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)),
                        arrow(
                            app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)),
                            app3(cst("Matrix"), bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// matrix_transpose : ∀ (m n : Nat) (α : Type), Matrix m n α → Matrix n m α
pub fn matrix_transpose_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "α",
                type0(),
                arrow(
                    app3(cst("Matrix"), bvar(1), bvar(0), bvar(0)),
                    app3(cst("Matrix"), bvar(1), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// matrix_identity : ∀ (n : Nat) (α : Type) [Zero α] [One α], Matrix n n α
pub fn matrix_identity_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("Zero"), bvar(0)),
                arrow(
                    app(cst("One"), bvar(1)),
                    app3(cst("Matrix"), bvar(2), bvar(2), bvar(2)),
                ),
            ),
        ),
    )
}
/// matrix_det : ∀ (n : Nat) (α : Type) [CommRing α], Matrix n n α → α
pub fn matrix_det_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                arrow(app3(cst("Matrix"), bvar(1), bvar(1), bvar(0)), bvar(0)),
            ),
        ),
    )
}
/// matrix_trace : ∀ (n : Nat) (α : Type) [Add α] [Zero α], Matrix n n α → α
pub fn matrix_trace_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(app3(cst("Matrix"), bvar(0), bvar(0), bvar(0)), bvar(0)),
        ),
    )
}
/// matrix_rank : ∀ (m n : Nat) (α : Type) [Field α], Matrix m n α → Nat
pub fn matrix_rank_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "α",
                type0(),
                arrow(
                    app(cst("Field"), bvar(0)),
                    arrow(app3(cst("Matrix"), bvar(2), bvar(1), bvar(0)), nat_ty()),
                ),
            ),
        ),
    )
}
/// LinearMap R M N = (M → N) satisfying linearity
/// LinearMap.mk : ∀ (R M N : Type) [Module R M] [Module R N],
///               (M → N) → (∀ (r : R) (m : M), f (r • m) = r • f m) →
///               (∀ (m₁ m₂ : M), f (m₁ + m₂) = f m₁ + f m₂) → LinearMap R M N
pub fn linear_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(BinderInfo::Default, "N", type0(), type0()),
        ),
    )
}
/// linear_map_apply : ∀ (R M N : Type), LinearMap R M N → M → N
pub fn linear_map_apply_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                arrow(
                    app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)),
                    arrow(bvar(1), bvar(1)),
                ),
            ),
        ),
    )
}
/// linear_map_comp : ∀ (R M N P : Type), LinearMap R M N → LinearMap R N P → LinearMap R M P
pub fn linear_map_comp_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                impl_pi(
                    "P",
                    type0(),
                    arrow(
                        app3(cst("LinearMap"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("LinearMap"), bvar(3), bvar(1), bvar(0)),
                            app3(cst("LinearMap"), bvar(3), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Kernel of a linear map: Submodule M
/// linear_map_ker : ∀ (R M N : Type) [Module R M] [Module R N], LinearMap R M N → Submodule R M
pub fn linear_map_ker_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                arrow(
                    app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)),
                    app2(cst("Submodule"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// Image of a linear map: Submodule N
/// linear_map_range : ∀ (R M N : Type) [Module R M] [Module R N], LinearMap R M N → Submodule R N
pub fn linear_map_range_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                arrow(
                    app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)),
                    app2(cst("Submodule"), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// inner : ∀ (E : Type) [InnerProductSpace ℝ E], E → E → ℝ
pub fn inner_product_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(
            app2(cst("InnerProductSpace"), cst("Real"), bvar(0)),
            arrow(bvar(0), arrow(bvar(1), cst("Real"))),
        ),
    )
}
/// norm : ∀ (E : Type) [NormedAddCommGroup E], E → ℝ
pub fn norm_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(
            app(cst("NormedAddCommGroup"), bvar(0)),
            arrow(bvar(0), cst("Real")),
        ),
    )
}
/// dist : ∀ (E : Type) [MetricSpace E], E → E → ℝ
pub fn dist_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(
            app(cst("MetricSpace"), bvar(0)),
            arrow(bvar(0), arrow(bvar(1), cst("Real"))),
        ),
    )
}
/// IsEigenvalue : ∀ (n : Nat) (α : Type) [CommRing α], Matrix n n α → α → Prop
/// f(v) = λ·v for some nonzero v
pub fn is_eigenvalue_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                arrow(
                    app3(cst("Matrix"), bvar(1), bvar(1), bvar(0)),
                    arrow(bvar(0), prop()),
                ),
            ),
        ),
    )
}
/// IsEigenvector : ∀ (n : Nat) (α : Type) [CommRing α], Matrix n n α → α → Vec n α → Prop
pub fn is_eigenvector_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                arrow(
                    app3(cst("Matrix"), bvar(1), bvar(1), bvar(0)),
                    arrow(bvar(0), arrow(app2(cst("Vec"), bvar(2), bvar(1)), prop())),
                ),
            ),
        ),
    )
}
/// CharacteristicPolynomial : ∀ (n : Nat) (α : Type) [CommRing α],
///   Matrix n n α → Polynomial α
pub fn characteristic_polynomial_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                arrow(
                    app3(cst("Matrix"), bvar(1), bvar(1), bvar(0)),
                    app(cst("Polynomial"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Rank-Nullity theorem:
/// rank(f) + nullity(f) = dim(M) for finite-dimensional M
pub fn rank_nullity_thm_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                pi(
                    BinderInfo::Default,
                    "f",
                    app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)),
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app2(
                            cst("Nat.add"),
                            app(cst("LinearMap.rank"), bvar(0)),
                            app(cst("LinearMap.nullity"), bvar(1)),
                        ),
                        app2(cst("FiniteDimensional.finrank"), bvar(4), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// Cayley-Hamilton: A satisfies its own characteristic polynomial
/// ∀ (n : Nat) (α : Type) [CommRing α] (A : Matrix n n α),
///   charPoly A evaluated at A = 0
pub fn cayley_hamilton_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "A",
                    app3(cst("Matrix"), bvar(2), bvar(2), bvar(1)),
                    app2(
                        app(cst("Eq"), app3(cst("Matrix"), bvar(3), bvar(3), bvar(2))),
                        app2(
                            cst("Polynomial.matrixEval"),
                            bvar(0),
                            app2(cst("characteristic_polynomial"), bvar(3), bvar(0)),
                        ),
                        app4(cst("matrix_zero"), bvar(3), bvar(3), bvar(2), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Spectral theorem: symmetric real matrices are diagonalizable
/// ∀ (n : Nat) (A : Matrix n n Real), IsSymmetric A → IsDiagonalizable ℝ A
pub fn spectral_theorem_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "A",
            app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
            arrow(
                app(cst("Matrix.IsSymmetric"), bvar(0)),
                app2(cst("Matrix.IsDiagonalizable"), cst("Real"), bvar(1)),
            ),
        ),
    )
}
/// Matrix.mul_det_comm: det(A * B) = det A * det B
pub fn det_mul_thm_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "A",
                    app3(cst("Matrix"), bvar(2), bvar(2), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "B",
                        app3(cst("Matrix"), bvar(3), bvar(3), bvar(2)),
                        app2(
                            app(cst("Eq"), bvar(2)),
                            app2(
                                cst("matrix_det"),
                                bvar(4),
                                app2(cst("matrix_mul"), bvar(1), bvar(0)),
                            ),
                            app2(
                                cst("HMul.hMul"),
                                app(cst("matrix_det"), bvar(1)),
                                app(cst("matrix_det"), bvar(0)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Populate an `Environment` with linear algebra definitions.
#[allow(dead_code)]
pub fn build_linear_algebra_env(env: &mut Environment) -> Result<(), String> {
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Vec"),
        univ_params: vec![],
        ty: vec_type(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Matrix"),
        univ_params: vec![],
        ty: matrix_type(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LinearMap"),
        univ_params: vec![],
        ty: linear_map_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Submodule"),
        univ_params: vec![Name::str("u")],
        ty: pi(
            BinderInfo::Default,
            "R",
            type0(),
            pi(BinderInfo::Default, "M", type0(), type0()),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("vec_zero"),
        univ_params: vec![],
        ty: vec_zero_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("vec_add"),
        univ_params: vec![],
        ty: vec_add_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("vec_smul"),
        univ_params: vec![],
        ty: vec_smul_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("vec_dot"),
        univ_params: vec![],
        ty: vec_dot_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("vec_norm_sq"),
        univ_params: vec![],
        ty: vec_norm_sq_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_zero"),
        univ_params: vec![],
        ty: matrix_zero_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_add"),
        univ_params: vec![],
        ty: matrix_add_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_mul"),
        univ_params: vec![],
        ty: matrix_mul_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_transpose"),
        univ_params: vec![],
        ty: matrix_transpose_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_identity"),
        univ_params: vec![],
        ty: matrix_identity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_det"),
        univ_params: vec![],
        ty: matrix_det_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_trace"),
        univ_params: vec![],
        ty: matrix_trace_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("matrix_rank"),
        univ_params: vec![],
        ty: matrix_rank_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("linear_map_apply"),
        univ_params: vec![],
        ty: linear_map_apply_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("linear_map_comp"),
        univ_params: vec![],
        ty: linear_map_comp_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("linear_map_ker"),
        univ_params: vec![],
        ty: linear_map_ker_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("linear_map_range"),
        univ_params: vec![],
        ty: linear_map_range_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsEigenvalue"),
        univ_params: vec![],
        ty: is_eigenvalue_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsEigenvector"),
        univ_params: vec![],
        ty: is_eigenvector_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("characteristic_polynomial"),
        univ_params: vec![],
        ty: characteristic_polynomial_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("inner"),
        univ_params: vec![],
        ty: inner_product_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("norm"),
        univ_params: vec![],
        ty: norm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("dist"),
        univ_params: vec![],
        ty: dist_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("rank_nullity"),
        univ_params: vec![],
        ty: rank_nullity_thm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("cayley_hamilton"),
        univ_params: vec![],
        ty: cayley_hamilton_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("spectral_theorem"),
        univ_params: vec![],
        ty: spectral_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("det_mul"),
        univ_params: vec![],
        ty: det_mul_thm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LinearMap.rank"),
        univ_params: vec![],
        ty: impl_pi(
            "R",
            type0(),
            impl_pi(
                "M",
                type0(),
                impl_pi(
                    "N",
                    type0(),
                    arrow(app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)), nat_ty()),
                ),
            ),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LinearMap.nullity"),
        univ_params: vec![],
        ty: impl_pi(
            "R",
            type0(),
            impl_pi(
                "M",
                type0(),
                impl_pi(
                    "N",
                    type0(),
                    arrow(app3(cst("LinearMap"), bvar(2), bvar(1), bvar(0)), nat_ty()),
                ),
            ),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FiniteDimensional.finrank"),
        univ_params: vec![],
        ty: impl_pi("R", type0(), arrow(type0(), nat_ty())),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Matrix.IsSymmetric"),
        univ_params: vec![],
        ty: impl_pi(
            "n",
            nat_ty(),
            impl_pi(
                "α",
                type0(),
                arrow(app3(cst("Matrix"), bvar(1), bvar(1), bvar(0)), prop()),
            ),
        ),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Matrix.IsDiagonalizable"),
        univ_params: vec![],
        ty: impl_pi(
            "α",
            type0(),
            impl_pi(
                "n",
                nat_ty(),
                arrow(app3(cst("Matrix"), bvar(0), bvar(0), bvar(1)), prop()),
            ),
        ),
    });
    Ok(())
}
/// TensorProduct R M N : Type  (the tensor product of modules)
pub fn tensor_product_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(BinderInfo::Default, "N", type0(), type0()),
        ),
    )
}
/// tensor_mk : ∀ (R M N : Type), M → N → TensorProduct R M N
pub fn tensor_mk_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                arrow(
                    bvar(1),
                    arrow(
                        bvar(1),
                        app3(cst("TensorProduct"), bvar(2), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// tensor_universal : ∀ (R M N P : Type) [Module R M] [Module R N] [Module R P],
///   BilinearMap R M N P → LinearMap R (TensorProduct R M N) P
pub fn tensor_universal_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                impl_pi(
                    "P",
                    type0(),
                    arrow(
                        app4(cst("BilinearMap"), bvar(3), bvar(2), bvar(1), bvar(0)),
                        app3(
                            cst("LinearMap"),
                            bvar(3),
                            app3(cst("TensorProduct"), bvar(3), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// tensor_naturality : ∀ (R M M' N N' : Type),
///   LinearMap R M M' → LinearMap R N N' →
///   LinearMap R (TensorProduct R M N) (TensorProduct R M' N')
pub fn tensor_naturality_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "M2",
                type0(),
                impl_pi(
                    "N",
                    type0(),
                    impl_pi(
                        "N2",
                        type0(),
                        arrow(
                            app3(cst("LinearMap"), bvar(4), bvar(3), bvar(2)),
                            arrow(
                                app3(cst("LinearMap"), bvar(4), bvar(1), bvar(0)),
                                app3(
                                    cst("LinearMap"),
                                    bvar(4),
                                    app3(cst("TensorProduct"), bvar(4), bvar(3), bvar(1)),
                                    app3(cst("TensorProduct"), bvar(4), bvar(2), bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// tensor_assoc : ∀ (R M N P : Type),
///   LinearEquiv R (TensorProduct R (TensorProduct R M N) P)
///               (TensorProduct R M (TensorProduct R N P))
pub fn tensor_assoc_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            impl_pi(
                "N",
                type0(),
                impl_pi(
                    "P",
                    type0(),
                    app3(
                        cst("LinearEquiv"),
                        bvar(3),
                        app3(
                            cst("TensorProduct"),
                            bvar(3),
                            app3(cst("TensorProduct"), bvar(3), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app3(
                            cst("TensorProduct"),
                            bvar(3),
                            bvar(2),
                            app3(cst("TensorProduct"), bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// ExteriorAlgebra R M : Type  (the exterior algebra of module M over R)
pub fn exterior_algebra_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(BinderInfo::Default, "M", type0(), type0()),
    )
}
/// exterior_wedge : ∀ (R M : Type) (k : Nat),
///   AlternatingMap R M (ExteriorAlgebra R M) k
pub fn exterior_wedge_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                app4(
                    cst("AlternatingMap"),
                    bvar(2),
                    bvar(1),
                    app2(cst("ExteriorAlgebra"), bvar(2), bvar(1)),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// exterior_graded : ∀ (R M : Type), GradedAlgebra R (ExteriorAlgebra R M)
pub fn exterior_graded_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            app2(
                cst("GradedAlgebra"),
                bvar(1),
                app2(cst("ExteriorAlgebra"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// exterior_anticomm : ∀ (R M : Type) (u v : M),
///   wedge u v = - wedge v u  (antisymmetry)
pub fn exterior_anticomm_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            pi(
                BinderInfo::Default,
                "u",
                bvar(0),
                pi(
                    BinderInfo::Default,
                    "v",
                    bvar(1),
                    app2(
                        app(cst("Eq"), app2(cst("ExteriorAlgebra"), bvar(3), bvar(2))),
                        app2(cst("exterior_wedge"), bvar(1), bvar(0)),
                        app(cst("Neg"), app2(cst("exterior_wedge"), bvar(0), bvar(1))),
                    ),
                ),
            ),
        ),
    )
}
/// symmetric_power : ∀ (R M : Type) (k : Nat), Type
pub fn symmetric_power_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(BinderInfo::Default, "k", nat_ty(), type0()),
        ),
    )
}
/// exterior_power : ∀ (R M : Type) (k : Nat), Type
pub fn exterior_power_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(
            BinderInfo::Default,
            "M",
            type0(),
            pi(BinderInfo::Default, "k", nat_ty(), type0()),
        ),
    )
}
/// FreeModule R (ι : Type) : Type  (free module with basis ι)
pub fn free_module_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        type0(),
        pi(BinderInfo::Default, "ι", type0(), type0()),
    )
}
/// free_module_basis : ∀ (R ι : Type), Basis ι R (FreeModule R ι)
pub fn free_module_basis_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "ι",
            type0(),
            app3(
                cst("Basis"),
                bvar(0),
                bvar(1),
                app2(cst("FreeModule"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// IsProjective : ∀ (R M : Type) [Module R M], Prop
pub fn is_projective_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            arrow(app2(cst("Module"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// IsInjective : ∀ (R M : Type) [Module R M], Prop
pub fn is_injective_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            arrow(app2(cst("Module"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// nakayama_lemma : ∀ (R M : Type) [CommRing R] [Module R M]
///   (I : Ideal R), IsMaximal I → (I • M = M) → M = 0
pub fn nakayama_lemma_ty() -> Expr {
    impl_pi(
        "R",
        type0(),
        impl_pi(
            "M",
            type0(),
            arrow(
                app(cst("Ideal"), bvar(1)),
                arrow(
                    app(cst("Ideal.IsMaximal"), bvar(0)),
                    arrow(
                        app2(
                            app(cst("Eq"), app2(cst("Submodule"), bvar(2), bvar(1))),
                            app2(
                                cst("Ideal.smul"),
                                bvar(0),
                                app2(cst("Submodule.top"), bvar(2), bvar(1)),
                            ),
                            app2(cst("Submodule.top"), bvar(2), bvar(1)),
                        ),
                        app2(
                            app(cst("Eq"), app2(cst("Submodule"), bvar(2), bvar(1))),
                            app2(cst("Submodule.top"), bvar(2), bvar(1)),
                            app2(cst("Submodule.bot"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// IsNormalOperator : ∀ (H : Type) [HilbertSpace H], (H → H) → Prop
///   T is normal if T* ∘ T = T ∘ T*
pub fn is_normal_operator_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            arrow(arrow(bvar(0), bvar(1)), prop()),
        ),
    )
}
/// spectral_decomposition : ∀ (H : Type) [HilbertSpace H] (T : H → H),
///   IsNormalOperator H T → SpectralMeasure H T
pub fn spectral_decomposition_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "T",
                arrow(bvar(0), bvar(1)),
                arrow(
                    app2(cst("IsNormalOperator"), bvar(2), bvar(0)),
                    app2(cst("SpectralMeasure"), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// functional_calculus : ∀ (H : Type) [HilbertSpace H] (T : H → H),
///   IsNormalOperator H T → (Real → Real) → (H → H)
pub fn functional_calculus_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "T",
                arrow(bvar(0), bvar(1)),
                arrow(
                    app2(cst("IsNormalOperator"), bvar(2), bvar(0)),
                    arrow(arrow(cst("Real"), cst("Real")), arrow(bvar(2), bvar(3))),
                ),
            ),
        ),
    )
}
/// spectrum : ∀ (E : Type) [BanachAlgebra E] (x : E), Set Real
pub fn spectrum_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(
            app(cst("BanachAlgebra"), bvar(0)),
            arrow(bvar(0), app(cst("Set"), cst("Real"))),
        ),
    )
}
/// spectral_radius : ∀ (E : Type) [BanachAlgebra E] (x : E), Real
pub fn spectral_radius_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(
            app(cst("BanachAlgebra"), bvar(0)),
            arrow(bvar(0), cst("Real")),
        ),
    )
}
/// CStarAlgebra : Type → Prop  (predicate on Banach *-algebras)
pub fn cstar_algebra_pred_ty() -> Expr {
    arrow(type0(), prop())
}
/// VonNeumannAlgebra : Type → Prop
pub fn von_neumann_algebra_pred_ty() -> Expr {
    arrow(type0(), prop())
}
/// double_commutant : ∀ (A H : Type) [VonNeumannAlgebra H],
///   Subalgebra A H → Subalgebra A H
pub fn double_commutant_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "H",
            type0(),
            arrow(
                app(cst("VonNeumannAlgebra"), bvar(0)),
                arrow(
                    app2(cst("Subalgebra"), bvar(1), bvar(1)),
                    app2(cst("Subalgebra"), bvar(1), bvar(1)),
                ),
            ),
        ),
    )
}
/// IsComplete : ∀ (E : Type) [MetricSpace E], Prop
pub fn is_complete_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        arrow(app(cst("MetricSpace"), bvar(0)), prop()),
    )
}
/// BanachSpace : Type → Prop
pub fn banach_space_pred_ty() -> Expr {
    arrow(type0(), prop())
}
/// HilbertSpace : Type → Prop  (complete inner product space)
pub fn hilbert_space_pred_ty() -> Expr {
    arrow(type0(), prop())
}
/// open_mapping : ∀ (E F : Type) [BanachSpace E] [BanachSpace F]
///   (T : LinearMap Real E F), IsSurjective T → IsOpenMap T
pub fn open_mapping_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "F",
            type0(),
            arrow(
                app(cst("BanachSpace"), bvar(1)),
                arrow(
                    app(cst("BanachSpace"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "T",
                        app3(cst("LinearMap"), cst("Real"), bvar(2), bvar(1)),
                        arrow(
                            app(cst("IsSurjective"), bvar(0)),
                            app(cst("IsOpenMap"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// closed_graph : ∀ (E F : Type) [BanachSpace E] [BanachSpace F]
///   (T : LinearMap Real E F), IsClosedGraph T → IsContinuous T
pub fn closed_graph_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "F",
            type0(),
            arrow(
                app(cst("BanachSpace"), bvar(1)),
                arrow(
                    app(cst("BanachSpace"), bvar(1)),
                    pi(
                        BinderInfo::Default,
                        "T",
                        app3(cst("LinearMap"), cst("Real"), bvar(2), bvar(1)),
                        arrow(
                            app(cst("IsClosedGraph"), bvar(0)),
                            app(cst("IsContinuous"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// riesz_representation : ∀ (H : Type) [HilbertSpace H]
///   (φ : H →L Real), ∃ v : H, φ = ⟨v, ·⟩
pub fn riesz_representation_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "φ",
                app3(
                    cst("ContinuousLinearMap"),
                    cst("Real"),
                    bvar(1),
                    cst("Real"),
                ),
                app(
                    app(cst("Exists"), bvar(1)),
                    app2(cst("IsInnerProductRep"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// ConditionNumber : ∀ (n : Nat) (A : Matrix n n Real), Real
pub fn condition_number_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(
            app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
            cst("Real"),
        ),
    )
}
/// IsBackwardStable : ∀ (n : Nat) (alg : Matrix n n Real → Matrix n n Real), Prop
pub fn is_backward_stable_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(
            arrow(
                app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
                app3(cst("Matrix"), bvar(1), bvar(1), cst("Real")),
            ),
            prop(),
        ),
    )
}
/// SingularValues : ∀ (m n : Nat) (A : Matrix m n Real), Vec (min m n) Real
pub fn singular_values_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            arrow(
                app3(cst("Matrix"), bvar(1), bvar(0), cst("Real")),
                app2(
                    cst("Vec"),
                    app2(cst("Nat.min"), bvar(1), bvar(0)),
                    cst("Real"),
                ),
            ),
        ),
    )
}
/// svd_decomp : ∀ (m n : Nat) (A : Matrix m n Real),
///   Σ (U : Matrix m m Real) (Σmat : Matrix m n Real) (V : Matrix n n Real),
///   A = U * Σmat * Vᵀ ∧ IsOrthogonal U ∧ IsOrthogonal V
pub fn svd_decomp_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "A",
                app3(cst("Matrix"), bvar(1), bvar(0), cst("Real")),
                app(cst("SVDDecomposition"), bvar(0)),
            ),
        ),
    )
}
/// FiniteField : ∀ (p : Nat), Type   (GF(p) the prime field)
pub fn finite_field_ty() -> Expr {
    pi(BinderInfo::Default, "p", nat_ty(), type0())
}
/// finite_field_order : ∀ (p : Nat), Fintype.card (FiniteField p) = p
pub fn finite_field_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        nat_ty(),
        app2(
            app(cst("Eq"), nat_ty()),
            app(cst("Fintype.card"), app(cst("FiniteField"), bvar(0))),
            bvar(0),
        ),
    )
}
/// vector_space_fin_field_dim : ∀ (p : Nat) (V : Type) [VectorSpace (FiniteField p) V] (n : Nat),
///   FiniteDimensional.finrank (FiniteField p) V = n →
///   Fintype.card V = p ^ n
pub fn vector_space_fin_field_dim_ty() -> Expr {
    impl_pi(
        "p",
        nat_ty(),
        impl_pi(
            "V",
            type0(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                arrow(
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app2(
                            cst("FiniteDimensional.finrank"),
                            app(cst("FiniteField"), bvar(2)),
                            bvar(1),
                        ),
                        bvar(0),
                    ),
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app(cst("Fintype.card"), bvar(1)),
                        app2(cst("Nat.pow"), bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// IsToeplitz : ∀ (n : Nat), Matrix n n Real → Prop
pub fn is_toeplitz_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")), prop()),
    )
}
/// IsCirculant : ∀ (n : Nat), Matrix n n Real → Prop
pub fn is_circulant_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")), prop()),
    )
}
/// IsCauchy : ∀ (n : Nat), Matrix n n Real → Prop
pub fn is_cauchy_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")), prop()),
    )
}
/// circulant_diagonalizable : ∀ (n : Nat) (A : Matrix n n Real),
///   IsCirculant n A → IsDiagonalizable Real A  (by DFT)
pub fn circulant_diagonalizable_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "A",
            app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
            arrow(
                app2(cst("IsCirculant"), bvar(1), bvar(0)),
                app2(cst("Matrix.IsDiagonalizable"), cst("Real"), bvar(1)),
            ),
        ),
    )
}
/// StiefelManifold : Nat → Nat → Type   (n × k matrices with orthonormal columns, k ≤ n)
pub fn stiefel_manifold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "k", nat_ty(), type0()),
    )
}
/// GrassmannManifold : Nat → Nat → Type   (k-planes in ℝ^n)
pub fn grassmann_manifold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "k", nat_ty(), type0()),
    )
}
/// OrthogonalGroup : Nat → Type   (O(n))
pub fn orthogonal_group_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), type0())
}
/// grassmann_is_homogeneous : ∀ (n k : Nat),
///   IsHomogeneousSpace (OrthogonalGroup n) (GrassmannManifold n k)
pub fn grassmann_is_homogeneous_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            app2(
                cst("IsHomogeneousSpace"),
                app(cst("OrthogonalGroup"), bvar(1)),
                app2(cst("GrassmannManifold"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// FredholdOperator : ∀ (E F : Type) [BanachSpace E] [BanachSpace F], (E → F) → Prop
pub fn fredholm_operator_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "F",
            type0(),
            arrow(
                app(cst("BanachSpace"), bvar(1)),
                arrow(
                    app(cst("BanachSpace"), bvar(1)),
                    arrow(arrow(bvar(1), bvar(1)), prop()),
                ),
            ),
        ),
    )
}
/// fredholm_index : ∀ (E F : Type) (T : E → F), FredholmOperator E F T → Int
pub fn fredholm_index_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "F",
            type0(),
            pi(
                BinderInfo::Default,
                "T",
                arrow(bvar(1), bvar(1)),
                arrow(
                    app3(cst("FredholmOperator"), bvar(2), bvar(2), bvar(0)),
                    cst("Int"),
                ),
            ),
        ),
    )
}
/// TraceClass : ∀ (H : Type) [HilbertSpace H], (H → H) → Prop
pub fn trace_class_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            arrow(arrow(bvar(0), bvar(1)), prop()),
        ),
    )
}
/// operator_trace : ∀ (H : Type) [HilbertSpace H] (T : H → H),
///   TraceClass H T → Real
pub fn operator_trace_ty() -> Expr {
    impl_pi(
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            pi(
                BinderInfo::Default,
                "T",
                arrow(bvar(0), bvar(1)),
                arrow(app2(cst("TraceClass"), bvar(2), bvar(0)), cst("Real")),
            ),
        ),
    )
}
/// linear_recurrence_solution : ∀ (d : Nat) (c : Vec d Real) (a₀ : Vec d Real) (n : Nat),
///   Real   (explicit formula via companion matrix)
pub fn linear_recurrence_solution_ty() -> Expr {
    impl_pi(
        "d",
        nat_ty(),
        arrow(
            app2(cst("Vec"), bvar(0), cst("Real")),
            arrow(
                app2(cst("Vec"), bvar(1), cst("Real")),
                arrow(nat_ty(), cst("Real")),
            ),
        ),
    )
}
/// companion_matrix : ∀ (d : Nat) (α : Type) [CommRing α],
///   Vec d α → Matrix d d α   (companion matrix of monic polynomial)
pub fn companion_matrix_ty() -> Expr {
    impl_pi(
        "d",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                arrow(
                    app2(cst("Vec"), bvar(1), bvar(0)),
                    app3(cst("Matrix"), bvar(2), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// cayley_hamilton_minimal_poly : ∀ (n : Nat) (α : Type) [CommRing α] (A : Matrix n n α),
///   minimal_poly A ∣ char_poly A
pub fn cayley_hamilton_minimal_poly_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("CommRing"), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "A",
                    app3(cst("Matrix"), bvar(2), bvar(2), bvar(1)),
                    app2(
                        cst("Polynomial.Dvd"),
                        app(cst("minimal_poly"), bvar(0)),
                        app(cst("char_poly"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// sylvester_eq_solution : ∀ (m n : Nat) (A : Matrix m m Real) (B : Matrix n n Real) (C : Matrix m n Real),
///   ¬ (spectra A ∩ spectra B ≠ ∅) → ∃! X : Matrix m n Real, A * X - X * B = C
pub fn sylvester_eq_solution_ty() -> Expr {
    impl_pi(
        "m",
        nat_ty(),
        impl_pi(
            "n",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "A",
                app3(cst("Matrix"), bvar(1), bvar(1), cst("Real")),
                pi(
                    BinderInfo::Default,
                    "B",
                    app3(cst("Matrix"), bvar(1), bvar(1), cst("Real")),
                    pi(
                        BinderInfo::Default,
                        "C",
                        app3(cst("Matrix"), bvar(3), bvar(2), cst("Real")),
                        arrow(
                            app(
                                cst("SpectraDisjoint"),
                                app2(cst("matrix_spec"), bvar(3), bvar(2)),
                            ),
                            app(
                                app(
                                    cst("ExistsUnique"),
                                    app3(cst("Matrix"), bvar(4), bvar(3), cst("Real")),
                                ),
                                app(cst("SylvesterSolution"), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// lyapunov_eq : ∀ (n : Nat) (A : Matrix n n Real) (Q : Matrix n n Real),
///   IsStable A → ∃! P : Matrix n n Real, A * P + P * Aᵀ + Q = 0
pub fn lyapunov_eq_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "A",
            app3(cst("Matrix"), bvar(0), bvar(0), cst("Real")),
            pi(
                BinderInfo::Default,
                "Q",
                app3(cst("Matrix"), bvar(1), bvar(1), cst("Real")),
                arrow(
                    app(cst("IsStable"), bvar(1)),
                    app(
                        app(
                            cst("ExistsUnique"),
                            app3(cst("Matrix"), bvar(2), bvar(2), cst("Real")),
                        ),
                        app2(cst("LyapunovSolution"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// MatrixPencil : ∀ (n : Nat) (α : Type), Type   (pair (A, B) for A - λB)
pub fn matrix_pencil_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "α", type0(), type0()),
    )
}
/// pencil_eigenvalue : ∀ (n : Nat) (α : Type) [Field α],
///   MatrixPencil n α → α → Prop
pub fn pencil_eigenvalue_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        impl_pi(
            "α",
            type0(),
            arrow(
                app(cst("Field"), bvar(0)),
                arrow(
                    app2(cst("MatrixPencil"), bvar(1), bvar(0)),
                    arrow(bvar(0), prop()),
                ),
            ),
        ),
    )
}
