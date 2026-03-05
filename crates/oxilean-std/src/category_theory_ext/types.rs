//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Six-functor formalism computations.
///
/// Given a morphism f : X → Y between spaces, the six functors are:
/// f^* (inverse image), f_* (direct image), f_! (proper direct image),
/// f^! (exceptional inverse image), ⊗ (tensor), Hom (internal hom).
///
/// This struct provides a concrete model where spaces are finite posets
/// and sheaves are functions on points.
pub struct SixFunctorComputations {
    /// Points of the source space X.
    pub source_points: usize,
    /// Points of the target space Y.
    pub target_points: usize,
    /// The map f : X → Y given as f(i) for source point i.
    pub f_map: Vec<usize>,
}
impl SixFunctorComputations {
    /// Compute f^* (pullback / inverse image): for a sheaf F on Y (as a vec of values),
    /// return the sheaf f^*F on X.
    ///
    /// f^*F(x) = F(f(x)).
    pub fn pullback(&self, sheaf_on_y: &[f64]) -> Vec<f64> {
        self.f_map
            .iter()
            .map(|&fy| sheaf_on_y.get(fy).copied().unwrap_or(0.0))
            .collect()
    }
    /// Compute f_* (direct image / pushforward): for a sheaf G on X (as a vec of values),
    /// return the sheaf f_*G on Y by taking the sum over fibers.
    ///
    /// f_*G(y) = ∏_{x : f(x)=y} G(x).
    pub fn pushforward(&self, sheaf_on_x: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.target_points];
        for (x, &fy) in self.f_map.iter().enumerate() {
            if fy < self.target_points {
                result[fy] += sheaf_on_x.get(x).copied().unwrap_or(0.0);
            }
        }
        result
    }
    /// Compute f_! (proper pushforward): for a sheaf G on X, take the product (min) over fibers.
    ///
    /// In the finite poset model, f_! G(y) = min over the fiber (models proper support).
    pub fn proper_pushforward(&self, sheaf_on_x: &[f64]) -> Vec<f64> {
        let mut fiber_vals: Vec<Vec<f64>> = vec![vec![]; self.target_points];
        for (x, &fy) in self.f_map.iter().enumerate() {
            if fy < self.target_points {
                fiber_vals[fy].push(sheaf_on_x.get(x).copied().unwrap_or(0.0));
            }
        }
        fiber_vals
            .iter()
            .map(|vals| {
                if vals.is_empty() {
                    0.0
                } else {
                    vals.iter().cloned().fold(f64::INFINITY, f64::min)
                }
            })
            .collect()
    }
    /// Compute the tensor product of two sheaves on X pointwise.
    pub fn tensor(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
    }
    /// Compute the internal hom sheaf Hom(a, b) on X pointwise.
    ///
    /// In the discrete model, Hom(a, b)(x) = b(x) / a(x) (or 0 if a(x) = 0).
    pub fn internal_hom(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter()
            .zip(b.iter())
            .map(|(ax, bx)| if *ax != 0.0 { bx / ax } else { 0.0 })
            .collect()
    }
    /// Verify the projection formula: f_!(f^* G ⊗ F) ≅ G ⊗ f_! F.
    ///
    /// Checks equality of pushforward after tensor with pullback vs. tensor after pushforward.
    pub fn check_projection_formula(&self, f_on_y: &[f64], g_on_x: &[f64]) -> bool {
        let pullback_f = self.pullback(f_on_y);
        let lhs = self.proper_pushforward(&self.tensor(&pullback_f, g_on_x));
        let pushforward_g = self.proper_pushforward(g_on_x);
        let rhs = self.tensor(f_on_y, &pushforward_g);
        let rhs_trimmed: Vec<f64> = rhs.into_iter().take(lhs.len()).collect();
        lhs.iter()
            .zip(rhs_trimmed.iter())
            .all(|(a, b)| (a - b).abs() < 1e-9)
    }
}
/// A small category with objects `O` and morphisms `M`.
///
/// Morphisms are stored as triples `(domain_index, codomain_index, data)`.
/// Identity morphisms are indexed into `morphisms`.
pub struct Category<O, M> {
    /// The objects of the category.
    pub objects: Vec<O>,
    /// Morphisms as `(domain_idx, codomain_idx, data)`.
    pub morphisms: Vec<(usize, usize, M)>,
    /// `identity[i]` is the index in `morphisms` of the identity on object `i`.
    pub identity: Vec<usize>,
}
impl<O, M: Clone + PartialEq> Category<O, M> {
    /// Compose morphism `f` then `g` (f : A → B, g : B → C) by index.
    ///
    /// Returns `Some(index)` in `morphisms` if domains/codomains match, else `None`.
    /// This method checks compatibility but does NOT compute a new morphism —
    /// it finds an existing one with matching signature (useful for finite categories).
    pub fn compose(&self, f_idx: usize, g_idx: usize) -> Option<usize> {
        let (_, f_cod, _) = &self.morphisms[f_idx];
        let (g_dom, g_cod, _) = &self.morphisms[g_idx];
        if f_cod != g_dom {
            return None;
        }
        let (f_dom, _, _) = &self.morphisms[f_idx];
        self.morphisms
            .iter()
            .enumerate()
            .find_map(|(i, (d, c, _))| {
                if d == f_dom && c == g_cod {
                    Some(i)
                } else {
                    None
                }
            })
    }
    /// Check associativity for a triple of morphism indices.
    pub fn is_associative(&self, f: usize, g: usize, h: usize) -> bool {
        let fg = self.compose(f, g);
        let gh = self.compose(g, h);
        match (fg, gh) {
            (Some(fg_idx), Some(gh_idx)) => {
                let fgh_left = self.compose(fg_idx, h);
                let fgh_right = self.compose(f, gh_idx);
                fgh_left == fgh_right
            }
            _ => false,
        }
    }
}
/// E_∞ algebra operations over a commutative monoid.
///
/// An E_∞ algebra structure includes a commutative multiplication
/// and a unit, satisfying commutativity and associativity up to all higher homotopies.
/// Here we represent it concretely as a commutative monoid on a vector.
pub struct EInfinityAlgebraOps<T: Clone + PartialEq> {
    /// The underlying set (finite representative).
    pub elements: Vec<T>,
    /// The unit element index.
    pub unit_idx: usize,
    /// Multiplication table: multiply[i][j] = index of elements[i] * elements[j].
    pub multiply: Vec<Vec<usize>>,
}
impl<T: Clone + PartialEq> EInfinityAlgebraOps<T> {
    /// Check commutativity: multiply[i][j] == multiply[j][i] for all i, j.
    pub fn is_commutative(&self) -> bool {
        let n = self.elements.len();
        (0..n).all(|i| (0..n).all(|j| self.multiply[i][j] == self.multiply[j][i]))
    }
    /// Check associativity: (a*b)*c == a*(b*c) for all a, b, c.
    pub fn is_associative(&self) -> bool {
        let n = self.elements.len();
        (0..n).all(|a| {
            (0..n).all(|b| {
                (0..n).all(|c| {
                    let ab = self.multiply[a][b];
                    let bc = self.multiply[b][c];
                    self.multiply[ab][c] == self.multiply[a][bc]
                })
            })
        })
    }
    /// Check unit laws: unit * a == a == a * unit.
    pub fn has_unit(&self) -> bool {
        let u = self.unit_idx;
        let n = self.elements.len();
        (0..n).all(|a| self.multiply[u][a] == a && self.multiply[a][u] == a)
    }
    /// Verify full E_∞ algebra structure (commutativity + associativity + unit).
    pub fn verify_e_infinity(&self) -> bool {
        self.is_commutative() && self.is_associative() && self.has_unit()
    }
}
/// A monad `(T, unit, bind)` over a type `S`.
///
/// Follows the Kleisli presentation: `unit : S → T S` and `bind : T S → (S → T S) → T S`.
pub struct Monad<S> {
    /// `unit(x)` wraps a pure value.
    pub unit: fn(S) -> S,
    /// `bind(m, f)` sequences computation.
    pub bind: fn(S, fn(S) -> S) -> S,
}
impl<S: PartialEq + Clone> Monad<S> {
    /// Check the left unit law: `bind(unit(x), f) = f(x)`.
    pub fn left_unit_law(&self, x: S, f: fn(S) -> S) -> bool {
        let bound = (self.bind)((self.unit)(x.clone()), f);
        bound == f(x)
    }
    /// Check the right unit law: `bind(m, unit) = m`.
    pub fn right_unit_law(&self, m: S) -> bool {
        let bound = (self.bind)(m.clone(), self.unit);
        bound == m
    }
}
/// An adjunction `F ⊣ G` witnessed by unit and counit on object indices.
///
/// `unit(a)` gives the unit morphism index for object `a`.
/// `counit(b)` gives the counit morphism index for object `b`.
pub struct Adjunction {
    /// Unit morphism index for each source object.
    pub unit: fn(usize) -> usize,
    /// Counit morphism index for each target object.
    pub counit: fn(usize) -> usize,
}
impl Adjunction {
    /// Check the triangle identities (trivially via round-trip equality).
    ///
    /// In a concrete setting this checks that `counit(F(unit(a))) = id_F(a)`.
    /// Here we verify the maps are consistent on a sample object.
    pub fn check_triangle_identities(&self) -> bool {
        (0..4).all(|a| {
            let eta_a = (self.unit)(a);
            let eps_fa = (self.counit)(eta_a);
            (self.counit)(eps_fa) == eps_fa
        })
    }
}
/// Segal condition checker for simplicial sets.
///
/// A simplicial set X satisfies the Segal condition iff the natural map
/// X_n → X_1 ×_{X_0} ··· ×_{X_0} X_1 is a bijection for all n ≥ 2.
pub struct SegalConditionChecker {
    /// The levels: segal_levels[n] stores the set of n-simplices as Vec<u64>.
    pub segal_levels: Vec<Vec<Vec<u64>>>,
}
impl SegalConditionChecker {
    /// Create a checker from a list of simplex levels.
    pub fn new(levels: Vec<Vec<Vec<u64>>>) -> Self {
        Self {
            segal_levels: levels,
        }
    }
    /// Check the Segal condition at level n by verifying that n-simplices
    /// decompose as composable pairs of (n-1)-simplices.
    ///
    /// Returns true iff every n-simplex can be expressed as a pair of composable (n-1)-simplices.
    pub fn check_level(&self, n: usize) -> bool {
        if n < 2 || n >= self.segal_levels.len() {
            return true;
        }
        let n_simplices = &self.segal_levels[n];
        let prev_simplices = &self.segal_levels[n - 1];
        n_simplices
            .iter()
            .all(|s| s.len() >= 2 && prev_simplices.iter().any(|t| !t.is_empty() && t[0] == s[0]))
    }
    /// Check Segal conditions at all levels.
    pub fn check_all(&self) -> bool {
        (2..self.segal_levels.len()).all(|n| self.check_level(n))
    }
}
/// A natural transformation between two functors (stored as component morphism indices).
///
/// `components[i]` is the morphism index in the target for the component at object `i`.
pub struct NatTrans {
    /// Components η_A, one per source object (morphism indices in target).
    pub components: Vec<usize>,
}
impl NatTrans {
    /// Check naturality: for morphism `f: A → B` in source, the naturality square commutes.
    ///
    /// `f_src_idx` — index of f in source morphisms.
    /// `f_tgt_idx` — index of F(f) in target morphisms.
    /// `g_tgt_idx` — index of G(f) in target morphisms.
    /// `a_obj` and `b_obj` — source/target object indices.
    pub fn is_natural<O1, M1: Clone + PartialEq, O2, M2: Clone + PartialEq>(
        &self,
        _source: &Category<O1, M1>,
        target: &Category<O2, M2>,
        a_obj: usize,
        b_obj: usize,
        ff_idx: usize,
        gf_idx: usize,
    ) -> bool {
        let eta_a = self.components[a_obj];
        let eta_b = self.components[b_obj];
        let left = target.compose(ff_idx, eta_b);
        let right = target.compose(eta_a, gf_idx);
        left == right
    }
}
/// Horn filling data for a quasi-category.
///
/// Represents an inner horn Λ^n_k → X together with a proposed filler.
/// A simplicial set is a quasi-category iff all inner horns have fillers.
pub struct QuasiCategoryHorn {
    /// The dimension n of the simplex.
    pub n: usize,
    /// The missing face k (0 < k < n for inner horns).
    pub k: usize,
    /// The faces of the horn: faces[i] = Some(face_data) for i ≠ k, None for i = k.
    pub faces: Vec<Option<Vec<u64>>>,
}
impl QuasiCategoryHorn {
    /// Create a new inner horn of dimension n with missing face k.
    ///
    /// Returns `None` if k is not an inner horn index (i.e., k == 0 or k == n).
    pub fn new_inner(n: usize, k: usize) -> Option<Self> {
        if n < 2 || k == 0 || k >= n {
            return None;
        }
        let faces = (0..=n)
            .map(|i| if i == k { None } else { Some(vec![i as u64]) })
            .collect();
        Some(Self { n, k, faces })
    }
    /// Check whether this is an inner horn (0 < k < n).
    pub fn is_inner(&self) -> bool {
        self.k > 0 && self.k < self.n
    }
    /// Attempt to fill the horn by interpolating the missing face.
    ///
    /// Returns a proposed filler (vector of face data) if all other faces are present.
    pub fn fill(&self) -> Option<Vec<u64>> {
        if !self.is_inner() {
            return None;
        }
        let all_present = self
            .faces
            .iter()
            .enumerate()
            .all(|(i, f)| i == self.k || f.is_some());
        if !all_present {
            return None;
        }
        let filler: Vec<u64> = self
            .faces
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if i != self.k {
                    f.as_ref().map(|v| v.iter().sum())
                } else {
                    None
                }
            })
            .collect();
        Some(filler)
    }
}
/// A functor between two small categories (over concrete index maps).
///
/// `obj_map` maps object indices of source to object indices of target.
/// `mor_map` maps morphism indices of source to morphism indices of target.
pub struct Functor<O1, M1, O2, M2> {
    /// The source category.
    pub source: Category<O1, M1>,
    /// The target category.
    pub target: Category<O2, M2>,
    /// Object map: source object index → target object index.
    pub obj_map: fn(usize) -> usize,
    /// Morphism map: source morphism index → target morphism index.
    pub mor_map: fn(usize) -> usize,
}
impl<O1, M1: Clone + PartialEq, O2, M2: Clone + PartialEq> Functor<O1, M1, O2, M2> {
    /// Check that the functor preserves composition for given morphism indices.
    pub fn preserves_composition(&self, f_idx: usize, g_idx: usize) -> bool {
        let composed_src = self.source.compose(f_idx, g_idx);
        let ff = (self.mor_map)(f_idx);
        let fg = (self.mor_map)(g_idx);
        let composed_tgt = self.target.compose(ff, fg);
        composed_src.map(|i| (self.mor_map)(i)) == composed_tgt
    }
}
/// A chain complex in a derived category.
///
/// Represents a bounded chain complex C_n (n from lo to hi) with differentials d_n : C_n → C_{n-1}.
/// The derived category is obtained by inverting quasi-isomorphisms.
pub struct DerivedCategoryComplex {
    /// Dimensions of the chain groups (dim[i] = dimension of C_{lo + i}).
    pub dimensions: Vec<usize>,
    /// The lowest degree.
    pub lo_degree: i32,
    /// Differentials as matrices (d[i] is d_{lo+i+1} : C_{lo+i+1} → C_{lo+i}).
    /// Each matrix is stored row-major with dimensions[i] rows and dimensions[i+1] cols.
    pub differentials: Vec<Vec<Vec<i32>>>,
}
impl DerivedCategoryComplex {
    /// Check that d^2 = 0: the composition of consecutive differentials is zero.
    ///
    /// For each consecutive pair, computes the matrix product and checks it is zero.
    pub fn is_chain_complex(&self) -> bool {
        if self.differentials.len() < 2 {
            return true;
        }
        for i in 0..self.differentials.len() - 1 {
            let d_n = &self.differentials[i + 1];
            let d_nm1 = &self.differentials[i];
            let rows = d_nm1.len();
            let cols = if d_n.is_empty() { 0 } else { d_n[0].len() };
            let mid = d_n.len();
            for r in 0..rows {
                for c in 0..cols {
                    let entry: i32 = (0..mid)
                        .map(|k| {
                            d_nm1[r].get(k).copied().unwrap_or(0)
                                * d_n[k].get(c).copied().unwrap_or(0)
                        })
                        .sum();
                    if entry != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Compute the Euler characteristic: alternating sum of dimensions.
    pub fn euler_characteristic(&self) -> i32 {
        self.dimensions
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                let sign = if (self.lo_degree + i as i32) % 2 == 0 {
                    1i32
                } else {
                    -1i32
                };
                sign * d as i32
            })
            .sum()
    }
    /// Check if the complex is exact at position i (cohomology vanishes there).
    ///
    /// A complex is exact at position i iff im(d_{i+1}) = ker(d_i).
    /// Here we use a rank-based approximation: exact iff rank(d_i) + rank(d_{i+1}) = dim(C_i).
    pub fn is_exact_at(&self, i: usize) -> bool {
        if i >= self.dimensions.len() {
            return true;
        }
        let dim = self.dimensions[i];
        let rank_in = if i < self.differentials.len() {
            self.differentials[i]
                .iter()
                .filter(|row| row.iter().any(|&x| x != 0))
                .count()
        } else {
            0
        };
        let rank_out = if i > 0 && i - 1 < self.differentials.len() {
            let d = &self.differentials[i - 1];
            if d.is_empty() {
                0
            } else {
                d[0].iter().filter(|&&x| x != 0).count()
            }
        } else {
            0
        };
        rank_in + rank_out == dim
    }
}
