//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BoundedOp, Distribution, FredholmOperatorData, InterpolationData, InterpolationMethod,
    L2Sequence, LuDecomposition, QrDecomposition, RnVector, SobolevSpaceData, WeakConvergenceData,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn _use_helpers() {
    let _ = (
        app(cst("a"), cst("b")),
        app2(cst("a"), cst("b"), cst("c")),
        prop(),
        type0(),
        arrow(prop(), prop()),
        bvar(0),
        nat_ty(),
        real_ty(),
        impl_pi("x", type0(), bvar(0)),
    );
}
pub fn normed_space_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn banach_space_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn hilbert_space_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn bounded_linear_op_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(BinderInfo::Default, "Y", type0(), type0()),
    )
}
pub fn dual_space_ty() -> Expr {
    arrow(type0(), type0())
}
pub fn spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app2(cst("BoundedLinearOp"), bvar(0), bvar(0)),
            app(cst("Set"), real_ty()),
        ),
    )
}
pub fn fredholm_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(BinderInfo::Default, "Y", type0(), prop()),
    )
}
pub fn fredholm_index_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(app2(cst("BoundedLinearOp"), bvar(1), bvar(0)), cst("Int")),
        ),
    )
}
pub fn compact_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(app2(cst("BoundedLinearOp"), bvar(1), bvar(0)), prop()),
        ),
    )
}
pub fn hahn_banach_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("NormedSpace"), bvar(0)), prop()),
    )
}
pub fn open_mapping_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app2(cst("BoundedLinearOp"), bvar(1), bvar(0)),
                arrow(
                    app(cst("BanachSpace"), bvar(2)),
                    arrow(app(cst("BanachSpace"), bvar(1)), prop()),
                ),
            ),
        ),
    )
}
pub fn closed_graph_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(app(cst("BanachSpace"), bvar(2)), prop()),
            ),
        ),
    )
}
pub fn uniform_boundedness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("BanachSpace"), bvar(1)),
                arrow(app(cst("BanachSpace"), bvar(0)), prop()),
            ),
        ),
    )
}
pub fn riesz_representation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(app(cst("HilbertSpace"), bvar(0)), prop()),
    )
}
pub fn spectral_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        type0(),
        arrow(
            app(cst("HilbertSpace"), bvar(0)),
            arrow(app2(cst("CompactSelfAdjoint"), bvar(1), bvar(0)), prop()),
        ),
    )
}
pub fn fredholm_alternative_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("BanachSpace"), bvar(0)),
            arrow(app2(cst("BoundedLinearOp"), bvar(1), bvar(1)), prop()),
        ),
    )
}
pub fn banach_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("BanachSpace"), bvar(0)),
            arrow(arrow(bvar(1), bvar(1)), prop()),
        ),
    )
}
pub fn build_functional_analysis_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("NormedSpace", normed_space_ty()),
        ("BanachSpace", banach_space_ty()),
        ("HilbertSpace", hilbert_space_ty()),
        ("BoundedLinearOp", bounded_linear_op_ty()),
        ("DualSpace", dual_space_ty()),
        ("Spectrum", spectrum_ty()),
        ("FredholmOperator", fredholm_operator_ty()),
        ("FredholmIndex", fredholm_index_ty()),
        ("CompactOperator", compact_operator_ty()),
        ("Set", arrow(type0(), type0())),
        (
            "CompactSelfAdjoint",
            pi(BinderInfo::Default, "H", type0(), arrow(type0(), type0())),
        ),
        ("Int", type0()),
        ("hahn_banach", hahn_banach_ty()),
        ("open_mapping", open_mapping_ty()),
        ("closed_graph", closed_graph_ty()),
        ("uniform_boundedness", uniform_boundedness_ty()),
        ("riesz_representation", riesz_representation_ty()),
        ("spectral_theorem", spectral_theorem_ty()),
        ("fredholm_alternative", fredholm_alternative_ty()),
        ("banach_fixed_point", banach_fixed_point_ty()),
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
pub fn gram_schmidt(vectors: &[RnVector]) -> Vec<RnVector> {
    let mut orthonormal = Vec::new();
    for v in vectors {
        let mut u = v.clone();
        for q in &orthonormal {
            let proj = u.project_onto(q);
            u = u.sub(&proj);
        }
        let n = u.norm();
        if n > 1e-12 {
            orthonormal.push(u.scale(1.0 / n));
        }
    }
    orthonormal
}
pub fn is_orthonormal(vectors: &[RnVector], tol: f64) -> bool {
    for (i, vi) in vectors.iter().enumerate() {
        if (vi.norm() - 1.0).abs() > tol {
            return false;
        }
        for vj in vectors.iter().skip(i + 1) {
            if vi.inner(vj).abs() > tol {
                return false;
            }
        }
    }
    true
}
pub fn spans_full_space(vectors: &[RnVector], dim: usize) -> bool {
    gram_schmidt(vectors).len() == dim
}
pub fn conjugate_gradient(a: &BoundedOp, b: &RnVector, max_iter: usize, tol: f64) -> RnVector {
    let n = b.dim();
    let mut x = RnVector::zero(n);
    let mut r = b.sub(&a.apply(&x));
    let mut p = r.clone();
    let mut rs_old = r.inner(&r);
    for _ in 0..max_iter {
        if rs_old.sqrt() < tol {
            break;
        }
        let ap = a.apply(&p);
        let alpha = rs_old / p.inner(&ap);
        x = x.add(&p.scale(alpha));
        r = r.sub(&ap.scale(alpha));
        let rs_new = r.inner(&r);
        if rs_new.sqrt() < tol {
            break;
        }
        p = r.add(&p.scale(rs_new / rs_old));
        rs_old = rs_new;
    }
    x
}
pub fn l2_norm_approx(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * f(a).powi(2) + 0.5 * f(b).powi(2);
    for i in 1..n {
        sum += f(a + i as f64 * h).powi(2);
    }
    (sum * h).sqrt()
}
pub fn l2_inner_approx(
    f: &dyn Fn(f64) -> f64,
    g: &dyn Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * f(a) * g(a) + 0.5 * f(b) * g(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x) * g(x);
    }
    sum * h
}
pub fn sup_norm_approx(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    if n == 0 {
        return f(a).abs();
    }
    let h = (b - a) / n as f64;
    let mut max_val = 0.0_f64;
    for i in 0..=n {
        max_val = max_val.max(f(a + i as f64 * h).abs());
    }
    max_val
}
pub fn banach_fixed_point_iter(
    t: &dyn Fn(f64) -> f64,
    x0: f64,
    max_iter: usize,
    tol: f64,
) -> (f64, usize) {
    let mut x = x0;
    for i in 0..max_iter {
        let x_next = t(x);
        if (x_next - x).abs() < tol {
            return (x_next, i + 1);
        }
        x = x_next;
    }
    (x, max_iter)
}
pub fn fourier_cosine_coefficients(
    f: &dyn Fn(f64) -> f64,
    period: f64,
    num_coeffs: usize,
    qp: usize,
) -> Vec<f64> {
    let h = period / qp as f64;
    let mut coeffs = Vec::with_capacity(num_coeffs);
    for k in 0..num_coeffs {
        let mut sum =
            0.5 * f(0.0) + 0.5 * f(period) * (2.0 * std::f64::consts::PI * k as f64).cos();
        for i in 1..qp {
            let x = i as f64 * h;
            sum += f(x) * (2.0 * std::f64::consts::PI * k as f64 * x / period).cos();
        }
        let factor = if k == 0 { 1.0 / period } else { 2.0 / period };
        coeffs.push(sum * h * factor);
    }
    coeffs
}
pub fn eval_fourier_cosine(coeffs: &[f64], period: f64, x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(k, &c)| c * (2.0 * std::f64::consts::PI * k as f64 * x / period).cos())
        .sum()
}
/// Compute QR decomposition of matrix A using modified Gram-Schmidt.
pub fn qr_decomposition(a: &BoundedOp) -> Option<QrDecomposition> {
    let (m, n) = (a.range_dim, a.domain_dim);
    if m == 0 || n == 0 {
        return None;
    }
    let mut cols: Vec<RnVector> = (0..n)
        .map(|j| RnVector::new((0..m).map(|i| a.matrix[i][j]).collect()))
        .collect();
    let mut q_cols: Vec<RnVector> = Vec::new();
    let mut r_matrix = vec![vec![0.0; n]; n.min(m)];
    for j in 0..n.min(m) {
        for k in 0..q_cols.len() {
            let rk = cols[j].inner(&q_cols[k]);
            r_matrix[k][j] = rk;
            cols[j] = cols[j].sub(&q_cols[k].scale(rk));
        }
        let nrm = cols[j].norm();
        if nrm < 1e-14 {
            r_matrix[j][j] = 0.0;
            q_cols.push(RnVector::zero(m));
        } else {
            r_matrix[j][j] = nrm;
            q_cols.push(cols[j].scale(1.0 / nrm));
        }
    }
    let q_n = q_cols.len();
    let q_matrix: Vec<Vec<f64>> = (0..m)
        .map(|i| (0..q_n).map(|j| q_cols[j].components[i]).collect())
        .collect();
    let q = BoundedOp {
        matrix: q_matrix,
        domain_dim: q_n,
        range_dim: m,
    };
    let r = BoundedOp {
        matrix: r_matrix,
        domain_dim: n,
        range_dim: q_n,
    };
    Some(QrDecomposition { q, r })
}
/// Compute LU decomposition with partial pivoting.
pub fn lu_decomposition(a: &BoundedOp) -> Option<LuDecomposition> {
    let n = a.domain_dim;
    if n != a.range_dim || n == 0 {
        return None;
    }
    let mut u = a.matrix.clone();
    let mut l = vec![vec![0.0; n]; n];
    let mut perm: Vec<usize> = (0..n).collect();
    for k in 0..n {
        let mut max_val = u[k][k].abs();
        let mut max_row = k;
        for row in (k + 1)..n {
            if u[row][k].abs() > max_val {
                max_val = u[row][k].abs();
                max_row = row;
            }
        }
        if max_val < 1e-15 {
            return None;
        }
        if max_row != k {
            u.swap(k, max_row);
            l.swap(k, max_row);
            perm.swap(k, max_row);
        }
        l[k][k] = 1.0;
        for i in (k + 1)..n {
            let factor = u[i][k] / u[k][k];
            l[i][k] = factor;
            for j in k..n {
                let val = u[k][j];
                u[i][j] -= factor * val;
            }
        }
    }
    Some(LuDecomposition {
        l: BoundedOp {
            matrix: l,
            domain_dim: n,
            range_dim: n,
        },
        u: BoundedOp {
            matrix: u,
            domain_dim: n,
            range_dim: n,
        },
        permutation: perm,
    })
}
/// Solve Ax = b using LU decomposition.
pub fn lu_solve(lu: &LuDecomposition, b: &RnVector) -> RnVector {
    let n = lu.l.domain_dim;
    let mut pb = vec![0.0; n];
    for (i, &pi) in lu.permutation.iter().enumerate() {
        pb[i] = b.components[pi];
    }
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = pb[i];
        for j in 0..i {
            sum -= lu.l.matrix[i][j] * y[j];
        }
        y[i] = sum;
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for j in (i + 1)..n {
            sum -= lu.u.matrix[i][j] * x[j];
        }
        if lu.u.matrix[i][i].abs() < 1e-15 {
            x[i] = 0.0;
        } else {
            x[i] = sum / lu.u.matrix[i][i];
        }
    }
    RnVector::new(x)
}
/// Compute singular values of matrix A (sorted descending).
pub fn singular_values(a: &BoundedOp, max_iter: usize) -> Vec<f64> {
    let ata = match a.transpose().compose(a) {
        Some(m) => m,
        None => return vec![],
    };
    let n = ata.domain_dim;
    if n == 0 {
        return vec![];
    }
    let mut current = ata.clone();
    for _ in 0..max_iter {
        let qr = match qr_decomposition(&current) {
            Some(qr) => qr,
            None => break,
        };
        current = match qr.r.compose(&qr.q) {
            Some(m) => m,
            None => break,
        };
    }
    let mut eigenvalues: Vec<f64> = (0..n).map(|i| current.matrix[i][i].max(0.0)).collect();
    eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    eigenvalues.iter().map(|&ev| ev.sqrt()).collect()
}
/// Condition number of a matrix (ratio of largest to smallest singular value).
pub fn condition_number(a: &BoundedOp, max_iter: usize) -> f64 {
    let svs = singular_values(a, max_iter);
    if svs.is_empty() {
        return f64::INFINITY;
    }
    let max_sv = svs[0];
    let min_sv = *svs
        .last()
        .expect("svs is non-empty: checked by early return");
    if min_sv < 1e-15 {
        f64::INFINITY
    } else {
        max_sv / min_sv
    }
}
/// Approximate spectral radius using power iteration.
pub fn spectral_radius(a: &BoundedOp, iterations: usize) -> f64 {
    if a.domain_dim != a.range_dim || a.domain_dim == 0 {
        return 0.0;
    }
    let n = a.domain_dim;
    let mut v = RnVector::new(vec![1.0; n]);
    let nrm = v.norm();
    if nrm > 0.0 {
        v = v.scale(1.0 / nrm);
    }
    let mut radius = 0.0;
    for _ in 0..iterations {
        let w = a.apply(&v);
        let nrm = w.norm();
        if nrm < 1e-15 {
            return 0.0;
        }
        radius = nrm;
        v = w.scale(1.0 / nrm);
    }
    radius
}
/// Approximate the H^1 Sobolev norm of f on \[a,b\]:
/// ||f||_{H^1}^2 = ||f||_{L^2}^2 + ||f'||_{L^2}^2
pub fn h1_norm_approx(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let l2_sq = l2_norm_approx(f, a, b, n).powi(2);
    if n < 2 {
        return l2_sq.sqrt();
    }
    let h = (b - a) / n as f64;
    let mut deriv_sq_sum = 0.0;
    for i in 0..n {
        let x = a + i as f64 * h;
        let df = (f(x + h) - f(x)) / h;
        deriv_sq_sum += df * df;
    }
    let deriv_l2_sq = deriv_sq_sum * h;
    (l2_sq + deriv_l2_sq).sqrt()
}
/// Approximate the H^2 Sobolev norm (includes second derivative).
pub fn h2_norm_approx(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h1_sq = h1_norm_approx(f, a, b, n).powi(2);
    if n < 3 {
        return h1_sq.sqrt();
    }
    let h = (b - a) / n as f64;
    let mut d2_sq_sum = 0.0;
    for i in 1..(n - 1) {
        let x = a + i as f64 * h;
        let d2f = (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h);
        d2_sq_sum += d2f * d2f;
    }
    let d2_l2_sq = d2_sq_sum * h;
    (h1_sq + d2_l2_sq).sqrt()
}
/// Simpson's rule for numerical integration.
pub fn simpson_integrate(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = if n % 2 == 0 { n } else { n + 1 };
    if n == 0 {
        return 0.0;
    }
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 2 == 0 { 2.0 * f(x) } else { 4.0 * f(x) };
    }
    sum * h / 3.0
}
/// Gauss-Legendre 3-point quadrature on \[a,b\].
pub fn gauss_legendre_3(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mid = (a + b) / 2.0;
    let half = (b - a) / 2.0;
    let nodes = [-0.7745966692414834, 0.0, 0.7745966692414834];
    let weights = [0.5555555555555556, 0.8888888888888888, 0.5555555555555556];
    let mut sum = 0.0;
    for i in 0..3 {
        sum += weights[i] * f(mid + half * nodes[i]);
    }
    sum * half
}
/// Composite Gauss-Legendre 3-point quadrature.
pub fn gauss_legendre_composite(f: &dyn Fn(f64) -> f64, a: f64, b: f64, panels: usize) -> f64 {
    if panels == 0 {
        return 0.0;
    }
    let h = (b - a) / panels as f64;
    let mut total = 0.0;
    for i in 0..panels {
        let lo = a + i as f64 * h;
        let hi = lo + h;
        total += gauss_legendre_3(f, lo, hi);
    }
    total
}
/// Matrix exponential via scaling and squaring with Pade approximation.
pub fn matrix_exponential(a: &BoundedOp, terms: usize) -> BoundedOp {
    let n = a.domain_dim;
    if n != a.range_dim {
        return BoundedOp::zero_op(n, n);
    }
    let mut result = BoundedOp::identity(n);
    let mut term = BoundedOp::identity(n);
    for k in 1..terms {
        term = match term.compose(a) {
            Some(m) => m.scalar_mul(1.0 / k as f64),
            None => break,
        };
        result = match result.op_add(&term) {
            Some(m) => m,
            None => break,
        };
    }
    result
}
/// Commutator \[A, B\] = AB - BA.
pub fn commutator(a: &BoundedOp, b: &BoundedOp) -> Option<BoundedOp> {
    let ab = a.compose(b)?;
    let ba = b.compose(a)?;
    let neg_ba = ba.scalar_mul(-1.0);
    ab.op_add(&neg_ba)
}
/// Anti-commutator {A, B} = AB + BA.
pub fn anti_commutator(a: &BoundedOp, b: &BoundedOp) -> Option<BoundedOp> {
    let ab = a.compose(b)?;
    let ba = b.compose(a)?;
    ab.op_add(&ba)
}
/// Check if operator T = I - K is Fredholm by estimating kernel and cokernel dimensions.
/// Returns (kernel_dim, cokernel_dim, index).
pub fn fredholm_index_numerical(k: &BoundedOp) -> (usize, usize, i64) {
    let n = k.domain_dim;
    if n != k.range_dim {
        return (0, 0, 0);
    }
    let i_minus_k = match BoundedOp::identity(n).op_add(&k.scalar_mul(-1.0)) {
        Some(m) => m,
        None => return (0, 0, 0),
    };
    let kernel_dim = i_minus_k.nullity();
    let cokernel_dim = i_minus_k.transpose().nullity();
    let index = kernel_dim as i64 - cokernel_dim as i64;
    (kernel_dim, cokernel_dim, index)
}
/// Jacobi iterative method for solving Ax = b.
pub fn jacobi_iteration(a: &BoundedOp, b: &RnVector, max_iter: usize, tol: f64) -> RnVector {
    let n = a.domain_dim;
    if n != a.range_dim || n != b.dim() {
        return RnVector::zero(n);
    }
    let mut x = RnVector::zero(n);
    for _ in 0..max_iter {
        let mut x_new = vec![0.0; n];
        for i in 0..n {
            if a.matrix[i][i].abs() < 1e-15 {
                x_new[i] = x.components[i];
                continue;
            }
            let mut sum = b.components[i];
            for j in 0..n {
                if j != i {
                    sum -= a.matrix[i][j] * x.components[j];
                }
            }
            x_new[i] = sum / a.matrix[i][i];
        }
        let new_x = RnVector::new(x_new);
        if new_x.sub(&x).norm() < tol {
            return new_x;
        }
        x = new_x;
    }
    x
}
/// Gauss-Seidel iterative method for solving Ax = b.
pub fn gauss_seidel_iteration(a: &BoundedOp, b: &RnVector, max_iter: usize, tol: f64) -> RnVector {
    let n = a.domain_dim;
    if n != a.range_dim || n != b.dim() {
        return RnVector::zero(n);
    }
    let mut x = vec![0.0; n];
    for _ in 0..max_iter {
        let old_x = x.clone();
        for i in 0..n {
            if a.matrix[i][i].abs() < 1e-15 {
                continue;
            }
            let mut sum = b.components[i];
            for j in 0..n {
                if j != i {
                    sum -= a.matrix[i][j] * x[j];
                }
            }
            x[i] = sum / a.matrix[i][i];
        }
        let diff: f64 = x
            .iter()
            .zip(old_x.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        if diff < tol {
            break;
        }
    }
    RnVector::new(x)
}
/// Compute Fourier sine coefficients on \[0, L\].
pub fn fourier_sine_coefficients(
    f: &dyn Fn(f64) -> f64,
    period: f64,
    num_coeffs: usize,
    qp: usize,
) -> Vec<f64> {
    let h = period / qp as f64;
    let mut coeffs = Vec::with_capacity(num_coeffs);
    for k in 0..num_coeffs {
        let mut sum = 0.0;
        for i in 0..=qp {
            let x = i as f64 * h;
            let w = if i == 0 || i == qp { 0.5 } else { 1.0 };
            sum += w * f(x) * (std::f64::consts::PI * (k + 1) as f64 * x / period).sin();
        }
        coeffs.push(sum * h * 2.0 / period);
    }
    coeffs
}
/// Evaluate Fourier sine series at a point.
pub fn eval_fourier_sine(coeffs: &[f64], period: f64, x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(k, &c)| c * (std::f64::consts::PI * (k + 1) as f64 * x / period).sin())
        .sum()
}
/// Evaluate the n-th Chebyshev polynomial T_n(x) via recurrence.
pub fn chebyshev_t(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }
    let (mut t0, mut t1) = (1.0, x);
    for _ in 2..=n {
        let t2 = 2.0 * x * t1 - t0;
        t0 = t1;
        t1 = t2;
    }
    t1
}
/// Chebyshev nodes on \[a,b\].
pub fn chebyshev_nodes(n: usize, a: f64, b: f64) -> Vec<f64> {
    (0..n)
        .map(|k| {
            let theta = std::f64::consts::PI * (2 * k + 1) as f64 / (2 * n) as f64;
            (a + b) / 2.0 + (b - a) / 2.0 * theta.cos()
        })
        .collect()
}
/// Chebyshev interpolation coefficients for f on \[a,b\] at n nodes.
pub fn chebyshev_coefficients(f: &dyn Fn(f64) -> f64, n: usize, a: f64, b: f64) -> Vec<f64> {
    let nodes = chebyshev_nodes(n, a, b);
    let values: Vec<f64> = nodes.iter().map(|&x| f(x)).collect();
    let mut coeffs = vec![0.0; n];
    for j in 0..n {
        let mut sum = 0.0;
        for (k, &val) in values.iter().enumerate() {
            let xk = 2.0 * (nodes[k] - a) / (b - a) - 1.0;
            sum += val * chebyshev_t(j, xk);
        }
        coeffs[j] = sum
            * if j == 0 {
                1.0 / n as f64
            } else {
                2.0 / n as f64
            };
    }
    coeffs
}
/// Evaluate Chebyshev series at point x on \[a,b\].
pub fn eval_chebyshev(coeffs: &[f64], a: f64, b: f64, x: f64) -> f64 {
    let t = 2.0 * (x - a) / (b - a) - 1.0;
    coeffs
        .iter()
        .enumerate()
        .map(|(k, &c)| c * chebyshev_t(k, t))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rn_vector_norm() {
        let v = RnVector::new(vec![3.0, 4.0]);
        assert!((v.norm() - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_inner() {
        let e1 = RnVector::new(vec![1.0, 0.0]);
        let e2 = RnVector::new(vec![0.0, 1.0]);
        assert!(e1.inner(&e2).abs() < 1e-10);
        assert!((e1.inner(&e1) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_normalized() {
        let v = RnVector::new(vec![3.0, 4.0]);
        let u = v.normalized().expect("nonzero");
        assert!((u.norm() - 1.0).abs() < 1e-10);
        assert!(RnVector::zero(3).normalized().is_none());
    }
    #[test]
    fn test_rn_vector_basis() {
        let e0 = RnVector::basis(3, 0);
        assert!((e0.components[0] - 1.0).abs() < 1e-10);
        assert!(e0.components[1].abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_sub() {
        let c = RnVector::new(vec![5.0, 3.0]).sub(&RnVector::new(vec![2.0, 1.0]));
        assert!((c.components[0] - 3.0).abs() < 1e-10);
        assert!((c.components[1] - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_cross() {
        let k = RnVector::new(vec![1.0, 0.0, 0.0])
            .cross(&RnVector::new(vec![0.0, 1.0, 0.0]))
            .expect("cross should succeed");
        assert!(k.components[0].abs() < 1e-10);
        assert!(k.components[1].abs() < 1e-10);
        assert!((k.components[2] - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_project() {
        let proj = RnVector::new(vec![3.0, 4.0]).project_onto(&RnVector::new(vec![1.0, 0.0]));
        assert!((proj.components[0] - 3.0).abs() < 1e-10);
        assert!(proj.components[1].abs() < 1e-10);
    }
    #[test]
    fn test_gram_schmidt() {
        let basis = gram_schmidt(&[
            RnVector::new(vec![1.0, 1.0, 0.0]),
            RnVector::new(vec![1.0, 0.0, 1.0]),
            RnVector::new(vec![0.0, 1.0, 1.0]),
        ]);
        assert_eq!(basis.len(), 3);
        assert!(is_orthonormal(&basis, 1e-10));
    }
    #[test]
    fn test_gram_schmidt_dependent() {
        let basis = gram_schmidt(&[
            RnVector::new(vec![1.0, 0.0]),
            RnVector::new(vec![2.0, 0.0]),
            RnVector::new(vec![0.0, 1.0]),
        ]);
        assert_eq!(basis.len(), 2);
        assert!(is_orthonormal(&basis, 1e-10));
    }
    #[test]
    fn test_spans_full_space() {
        assert!(spans_full_space(
            &[RnVector::new(vec![1.0, 0.0]), RnVector::new(vec![0.0, 1.0])],
            2
        ));
        assert!(!spans_full_space(
            &[
                RnVector::new(vec![1.0, 0.0, 0.0]),
                RnVector::new(vec![0.0, 1.0, 0.0])
            ],
            3
        ));
    }
    #[test]
    fn test_bounded_op_apply() {
        let id = BoundedOp::identity(3);
        let v = RnVector::new(vec![1.0, 2.0, 3.0]);
        let w = id.apply(&v);
        for (a, b) in w.components.iter().zip(v.components.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }
    #[test]
    fn test_bounded_op_transpose() {
        let a = BoundedOp::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let at = a.transpose();
        assert_eq!(at.range_dim, 3);
        assert_eq!(at.domain_dim, 2);
        let att = at.transpose();
        for i in 0..2 {
            for j in 0..3 {
                assert!((att.matrix[i][j] - a.matrix[i][j]).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_bounded_op_symmetric() {
        assert!(BoundedOp::new(vec![vec![2.0, 1.0], vec![1.0, 3.0]]).is_symmetric());
        assert!(!BoundedOp::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).is_symmetric());
    }
    #[test]
    fn test_bounded_op_diagonal() {
        let w = BoundedOp::diagonal(&[2.0, 3.0, 5.0]).apply(&RnVector::new(vec![1.0, 1.0, 1.0]));
        assert!((w.components[0] - 2.0).abs() < 1e-10);
        assert!((w.components[1] - 3.0).abs() < 1e-10);
        assert!((w.components[2] - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_bounded_op_add() {
        let c = BoundedOp::identity(2)
            .op_add(&BoundedOp::identity(2))
            .expect("op_add should succeed");
        assert!((c.matrix[0][0] - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_bounded_op_determinant() {
        assert!(
            (BoundedOp::identity(3)
                .determinant()
                .expect("determinant should succeed")
                - 1.0)
                .abs()
                < 1e-10
        );
        assert!(
            (BoundedOp::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
                .determinant()
                .expect("determinant should succeed")
                - (-2.0))
                .abs()
                < 1e-10
        );
    }
    #[test]
    fn test_bounded_op_solve() {
        let x = BoundedOp::new(vec![vec![2.0, 1.0], vec![1.0, 3.0]])
            .solve(&RnVector::new(vec![5.0, 7.0]))
            .expect("solve should succeed");
        assert!((x.components[0] - 1.6).abs() < 1e-10);
        assert!((x.components[1] - 1.8).abs() < 1e-10);
    }
    #[test]
    fn test_bounded_op_rank_nullity() {
        let a = BoundedOp::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);
        assert_eq!(a.rank(), 2);
        assert_eq!(a.nullity(), 1);
    }
    #[test]
    fn test_bounded_op_positive_definite() {
        assert!(BoundedOp::new(vec![vec![2.0, 1.0], vec![1.0, 2.0]]).is_positive_definite());
        assert!(!BoundedOp::new(vec![vec![1.0, 2.0], vec![2.0, 1.0]]).is_positive_definite());
    }
    #[test]
    fn test_power_iteration() {
        let (ev, _) = BoundedOp::diagonal(&[5.0, 1.0])
            .power_iteration(100)
            .expect("power_iteration should succeed");
        assert!((ev - 5.0).abs() < 1e-6, "got {ev}");
    }
    #[test]
    fn test_eigenvalues_2x2() {
        let (e1, e2) = BoundedOp::new(vec![vec![3.0, 1.0], vec![1.0, 3.0]])
            .eigenvalues_2x2()
            .expect("eigenvalues_2x2 should succeed");
        assert!((e1 - 4.0).abs() < 1e-10);
        assert!((e2 - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_conjugate_gradient() {
        let a = BoundedOp::new(vec![vec![4.0, 1.0], vec![1.0, 3.0]]);
        let x = conjugate_gradient(&a, &RnVector::new(vec![1.0, 2.0]), 100, 1e-12);
        let ax = a.apply(&x);
        assert!((ax.components[0] - 1.0).abs() < 1e-8);
        assert!((ax.components[1] - 2.0).abs() < 1e-8);
    }
    #[test]
    fn test_conjugate_gradient_identity() {
        let b = RnVector::new(vec![1.0, 2.0, 3.0]);
        let x = conjugate_gradient(&BoundedOp::identity(3), &b, 100, 1e-12);
        for i in 0..3 {
            assert!((x.components[i] - b.components[i]).abs() < 1e-8);
        }
    }
    #[test]
    fn test_l2_sequence_norm() {
        assert!((L2Sequence::new(vec![1.0, 0.0, 0.0]).l2_norm() - 1.0).abs() < 1e-10);
        let b = L2Sequence::new(vec![3.0, 4.0]);
        assert!((b.l2_norm() - 5.0).abs() < 1e-10);
        assert!(b.is_in_l2());
    }
    #[test]
    fn test_l2_sequence_shift() {
        let s = L2Sequence::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(s.shift_left().terms, vec![2.0, 3.0]);
        assert_eq!(s.shift_right().terms, vec![0.0, 1.0, 2.0, 3.0]);
    }
    #[test]
    fn test_l2_sequence_convolve() {
        let c = L2Sequence::new(vec![1.0, 2.0]).convolve(&L2Sequence::new(vec![3.0, 4.0]));
        assert!((c.terms[0] - 3.0).abs() < 1e-10);
        assert!((c.terms[1] - 10.0).abs() < 1e-10);
        assert!((c.terms[2] - 8.0).abs() < 1e-10);
    }
    #[test]
    fn test_l2_sequence_parseval() {
        assert!(L2Sequence::new(vec![1.0, 2.0, 3.0, 4.0]).parseval_residual() < 1e-10);
    }
    #[test]
    fn test_l2_norm_approx() {
        assert!((l2_norm_approx(&|_| 1.0, 0.0, 1.0, 1000) - 1.0).abs() < 1e-3);
    }
    #[test]
    fn test_l2_inner_approx() {
        let ip = l2_inner_approx(
            &|x: f64| x.sin(),
            &|x: f64| x.cos(),
            0.0,
            2.0 * std::f64::consts::PI,
            10000,
        );
        assert!(ip.abs() < 1e-3, "got {ip}");
    }
    #[test]
    fn test_sup_norm_approx() {
        let n = sup_norm_approx(&|x: f64| x.sin(), 0.0, std::f64::consts::PI, 10000);
        assert!((n - 1.0).abs() < 1e-3, "got {n}");
    }
    #[test]
    fn test_banach_fixed_point_iter() {
        let (fp, iters) = banach_fixed_point_iter(&|x: f64| x.cos(), 0.5, 1000, 1e-10);
        assert!((fp - fp.cos()).abs() < 1e-8);
        assert!(iters < 1000);
    }
    #[test]
    fn test_fourier_cosine_coefficients() {
        let c = fourier_cosine_coefficients(&|_| 1.0, 1.0, 5, 1000);
        assert!((c[0] - 1.0).abs() < 1e-3);
        for k in 1..5 {
            assert!(c[k].abs() < 1e-3);
        }
    }
    #[test]
    fn test_eval_fourier_cosine() {
        assert!((eval_fourier_cosine(&[1.0, 0.0, 0.0], 1.0, 0.5) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_bounded_op_operator_norm_power() {
        let n = BoundedOp::diagonal(&[3.0, 1.0, 2.0]).operator_norm_power_iter(200);
        assert!((n - 3.0).abs() < 1e-3, "got {n}");
    }
    #[test]
    fn test_bounded_op_frobenius_norm() {
        let frob = BoundedOp::identity(2).frobenius_norm();
        assert!((frob - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(BoundedOp::identity(2).operator_norm() > 0.0);
    }
    #[test]
    fn test_bounded_op_compose() {
        let a = BoundedOp::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let c = a
            .compose(&BoundedOp::identity(2))
            .expect("BoundedOp::new should succeed");
        for i in 0..2 {
            for j in 0..2 {
                assert!((c.matrix[i][j] - a.matrix[i][j]).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_bounded_op_scalar_mul() {
        let b = BoundedOp::identity(2).scalar_mul(3.0);
        assert!((b.matrix[0][0] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_rn_vector_lp_norms() {
        let v = RnVector::new(vec![3.0, 4.0]);
        assert!((v.norm_p(1.0) - 7.0).abs() < 1e-10);
        assert!((v.norm_p(f64::INFINITY) - 4.0).abs() < 1e-10);
    }
    #[test]
    fn test_build_functional_analysis_env() {
        let mut env = Environment::new();
        build_functional_analysis_env(&mut env);
        assert!(env.get(&Name::str("NormedSpace")).is_some());
        assert!(env.get(&Name::str("hahn_banach")).is_some());
        assert!(env.get(&Name::str("fredholm_alternative")).is_some());
        assert!(env.get(&Name::str("banach_fixed_point")).is_some());
    }
    #[test]
    fn test_bounded_op_injective_surjective() {
        assert!(BoundedOp::identity(3).is_injective());
        assert!(BoundedOp::identity(3).is_surjective());
        let s = BoundedOp::new(vec![vec![1.0, 0.0], vec![0.0, 0.0]]);
        assert!(!s.is_injective());
        assert!(!s.is_surjective());
    }
    #[test]
    fn test_rn_vector_angle() {
        let angle = RnVector::new(vec![1.0, 0.0]).angle_with(&RnVector::new(vec![0.0, 1.0]));
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }
    #[test]
    fn test_qr_identity() {
        let qr = qr_decomposition(&BoundedOp::identity(3)).expect("operation should succeed");
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((qr.q.matrix[i][j] - expected).abs() < 1e-10);
                assert!((qr.r.matrix[i][j] - expected).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_qr_reconstruction() {
        let a = BoundedOp::new(vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
        let qr = qr_decomposition(&a).expect("BoundedOp::new should succeed");
        let prod = qr.q.compose(&qr.r).expect("compose should succeed");
        for i in 0..a.range_dim {
            for j in 0..a.domain_dim {
                assert!(
                    (prod.matrix[i][j] - a.matrix[i][j]).abs() < 1e-10,
                    "mismatch at ({i},{j}): {} vs {}",
                    prod.matrix[i][j],
                    a.matrix[i][j]
                );
            }
        }
    }
    #[test]
    fn test_qr_orthogonality() {
        let a = BoundedOp::new(vec![
            vec![12.0, -51.0, 4.0],
            vec![6.0, 167.0, -68.0],
            vec![-4.0, 24.0, -41.0],
        ]);
        let qr = qr_decomposition(&a).expect("operation should succeed");
        let qtq =
            qr.q.transpose()
                .compose(&qr.q)
                .expect("compose should succeed");
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((qtq.matrix[i][j] - expected).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_lu_solve() {
        let a = BoundedOp::new(vec![
            vec![2.0, 1.0, 1.0],
            vec![4.0, 3.0, 3.0],
            vec![8.0, 7.0, 9.0],
        ]);
        let b = RnVector::new(vec![1.0, 1.0, 1.0]);
        let lu = lu_decomposition(&a).expect("RnVector::new should succeed");
        let x = lu_solve(&lu, &b);
        let ax = a.apply(&x);
        for i in 0..3 {
            assert!(
                (ax.components[i] - b.components[i]).abs() < 1e-8,
                "lu_solve error at {i}"
            );
        }
    }
    #[test]
    fn test_lu_identity() {
        let lu = lu_decomposition(&BoundedOp::identity(3)).expect("operation should succeed");
        for i in 0..3 {
            assert!((lu.l.matrix[i][i] - 1.0).abs() < 1e-10);
            assert!((lu.u.matrix[i][i] - 1.0).abs() < 1e-10);
        }
    }
    #[test]
    fn test_singular_values_diagonal() {
        let svs = singular_values(&BoundedOp::diagonal(&[3.0, 1.0, 5.0]), 50);
        assert!((svs[0] - 5.0).abs() < 0.1, "sv[0] = {}", svs[0]);
        assert!((svs[1] - 3.0).abs() < 0.1, "sv[1] = {}", svs[1]);
        assert!((svs[2] - 1.0).abs() < 0.1, "sv[2] = {}", svs[2]);
    }
    #[test]
    fn test_condition_number_identity() {
        let cn = condition_number(&BoundedOp::identity(3), 50);
        assert!((cn - 1.0).abs() < 0.1, "cond(I) = {cn}");
    }
    #[test]
    fn test_condition_number_ill_conditioned() {
        let cn = condition_number(&BoundedOp::diagonal(&[100.0, 0.01]), 50);
        assert!(cn > 1000.0, "expected ill-conditioned, got {cn}");
    }
    #[test]
    fn test_spectral_radius() {
        let rho = spectral_radius(&BoundedOp::diagonal(&[2.0, -3.0, 1.0]), 200);
        assert!((rho - 3.0).abs() < 0.1, "got {rho}");
    }
    #[test]
    fn test_h1_norm_constant() {
        let h1 = h1_norm_approx(&|_| 1.0, 0.0, 1.0, 1000);
        assert!((h1 - 1.0).abs() < 1e-2, "got {h1}");
    }
    #[test]
    fn test_h1_norm_linear() {
        let h1 = h1_norm_approx(&|x| x, 0.0, 1.0, 1000);
        let expected = (1.0 / 3.0 + 1.0_f64).sqrt();
        assert!(
            (h1 - expected).abs() < 0.05,
            "got {h1}, expected {expected}"
        );
    }
    #[test]
    fn test_h2_norm() {
        let h2 = h2_norm_approx(&|x: f64| x * x, 0.0, 1.0, 1000);
        assert!(h2 > 0.0);
    }
    #[test]
    fn test_simpson_integrate() {
        let result = simpson_integrate(&|x| x * x, 0.0, 1.0, 100);
        assert!((result - 1.0 / 3.0).abs() < 1e-6, "got {result}");
    }
    #[test]
    fn test_gauss_legendre_quadrature() {
        let result = gauss_legendre_composite(&|x| x * x, 0.0, 1.0, 10);
        assert!((result - 1.0 / 3.0).abs() < 1e-10, "got {result}");
    }
    #[test]
    fn test_matrix_exponential_zero() {
        let exp_zero = matrix_exponential(&BoundedOp::zero_op(3, 3), 20);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((exp_zero.matrix[i][j] - expected).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_matrix_exponential_identity() {
        let exp_i = matrix_exponential(&BoundedOp::identity(2), 20);
        let e = std::f64::consts::E;
        assert!(
            (exp_i.matrix[0][0] - e).abs() < 1e-6,
            "got {}",
            exp_i.matrix[0][0]
        );
        assert!((exp_i.matrix[1][1] - e).abs() < 1e-6);
        assert!(exp_i.matrix[0][1].abs() < 1e-6);
    }
    #[test]
    fn test_commutator_self() {
        let a = BoundedOp::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let c = commutator(&a, &a).expect("BoundedOp::new should succeed");
        for i in 0..2 {
            for j in 0..2 {
                assert!(c.matrix[i][j].abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_anti_commutator() {
        let a = BoundedOp::identity(2);
        let ac = anti_commutator(&a, &a).expect("BoundedOp::identity should succeed");
        assert!((ac.matrix[0][0] - 2.0).abs() < 1e-10);
        assert!((ac.matrix[1][1] - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_fredholm_index_zero() {
        let k = BoundedOp::zero_op(3, 3);
        let (kd, cd, idx) = fredholm_index_numerical(&k);
        assert_eq!(kd, 0);
        assert_eq!(cd, 0);
        assert_eq!(idx, 0);
    }
    #[test]
    fn test_fredholm_index_projection() {
        let k = BoundedOp::new(vec![vec![1.0, 0.0], vec![0.0, 0.0]]);
        let (kd, _cd, _idx) = fredholm_index_numerical(&k);
        assert_eq!(kd, 1);
    }
    #[test]
    fn test_jacobi_iteration() {
        let a = BoundedOp::new(vec![vec![4.0, 1.0], vec![1.0, 3.0]]);
        let b = RnVector::new(vec![1.0, 2.0]);
        let x = jacobi_iteration(&a, &b, 200, 1e-10);
        let ax = a.apply(&x);
        assert!((ax.components[0] - b.components[0]).abs() < 1e-6);
        assert!((ax.components[1] - b.components[1]).abs() < 1e-6);
    }
    #[test]
    fn test_gauss_seidel_iteration() {
        let a = BoundedOp::new(vec![vec![4.0, 1.0], vec![1.0, 3.0]]);
        let b = RnVector::new(vec![1.0, 2.0]);
        let x = gauss_seidel_iteration(&a, &b, 200, 1e-10);
        let ax = a.apply(&x);
        assert!((ax.components[0] - b.components[0]).abs() < 1e-6);
        assert!((ax.components[1] - b.components[1]).abs() < 1e-6);
    }
    #[test]
    fn test_fourier_sine_coefficients() {
        let coeffs =
            fourier_sine_coefficients(&|x: f64| (std::f64::consts::PI * x).sin(), 1.0, 5, 1000);
        assert!((coeffs[0] - 1.0).abs() < 1e-2, "got {}", coeffs[0]);
        for k in 1..5 {
            assert!(coeffs[k].abs() < 1e-2, "coeff[{k}] = {}", coeffs[k]);
        }
    }
    #[test]
    fn test_eval_fourier_sine() {
        assert!((eval_fourier_sine(&[1.0, 0.0, 0.0], 1.0, 0.5) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_chebyshev_t_values() {
        assert!((chebyshev_t(0, 0.5) - 1.0).abs() < 1e-10);
        assert!((chebyshev_t(1, 0.5) - 0.5).abs() < 1e-10);
        assert!((chebyshev_t(2, 0.5) - (-0.5)).abs() < 1e-10);
    }
    #[test]
    fn test_chebyshev_nodes() {
        let nodes = chebyshev_nodes(4, -1.0, 1.0);
        assert_eq!(nodes.len(), 4);
        for &x in &nodes {
            assert!(x >= -1.0 && x <= 1.0);
        }
    }
    #[test]
    fn test_chebyshev_approximation() {
        let coeffs = chebyshev_coefficients(&|x: f64| x.sin(), 10, -1.0, 1.0);
        let approx = eval_chebyshev(&coeffs, -1.0, 1.0, 0.5);
        assert!(
            (approx - 0.5_f64.sin()).abs() < 0.1,
            "got {approx}, expected {}",
            0.5_f64.sin()
        );
    }
}
#[cfg(test)]
mod tests_functional_analysis_ext {
    use super::*;
    #[test]
    fn test_fredholm_operator() {
        let fo = FredholmOperatorData::new("D_A", 2, 1);
        assert_eq!(fo.index(), 1);
        assert!(!fo.is_isomorphism());
        assert!(fo.index_stable_under_compact());
        assert!(fo.atkinson_description().contains("index 1"));
    }
    #[test]
    fn test_fredholm_isomorphism() {
        let fo = FredholmOperatorData::new("I", 0, 0);
        assert_eq!(fo.index(), 0);
        assert!(fo.is_isomorphism());
    }
    #[test]
    fn test_sobolev_space() {
        let ss = SobolevSpaceData::new(1, 2.0, 3, "R^3");
        let p_star = ss
            .critical_sobolev_exponent()
            .expect("critical_sobolev_exponent should succeed");
        assert!((p_star - 6.0).abs() < 1e-10, "p* should be 6, got {p_star}");
        assert!(ss.rellich_kondrachov_compact());
        assert!(ss.trace_theorem().contains("Trace"));
    }
    #[test]
    fn test_sobolev_supercritical() {
        let ss = SobolevSpaceData::new(2, 2.0, 2, "R^2");
        assert!(ss.critical_sobolev_exponent().is_none());
    }
    #[test]
    fn test_interpolation() {
        let id = InterpolationData::new("L^1", "L^inf", 0.5, InterpolationMethod::Complex);
        let exp = id.lp_exponent(1.0, 1e10);
        assert!((exp - 0.5).abs() < 1e-9, "1/p should be ~0.5");
        let bound = id.riesz_thorin_bound(1.0, 1.0);
        assert!((bound - 1.0).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_functional_analysis_ext2 {
    use super::*;
    #[test]
    fn test_weak_convergence() {
        let seq: Vec<Vec<f64>> = (1..=10).map(|n| vec![1.0 / n as f64, 0.0]).collect();
        let wcd = WeakConvergenceData::new(seq);
        assert!(wcd.is_bounded);
        assert!(wcd.banach_alaoglu_applies());
        assert!(wcd.check_weak_convergence(&[0.0, 0.0], 0.2));
    }
    #[test]
    fn test_distribution_dirac() {
        let delta = Distribution::dirac_delta(0.0);
        assert!(!delta.is_regular);
        assert_eq!(delta.order, Some(0));
        assert!(delta.is_tempered());
        let deriv = delta.differentiate();
        assert_eq!(deriv.order, Some(1));
    }
    #[test]
    fn test_distribution_regular() {
        let reg = Distribution::regular("f", "R").differentiate();
        assert!(!reg.is_regular);
        assert_eq!(reg.order, Some(1));
        assert!(reg
            .fourier_transform_description()
            .contains("distributional"));
    }
}
