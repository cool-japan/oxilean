//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A unified fuzzy inference system supporting both Mamdani and Sugeno styles.
pub struct FuzzyInferenceSystem {
    /// System type.
    pub fis_type: FISType,
    /// Number of input variables.
    pub n_inputs: usize,
    /// Universe size for output (used in Mamdani).
    pub output_size: usize,
    /// Domain for Mamdani defuzzification.
    pub output_domain: Vec<f64>,
    /// Defuzzification method (for Mamdani).
    pub defuzz_method: DefuzzMethod,
    /// Mamdani rules (populated if Mamdani).
    pub mamdani_rules: Vec<MamdaniRule>,
    /// Sugeno rules (populated if Sugeno).
    pub sugeno_rules: Vec<SugenoRule>,
}
impl FuzzyInferenceSystem {
    /// Create a Mamdani FIS.
    pub fn mamdani(output_size: usize, output_domain: Vec<f64>) -> Self {
        FuzzyInferenceSystem {
            fis_type: FISType::Mamdani,
            n_inputs: 0,
            output_size,
            output_domain,
            defuzz_method: DefuzzMethod::CentroidOfArea,
            mamdani_rules: Vec::new(),
            sugeno_rules: Vec::new(),
        }
    }
    /// Create a Sugeno FIS.
    pub fn sugeno(n_inputs: usize) -> Self {
        FuzzyInferenceSystem {
            fis_type: FISType::Sugeno,
            n_inputs,
            output_size: 0,
            output_domain: Vec::new(),
            defuzz_method: DefuzzMethod::CentroidOfArea,
            mamdani_rules: Vec::new(),
            sugeno_rules: Vec::new(),
        }
    }
    /// Set the defuzzification method (Mamdani only).
    pub fn with_defuzz(mut self, method: DefuzzMethod) -> Self {
        self.defuzz_method = method;
        self
    }
    /// Add a Mamdani rule.
    pub fn add_mamdani_rule(&mut self, antecedent_mf: Vec<f64>, consequent: FuzzySet) {
        self.mamdani_rules.push(MamdaniRule {
            antecedent_mf,
            consequent,
        });
    }
    /// Add a Sugeno rule.
    pub fn add_sugeno_rule(
        &mut self,
        antecedent_mf: Vec<f64>,
        output_coeffs: Vec<f64>,
        output_const: f64,
    ) {
        self.sugeno_rules.push(SugenoRule {
            antecedent_mf,
            output_coeffs,
            output_const,
        });
    }
    /// Run Mamdani inference and defuzzify.
    ///
    /// `input_degrees[i]` contains the firing degrees for input variable i.
    pub fn infer_mamdani(&self, input_degrees: &[Vec<f64>]) -> f64 {
        let sys = MamdaniSystem {
            rules: self.mamdani_rules.clone(),
            output_size: self.output_size,
        };
        let out_fuzzy = sys.infer(input_degrees);
        defuzzify(&out_fuzzy, &self.output_domain, self.defuzz_method)
    }
    /// Run Sugeno inference and return crisp output.
    pub fn infer_sugeno(&self, inputs: &[f64], input_degrees: &[Vec<f64>]) -> f64 {
        let sys = SugenoSystem {
            rules: self.sugeno_rules.clone(),
        };
        sys.infer(inputs, input_degrees)
    }
    /// Returns `true` if the FIS has at least one rule.
    pub fn is_configured(&self) -> bool {
        match self.fis_type {
            FISType::Mamdani => !self.mamdani_rules.is_empty(),
            FISType::Sugeno => !self.sugeno_rules.is_empty(),
        }
    }
}
/// Standard t-conorm variants (dual to t-norms via De Morgan).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TConorm {
    Maximum,
    ProbabilisticSum,
    BoundedSum,
    Drastic,
}
impl TConorm {
    /// Evaluate the t-conorm at (a, b) ∈ [0,1]².
    pub fn eval(self, a: f64, b: f64) -> f64 {
        match self {
            TConorm::Maximum => a.max(b),
            TConorm::ProbabilisticSum => a + b - a * b,
            TConorm::BoundedSum => (a + b).min(1.0),
            TConorm::Drastic => {
                if a < 1e-9 {
                    b
                } else if b < 1e-9 {
                    a
                } else {
                    1.0
                }
            }
        }
    }
}
/// Fuzzy clustering (c-means membership matrix).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuzzyClustering {
    pub n_samples: usize,
    pub n_clusters: usize,
    pub fuzziness: f64,
    pub membership: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl FuzzyClustering {
    pub fn new(n_samples: usize, n_clusters: usize, fuzziness: f64) -> Self {
        let membership = vec![vec![1.0 / n_clusters as f64; n_samples]; n_clusters];
        FuzzyClustering {
            n_samples,
            n_clusters,
            fuzziness,
            membership,
        }
    }
    pub fn get_membership(&self, cluster: usize, sample: usize) -> f64 {
        self.membership[cluster][sample]
    }
    pub fn hard_assignment(&self, sample: usize) -> usize {
        (0..self.n_clusters)
            .max_by(|&a, &b| {
                self.membership[a][sample]
                    .partial_cmp(&self.membership[b][sample])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Partition coefficient: measures crispness (1.0 = crisp, 1/c = max fuzz).
    pub fn partition_coefficient(&self) -> f64 {
        let mut sum = 0.0;
        for c in 0..self.n_clusters {
            for s in 0..self.n_samples {
                sum += self.membership[c][s].powi(2);
            }
        }
        sum / self.n_samples as f64
    }
    pub fn set_membership(&mut self, cluster: usize, sample: usize, val: f64) {
        self.membership[cluster][sample] = val.clamp(0.0, 1.0);
    }
}
/// Fuzzy C-Means (FCM) clustering algorithm (Bezdek 1981).
///
/// Minimises: J_m = Σ_i Σ_k (u_{ik})^m · d(x_i, c_k)^2
/// subject to Σ_k u_{ik} = 1 for each data point i.
pub struct FuzzyCMeans {
    /// Number of clusters.
    pub c: usize,
    /// Fuzziness parameter m > 1 (m=2 is most common).
    pub m: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance for cluster center movement.
    pub tol: f64,
}
impl FuzzyCMeans {
    /// Create with default parameters (m=2, max_iter=100, tol=1e-6).
    pub fn new(c: usize) -> Self {
        FuzzyCMeans {
            c,
            m: 2.0,
            max_iter: 100,
            tol: 1e-6,
        }
    }
    /// Set fuzziness parameter.
    pub fn with_m(mut self, m: f64) -> Self {
        self.m = m;
        self
    }
    /// Set maximum iterations.
    pub fn with_max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }
    /// Euclidean distance between two data points.
    fn dist(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    /// Run FCM on `data` (each row is a data point).
    ///
    /// Returns `(membership, centers)`:
    /// - `membership[i][k]` = degree of point i belonging to cluster k.
    /// - `centers[k]` = center of cluster k.
    pub fn fit(&self, data: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = data.len();
        let dim = if n > 0 { data[0].len() } else { 1 };
        if n == 0 || self.c == 0 {
            return (Vec::new(), Vec::new());
        }
        let mut u: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let mut row: Vec<f64> = (0..self.c)
                    .map(|k| {
                        let base = 1.0 / self.c as f64;
                        let delta = 0.1 * ((i + k) % 3) as f64 / 3.0 - 0.05;
                        (base + delta).clamp(0.01, 0.99)
                    })
                    .collect();
                let s: f64 = row.iter().sum();
                for v in &mut row {
                    *v /= s;
                }
                row
            })
            .collect();
        let mut centers: Vec<Vec<f64>> = vec![vec![0.0; dim]; self.c];
        for _iter in 0..self.max_iter {
            let mut old_centers = centers.clone();
            for k in 0..self.c {
                let mut num = vec![0.0_f64; dim];
                let mut denom = 0.0_f64;
                for i in 0..n {
                    let w = u[i][k].powf(self.m);
                    denom += w;
                    for d in 0..dim {
                        num[d] += w * data[i][d];
                    }
                }
                if denom.abs() > 1e-15 {
                    for d in 0..dim {
                        centers[k][d] = num[d] / denom;
                    }
                }
            }
            for i in 0..n {
                let dists: Vec<f64> = (0..self.c)
                    .map(|k| Self::dist(&data[i], &centers[k]).max(1e-15))
                    .collect();
                let zero_k: Vec<usize> = (0..self.c).filter(|&k| dists[k] < 1e-12).collect();
                if !zero_k.is_empty() {
                    for k in 0..self.c {
                        u[i][k] = 0.0;
                    }
                    let share = 1.0 / zero_k.len() as f64;
                    for &k in &zero_k {
                        u[i][k] = share;
                    }
                } else {
                    let exp = 2.0 / (self.m - 1.0);
                    for k in 0..self.c {
                        let sum: f64 = (0..self.c).map(|j| (dists[k] / dists[j]).powf(exp)).sum();
                        u[i][k] = 1.0 / sum;
                    }
                }
            }
            let movement: f64 = old_centers
                .iter_mut()
                .zip(centers.iter())
                .map(|(oc, nc)| Self::dist(oc, nc))
                .sum();
            if movement < self.tol {
                break;
            }
        }
        (u, centers)
    }
    /// Compute the partition coefficient V_PC = (1/N) Σ Σ u_{ik}^2 ∈ [1/c, 1].
    ///
    /// V_PC = 1 means hard partition, 1/c means maximum fuzziness.
    pub fn partition_coefficient(membership: &[Vec<f64>]) -> f64 {
        let n = membership.len();
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = membership
            .iter()
            .flat_map(|row| row.iter())
            .map(|&u| u * u)
            .sum();
        sum / n as f64
    }
    /// Compute the classification entropy V_CE = −(1/N) Σ Σ u_{ik} log(u_{ik}).
    pub fn classification_entropy(membership: &[Vec<f64>]) -> f64 {
        let n = membership.len();
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = membership
            .iter()
            .flat_map(|row| row.iter())
            .map(|&u| if u > 1e-15 { -u * u.ln() } else { 0.0 })
            .sum();
        sum / n as f64
    }
}
/// Fuzzy inference engine (Mamdani-type).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MamdaniEngine {
    pub input_names: Vec<String>,
    pub output_name: String,
    pub n_rules: usize,
    pub defuzz_method: DefuzzMethod,
}
#[allow(dead_code)]
impl MamdaniEngine {
    pub fn new(inputs: Vec<&str>, output: &str, n_rules: usize) -> Self {
        MamdaniEngine {
            input_names: inputs.iter().map(|s| s.to_string()).collect(),
            output_name: output.to_string(),
            n_rules,
            defuzz_method: DefuzzMethod::CentroidOfArea,
        }
    }
    pub fn set_defuzz(&mut self, method: DefuzzMethod) {
        self.defuzz_method = method;
    }
    pub fn n_inputs(&self) -> usize {
        self.input_names.len()
    }
    /// Centroid defuzzification over uniformly-sampled output range.
    pub fn centroid_defuzz(values: &[f64], memberships: &[f64]) -> f64 {
        let num: f64 = values
            .iter()
            .zip(memberships.iter())
            .map(|(v, m)| v * m)
            .sum();
        let den: f64 = memberships.iter().sum();
        if den.abs() < 1e-12 {
            0.0
        } else {
            num / den
        }
    }
}
/// A Sugeno rule: antecedent + linear output function.
#[derive(Debug, Clone)]
pub struct SugenoRule {
    /// Input membership degrees for the antecedent.
    pub antecedent_mf: Vec<f64>,
    /// Coefficients for the linear output: z = c0 + c1*x1 + c2*x2 + ...
    pub output_coeffs: Vec<f64>,
    pub output_const: f64,
}
/// Many-valued logic variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManyValuedLogic {
    Lukasiewicz,
    Godel,
    Product,
}
impl ManyValuedLogic {
    /// Conjunction (t-norm) for the logic.
    pub fn conj(self, a: f64, b: f64) -> f64 {
        match self {
            ManyValuedLogic::Lukasiewicz => (a + b - 1.0).max(0.0),
            ManyValuedLogic::Godel => a.min(b),
            ManyValuedLogic::Product => a * b,
        }
    }
    /// Disjunction (t-conorm) for the logic.
    pub fn disj(self, a: f64, b: f64) -> f64 {
        match self {
            ManyValuedLogic::Lukasiewicz => (a + b).min(1.0),
            ManyValuedLogic::Godel => a.max(b),
            ManyValuedLogic::Product => a + b - a * b,
        }
    }
    /// Residuum (implication a → b).
    pub fn residuum(self, a: f64, b: f64) -> f64 {
        match self {
            ManyValuedLogic::Lukasiewicz => (1.0 - a + b).min(1.0),
            ManyValuedLogic::Godel => {
                if a <= b {
                    1.0
                } else {
                    b
                }
            }
            ManyValuedLogic::Product => {
                if a <= b {
                    1.0
                } else {
                    b / a
                }
            }
        }
    }
    /// Negation: ¬a = a → 0.
    pub fn neg(self, a: f64) -> f64 {
        self.residuum(a, 0.0)
    }
    /// Biconditional: a ↔ b = (a → b) ∧ (b → a).
    pub fn iff(self, a: f64, b: f64) -> f64 {
        self.conj(self.residuum(a, b), self.residuum(b, a))
    }
}
/// A finite MTL (Monoidal T-norm Logic) algebra over {0, ..., n-1}.
/// The order is the integer order; 0 = bottom, n-1 = top.
#[derive(Debug, Clone)]
pub struct FiniteMTLAlgebra {
    pub size: usize,
    /// t-norm table: tnorm[i][j]
    pub tnorm: Vec<Vec<usize>>,
    /// residuum table: residuum[i][j] = max { k | tnorm[k][i] ≤ j }
    pub residuum: Vec<Vec<usize>>,
}
impl FiniteMTLAlgebra {
    /// Build an MTL algebra from a t-norm table (as usize indices).
    pub fn from_tnorm(size: usize, tnorm: Vec<Vec<usize>>) -> Self {
        let mut residuum = vec![vec![0usize; size]; size];
        for a in 0..size {
            for b in 0..size {
                let mut best = 0usize;
                for k in 0..size {
                    if tnorm[k][a] <= b {
                        best = best.max(k);
                    }
                }
                residuum[a][b] = best;
            }
        }
        FiniteMTLAlgebra {
            size,
            tnorm,
            residuum,
        }
    }
    /// Evaluate the t-norm.
    pub fn t(&self, a: usize, b: usize) -> usize {
        self.tnorm[a][b]
    }
    /// Evaluate the residuum (implication).
    pub fn r(&self, a: usize, b: usize) -> usize {
        self.residuum[a][b]
    }
    /// Negation: ¬a = a → 0.
    pub fn neg(&self, a: usize) -> usize {
        self.r(a, 0)
    }
    /// Check if the algebra satisfies prelinearity: (a → b) ∨ (b → a) = 1.
    pub fn satisfies_prelinearity(&self) -> bool {
        let top = self.size - 1;
        for a in 0..self.size {
            for b in 0..self.size {
                let imp_ab = self.r(a, b);
                let imp_ba = self.r(b, a);
                let join = imp_ab.max(imp_ba);
                if join != top {
                    return false;
                }
            }
        }
        true
    }
    /// Check if it is a BL algebra: divisibility holds (a ∧ b = a * (a → b)).
    pub fn satisfies_divisibility(&self) -> bool {
        for a in 0..self.size {
            for b in 0..self.size {
                let meet = a.min(b);
                let product = self.t(a, self.r(a, b));
                if meet != product {
                    return false;
                }
            }
        }
        true
    }
}
/// A fuzzy rule for a Mamdani system: IF antecedent THEN consequent.
#[derive(Debug, Clone)]
pub struct MamdaniRule {
    /// Antecedent membership degrees for each input variable's linguistic value.
    pub antecedent_mf: Vec<f64>,
    /// Consequent fuzzy set (output).
    pub consequent: FuzzySet,
}
/// System type: Mamdani (fuzzy output) or Sugeno (crisp linear output).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FISType {
    Mamdani,
    Sugeno,
}
/// Defuzzification strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefuzzMethod {
    CentroidOfArea,
    BisectorOfArea,
    MeanOfMaxima,
    SmallestOfMaxima,
    LargestOfMaxima,
}
/// Standard t-norm variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TNorm {
    Minimum,
    Product,
    Lukasiewicz,
    Drastic,
}
impl TNorm {
    /// Evaluate the t-norm at (a, b) ∈ [0,1]².
    pub fn eval(self, a: f64, b: f64) -> f64 {
        match self {
            TNorm::Minimum => a.min(b),
            TNorm::Product => a * b,
            TNorm::Lukasiewicz => (a + b - 1.0).max(0.0),
            TNorm::Drastic => {
                if (a - 1.0).abs() < 1e-9 {
                    b
                } else if (b - 1.0).abs() < 1e-9 {
                    a
                } else {
                    0.0
                }
            }
        }
    }
    /// Check the commutativity property at sample points.
    pub fn is_commutative_sample(&self, a: f64, b: f64) -> bool {
        (self.eval(a, b) - self.eval(b, a)).abs() < 1e-9
    }
    /// Check associativity at sample points.
    pub fn is_associative_sample(&self, a: f64, b: f64, c: f64) -> bool {
        (self.eval(self.eval(a, b), c) - self.eval(a, self.eval(b, c))).abs() < 1e-9
    }
}
/// A fuzzy set over a universe represented as a list of (element, degree) pairs.
/// Degrees are in [0, 1].
#[derive(Debug, Clone)]
pub struct FuzzySet {
    pub universe_size: usize,
    /// membership[i] ∈ [0.0, 1.0]
    pub membership: Vec<f64>,
}
impl FuzzySet {
    /// Create a crisp empty fuzzy set over a universe of the given size.
    pub fn new(universe_size: usize) -> Self {
        FuzzySet {
            universe_size,
            membership: vec![0.0; universe_size],
        }
    }
    /// Set the membership degree for element i (clamped to [0,1]).
    pub fn set(&mut self, i: usize, degree: f64) {
        self.membership[i] = degree.clamp(0.0, 1.0);
    }
    /// Get the membership degree for element i.
    pub fn get(&self, i: usize) -> f64 {
        self.membership[i]
    }
    /// α-cut: returns a crisp set of elements with membership ≥ α.
    pub fn alpha_cut(&self, alpha: f64) -> Vec<usize> {
        self.membership
            .iter()
            .enumerate()
            .filter(|(_, &d)| d >= alpha)
            .map(|(i, _)| i)
            .collect()
    }
    /// Strong α-cut: elements with membership > α.
    pub fn strong_alpha_cut(&self, alpha: f64) -> Vec<usize> {
        self.membership
            .iter()
            .enumerate()
            .filter(|(_, &d)| d > alpha)
            .map(|(i, _)| i)
            .collect()
    }
    /// Fuzzy complement using standard negation: 1 − μ(x).
    pub fn complement(&self) -> FuzzySet {
        let membership = self.membership.iter().map(|&d| 1.0 - d).collect();
        FuzzySet {
            universe_size: self.universe_size,
            membership,
        }
    }
    /// Height of the fuzzy set: max membership degree.
    pub fn height(&self) -> f64 {
        self.membership.iter().cloned().fold(0.0_f64, f64::max)
    }
    /// Support: indices with membership > 0.
    pub fn support(&self) -> Vec<usize> {
        self.strong_alpha_cut(0.0)
    }
    /// Core: indices with membership = 1.
    pub fn core(&self) -> Vec<usize> {
        self.membership
            .iter()
            .enumerate()
            .filter(|(_, &d)| (d - 1.0).abs() < 1e-9)
            .map(|(i, _)| i)
            .collect()
    }
    /// Is the fuzzy set normal (height = 1)?
    pub fn is_normal(&self) -> bool {
        (self.height() - 1.0).abs() < 1e-9
    }
}
/// A fuzzy topology: a collection of fuzzy sets (open sets) over a universe.
#[derive(Debug, Clone)]
pub struct FuzzyTopology {
    pub universe_size: usize,
    /// Open fuzzy sets: each is a membership vector.
    pub open_sets: Vec<Vec<f64>>,
}
impl FuzzyTopology {
    pub fn new(universe_size: usize) -> Self {
        let mut ft = FuzzyTopology {
            universe_size,
            open_sets: Vec::new(),
        };
        ft.open_sets.push(vec![0.0; universe_size]);
        ft.open_sets.push(vec![1.0; universe_size]);
        ft
    }
    pub fn add_open_set(&mut self, set: Vec<f64>) {
        assert_eq!(set.len(), self.universe_size);
        self.open_sets.push(set);
    }
    /// Check closure under finite intersection (minimum).
    pub fn closed_under_intersection(&self) -> bool {
        let n = self.open_sets.len();
        for i in 0..n {
            for j in i..n {
                let inter: Vec<f64> = self.open_sets[i]
                    .iter()
                    .zip(self.open_sets[j].iter())
                    .map(|(&a, &b)| a.min(b))
                    .collect();
                if !self.contains_set(&inter) {
                    return false;
                }
            }
        }
        true
    }
    /// Check closure under arbitrary union (maximum over all subsets — here pairwise).
    pub fn closed_under_union(&self) -> bool {
        let n = self.open_sets.len();
        for i in 0..n {
            for j in i..n {
                let union: Vec<f64> = self.open_sets[i]
                    .iter()
                    .zip(self.open_sets[j].iter())
                    .map(|(&a, &b)| a.max(b))
                    .collect();
                if !self.contains_set(&union) {
                    return false;
                }
            }
        }
        true
    }
    fn contains_set(&self, s: &[f64]) -> bool {
        self.open_sets
            .iter()
            .any(|o| o.iter().zip(s.iter()).all(|(&a, &b)| (a - b).abs() < 1e-9))
    }
}
/// Evaluates various t-norm families and verifies their algebraic properties.
pub struct TNormComputer;
impl TNormComputer {
    /// Evaluate the Frank t-norm F_s(a,b) for parameter s ∈ (0,∞) \ {1}.
    ///
    /// - s→0: drastic t-norm, s→1: product, s→∞: minimum.
    pub fn frank(s: f64, a: f64, b: f64) -> f64 {
        if s <= 0.0 {
            return a.min(b);
        }
        if (s - 1.0).abs() < 1e-9 {
            return a * b;
        }
        if s > 1e9 {
            return a.min(b);
        }
        let sa = s.powf(a) - 1.0;
        let sb = s.powf(b) - 1.0;
        let denom = s - 1.0;
        if denom.abs() < 1e-15 {
            return a.min(b);
        }
        let inner = 1.0 + sa * sb / denom;
        if inner <= 0.0 {
            return 0.0;
        }
        inner.log(s).clamp(0.0, 1.0)
    }
    /// Evaluate the Yager t-norm T_p(a,b) = max(0, 1 − ((1−a)^p + (1−b)^p)^{1/p}).
    pub fn yager(p: f64, a: f64, b: f64) -> f64 {
        if p <= 0.0 {
            return a.min(b);
        }
        let sum = (1.0 - a).powf(p) + (1.0 - b).powf(p);
        (1.0 - sum.powf(1.0 / p)).max(0.0).min(1.0)
    }
    /// Evaluate the Schweizer-Sklar t-norm T_p(a,b) = (a^p + b^p − 1)^{1/p}.
    pub fn schweizer_sklar(p: f64, a: f64, b: f64) -> f64 {
        if p == 0.0 {
            return a * b;
        }
        if p < 0.0 {
            let val = (a.powf(p) + b.powf(p) - 1.0).powf(1.0 / p);
            return val.max(0.0).min(1.0);
        }
        let val = (a.powf(p) + b.powf(p) - 1.0).powf(1.0 / p);
        val.max(0.0).min(1.0)
    }
    /// Check commutativity of a t-norm at sample points.
    pub fn check_commutativity<F: Fn(f64, f64) -> f64>(t: &F, samples: &[(f64, f64)]) -> bool {
        samples
            .iter()
            .all(|&(a, b)| (t(a, b) - t(b, a)).abs() < 1e-9)
    }
    /// Check associativity of a t-norm at sample triples.
    pub fn check_associativity<F: Fn(f64, f64) -> f64>(t: &F, triples: &[(f64, f64, f64)]) -> bool {
        triples
            .iter()
            .all(|&(a, b, c)| (t(t(a, b), c) - t(a, t(b, c))).abs() < 1e-9)
    }
    /// Check the boundary condition: T(a, 1) = a.
    pub fn check_boundary<F: Fn(f64, f64) -> f64>(t: &F, samples: &[f64]) -> bool {
        samples.iter().all(|&a| (t(a, 1.0) - a).abs() < 1e-9)
    }
    /// Check monotonicity: a ≤ b implies T(a,c) ≤ T(b,c).
    pub fn check_monotonicity<F: Fn(f64, f64) -> f64>(t: &F, samples: &[(f64, f64, f64)]) -> bool {
        samples
            .iter()
            .all(|&(a, b, c)| a > b || t(a, c) <= t(b, c) + 1e-9)
    }
}
/// Gradual element: a fuzzy set representing a graded truth value.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GradualElement {
    pub name: String,
    pub degree: f64,
}
#[allow(dead_code)]
impl GradualElement {
    pub fn new(name: &str, degree: f64) -> Self {
        GradualElement {
            name: name.to_string(),
            degree: degree.clamp(0.0, 1.0),
        }
    }
    pub fn is_true(&self) -> bool {
        self.degree > 0.5
    }
    pub fn complement(&self) -> Self {
        GradualElement::new(&format!("not_{}", self.name), 1.0 - self.degree)
    }
    pub fn conjunction(&self, other: &GradualElement) -> GradualElement {
        let deg = self.degree.min(other.degree);
        GradualElement::new(&format!("({} AND {})", self.name, other.name), deg)
    }
    pub fn disjunction(&self, other: &GradualElement) -> GradualElement {
        let deg = self.degree.max(other.degree);
        GradualElement::new(&format!("({} OR {})", self.name, other.name), deg)
    }
}
/// A fuzzy metric space (in the sense of George and Veeramani).
/// M(x, y, t) ∈ [0,1] represents the "probability" that d(x,y) < t.
#[derive(Debug, Clone)]
pub struct FuzzyMetricSpace {
    pub points: usize,
    /// M[x][y][t_idx] — indexed over a finite grid of t values.
    pub metric: Vec<Vec<Vec<f64>>>,
    pub t_grid: Vec<f64>,
}
impl FuzzyMetricSpace {
    pub fn new(points: usize, t_grid: Vec<f64>) -> Self {
        let nt = t_grid.len();
        FuzzyMetricSpace {
            points,
            metric: vec![vec![vec![0.0; nt]; points]; points],
            t_grid,
        }
    }
    pub fn set_metric(&mut self, x: usize, y: usize, t_idx: usize, value: f64) {
        self.metric[x][y][t_idx] = value.clamp(0.0, 1.0);
        self.metric[y][x][t_idx] = value.clamp(0.0, 1.0);
    }
    /// GV axiom: M(x, y, t) → 1 as t → ∞ (check last t entry is 1).
    pub fn check_limit_axiom(&self) -> bool {
        let last = self.t_grid.len() - 1;
        for x in 0..self.points {
            for y in 0..self.points {
                if (self.metric[x][y][last] - 1.0).abs() > 1e-6 {
                    return false;
                }
            }
        }
        true
    }
    /// GV axiom: M(x, x, t) = 1 for all t > 0.
    pub fn check_diagonal_axiom(&self) -> bool {
        for x in 0..self.points {
            for t_idx in 0..self.t_grid.len() {
                if (self.metric[x][x][t_idx] - 1.0).abs() > 1e-6 {
                    return false;
                }
            }
        }
        true
    }
    /// GV non-separability check: M(x, y, t) = 1 for all t > 0 implies x = y.
    pub fn check_non_separability(&self) -> bool {
        for x in 0..self.points {
            for y in 0..self.points {
                if x == y {
                    continue;
                }
                let all_one = self.metric[x][y].iter().all(|&v| (v - 1.0).abs() < 1e-6);
                if all_one {
                    return false;
                }
            }
        }
        true
    }
}
/// Applies linguistic hedges (modifiers) to fuzzy membership degrees.
///
/// Hedges modify the membership function to represent qualifications
/// like "very", "more or less", "somewhat", "extremely", etc.
pub struct LinguisticHedgeApplier;
impl LinguisticHedgeApplier {
    /// "very A": μ_A(x)^2 (concentration).
    pub fn very(degree: f64) -> f64 {
        degree * degree
    }
    /// "more or less A": μ_A(x)^{0.5} (dilation).
    pub fn more_or_less(degree: f64) -> f64 {
        degree.sqrt()
    }
    /// "somewhat A": μ_A(x)^{0.333} (moderate dilation).
    pub fn somewhat(degree: f64) -> f64 {
        degree.powf(1.0 / 3.0)
    }
    /// "extremely A": μ_A(x)^3 (stronger concentration).
    pub fn extremely(degree: f64) -> f64 {
        degree.powi(3)
    }
    /// "not A": 1 − μ_A(x) (standard negation).
    pub fn not(degree: f64) -> f64 {
        1.0 - degree
    }
    /// "slightly A": intermediate concentration μ_A(x)^{1.7}.
    pub fn slightly(degree: f64) -> f64 {
        degree.powf(1.7)
    }
    /// "indeed A": normalization-based hedge (intensification).
    ///
    /// INT(μ) = 2μ^2 if μ ≤ 0.5, else 1 − 2(1−μ)^2.
    pub fn indeed(degree: f64) -> f64 {
        if degree <= 0.5 {
            2.0 * degree * degree
        } else {
            1.0 - 2.0 * (1.0 - degree).powi(2)
        }
    }
    /// "plus A": μ_A(x)^{1.25} (gentle concentration).
    pub fn plus(degree: f64) -> f64 {
        degree.powf(1.25)
    }
    /// Apply hedge by name. Returns degree unchanged if hedge is unknown.
    pub fn apply(hedge: &str, degree: f64) -> f64 {
        match hedge {
            "very" => Self::very(degree),
            "more_or_less" | "more-or-less" | "sort_of" => Self::more_or_less(degree),
            "somewhat" => Self::somewhat(degree),
            "extremely" => Self::extremely(degree),
            "not" => Self::not(degree),
            "slightly" => Self::slightly(degree),
            "indeed" => Self::indeed(degree),
            "plus" => Self::plus(degree),
            _ => degree,
        }
    }
    /// Apply a sequence of hedges in order (innermost first).
    pub fn apply_chain(hedges: &[&str], degree: f64) -> f64 {
        hedges.iter().fold(degree, |d, h| Self::apply(h, d))
    }
    /// Apply hedge to an entire fuzzy set.
    pub fn apply_to_set(hedge: &str, set: &FuzzySet) -> FuzzySet {
        let membership = set
            .membership
            .iter()
            .map(|&d| Self::apply(hedge, d))
            .collect();
        FuzzySet {
            universe_size: set.universe_size,
            membership,
        }
    }
    /// Returns the power exponent associated with a hedge (for analysis).
    pub fn exponent(hedge: &str) -> Option<f64> {
        match hedge {
            "very" => Some(2.0),
            "more_or_less" => Some(0.5),
            "somewhat" => Some(1.0 / 3.0),
            "extremely" => Some(3.0),
            "slightly" => Some(1.7),
            "plus" => Some(1.25),
            _ => None,
        }
    }
}
/// Fuzzy rough set approximation over a fuzzy similarity relation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuzzyRoughApprox {
    pub universe_size: usize,
    pub similarity: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl FuzzyRoughApprox {
    pub fn new(n: usize) -> Self {
        let mut sim = vec![vec![0.0; n]; n];
        for i in 0..n {
            sim[i][i] = 1.0;
        }
        FuzzyRoughApprox {
            universe_size: n,
            similarity: sim,
        }
    }
    pub fn set_similarity(&mut self, x: usize, y: usize, val: f64) {
        self.similarity[x][y] = val.clamp(0.0, 1.0);
        self.similarity[y][x] = val.clamp(0.0, 1.0);
    }
    /// Lower approximation of fuzzy set A: (R_↓ A)(x) = inf_y T(R(x,y), A(y))
    pub fn lower_approx(&self, a: &[f64]) -> Vec<f64> {
        (0..self.universe_size)
            .map(|x| {
                (0..self.universe_size)
                    .map(|y| {
                        let r = self.similarity[x][y];
                        let ay = a[y];
                        (1.0 - r + ay).min(1.0)
                    })
                    .fold(f64::INFINITY, f64::min)
            })
            .collect()
    }
    /// Upper approximation of fuzzy set A: (R^↑ A)(x) = sup_y T(R(x,y), A(y))
    pub fn upper_approx(&self, a: &[f64]) -> Vec<f64> {
        (0..self.universe_size)
            .map(|x| {
                (0..self.universe_size)
                    .map(|y| self.similarity[x][y].min(a[y]))
                    .fold(0.0f64, f64::max)
            })
            .collect()
    }
}
/// Mamdani fuzzy inference system.
#[derive(Debug, Clone)]
pub struct MamdaniSystem {
    pub rules: Vec<MamdaniRule>,
    pub output_size: usize,
}
impl MamdaniSystem {
    pub fn new(output_size: usize) -> Self {
        MamdaniSystem {
            rules: Vec::new(),
            output_size,
        }
    }
    pub fn add_rule(&mut self, rule: MamdaniRule) {
        self.rules.push(rule);
    }
    /// Aggregate all rule outputs using maximum, clip each by firing strength.
    pub fn infer(&self, input_degrees: &[Vec<f64>]) -> FuzzySet {
        let mut agg = FuzzySet::new(self.output_size);
        for rule in &self.rules {
            let strength = rule
                .antecedent_mf
                .iter()
                .zip(input_degrees.iter().flatten())
                .map(|(&a, &b)| a.min(b))
                .fold(1.0_f64, f64::min);
            for i in 0..self.output_size {
                let clipped = rule.consequent.get(i).min(strength);
                let current = agg.get(i);
                agg.set(i, current.max(clipped));
            }
        }
        agg
    }
}
/// Sugeno (Takagi-Sugeno) fuzzy inference system.
#[derive(Debug, Clone)]
pub struct SugenoSystem {
    pub rules: Vec<SugenoRule>,
}
impl SugenoSystem {
    pub fn new() -> Self {
        SugenoSystem { rules: Vec::new() }
    }
    pub fn add_rule(&mut self, rule: SugenoRule) {
        self.rules.push(rule);
    }
    /// Compute the crisp output using weighted average defuzzification.
    pub fn infer(&self, inputs: &[f64], input_degrees: &[Vec<f64>]) -> f64 {
        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;
        for rule in &self.rules {
            let strength: f64 = rule
                .antecedent_mf
                .iter()
                .zip(input_degrees.iter().flatten())
                .map(|(&a, &b)| a.min(b))
                .fold(1.0_f64, f64::min);
            let z = rule.output_const
                + rule
                    .output_coeffs
                    .iter()
                    .zip(inputs.iter())
                    .map(|(&c, &x)| c * x)
                    .sum::<f64>();
            weighted_sum += strength * z;
            weight_total += strength;
        }
        if weight_total.abs() < 1e-12 {
            0.0
        } else {
            weighted_sum / weight_total
        }
    }
}
/// Fuzzy number arithmetic (triangular fuzzy numbers).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TriangularFuzzyNum {
    pub lower: f64,
    pub modal: f64,
    pub upper: f64,
}
#[allow(dead_code)]
impl TriangularFuzzyNum {
    pub fn new(lower: f64, modal: f64, upper: f64) -> Self {
        assert!(lower <= modal && modal <= upper);
        TriangularFuzzyNum {
            lower,
            modal,
            upper,
        }
    }
    pub fn membership(&self, x: f64) -> f64 {
        if x < self.lower || x > self.upper {
            0.0
        } else if x <= self.modal {
            (x - self.lower) / (self.modal - self.lower).max(1e-12)
        } else {
            (self.upper - x) / (self.upper - self.modal).max(1e-12)
        }
    }
    pub fn add(&self, other: &TriangularFuzzyNum) -> TriangularFuzzyNum {
        TriangularFuzzyNum::new(
            self.lower + other.lower,
            self.modal + other.modal,
            self.upper + other.upper,
        )
    }
    pub fn scale(&self, k: f64) -> TriangularFuzzyNum {
        if k >= 0.0 {
            TriangularFuzzyNum::new(k * self.lower, k * self.modal, k * self.upper)
        } else {
            TriangularFuzzyNum::new(k * self.upper, k * self.modal, k * self.lower)
        }
    }
    pub fn defuzzify_centroid(&self) -> f64 {
        (self.lower + self.modal + self.upper) / 3.0
    }
    pub fn alpha_cut(&self, alpha: f64) -> (f64, f64) {
        let lo = self.lower + alpha * (self.modal - self.lower);
        let hi = self.upper - alpha * (self.upper - self.modal);
        (lo, hi)
    }
}
