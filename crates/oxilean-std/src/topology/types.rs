//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Covering space data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoveringSpaceData {
    /// Total space E.
    pub total: String,
    /// Base space B.
    pub base: String,
    /// Number of sheets.
    pub sheets: Option<usize>,
    /// Deck transformation group.
    pub deck_group: String,
    /// Whether the covering is universal.
    pub is_universal: bool,
}
#[allow(dead_code)]
impl CoveringSpaceData {
    /// Creates covering space data.
    pub fn new(total: &str, base: &str, sheets: Option<usize>) -> Self {
        CoveringSpaceData {
            total: total.to_string(),
            base: base.to_string(),
            sheets,
            deck_group: "unknown".to_string(),
            is_universal: false,
        }
    }
    /// Sets the deck group.
    pub fn with_deck_group(mut self, g: &str) -> Self {
        self.deck_group = g.to_string();
        self
    }
    /// Marks as universal cover.
    pub fn universal(mut self) -> Self {
        self.is_universal = true;
        self
    }
    /// Monodromy representation: π_1(B) → S_n (for n-sheeted cover).
    pub fn monodromy_description(&self) -> String {
        let n = self
            .sheets
            .map(|k| k.to_string())
            .unwrap_or("∞".to_string());
        format!("Monodromy: π_1({}) → S_{}", self.base, n)
    }
    /// Lifting theorem: π_1(E) ↪ π_1(B) with index = sheets.
    pub fn lifting_theorem(&self) -> String {
        format!(
            "π_1({}) ≅ subgroup of π_1({}) with index {:?}",
            self.total, self.base, self.sheets
        )
    }
}
/// Represents the separation axioms satisfied by a topological space.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SeparationAxioms {
    /// T0 (Kolmogorov).
    pub t0: bool,
    /// T1 (Fréchet).
    pub t1: bool,
    /// T2 (Hausdorff).
    pub t2: bool,
    /// T2.5 (Urysohn).
    pub t2_5: bool,
    /// T3 (regular + T1).
    pub t3: bool,
    /// T3.5 (completely regular + T1 = Tychonoff).
    pub t3_5: bool,
    /// T4 (normal + T1).
    pub t4: bool,
}
#[allow(dead_code)]
impl SeparationAxioms {
    /// Creates separation axioms for a T4 (normal Hausdorff) space.
    pub fn normal_hausdorff() -> Self {
        SeparationAxioms {
            t0: true,
            t1: true,
            t2: true,
            t2_5: true,
            t3: true,
            t3_5: true,
            t4: true,
        }
    }
    /// Creates T2 (Hausdorff) axioms.
    pub fn hausdorff() -> Self {
        SeparationAxioms {
            t0: true,
            t1: true,
            t2: true,
            t2_5: false,
            t3: false,
            t3_5: false,
            t4: false,
        }
    }
    /// Checks if the space is completely regular (T3.5).
    pub fn is_tychonoff(&self) -> bool {
        self.t3_5
    }
    /// Urysohn's lemma applies in T4 spaces.
    pub fn urysohn_lemma_applies(&self) -> bool {
        self.t4
    }
    /// Tietze extension applies in T4 spaces.
    pub fn tietze_applies(&self) -> bool {
        self.t4
    }
}
/// Data for Tychonoff's theorem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TychonoffData {
    /// Collection of compact spaces.
    pub factors: Vec<String>,
    /// Whether the product is compact.
    pub product_compact: bool,
}
#[allow(dead_code)]
impl TychonoffData {
    /// Creates Tychonoff data.
    pub fn new(factors: Vec<String>) -> Self {
        TychonoffData {
            product_compact: !factors.is_empty(),
            factors,
        }
    }
    /// Returns Tychonoff's theorem statement.
    pub fn tychonoff_theorem(&self) -> String {
        format!(
            "Tychonoff: ∏ {} is compact (using AC)",
            self.factors.join(" × ")
        )
    }
    /// Returns the Čech-Stone compactification connection.
    pub fn cech_stone_connection(&self) -> String {
        "βX = Stone-Čech compactification = max compact Hausdorff extension of X".to_string()
    }
}
/// Data for a compact Hausdorff space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompactHausdorffData {
    /// Description of the space.
    pub space: String,
    /// Whether the space is metrizable.
    pub metrizable: bool,
    /// Whether the space is second countable.
    pub second_countable: bool,
    /// Whether the space is connected.
    pub connected: bool,
    /// Dimension (if finite-dimensional CW complex or manifold).
    pub dimension: Option<usize>,
}
#[allow(dead_code)]
impl CompactHausdorffData {
    /// Creates compact Hausdorff data.
    pub fn new(space: &str) -> Self {
        CompactHausdorffData {
            space: space.to_string(),
            metrizable: false,
            second_countable: false,
            connected: false,
            dimension: None,
        }
    }
    /// Marks as metrizable.
    pub fn metrizable(mut self) -> Self {
        self.metrizable = true;
        self.second_countable = true;
        self
    }
    /// Marks as connected.
    pub fn connected(mut self) -> Self {
        self.connected = true;
        self
    }
    /// Sets dimension.
    pub fn with_dimension(mut self, d: usize) -> Self {
        self.dimension = Some(d);
        self
    }
    /// Urysohn metrization: compact Hausdorff + 2nd countable → metrizable.
    pub fn urysohn_metrization(&self) -> bool {
        self.second_countable
    }
    /// Stone-Weierstrass applicability: C(X) separates points.
    pub fn stone_weierstrass_applies(&self) -> bool {
        true
    }
    /// Tietze extension theorem: every closed subspace has continuous extension.
    pub fn tietze_extension(&self) -> String {
        format!(
            "Tietze: every f: A → R (A ⊆ {} closed) extends to F: {} → R",
            self.space, self.space
        )
    }
}
/// Data for a quotient space X/~.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuotientSpaceData {
    /// Base space.
    pub space: String,
    /// Equivalence relation description.
    pub relation: String,
    /// Resulting quotient.
    pub quotient: String,
    /// Whether the quotient map is open.
    pub open_map: bool,
}
#[allow(dead_code)]
impl QuotientSpaceData {
    /// Creates quotient space data.
    pub fn new(space: &str, relation: &str, quotient: &str) -> Self {
        QuotientSpaceData {
            space: space.to_string(),
            relation: relation.to_string(),
            quotient: quotient.to_string(),
            open_map: false,
        }
    }
    /// Checks if the quotient is Hausdorff.
    /// A quotient of a compact Hausdorff space by a closed equiv. relation is Hausdorff.
    pub fn is_hausdorff_when_compact_and_closed(&self) -> bool {
        true
    }
    /// Characteristic property: f: X/~ → Y continuous iff f ∘ π continuous.
    pub fn characteristic_property(&self) -> String {
        format!(
            "f: {}/~ → Y is continuous iff f∘π: {} → Y is continuous",
            self.space, self.space
        )
    }
    /// Marks as open map.
    pub fn with_open_map(mut self) -> Self {
        self.open_map = true;
        self
    }
}
