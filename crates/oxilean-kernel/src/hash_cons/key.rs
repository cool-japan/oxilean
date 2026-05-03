//! `ExprKey` — a hashable, owning representation of an `Expr` node.
//!
//! `Expr` is already `Hash + Eq` (derived), so `ExprKey` is simply a
//! newtype wrapper that makes the intent explicit: it identifies a node's
//! *structural content*, not its arena slot.
//!
//! Using a newtype keeps the cache `HashMap` key type distinct from `Expr`
//! so callers cannot accidentally conflate "a key for lookup" with "a live
//! expression value".

use crate::{BinderInfo, Expr, FVarId, Level, Literal, Name};
use std::hash::{Hash, Hasher};

/// Structural key derived from the content of one `Expr` node.
///
/// Two `ExprKey` values are equal if and only if the corresponding `Expr`
/// values are structurally identical (same variant, same fields). This is
/// precisely the condition under which hash-consing must return the same
/// `Idx<Expr>`.
///
/// Internally the key clones the `Expr` so that the `HashMap<ExprKey, …>`
/// owns its keys independently of any arena.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprKey(pub Expr);

impl Hash for ExprKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Delegate to `Expr`'s derived `Hash` implementation which performs
        // a full structural walk.
        self.0.hash(state);
    }
}

impl ExprKey {
    /// Construct an `ExprKey` from an owned `Expr`.
    #[inline]
    pub fn new(expr: Expr) -> Self {
        Self(expr)
    }

    /// Construct an `ExprKey` from a reference, cloning the expression.
    #[inline]
    pub fn from_ref(expr: &Expr) -> Self {
        Self(expr.clone())
    }

    /// Return the inner `Expr`.
    #[inline]
    pub fn into_inner(self) -> Expr {
        self.0
    }

    /// Borrow the inner `Expr`.
    #[inline]
    pub fn inner(&self) -> &Expr {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// Constructors that build ExprKey values directly from field values.
// These mirror the `mk_*` constructors on `HashConsArena` so the cache layer
// can construct a lookup key without first allocating an `Expr`.
// ---------------------------------------------------------------------------

impl ExprKey {
    /// Key for `Expr::Sort(level)`.
    pub fn sort(level: Level) -> Self {
        Self::new(Expr::Sort(level))
    }

    /// Key for `Expr::BVar(idx)`.
    pub fn bvar(idx: u32) -> Self {
        Self::new(Expr::BVar(idx))
    }

    /// Key for `Expr::FVar(id)`.
    pub fn fvar(id: FVarId) -> Self {
        Self::new(Expr::FVar(id))
    }

    /// Key for `Expr::Const(name, levels)`.
    pub fn const_(name: Name, levels: Vec<Level>) -> Self {
        Self::new(Expr::Const(name, levels))
    }

    /// Key for `Expr::App(f, a)`.
    pub fn app(f: Expr, a: Expr) -> Self {
        Self::new(Expr::App(Box::new(f), Box::new(a)))
    }

    /// Key for `Expr::Lam(bi, name, dom, body)`.
    pub fn lam(bi: BinderInfo, name: Name, dom: Expr, body: Expr) -> Self {
        Self::new(Expr::Lam(bi, name, Box::new(dom), Box::new(body)))
    }

    /// Key for `Expr::Pi(bi, name, dom, cod)`.
    pub fn pi(bi: BinderInfo, name: Name, dom: Expr, cod: Expr) -> Self {
        Self::new(Expr::Pi(bi, name, Box::new(dom), Box::new(cod)))
    }

    /// Key for `Expr::Let(name, ty, val, body)`.
    pub fn let_(name: Name, ty: Expr, val: Expr, body: Expr) -> Self {
        Self::new(Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body)))
    }

    /// Key for `Expr::Lit(lit)`.
    pub fn lit(lit: Literal) -> Self {
        Self::new(Expr::Lit(lit))
    }

    /// Key for `Expr::Proj(name, idx, struct_expr)`.
    pub fn proj(name: Name, idx: u32, struct_expr: Expr) -> Self {
        Self::new(Expr::Proj(name, idx, Box::new(struct_expr)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_key_eq_same_bvar() {
        let k1 = ExprKey::bvar(0);
        let k2 = ExprKey::bvar(0);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_ne_different_bvar() {
        let k1 = ExprKey::bvar(0);
        let k2 = ExprKey::bvar(1);
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_expr_key_sort_zero_equals() {
        let k1 = ExprKey::sort(Level::Zero);
        let k2 = ExprKey::sort(Level::Zero);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_fvar_equals() {
        let id = FVarId::new(42);
        let k1 = ExprKey::fvar(id);
        let k2 = ExprKey::fvar(id);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_hash_consistent() {
        use std::collections::hash_map::DefaultHasher;
        let k1 = ExprKey::bvar(5);
        let k2 = ExprKey::bvar(5);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        k1.hash(&mut h1);
        k2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_expr_key_into_inner() {
        let k = ExprKey::bvar(3);
        let expr = k.into_inner();
        assert_eq!(expr, Expr::BVar(3));
    }

    #[test]
    fn test_expr_key_from_ref() {
        let expr = Expr::BVar(7);
        let k1 = ExprKey::from_ref(&expr);
        let k2 = ExprKey::bvar(7);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_const_equals() {
        let name = Name::str("Nat");
        let k1 = ExprKey::const_(name.clone(), vec![]);
        let k2 = ExprKey::const_(name, vec![]);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_app_equals() {
        let f = Expr::BVar(0);
        let a = Expr::BVar(1);
        let k1 = ExprKey::app(f.clone(), a.clone());
        let k2 = ExprKey::app(f, a);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_lam_equals() {
        let name = Name::str("x");
        let dom = Expr::Sort(Level::Zero);
        let body = Expr::BVar(0);
        let k1 = ExprKey::lam(BinderInfo::Default, name.clone(), dom.clone(), body.clone());
        let k2 = ExprKey::lam(BinderInfo::Default, name, dom, body);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_pi_equals() {
        let name = Name::str("A");
        let dom = Expr::Sort(Level::Zero);
        let cod = Expr::BVar(0);
        let k1 = ExprKey::pi(BinderInfo::Default, name.clone(), dom.clone(), cod.clone());
        let k2 = ExprKey::pi(BinderInfo::Default, name, dom, cod);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_let_equals() {
        let name = Name::str("n");
        let ty = Expr::Sort(Level::Zero);
        let val = Expr::BVar(0);
        let body = Expr::BVar(1);
        let k1 = ExprKey::let_(name.clone(), ty.clone(), val.clone(), body.clone());
        let k2 = ExprKey::let_(name, ty, val, body);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_lit_nat_equals() {
        let k1 = ExprKey::lit(Literal::Nat(42));
        let k2 = ExprKey::lit(Literal::Nat(42));
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_proj_equals() {
        let name = Name::str("fst");
        let se = Expr::BVar(0);
        let k1 = ExprKey::proj(name.clone(), 0, se.clone());
        let k2 = ExprKey::proj(name, 0, se);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_expr_key_usable_as_hashmap_key() {
        use std::collections::HashMap;
        let mut map: HashMap<ExprKey, u32> = HashMap::new();
        map.insert(ExprKey::bvar(0), 100);
        map.insert(ExprKey::bvar(1), 200);
        assert_eq!(map.get(&ExprKey::bvar(0)), Some(&100));
        assert_eq!(map.get(&ExprKey::bvar(1)), Some(&200));
        assert_eq!(map.get(&ExprKey::bvar(2)), None);
    }
}
