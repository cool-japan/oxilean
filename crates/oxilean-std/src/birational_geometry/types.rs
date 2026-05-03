//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Data for Kawamata-Viehweg vanishing theorem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KVVanishingData {
    /// Variety.
    pub variety: String,
    /// Dimension.
    pub dim: usize,
    /// Whether the nef and big conditions hold.
    pub is_nef_big: bool,
    /// The degree i at which H^i(K_X + L) = 0.
    pub vanishing_degrees: Vec<usize>,
}
#[allow(dead_code)]
impl KVVanishingData {
    /// Creates KV vanishing data.
    pub fn new(variety: &str, dim: usize) -> Self {
        KVVanishingData {
            variety: variety.to_string(),
            dim,
            is_nef_big: false,
            vanishing_degrees: Vec::new(),
        }
    }
    /// Marks the line bundle as nef and big.
    pub fn nef_and_big(mut self) -> Self {
        self.is_nef_big = true;
        self.vanishing_degrees = (1..=self.dim).collect();
        self
    }
    /// Returns the KV vanishing statement.
    pub fn vanishing_statement(&self) -> String {
        if self.is_nef_big {
            format!("H^i(K_X + L) = 0 for i > 0 on {}", self.variety)
        } else {
            format!(
                "KV vanishing not applicable: L not nef+big on {}",
                self.variety
            )
        }
    }
    /// Checks if degree i vanishes.
    pub fn vanishes_at(&self, i: usize) -> bool {
        self.vanishing_degrees.contains(&i)
    }
}
/// A log pair (X, Δ) consisting of a variety and an effective boundary divisor.
#[derive(Debug, Clone)]
pub struct LogPair {
    /// The underlying variety.
    pub variety: String,
    /// Coefficients of the boundary divisor Δ = ∑ a_i D_i (each a_i ∈ \[0, 1\]).
    pub boundary_coeffs: Vec<f64>,
    /// Names of the boundary divisors.
    pub boundary_components: Vec<String>,
}
impl LogPair {
    /// Create a log pair with no boundary.
    pub fn trivial(variety: impl Into<String>) -> Self {
        LogPair {
            variety: variety.into(),
            boundary_coeffs: vec![],
            boundary_components: vec![],
        }
    }
    /// Add a boundary component with coefficient a ∈ \[0, 1\].
    pub fn with_boundary(mut self, name: impl Into<String>, coeff: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&coeff),
            "Boundary coefficients must be in [0, 1]"
        );
        self.boundary_components.push(name.into());
        self.boundary_coeffs.push(coeff);
        self
    }
    /// Check if (X, Δ) is log-canonical (all coefficients ≤ 1).
    pub fn is_log_canonical(&self) -> bool {
        self.boundary_coeffs.iter().all(|&a| a <= 1.0 + 1e-10)
    }
    /// Check if (X, Δ) is klt (all coefficients < 1).
    pub fn is_klt(&self) -> bool {
        self.boundary_coeffs.iter().all(|&a| a < 1.0 - 1e-10)
    }
    /// Check if (X, Δ) is plt (purely log-terminal): reduced components can have coeff = 1.
    pub fn is_plt(&self) -> bool {
        self.is_log_canonical()
    }
    /// Total degree of the boundary ∑ a_i.
    pub fn boundary_degree(&self) -> f64 {
        self.boundary_coeffs.iter().sum()
    }
}
/// Blow-up data: records what was blown up and the resulting variety.
#[derive(Debug, Clone)]
pub struct BlowUpData {
    /// The original variety.
    pub original: String,
    /// The center of the blow-up (subvariety or ideal).
    pub center: String,
    /// Codimension of the center.
    pub center_codim: usize,
    /// The exceptional divisor E ≅ P^{r-1}-bundle over the center.
    pub exceptional_divisor: String,
}
impl BlowUpData {
    /// Blow up a smooth variety along a smooth center of codimension r.
    pub fn new(original: impl Into<String>, center: impl Into<String>, codim: usize) -> Self {
        let orig = original.into();
        let c = center.into();
        BlowUpData {
            exceptional_divisor: format!("E = P(N_{{{}/{}}})", c, orig),
            original: orig,
            center: c,
            center_codim: codim,
        }
    }
    /// The self-intersection number E^{r-1} on the blow-up equals (-1)^{r-1} deg(center).
    pub fn exceptional_self_intersection(&self) -> i64 {
        let r = self.center_codim;
        if r == 0 {
            return 1;
        }
        if (r - 1) % 2 == 0 {
            1i64
        } else {
            -1i64
        }
    }
    /// The discrepancy of the exceptional divisor on the smooth blow-up: a(E) = r - 1.
    pub fn exceptional_discrepancy(&self) -> i64 {
        self.center_codim as i64 - 1
    }
}
/// Type of MMP operation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MMPOperation {
    /// Divisorial contraction.
    DivisorialContraction,
    /// Flip.
    Flip,
    /// Mori fiber space (final step).
    MoriFiberSpace,
    /// Minimal model (K_X nef).
    MinimalModel,
}
/// Minimal model program step: classify the type of the contraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MMPStep {
    /// Contract a divisor E to a variety of strictly smaller Picard number.
    DivisorialContraction { contracted_divisor: String },
    /// Replace a flipping contraction by its flip.
    Flip { flipping_locus: String },
    /// Reached a minimal model (K_X nef).
    MinimalModel,
    /// X is uniruled and MMP terminates in a Fano fibration.
    FanoFibration { base: String },
}
impl MMPStep {
    /// Description of the MMP step.
    pub fn description(&self) -> String {
        match self {
            MMPStep::DivisorialContraction { contracted_divisor } => {
                format!("Divisorial contraction: contract {}", contracted_divisor)
            }
            MMPStep::Flip { flipping_locus } => {
                format!("Flip: flip over {}", flipping_locus)
            }
            MMPStep::MinimalModel => "Minimal model reached: K_X is nef".to_string(),
            MMPStep::FanoFibration { base } => format!("Fano fibration over {}", base),
        }
    }
    /// Check if this step terminates the MMP.
    pub fn is_terminal(&self) -> bool {
        matches!(self, MMPStep::MinimalModel | MMPStep::FanoFibration { .. })
    }
}
/// Classification of the contraction associated with an extremal ray.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractionType {
    /// Contracts a divisor to a lower-dimensional variety.
    Divisorial,
    /// Small contraction: flipping locus has codimension ≥ 2.
    Small,
    /// Fiber type: target has strictly smaller dimension.
    FiberType,
}
/// A step in the Minimal Model Program (MMP).
///
/// Encodes the four possible outcomes of a single MMP step:
/// divisorial contraction, flip, reaching a minimal model, or arriving at a Mori fiber space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmpStep {
    /// Contract a divisor E: reduces Picard number by 1.
    Contraction {
        /// The divisor being contracted.
        divisor: String,
        /// The type of the target singularity after contraction.
        singularity_type: SingularityType,
    },
    /// Replace a flipping contraction by its flip.
    Flip {
        /// Description of the flipping locus.
        locus: String,
    },
    /// A flopping contraction followed by its flop.
    Flop {
        /// Description of the flopping locus.
        locus: String,
    },
    /// Terminate: K_X is nef (minimal model reached).
    MinimalModel,
    /// Terminate: Mori fiber space structure X → Z.
    FiberSpace {
        /// The base of the fiber space.
        base: String,
        /// Dimension of the fiber.
        fiber_dim: usize,
    },
}
impl MmpStep {
    /// Human-readable description of the step.
    pub fn description(&self) -> String {
        match self {
            MmpStep::Contraction {
                divisor,
                singularity_type,
            } => {
                format!(
                    "Divisorial contraction of {} (→ {} singularities)",
                    divisor,
                    singularity_type.name()
                )
            }
            MmpStep::Flip { locus } => format!("Flip over {}", locus),
            MmpStep::Flop { locus } => format!("Flop over {}", locus),
            MmpStep::MinimalModel => "Minimal model reached (K_X nef)".to_string(),
            MmpStep::FiberSpace { base, fiber_dim } => {
                format!("Mori fiber space over {} (fiber dim = {})", base, fiber_dim)
            }
        }
    }
    /// Whether the step terminates the MMP.
    pub fn is_terminal(&self) -> bool {
        matches!(self, MmpStep::MinimalModel | MmpStep::FiberSpace { .. })
    }
}
/// Data for a log pair (X, Δ) in birational geometry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogPairData {
    /// Variety X.
    pub variety: String,
    /// Boundary divisor Δ = sum a_i D_i.
    pub boundary_components: Vec<(String, f64)>,
    /// Log discrepancies (for singularity type).
    pub log_discrepancies: Vec<f64>,
}
#[allow(dead_code)]
impl LogPairData {
    /// Creates a log pair.
    pub fn new(variety: &str) -> Self {
        LogPairData {
            variety: variety.to_string(),
            boundary_components: Vec::new(),
            log_discrepancies: Vec::new(),
        }
    }
    /// Adds a boundary component a_i D_i.
    pub fn add_boundary(&mut self, divisor: &str, coeff: f64) {
        self.boundary_components.push((divisor.to_string(), coeff));
    }
    /// Adds a log discrepancy.
    pub fn add_log_discrepancy(&mut self, a: f64) {
        self.log_discrepancies.push(a);
    }
    /// Returns the total boundary coefficient.
    pub fn total_coefficient(&self) -> f64 {
        self.boundary_components.iter().map(|(_, a)| a).sum()
    }
    /// Checks if the pair is Kawamata log terminal (klt): all log discrepancies > -1.
    pub fn is_klt(&self) -> bool {
        self.log_discrepancies.iter().all(|&a| a > -1.0)
    }
    /// Checks if the pair is log canonical (lc): all log discrepancies >= -1.
    pub fn is_log_canonical(&self) -> bool {
        self.log_discrepancies.iter().all(|&a| a >= -1.0)
    }
    /// Returns the singularity type string.
    pub fn singularity_type(&self) -> &str {
        if self.log_discrepancies.is_empty() {
            "smooth"
        } else if self.is_klt() {
            "klt (Kawamata log terminal)"
        } else if self.is_log_canonical() {
            "lc (log canonical)"
        } else {
            "worse than log canonical"
        }
    }
}
/// Zariski decomposition of a pseudoeffective divisor D = P + N.
///
/// - P is the nef part (intersection-theoretically trivial on curves in N).
/// - N is the negative part (effective, whose support contains all negative-definite components).
///
/// This is a simplified representation storing divisor names and rational coefficients.
#[derive(Debug, Clone)]
pub struct ZariskiDecomp {
    /// Original divisor name.
    pub divisor: String,
    /// Nef part P.
    pub nef_part: Vec<(String, f64)>,
    /// Negative part N.
    pub neg_part: Vec<(String, f64)>,
}
impl ZariskiDecomp {
    /// Construct a Zariski decomposition.
    pub fn new(
        divisor: impl Into<String>,
        nef_part: Vec<(String, f64)>,
        neg_part: Vec<(String, f64)>,
    ) -> Self {
        ZariskiDecomp {
            divisor: divisor.into(),
            nef_part,
            neg_part,
        }
    }
    /// Check that all N-coefficients are non-negative.
    pub fn is_negative_part_effective(&self) -> bool {
        self.neg_part.iter().all(|(_, c)| *c >= 0.0)
    }
    /// Total coefficient of the nef part.
    pub fn nef_degree(&self) -> f64 {
        self.nef_part.iter().map(|(_, c)| c).sum()
    }
    /// Total coefficient of the negative part.
    pub fn neg_degree(&self) -> f64 {
        self.neg_part.iter().map(|(_, c)| c).sum()
    }
}
/// A projective variety represented by dimension and degree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectiveVariety {
    /// Dimension of the variety.
    pub dim: usize,
    /// Degree in projective space (self-intersection of hyperplane class).
    pub degree: u64,
    /// Name / description.
    pub name: String,
}
impl ProjectiveVariety {
    /// Create a new projective variety.
    pub fn new(dim: usize, degree: u64, name: impl Into<String>) -> Self {
        ProjectiveVariety {
            dim,
            degree,
            name: name.into(),
        }
    }
    /// Projective space P^n, which has degree 1.
    pub fn projective_space(n: usize) -> Self {
        ProjectiveVariety::new(n, 1, format!("P^{}", n))
    }
    /// A quadric hypersurface Q ⊂ P^{n+1} of dimension n and degree 2.
    pub fn quadric(n: usize) -> Self {
        ProjectiveVariety::new(n, 2, format!("Q^{}", n))
    }
    /// Del Pezzo surface of degree d (1 ≤ d ≤ 9).
    /// S_d = P^2 blown up at (9-d) points in general position.
    pub fn del_pezzo(d: usize) -> Self {
        assert!((1..=9).contains(&d), "Del Pezzo degree must be 1..=9");
        ProjectiveVariety::new(2, d as u64, format!("S_{}", d))
    }
}
/// Kodaira dimension enum with -∞ = NegInfinity convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KodairaDim {
    /// κ = -∞: uniruled varieties (covered by rational curves).
    NegInfinity,
    /// κ = k for k ≥ 0.
    Finite(i64),
}
impl KodairaDim {
    /// Construct κ = 0 (Calabi-Yau, abelian varieties, K3, Enriques, ...).
    pub fn zero() -> Self {
        KodairaDim::Finite(0)
    }
    /// Kodaira dimension of a product: κ(X × Y) = κ(X) + κ(Y).
    pub fn product(self, other: KodairaDim) -> KodairaDim {
        match (self, other) {
            (KodairaDim::NegInfinity, _) | (_, KodairaDim::NegInfinity) => KodairaDim::NegInfinity,
            (KodairaDim::Finite(a), KodairaDim::Finite(b)) => KodairaDim::Finite(a + b),
        }
    }
    /// Whether this variety is of general type: κ = dim.
    pub fn is_general_type(self, dim: usize) -> bool {
        matches!(self, KodairaDim::Finite(k) if k == dim as i64)
    }
    /// Whether the variety is uniruled: κ = -∞.
    pub fn is_uniruled(self) -> bool {
        self == KodairaDim::NegInfinity
    }
    /// Human-readable classification.
    pub fn classify(self, dim: usize) -> &'static str {
        match self {
            KodairaDim::NegInfinity => "uniruled",
            KodairaDim::Finite(0) => "Kodaira dim 0 (CY / K3 / abelian type)",
            KodairaDim::Finite(k) if k == dim as i64 => "general type",
            KodairaDim::Finite(_) => "intermediate Kodaira dimension",
        }
    }
}
/// Represents a step in the Minimal Model Program.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MMPStepData {
    /// Type of operation.
    pub operation: MMPOperation,
    /// Description of the resulting variety.
    pub result_variety: String,
    /// The exceptional divisor (for contractions).
    pub exceptional_divisor: Option<String>,
}
#[allow(dead_code)]
impl MMPStepData {
    /// Creates an MMP step.
    pub fn new(op: MMPOperation, result: &str) -> Self {
        MMPStepData {
            operation: op,
            result_variety: result.to_string(),
            exceptional_divisor: None,
        }
    }
    /// Sets the exceptional divisor.
    pub fn with_exceptional(mut self, div: &str) -> Self {
        self.exceptional_divisor = Some(div.to_string());
        self
    }
    /// Returns the description of this step.
    pub fn description(&self) -> String {
        let op_name = match &self.operation {
            MMPOperation::DivisorialContraction => "Divisorial contraction",
            MMPOperation::Flip => "Flip",
            MMPOperation::MoriFiberSpace => "Mori fiber space",
            MMPOperation::MinimalModel => "Minimal model",
        };
        format!("{} → {}", op_name, self.result_variety)
    }
    /// Checks if this is the final step.
    pub fn is_final(&self) -> bool {
        matches!(
            &self.operation,
            MMPOperation::MoriFiberSpace | MMPOperation::MinimalModel
        )
    }
}
/// Sarkisov link type.
///
/// The Sarkisov program decomposes every birational map between
/// Mori fiber spaces into elementary links of type I–IV.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SarkisovLinkType {
    /// Type I: blow-up on the left side.
    TypeI,
    /// Type II: blow-up on both sides.
    TypeII,
    /// Type III: blow-down on the right side.
    TypeIII,
    /// Type IV: blow-down then blow-up without changing the base.
    TypeIV,
}
impl SarkisovLinkType {
    /// Description of the link type.
    pub fn description(&self) -> &'static str {
        match self {
            SarkisovLinkType::TypeI => "Type I: blow-up (left), fiber space change",
            SarkisovLinkType::TypeII => "Type II: blow-up (both), same base",
            SarkisovLinkType::TypeIII => "Type III: blow-down (right), base change",
            SarkisovLinkType::TypeIV => "Type IV: blow-down (right), flip base",
        }
    }
    /// Number code (I=1, II=2, III=3, IV=4).
    pub fn code(&self) -> u8 {
        match self {
            SarkisovLinkType::TypeI => 1,
            SarkisovLinkType::TypeII => 2,
            SarkisovLinkType::TypeIII => 3,
            SarkisovLinkType::TypeIV => 4,
        }
    }
}
/// Iitaka fibration data for a variety of Kodaira dimension κ.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IitakaFibration {
    /// Source variety.
    pub source: String,
    /// Base of the Iitaka fibration.
    pub base: String,
    /// Fiber description.
    pub fiber: String,
    /// Kodaira dimension of the source.
    pub kodaira_dim: i64,
    /// Dimension of the base (= kodaira_dim for Iitaka fibration).
    pub base_dim: usize,
}
#[allow(dead_code)]
impl IitakaFibration {
    /// Creates Iitaka fibration data.
    pub fn new(source: &str, base: &str, fiber: &str, kappa: i64) -> Self {
        let base_dim = kappa.max(0) as usize;
        IitakaFibration {
            source: source.to_string(),
            base: base.to_string(),
            fiber: fiber.to_string(),
            kodaira_dim: kappa,
            base_dim,
        }
    }
    /// Returns the Iitaka-Kodaira addition formula: κ(X) <= κ(F) + κ(B).
    pub fn addition_formula(&self, kappa_fiber: i64, kappa_base: i64) -> bool {
        self.kodaira_dim <= kappa_fiber + kappa_base
    }
    /// Returns a description of the fibration.
    pub fn description(&self) -> String {
        format!(
            "{} → {} (base, κ={}) with fiber {}",
            self.source, self.base, self.kodaira_dim, self.fiber
        )
    }
    /// Checks if source is of general type (κ = dim).
    pub fn is_general_type(&self, dim: usize) -> bool {
        self.kodaira_dim == dim as i64
    }
}
/// Mori cone information for a smooth Fano variety.
///
/// Stores the generators of the Mori cone NE(X) as curve classes (with K_X-degree).
#[derive(Debug, Clone)]
pub struct MoriCone {
    /// Dimension of the variety.
    pub dim: usize,
    /// Extremal rays represented by K_X-degree (should be < 0).
    pub extremal_rays: Vec<i64>,
}
impl MoriCone {
    /// Create a Mori cone for projective space P^n with a single extremal ray (K_{P^n} · l = -(n+1)).
    pub fn projective_space(n: usize) -> Self {
        MoriCone {
            dim: n,
            extremal_rays: vec![-(n as i64 + 1)],
        }
    }
    /// Create a Mori cone for a del Pezzo surface S_d.
    /// S_9 = P^2 has one extremal ray; S_{9-k} has k more from the blown-up points.
    pub fn del_pezzo(d: usize) -> Self {
        let blown_up = 9 - d;
        let mut rays: Vec<i64> = vec![-3];
        rays.extend(std::iter::repeat(-1).take(blown_up));
        MoriCone {
            dim: 2,
            extremal_rays: rays,
        }
    }
    /// Number of extremal rays.
    pub fn num_extremal_rays(&self) -> usize {
        self.extremal_rays.len()
    }
    /// Check if all extremal rays are K-negative (Fano condition).
    pub fn is_fano(&self) -> bool {
        self.extremal_rays.iter().all(|&r| r < 0)
    }
}
/// Data tracking the abundance conjecture.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbundanceData {
    /// Variety.
    pub variety: String,
    /// Kodaira dimension κ.
    pub kodaira_dim: Option<i64>,
    /// Whether K_X is nef.
    pub kx_nef: bool,
    /// Whether K_X is semi-ample (abundance holds).
    pub kx_semi_ample: bool,
    /// Dimension of the variety.
    pub dim: usize,
}
#[allow(dead_code)]
impl AbundanceData {
    /// Creates abundance data.
    pub fn new(variety: &str, dim: usize) -> Self {
        AbundanceData {
            variety: variety.to_string(),
            kodaira_dim: None,
            kx_nef: false,
            kx_semi_ample: false,
            dim,
        }
    }
    /// Sets Kodaira dimension.
    pub fn with_kodaira_dim(mut self, kappa: i64) -> Self {
        self.kodaira_dim = Some(kappa);
        self
    }
    /// Marks K_X as nef.
    pub fn nef(mut self) -> Self {
        self.kx_nef = true;
        self
    }
    /// Marks abundance as holding.
    pub fn abundant(mut self) -> Self {
        self.kx_semi_ample = true;
        self
    }
    /// Returns conjecture status.
    pub fn abundance_status(&self) -> String {
        if self.kx_nef && self.kx_semi_ample {
            format!(
                "Abundance holds for {}: K_X nef and semi-ample",
                self.variety
            )
        } else if self.kx_nef {
            format!("Abundance not yet verified for {}", self.variety)
        } else {
            format!("{} does not have nef canonical class", self.variety)
        }
    }
    /// Checks dim-3 abundance (known in dim <= 3).
    pub fn abundance_known(&self) -> bool {
        self.dim <= 3
    }
}
/// Minimal model flowchart simulator.
///
/// Runs a simplified MMP on a sequence of user-specified steps.
/// Terminates when a terminal step (minimal model or fiber space) is reached.
pub struct MmpFlowchart {
    /// The log pair being processed.
    pub pair: LogPair,
    /// The history of steps taken so far.
    pub history: Vec<MmpStep>,
    /// The current Picard number (decreases with divisorial contractions).
    pub picard_number: usize,
}
impl MmpFlowchart {
    /// Create a new MMP flowchart starting from a log pair.
    pub fn new(pair: LogPair, initial_picard: usize) -> Self {
        MmpFlowchart {
            pair,
            history: Vec::new(),
            picard_number: initial_picard,
        }
    }
    /// Apply a single MMP step and record it.
    /// Returns `true` if the MMP terminated, `false` if another step is needed.
    pub fn apply(&mut self, step: MmpStep) -> bool {
        let done = step.is_terminal();
        match &step {
            MmpStep::Contraction { .. } => {
                if self.picard_number > 0 {
                    self.picard_number -= 1;
                }
            }
            MmpStep::Flip { .. } | MmpStep::Flop { .. } => {}
            MmpStep::MinimalModel | MmpStep::FiberSpace { .. } => {}
        }
        self.history.push(step);
        done
    }
    /// Run the MMP on a predetermined sequence of steps.
    /// Stops at the first terminal step.
    pub fn run(&mut self, steps: Vec<MmpStep>) -> &[MmpStep] {
        for step in steps {
            let done = self.apply(step);
            if done {
                break;
            }
        }
        &self.history
    }
    /// Summarize the MMP run.
    pub fn summary(&self) -> String {
        let mut s = format!(
            "MMP run on '{}' ({} steps, final Picard number: {}):\n",
            self.pair.variety,
            self.history.len(),
            self.picard_number
        );
        for (i, step) in self.history.iter().enumerate() {
            s.push_str(&format!("  Step {}: {}\n", i + 1, step.description()));
        }
        s
    }
}
/// The type of singularity produced after a contraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingularityType {
    /// Smooth point.
    Smooth,
    /// Terminal singularity: discrepancy > 0 for all exceptional divisors.
    Terminal,
    /// Canonical singularity: discrepancy ≥ 0 for all exceptional divisors.
    Canonical,
    /// Kawamata log-terminal: discrepancy > -1 for log pairs.
    Klt,
    /// Divisorially log-terminal: klt except possibly along reduced boundary.
    Dlt,
    /// Log-canonical: discrepancy ≥ -1.
    LogCanonical,
}
impl SingularityType {
    /// Short name for display.
    pub fn name(&self) -> &'static str {
        match self {
            SingularityType::Smooth => "smooth",
            SingularityType::Terminal => "terminal",
            SingularityType::Canonical => "canonical",
            SingularityType::Klt => "klt",
            SingularityType::Dlt => "dlt",
            SingularityType::LogCanonical => "lc",
        }
    }
    /// Partial order: severity (smooth is mildest, lc is most general).
    /// Returns true if `self` is at least as mild as `other`.
    pub fn at_least_as_mild_as(&self, other: &SingularityType) -> bool {
        let rank = |s: &SingularityType| -> usize {
            match s {
                SingularityType::Smooth => 0,
                SingularityType::Terminal => 1,
                SingularityType::Canonical => 2,
                SingularityType::Klt => 3,
                SingularityType::Dlt => 4,
                SingularityType::LogCanonical => 5,
            }
        };
        rank(self) <= rank(other)
    }
}
/// An extremal ray in the Mori cone NE(X).
///
/// Geometrically, this is a half-line R ⊂ NE(X) satisfying
/// K_X · R < 0 (for the MMP), and any effective curve class
/// on the ray is a positive multiple of the generator.
#[derive(Debug, Clone)]
pub struct ExtremeRay {
    /// Name or description of the ray generator.
    pub generator: String,
    /// Degree K_X · C for a generator C (should be < 0 for K-negative rays).
    pub k_degree: i64,
    /// The type of the associated contraction (small, divisorial, or fiber).
    pub contraction_type: ContractionType,
}
impl ExtremeRay {
    /// Create a new extremal ray.
    pub fn new(generator: impl Into<String>, k_degree: i64, ty: ContractionType) -> Self {
        ExtremeRay {
            generator: generator.into(),
            k_degree,
            contraction_type: ty,
        }
    }
    /// Whether the ray is K-negative (required for MMP).
    pub fn is_k_negative(&self) -> bool {
        self.k_degree < 0
    }
    /// The MMP step corresponding to this ray.
    pub fn mmp_step(&self) -> MmpStep {
        match &self.contraction_type {
            ContractionType::Divisorial => MmpStep::Contraction {
                divisor: self.generator.clone(),
                singularity_type: SingularityType::Terminal,
            },
            ContractionType::Small => MmpStep::Flip {
                locus: self.generator.clone(),
            },
            ContractionType::FiberType => MmpStep::FiberSpace {
                base: format!("base({})", self.generator),
                fiber_dim: 1,
            },
        }
    }
}
