//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// The Hopf fibration: S¹ → S³ → S² (as a named structure).
pub struct HopfFibration;
impl HopfFibration {
    /// The fiber sequence of the Hopf fibration.
    pub fn fiber_sequence() -> FiberSequence {
        FiberSequence::hopf()
    }
    /// The Hopf invariant of the Hopf fibration is 1.
    pub fn hopf_invariant() -> i32 {
        1
    }
}
/// A witnessed path between two values of a type (Rust-level, propositional).
///
/// In a dynamically-typed proof assistant we represent a path as a relation
/// token — concrete path terms live in the kernel expression language.
#[derive(Debug, Clone)]
pub struct IdentityType {
    /// Name of the type A
    pub type_name: String,
    /// Left endpoint (a : A) as a string representation
    pub left: String,
    /// Right endpoint (b : A) as a string representation
    pub right: String,
}
impl IdentityType {
    /// Construct the reflexivity path refl_a : a = a.
    pub fn refl(type_name: impl Into<String>, a: impl Into<String>) -> Self {
        let a = a.into();
        Self {
            type_name: type_name.into(),
            left: a.clone(),
            right: a,
        }
    }
    /// Check if this is a reflexivity path.
    pub fn is_refl(&self) -> bool {
        self.left == self.right
    }
}
/// The circle S¹ as a Rust ADT.
///
/// In full HoTT the circle is a HIT; here we represent elements as
/// either the base point or an angle (for computational purposes).
#[derive(Debug, Clone, PartialEq)]
pub enum Circle {
    /// The base point
    Base,
    /// A point on the circle parameterized by angle (in [0, 2π))
    Point(f64),
}
impl Circle {
    /// The unique loop at base: represents the generator of π₁(S¹) ≅ ℤ.
    pub fn loop_path() -> IdentityType {
        IdentityType::refl("Circle", "base")
    }
}
/// Computes (or looks up) the n-th homotopy group of common spaces.
#[allow(dead_code)]
pub struct HomotopyGroupComputer;
#[allow(dead_code)]
impl HomotopyGroupComputer {
    /// Return the description of π_n(S^k) for small n, k.
    ///
    /// Uses the known tables of homotopy groups of spheres.
    pub fn sphere_homotopy_group(n: u32, k: u32) -> String {
        match (n, k) {
            (0, _) => "0".to_string(),
            (_, 0) => "0".to_string(),
            (n, k) if n < k => "0".to_string(),
            (n, k) if n == k => "Z".to_string(),
            (2, 1) => "Z".to_string(),
            (3, 2) => "Z".to_string(),
            (4, 2) => "Z/2".to_string(),
            (5, 2) => "Z/2".to_string(),
            (4, 3) => "Z/2".to_string(),
            (5, 3) => "Z/2".to_string(),
            (6, 3) => "Z/12".to_string(),
            (5, 4) => "Z/2".to_string(),
            _ => "?".to_string(),
        }
    }
    /// Check if the given space name is simply connected (π₁ = 0).
    pub fn is_simply_connected(space: &str) -> bool {
        matches!(
            space,
            "S2" | "S3" | "S4" | "CP2" | "HP2" | "K3" | "SuspCircle"
        )
    }
    /// Return the first non-trivial homotopy group dimension of a space.
    pub fn first_non_trivial(space: &str) -> Option<u32> {
        match space {
            "Circle" | "S1" => Some(1),
            "S2" | "CP1" => Some(2),
            "S3" | "SU2" => Some(3),
            "Torus" => Some(1),
            "KleinBottle" => Some(1),
            _ => None,
        }
    }
}
/// Path composition p · q.
pub struct PathComposition {
    pub p: IdentityType,
    pub q: IdentityType,
}
impl PathComposition {
    /// Compose paths p : a = b and q : b = c to get a = c.
    /// Returns None if the endpoints don't match (b ≠ b').
    pub fn compose(p: IdentityType, q: IdentityType) -> Option<IdentityType> {
        if p.right == q.left && p.type_name == q.type_name {
            Some(IdentityType {
                type_name: p.type_name.clone(),
                left: p.left.clone(),
                right: q.right.clone(),
            })
        } else {
            None
        }
    }
}
/// Homotopy level of a type in the HoTT hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HomotopyLevel {
    /// (-2): contractible — essentially unique object
    Contr,
    /// (-1): mere proposition — all elements equal
    HProp,
    /// 0: h-set — all paths between elements are equal
    HSet,
    /// 1-groupoid: all 2-paths trivial
    OneGroupoid,
    /// 2-groupoid: all 3-paths trivial
    TwoGroupoid,
    /// n-groupoid for n ≥ 3
    NGrpd(u32),
    /// ∞-groupoid: no truncation
    Infty,
}
impl HomotopyLevel {
    /// The successor homotopy level.
    pub fn succ(&self) -> Self {
        match self {
            HomotopyLevel::Contr => HomotopyLevel::HProp,
            HomotopyLevel::HProp => HomotopyLevel::HSet,
            HomotopyLevel::HSet => HomotopyLevel::OneGroupoid,
            HomotopyLevel::OneGroupoid => HomotopyLevel::TwoGroupoid,
            HomotopyLevel::TwoGroupoid => HomotopyLevel::NGrpd(3),
            HomotopyLevel::NGrpd(n) => HomotopyLevel::NGrpd(n + 1),
            HomotopyLevel::Infty => HomotopyLevel::Infty,
        }
    }
    /// As an integer (where Contr = -2, HProp = -1, HSet = 0, ...).
    pub fn as_int(&self) -> Option<i32> {
        match self {
            HomotopyLevel::Contr => Some(-2),
            HomotopyLevel::HProp => Some(-1),
            HomotopyLevel::HSet => Some(0),
            HomotopyLevel::OneGroupoid => Some(1),
            HomotopyLevel::TwoGroupoid => Some(2),
            HomotopyLevel::NGrpd(n) => Some(*n as i32),
            HomotopyLevel::Infty => None,
        }
    }
}
/// The fundamental group π₁(X, x): elements are loop homotopy classes.
pub struct FundamentalGroup {
    /// The base space name
    pub space: String,
    /// The base point name
    pub base_point: String,
    /// Generators of the fundamental group (names)
    pub generators: Vec<String>,
    /// Relations among generators (in group presentation form)
    pub relations: Vec<String>,
}
impl FundamentalGroup {
    /// The fundamental group of the circle: π₁(S¹) ≅ ℤ (one generator, no relations).
    pub fn of_circle() -> Self {
        Self {
            space: "Circle".to_string(),
            base_point: "base".to_string(),
            generators: vec!["loop".to_string()],
            relations: vec![],
        }
    }
    /// The fundamental group of the torus T²: ℤ × ℤ (abelian, two generators).
    pub fn of_torus() -> Self {
        Self {
            space: "Torus".to_string(),
            base_point: "base".to_string(),
            generators: vec!["p".to_string(), "q".to_string()],
            relations: vec!["p·q = q·p".to_string()],
        }
    }
}
/// Univalent fibration over a type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnivalentFibration {
    pub base_type: String,
    pub fiber_type: String,
    pub is_univalent: bool,
}
#[allow(dead_code)]
impl UnivalentFibration {
    pub fn new(base: &str, fiber: &str) -> Self {
        UnivalentFibration {
            base_type: base.to_string(),
            fiber_type: fiber.to_string(),
            is_univalent: false,
        }
    }
    pub fn univalent(base: &str, fiber: &str) -> Self {
        UnivalentFibration {
            base_type: base.to_string(),
            fiber_type: fiber.to_string(),
            is_univalent: true,
        }
    }
    /// Universe fibration U: Σ (A : U), A → U.
    pub fn universe_fibration() -> Self {
        UnivalentFibration::univalent("U", "El")
    }
    pub fn path_fibration(a: &str) -> Self {
        UnivalentFibration::univalent(a, &format!("{a}=x for x:{a}"))
    }
}
/// Pushout type (homotopy pushout).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PushoutType {
    pub type_a: String,
    pub type_b: String,
    pub type_c: String,
    pub left_map: String,
    pub right_map: String,
}
#[allow(dead_code)]
impl PushoutType {
    pub fn new(a: &str, b: &str, c: &str, f: &str, g: &str) -> Self {
        PushoutType {
            type_a: a.to_string(),
            type_b: b.to_string(),
            type_c: c.to_string(),
            left_map: f.to_string(),
            right_map: g.to_string(),
        }
    }
    /// A + B = pushout over ⊥ (initial type).
    pub fn coproduct(a: &str, b: &str) -> Self {
        PushoutType::new(a, b, "0", "!", "!")
    }
    pub fn suspension(a: &str) -> Self {
        PushoutType::new("1", "1", a, "const_*", "const_*")
    }
}
/// Describes an Eilenberg–MacLane space K(G, n) and its properties.
#[allow(dead_code)]
pub struct EilenbergMacLaneSpace {
    /// The coefficient group name (e.g. "Z", "Z/2", "Z/nZ")
    pub group: String,
    /// The dimension n
    pub n: u32,
}
#[allow(dead_code)]
impl EilenbergMacLaneSpace {
    /// Create a new K(G, n) description.
    pub fn new(group: impl Into<String>, n: u32) -> Self {
        Self {
            group: group.into(),
            n,
        }
    }
    /// K(ℤ, 1) = S¹
    pub fn k_z_1() -> Self {
        Self::new("Z", 1)
    }
    /// K(ℤ, 2) = CP∞ (infinite complex projective space)
    pub fn k_z_2() -> Self {
        Self::new("Z", 2)
    }
    /// K(ℤ/2, 1) = RP∞ (infinite real projective space)
    pub fn k_z2_1() -> Self {
        Self::new("Z/2", 1)
    }
    /// Return the traditional name for this K(G, n), if known.
    pub fn traditional_name(&self) -> Option<&'static str> {
        match (self.group.as_str(), self.n) {
            ("Z", 1) => Some("S1"),
            ("Z", 2) => Some("CP_inf"),
            ("Z/2", 1) => Some("RP_inf"),
            ("Z/2", 2) => Some("BSO"),
            _ => None,
        }
    }
    /// The n-th cohomology of K(G, n) with coefficients in G.
    ///
    /// By the universal coefficient theorem / Brown representability:
    /// H^n(K(G,n); G) = G.
    pub fn top_cohomology_description(&self) -> String {
        format!(
            "H^{}(K({},{}); {}) = {}",
            self.n, self.group, self.n, self.group, self.group
        )
    }
    /// Describe the loop space: Ω K(G, n) ≃ K(G, n-1) for n ≥ 2.
    pub fn loop_space(&self) -> Option<EilenbergMacLaneSpace> {
        if self.n == 0 {
            None
        } else {
            Some(EilenbergMacLaneSpace::new(self.group.clone(), self.n - 1))
        }
    }
    /// Delooping: B K(G, n) = K(G, n+1) (the classifying space).
    pub fn delooping(&self) -> EilenbergMacLaneSpace {
        EilenbergMacLaneSpace::new(self.group.clone(), self.n + 1)
    }
}
/// Suspension type constructor Σ A.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SuspensionType {
    pub base: String,
    pub north_pole: String,
    pub south_pole: String,
    pub merid_constructor: String,
}
#[allow(dead_code)]
impl SuspensionType {
    pub fn new(a: &str) -> Self {
        SuspensionType {
            base: a.to_string(),
            north_pole: "N".to_string(),
            south_pole: "S".to_string(),
            merid_constructor: format!("merid : {a} → N = S"),
        }
    }
    pub fn circle() -> Self {
        SuspensionType::new("Bool")
    }
    pub fn sphere_n(n: usize) -> Self {
        SuspensionType::new(&format!("S^{}", n - 1))
    }
    pub fn homotopy_groups_are_interesting(&self) -> bool {
        true
    }
}
/// The n-th homotopy group π_n(X, x) as an abstract group presentation.
pub struct HomotopyGroups {
    /// The space
    pub space: String,
    /// The base point
    pub base_point: String,
    /// The homotopy level n
    pub n: u32,
    /// Description of the group (e.g. "Z", "Z/2", "0")
    pub description: String,
}
impl HomotopyGroups {
    /// π₁(S¹) = ℤ
    pub fn circle_pi1() -> Self {
        Self {
            space: "Circle".to_string(),
            base_point: "base".to_string(),
            n: 1,
            description: "Z".to_string(),
        }
    }
    /// π₂(S²) = ℤ (Hurewicz theorem)
    pub fn s2_pi2() -> Self {
        Self {
            space: "S2".to_string(),
            base_point: "north".to_string(),
            n: 2,
            description: "Z".to_string(),
        }
    }
    /// π₃(S²) = ℤ (non-trivial, detected by Hopf fibration)
    pub fn s2_pi3() -> Self {
        Self {
            space: "S2".to_string(),
            base_point: "north".to_string(),
            n: 3,
            description: "Z".to_string(),
        }
    }
}
/// Composes a sequence of paths with explicit associativity tracking.
///
/// Maintains a composition chain and checks endpoint matching at each step.
#[allow(dead_code)]
pub struct PathCompositionChain {
    /// The current composed path (None if empty chain)
    pub current: Option<IdentityType>,
    /// The number of paths composed so far
    pub length: usize,
}
#[allow(dead_code)]
impl PathCompositionChain {
    /// Start a new empty composition chain.
    pub fn new() -> Self {
        Self {
            current: None,
            length: 0,
        }
    }
    /// Extend the chain by composing with another path.
    /// Returns false if the endpoints do not match.
    pub fn extend(&mut self, p: IdentityType) -> bool {
        match self.current.take() {
            None => {
                self.current = Some(p);
                self.length = 1;
                true
            }
            Some(prev) => match PathComposition::compose(prev.clone(), p) {
                Some(composed) => {
                    self.current = Some(composed);
                    self.length += 1;
                    true
                }
                None => {
                    self.current = Some(prev);
                    false
                }
            },
        }
    }
    /// Retrieve the composed result, if any.
    pub fn result(&self) -> Option<&IdentityType> {
        self.current.as_ref()
    }
    /// Check whether the resulting path is reflexivity.
    pub fn is_loop(&self) -> bool {
        self.current.as_ref().map_or(false, |p| p.is_refl())
    }
}
/// Transport along a path (dependent substitution).
pub struct Transport;
impl Transport {
    /// Transport a term description along a path: returns the transported name.
    pub fn transport(path: &IdentityType, term: impl Into<String>) -> String {
        format!("transport({} = {}, {})", path.left, path.right, term.into())
    }
}
/// Cohomology operation in HoTT (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CohomologyOp {
    pub name: String,
    pub source_degree: i64,
    pub target_degree: i64,
    pub is_stable: bool,
}
#[allow(dead_code)]
impl CohomologyOp {
    pub fn new(name: &str, src: i64, tgt: i64, stable: bool) -> Self {
        CohomologyOp {
            name: name.to_string(),
            source_degree: src,
            target_degree: tgt,
            is_stable: stable,
        }
    }
    pub fn steenrod_sq(i: i64) -> Self {
        CohomologyOp::new(&format!("Sq^{}", i), 0, i, true)
    }
    pub fn is_primary(&self) -> bool {
        true
    }
    pub fn degree_shift(&self) -> i64 {
        self.target_degree - self.source_degree
    }
}
/// The suspension ΣA.
#[derive(Debug, Clone)]
pub enum Suspension<A> {
    /// The north pole N
    North,
    /// The south pole S
    South,
    /// A meridian point (parameterized by a : A and t ∈ [0,1])
    Merid(A, f64),
}
/// The Blakers-Massey theorem: a connectivity estimate for pushouts.
pub struct BlakerssMasseyThm {
    /// Connectivity of f
    pub m: u32,
    /// Connectivity of g
    pub n: u32,
}
impl BlakerssMasseyThm {
    /// State the theorem: the pushout map is (m+n)-connected.
    pub fn new(m: u32, n: u32) -> Self {
        Self { m, n }
    }
    /// Connectivity of the resulting pushout comparison map.
    pub fn pushout_connectivity(&self) -> u32 {
        self.m + self.n
    }
}
/// A homotopy equivalence A ≃ B: a function with quasi-inverse and homotopies.
pub struct HomotopyEquivalence {
    /// Domain type name
    pub domain: String,
    /// Codomain type name
    pub codomain: String,
    /// Forward function name
    pub fwd: String,
    /// Backward (quasi-inverse) function name
    pub bwd: String,
    /// Left homotopy name: bwd ∘ fwd ~ id
    pub left_htpy: String,
    /// Right homotopy name: fwd ∘ bwd ~ id
    pub right_htpy: String,
}
impl HomotopyEquivalence {
    /// Construct an equivalence record.
    pub fn new(
        domain: impl Into<String>,
        codomain: impl Into<String>,
        fwd: impl Into<String>,
        bwd: impl Into<String>,
    ) -> Self {
        let fwd = fwd.into();
        let bwd = bwd.into();
        let lh = format!("{}_{}_left_htpy", bwd, fwd);
        let rh = format!("{}_{}_right_htpy", fwd, bwd);
        Self {
            domain: domain.into(),
            codomain: codomain.into(),
            left_htpy: lh,
            right_htpy: rh,
            fwd,
            bwd,
        }
    }
    /// The identity equivalence A ≃ A.
    pub fn id(ty: impl Into<String>) -> Self {
        let ty = ty.into();
        Self::new(ty.clone(), ty.clone(), "id", "id")
    }
    /// Compose two equivalences: A ≃ B and B ≃ C gives A ≃ C.
    pub fn compose(self, other: Self) -> Option<Self> {
        if self.codomain != other.domain {
            return None;
        }
        let fwd = format!("{} ∘ {}", other.fwd, self.fwd);
        let bwd = format!("{} ∘ {}", self.bwd, other.bwd);
        Some(HomotopyEquivalence::new(
            self.domain,
            other.codomain,
            fwd,
            bwd,
        ))
    }
}
/// Represents a step in the long exact sequence of homotopy groups
/// for a fibration F → E → B.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LESEntry {
    /// The space name (F, E, or B)
    pub space: String,
    /// The homotopy group index n
    pub n: u32,
    /// The group description
    pub group: String,
}
/// A fiber sequence F → E → B.
pub struct FiberSequence {
    /// The fiber type name
    pub fiber: String,
    /// The total space type name
    pub total_space: String,
    /// The base type name
    pub base: String,
    /// The inclusion map name
    pub inclusion: String,
    /// The projection map name
    pub projection: String,
}
impl FiberSequence {
    /// The Hopf fibration S¹ → S³ → S².
    pub fn hopf() -> Self {
        Self {
            fiber: "Circle".to_string(),
            total_space: "S3".to_string(),
            base: "S2".to_string(),
            inclusion: "hopf_incl".to_string(),
            projection: "hopf_proj".to_string(),
        }
    }
}
/// The n-truncation of a type (propositional level).
#[derive(Debug, Clone)]
pub struct Truncation<A> {
    /// The truncation level n
    pub level: i32,
    /// The underlying element (after truncation, equalities are collapsed)
    pub element: A,
}
impl<A> Truncation<A> {
    /// Introduce an element into the n-truncation.
    pub fn into_trunc(level: i32, a: A) -> Self {
        Self { level, element: a }
    }
    /// The propositional truncation (‖A‖_{-1}).
    pub fn prop_trunc(a: A) -> Self {
        Self::into_trunc(-1, a)
    }
    /// The set truncation (‖A‖_0).
    pub fn set_trunc(a: A) -> Self {
        Self::into_trunc(0, a)
    }
}
/// Computes the effective homotopy truncation level of a described type.
///
/// Given a type's name and a list of known generator dimensions,
/// returns the minimum homotopy level at which the type is truncated.
#[allow(dead_code)]
pub struct TruncationComputer {
    /// The type being analyzed
    pub type_name: String,
    /// Generator dimensions (loop dimensions present in the type)
    pub generator_dims: Vec<u32>,
}
#[allow(dead_code)]
impl TruncationComputer {
    /// Create a new truncation computer for a named type.
    pub fn new(type_name: impl Into<String>, generator_dims: Vec<u32>) -> Self {
        Self {
            type_name: type_name.into(),
            generator_dims,
        }
    }
    /// Compute the homotopy level: max generator dimension + 1,
    /// or Contr (-2) if no generators, HProp (-1) if only 0-dim generators.
    pub fn truncation_level(&self) -> HomotopyLevel {
        if self.generator_dims.is_empty() {
            return HomotopyLevel::Contr;
        }
        let max_dim = self.generator_dims.iter().copied().max().unwrap_or(0);
        match max_dim {
            0 => HomotopyLevel::HProp,
            1 => HomotopyLevel::HSet,
            2 => HomotopyLevel::OneGroupoid,
            3 => HomotopyLevel::TwoGroupoid,
            n => HomotopyLevel::NGrpd(n),
        }
    }
    /// Check if the type is at most a set (0-truncated).
    pub fn is_set(&self) -> bool {
        matches!(
            self.truncation_level(),
            HomotopyLevel::Contr | HomotopyLevel::HProp | HomotopyLevel::HSet
        )
    }
    /// Check if the type is a mere proposition.
    pub fn is_prop(&self) -> bool {
        matches!(
            self.truncation_level(),
            HomotopyLevel::Contr | HomotopyLevel::HProp
        )
    }
    /// Describe the type's truncation in HoTT notation.
    pub fn describe(&self) -> String {
        match self.truncation_level().as_int() {
            Some(-2) => format!("{} is contractible", self.type_name),
            Some(-1) => format!("{} is a mere proposition", self.type_name),
            Some(0) => format!("{} is a set (0-type)", self.type_name),
            Some(1) => format!("{} is a 1-groupoid", self.type_name),
            Some(n) => format!("{} is a {}-type", self.type_name, n),
            None => format!("{} is an ∞-groupoid", self.type_name),
        }
    }
}
/// Computes entries of the long exact sequence for a fiber sequence.
#[allow(dead_code)]
pub struct FibrationSequenceComputer {
    /// The fiber type name
    pub fiber: String,
    /// The total space type name
    pub total: String,
    /// The base type name
    pub base: String,
}
#[allow(dead_code)]
impl FibrationSequenceComputer {
    /// Create a new fibration sequence computer.
    pub fn new(
        fiber: impl Into<String>,
        total: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            fiber: fiber.into(),
            total: total.into(),
            base: base.into(),
        }
    }
    /// Compute the Hopf fibration long exact sequence for π_n, n = 1..=4.
    ///
    /// Returns a list of LES entries: ... → π_n(F) → π_n(E) → π_n(B) → π_{n-1}(F) → ...
    pub fn hopf_les(&self) -> Vec<LESEntry> {
        vec![
            LESEntry {
                space: "S3".to_string(),
                n: 4,
                group: "Z/2".to_string(),
            },
            LESEntry {
                space: "S2".to_string(),
                n: 4,
                group: "Z/2".to_string(),
            },
            LESEntry {
                space: "S1".to_string(),
                n: 3,
                group: "0".to_string(),
            },
            LESEntry {
                space: "S3".to_string(),
                n: 3,
                group: "Z".to_string(),
            },
            LESEntry {
                space: "S2".to_string(),
                n: 3,
                group: "Z".to_string(),
            },
            LESEntry {
                space: "S1".to_string(),
                n: 2,
                group: "0".to_string(),
            },
            LESEntry {
                space: "S3".to_string(),
                n: 2,
                group: "0".to_string(),
            },
            LESEntry {
                space: "S2".to_string(),
                n: 2,
                group: "Z".to_string(),
            },
            LESEntry {
                space: "S1".to_string(),
                n: 1,
                group: "Z".to_string(),
            },
            LESEntry {
                space: "S3".to_string(),
                n: 1,
                group: "0".to_string(),
            },
        ]
    }
    /// Count the total number of non-trivial groups in the LES (for the Hopf case).
    pub fn non_trivial_count(&self) -> usize {
        self.hopf_les().iter().filter(|e| e.group != "0").count()
    }
}
/// Witness that a type is contractible: a center and a contraction.
pub struct IsContr {
    /// The center of contraction
    pub center: String,
    /// A description of the contraction path function
    pub contraction: String,
}
impl IsContr {
    /// Build a contractibility witness from center and contraction description.
    pub fn new(center: impl Into<String>, contraction: impl Into<String>) -> Self {
        Self {
            center: center.into(),
            contraction: contraction.into(),
        }
    }
}
/// Path inversion p^{-1}.
pub struct PathInversion;
impl PathInversion {
    /// Invert a path p : a = b to get p⁻¹ : b = a.
    pub fn invert(p: IdentityType) -> IdentityType {
        IdentityType {
            type_name: p.type_name,
            left: p.right,
            right: p.left,
        }
    }
}
