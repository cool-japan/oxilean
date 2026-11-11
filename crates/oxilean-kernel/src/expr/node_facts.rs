//! The content flags of a term: whether it contains a free variable, a
//! universe parameter, a universe metavariable.
//!
//! Every shared subterm ([`Node`]) stores the three, in the bits of one word
//! above its tree size ([`REBUILD_COST_MAX`]), computed when the node is
//! built from the flags its children store. They are what lets a walk that
//! only concerns one of those things pass over everything else without
//! visiting it: abstracting a free variable skips what has none,
//! instantiating universe parameters skips what mentions none, and the
//! closed-term and universe-hygiene checks of a declaration are read off the
//! root.
//!
//! A flag is exact: it is set if and only if such a subterm occurs. (The
//! tests in `node_facts_tests` compare each with a walk of the term.)

use super::types::{Expr, Node, REBUILD_COST_MAX};
use crate::Level;

/// Flag: the term contains a free variable.
const FLAG_FVAR: u32 = 1 << 29;
/// Flag: the term contains a universe parameter.
const FLAG_LEVEL_PARAM: u32 = 1 << 30;
/// Flag: the term contains a universe metavariable.
const FLAG_LEVEL_MVAR: u32 = 1 << 31;

// The flags and the size share a word without overlapping.
const _: () = assert!((FLAG_FVAR | FLAG_LEVEL_PARAM | FLAG_LEVEL_MVAR) & REBUILD_COST_MAX == 0);

impl Expr {
    /// The content flags of this term, O(1) from the children's cached flags
    /// (a `Sort` or a `Const` looks through its universe levels).
    #[inline]
    #[must_use]
    pub(super) fn flags(&self) -> u32 {
        fn level_flags(level: &Level) -> u32 {
            let param = if level.has_param() {
                FLAG_LEVEL_PARAM
            } else {
                0
            };
            let mvar = if level.has_mvar() { FLAG_LEVEL_MVAR } else { 0 };
            param | mvar
        }
        match self {
            Expr::Sort(level) => level_flags(level),
            Expr::Const(_, levels) => levels
                .iter()
                .fold(0, |flags, level| flags | level_flags(level)),
            Expr::FVar(_) => FLAG_FVAR,
            Expr::BVar(_) | Expr::Lit(_) => 0,
            Expr::App(f, a) => f.flags() | a.flags(),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => ty.flags() | body.flags(),
            Expr::Let(_, ty, val, body) => ty.flags() | val.flags() | body.flags(),
            Expr::Proj(_, _, inner) => inner.flags(),
        }
    }

    /// Whether this term contains a free variable. O(1).
    #[inline]
    #[must_use]
    pub(crate) fn has_fvar(&self) -> bool {
        self.flags() & FLAG_FVAR != 0
    }

    /// Whether this term contains a universe parameter. O(1) (a `Sort` or a
    /// `Const` looks through its own universe levels).
    #[inline]
    #[must_use]
    pub(crate) fn has_level_param(&self) -> bool {
        self.flags() & FLAG_LEVEL_PARAM != 0
    }

    /// Whether this term contains a universe metavariable. O(1), as
    /// [`Self::has_level_param`].
    #[inline]
    #[must_use]
    pub(crate) fn has_level_mvar(&self) -> bool {
        self.flags() & FLAG_LEVEL_MVAR != 0
    }
}

#[cfg(feature = "term-depth")]
impl Expr {
    /// The depth of this term: the number of nodes on the longest path from
    /// it down to a leaf, itself included (a leaf is 1). O(1), from the
    /// depths its children cache; saturating.
    ///
    /// Universe levels are not counted: a `Sort` or a `Const` is a leaf
    /// whatever its levels are.
    #[inline]
    #[must_use]
    pub(crate) fn depth(&self) -> u32 {
        let below = match self {
            Expr::Sort(_) | Expr::Const(_, _) | Expr::FVar(_) | Expr::BVar(_) | Expr::Lit(_) => 0,
            Expr::App(f, a) => f.depth().max(a.depth()),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => ty.depth().max(body.depth()),
            Expr::Let(_, ty, val, body) => ty.depth().max(val.depth()).max(body.depth()),
            Expr::Proj(_, _, inner) => inner.depth(),
        };
        below.saturating_add(1)
    }
}

impl Node {
    /// Whether this subterm contains a free variable (O(1)).
    #[inline]
    #[must_use]
    pub(crate) fn has_fvar(&self) -> bool {
        self.flags() & FLAG_FVAR != 0
    }

    /// Whether this subterm contains a universe parameter (O(1)).
    #[inline]
    #[must_use]
    pub(crate) fn has_level_param(&self) -> bool {
        self.flags() & FLAG_LEVEL_PARAM != 0
    }
}
