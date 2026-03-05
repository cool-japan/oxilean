//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Noetherian ring properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NoetherianRing {
    pub name: String,
    pub krull_dimension: Option<usize>,
}
impl NoetherianRing {
    #[allow(dead_code)]
    pub fn new(name: &str, krull_dim: Option<usize>) -> Self {
        Self {
            name: name.to_string(),
            krull_dimension: krull_dim,
        }
    }
    #[allow(dead_code)]
    pub fn hilbert_basis_theorem(&self) -> String {
        format!(
            "If {} is Noetherian, then {}[x] is Noetherian",
            self.name, self.name
        )
    }
    #[allow(dead_code)]
    pub fn primary_decomposition_exists(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn krull_dimension_description(&self) -> String {
        match self.krull_dimension {
            Some(d) => {
                format!("dim({}) = {} (max length of chain of primes)", self.name, d)
            }
            None => format!("dim({}) = infinity", self.name),
        }
    }
}
/// Module over a ring.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Module {
    pub ring: String,
    pub name: String,
    pub is_free: bool,
    pub rank: Option<usize>,
    pub is_finitely_generated: bool,
}
impl Module {
    #[allow(dead_code)]
    pub fn free(ring: &str, name: &str, rank: usize) -> Self {
        Self {
            ring: ring.to_string(),
            name: name.to_string(),
            is_free: true,
            rank: Some(rank),
            is_finitely_generated: true,
        }
    }
    #[allow(dead_code)]
    pub fn cyclic(ring: &str, name: &str) -> Self {
        Self {
            ring: ring.to_string(),
            name: name.to_string(),
            is_free: false,
            rank: Some(1),
            is_finitely_generated: true,
        }
    }
    #[allow(dead_code)]
    pub fn over_pid_structure_theorem(&self) -> String {
        format!(
            "M ~ R^r + R/d1R + ... + R/dk*R (invariant factors) over PID {}",
            self.ring
        )
    }
    #[allow(dead_code)]
    pub fn is_projective(&self) -> bool {
        self.is_free
    }
    #[allow(dead_code)]
    pub fn is_flat(&self) -> bool {
        self.is_projective()
    }
}
/// Galois theory for field extensions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaloisExtension {
    pub base_field: String,
    pub extension_field: String,
    pub galois_group: String,
    pub degree: usize,
}
impl GaloisExtension {
    #[allow(dead_code)]
    pub fn new(base: &str, ext: &str, gal: &str, degree: usize) -> Self {
        Self {
            base_field: base.to_string(),
            extension_field: ext.to_string(),
            galois_group: gal.to_string(),
            degree,
        }
    }
    #[allow(dead_code)]
    pub fn cyclotomic(n: usize) -> Self {
        Self {
            base_field: "Q".to_string(),
            extension_field: format!("Q(zeta_{n})"),
            galois_group: format!("(Z/{}Z)*", n),
            degree: euler_phi_algebra(n as u64) as usize,
        }
    }
    #[allow(dead_code)]
    pub fn fundamental_theorem(&self) -> String {
        format!(
            "Subgroups of Gal({}/{}) <-> subfields between {} and {}",
            self.extension_field, self.base_field, self.base_field, self.extension_field
        )
    }
    #[allow(dead_code)]
    pub fn is_abelian_extension(&self) -> bool {
        self.galois_group.contains("Z") || self.galois_group.contains("abelian")
    }
}
/// Short exact sequence.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ShortExactSequence {
    pub a: String,
    pub b: String,
    pub c: String,
}
impl ShortExactSequence {
    #[allow(dead_code)]
    pub fn new(a: &str, b: &str, c: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            c: c.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!("0 -> {} -> {} -> {} -> 0", self.a, self.b, self.c)
    }
    #[allow(dead_code)]
    pub fn is_split(&self) -> bool {
        self.b.contains('+') || self.b.contains("oplus")
    }
}
/// Graded ring and algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GradedRing {
    pub name: String,
    pub grading_monoid: String,
    pub is_commutative: bool,
}
impl GradedRing {
    #[allow(dead_code)]
    pub fn polynomial_ring(vars: &[&str]) -> Self {
        Self {
            name: format!("k[{}]", vars.join(",")),
            grading_monoid: "N (by total degree)".to_string(),
            is_commutative: true,
        }
    }
    #[allow(dead_code)]
    pub fn exterior_algebra(n: usize) -> Self {
        Self {
            name: format!("Lambda^* (n={n})"),
            grading_monoid: "Z (by degree)".to_string(),
            is_commutative: false,
        }
    }
    #[allow(dead_code)]
    pub fn is_noetherian(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn hilbert_series_description(&self) -> String {
        format!("H({}, t) = sum dim(R_n) t^n for {}", self.name, self.name)
    }
}
/// Projective resolution.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectiveResolution {
    pub module: String,
    pub projective_dimension: Option<usize>,
    pub length: usize,
}
impl ProjectiveResolution {
    #[allow(dead_code)]
    pub fn new(module: &str, pd: Option<usize>, len: usize) -> Self {
        Self {
            module: module.to_string(),
            projective_dimension: pd,
            length: len,
        }
    }
    #[allow(dead_code)]
    pub fn is_finite(&self) -> bool {
        self.projective_dimension.is_some()
    }
    #[allow(dead_code)]
    pub fn global_dimension_description(&self) -> String {
        format!("gldim(R) = sup over M of pd(M) for all R-modules M")
    }
}
/// Derived category construction.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DerivedCategory {
    pub abelian_category: String,
}
impl DerivedCategory {
    #[allow(dead_code)]
    pub fn new(cat: &str) -> Self {
        Self {
            abelian_category: cat.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn objects_are_complexes(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn localization_description(&self) -> String {
        format!(
            "D({}) = K({})[quasi-isomorphisms^-1]",
            self.abelian_category, self.abelian_category
        )
    }
    #[allow(dead_code)]
    pub fn t_structure_heart(&self) -> String {
        format!(
            "Heart of standard t-structure on D({}) = {}",
            self.abelian_category, self.abelian_category
        )
    }
}
/// Abelian category axioms check.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbelianCategory {
    pub name: String,
    pub has_zero_object: bool,
    pub has_biproducts: bool,
    pub every_morphism_has_kernel_cokernel: bool,
}
impl AbelianCategory {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            has_zero_object: true,
            has_biproducts: true,
            every_morphism_has_kernel_cokernel: true,
        }
    }
    #[allow(dead_code)]
    pub fn of_modules(ring: &str) -> Self {
        Self::new(&format!("R-Mod (R={})", ring))
    }
    #[allow(dead_code)]
    pub fn snake_lemma(&self) -> String {
        "Snake lemma: long exact sequence from short exact sequences in an abelian category"
            .to_string()
    }
    #[allow(dead_code)]
    pub fn five_lemma(&self) -> String {
        "Five lemma: if 4 maps are isomorphisms, the middle one is too".to_string()
    }
}
/// Lie algebra structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LieAlgebra {
    pub name: String,
    pub dimension: usize,
    pub is_semisimple: bool,
    pub is_nilpotent: bool,
    pub is_solvable: bool,
}
impl LieAlgebra {
    #[allow(dead_code)]
    pub fn sl_n(n: usize) -> Self {
        Self {
            name: format!("sl({n})"),
            dimension: n * n - 1,
            is_semisimple: true,
            is_nilpotent: false,
            is_solvable: false,
        }
    }
    #[allow(dead_code)]
    pub fn heisenberg(n: usize) -> Self {
        Self {
            name: format!("h_{n} (Heisenberg)"),
            dimension: 2 * n + 1,
            is_semisimple: false,
            is_nilpotent: true,
            is_solvable: true,
        }
    }
    #[allow(dead_code)]
    pub fn abelian(n: usize) -> Self {
        Self {
            name: format!("k^{n} (abelian)"),
            dimension: n,
            is_semisimple: false,
            is_nilpotent: true,
            is_solvable: true,
        }
    }
    #[allow(dead_code)]
    pub fn engel_theorem(&self) -> String {
        "Engel's theorem: g nilpotent iff all adj(x) are nilpotent endomorphisms".to_string()
    }
    #[allow(dead_code)]
    pub fn lies_theorem(&self) -> String {
        "Lie's theorem: solvable Lie algebra acts by upper triangular matrices (over alg. closed field)"
            .to_string()
    }
    #[allow(dead_code)]
    pub fn cartan_criterion(&self) -> String {
        format!(
            "{} is semisimple iff its Killing form is non-degenerate",
            self.name
        )
    }
}
/// Ext and Tor groups.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExtTorGroups {
    pub ring: String,
    pub module_m: String,
    pub module_n: String,
}
impl ExtTorGroups {
    #[allow(dead_code)]
    pub fn new(ring: &str, m: &str, n: &str) -> Self {
        Self {
            ring: ring.to_string(),
            module_m: m.to_string(),
            module_n: n.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn ext_description(&self) -> String {
        format!(
            "Ext^n_{{{}}}({},{}) = derived functor of Hom, classifies n-fold extensions",
            self.ring, self.module_m, self.module_n
        )
    }
    #[allow(dead_code)]
    pub fn tor_description(&self) -> String {
        format!(
            "Tor^n_{{{}}}({},{}) = derived functor of tensor, measures flatness failure",
            self.ring, self.module_m, self.module_n
        )
    }
    #[allow(dead_code)]
    pub fn short_exact_sequence_long_ext(&self) -> String {
        "0 -> Hom(M,N) -> ... -> Ext^1(M,N) -> ... long exact sequence".to_string()
    }
}
/// Localization of a ring.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Localization {
    pub ring: String,
    pub multiplicative_set: String,
    pub localized_ring: String,
}
impl Localization {
    #[allow(dead_code)]
    pub fn at_prime(ring: &str, prime: &str) -> Self {
        Self {
            ring: ring.to_string(),
            multiplicative_set: format!("{ring} \\ {prime}"),
            localized_ring: format!("{ring}_{prime}"),
        }
    }
    #[allow(dead_code)]
    pub fn away_from(ring: &str, element: &str) -> Self {
        Self {
            ring: ring.to_string(),
            multiplicative_set: format!("powers of {element}"),
            localized_ring: format!("{ring}[1/{element}]"),
        }
    }
    #[allow(dead_code)]
    pub fn universal_property(&self) -> String {
        format!(
            "S^-1 {}: maps to rings where S is invertible factor uniquely through {}",
            self.ring, self.localized_ring
        )
    }
}
/// Tensor product of modules.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TensorProduct {
    pub module_a: String,
    pub module_b: String,
    pub ring: String,
}
impl TensorProduct {
    #[allow(dead_code)]
    pub fn new(a: &str, b: &str, ring: &str) -> Self {
        Self {
            module_a: a.to_string(),
            module_b: b.to_string(),
            ring: ring.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn right_exact_description(&self) -> String {
        format!(
            "- tensor_R {} is right exact (M' -> M -> M'' -> 0 stays exact) over {}",
            self.module_b, self.ring
        )
    }
    #[allow(dead_code)]
    pub fn adjunction_hom_tensor(&self) -> String {
        format!(
            "Hom_R(A tensor_R B, C) = Hom_R(A, Hom_R(B, C)) for {},{},{} over {}",
            self.module_a, self.module_b, "C", self.ring
        )
    }
}
