//! `HashConsArena` — deduplicated `Arena<Expr>`.

use crate::arena::{Arena, Idx};
use crate::{BinderInfo, Expr, FVarId, Level, Literal, Name};
use std::collections::HashMap;

use super::key::ExprKey;
use super::stats::HashConsStats;

/// A hash-consing wrapper around `Arena<Expr>`.
///
/// Every `mk_*` method checks a `HashMap<ExprKey, Idx<Expr>>` before
/// allocating. If an equal node already exists in the arena the existing
/// `Idx` is returned (a *hit*). Otherwise the node is allocated and
/// indexed (a *miss*).
///
/// # Structural sharing guarantee
///
/// For any two calls `mk_foo(…args…)` with structurally identical arguments,
/// the returned `Idx<Expr>` values are equal. This means:
///
/// - `O(1)` equality of deduplicated nodes via index comparison
/// - Significantly reduced arena memory for expression-heavy workloads
///   (e.g. type checking with many repeated sub-expressions)
pub struct HashConsArena {
    /// Backing arena; all nodes live here.
    arena: Arena<Expr>,
    /// Cache: expression content → arena index.
    cache: HashMap<ExprKey, Idx<Expr>>,
    /// Hit counter.
    hits: u64,
    /// Miss counter.
    misses: u64,
}

impl HashConsArena {
    /// Creates an empty `HashConsArena`.
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Creates a `HashConsArena` with pre-allocated capacity for `cap` nodes.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            arena: Arena::with_capacity(cap),
            cache: HashMap::with_capacity(cap),
            hits: 0,
            misses: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Core deduplication: look up `key`; allocate `expr` on a miss.
    ///
    /// Takes both `key` (for lookup) and `expr` (for insertion) separately so
    /// the caller can build the key cheaply and only clone the full `Expr`
    /// when a miss occurs.
    fn get_or_insert(&mut self, key: ExprKey, expr: Expr) -> Idx<Expr> {
        if let Some(idx) = self.cache.get(&key) {
            self.hits += 1;
            return idx.clone();
        }
        self.misses += 1;
        let idx = self.arena.alloc(expr);
        self.cache.insert(key, idx.clone());
        idx
    }

    // -----------------------------------------------------------------------
    // Public read API
    // -----------------------------------------------------------------------

    /// Retrieve the `Expr` stored at `idx`.
    pub fn get(&self, idx: Idx<Expr>) -> &Expr {
        self.arena.get(idx)
    }

    /// Returns the number of distinct nodes in the arena.
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Returns `true` when no nodes have been allocated.
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    /// Returns a statistics snapshot.
    pub fn stats(&self) -> HashConsStats {
        HashConsStats {
            hits: self.hits,
            misses: self.misses,
            total: self.hits + self.misses,
            unique_nodes: self.arena.len(),
        }
    }

    /// Borrow the underlying `Arena<Expr>` for read-only access.
    pub fn arena(&self) -> &Arena<Expr> {
        &self.arena
    }

    // -----------------------------------------------------------------------
    // mk_* constructors
    // -----------------------------------------------------------------------

    /// Allocate (or deduplicate) `Expr::Sort(level)`.
    pub fn mk_sort(&mut self, level: Level) -> Idx<Expr> {
        let key = ExprKey::sort(level.clone());
        let expr = Expr::Sort(level);
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::BVar(idx)`.
    pub fn mk_bvar(&mut self, idx: u32) -> Idx<Expr> {
        let key = ExprKey::bvar(idx);
        let expr = Expr::BVar(idx);
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::FVar(id)`.
    pub fn mk_fvar(&mut self, id: FVarId) -> Idx<Expr> {
        let key = ExprKey::fvar(id);
        let expr = Expr::FVar(id);
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Const(name, levels)`.
    pub fn mk_const(&mut self, name: Name, levels: Vec<Level>) -> Idx<Expr> {
        let key = ExprKey::const_(name.clone(), levels.clone());
        let expr = Expr::Const(name, levels);
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::App(f_expr, a_expr)`.
    ///
    /// `f_expr` and `a_expr` are cloned once to build the `ExprKey`; no
    /// extra clones occur on a cache hit.
    pub fn mk_app(&mut self, f_expr: Expr, a_expr: Expr) -> Idx<Expr> {
        let key = ExprKey::app(f_expr.clone(), a_expr.clone());
        let expr = Expr::App(Box::new(f_expr), Box::new(a_expr));
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Lam(bi, name, dom, body)`.
    pub fn mk_lam(&mut self, bi: BinderInfo, name: Name, dom: Expr, body: Expr) -> Idx<Expr> {
        let key = ExprKey::lam(bi, name.clone(), dom.clone(), body.clone());
        let expr = Expr::Lam(bi, name, Box::new(dom), Box::new(body));
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Pi(bi, name, dom, cod)`.
    pub fn mk_pi(&mut self, bi: BinderInfo, name: Name, dom: Expr, cod: Expr) -> Idx<Expr> {
        let key = ExprKey::pi(bi, name.clone(), dom.clone(), cod.clone());
        let expr = Expr::Pi(bi, name, Box::new(dom), Box::new(cod));
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Let(name, ty, val, body)`.
    pub fn mk_let(&mut self, name: Name, ty: Expr, val: Expr, body: Expr) -> Idx<Expr> {
        let key = ExprKey::let_(name.clone(), ty.clone(), val.clone(), body.clone());
        let expr = Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body));
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Lit(lit)`.
    pub fn mk_lit(&mut self, lit: Literal) -> Idx<Expr> {
        let key = ExprKey::lit(lit.clone());
        let expr = Expr::Lit(lit);
        self.get_or_insert(key, expr)
    }

    /// Allocate (or deduplicate) `Expr::Proj(name, field_idx, struct_expr)`.
    pub fn mk_proj(&mut self, name: Name, field_idx: u32, struct_expr: Expr) -> Idx<Expr> {
        let key = ExprKey::proj(name.clone(), field_idx, struct_expr.clone());
        let expr = Expr::Proj(name, field_idx, Box::new(struct_expr));
        self.get_or_insert(key, expr)
    }
}

impl Default for HashConsArena {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Name};

    // ---- mk_sort -----------------------------------------------------------

    #[test]
    fn test_mk_sort_deduplicates() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_sort(Level::Zero);
        let i2 = hc.mk_sort(Level::Zero);
        assert_eq!(i1, i2);
        assert_eq!(hc.len(), 1);
    }

    #[test]
    fn test_mk_sort_distinct_levels() {
        let mut hc = HashConsArena::new();
        let i0 = hc.mk_sort(Level::Zero);
        let i1 = hc.mk_sort(Level::succ(Level::Zero));
        assert_ne!(i0, i1);
        assert_eq!(hc.len(), 2);
    }

    // ---- mk_bvar -----------------------------------------------------------

    #[test]
    fn test_mk_bvar_deduplicates() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_bvar(0);
        let i2 = hc.mk_bvar(0);
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_mk_bvar_distinct_indices() {
        let mut hc = HashConsArena::new();
        let i0 = hc.mk_bvar(0);
        let i1 = hc.mk_bvar(1);
        assert_ne!(i0, i1);
    }

    // ---- mk_fvar -----------------------------------------------------------

    #[test]
    fn test_mk_fvar_deduplicates() {
        let mut hc = HashConsArena::new();
        let id = FVarId::new(7);
        let i1 = hc.mk_fvar(id);
        let i2 = hc.mk_fvar(id);
        assert_eq!(i1, i2);
    }

    // ---- mk_const ----------------------------------------------------------

    #[test]
    fn test_mk_const_deduplicates() {
        let mut hc = HashConsArena::new();
        let name = Name::str("Nat");
        let i1 = hc.mk_const(name.clone(), vec![]);
        let i2 = hc.mk_const(name, vec![]);
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_mk_const_different_names_distinct() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_const(Name::str("Nat"), vec![]);
        let i2 = hc.mk_const(Name::str("Int"), vec![]);
        assert_ne!(i1, i2);
    }

    // ---- mk_app ------------------------------------------------------------

    #[test]
    fn test_mk_app_deduplicates() {
        let mut hc = HashConsArena::new();
        let f = Expr::BVar(0);
        let a = Expr::BVar(1);
        let i1 = hc.mk_app(f.clone(), a.clone());
        let i2 = hc.mk_app(f, a);
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_mk_app_different_args_distinct() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_app(Expr::BVar(0), Expr::BVar(1));
        let i2 = hc.mk_app(Expr::BVar(0), Expr::BVar(2));
        assert_ne!(i1, i2);
    }

    // ---- mk_lam ------------------------------------------------------------

    #[test]
    fn test_mk_lam_deduplicates() {
        let mut hc = HashConsArena::new();
        let name = Name::str("x");
        let dom = Expr::Sort(Level::Zero);
        let body = Expr::BVar(0);
        let i1 = hc.mk_lam(BinderInfo::Default, name.clone(), dom.clone(), body.clone());
        let i2 = hc.mk_lam(BinderInfo::Default, name, dom, body);
        assert_eq!(i1, i2);
    }

    // ---- mk_pi -------------------------------------------------------------

    #[test]
    fn test_mk_pi_deduplicates() {
        let mut hc = HashConsArena::new();
        let name = Name::str("A");
        let dom = Expr::Sort(Level::Zero);
        let cod = Expr::BVar(0);
        let i1 = hc.mk_pi(BinderInfo::Default, name.clone(), dom.clone(), cod.clone());
        let i2 = hc.mk_pi(BinderInfo::Default, name, dom, cod);
        assert_eq!(i1, i2);
    }

    // ---- mk_let ------------------------------------------------------------

    #[test]
    fn test_mk_let_deduplicates() {
        let mut hc = HashConsArena::new();
        let name = Name::str("n");
        let ty = Expr::Sort(Level::Zero);
        let val = Expr::BVar(0);
        let body = Expr::BVar(1);
        let i1 = hc.mk_let(name.clone(), ty.clone(), val.clone(), body.clone());
        let i2 = hc.mk_let(name, ty, val, body);
        assert_eq!(i1, i2);
    }

    // ---- mk_lit / mk_proj --------------------------------------------------

    #[test]
    fn test_mk_lit_nat_deduplicates() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_lit(Literal::Nat(42));
        let i2 = hc.mk_lit(Literal::Nat(42));
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_mk_lit_str_deduplicates() {
        let mut hc = HashConsArena::new();
        let i1 = hc.mk_lit(Literal::Str("hello".to_string()));
        let i2 = hc.mk_lit(Literal::Str("hello".to_string()));
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_mk_proj_deduplicates() {
        let mut hc = HashConsArena::new();
        let name = Name::str("fst");
        let se = Expr::BVar(0);
        let i1 = hc.mk_proj(name.clone(), 0, se.clone());
        let i2 = hc.mk_proj(name, 0, se);
        assert_eq!(i1, i2);
    }

    // ---- stats -------------------------------------------------------------

    #[test]
    fn test_stats_hit_miss_tracking() {
        let mut hc = HashConsArena::new();
        hc.mk_sort(Level::Zero); // miss
        hc.mk_sort(Level::Zero); // hit
        hc.mk_sort(Level::Zero); // hit
        hc.mk_bvar(0); // miss
        let s = hc.stats();
        assert_eq!(s.misses, 2);
        assert_eq!(s.hits, 2);
        assert_eq!(s.total, 4);
        assert_eq!(s.unique_nodes, 2);
    }

    #[test]
    fn test_stats_dedup_ratio() {
        let mut hc = HashConsArena::new();
        hc.mk_bvar(0); // miss
        hc.mk_bvar(0); // hit
        hc.mk_bvar(0); // hit
        hc.mk_bvar(0); // hit
        let s = hc.stats();
        // dedup_ratio = misses / total = 1/4 = 0.25
        assert!((s.dedup_ratio() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_get_returns_correct_expr() {
        let mut hc = HashConsArena::new();
        let idx = hc.mk_bvar(5);
        assert_eq!(hc.get(idx), &Expr::BVar(5));
    }

    #[test]
    fn test_is_empty_initially() {
        let hc = HashConsArena::new();
        assert!(hc.is_empty());
    }

    #[test]
    fn test_with_capacity_works() {
        let mut hc = HashConsArena::with_capacity(64);
        hc.mk_sort(Level::Zero);
        assert_eq!(hc.len(), 1);
    }

    #[test]
    fn test_default_is_empty() {
        let hc = HashConsArena::default();
        assert!(hc.is_empty());
    }

    #[test]
    fn test_cross_variant_no_collision() {
        let mut hc = HashConsArena::new();
        // BVar(0) and Sort(Zero) must NOT collide even though both are "zero-ish"
        let bv = hc.mk_bvar(0);
        let sv = hc.mk_sort(Level::Zero);
        assert_ne!(bv, sv);
    }

    #[test]
    fn test_binder_info_distinguishes_lam() {
        let mut hc = HashConsArena::new();
        let name = Name::str("x");
        let dom = Expr::Sort(Level::Zero);
        let body = Expr::BVar(0);
        let explicit = hc.mk_lam(BinderInfo::Default, name.clone(), dom.clone(), body.clone());
        let implicit = hc.mk_lam(BinderInfo::Implicit, name, dom, body);
        assert_ne!(explicit, implicit);
    }

    #[test]
    fn test_complex_expression_sharing() {
        let mut hc = HashConsArena::new();
        // Build `id : Π (A : Type 0), A → A` structurally, twice.
        let prop = Expr::Sort(Level::Zero);
        let bv0 = Expr::BVar(0);
        // A → A  =  Π (_ : A), A
        let arr = Expr::Pi(
            BinderInfo::Default,
            Name::Anonymous,
            Box::new(bv0.clone()),
            Box::new(bv0.clone()),
        );
        let id_type_1 = hc.mk_pi(
            BinderInfo::Default,
            Name::str("A"),
            prop.clone(),
            arr.clone(),
        );
        let id_type_2 = hc.mk_pi(
            BinderInfo::Default,
            Name::str("A"),
            prop.clone(),
            arr.clone(),
        );
        assert_eq!(
            id_type_1, id_type_2,
            "identical Pi types must be deduplicated"
        );
        assert_eq!(hc.len(), 1, "only one node in the arena");
    }
}
