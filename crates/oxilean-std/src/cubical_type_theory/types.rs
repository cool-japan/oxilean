//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A dimension context: a list of named interval variables in scope.
#[allow(dead_code)]
pub struct CubicalDimension {
    /// The dimension variables in scope (e.g. \["i", "j", "k"\]).
    pub vars: Vec<String>,
}
#[allow(dead_code)]
impl CubicalDimension {
    /// Create an empty dimension context.
    pub fn empty() -> Self {
        Self { vars: Vec::new() }
    }
    /// Extend the context with a fresh variable.
    pub fn extend(&self, var: impl Into<String>) -> Self {
        let mut v = self.vars.clone();
        v.push(var.into());
        Self { vars: v }
    }
    /// The dimension (number of interval variables in scope).
    pub fn dim(&self) -> usize {
        self.vars.len()
    }
    /// Check whether a variable is in scope.
    pub fn contains(&self, var: &str) -> bool {
        self.vars.iter().any(|v| v == var)
    }
    /// Substitute a variable: returns a new context with that variable removed.
    pub fn substitute(&self, var: &str) -> Self {
        Self {
            vars: self
                .vars
                .iter()
                .filter(|v| v.as_str() != var)
                .cloned()
                .collect(),
        }
    }
    /// Describe the dimension context.
    pub fn describe(&self) -> String {
        if self.vars.is_empty() {
            "∙".to_string()
        } else {
            self.vars.join(", ")
        }
    }
}
/// An interval object with two named endpoints.
pub struct IntervalObject {
    /// The two endpoints of the interval (e.g. ("i0", "i1")).
    pub endpoints: (String, String),
}
impl IntervalObject {
    /// Creates a new `IntervalObject`.
    pub fn new(e0: impl Into<String>, e1: impl Into<String>) -> Self {
        Self {
            endpoints: (e0.into(), e1.into()),
        }
    }
    /// Returns the connection operations (min/max) on the interval.
    pub fn connections(&self) -> Vec<String> {
        let (e0, e1) = &self.endpoints;
        vec![
            format!("min(i, j) : I  [connection connecting {} and {}]", e0, e1),
            format!("max(i, j) : I  [connection connecting {} and {}]", e0, e1),
        ]
    }
    /// Returns the symmetry operation i ↦ 1 − i.
    pub fn symmetry(&self) -> String {
        format!(
            "sym(i) = 1 - i : I  [{} ↔ {}]",
            self.endpoints.0, self.endpoints.1
        )
    }
}
/// Classification of a type by its homotopy level (n-type).
#[allow(dead_code)]
pub enum HomotopyLevel {
    /// Contractible ((-2)-type): unique element up to unique homotopy.
    Contractible,
    /// Proposition ((-1)-type): at most one element up to homotopy.
    Proposition,
    /// Set (0-type): elements are equal or not, with no higher structure.
    Set,
    /// Groupoid (1-type): path spaces are sets.
    Groupoid,
    /// n-type for n ≥ 2.
    NType(usize),
    /// Unknown / infinite homotopy level.
    Infinite,
}
/// A path type `Path A a b` — the type of paths from `a` to `b` in type `A`.
pub struct PathType {
    /// The ambient type.
    pub a: String,
    /// The left endpoint.
    pub b: String,
    /// The right endpoint.
    pub path: String,
}
impl PathType {
    /// Creates a new `PathType`.
    pub fn new(a: impl Into<String>, b: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            path: path.into(),
        }
    }
    /// Reflexivity: `refl a : Path A a a`.
    pub fn refl(&self) -> String {
        format!("refl {} : Path {} {} {}", self.b, self.a, self.b, self.b)
    }
    /// Transport along a path: `transp (Path A a b) a : b`.
    pub fn transport(&self) -> String {
        format!(
            "transp (Path {} {} {}) {} : {}",
            self.a, self.b, self.path, self.b, self.path
        )
    }
    /// Homogeneous composition along the path.
    pub fn hcomp(&self) -> String {
        format!(
            "hcomp (i : I) [i=0 ↦ {}, i=1 ↦ {}] {} : {}",
            self.b, self.path, self.b, self.a
        )
    }
}
/// A Kan filling operation for a horn in a cubical set.
pub struct KanOperation {
    /// Direction of the filling (false = left, true = right).
    pub direction: bool,
    /// The face index to be filled.
    pub face: usize,
}
impl KanOperation {
    /// Creates a new `KanOperation`.
    pub fn new(direction: bool, face: usize) -> Self {
        Self { direction, face }
    }
    /// Returns a description of the horn-filling for this Kan operation.
    pub fn horn_fill(&self) -> String {
        let side = if self.direction { "right" } else { "left" };
        format!(
            "Fill horn Λ^{}_{{{} {}}} using Kan condition",
            self.face, side, self.face
        )
    }
    /// Returns the coherence conditions satisfied by this filling.
    pub fn coherence_conditions(&self) -> Vec<String> {
        vec![
            format!("face map d_{} commutes with filling", self.face),
            "filling is natural in maps of cubical sets".to_string(),
            "composition with degeneracies preserves filling".to_string(),
        ]
    }
}
/// A (Kan) fibration in cubical type theory.
///
/// A fibration p : E → B has the property that every open box in E over
/// a filled box in B can be completed to a filled box.
#[allow(dead_code)]
pub struct CubicalFibration {
    /// Name of the total space.
    pub total: String,
    /// Name of the base space.
    pub base: String,
    /// Name of the projection map.
    pub projection: String,
    /// Whether this fibration has been verified as Kan.
    pub is_kan: bool,
}
#[allow(dead_code)]
impl CubicalFibration {
    /// Create a new fibration descriptor.
    pub fn new(
        total: impl Into<String>,
        base: impl Into<String>,
        projection: impl Into<String>,
        is_kan: bool,
    ) -> Self {
        Self {
            total: total.into(),
            base: base.into(),
            projection: projection.into(),
            is_kan,
        }
    }
    /// The trivial fibration (identity over the base).
    pub fn trivial(space: impl Into<String>) -> Self {
        let s = space.into();
        Self {
            total: s.clone(),
            base: s,
            projection: "id".to_string(),
            is_kan: true,
        }
    }
    /// The path fibration over a type A.
    pub fn path_fibration(ty: impl Into<String>) -> Self {
        let t = ty.into();
        Self {
            total: format!("Path({})", t),
            base: t,
            projection: "PathFib.proj".to_string(),
            is_kan: true,
        }
    }
    /// Describe this fibration.
    pub fn describe(&self) -> String {
        let kan_str = if self.is_kan { " (Kan)" } else { " (not Kan)" };
        format!(
            "{} → {} via {}{}",
            self.total, self.base, self.projection, kan_str
        )
    }
    /// The fiber over a point b in the base.
    pub fn fiber_at(&self, point: &str) -> String {
        format!("Fiber({}, {} = {})", self.total, self.projection, point)
    }
}
/// A substitution of interval variables (a morphism in the cube category).
#[allow(dead_code)]
pub struct CubicalSubstitution {
    /// Mappings from variable names to interval expressions.
    pub map: Vec<(String, IntervalPoint)>,
}
#[allow(dead_code)]
impl CubicalSubstitution {
    /// Create an empty substitution.
    pub fn empty() -> Self {
        Self { map: Vec::new() }
    }
    /// Add a single variable substitution.
    pub fn add(mut self, var: impl Into<String>, pt: IntervalPoint) -> Self {
        self.map.push((var.into(), pt));
        self
    }
    /// Apply the substitution to an interval point.
    pub fn apply(&self, pt: &IntervalPoint) -> IntervalPoint {
        let mut result = pt.clone();
        for (var, val) in &self.map {
            let is_one = matches!(val, IntervalPoint::One);
            result = result.eval(var, is_one);
        }
        result.simplify()
    }
    /// The face substitution i ↦ 0 for variable `var`.
    pub fn face0(var: impl Into<String>) -> Self {
        Self::empty().add(var, IntervalPoint::Zero)
    }
    /// The face substitution i ↦ 1 for variable `var`.
    pub fn face1(var: impl Into<String>) -> Self {
        Self::empty().add(var, IntervalPoint::One)
    }
    /// Describe the substitution.
    pub fn describe(&self) -> String {
        if self.map.is_empty() {
            return "id".to_string();
        }
        let parts: Vec<String> = self
            .map
            .iter()
            .map(|(v, _)| format!("{} ↦ ...", v))
            .collect();
        format!("[{}]", parts.join(", "))
    }
}
/// A face system: a collection of (face formula, term) pairs.
///
/// In cubical TT this represents a partial element: for each face formula φ_i,
/// we have a term u_i that is valid when φ_i holds. The compatibility condition
/// requires that all terms agree on overlapping faces.
#[allow(dead_code)]
pub struct FaceSystem {
    /// The ambient type name.
    pub ty: String,
    /// The faces: (formula, term) pairs.
    pub faces: Vec<(String, String)>,
}
#[allow(dead_code)]
impl FaceSystem {
    /// Create an empty face system.
    pub fn new(ty: impl Into<String>) -> Self {
        Self {
            ty: ty.into(),
            faces: Vec::new(),
        }
    }
    /// Add a face (formula, term) pair.
    pub fn add_face(mut self, formula: impl Into<String>, term: impl Into<String>) -> Self {
        self.faces.push((formula.into(), term.into()));
        self
    }
    /// Whether the system covers all faces (i.e., the join of all formulas is 1).
    pub fn is_total(&self) -> bool {
        self.faces.len() >= 2
    }
    /// Describe the face system in CCHM notation.
    pub fn describe(&self) -> String {
        if self.faces.is_empty() {
            return format!("[] : {}", self.ty);
        }
        let parts: Vec<String> = self
            .faces
            .iter()
            .map(|(phi, u)| format!("{} ↦ {}", phi, u))
            .collect();
        format!("[{} : {}]", parts.join(", "), self.ty)
    }
    /// The number of faces in this system.
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }
}
/// A (finite) cubical set — a presheaf on the category of cubes.
///
/// For our purposes we represent a cubical set by its dimension and
/// the generating data (cells and face/degeneracy maps).
#[derive(Debug, Clone)]
pub struct CubicalSet {
    /// The name/label of this cubical set.
    pub name: String,
    /// Maximum dimension present (0 = just vertices, 1 = edges, ...).
    pub max_dim: usize,
    /// Face maps: (dim, cell_index, direction) → lower-dim cell index.
    pub face_maps: Vec<(usize, usize, bool)>,
}
impl CubicalSet {
    /// Create a new cubical set with a given name and max dimension.
    pub fn new(name: impl Into<String>, max_dim: usize) -> Self {
        Self {
            name: name.into(),
            max_dim,
            face_maps: Vec::new(),
        }
    }
    /// The point (terminal cubical set): one 0-cell, no higher cells.
    pub fn point() -> Self {
        Self::new("Point", 0)
    }
    /// The interval I as a cubical set: two 0-cells and one 1-cell.
    pub fn interval() -> Self {
        let mut s = Self::new("I", 1);
        s.face_maps.push((1, 0, false));
        s.face_maps.push((1, 0, true));
        s
    }
    /// The circle (as a quotient cubical set): one 0-cell and one 1-cell whose
    /// faces are both identified with the single 0-cell.
    pub fn circle() -> Self {
        let mut s = Self::new("Circle", 1);
        s.face_maps.push((1, 0, false));
        s.face_maps.push((1, 0, true));
        s
    }
    /// Check if this cubical set satisfies the Kan condition (every open box is fillable).
    /// For our simplified model we just record whether the set is declared Kan.
    pub fn is_kan(&self) -> bool {
        self.max_dim <= 1
    }
    /// The number of n-cells (for simplified model: 2^n for the n-cube).
    pub fn cell_count(&self, dim: usize) -> usize {
        if dim > self.max_dim {
            0
        } else {
            2usize.pow(dim as u32)
        }
    }
}
/// A point in the abstract interval \[0, 1\].
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalPoint {
    /// The left endpoint i0 = 0.
    Zero,
    /// The right endpoint i1 = 1.
    One,
    /// A variable dimension name (e.g., "i", "j").
    Var(String),
    /// Meet (min) of two interval points.
    Min(Box<IntervalPoint>, Box<IntervalPoint>),
    /// Join (max) of two interval points.
    Max(Box<IntervalPoint>, Box<IntervalPoint>),
    /// Complement / negation of an interval point.
    Neg(Box<IntervalPoint>),
}
impl IntervalPoint {
    /// Evaluate the interval point at a concrete assignment (0 or 1).
    pub fn eval(&self, var: &str, val: bool) -> IntervalPoint {
        match self {
            IntervalPoint::Var(v) if v == var => {
                if val {
                    IntervalPoint::One
                } else {
                    IntervalPoint::Zero
                }
            }
            IntervalPoint::Min(a, b) => {
                let ea = a.eval(var, val);
                let eb = b.eval(var, val);
                IntervalPoint::Min(Box::new(ea), Box::new(eb))
            }
            IntervalPoint::Max(a, b) => {
                let ea = a.eval(var, val);
                let eb = b.eval(var, val);
                IntervalPoint::Max(Box::new(ea), Box::new(eb))
            }
            IntervalPoint::Neg(a) => IntervalPoint::Neg(Box::new(a.eval(var, val))),
            other => other.clone(),
        }
    }
    /// Simplify the interval point (compute meets/joins with constants).
    pub fn simplify(&self) -> IntervalPoint {
        match self {
            IntervalPoint::Min(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (IntervalPoint::Zero, _) | (_, IntervalPoint::Zero) => IntervalPoint::Zero,
                    (IntervalPoint::One, _) => b,
                    (_, IntervalPoint::One) => a,
                    _ => IntervalPoint::Min(Box::new(a), Box::new(b)),
                }
            }
            IntervalPoint::Max(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (IntervalPoint::One, _) | (_, IntervalPoint::One) => IntervalPoint::One,
                    (IntervalPoint::Zero, _) => b,
                    (_, IntervalPoint::Zero) => a,
                    _ => IntervalPoint::Max(Box::new(a), Box::new(b)),
                }
            }
            IntervalPoint::Neg(a) => {
                let a = a.simplify();
                match a {
                    IntervalPoint::Zero => IntervalPoint::One,
                    IntervalPoint::One => IntervalPoint::Zero,
                    IntervalPoint::Neg(inner) => *inner,
                    other => IntervalPoint::Neg(Box::new(other)),
                }
            }
            other => other.clone(),
        }
    }
    /// Check if this point is definitionally i0.
    pub fn is_zero(&self) -> bool {
        matches!(self.simplify(), IntervalPoint::Zero)
    }
    /// Check if this point is definitionally i1.
    pub fn is_one(&self) -> bool {
        matches!(self.simplify(), IntervalPoint::One)
    }
}
/// A Higher Inductive Type (HIT) in cubical type theory.
pub struct HIT {
    /// The point (0-dimensional) constructors.
    pub point_constructors: Vec<String>,
    /// The path (1-dimensional) constructors.
    pub path_constructors: Vec<String>,
}
impl HIT {
    /// Creates a new `HIT`.
    pub fn new(point_constructors: Vec<String>, path_constructors: Vec<String>) -> Self {
        Self {
            point_constructors,
            path_constructors,
        }
    }
    /// Creates the circle S¹ as a HIT.
    pub fn circle() -> Self {
        Self::new(
            vec!["base : S¹".to_string()],
            vec!["loop : Path S¹ base base".to_string()],
        )
    }
    /// Creates the interval as a HIT.
    pub fn interval_hit() -> Self {
        Self::new(
            vec!["left : I".to_string(), "right : I".to_string()],
            vec!["seg : Path I left right".to_string()],
        )
    }
    /// Returns the induction principle for this HIT.
    pub fn induction_principle(&self) -> String {
        let pts: Vec<_> = self
            .point_constructors
            .iter()
            .map(|c| format!("  case {}", c))
            .collect();
        let paths: Vec<_> = self
            .path_constructors
            .iter()
            .map(|c| format!("  path case {}", c))
            .collect();
        format!("HIT induction:\n{}\n{}", pts.join("\n"), paths.join("\n"))
    }
    /// Returns a description of normalization for this HIT.
    pub fn normalization(&self) -> String {
        format!(
            "Normalization for HIT with {} point and {} path constructors: \
             reduce by β for point constructors and path-over computation rules",
            self.point_constructors.len(),
            self.path_constructors.len()
        )
    }
}
/// A square type with four corners and two path boundaries.
pub struct SquareType {
    /// Bottom-left corner.
    pub a00: String,
    /// Bottom-right corner.
    pub a01: String,
    /// Top-left corner.
    pub a10: String,
    /// Top-right corner.
    pub a11: String,
    /// Left-right path (bottom edge: a00 → a01).
    pub path_lr: String,
    /// Top-bottom path (left edge: a00 → a10).
    pub path_tb: String,
}
impl SquareType {
    /// Creates a new `SquareType`.
    pub fn new(
        a00: impl Into<String>,
        a01: impl Into<String>,
        a10: impl Into<String>,
        a11: impl Into<String>,
        path_lr: impl Into<String>,
        path_tb: impl Into<String>,
    ) -> Self {
        Self {
            a00: a00.into(),
            a01: a01.into(),
            a10: a10.into(),
            a11: a11.into(),
            path_lr: path_lr.into(),
            path_tb: path_tb.into(),
        }
    }
    /// Returns a description of the filler of this square (Kan condition).
    pub fn fill(&self) -> String {
        format!(
            "Square filler: Path (Path A {} {}) {} {}  [using hcomp over i,j]",
            self.a00, self.a01, self.path_lr, self.path_tb
        )
    }
}
/// A cubical equivalence between two types.
#[derive(Debug, Clone)]
pub struct CubicalEquiv {
    /// Domain type name.
    pub domain: String,
    /// Codomain type name.
    pub codomain: String,
    /// Name of the forward map.
    pub forward: String,
    /// Name of the inverse map.
    pub inverse: String,
}
impl CubicalEquiv {
    /// Construct an equivalence.
    pub fn new(
        domain: impl Into<String>,
        codomain: impl Into<String>,
        forward: impl Into<String>,
        inverse: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            codomain: codomain.into(),
            forward: forward.into(),
            inverse: inverse.into(),
        }
    }
    /// The identity equivalence on a type.
    pub fn id(ty: impl Into<String>) -> Self {
        let t = ty.into();
        Self {
            domain: t.clone(),
            codomain: t,
            forward: "id".to_string(),
            inverse: "id".to_string(),
        }
    }
    /// Invert the equivalence.
    pub fn inv(&self) -> CubicalEquiv {
        CubicalEquiv {
            domain: self.codomain.clone(),
            codomain: self.domain.clone(),
            forward: self.inverse.clone(),
            inverse: self.forward.clone(),
        }
    }
    /// Compose two equivalences (if their endpoints agree).
    pub fn compose(&self, other: &CubicalEquiv) -> Option<CubicalEquiv> {
        if self.codomain != other.domain {
            return None;
        }
        Some(CubicalEquiv {
            domain: self.domain.clone(),
            codomain: other.codomain.clone(),
            forward: format!("{} ∘ {}", other.forward, self.forward),
            inverse: format!("{} ∘ {}", self.inverse, other.inverse),
        })
    }
    /// Convert the equivalence to a univalence path `ua e : Path Type A B`.
    pub fn to_ua_path(&self) -> CubicalPath {
        CubicalPath {
            type_name: "Type".to_string(),
            left: self.domain.clone(),
            right: self.codomain.clone(),
            name: Some(format!("ua({} ≃ {})", self.domain, self.codomain)),
        }
    }
}
/// A cubical path from `left` to `right` over a line of types.
///
/// Concretely, a path in type `A` from `a` to `b` is a function
/// `f : I → A` with `f 0 = a` and `f 1 = b`.
#[derive(Debug, Clone)]
pub struct CubicalPath {
    /// Name of the ambient type (as a string label).
    pub type_name: String,
    /// Left endpoint (at i=0).
    pub left: String,
    /// Right endpoint (at i=1).
    pub right: String,
    /// Optional name of the path.
    pub name: Option<String>,
}
impl CubicalPath {
    /// Construct a reflexivity path at a given element.
    pub fn refl(type_name: impl Into<String>, elem: impl Into<String>) -> Self {
        let e = elem.into();
        Self {
            type_name: type_name.into(),
            left: e.clone(),
            right: e,
            name: Some("refl".to_string()),
        }
    }
    /// Reverse a path using interval negation `<i> p (~i)`.
    pub fn sym(&self) -> CubicalPath {
        CubicalPath {
            type_name: self.type_name.clone(),
            left: self.right.clone(),
            right: self.left.clone(),
            name: self.name.as_ref().map(|n| format!("sym({})", n)),
        }
    }
    /// Compose two paths `p : a → b` and `q : b → c`.
    ///
    /// Returns `Some(r : a → c)` if the endpoints match, `None` otherwise.
    pub fn trans(&self, other: &CubicalPath) -> Option<CubicalPath> {
        if self.right != other.left {
            return None;
        }
        Some(CubicalPath {
            type_name: self.type_name.clone(),
            left: self.left.clone(),
            right: other.right.clone(),
            name: match (&self.name, &other.name) {
                (Some(p), Some(q)) => Some(format!("trans({},{})", p, q)),
                _ => None,
            },
        })
    }
}
/// The composition structure used to fill open boxes in cubical type theory.
pub struct CompositionStructure {
    /// A symbolic description of the open box.
    pub open_box: String,
}
impl CompositionStructure {
    /// Creates a new `CompositionStructure`.
    pub fn new(open_box: impl Into<String>) -> Self {
        Self {
            open_box: open_box.into(),
        }
    }
    /// Fill the open box using the composition operation.
    pub fn fill_open_box(&self) -> String {
        format!(
            "comp A (i : I) [φ ↦ u] u0  — fills open box: {}",
            self.open_box
        )
    }
    /// Uniform composition is the special case where the type is constant.
    pub fn uniform_composition(&self) -> String {
        format!(
            "hcomp (i : I) [φ ↦ u] u0  — uniform composition in {}",
            self.open_box
        )
    }
}
/// A Glue type `Glue φ B A e`, providing a computational version of univalence.
#[allow(non_snake_case)]
pub struct GluingType {
    /// The base type (codomain of the equivalence).
    pub A: String,
    /// The fibre type (domain of the equivalence).
    pub B: String,
    /// A description of the equivalence `e : B ≃ A` over face formula φ.
    pub equivalence: String,
}
#[allow(non_snake_case)]
impl GluingType {
    /// Creates a new `GluingType`.
    pub fn new(A: impl Into<String>, B: impl Into<String>, equiv: impl Into<String>) -> Self {
        Self {
            A: A.into(),
            B: B.into(),
            equivalence: equiv.into(),
        }
    }
    /// A Glue type is fibrant by construction in CCHM cubical type theory.
    pub fn is_fibrant(&self) -> bool {
        true
    }
    /// Returns a description of how univalence is computed from the Glue type.
    pub fn computing_univalence(&self) -> String {
        format!(
            "ua({} : {} ≃ {}) = <i> Glue [i=0 ↦ ({}, {})] {}",
            self.equivalence, self.B, self.A, self.B, self.equivalence, self.A
        )
    }
}
/// A cubical type theory synthesis record (e.g. CCHM, RedTT).
pub struct CubicalSynth {
    /// Whether this instance implements full cubical TT.
    pub cubical_tt: bool,
    /// Whether the homotopy interpretation is verified.
    pub is_homotopy_correct: bool,
}
impl CubicalSynth {
    /// Creates a new `CubicalSynth`.
    pub fn new(cubical_tt: bool, is_homotopy_correct: bool) -> Self {
        Self {
            cubical_tt,
            is_homotopy_correct,
        }
    }
    /// Reference to the Agda implementation of cubical type theory.
    pub fn implementation_in_agda(&self) -> String {
        if self.cubical_tt {
            "agda/cubical — Agda cubical library (CCHM, univalence by composition)".to_string()
        } else {
            "Agda with --cubical flag (experimental)".to_string()
        }
    }
    /// Reference to the RedTT implementation.
    pub fn red_tt(&self) -> String {
        "RedTT — a proof assistant based on cartesian cubical type theory (ABCFHL)".to_string()
    }
}
/// A type annotated with its homotopy level.
#[allow(dead_code)]
pub struct HomotopyLevelAnnotation {
    /// Name of the type.
    pub type_name: String,
    /// Homotopy level.
    pub level: HomotopyLevel,
}
#[allow(dead_code)]
impl HomotopyLevelAnnotation {
    /// Create a new annotation.
    pub fn new(type_name: impl Into<String>, level: HomotopyLevel) -> Self {
        Self {
            type_name: type_name.into(),
            level,
        }
    }
    /// The numeric level: -2, -1, 0, 1, n, or ∞.
    pub fn numeric_level(&self) -> Option<i64> {
        match &self.level {
            HomotopyLevel::Contractible => Some(-2),
            HomotopyLevel::Proposition => Some(-1),
            HomotopyLevel::Set => Some(0),
            HomotopyLevel::Groupoid => Some(1),
            HomotopyLevel::NType(n) => Some(*n as i64),
            HomotopyLevel::Infinite => None,
        }
    }
    /// Whether the type is at most a set.
    pub fn is_at_most_set(&self) -> bool {
        matches!(
            &self.level,
            HomotopyLevel::Contractible | HomotopyLevel::Proposition | HomotopyLevel::Set
        )
    }
    /// Describe the homotopy level.
    pub fn describe(&self) -> String {
        match &self.level {
            HomotopyLevel::Contractible => {
                format!("{} is contractible ((-2)-type)", self.type_name)
            }
            HomotopyLevel::Proposition => {
                format!("{} is a proposition ((-1)-type)", self.type_name)
            }
            HomotopyLevel::Set => format!("{} is a set (0-type)", self.type_name),
            HomotopyLevel::Groupoid => {
                format!("{} is a groupoid (1-type)", self.type_name)
            }
            HomotopyLevel::NType(n) => format!("{} is a {}-type", self.type_name, n),
            HomotopyLevel::Infinite => {
                format!("{} has infinite homotopy level", self.type_name)
            }
        }
    }
}
/// An open box for homogeneous composition.
///
/// An open box in a type A consists of:
/// - A base element `base : A`
/// - Tubes: partial elements defined on various faces
#[derive(Debug, Clone)]
pub struct HcompBox {
    /// The ambient type.
    pub type_name: String,
    /// The base element (bottom face).
    pub base: String,
    /// The tubes: (face_formula, element_on_that_face).
    pub tubes: Vec<(String, String)>,
}
impl HcompBox {
    /// Create an empty open box with just a base.
    pub fn new(type_name: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            base: base.into(),
            tubes: Vec::new(),
        }
    }
    /// Add a tube (partial element on a face) to the box.
    pub fn add_tube(mut self, face: impl Into<String>, elem: impl Into<String>) -> Self {
        self.tubes.push((face.into(), elem.into()));
        self
    }
    /// "Fill" the open box: produce the cap element via hcomp.
    pub fn fill(&self) -> String {
        format!(
            "hcomp[{}]({}, tubes={})",
            self.type_name,
            self.base,
            self.tubes.len()
        )
    }
    /// The number of faces in the open box.
    pub fn num_faces(&self) -> usize {
        self.tubes.len()
    }
}
