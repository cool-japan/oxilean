//! # Expr - Trait Implementations
//!
//! This module contains trait implementations for `Expr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::Cell;
use std::fmt;

use super::functions::*;
use super::types::{Expr, Literal};

/// How many subterms one rendering of a term (`Display` or `Debug`) writes
/// before it writes `…` in place of the rest.
///
/// A term is a DAG, and rendering writes it as the tree it stands for: a
/// proof term of a few thousand shared nodes can be a tree of billions. An
/// error message that rendered such a term in full — `check_type` puts the
/// argument it was checking into its context — ran a 24 GB process out of
/// memory on one declaration of the GRH proof export before the declaration
/// could even be reported. Every rendering is now cut off after this many
/// subterms, which bounds both its length and its time.
pub const RENDER_NODE_LIMIT: u32 = 2048;

thread_local! {
    /// Subterms the rendering in progress on this thread may still write;
    /// `None` when no rendering is in progress.
    static RENDER_LEFT: Cell<Option<u32>> = const { Cell::new(None) };
}

/// One subterm's turn in a bounded rendering.
pub(crate) enum RenderTurn {
    /// The outermost subterm: it opened the budget and closes it on drop.
    Outermost,
    /// A subterm inside a rendering that still has budget.
    Inner,
    /// The budget is spent: write `…` instead of the subterm.
    Spent,
}

impl RenderTurn {
    /// Take a turn: open the budget when no rendering is in progress, or
    /// spend one unit of the one that is.
    pub(crate) fn take() -> RenderTurn {
        RENDER_LEFT.with(|left| match left.get() {
            None => {
                left.set(Some(RENDER_NODE_LIMIT.saturating_sub(1)));
                RenderTurn::Outermost
            }
            Some(0) => RenderTurn::Spent,
            Some(n) => {
                left.set(Some(n - 1));
                RenderTurn::Inner
            }
        })
    }

    /// Whether the budget is spent.
    pub(crate) fn is_spent(&self) -> bool {
        matches!(self, RenderTurn::Spent)
    }
}

impl Drop for RenderTurn {
    fn drop(&mut self) {
        if matches!(self, RenderTurn::Outermost) {
            RENDER_LEFT.with(|left| left.set(None));
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let turn = RenderTurn::take();
        if turn.is_spent() {
            return f.write_str("…");
        }
        match self {
            Expr::Sort(l) if l.is_zero() => write!(f, "Prop"),
            Expr::Sort(l) => write!(f, "Type {}", l),
            Expr::BVar(n) => write!(f, "#{}", n),
            Expr::FVar(id) => write!(f, "fvar_{}", id.0),
            Expr::Const(name, levels) if levels.is_empty() => write!(f, "{}", name),
            Expr::Const(name, levels) => {
                write!(f, "{}.{{", name)?;
                for (i, l) in levels.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", l)?;
                }
                write!(f, "}}")
            }
            Expr::App(fun, arg) => write!(f, "({} {})", fun, arg),
            Expr::Lam(_, name, ty, body) => write!(f, "(λ {} : {}, {})", name, ty, body),
            Expr::Pi(_, name, ty, body) => {
                if !has_loose_bvar(body, 0) {
                    write!(f, "({} → {})", ty, body)
                } else {
                    write!(f, "(Π {} : {}, {})", name, ty, body)
                }
            }
            Expr::Let(name, ty, val, body) => {
                write!(f, "(let {} : {} := {} in {})", name, ty, val, body)
            }
            Expr::Lit(Literal::Nat(n)) => write!(f, "{}", n),
            Expr::Lit(Literal::Str(s)) => write!(f, "\"{}\"", s),
            Expr::Proj(name, idx, e) => write!(f, "{}.{} {}", name, idx, e),
        }
    }
}

#[cfg(test)]
mod render_bound_tests {
    use super::*;
    use crate::{Name, Node};

    /// `f (f (f … x))` with every level shared twice: a DAG of `depth`
    /// nodes standing for a tree of about `2^depth`.
    fn doubling_dag(depth: usize) -> Expr {
        let mut e = Expr::Const(Name::str("x"), vec![]);
        for _ in 0..depth {
            let n = Node::new(e);
            e = Expr::App(n.clone(), n);
        }
        e
    }

    #[test]
    fn display_of_a_huge_shared_term_is_bounded() {
        let e = doubling_dag(200);
        let s = e.to_string();
        assert!(s.contains('…'));
        assert!(s.len() < 64 * 1024, "rendered {} bytes", s.len());
        // A second rendering on the same thread gets a fresh budget.
        assert_eq!(e.to_string(), s);
    }

    #[test]
    fn debug_of_a_huge_shared_term_is_bounded() {
        let e = doubling_dag(200);
        let s = format!("{e:?}");
        assert!(s.contains('…'));
        assert!(s.len() < 256 * 1024, "rendered {} bytes", s.len());
    }

    #[test]
    fn small_terms_render_in_full() {
        let e = doubling_dag(3);
        assert_eq!(e.to_string(), "(((x x) (x x)) ((x x) (x x)))");
        assert!(!format!("{e:?}").contains('…'));
    }
}
