//! The types a [`TypeChecker`] has inferred, remembered by term.

use super::types::TypeChecker;
use crate::error::KernelError;
use crate::Expr;

impl TypeChecker<'_> {
    /// Infer the type of an expression.
    ///
    /// # The cache
    ///
    /// A type that was inferred is kept, by term, and returned the next time
    /// the same term is asked about (Lean's `m_infer_type`). A proof term is
    /// a DAG — the terms `omega` and `simp` build reach one subterm along
    /// thousands of paths — and without the table every path inferred the
    /// subterm again: millions of inferences for a few thousand distinct
    /// subterms.
    ///
    /// The type of a term is a function of the term, the environment and
    /// the types of the free variables in it, so an entry stays valid while
    /// those do:
    ///
    /// * the environment is fixed for the checker's life;
    /// * a free variable keeps its type while it is in scope, and its id is
    ///   not handed out again after its binder closes — so an entry that
    ///   mentions a closed variable is simply never asked for again. Should
    ///   a caller of [`Self::push_local`] bind an id a second time (to
    ///   another type, or after its binder closed), the table is dropped
    ///   (where the checker mirrors a local into its reducer);
    /// * in checking mode an entry also records that the term CHECKED, which
    ///   depends on the transparency mode: [`Self::set_transparency`] drops
    ///   the table. The mode itself (checking / infer-only) is fixed per
    ///   checker.
    ///
    /// Only successes are kept. A failure is recomputed: it may be the
    /// per-declaration budget running out, which is not a property of the
    /// term. A success reached while the budget was running out is exact
    /// (see [`crate::fuel`]) and safe to keep.
    #[allow(clippy::result_large_err)]
    pub fn infer_type(&mut self, expr: &Expr) -> Result<Expr, KernelError> {
        // C16b: inference is the third loop (besides `whnf` and `is_def_eq`)
        // that must observe the per-declaration fuel latch. A `let`-tower or
        // dependent application chain grows its instantiated bodies
        // geometrically *between* whnf/def-eq steps, so without this check a
        // single `infer_type` call tree can allocate unboundedly while the
        // budget is long exhausted (Init decl #3864,
        // `Array.extract_append_extract._proof_1_1`, reached >12 GiB). The
        // error is a typed abort, mapped by callers to the NAMED resource
        // limit — never a rejection, and never a truncated term.
        if crate::fuel::is_exhausted() {
            return Err(KernelError::Other(
                "per-declaration resource budget exhausted".to_string(),
            ));
        }
        // A `Sort`, a literal and a loose bound variable are read straight
        // off the term: nothing to look up, nothing to keep.
        let remembered = !matches!(expr, Expr::Sort(_) | Expr::BVar(_) | Expr::Lit(_));
        if remembered {
            if let Some(ty) = self.infer_cache.get(expr) {
                return Ok(ty.clone());
            }
        }
        let ty = self.infer_type_uncached(expr)?;
        if remembered {
            self.infer_cache.insert(expr, ty.clone());
        }
        Ok(ty)
    }
}
