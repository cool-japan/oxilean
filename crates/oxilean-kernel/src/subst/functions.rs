//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::node_memo::NodeMemo;
use crate::Node;
use crate::{Expr, FVarId};
use std::collections::HashMap;
use std::rc::Rc;

use super::types::{
    ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, LabelSet, NonEmptyVec,
    PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, SubstStats, Substitution, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Instantiate BVar(0) with arg in body, shifting down all other loose BVars.
///
/// This is the core operation for β-reduction: `(λx.body) arg → body[arg/x]`.
///
/// `arg` is expressed in the context *outside* the redex. When the
/// substitution site sits under `depth` additional binders inside `body`,
/// every loose bound variable of `arg` must be lifted by `depth` so it keeps
/// referring to the same outer binder (otherwise it would be captured by the
/// inner binders — the classic de Bruijn capture bug, exposed by open-term
/// reduction such as iota under a `Pi` body during def-eq).
///
/// # Sharing
///
/// The result shares with `body` every subterm the substitution does not
/// reach, and keeps what `body` shares:
///
/// * a subterm with no loose bound variable at or above the current depth
///   (`range <= depth`, read off the [`Node`] in O(1)) is returned as the
///   SAME allocation, not copied;
/// * a subterm that IS rebuilt and has several owners is rebuilt once per
///   depth it is reached at — the result is remembered for the walk
///   (the kernel-private `node_memo` module) and the second path to it gets
///   the same allocation;
/// * `arg` goes in as one allocation per binder depth, however many times
///   the variable occurs there.
///
/// So substituting into a term of `n` distinct nodes builds at most `n` new
/// ones per depth, whatever the term's size as a tree.
///
/// # Fuel
///
/// One unit per subterm the walk reaches, whether it rebuilds it, shares it
/// unchanged or finds it in the memo (see [`crate::fuel`]). Every node the
/// walk allocates is therefore charged.
pub fn instantiate(body: &Expr, arg: &Expr) -> Expr {
    if body.range() == 0 {
        // Closed: no variable to replace and none to shift down.
        return body.clone();
    }
    let mut walk = Instantiate {
        args: Args::Terms(std::slice::from_ref(&arg)),
        rebuilt: NodeMemo::new(),
        lifted: Vec::new(),
    };
    walk.expr(body, 0)
}

/// Instantiate the `args.len()` outermost loose bound variables of `body`
/// at once: `BVar(0)` with the LAST argument, `BVar(1)` with the one before
/// it, and so on, shifting every other loose variable down by `args.len()`.
///
/// This is what `(λx₁ … xₙ. body) a₁ … aₙ` reduces to — `body` being what
/// stands under the `n` binders and `args` being `[a₁, …, aₙ]` — and equal to
/// `n` nested calls of [`instantiate`], in one walk of `body` instead of `n`.
/// Sharing and fuel are as documented on [`instantiate`]. An argument that
/// needs no lifting goes into the result as the allocation the caller holds.
pub(crate) fn instantiate_rev_nodes(body: &Expr, args: &[&Node]) -> Expr {
    if args.is_empty() || body.range() == 0 {
        return body.clone();
    }
    let mut walk = Instantiate {
        args: Args::Nodes(args),
        rebuilt: NodeMemo::new(),
        lifted: Vec::new(),
    };
    walk.expr(body, 0)
}

/// The arguments of one substitution, outermost binder first.
enum Args<'arg> {
    /// Terms the caller holds by reference.
    Terms(&'arg [&'arg Expr]),
    /// Subterms the caller holds as shared allocations.
    Nodes(&'arg [&'arg Node]),
}

impl Args<'_> {
    fn len(&self) -> usize {
        match self {
            Args::Terms(terms) => terms.len(),
            Args::Nodes(nodes) => nodes.len(),
        }
    }

    /// Argument `index` as it must appear under `depth` binders, as an
    /// allocation: the caller's own when it needs no lifting.
    fn lifted(&self, index: usize, depth: u32) -> Node {
        match self {
            Args::Terms(terms) => {
                Node::new(crate::expr_util::lift_loose_bvars(terms[index], depth, 0))
            }
            Args::Nodes(nodes) => {
                let node = nodes[index];
                if depth == 0 || node.range() == 0 {
                    node.clone()
                } else {
                    Node::new(crate::expr_util::lift_loose_bvars(node, depth, 0))
                }
            }
        }
    }

    /// Argument `index` as it must appear under `depth` binders, as a term.
    fn lifted_term(&self, index: usize, depth: u32) -> Expr {
        let term: &Expr = match self {
            Args::Terms(terms) => terms[index],
            Args::Nodes(nodes) => nodes[index],
        };
        crate::expr_util::lift_loose_bvars(term, depth, 0)
    }
}

/// One substitution: the arguments, and what the walk has built so far.
struct Instantiate<'term, 'arg> {
    /// What replaces the outermost loose variables of the body: the last
    /// argument replaces `BVar(0)`.
    args: Args<'arg>,
    /// The rebuilt form of each shared subterm of the body the walk has
    /// finished, by subterm and depth.
    rebuilt: NodeMemo<'term, Node>,
    /// Each argument lifted to each binder depth its variable has been met
    /// at, by (depth, argument). Almost always a handful of entries, hence
    /// a list.
    lifted: Vec<(u32, usize, Node)>,
}

impl<'term> Instantiate<'term, '_> {
    /// Which argument replaces `BVar(index)` under `depth` binders, if it is
    /// one of the substituted variables. (`index >= depth` is the caller's.)
    fn arg_index(&self, index: u32, depth: u32) -> Option<usize> {
        let count = self.args.len();
        // `usize` is at least 32 bits wide on every supported target.
        let offset = (index - depth) as usize;
        (offset < count).then(|| count - 1 - offset)
    }

    /// How far a loose variable that is NOT substituted moves down.
    fn shift(&self) -> u32 {
        // More arguments than a `u32` can count cannot be reached: each one
        // is a binder of the body. Saturating keeps the arithmetic total.
        u32::try_from(self.args.len()).unwrap_or(u32::MAX)
    }

    /// The substitution applied to a child edge at binder depth `depth`.
    fn node(&mut self, node: &'term Node, depth: u32) -> Node {
        crate::fuel::charge(1);
        if node.range() <= depth {
            // No loose bound variable at or above `depth`: untouched.
            return node.clone();
        }
        if let Expr::BVar(index) = &**node {
            // `range > depth` for a variable means `index >= depth`: one of
            // the substituted variables, or one bound further out.
            return match self.arg_index(*index, depth) {
                Some(which) => self.arg_at(which, depth),
                None => Node::new(Expr::BVar(index.saturating_sub(self.shift()))),
            };
        }
        let shared = node.is_shared();
        if shared {
            if let Some(done) = self.rebuilt.get(node, depth) {
                return done.clone();
            }
        }
        let result = Node::new(self.rebuild(node, depth));
        if shared {
            self.rebuilt.insert(node, depth, result.clone());
        }
        result
    }

    /// Argument `which` as it must appear under `depth` binders: lifted once
    /// per depth and shared by every occurrence of its variable there.
    fn arg_at(&mut self, which: usize, depth: u32) -> Node {
        if let Some((_, _, lifted)) = self
            .lifted
            .iter()
            .find(|(d, w, _)| *d == depth && *w == which)
        {
            return lifted.clone();
        }
        let lifted = self.args.lifted(which, depth);
        self.lifted.push((depth, which, lifted.clone()));
        lifted
    }

    /// The substitution applied to the body itself.
    fn expr(&mut self, expr: &'term Expr, depth: u32) -> Expr {
        crate::fuel::charge(1);
        self.rebuild(expr, depth)
    }

    /// Rebuild `expr`, which has a loose bound variable at or above `depth`.
    fn rebuild(&mut self, expr: &'term Expr, depth: u32) -> Expr {
        match expr {
            Expr::BVar(n) => {
                if *n < depth {
                    return expr.clone();
                }
                match self.arg_index(*n, depth) {
                    Some(which) => self.args.lifted_term(which, depth),
                    None => Expr::BVar(n.saturating_sub(self.shift())),
                }
            }
            Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
            Expr::App(f, a) => Expr::App(self.node(f, depth), self.node(a, depth)),
            Expr::Lam(bi, name, ty, body) => Expr::Lam(
                *bi,
                name.clone(),
                self.node(ty, depth),
                self.node(body, depth + 1),
            ),
            Expr::Pi(bi, name, ty, body) => Expr::Pi(
                *bi,
                name.clone(),
                self.node(ty, depth),
                self.node(body, depth + 1),
            ),
            Expr::Let(name, ty, val, body) => Expr::Let(
                name.clone(),
                self.node(ty, depth),
                self.node(val, depth),
                self.node(body, depth + 1),
            ),
            Expr::Proj(name, idx, e) => Expr::Proj(name.clone(), *idx, self.node(e, depth)),
        }
    }
}

/// Replace FVar with BVar(0), shifting up loose (unbound) BVars.
///
/// This is the inverse of instantiation, used when forming binders.
/// Bound variables (BVars pointing at binders *inside* `expr`) are left
/// unchanged; only loose BVars — which refer to binders outside `expr`
/// and therefore move one binder further away when the new binder is
/// wrapped around the result — are shifted by one.
///
/// # Sharing
///
/// A subterm the abstraction leaves as it is — it holds neither the
/// variable nor a loose bound variable to shift — is the SAME allocation in
/// the result, and a shared subterm that does change is rebuilt once per
/// depth (as in [`instantiate`]). A subterm with no free variable at all and
/// no loose bound variable at or above the current depth is passed over
/// without being visited (both facts are read off the [`Node`]).
///
/// # Fuel
///
/// One unit per subterm the walk reaches (see [`crate::fuel`]).
pub fn abstract_expr(expr: &Expr, fvar: FVarId) -> Expr {
    let mut walk = Abstract {
        fvar,
        done: NodeMemo::new(),
    };
    crate::fuel::charge(1);
    match walk.rebuild(expr, 0) {
        Some(changed) => changed,
        None => expr.clone(),
    }
}

/// One [`abstract_expr`] call. `None` stands for "unchanged" throughout: the
/// caller keeps the subterm it already holds.
struct Abstract<'term> {
    fvar: FVarId,
    /// The outcome for each shared subterm the walk has finished, by subterm
    /// and depth.
    done: NodeMemo<'term, Option<Node>>,
}

impl<'term> Abstract<'term> {
    /// The abstraction applied to a child edge under `depth` binders;
    /// `None` when it leaves the subterm as it is.
    fn node(&mut self, node: &'term Node, depth: u32) -> Option<Node> {
        crate::fuel::charge(1);
        if !node.has_fvar() && node.range() <= depth {
            // Neither the variable nor a loose bound variable to shift.
            return None;
        }
        if matches!(&**node, Expr::FVar(_) | Expr::BVar(_)) {
            // A leaf is cheaper to redo than to look up.
            return self.rebuild(node, depth).map(Node::new);
        }
        let shared = node.is_shared();
        if shared {
            if let Some(done) = self.done.get(node, depth) {
                return done.clone();
            }
        }
        let result = self.rebuild(node, depth).map(Node::new);
        if shared {
            self.done.insert(node, depth, result.clone());
        }
        result
    }

    /// Rebuild `expr` under `depth` binders; `None` when nothing in it
    /// changes.
    fn rebuild(&mut self, expr: &'term Expr, depth: u32) -> Option<Expr> {
        match expr {
            Expr::FVar(id) if *id == self.fvar => Some(Expr::BVar(depth)),
            // Loose: bound outside `expr`; the new binder adds one level.
            // (An index that cannot be incremented stays where it is: it is
            // above every binder there can be, so it stays loose.)
            Expr::BVar(n) if *n >= depth => Some(Expr::BVar(n.saturating_add(1))),
            Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {
                None
            }
            Expr::App(f, a) => {
                let f_new = self.node(f, depth);
                let a_new = self.node(a, depth);
                if f_new.is_none() && a_new.is_none() {
                    return None;
                }
                Some(Expr::App(
                    f_new.unwrap_or_else(|| f.clone()),
                    a_new.unwrap_or_else(|| a.clone()),
                ))
            }
            Expr::Lam(bi, name, ty, body) => {
                let ty_new = self.node(ty, depth);
                let body_new = self.node(body, depth + 1);
                if ty_new.is_none() && body_new.is_none() {
                    return None;
                }
                Some(Expr::Lam(
                    *bi,
                    name.clone(),
                    ty_new.unwrap_or_else(|| ty.clone()),
                    body_new.unwrap_or_else(|| body.clone()),
                ))
            }
            Expr::Pi(bi, name, ty, body) => {
                let ty_new = self.node(ty, depth);
                let body_new = self.node(body, depth + 1);
                if ty_new.is_none() && body_new.is_none() {
                    return None;
                }
                Some(Expr::Pi(
                    *bi,
                    name.clone(),
                    ty_new.unwrap_or_else(|| ty.clone()),
                    body_new.unwrap_or_else(|| body.clone()),
                ))
            }
            Expr::Let(name, ty, val, body) => {
                let ty_new = self.node(ty, depth);
                let val_new = self.node(val, depth);
                let body_new = self.node(body, depth + 1);
                if ty_new.is_none() && val_new.is_none() && body_new.is_none() {
                    return None;
                }
                Some(Expr::Let(
                    name.clone(),
                    ty_new.unwrap_or_else(|| ty.clone()),
                    val_new.unwrap_or_else(|| val.clone()),
                    body_new.unwrap_or_else(|| body.clone()),
                ))
            }
            Expr::Proj(name, idx, e) => {
                let e_new = self.node(e, depth)?;
                Some(Expr::Proj(name.clone(), *idx, e_new))
            }
        }
    }
}
/// A finite mapping from free-variable identifiers to expressions.
pub type SubstMap = HashMap<FVarId, Expr>;
/// Simultaneously substitute `BVar(0)..BVar(k-1)` with `args[0]..args[k-1]`.
pub fn instantiate_many(expr: &Expr, args: &[Expr]) -> Expr {
    let k = args.len() as u32;
    if k == 0 {
        return expr.clone();
    }
    instantiate_many_at(expr, args, k, 0)
}
fn instantiate_many_at(expr: &Expr, args: &[Expr], k: u32, offset: u32) -> Expr {
    // C16b: see `instantiate_at` — one fuel unit per visited (rebuilt) node.
    crate::fuel::charge(1);
    match expr {
        Expr::BVar(n) => {
            let idx = *n;
            if idx >= offset && idx < offset + k {
                args[(idx - offset) as usize].clone()
            } else if idx >= offset + k {
                Expr::BVar(idx - k)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(instantiate_many_at(f, args, k, offset)),
            Node::new(instantiate_many_at(a, args, k, offset)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Node::new(instantiate_many_at(ty, args, k, offset)),
            Node::new(instantiate_many_at(body, args, k, offset + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Node::new(instantiate_many_at(ty, args, k, offset)),
            Node::new(instantiate_many_at(body, args, k, offset + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Node::new(instantiate_many_at(ty, args, k, offset)),
            Node::new(instantiate_many_at(val, args, k, offset)),
            Node::new(instantiate_many_at(body, args, k, offset + 1)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Node::new(instantiate_many_at(e, args, k, offset)),
        ),
    }
}
/// Apply a parallel substitution: replace each `FVar(id)` with the corresponding
/// expression in `map`.
pub fn parallel_subst(expr: &Expr, map: &SubstMap) -> Expr {
    if map.is_empty() {
        return expr.clone();
    }
    parallel_subst_impl(expr, map)
}
fn parallel_subst_impl(expr: &Expr, map: &SubstMap) -> Expr {
    // C16b: see `instantiate_at` — one fuel unit per visited (rebuilt) node.
    crate::fuel::charge(1);
    match expr {
        Expr::FVar(id) => {
            if let Some(replacement) = map.get(id) {
                replacement.clone()
            } else {
                expr.clone()
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(parallel_subst_impl(f, map)),
            Node::new(parallel_subst_impl(a, map)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Node::new(parallel_subst_impl(ty, map)),
            Node::new(parallel_subst_impl(body, map)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Node::new(parallel_subst_impl(ty, map)),
            Node::new(parallel_subst_impl(body, map)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Node::new(parallel_subst_impl(ty, map)),
            Node::new(parallel_subst_impl(val, map)),
            Node::new(parallel_subst_impl(body, map)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Node::new(parallel_subst_impl(e, map)))
        }
    }
}
/// Shift all `BVar(i)` with `i >= cutoff` up by `amount`.
pub fn shift_bvars(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    // C16b: see `instantiate_at` — one fuel unit per visited (rebuilt) node.
    crate::fuel::charge(1);
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(*i + amount)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(shift_bvars(f, amount, cutoff)),
            Node::new(shift_bvars(a, amount, cutoff)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Node::new(shift_bvars(ty, amount, cutoff)),
            Node::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Node::new(shift_bvars(ty, amount, cutoff)),
            Node::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Node::new(shift_bvars(ty, amount, cutoff)),
            Node::new(shift_bvars(val, amount, cutoff)),
            Node::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Node::new(shift_bvars(e, amount, cutoff)),
        ),
    }
}
/// Collect the set of free `FVarId`s occurring in `expr`.
pub fn free_vars(expr: &Expr) -> std::collections::HashSet<FVarId> {
    let mut set = std::collections::HashSet::new();
    free_vars_impl(expr, &mut set);
    set
}
fn free_vars_impl(expr: &Expr, set: &mut std::collections::HashSet<FVarId>) {
    match expr {
        Expr::FVar(id) => {
            set.insert(*id);
        }
        Expr::App(f, a) => {
            free_vars_impl(f, set);
            free_vars_impl(a, set);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            free_vars_impl(ty, set);
            free_vars_impl(body, set);
        }
        Expr::Let(_, ty, val, body) => {
            free_vars_impl(ty, set);
            free_vars_impl(val, set);
            free_vars_impl(body, set);
        }
        Expr::Proj(_, _, e) => free_vars_impl(e, set),
        _ => {}
    }
}
/// Return `true` if `fvar` occurs free in `expr`.
pub fn occurs_free(expr: &Expr, fvar: FVarId) -> bool {
    match expr {
        Expr::FVar(id) => *id == fvar,
        Expr::App(f, a) => occurs_free(f, fvar) || occurs_free(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            occurs_free(ty, fvar) || occurs_free(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            occurs_free(ty, fvar) || occurs_free(val, fvar) || occurs_free(body, fvar)
        }
        Expr::Proj(_, _, e) => occurs_free(e, fvar),
        _ => false,
    }
}
/// Count how many times `fvar` occurs free in `expr`.
pub fn count_free_occurrences(expr: &Expr, fvar: FVarId) -> usize {
    match expr {
        Expr::FVar(id) => usize::from(*id == fvar),
        Expr::App(f, a) => count_free_occurrences(f, fvar) + count_free_occurrences(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_free_occurrences(ty, fvar) + count_free_occurrences(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            count_free_occurrences(ty, fvar)
                + count_free_occurrences(val, fvar)
                + count_free_occurrences(body, fvar)
        }
        Expr::Proj(_, _, e) => count_free_occurrences(e, fvar),
        _ => 0,
    }
}
/// Like [`instantiate`] but also fills in [`SubstStats`].
pub fn instantiate_tracked(body: &Expr, arg: &Expr, stats: &mut SubstStats) -> Expr {
    instantiate_tracked_at(body, arg, 0, stats)
}
fn instantiate_tracked_at(expr: &Expr, arg: &Expr, depth: u32, stats: &mut SubstStats) -> Expr {
    // C16b: see `instantiate_at` — one fuel unit per visited (rebuilt) node.
    crate::fuel::charge(1);
    stats.nodes_visited += 1;
    match expr {
        Expr::BVar(n) => {
            if *n == depth {
                stats.bvar_hits += 1;
                // Same capture-avoidance as `instantiate_at`: lift the loose
                // bound variables of `arg` past the binders it is inserted
                // under.
                crate::expr_util::lift_loose_bvars(arg, depth, 0)
            } else if *n > depth {
                stats.bvar_misses += 1;
                Expr::BVar(*n - 1)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(instantiate_tracked_at(f, arg, depth, stats)),
            Node::new(instantiate_tracked_at(a, arg, depth, stats)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Node::new(instantiate_tracked_at(ty, arg, depth, stats)),
            Node::new(instantiate_tracked_at(body, arg, depth + 1, stats)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Node::new(instantiate_tracked_at(ty, arg, depth, stats)),
            Node::new(instantiate_tracked_at(body, arg, depth + 1, stats)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Node::new(instantiate_tracked_at(ty, arg, depth, stats)),
            Node::new(instantiate_tracked_at(val, arg, depth, stats)),
            Node::new(instantiate_tracked_at(body, arg, depth + 1, stats)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Node::new(instantiate_tracked_at(e, arg, depth, stats)),
        ),
    }
}
/// Build a `SubstMap` from parallel slices of ids and replacements.
pub fn build_subst_map(fvars: &[FVarId], replacements: &[Expr]) -> SubstMap {
    assert_eq!(fvars.len(), replacements.len());
    fvars
        .iter()
        .zip(replacements.iter())
        .map(|(id, expr)| (*id, expr.clone()))
        .collect()
}
/// Apply beta-reduction once at the top level if possible.
pub fn try_beta_reduce(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_, _, _, body) = f.as_ref() {
                Some(instantiate(body, a))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Repeatedly apply top-level beta reduction until no more is possible.
pub fn beta_reduce_head(expr: Expr) -> Expr {
    let mut curr = expr;
    while let Some(reduced) = try_beta_reduce(&curr) {
        curr = reduced;
    }
    curr
}
/// Substitute a single free variable with an expression.
pub fn subst_fvar(expr: &Expr, fvar: FVarId, replacement: &Expr) -> Expr {
    let mut map = SubstMap::new();
    map.insert(fvar, replacement.clone());
    parallel_subst(expr, &map)
}
/// Check whether `expr` is in weak head normal form with respect to beta.
pub fn is_whnf_beta(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => match f.as_ref() {
            Expr::Lam(_, _, _, _) => false,
            other => is_whnf_beta(other),
        },
        _ => true,
    }
}
/// Collect the indices of all loose bound variables.
pub fn collect_loose_bvar_indices(expr: &Expr) -> Vec<u32> {
    let mut indices = Vec::new();
    collect_loose_bvar_impl(expr, 0, &mut indices);
    indices.sort();
    indices.dedup();
    indices
}
fn collect_loose_bvar_impl(expr: &Expr, depth: u32, result: &mut Vec<u32>) {
    match expr {
        Expr::BVar(n) if *n >= depth => {
            result.push(*n - depth);
        }
        Expr::BVar(_) => {}
        Expr::App(f, a) => {
            collect_loose_bvar_impl(f, depth, result);
            collect_loose_bvar_impl(a, depth, result);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_loose_bvar_impl(ty, depth, result);
            collect_loose_bvar_impl(body, depth + 1, result);
        }
        Expr::Let(_, ty, val, body) => {
            collect_loose_bvar_impl(ty, depth, result);
            collect_loose_bvar_impl(val, depth, result);
            collect_loose_bvar_impl(body, depth + 1, result);
        }
        Expr::Proj(_, _, e) => collect_loose_bvar_impl(e, depth, result),
        _ => {}
    }
}
/// Apply a parallel substitution and collect profiling statistics.
pub fn parallel_subst_tracked(expr: &Expr, map: &SubstMap, stats: &mut SubstStats) -> Expr {
    if map.is_empty() {
        return expr.clone();
    }
    parallel_subst_tracked_impl(expr, map, stats)
}
fn parallel_subst_tracked_impl(expr: &Expr, map: &SubstMap, stats: &mut SubstStats) -> Expr {
    stats.nodes_visited += 1;
    match expr {
        Expr::FVar(id) => {
            if let Some(r) = map.get(id) {
                stats.fvar_hits += 1;
                r.clone()
            } else {
                expr.clone()
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Node::new(parallel_subst_tracked_impl(f, map, stats)),
            Node::new(parallel_subst_tracked_impl(a, map, stats)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Node::new(parallel_subst_tracked_impl(ty, map, stats)),
            Node::new(parallel_subst_tracked_impl(body, map, stats)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Node::new(parallel_subst_tracked_impl(ty, map, stats)),
            Node::new(parallel_subst_tracked_impl(body, map, stats)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Node::new(parallel_subst_tracked_impl(ty, map, stats)),
            Node::new(parallel_subst_tracked_impl(val, map, stats)),
            Node::new(parallel_subst_tracked_impl(body, map, stats)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Node::new(parallel_subst_tracked_impl(e, map, stats)),
        ),
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    fn fvar(id: u64) -> Expr {
        Expr::FVar(FVarId(id))
    }
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::nat(n))
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_instantiate_many_two() {
        let app = Expr::App(Node::new(Expr::BVar(0)), Node::new(Expr::BVar(1)));
        let result = instantiate_many(&app, &[lit(10), lit(20)]);
        match result {
            Expr::App(f, a) => {
                assert_eq!(*f, lit(10));
                assert_eq!(*a, lit(20));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_parallel_subst_single() {
        let mut map = SubstMap::new();
        map.insert(FVarId(1), lit(99));
        let expr = fvar(1);
        assert_eq!(parallel_subst(&expr, &map), lit(99));
    }
    #[test]
    fn test_parallel_subst_empty() {
        let expr = fvar(5);
        let map = SubstMap::new();
        assert_eq!(parallel_subst(&expr, &map), expr);
    }
    #[test]
    fn test_shift_bvars_above_cutoff() {
        let e = Expr::BVar(2);
        let shifted = shift_bvars(&e, 3, 1);
        assert_eq!(shifted, Expr::BVar(5));
    }
    #[test]
    fn test_shift_bvars_below_cutoff() {
        let e = Expr::BVar(0);
        let shifted = shift_bvars(&e, 2, 1);
        assert_eq!(shifted, Expr::BVar(0));
    }
    #[test]
    fn test_free_vars_single() {
        let set = free_vars(&fvar(7));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&FVarId(7)));
    }
    #[test]
    fn test_occurs_free_true() {
        let expr = Expr::App(Node::new(fvar(1)), Node::new(fvar(2)));
        assert!(occurs_free(&expr, FVarId(1)));
        assert!(!occurs_free(&expr, FVarId(3)));
    }
    #[test]
    fn test_count_free_occurrences() {
        let expr = Expr::App(Node::new(fvar(1)), Node::new(fvar(1)));
        assert_eq!(count_free_occurrences(&expr, FVarId(1)), 2);
    }
    #[test]
    fn test_subst_stats_merge() {
        let mut a = SubstStats {
            bvar_hits: 3,
            bvar_misses: 1,
            fvar_hits: 2,
            nodes_visited: 10,
        };
        let b = SubstStats {
            bvar_hits: 1,
            bvar_misses: 0,
            fvar_hits: 1,
            nodes_visited: 5,
        };
        a.merge(&b);
        assert_eq!(a.bvar_hits, 4);
        assert_eq!(a.total_substs(), 7);
    }
    #[test]
    fn test_instantiate_tracked() {
        let body = Expr::BVar(0);
        let arg = lit(42);
        let mut stats = SubstStats::default();
        let result = instantiate_tracked(&body, &arg, &mut stats);
        assert_eq!(result, lit(42));
        assert_eq!(stats.bvar_hits, 1);
    }
    #[test]
    fn test_build_subst_map() {
        let fvars = vec![FVarId(1), FVarId(2)];
        let replacements = vec![lit(10), lit(20)];
        let map = build_subst_map(&fvars, &replacements);
        assert_eq!(map.get(&FVarId(1)), Some(&lit(10)));
    }
    #[test]
    fn test_try_beta_reduce_success() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(sort0()),
            Node::new(Expr::BVar(0)),
        );
        let app = Expr::App(Node::new(lam), Node::new(lit(42)));
        let result = try_beta_reduce(&app).expect("result should be present");
        assert_eq!(result, lit(42));
    }
    #[test]
    fn test_try_beta_reduce_fail() {
        let app = Expr::App(Node::new(fvar(1)), Node::new(lit(42)));
        assert!(try_beta_reduce(&app).is_none());
    }
    #[test]
    fn test_beta_reduce_head() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(sort0()),
            Node::new(Expr::BVar(0)),
        );
        let app = Expr::App(Node::new(lam), Node::new(lit(7)));
        let result = beta_reduce_head(app);
        assert_eq!(result, lit(7));
    }
    #[test]
    fn test_subst_fvar() {
        let expr = fvar(1);
        let result = subst_fvar(&expr, FVarId(1), &lit(99));
        assert_eq!(result, lit(99));
    }
    #[test]
    fn test_is_whnf_beta_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(sort0()),
            Node::new(Expr::BVar(0)),
        );
        assert!(is_whnf_beta(&lam));
    }
    #[test]
    fn test_is_whnf_beta_app_not_whnf() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(sort0()),
            Node::new(Expr::BVar(0)),
        );
        let app = Expr::App(Node::new(lam), Node::new(lit(1)));
        assert!(!is_whnf_beta(&app));
    }
    #[test]
    fn test_collect_loose_bvar_indices() {
        let e = Expr::App(Node::new(Expr::BVar(0)), Node::new(Expr::BVar(2)));
        let indices = collect_loose_bvar_indices(&e);
        assert!(indices.contains(&0));
        assert!(indices.contains(&2));
    }
    #[test]
    fn test_parallel_subst_tracked() {
        let expr = Expr::App(Node::new(fvar(1)), Node::new(fvar(2)));
        let mut map = SubstMap::new();
        map.insert(FVarId(1), lit(10));
        map.insert(FVarId(2), lit(20));
        let mut stats = SubstStats::default();
        let result = parallel_subst_tracked(&expr, &map, &mut stats);
        assert_eq!(stats.fvar_hits, 2);
        match result {
            Expr::App(f, a) => {
                assert_eq!(*f, lit(10));
                assert_eq!(*a, lit(20));
            }
            _ => panic!("Expected App"),
        }
    }
}
/// Apply a list of arguments to a function expression via beta reduction.
///
/// Given `f` and `[a1, a2, ..., an]`, returns `((...((f a1) a2) ...) an)`
/// after instantiating each lambda body with the corresponding argument.
pub fn apply_args(f: &Expr, args: &[Expr]) -> Expr {
    let mut result = f.clone();
    for arg in args {
        if let Expr::Lam(_, _, _, body) = result {
            result = instantiate(&body, arg);
        } else {
            result = Expr::App(Node::new(result), Node::new(arg.clone()));
        }
    }
    result
}
/// Check if an expression is a lambda abstraction.
#[inline]
pub fn is_lambda(e: &Expr) -> bool {
    matches!(e, Expr::Lam(_, _, _, _))
}
/// Peel off `n` leading lambda binders, collecting binder info.
///
/// Returns `(binders, body)` where `binders` is a vector of `(name, binder_info, type)`
/// tuples and `body` is the inner expression with `n` fewer lambdas.
#[allow(clippy::type_complexity)]
pub fn peel_lambdas(e: &Expr, n: usize) -> (Vec<(crate::Name, crate::BinderInfo, Node)>, &Expr) {
    let mut binders = Vec::new();
    let mut cur = e;
    for _ in 0..n {
        if let Expr::Lam(bi, _, ty, body) = cur {
            binders.push((crate::Name::anonymous(), *bi, ty.clone()));
            cur = body;
        } else {
            break;
        }
    }
    (binders, cur)
}
/// Count the number of leading lambda binders.
pub fn count_lambdas(e: &Expr) -> usize {
    let mut n = 0;
    let mut cur = e;
    while let Expr::Lam(_, _, _, body) = cur {
        n += 1;
        cur = body;
    }
    n
}
/// Count the number of leading pi binders.
pub fn count_pis(e: &Expr) -> usize {
    let mut n = 0;
    let mut cur = e;
    while let Expr::Pi(_, _, _, body) = cur {
        n += 1;
        cur = body;
    }
    n
}
#[cfg(test)]
mod extended2_tests {
    use super::*;
    use crate::{BinderInfo, Expr, FVarId, Literal, Name};
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::nat(n))
    }
    fn fvar(id: u64) -> Expr {
        Expr::FVar(FVarId(id))
    }
    #[test]
    fn test_apply_args_lam() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(body),
        );
        let result = apply_args(&lam, &[lit(42)]);
        assert_eq!(result, lit(42));
    }
    #[test]
    fn test_apply_args_non_lam() {
        let f = fvar(1);
        let result = apply_args(&f, &[lit(1), lit(2)]);
        match &result {
            Expr::App(outer, arg2) => {
                assert_eq!(**arg2, lit(2));
                match outer.as_ref() {
                    Expr::App(inner_f, arg1) => {
                        assert_eq!(**inner_f, fvar(1));
                        assert_eq!(**arg1, lit(1));
                    }
                    _ => panic!("Expected App"),
                }
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_is_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(lit(1)),
        );
        assert!(is_lambda(&lam));
        assert!(!is_lambda(&lit(0)));
    }
    #[test]
    fn test_count_lambdas() {
        let inner = lit(0);
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(inner),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(lam1),
        );
        assert_eq!(count_lambdas(&lam2), 2);
        assert_eq!(count_lambdas(&lit(0)), 0);
    }
    #[test]
    fn test_count_pis() {
        let inner = lit(0);
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(inner),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(pi1),
        );
        assert_eq!(count_pis(&pi2), 2);
        assert_eq!(count_pis(&lit(0)), 0);
    }
    #[test]
    fn test_peel_lambdas_two() {
        let body = Expr::BVar(0);
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(body),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Implicit,
            Name::str("_"),
            Node::new(lit(0)),
            Node::new(lam1),
        );
        let (binders, inner) = peel_lambdas(&lam2, 2);
        assert_eq!(binders.len(), 2);
        assert_eq!(binders[0].1, BinderInfo::Implicit);
        assert_eq!(binders[1].1, BinderInfo::Default);
        assert_eq!(*inner, Expr::BVar(0));
    }
    #[test]
    fn test_apply_args_empty() {
        let e = lit(5);
        let result = apply_args(&e, &[]);
        assert_eq!(result, e);
    }
}
/// Apply a substitution list (as parallel arrays) to an expression.
#[allow(dead_code)]
pub fn substitute_fvars(expr: &Expr, fvars: &[FVarId], replacements: &[Expr]) -> Expr {
    assert_eq!(fvars.len(), replacements.len());
    let mut subst = Substitution::new();
    for (fvar, rep) in fvars.iter().zip(replacements.iter()) {
        subst.insert(*fvar, rep.clone());
    }
    subst.apply(expr)
}
/// Check if an expression contains any free variable in the given set.
#[allow(dead_code)]
pub fn expr_contains_fvar(expr: &Expr, fvars: &[FVarId]) -> bool {
    match expr {
        Expr::FVar(id) => fvars.contains(id),
        Expr::App(f, a) => expr_contains_fvar(f, fvars) || expr_contains_fvar(a, fvars),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            expr_contains_fvar(ty, fvars) || expr_contains_fvar(body, fvars)
        }
        Expr::Let(_, ty, val, body) => {
            expr_contains_fvar(ty, fvars)
                || expr_contains_fvar(val, fvars)
                || expr_contains_fvar(body, fvars)
        }
        Expr::Proj(_, _, inner) => expr_contains_fvar(inner, fvars),
        _ => false,
    }
}
/// Replace the outermost `BVar(0)` of a body with `FVar(id)` (open a binder).
///
/// This is the "open" operation in locally nameless representation.
#[allow(dead_code)]
pub fn open_binder(body: &Expr, id: FVarId) -> Expr {
    open_binder_at(body, &Expr::FVar(id), 0)
}
fn open_binder_at(expr: &Expr, fvar_expr: &Expr, depth: u32) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n == depth {
                fvar_expr.clone()
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = open_binder_at(f, fvar_expr, depth);
            let a2 = open_binder_at(a, fvar_expr, depth);
            Expr::App(Node::new(f2), Node::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = open_binder_at(ty, fvar_expr, depth);
            let body2 = open_binder_at(body, fvar_expr, depth + 1);
            Expr::Lam(*bi, name.clone(), Node::new(ty2), Node::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = open_binder_at(ty, fvar_expr, depth);
            let body2 = open_binder_at(body, fvar_expr, depth + 1);
            Expr::Pi(*bi, name.clone(), Node::new(ty2), Node::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = open_binder_at(ty, fvar_expr, depth);
            let val2 = open_binder_at(val, fvar_expr, depth);
            let body2 = open_binder_at(body, fvar_expr, depth + 1);
            Expr::Let(
                name.clone(),
                Node::new(ty2),
                Node::new(val2),
                Node::new(body2),
            )
        }
        Expr::Proj(name, idx, inner) => {
            let inner2 = open_binder_at(inner, fvar_expr, depth);
            Expr::Proj(name.clone(), *idx, Node::new(inner2))
        }
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod extended3_subst_tests {
    use super::*;
    use crate::{BinderInfo, Expr, FVarId, Literal, Name};
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::nat(n))
    }
    fn fvar(id: u64) -> Expr {
        Expr::FVar(FVarId(id))
    }
    #[test]
    fn test_substitution_insert_and_get() {
        let mut s = Substitution::new();
        s.insert(FVarId(0), lit(42));
        assert_eq!(s.get(FVarId(0)), Some(&lit(42)));
        assert_eq!(s.get(FVarId(1)), None);
    }
    #[test]
    fn test_substitution_len() {
        let mut s = Substitution::new();
        assert_eq!(s.len(), 0);
        s.insert(FVarId(0), lit(1));
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_substitution_apply_fvar() {
        let mut s = Substitution::new();
        s.insert(FVarId(1), lit(99));
        let expr = fvar(1);
        assert_eq!(s.apply(&expr), lit(99));
    }
    #[test]
    fn test_substitution_apply_no_match() {
        let s = Substitution::new();
        let expr = fvar(5);
        assert_eq!(s.apply(&expr), fvar(5));
    }
    #[test]
    fn test_substitution_remove() {
        let mut s = Substitution::new();
        s.insert(FVarId(0), lit(1));
        s.remove(FVarId(0));
        assert_eq!(s.len(), 0);
    }
    #[test]
    fn test_substitution_restrict() {
        let mut s = Substitution::new();
        s.insert(FVarId(0), lit(1));
        s.insert(FVarId(1), lit(2));
        let r = s.restrict(&[FVarId(0)]);
        assert_eq!(r.len(), 1);
        assert!(r.get(FVarId(0)).is_some());
        assert!(r.get(FVarId(1)).is_none());
    }
    #[test]
    fn test_substitute_fvars_parallel() {
        let expr = Expr::App(Node::new(fvar(0)), Node::new(fvar(1)));
        let result = substitute_fvars(&expr, &[FVarId(0), FVarId(1)], &[lit(10), lit(20)]);
        match result {
            Expr::App(f, a) => {
                assert_eq!(*f, lit(10));
                assert_eq!(*a, lit(20));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_expr_contains_fvar_true() {
        let expr = Expr::App(Node::new(fvar(3)), Node::new(lit(0)));
        assert!(expr_contains_fvar(&expr, &[FVarId(3)]));
    }
    #[test]
    fn test_expr_contains_fvar_false() {
        let expr = lit(5);
        assert!(!expr_contains_fvar(&expr, &[FVarId(0)]));
    }
    #[test]
    fn test_open_binder_simple() {
        let body = Expr::BVar(0);
        let result = open_binder(&body, FVarId(7));
        assert_eq!(result, fvar(7));
    }
    #[test]
    fn test_open_binder_nested() {
        let inner_body = Expr::BVar(0);
        let inner_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(inner_body),
        );
        let outer_body = Expr::App(Node::new(inner_lam), Node::new(Expr::BVar(0)));
        let opened = open_binder(&outer_body, FVarId(99));
        match opened {
            Expr::App(f, arg) => {
                assert!(matches!(*f, Expr::Lam(_, _, _, _)));
                assert_eq!(*arg, fvar(99));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_substitution_compose() {
        let mut s1 = Substitution::new();
        s1.insert(FVarId(0), lit(10));
        let mut s2 = Substitution::new();
        s2.insert(FVarId(1), lit(20));
        let composed = s1.compose(&s2);
        assert_eq!(composed.get(FVarId(0)), Some(&lit(10)));
        assert_eq!(composed.get(FVarId(1)), Some(&lit(20)));
    }
    #[test]
    fn test_substitution_insert_replace() {
        let mut s = Substitution::new();
        s.insert(FVarId(0), lit(1));
        s.insert(FVarId(0), lit(2));
        assert_eq!(s.len(), 1);
        assert_eq!(s.get(FVarId(0)), Some(&lit(2)));
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 0);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        assert_eq!(calc.add(), Some(7));
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        assert_eq!(calc.mul(), Some(14));
        assert_eq!(calc.peek(), Some(14));
    }
}
