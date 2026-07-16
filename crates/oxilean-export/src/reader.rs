//! The streaming NDJSON reader.
//!
//! Feeds lines from any [`BufRead`], dispatches each record, maintains the
//! name/level/expr index tables (Vec-based, dense, 0-based per the spec), and
//! validates the forward-reference and duplicate-index discipline.
//!
//! ## Sharing, bombs, and the materialization budget
//!
//! The export format shares subterms by index: one record may be referenced by
//! many later records, so the *tree* a record denotes can be exponentially
//! larger than the file (`{"app":{"fn":k,"arg":k}}` doubles at every step).
//! The kernel's `Expr` is `Box`-based (no sharing), so naively storing kernel
//! expressions per index would be a memory/time bomb on untrusted input.
//!
//! The reader therefore stores **index nodes** (`ENode`, `LNode`) — O(1) per
//! record — alongside a memoized subtree size (saturating `u64`). Kernel
//! `Expr`/`Level` trees are only materialized at **declaration boundaries**
//! (types, values, recursor rule RHS), and every materialization is charged
//! against a cumulative node budget ([`Limits::materialize_budget`]) *before*
//! any allocation happens. Exceeding the budget yields a named `Unsupported`
//! error, never an OOM. Materializers are iterative (explicit stacks), so
//! parser stack depth is independent of term depth.

use std::io::BufRead;

use oxilean_kernel::{
    AxiomVal, BigNat, BinderInfo, ConstantVal, ConstructorVal, DefinitionSafety, DefinitionVal,
    Expr, InductiveVal, Level, Literal, Name, Node, OpaqueVal, QuotKind, QuotVal, RecursorRule,
    RecursorVal, ReducibilityHint, TheoremVal,
};

use crate::convert::{parse_index, Obj};
use crate::error::{ExportError, ExportResult, Position};
use crate::json::{self, JsonValue};
use crate::model::{ExportDecl, InductiveBundle, Meta, OversizedVal};

/// Default cumulative materialization budget, in expression/level tree nodes.
///
/// Materialized kernel nodes cost roughly 60–100 bytes each, so the default
/// (2^26 nodes) bounds reader-materialized memory at a few GiB even for
/// adversarial sharing. Callers with larger (trusted) inputs can raise it via
/// [`Limits`].
pub const DEFAULT_MATERIALIZE_BUDGET: u64 = 1 << 26;

/// The named feature reported when the materialization budget is exceeded.
pub const BUDGET_FEATURE: &str =
    "expression materialization budget exceeded (indexed sharing expands too large)";

/// Materialization budget preset for whole-corpus reads/replays (see
/// [`Limits::corpus`]): 2^30 ≈ 1.07 billion nodes.
pub const CORPUS_MATERIALIZE_BUDGET: u64 = 1 << 30;

/// The named feature reported when a SINGLE declaration's kernel trees exceed
/// the per-declaration materialization budget
/// ([`Limits::decl_materialize_budget`]).
///
/// C22: the cumulative budget alone cannot stop one declaration from
/// legally DAG-sharing into a tree of hundreds of millions of nodes —
/// Lean core's `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix`
/// (Init decl #42,086) materializes >100M nodes (≈7+ GiB) in one theorem
/// value and OOM-killed a 12 GiB cage before the kernel ever ran. The
/// per-declaration cap turns that into this NAMED unsupported bucket
/// (checked in O(1) against the precomputed size table, before any
/// expansion work), and the replayer cascades its dependents exactly like
/// any other unsupported declaration — never a wrong verdict, never an OOM.
pub const DECL_BUDGET_FEATURE: &str = "declaration materializes too large (per-declaration budget)";

/// Per-declaration materialization cap of the corpus preset (see
/// [`Limits::corpus`]): 2^25 ≈ 33.5M nodes. Materialized kernel nodes cost
/// roughly 60–100 bytes each, so one declaration is bounded at ≈2–3.4 GiB.
/// Measured on `Init.ndjson` (Lean v4.32.0-rc1): the largest declaration a
/// 14 GiB machine can actually check materializes well under this cap, while
/// the known over-budget family (`WellFounded.partialExtrinsicFix₃_eq_*`,
/// >100M nodes each) is cut off cheaply.
pub const CORPUS_DECL_MATERIALIZE_BUDGET: u64 = 1 << 25;

/// Resource limits for a read.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Cumulative cap on materialized kernel tree nodes across the whole file.
    pub materialize_budget: u64,
    /// Cap on the kernel tree nodes materialized by a SINGLE declaration
    /// record (all its types, values and recursor rule RHSes together). A
    /// declaration over this cap is surfaced as
    /// [`ExportDecl::Oversized`](crate::ExportDecl::Oversized) — a NAMED
    /// unsupported declaration ([`DECL_BUDGET_FEATURE`]) — instead of
    /// aborting the read or exhausting memory (C22).
    pub decl_materialize_budget: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            materialize_budget: DEFAULT_MATERIALIZE_BUDGET,
            // For untrusted inputs the cumulative budget is already the
            // strongest bound; a distinct per-declaration cap only matters
            // for large trusted corpora.
            decl_materialize_budget: DEFAULT_MATERIALIZE_BUDGET,
        }
    }
}

impl Limits {
    /// The documented whole-corpus preset for **trusted** large exports.
    ///
    /// Raises the node budget to [`CORPUS_MATERIALIZE_BUDGET`] (2^30 nodes).
    /// Measured on `Init.ndjson` (Lean 4.32.0-rc1, 6,453,270 lines, 57,277
    /// declarations): the full corpus materializes 908,550,041 kernel nodes —
    /// 13.5× the default budget — and a streaming read peaked at ~8.4 GiB RSS
    /// in release mode, dominated by a few very large individual declaration
    /// terms (the kernel `Expr` is `Box`-based, no structural sharing) plus
    /// the ~6.1M-entry index tables. The preset covers Init with ~18%
    /// headroom.
    ///
    /// The default budget stays small on purpose: it is the untrusted-input
    /// bound. Only use this preset on corpora you trust to be non-adversarial
    /// (kernel-side structural sharing to shrink the RSS cost is tracked
    /// separately on the TCB roadmap).
    #[must_use]
    pub fn corpus() -> Self {
        Self {
            materialize_budget: CORPUS_MATERIALIZE_BUDGET,
            decl_materialize_budget: CORPUS_DECL_MATERIALIZE_BUDGET,
        }
    }
}

/// The result of fully reading an export stream.
#[derive(Debug, Clone)]
pub struct ExportFile {
    /// The parsed metadata header.
    pub meta: Meta,
    /// The declarations, in file order.
    pub decls: Vec<ExportDecl>,
    /// Per-record-type counts (for reporting).
    pub stats: ReadStats,
}

/// Counts of each record kind consumed, for three-bucket reporting.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadStats {
    /// Total non-empty lines consumed (including the meta line).
    pub total_lines: usize,
    /// `Name.str` records.
    pub name_str: usize,
    /// `Name.num` records.
    pub name_num: usize,
    /// `Level.succ` records.
    pub level_succ: usize,
    /// `Level.max` records.
    pub level_max: usize,
    /// `Level.imax` records.
    pub level_imax: usize,
    /// `Level.param` records.
    pub level_param: usize,
    /// `Expr.bvar` records.
    pub expr_bvar: usize,
    /// `Expr.sort` records.
    pub expr_sort: usize,
    /// `Expr.const` records.
    pub expr_const: usize,
    /// `Expr.app` records.
    pub expr_app: usize,
    /// `Expr.lam` records.
    pub expr_lam: usize,
    /// `Expr.forallE` records.
    pub expr_forall: usize,
    /// `Expr.letE` records.
    pub expr_let: usize,
    /// `Expr.proj` records.
    pub expr_proj: usize,
    /// `Expr.natVal` literal records.
    pub expr_nat_lit: usize,
    /// `Expr.strVal` literal records.
    pub expr_str_lit: usize,
    /// `Expr.mdata` records.
    pub expr_mdata: usize,
    /// `axiom` declaration records.
    pub decl_axiom: usize,
    /// `def` declaration records.
    pub decl_def: usize,
    /// `thm` declaration records.
    pub decl_thm: usize,
    /// `opaque` declaration records.
    pub decl_opaque: usize,
    /// `quot` declaration records.
    pub decl_quot: usize,
    /// `inductive` bundle records.
    pub decl_inductive: usize,
    /// Total kernel tree nodes materialized for declarations (charged against
    /// the budget).
    pub materialized_nodes: u64,
    /// Largest node count any single declaration record materialized (for
    /// calibrating [`Limits::decl_materialize_budget`]).
    pub max_decl_materialized: u64,
    /// Declaration records skipped as [`ExportDecl::Oversized`] because they
    /// exceeded the per-declaration materialization budget.
    pub decl_oversized: usize,
}

impl ReadStats {
    /// Total number of declaration records (an inductive bundle counts once).
    #[must_use]
    pub fn total_decls(&self) -> usize {
        self.decl_axiom
            + self.decl_def
            + self.decl_thm
            + self.decl_opaque
            + self.decl_quot
            + self.decl_inductive
            + self.decl_oversized
    }
}

/// Read a complete export file from any [`BufRead`] with default [`Limits`].
///
/// # Errors
/// Returns a three-bucket [`ExportError`] on the first malformed record,
/// unsupported construct, or I/O failure.
pub fn read<R: BufRead>(reader: R) -> ExportResult<ExportFile> {
    read_with_limits(reader, Limits::default())
}

/// Read a complete export file with explicit [`Limits`].
///
/// # Errors
/// See [`read`].
pub fn read_with_limits<R: BufRead>(reader: R, limits: Limits) -> ExportResult<ExportFile> {
    let mut decls = Vec::new();
    let (meta, stats) = read_streaming(reader, limits, |d| {
        decls.push(d);
        Ok(())
    })?;
    Ok(ExportFile { meta, decls, stats })
}

/// Convenience: read an export file from a string slice with default limits.
///
/// # Errors
/// See [`read`].
pub fn read_str(s: &str) -> ExportResult<ExportFile> {
    read(std::io::Cursor::new(s))
}

/// Streaming read: `on_decl` is invoked for each declaration as soon as it is
/// parsed, and declarations are NOT retained by the reader. This keeps resident
/// memory proportional to the primitive tables even for very large corpora.
///
/// # Errors
/// See [`read`]; errors returned by `on_decl` abort the read.
pub fn read_streaming<R: BufRead>(
    reader: R,
    limits: Limits,
    mut on_decl: impl FnMut(ExportDecl) -> ExportResult<()>,
) -> ExportResult<(Meta, ReadStats)> {
    let mut r = Reader::new(limits);
    let mut meta: Option<Meta> = None;
    let mut line_no = 0usize;

    for line in reader.lines() {
        line_no += 1;
        let line = line.map_err(|e| {
            ExportError::malformed(Position::line_only(line_no), format!("I/O error: {e}"))
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        r.stats.total_lines += 1;

        let value = json::parse(line.as_bytes()).map_err(|e| {
            ExportError::malformed(
                Position::new(line_no, e.offset),
                format!("invalid JSON: {}", e.message),
            )
        })?;

        if meta.is_none() {
            meta = Some(r.parse_meta(&value, line_no)?);
            continue;
        }
        r.dispatch(&value, line_no)?;
        for d in r.pending.drain(..) {
            on_decl(d)?;
        }
    }

    let meta = meta.ok_or_else(|| {
        ExportError::malformed(
            Position::line_only(line_no.max(1)),
            "empty file: no meta record",
        )
    })?;

    Ok((meta, r.stats))
}

/// An interned universe level node (children are level-table indices).
#[derive(Debug, Clone)]
enum LNode {
    /// `Level.zero` — pre-seeded at index 0, never emitted as a record.
    Zero,
    Succ(usize),
    Max(usize, usize),
    IMax(usize, usize),
    Param(Name),
}

/// An interned expression node (children are table indices).
#[derive(Debug, Clone)]
enum ENode {
    BVar(u32),
    /// Level index.
    Sort(usize),
    /// Resolved constant name + level indices.
    Const(Name, Vec<usize>),
    App(usize, usize),
    Lam(BinderInfo, Name, usize, usize),
    Pi(BinderInfo, Name, usize, usize),
    Let(Name, usize, usize, usize),
    NatLit(BigNat),
    StrLit(String),
    Proj(Name, u32, usize),
}

/// The streaming reader state: dense index tables with memoized subtree sizes,
/// pending declarations, statistics and the materialization budget.
struct Reader {
    names: Vec<Name>,
    lnodes: Vec<LNode>,
    lsizes: Vec<u64>,
    enodes: Vec<ENode>,
    esizes: Vec<u64>,
    /// Structural sharing (wave5 Stage B): file-global memo mapping each export
    /// DAG id to the single `Rc<Expr>` it materializes to. An id referenced by
    /// many parents (or many declarations) is built once and shared by
    /// refcount, so materialization cost and residency are proportional to the
    /// number of *distinct* reachable nodes, not the exploded tree size.
    memo: Vec<Option<Node>>,
    pending: Vec<ExportDecl>,
    budget_left: u64,
    /// Per-declaration cap ([`Limits::decl_materialize_budget`]).
    decl_budget: u64,
    /// Nodes materialized by the declaration record currently being read
    /// (reset by `dispatch` at each declaration record).
    decl_used: u64,
    stats: ReadStats,
}

impl Reader {
    fn new(limits: Limits) -> Self {
        // Pre-seed: name[0] = anonymous, level[0] = zero. Exprs start empty.
        Self {
            names: vec![Name::Anonymous],
            lnodes: vec![LNode::Zero],
            lsizes: vec![1],
            enodes: Vec::new(),
            esizes: Vec::new(),
            memo: Vec::new(),
            pending: Vec::new(),
            budget_left: limits.materialize_budget,
            decl_budget: limits.decl_materialize_budget,
            decl_used: 0,
            stats: ReadStats::default(),
        }
    }

    // --- Index-table access with forward-reference checking ---------------

    fn name_at(&self, idx: usize, pos: Position) -> ExportResult<Name> {
        self.names
            .get(idx)
            .cloned()
            .ok_or_else(|| ExportError::malformed(pos, format!("name index {idx} not yet defined")))
    }

    /// Validate that a level index is already defined and return it.
    fn level_ref(&self, idx: usize, pos: Position) -> ExportResult<usize> {
        if idx < self.lnodes.len() {
            Ok(idx)
        } else {
            Err(ExportError::malformed(
                pos,
                format!("level index {idx} not yet defined"),
            ))
        }
    }

    /// Validate that an expression index is already defined and return it.
    fn expr_ref(&self, idx: usize, pos: Position) -> ExportResult<usize> {
        if idx < self.enodes.len() {
            Ok(idx)
        } else {
            Err(ExportError::malformed(
                pos,
                format!("expr index {idx} not yet defined"),
            ))
        }
    }

    /// Insert a name at `idx`, enforcing dense, non-duplicate assignment.
    fn put_name(&mut self, idx: usize, name: Name, pos: Position) -> ExportResult<()> {
        Self::check_slot(self.names.len(), idx, pos, "name")?;
        self.names.push(name);
        Ok(())
    }

    fn put_level(&mut self, idx: usize, node: LNode, pos: Position) -> ExportResult<()> {
        Self::check_slot(self.lnodes.len(), idx, pos, "level")?;
        let size = match &node {
            LNode::Zero | LNode::Param(_) => 1,
            LNode::Succ(a) => 1u64.saturating_add(self.lsize(*a)),
            LNode::Max(a, b) | LNode::IMax(a, b) => 1u64
                .saturating_add(self.lsize(*a))
                .saturating_add(self.lsize(*b)),
        };
        self.lnodes.push(node);
        self.lsizes.push(size);
        Ok(())
    }

    fn put_expr(&mut self, idx: usize, node: ENode, pos: Position) -> ExportResult<()> {
        Self::check_slot(self.enodes.len(), idx, pos, "expr")?;
        let size = match &node {
            ENode::BVar(_) | ENode::NatLit(_) | ENode::StrLit(_) => 1,
            ENode::Sort(l) => 1u64.saturating_add(self.lsize(*l)),
            ENode::Const(_, us) => {
                let mut s = 1u64;
                for u in us {
                    s = s.saturating_add(self.lsize(*u));
                }
                s
            }
            ENode::App(f, a) => 1u64
                .saturating_add(self.esize(*f))
                .saturating_add(self.esize(*a)),
            ENode::Lam(_, _, t, b) | ENode::Pi(_, _, t, b) => 1u64
                .saturating_add(self.esize(*t))
                .saturating_add(self.esize(*b)),
            ENode::Let(_, t, v, b) => 1u64
                .saturating_add(self.esize(*t))
                .saturating_add(self.esize(*v))
                .saturating_add(self.esize(*b)),
            ENode::Proj(_, _, s) => 1u64.saturating_add(self.esize(*s)),
        };
        self.enodes.push(node);
        self.esizes.push(size);
        Ok(())
    }

    fn lsize(&self, idx: usize) -> u64 {
        self.lsizes.get(idx).copied().unwrap_or(u64::MAX)
    }

    fn esize(&self, idx: usize) -> u64 {
        self.esizes.get(idx).copied().unwrap_or(u64::MAX)
    }

    /// Dense-insert precondition: the next assigned index must equal the
    /// current table length (indices arrive in order, no gaps, no reuse).
    fn check_slot(len: usize, idx: usize, pos: Position, what: &str) -> ExportResult<()> {
        if idx == len {
            Ok(())
        } else if idx < len {
            Err(ExportError::malformed(
                pos,
                format!("duplicate {what} index {idx} (already defined)"),
            ))
        } else {
            Err(ExportError::malformed(
                pos,
                format!("non-contiguous {what} index {idx} (expected {len})"),
            ))
        }
    }

    // --- Materialization (budgeted, iterative) -----------------------------

    /// Materialize the kernel [`Level`] for a level index. The caller has
    /// already charged the budget (level sizes are included in expression
    /// sizes, and `materialize_expr` charges the whole tree up front).
    fn build_level(&self, root: usize, pos: Position) -> ExportResult<Level> {
        enum Frame {
            Enter(usize),
            Exit(usize),
        }
        let mut stack = vec![Frame::Enter(root)];
        let mut out: Vec<Level> = Vec::new();
        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Enter(i) => {
                    let node = self.lnodes.get(i).ok_or_else(|| {
                        ExportError::internal(pos, format!("level node {i} missing"))
                    })?;
                    match node {
                        LNode::Zero => out.push(Level::zero()),
                        LNode::Param(n) => out.push(Level::param(n.clone())),
                        LNode::Succ(a) => {
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(*a));
                        }
                        LNode::Max(a, b) | LNode::IMax(a, b) => {
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(*b));
                            stack.push(Frame::Enter(*a));
                        }
                    }
                }
                Frame::Exit(i) => {
                    let node = self.lnodes.get(i).ok_or_else(|| {
                        ExportError::internal(pos, format!("level node {i} missing"))
                    })?;
                    match node {
                        LNode::Succ(_) => {
                            let a = Self::pop(&mut out, pos)?;
                            out.push(Level::succ(a));
                        }
                        LNode::Max(_, _) => {
                            let b = Self::pop(&mut out, pos)?;
                            let a = Self::pop(&mut out, pos)?;
                            out.push(Level::max(a, b));
                        }
                        LNode::IMax(_, _) => {
                            let b = Self::pop(&mut out, pos)?;
                            let a = Self::pop(&mut out, pos)?;
                            out.push(Level::imax(a, b));
                        }
                        LNode::Zero | LNode::Param(_) => {
                            return Err(ExportError::internal(pos, "leaf level on exit stack"));
                        }
                    }
                }
            }
        }
        Self::pop(&mut out, pos)
    }

    /// Materialize the kernel [`Expr`] for an expression index.
    ///
    /// Structural sharing (wave5 Stage B): a file-global memo maps each export
    /// DAG id to the single `Rc<Expr>` it materializes to, so an id referenced
    /// by *N* parents (or reused across declarations) is built once and shared
    /// by refcount. Cost is therefore the number of *distinct* nodes newly
    /// constructed (memo misses), not the exploded tree size. The
    /// per-declaration and file budgets are charged one unit per newly-built
    /// node; an over-budget declaration aborts after at most `decl_budget`
    /// fresh nodes (the memo entries built before the breach remain valid and
    /// shareable) and `dispatch` recovers the specific error into an
    /// [`ExportDecl::Oversized`](crate::ExportDecl::Oversized) event.
    fn materialize_expr(&mut self, root: usize, pos: Position) -> ExportResult<Expr> {
        if self.memo.len() < self.enodes.len() {
            self.memo.resize(self.enodes.len(), None);
        }
        let node = self.build_expr(root, pos)?;
        // One shallow clone of the root (children are shared) so the caller
        // owns an `Expr`; the shared subtree stays interned in the memo.
        Ok((*node).clone())
    }

    /// Charge `n` newly-materialized nodes against the per-declaration and file
    /// budgets. An over-budget declaration aborts with [`DECL_BUDGET_FEATURE`]
    /// (recovered by `dispatch` into
    /// [`ExportDecl::Oversized`](crate::ExportDecl::Oversized)); the file budget
    /// is a hard [`BUDGET_FEATURE`] error. Charging `n` up front — for a Sort or
    /// Const, `n` includes the size of its (unmemoized) level subtree — is what
    /// keeps a sharing bomb from allocating: a `max[k,k]` doubling level has a
    /// saturated `lsize`, so it is rejected here before `build_level` expands it.
    fn charge_budget(&mut self, n: u64, pos: Position) -> ExportResult<()> {
        if self.decl_used.saturating_add(n) > self.decl_budget {
            return Err(ExportError::unsupported(pos, DECL_BUDGET_FEATURE));
        }
        if n > self.budget_left {
            return Err(ExportError::unsupported(pos, BUDGET_FEATURE));
        }
        self.budget_left -= n;
        self.decl_used = self.decl_used.saturating_add(n);
        if self.decl_used > self.stats.max_decl_materialized {
            self.stats.max_decl_materialized = self.decl_used;
        }
        self.stats.materialized_nodes = self.stats.materialized_nodes.saturating_add(n);
        Ok(())
    }

    /// Intern a freshly-built node in the file-global memo (so an id referenced
    /// again materializes to the same shared [`Node`]) and return the handle.
    fn intern(&mut self, i: usize, e: Expr) -> Node {
        let node = Node::new(e);
        if let Some(slot) = self.memo.get_mut(i) {
            *slot = Some(node.clone());
        }
        node
    }

    /// Iterative post-order expansion of an expression index into a *shared*
    /// kernel tree. A memo hit short-circuits an entire subtree in O(1); each
    /// memo miss constructs one node, charges it, and interns it. The export
    /// DAG is acyclic, so post-order fully completes a shared node's subtree
    /// before any sibling re-references it — the memo is therefore always
    /// populated by the time a second reference is entered.
    fn build_expr(&mut self, root: usize, pos: Position) -> ExportResult<Node> {
        enum Frame {
            Enter(usize),
            Exit(usize),
        }
        let mut stack = vec![Frame::Enter(root)];
        let mut out: Vec<Node> = Vec::new();
        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Enter(i) => {
                    if let Some(Some(node)) = self.memo.get(i) {
                        out.push(node.clone());
                        continue;
                    }
                    let node = self.enodes.get(i).ok_or_else(|| {
                        ExportError::internal(pos, format!("expr node {i} missing"))
                    })?;
                    match node {
                        ENode::BVar(n) => {
                            let n = *n;
                            self.charge_budget(1, pos)?;
                            out.push(self.intern(i, Expr::BVar(n)));
                        }
                        ENode::NatLit(n) => {
                            let n = n.clone();
                            self.charge_budget(1, pos)?;
                            out.push(self.intern(i, Expr::Lit(Literal::Nat(n))));
                        }
                        ENode::StrLit(s) => {
                            let s = s.clone();
                            self.charge_budget(1, pos)?;
                            out.push(self.intern(i, Expr::Lit(Literal::Str(s))));
                        }
                        ENode::Sort(l) => {
                            let l = *l;
                            // Charge the Sort node plus its level subtree BEFORE
                            // building it: an over-budget (bomb) level never
                            // reaches `build_level`. Levels are not memoized, so
                            // the whole subtree is (re)built and charged here.
                            self.charge_budget(1u64.saturating_add(self.lsize(l)), pos)?;
                            let lvl = self.build_level(l, pos)?;
                            out.push(self.intern(i, Expr::Sort(lvl)));
                        }
                        ENode::Const(name, us) => {
                            let name = name.clone();
                            let us = us.clone();
                            let mut cost = 1u64;
                            for u in &us {
                                cost = cost.saturating_add(self.lsize(*u));
                            }
                            self.charge_budget(cost, pos)?;
                            let mut levels = Vec::with_capacity(us.len());
                            for u in us {
                                levels.push(self.build_level(u, pos)?);
                            }
                            out.push(self.intern(i, Expr::Const(name, levels)));
                        }
                        ENode::App(f, a) => {
                            let (f, a) = (*f, *a);
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(a));
                            stack.push(Frame::Enter(f));
                        }
                        ENode::Lam(_, _, t, b) | ENode::Pi(_, _, t, b) => {
                            let (t, b) = (*t, *b);
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(b));
                            stack.push(Frame::Enter(t));
                        }
                        ENode::Let(_, t, v, b) => {
                            let (t, v, b) = (*t, *v, *b);
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(b));
                            stack.push(Frame::Enter(v));
                            stack.push(Frame::Enter(t));
                        }
                        ENode::Proj(_, _, s) => {
                            let s = *s;
                            stack.push(Frame::Exit(i));
                            stack.push(Frame::Enter(s));
                        }
                    }
                }
                Frame::Exit(i) => {
                    let e = {
                        let node = self.enodes.get(i).ok_or_else(|| {
                            ExportError::internal(pos, format!("expr node {i} missing"))
                        })?;
                        match node {
                            ENode::App(_, _) => {
                                let a = Self::pop(&mut out, pos)?;
                                let f = Self::pop(&mut out, pos)?;
                                Expr::App(f, a)
                            }
                            ENode::Lam(bi, n, _, _) => {
                                let (bi, n) = (*bi, n.clone());
                                let b = Self::pop(&mut out, pos)?;
                                let t = Self::pop(&mut out, pos)?;
                                Expr::Lam(bi, n, t, b)
                            }
                            ENode::Pi(bi, n, _, _) => {
                                let (bi, n) = (*bi, n.clone());
                                let b = Self::pop(&mut out, pos)?;
                                let t = Self::pop(&mut out, pos)?;
                                Expr::Pi(bi, n, t, b)
                            }
                            ENode::Let(n, _, _, _) => {
                                let n = n.clone();
                                let b = Self::pop(&mut out, pos)?;
                                let v = Self::pop(&mut out, pos)?;
                                let t = Self::pop(&mut out, pos)?;
                                Expr::Let(n, t, v, b)
                            }
                            ENode::Proj(n, field, _) => {
                                let (n, field) = (n.clone(), *field);
                                let s = Self::pop(&mut out, pos)?;
                                Expr::Proj(n, field, s)
                            }
                            _ => {
                                return Err(ExportError::internal(pos, "leaf expr on exit stack"));
                            }
                        }
                    };
                    self.charge_budget(1, pos)?;
                    out.push(self.intern(i, e));
                }
            }
        }
        Self::pop(&mut out, pos)
    }

    fn pop<T>(out: &mut Vec<T>, pos: Position) -> ExportResult<T> {
        out.pop()
            .ok_or_else(|| ExportError::internal(pos, "materializer value stack underflow"))
    }

    // --- Meta ------------------------------------------------------------

    fn parse_meta(&self, value: &JsonValue, line_no: usize) -> ExportResult<Meta> {
        let pos = Position::line_only(line_no);
        let root = Obj::new(value, pos)?;
        let meta = Obj::new(root.get("meta")?, pos).map_err(|_| {
            ExportError::malformed(pos, "first record must be a {\"meta\": ...} object")
        })?;
        let exporter = Obj::new(meta.get("exporter")?, pos)?;
        let lean = Obj::new(meta.get("lean")?, pos)?;
        let format = Obj::new(meta.get("format")?, pos)?;

        let m = Meta {
            exporter_name: exporter.str("name")?.to_string(),
            exporter_version: exporter.str("version")?.to_string(),
            lean_githash: lean.str("githash")?.to_string(),
            lean_version: lean.str("version")?.to_string(),
            format_version: format.str("version")?.to_string(),
        };

        match m.format_major() {
            Some(3) => Ok(m),
            Some(other) => Err(ExportError::unsupported(pos, unsupported_major(other))),
            None => Err(ExportError::malformed(
                pos,
                format!("unparseable format version {:?}", m.format_version),
            )),
        }
    }

    // --- Record dispatch -------------------------------------------------

    fn dispatch(&mut self, value: &JsonValue, line_no: usize) -> ExportResult<()> {
        let pos = Position::line_only(line_no);
        let obj = Obj::new(value, pos)?;
        let map = value
            .as_object()
            .ok_or_else(|| ExportError::malformed(pos, "record must be a JSON object"))?;

        // A record's discriminator is the single key that is not an index tag.
        // Primitive records have exactly two keys (payload + index tag); decl
        // records have exactly one key.
        let discriminator = map
            .keys()
            .find(|k| !matches!(k.as_str(), "in" | "il" | "ie"))
            .map(String::as_str)
            .ok_or_else(|| ExportError::malformed(pos, "record has no discriminator key"))?;

        match discriminator {
            // Name records
            "str" => self.rec_name_str(map, pos),
            "num" => self.rec_name_num(map, pos),
            // Level records
            "succ" => self.rec_level_succ(map, pos),
            "max" => self.rec_level_max(map, pos),
            "imax" => self.rec_level_imax(map, pos),
            "param" => self.rec_level_param(map, pos),
            // Expr records
            "bvar" => self.rec_expr_bvar(map, pos),
            "sort" => self.rec_expr_sort(map, pos),
            "const" => self.rec_expr_const(map, pos),
            "app" => self.rec_expr_app(map, pos),
            "lam" => self.rec_expr_binder(map, pos, true),
            "forallE" => self.rec_expr_binder(map, pos, false),
            "letE" => self.rec_expr_let(map, pos),
            "proj" => self.rec_expr_proj(map, pos),
            "natVal" => self.rec_expr_nat_lit(map, pos),
            "strVal" => self.rec_expr_str_lit(map, pos),
            "mdata" => self.rec_expr_mdata(map, pos),
            // Declaration records (per-declaration budget scope: C22)
            "axiom" | "def" | "thm" | "opaque" | "quot" | "inductive" => {
                self.decl_used = 0;
                let res = match discriminator {
                    "axiom" => self.rec_axiom(&obj, pos),
                    "def" => self.rec_def(&obj, pos),
                    "thm" => self.rec_thm(&obj, pos),
                    "opaque" => self.rec_opaque(&obj, pos),
                    "quot" => self.rec_quot(&obj, pos),
                    _ => self.rec_inductive(&obj, pos),
                };
                match res {
                    // C22 recovery: the declaration's kernel trees exceed the
                    // per-declaration materialization budget. Surface it as a
                    // NAMED oversized declaration (its names are read from
                    // the interned name table without materializing any
                    // expression) instead of aborting the read. `pending` can
                    // only hold members of THIS record here — read_streaming
                    // drains it after every dispatch — so clearing it drops
                    // exactly the partially-read group.
                    Err(ExportError::Unsupported { feature, .. })
                        if feature == DECL_BUDGET_FEATURE =>
                    {
                        self.pending.clear();
                        let names = self.decl_record_names(discriminator, &obj, pos)?;
                        self.pending.push(ExportDecl::Oversized(OversizedVal {
                            names,
                            feature: DECL_BUDGET_FEATURE,
                        }));
                        self.stats.decl_oversized += 1;
                        Ok(())
                    }
                    other => other,
                }
            }
            other => Err(ExportError::malformed(
                pos,
                format!("unknown record discriminator {other:?}"),
            )),
        }
    }

    /// Every name a declaration record would introduce, read from the name
    /// table only — no expression is materialized. Used by the C22 oversized
    /// recovery in `dispatch`.
    fn decl_record_names(
        &mut self,
        discriminator: &str,
        obj: &Obj,
        pos: Position,
    ) -> ExportResult<Vec<Name>> {
        let mut names = Vec::new();
        match discriminator {
            "axiom" | "quot" => {
                let o = Obj::new(obj.get(discriminator)?, pos)?;
                names.push(self.name_at(o.index("name")?, pos)?);
            }
            "def" | "thm" | "opaque" => {
                for m in Self::decl_members(obj, discriminator, pos)? {
                    names.push(self.name_at(m.index("name")?, pos)?);
                }
            }
            "inductive" => {
                let o = Obj::new(obj.get("inductive")?, pos)?;
                let (types_key, ctors_key, recs_key) = if o.get_opt("types").is_some() {
                    ("types", "ctors", "recs")
                } else {
                    ("inductiveVals", "constructorVals", "recursorVals")
                };
                for key in [types_key, ctors_key, recs_key] {
                    for t in o.array(key)? {
                        let to = Obj::new(t, pos)?;
                        names.push(self.name_at(to.index("name")?, pos)?);
                    }
                }
            }
            other => {
                return Err(ExportError::internal(
                    pos,
                    format!("decl_record_names on non-declaration record {other:?}"),
                ));
            }
        }
        Ok(names)
    }

    fn index_tag(
        map: &std::collections::BTreeMap<String, JsonValue>,
        tag: &str,
        pos: Position,
    ) -> ExportResult<usize> {
        let v = map
            .get(tag)
            .ok_or_else(|| ExportError::malformed(pos, format!("missing index tag {tag:?}")))?;
        parse_index(v, pos, tag)
    }

    // --- Name records ----------------------------------------------------

    fn rec_name_str(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "in", pos)?;
        let body = Obj::new(
            map.get("str")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'str'"))?,
            pos,
        )?;
        let pre = self.name_at(body.index("pre")?, pos)?;
        let s = body.str("str")?.to_string();
        self.put_name(idx, pre.append_str(s), pos)?;
        self.stats.name_str += 1;
        Ok(())
    }

    fn rec_name_num(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "in", pos)?;
        let body = Obj::new(
            map.get("num")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'num'"))?,
            pos,
        )?;
        let pre = self.name_at(body.index("pre")?, pos)?;
        let n = body.u64("i")?;
        self.put_name(idx, pre.append_num(n), pos)?;
        self.stats.name_num += 1;
        Ok(())
    }

    // --- Level records ---------------------------------------------------

    fn rec_level_succ(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "il", pos)?;
        let inner = parse_index(
            map.get("succ")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'succ'"))?,
            pos,
            "succ",
        )?;
        let a = self.level_ref(inner, pos)?;
        self.put_level(idx, LNode::Succ(a), pos)?;
        self.stats.level_succ += 1;
        Ok(())
    }

    fn level_pair(
        &self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
        key: &str,
    ) -> ExportResult<(usize, usize, usize)> {
        let idx = Self::index_tag(map, "il", pos)?;
        let arr = map
            .get(key)
            .and_then(JsonValue::as_array)
            .ok_or_else(|| ExportError::malformed(pos, format!("'{key}' must be a 2-array")))?;
        if arr.len() != 2 {
            return Err(ExportError::malformed(
                pos,
                format!("'{key}' must have exactly 2 elements"),
            ));
        }
        let a = self.level_ref(parse_index(&arr[0], pos, key)?, pos)?;
        let b = self.level_ref(parse_index(&arr[1], pos, key)?, pos)?;
        Ok((idx, a, b))
    }

    fn rec_level_max(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let (idx, a, b) = self.level_pair(map, pos, "max")?;
        self.put_level(idx, LNode::Max(a, b), pos)?;
        self.stats.level_max += 1;
        Ok(())
    }

    fn rec_level_imax(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let (idx, a, b) = self.level_pair(map, pos, "imax")?;
        self.put_level(idx, LNode::IMax(a, b), pos)?;
        self.stats.level_imax += 1;
        Ok(())
    }

    fn rec_level_param(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "il", pos)?;
        let name_idx = parse_index(
            map.get("param")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'param'"))?,
            pos,
            "param",
        )?;
        let name = self.name_at(name_idx, pos)?;
        self.put_level(idx, LNode::Param(name), pos)?;
        self.stats.level_param += 1;
        Ok(())
    }

    // --- Expr records ----------------------------------------------------

    fn rec_expr_bvar(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let n = map
            .get("bvar")
            .and_then(JsonValue::as_int_str)
            .ok_or_else(|| ExportError::malformed(pos, "'bvar' must be an integer"))?;
        let n = n.parse::<u32>().map_err(|_| {
            ExportError::malformed(pos, format!("bvar de Bruijn index out of u32 range: {n}"))
        })?;
        self.put_expr(idx, ENode::BVar(n), pos)?;
        self.stats.expr_bvar += 1;
        Ok(())
    }

    fn rec_expr_sort(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let lvl = parse_index(
            map.get("sort")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'sort'"))?,
            pos,
            "sort",
        )?;
        let l = self.level_ref(lvl, pos)?;
        self.put_expr(idx, ENode::Sort(l), pos)?;
        self.stats.expr_sort += 1;
        Ok(())
    }

    fn rec_expr_const(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let body = Obj::new(
            map.get("const")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'const'"))?,
            pos,
        )?;
        let name = self.name_at(body.index("name")?, pos)?;
        let mut us = Vec::new();
        for u in body.index_array("us")? {
            us.push(self.level_ref(u, pos)?);
        }
        self.put_expr(idx, ENode::Const(name, us), pos)?;
        self.stats.expr_const += 1;
        Ok(())
    }

    fn rec_expr_app(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let body = Obj::new(
            map.get("app")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'app'"))?,
            pos,
        )?;
        let f = self.expr_ref(body.index("fn")?, pos)?;
        let a = self.expr_ref(body.index("arg")?, pos)?;
        self.put_expr(idx, ENode::App(f, a), pos)?;
        self.stats.expr_app += 1;
        Ok(())
    }

    fn rec_expr_binder(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
        is_lam: bool,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let key = if is_lam { "lam" } else { "forallE" };
        let body = Obj::new(
            map.get(key)
                .ok_or_else(|| ExportError::malformed(pos, "missing binder body"))?,
            pos,
        )?;
        let name = self.name_at(body.index("name")?, pos)?;
        let ty = self.expr_ref(body.index("type")?, pos)?;
        let bd = self.expr_ref(body.index("body")?, pos)?;
        let bi = parse_binder_info(body.str("binderInfo")?, pos)?;
        let node = if is_lam {
            ENode::Lam(bi, name, ty, bd)
        } else {
            ENode::Pi(bi, name, ty, bd)
        };
        self.put_expr(idx, node, pos)?;
        if is_lam {
            self.stats.expr_lam += 1;
        } else {
            self.stats.expr_forall += 1;
        }
        Ok(())
    }

    fn rec_expr_let(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let body = Obj::new(
            map.get("letE")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'letE'"))?,
            pos,
        )?;
        let name = self.name_at(body.index("name")?, pos)?;
        let ty = self.expr_ref(body.index("type")?, pos)?;
        let val = self.expr_ref(body.index("value")?, pos)?;
        let bd = self.expr_ref(body.index("body")?, pos)?;
        // `nondep` is a serializer hint; the kernel Let carries no such flag.
        self.put_expr(idx, ENode::Let(name, ty, val, bd), pos)?;
        self.stats.expr_let += 1;
        Ok(())
    }

    fn rec_expr_proj(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let body = Obj::new(
            map.get("proj")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'proj'"))?,
            pos,
        )?;
        let type_name = self.name_at(body.index("typeName")?, pos)?;
        let field = body.u32("idx")?;
        let structure = self.expr_ref(body.index("struct")?, pos)?;
        self.put_expr(idx, ENode::Proj(type_name, field, structure), pos)?;
        self.stats.expr_proj += 1;
        Ok(())
    }

    fn rec_expr_nat_lit(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        // Per spec: natVal is a DECIMAL STRING (arbitrary precision). Route it
        // through BigNat::from_decimal_str; never through u64/f64.
        let s = map
            .get("natVal")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| ExportError::malformed(pos, "'natVal' must be a decimal string"))?;
        let n = BigNat::from_decimal_str(s)
            .ok_or_else(|| ExportError::malformed(pos, format!("invalid decimal natVal {s:?}")))?;
        self.put_expr(idx, ENode::NatLit(n), pos)?;
        self.stats.expr_nat_lit += 1;
        Ok(())
    }

    fn rec_expr_str_lit(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let s = map
            .get("strVal")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| ExportError::malformed(pos, "'strVal' must be a string"))?;
        self.put_expr(idx, ENode::StrLit(s.to_string()), pos)?;
        self.stats.expr_str_lit += 1;
        Ok(())
    }

    fn rec_expr_mdata(
        &mut self,
        map: &std::collections::BTreeMap<String, JsonValue>,
        pos: Position,
    ) -> ExportResult<()> {
        let idx = Self::index_tag(map, "ie", pos)?;
        let body = Obj::new(
            map.get("mdata")
                .ok_or_else(|| ExportError::malformed(pos, "missing 'mdata'"))?,
            pos,
        )?;
        // The metadata payload is discarded (as the reference parser does); the
        // annotated expression is surfaced transparently by aliasing its node.
        let inner = self.expr_ref(body.index("expr")?, pos)?;
        let node = self
            .enodes
            .get(inner)
            .cloned()
            .ok_or_else(|| ExportError::internal(pos, "mdata inner node missing"))?;
        self.put_expr(idx, node, pos)?;
        self.stats.expr_mdata += 1;
        Ok(())
    }

    // --- Declaration records ---------------------------------------------

    /// Read the common `name`/`levelParams`/`type` triple.
    fn read_common(&mut self, o: &Obj, pos: Position) -> ExportResult<ConstantVal> {
        let name = self.name_at(o.index("name")?, pos)?;
        let mut level_params = Vec::new();
        for np in o.index_array("levelParams")? {
            level_params.push(self.name_at(np, pos)?);
        }
        let ty_idx = self.expr_ref(o.index("type")?, pos)?;
        let ty = self.materialize_expr(ty_idx, pos)?;
        Ok(ConstantVal {
            name,
            level_params,
            ty,
        })
    }

    fn read_all(&self, o: &Obj, pos: Position) -> ExportResult<Vec<Name>> {
        let mut all = Vec::new();
        for n in o.index_array("all")? {
            all.push(self.name_at(n, pos)?);
        }
        Ok(all)
    }

    fn read_value(&mut self, o: &Obj, pos: Position) -> ExportResult<Expr> {
        let idx = self.expr_ref(o.index("value")?, pos)?;
        self.materialize_expr(idx, pos)
    }

    fn rec_axiom(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let o = Obj::new(obj.get("axiom")?, pos)?;
        let common = self.read_common(&o, pos)?;
        let is_unsafe = o.bool("isUnsafe")?;
        self.pending
            .push(ExportDecl::Axiom(AxiomVal { common, is_unsafe }));
        self.stats.decl_axiom += 1;
        Ok(())
    }

    /// `def`/`thm`/`opaque` payloads are either a single object (v3.1.0) or a
    /// single/multi element array (v3.0.0 mutual group). This returns each
    /// member object with its position.
    fn decl_members<'a>(obj: &Obj<'a>, key: &str, pos: Position) -> ExportResult<Vec<Obj<'a>>> {
        let payload = obj.get(key)?;
        match payload {
            JsonValue::Array(items) => items.iter().map(|v| Obj::new(v, pos)).collect(),
            JsonValue::Object(_) => Ok(vec![Obj::new(payload, pos)?]),
            _ => Err(ExportError::malformed(
                pos,
                format!("'{key}' must be an object or array of objects"),
            )),
        }
    }

    fn read_def_member(&mut self, o: &Obj, pos: Position) -> ExportResult<DefinitionVal> {
        let common = self.read_common(o, pos)?;
        let value = self.read_value(o, pos)?;
        let hints = parse_hints(o.get("hints")?, pos)?;
        let safety = parse_safety(o.str("safety")?, pos)?;
        let all = self.read_all(o, pos)?;
        Ok(DefinitionVal {
            common,
            value,
            hints,
            safety,
            all,
        })
    }

    fn rec_def(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let members = Self::decl_members(obj, "def", pos)?;
        for m in &members {
            let dv = self.read_def_member(m, pos)?;
            self.pending.push(ExportDecl::Definition(dv));
            self.stats.decl_def += 1;
        }
        Ok(())
    }

    fn read_thm_member(&mut self, o: &Obj, pos: Position) -> ExportResult<TheoremVal> {
        let common = self.read_common(o, pos)?;
        let value = self.read_value(o, pos)?;
        let all = self.read_all(o, pos)?;
        Ok(TheoremVal { common, value, all })
    }

    fn rec_thm(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let members = Self::decl_members(obj, "thm", pos)?;
        for m in &members {
            let tv = self.read_thm_member(m, pos)?;
            self.pending.push(ExportDecl::Theorem(tv));
            self.stats.decl_thm += 1;
        }
        Ok(())
    }

    fn read_opaque_member(&mut self, o: &Obj, pos: Position) -> ExportResult<OpaqueVal> {
        let common = self.read_common(o, pos)?;
        let value = self.read_value(o, pos)?;
        let is_unsafe = o.bool("isUnsafe")?;
        let all = self.read_all(o, pos)?;
        Ok(OpaqueVal {
            common,
            value,
            is_unsafe,
            all,
        })
    }

    fn rec_opaque(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let members = Self::decl_members(obj, "opaque", pos)?;
        for m in &members {
            let ov = self.read_opaque_member(m, pos)?;
            self.pending.push(ExportDecl::Opaque(ov));
            self.stats.decl_opaque += 1;
        }
        Ok(())
    }

    fn rec_quot(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let o = Obj::new(obj.get("quot")?, pos)?;
        let common = self.read_common(&o, pos)?;
        let kind = parse_quot_kind(o.str("kind")?, pos)?;
        self.pending
            .push(ExportDecl::Quot(QuotVal { common, kind }));
        self.stats.decl_quot += 1;
        Ok(())
    }

    fn rec_inductive(&mut self, obj: &Obj, pos: Position) -> ExportResult<()> {
        let o = Obj::new(obj.get("inductive")?, pos)?;

        // v3.1.0 keys: types/ctors/recs. v3.0.0 keys:
        // inductiveVals/constructorVals/recursorVals. Accept both.
        let (types_key, ctors_key, recs_key) = if o.get_opt("types").is_some() {
            ("types", "ctors", "recs")
        } else {
            ("inductiveVals", "constructorVals", "recursorVals")
        };

        let mut types = Vec::new();
        for t in o.array(types_key)? {
            let to = Obj::new(t, pos)?;
            types.push(self.read_inductive_val(&to, pos)?);
        }
        let mut ctors = Vec::new();
        for c in o.array(ctors_key)? {
            let co = Obj::new(c, pos)?;
            ctors.push(self.read_constructor_val(&co, pos)?);
        }
        let mut recs = Vec::new();
        for r in o.array(recs_key)? {
            let ro = Obj::new(r, pos)?;
            recs.push(self.read_recursor_val(&ro, pos)?);
        }

        self.pending.push(ExportDecl::Inductive(InductiveBundle {
            types,
            ctors,
            recs,
        }));
        self.stats.decl_inductive += 1;
        Ok(())
    }

    fn read_inductive_val(&mut self, o: &Obj, pos: Position) -> ExportResult<InductiveVal> {
        let common = self.read_common(o, pos)?;
        let num_params = o.u32("numParams")?;
        let num_indices = o.u32("numIndices")?;
        let all = self.read_all(o, pos)?;
        let mut ctors = Vec::new();
        for c in o.index_array("ctors")? {
            ctors.push(self.name_at(c, pos)?);
        }
        Ok(InductiveVal {
            common,
            num_params,
            num_indices,
            all,
            ctors,
            num_nested: o.u32("numNested")?,
            is_rec: o.bool("isRec")?,
            is_unsafe: o.bool("isUnsafe")?,
            is_reflexive: o.bool("isReflexive")?,
            // `isProp` is not present in the export format; the kernel derives
            // it during checking, so a conservative `false` is correct here.
            is_prop: o
                .get_opt("isProp")
                .and_then(JsonValue::as_bool)
                .unwrap_or(false),
        })
    }

    fn read_constructor_val(&mut self, o: &Obj, pos: Position) -> ExportResult<ConstructorVal> {
        let common = self.read_common(o, pos)?;
        let induct = self.name_at(o.index("induct")?, pos)?;
        Ok(ConstructorVal {
            common,
            induct,
            cidx: o.u32("cidx")?,
            num_params: o.u32("numParams")?,
            num_fields: o.u32("numFields")?,
            is_unsafe: o.bool("isUnsafe")?,
        })
    }

    fn read_recursor_val(&mut self, o: &Obj, pos: Position) -> ExportResult<RecursorVal> {
        let common = self.read_common(o, pos)?;
        let all = self.read_all(o, pos)?;
        let mut rules = Vec::new();
        for r in o.array("rules")? {
            let ro = Obj::new(r, pos)?;
            let ctor = self.name_at(ro.index("ctor")?, pos)?;
            let nfields = ro.u32("nfields")?;
            let rhs_idx = self.expr_ref(ro.index("rhs")?, pos)?;
            let rhs = self.materialize_expr(rhs_idx, pos)?;
            rules.push(RecursorRule { ctor, nfields, rhs });
        }
        Ok(RecursorVal {
            common,
            all,
            num_params: o.u32("numParams")?,
            num_indices: o.u32("numIndices")?,
            num_motives: o.u32("numMotives")?,
            num_minors: o.u32("numMinors")?,
            rules,
            k: o.bool("k")?,
            is_unsafe: o.bool("isUnsafe")?,
        })
    }
}

/// A named-feature string for an unsupported major format version.
fn unsupported_major(major: u32) -> &'static str {
    match major {
        0..=2 => "legacy text-format lean4export (pre-3.x)",
        _ => "future NDJSON format major version (>3)",
    }
}

/// Parse a `binderInfo` string.
fn parse_binder_info(s: &str, pos: Position) -> ExportResult<BinderInfo> {
    match s {
        "default" => Ok(BinderInfo::Default),
        "implicit" => Ok(BinderInfo::Implicit),
        "strictImplicit" => Ok(BinderInfo::StrictImplicit),
        "instImplicit" => Ok(BinderInfo::InstImplicit),
        other => Err(ExportError::malformed(
            pos,
            format!("unknown binderInfo {other:?}"),
        )),
    }
}

/// Parse the `hints` field of a definition (string `"opaque"`/`"abbrev"` or
/// `{"regular": <height>}`).
fn parse_hints(v: &JsonValue, pos: Position) -> ExportResult<ReducibilityHint> {
    match v {
        JsonValue::Str(s) if s == "opaque" => Ok(ReducibilityHint::Opaque),
        JsonValue::Str(s) if s == "abbrev" => Ok(ReducibilityHint::Abbrev),
        JsonValue::Str(other) => Err(ExportError::malformed(
            pos,
            format!("unknown hints string {other:?}"),
        )),
        JsonValue::Object(m) => {
            let height = m
                .get("regular")
                .ok_or_else(|| ExportError::malformed(pos, "hints object must have 'regular'"))?;
            let h = parse_index(height, pos, "regular")?;
            let h = u32::try_from(h).map_err(|_| {
                ExportError::malformed(pos, format!("regular hint height out of u32 range: {h}"))
            })?;
            Ok(ReducibilityHint::Regular(h))
        }
        _ => Err(ExportError::malformed(
            pos,
            "hints must be a string or object",
        )),
    }
}

/// Parse a `safety` string.
fn parse_safety(s: &str, pos: Position) -> ExportResult<DefinitionSafety> {
    match s {
        "safe" => Ok(DefinitionSafety::Safe),
        "unsafe" => Ok(DefinitionSafety::Unsafe),
        "partial" => Ok(DefinitionSafety::Partial),
        other => Err(ExportError::malformed(
            pos,
            format!("unknown safety {other:?}"),
        )),
    }
}

/// Parse a quotient `kind` string.
fn parse_quot_kind(s: &str, pos: Position) -> ExportResult<QuotKind> {
    match s {
        "type" => Ok(QuotKind::Type),
        "ctor" => Ok(QuotKind::Mk),
        "lift" => Ok(QuotKind::Lift),
        "ind" => Ok(QuotKind::Ind),
        other => Err(ExportError::malformed(
            pos,
            format!("unknown quot kind {other:?}"),
        )),
    }
}
