//! Replay: drive an `oxilean_kernel` [`Environment`] from parsed declarations.
//!
//! Replay is a **true checker**: the environment starts *empty* and every
//! constant must be introduced by the export file itself (lean4export emits
//! the full transitive closure of a declaration, so a valid export is
//! self-contained). Nothing is pre-seeded and nothing is trusted:
//!
//! * Axioms, definitions, theorems and opaques go through the kernel's
//!   `check_constant_info` (type-checks and adds to the env).
//! * `inductive` bundles are checked as a family
//!   (`check_and_derive_family`: telescopes, universes, WHNF-hardened strict
//!   positivity) and installed with the exported declarations, each
//!   re-verified by the kernel; exported **recursors are never trusted** —
//!   the kernel re-derives them and requires a definitional match.
//! * `quot` records install the four `#QUOT` primitives via the kernel's
//!   once-only `Environment::add_quot` (kernel-constructed canonical types);
//!   each exported record is then validated against the installed primitive.
//! * The `Quot.sound` **axiom** (a plain axiom record in exports) is validated
//!   against the kernel's canonical soundness type — an export cannot smuggle
//!   an arbitrary assertion under that trusted name.
//!
//! ## Three buckets
//!
//! Every declaration lands in exactly one bucket ([`ReplayOutcome`]):
//!
//! * `Checked` — the kernel accepted it.
//! * `Unsupported { feature }` — valid Lean the kernel cannot replay *yet*,
//!   with a named feature (currently: [`NESTED_INDUCTIVES`]; plus
//!   [`DEFERRED_DEPENDENCY`] for declarations that only fail because they
//!   depend on an unsupported one).
//! * `Rejected { reason }` — the kernel judged it ill-typed (or it depends on
//!   a rejected declaration; the reason names the root cause).
//!
//! ## Corpus-scale replay
//!
//! [`replay_streaming`] replays straight from a reader without retaining
//! declarations, under explicit [`ReplayLimits`] (reader node budget + an
//! optional advisory per-declaration time budget). See
//! [`crate::Limits::corpus`] for the documented whole-corpus preset.

use oxilean_kernel::wall_clock::Instant;
use std::collections::HashSet;
use std::io::BufRead;
use std::time::Duration;

use oxilean_kernel::env::{
    canonical_quot_level_params, canonical_quot_sound_level_params, canonical_quot_sound_type,
    canonical_quot_type, quot_sound_name,
};
use oxilean_kernel::instantiate::instantiate_type_lparams;
use oxilean_kernel::{
    check_constant_info, AxiomVal, ConstantInfo, Environment, InductiveSpec, KernelError, Level,
    Name, QuotVal, TypeChecker,
};

use crate::error::ExportResult;
use crate::model::{ExportDecl, InductiveBundle, Meta};
use crate::reader::{read_streaming, ExportFile, Limits, ReadStats};

/// The outcome of replaying one declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayOutcome {
    /// The kernel accepted (type-checked and added) the declaration.
    Checked,
    /// The declaration is valid but this kernel version cannot replay it yet.
    /// Carries the named feature that is missing.
    Unsupported {
        /// A stable, named feature string.
        feature: &'static str,
    },
    /// The kernel rejected the declaration. Carries a human-readable reason.
    Rejected {
        /// Rendered kernel error.
        reason: String,
    },
}

impl ReplayOutcome {
    /// `true` if the kernel accepted the declaration.
    #[must_use]
    pub fn is_checked(&self) -> bool {
        matches!(self, ReplayOutcome::Checked)
    }

    /// `true` if the declaration was deferred as unsupported.
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        matches!(self, ReplayOutcome::Unsupported { .. })
    }

    /// `true` if the kernel rejected the declaration.
    #[must_use]
    pub fn is_rejected(&self) -> bool {
        matches!(self, ReplayOutcome::Rejected { .. })
    }
}

/// A per-declaration replay result.
#[derive(Debug, Clone)]
pub struct ReplayEntry {
    /// The declaration's primary name.
    pub name: Name,
    /// The declaration kind label (`"axiom"`, `"def"`, ...).
    pub kind: &'static str,
    /// The outcome.
    pub outcome: ReplayOutcome,
}

/// The full replay report: one entry per declaration, in file order, plus a
/// three-bucket summary.
#[derive(Debug, Clone, Default)]
pub struct ReplayReport {
    /// Per-declaration entries.
    pub entries: Vec<ReplayEntry>,
}

impl ReplayReport {
    /// Number of declarations the kernel accepted.
    #[must_use]
    pub fn checked(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome.is_checked())
            .count()
    }

    /// Number of declarations deferred as unsupported.
    #[must_use]
    pub fn unsupported(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome.is_unsupported())
            .count()
    }

    /// Number of declarations the kernel rejected.
    #[must_use]
    pub fn rejected(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome.is_rejected())
            .count()
    }
}

/// The named feature reported when a declaration is deferred only because it
/// depends on another *unsupported* declaration.
pub const DEFERRED_DEPENDENCY: &str = "dependency on an unsupported declaration";

/// The named feature for nested inductive types (an inductive occurring under
/// a non-family type constructor in a constructor field). The kernel does not
/// implement Lean's nested-to-mutual elaboration yet and reports
/// `KernelError::UnsupportedNestedInductive`; replay surfaces that as this
/// named unsupported feature (three-bucket rule: valid Lean, not replayable
/// yet, never a rejection).
pub const NESTED_INDUCTIVES: &str = "nested inductives";

/// The named feature reported when a declaration exceeded the deterministic
/// per-declaration resource budget ([`Replayer::set_per_decl_fuel`], measured
/// in kernel `Expr` nodes cloned — see `oxilean_kernel::fuel`).
///
/// This is a **named unsupported** verdict, never a rejection: the kernel
/// degrades conservatively when fuel runs out (stuck reduction, syntactic
/// def-eq only), so the failure means "this build refused to spend more
/// resources", not "this is not a proof". Deterministic: the same
/// declaration with the same budget always lands in the same bucket.
pub const RESOURCE_LIMIT: &str = "resource limit exceeded: per-declaration clone-fuel budget";

/// A stateful replayer: an initially **empty** kernel [`Environment`] plus the
/// bookkeeping needed for honest three-bucket reporting:
///
/// * `deferred` — names whose declarations were *unsupported* (e.g. nested
///   inductives). A later declaration failing only with
///   `KernelError::UnknownConstant` on a deferred name is reported as
///   `Unsupported` ([`DEFERRED_DEPENDENCY`]) rather than as a rejection, and
///   its own name joins the set (the cascade).
/// * `rejected` — names whose declarations the kernel *rejected*. A later
///   declaration failing only with `UnknownConstant` on a rejected name is
///   reported as `Rejected` with a reason naming the root cause.
///
/// This keeps `Rejected` meaning exactly "the kernel could not verify this"
/// and `Unsupported` meaning "valid Lean this build cannot replay yet".
pub struct Replayer {
    env: Environment,
    deferred: HashSet<Name>,
    rejected: HashSet<Name>,
    /// Deterministic per-declaration resource budget, in kernel `Expr` nodes
    /// cloned (`None` = unlimited). See [`RESOURCE_LIMIT`].
    per_decl_fuel: Option<u64>,
    /// Wall-clock per-declaration deadline (`None` = none). Backstop for
    /// reduction/def-eq loops that make no metered progress; an expired
    /// declaration is degraded like a fuel-exhausted one and reported as
    /// [`RESOURCE_LIMIT`]. Machine-dependent, so it never changes a verdict —
    /// only bounds an otherwise-unbounded declaration.
    per_decl_time_budget: Option<std::time::Duration>,
}

impl Replayer {
    /// Create a replayer over a fresh, **empty** environment.
    ///
    /// The environment is deliberately not seeded with any builtin constants:
    /// a lean4export file re-declares everything it uses (including `Nat`,
    /// `Eq`, ...), so pre-seeded constants would collide with — and worse,
    /// would have to be *trusted instead of checked* against — the export's
    /// own declarations.
    ///
    /// # Errors
    /// Currently infallible; the `Result` is kept for API stability.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            env: Environment::new(),
            deferred: HashSet::new(),
            rejected: HashSet::new(),
            per_decl_fuel: None,
            per_decl_time_budget: None,
        })
    }

    /// Set the deterministic per-declaration resource budget, in kernel
    /// `Expr` nodes cloned (`None` = unlimited, the default).
    ///
    /// When a declaration exhausts the budget the kernel degrades
    /// conservatively (stuck reduction, syntactic def-eq only — never a
    /// wrong accept) and the declaration is reported as *unsupported* with
    /// the named feature [`RESOURCE_LIMIT`], not as rejected. One
    /// declaration can therefore never OOM the process.
    pub fn set_per_decl_fuel(&mut self, fuel: Option<u64>) {
        self.per_decl_fuel = fuel;
    }

    /// Set the wall-clock per-declaration deadline (`None` = none, the default).
    ///
    /// This is a non-deterministic backstop for declarations whose reduction or
    /// def-eq loops make progress without constructing nodes (so the
    /// deterministic fuel never catches them). An over-time declaration is
    /// degraded exactly like a fuel-exhausted one — a stuck reduction and
    /// syntactic-only def-eq, never a wrong accept — and reported as
    /// [`RESOURCE_LIMIT`]. Because it is wall-clock based, WHICH declarations
    /// hit it is machine-dependent; it never changes a positive/negative
    /// verdict.
    pub fn set_per_decl_time_budget(&mut self, budget: Option<std::time::Duration>) {
        self.per_decl_time_budget = budget;
    }

    /// Borrow the underlying environment (e.g. to inspect checked constants).
    #[must_use]
    pub fn env(&self) -> &Environment {
        &self.env
    }

    /// Replay one declaration.
    pub fn replay(&mut self, decl: &ExportDecl) -> ReplayEntry {
        oxilean_kernel::fuel::set_budget(self.per_decl_fuel);
        oxilean_kernel::deadline::set_deadline(self.per_decl_time_budget);
        let name = decl.primary_name();
        let kind = decl.kind_label();
        let outcome = match decl {
            ExportDecl::Axiom(v) => self.check(ConstantInfo::Axiom(v.clone()), &name),
            ExportDecl::Definition(v) => self.check(ConstantInfo::Definition(v.clone()), &name),
            ExportDecl::Theorem(v) => self.check(ConstantInfo::Theorem(v.clone()), &name),
            ExportDecl::Opaque(v) => self.check(ConstantInfo::Opaque(v.clone()), &name),
            ExportDecl::Quot(v) => {
                let res = replay_quot_core(&mut self.env, v);
                self.finish_group(std::slice::from_ref(&v.common.name), res)
            }
            ExportDecl::Inductive(bundle) => {
                let names: Vec<Name> = bundle_names(bundle);
                let res = replay_inductive_core(&mut self.env, bundle);
                self.finish_group(&names, res)
            }
            // C22: the reader refused to materialize this declaration (its
            // kernel trees exceed the per-declaration budget). Defer every
            // name it would have introduced so dependents cascade as
            // DEFERRED_DEPENDENCY — exactly like any other named-unsupported
            // declaration, and never a rejection.
            ExportDecl::Oversized(v) => {
                self.defer_all(&v.names);
                ReplayOutcome::Unsupported { feature: v.feature }
            }
        };
        ReplayEntry {
            name,
            kind,
            outcome,
        }
    }

    /// Run `check_constant_info` and map its result to a [`ReplayOutcome`],
    /// cascading deferrals/rejections for unknown constants whose declarations
    /// were themselves unsupported/rejected.
    fn check(&mut self, ci: ConstantInfo, own_name: &Name) -> ReplayOutcome {
        if let ConstantInfo::Axiom(av) = &ci {
            if let Some(reason) = validate_quot_sound_axiom(&self.env, av) {
                self.rejected.insert(own_name.clone());
                return ReplayOutcome::Rejected { reason };
            }
        }
        match check_constant_info(&mut self.env, ci) {
            Ok(()) => ReplayOutcome::Checked,
            Err(e) => self.map_failure(std::slice::from_ref(own_name), SeamFailure::Kernel(e)),
        }
    }

    /// Map a group failure (quot / inductive bundle) onto an outcome, updating
    /// the cascade sets for every name the group would have introduced.
    fn finish_group(&mut self, names: &[Name], res: Result<(), SeamFailure>) -> ReplayOutcome {
        match res {
            Ok(()) => ReplayOutcome::Checked,
            Err(failure) => self.map_failure(names, failure),
        }
    }

    fn map_failure(&mut self, own_names: &[Name], failure: SeamFailure) -> ReplayOutcome {
        match failure {
            SeamFailure::Unsupported(feature) => {
                self.defer_all(own_names);
                ReplayOutcome::Unsupported { feature }
            }
            SeamFailure::Kernel(KernelError::UnknownConstant(n)) if self.deferred.contains(&n) => {
                self.defer_all(own_names);
                ReplayOutcome::Unsupported {
                    feature: DEFERRED_DEPENDENCY,
                }
            }
            SeamFailure::Kernel(KernelError::UnknownConstant(n)) if self.rejected.contains(&n) => {
                self.reject_all(own_names);
                ReplayOutcome::Rejected {
                    reason: format!("depends on rejected declaration '{n}'"),
                }
            }
            // A failure with the resource budget exhausted is a NAMED
            // unsupported verdict, never a rejection: the kernel degraded
            // conservatively once fuel ran out (stuck reduction, syntactic
            // def-eq only), so the error only means "this build refused to
            // spend more resources on this declaration". Genuine kernel
            // verdicts reached *before* exhaustion are unaffected (the flag
            // only latches when the budget is actually crossed), and the
            // known-cause cascades above take precedence.
            SeamFailure::Kernel(_) | SeamFailure::Reason(_)
                if oxilean_kernel::fuel::is_exhausted() =>
            {
                self.defer_all(own_names);
                ReplayOutcome::Unsupported {
                    feature: RESOURCE_LIMIT,
                }
            }
            SeamFailure::Kernel(e) => {
                self.reject_all(own_names);
                ReplayOutcome::Rejected {
                    reason: render_kernel_error(&e),
                }
            }
            SeamFailure::Reason(reason) => {
                self.reject_all(own_names);
                ReplayOutcome::Rejected { reason }
            }
        }
    }

    fn defer_all(&mut self, names: &[Name]) {
        for n in names {
            self.deferred.insert(n.clone());
        }
    }

    fn reject_all(&mut self, names: &[Name]) {
        for n in names {
            self.rejected.insert(n.clone());
        }
    }
}

/// All names an inductive bundle introduces (types, constructors, recursors).
fn bundle_names(bundle: &InductiveBundle) -> Vec<Name> {
    bundle
        .types
        .iter()
        .map(|t| t.common.name.clone())
        .chain(bundle.ctors.iter().map(|c| c.common.name.clone()))
        .chain(bundle.recs.iter().map(|r| r.common.name.clone()))
        .collect()
}

/// Replay a whole [`ExportFile`] into a fresh, empty environment, returning a
/// per-declaration [`ReplayReport`].
///
/// If creating the replayer fails the error is surfaced as a `Rejected`
/// outcome on a synthetic first entry so the caller still gets a report,
/// never a panic.
#[must_use]
pub fn replay_file(file: &ExportFile) -> ReplayReport {
    let mut report = ReplayReport::default();
    let mut replayer = match Replayer::new() {
        Ok(r) => r,
        Err(e) => {
            report.entries.push(ReplayEntry {
                name: Name::str("<init>"),
                kind: "init",
                outcome: ReplayOutcome::Rejected {
                    reason: format!("replayer init failed: {e}"),
                },
            });
            return report;
        }
    };

    for decl in &file.decls {
        let entry = replayer.replay(decl);
        report.entries.push(entry);
    }
    report
}

/// Replay one declaration into `env` (stateless convenience; no deferral
/// cascade — prefer [`Replayer`] for whole files).
#[must_use]
pub fn replay_decl(env: &mut Environment, decl: &ExportDecl) -> ReplayEntry {
    let name = decl.primary_name();
    let kind = decl.kind_label();
    let outcome = match decl {
        ExportDecl::Axiom(v) => check_one(env, ConstantInfo::Axiom(v.clone())),
        ExportDecl::Definition(v) => check_one(env, ConstantInfo::Definition(v.clone())),
        ExportDecl::Theorem(v) => check_one(env, ConstantInfo::Theorem(v.clone())),
        ExportDecl::Opaque(v) => check_one(env, ConstantInfo::Opaque(v.clone())),
        ExportDecl::Quot(_) => wave3b::replay_quot(env, decl),
        ExportDecl::Inductive(_) => wave3b::replay_inductive(env, decl),
        ExportDecl::Oversized(v) => ReplayOutcome::Unsupported { feature: v.feature },
    };
    ReplayEntry {
        name,
        kind,
        outcome,
    }
}

/// Run `check_constant_info` and map its result to a [`ReplayOutcome`].
fn check_one(env: &mut Environment, ci: ConstantInfo) -> ReplayOutcome {
    if let ConstantInfo::Axiom(av) = &ci {
        if let Some(reason) = validate_quot_sound_axiom(env, av) {
            return ReplayOutcome::Rejected { reason };
        }
    }
    match check_constant_info(env, ci) {
        Ok(()) => ReplayOutcome::Checked,
        Err(e) => ReplayOutcome::Rejected {
            reason: render_kernel_error(&e),
        },
    }
}

/// Render a [`KernelError`] into a compact human-readable string.
fn render_kernel_error(e: &KernelError) -> String {
    format!("{e:?}")
}

// ---------------------------------------------------------------------------
// Quotients: add_quot + per-record validation, and the Quot.sound axiom.
// ---------------------------------------------------------------------------

/// A group replay failure, before cascade mapping.
enum SeamFailure {
    /// Valid Lean this build cannot replay yet (named feature).
    Unsupported(&'static str),
    /// A kernel error (subject to the `UnknownConstant` cascade).
    Kernel(KernelError),
    /// A rejection with a pre-rendered reason (no cascade applies).
    Reason(String),
}

/// Replay one `quot` record.
///
/// The **first** quot record triggers the kernel's once-only
/// [`Environment::add_quot`], which installs all four primitives (`Quot`,
/// `Quot.mk`, `Quot.lift`, `Quot.ind`) with kernel-constructed canonical
/// types (it requires `Eq` to be declared first). Every quot record —
/// including the triggering one — is then validated against the installed
/// primitive: same kind under that name, matching level-parameter count, and
/// a type definitionally equal to the canonical one (after aligning level
/// parameter names). Records that validate are `Checked` (the primitive is
/// already installed); mismatches are rejections.
#[allow(clippy::result_large_err)] // KernelError is large; replay is not a hot error path
fn replay_quot_core(env: &mut Environment, qv: &QuotVal) -> Result<(), SeamFailure> {
    if !env.quot_initialized() {
        env.add_quot()
            .map_err(|e| SeamFailure::Reason(format!("cannot initialize quotients: {e}")))?;
    }
    // The record must name one of the four installed primitives, with the
    // right kind.
    match env.find(&qv.common.name) {
        Some(ConstantInfo::Quotient(installed)) if installed.kind == qv.kind => {}
        Some(ConstantInfo::Quotient(installed)) => {
            return Err(SeamFailure::Reason(format!(
                "quot record '{}' declares kind {:?} but the canonical primitive has kind {:?}",
                qv.common.name, qv.kind, installed.kind
            )));
        }
        _ => {
            return Err(SeamFailure::Reason(format!(
                "quot record '{}' does not name a canonical quotient primitive",
                qv.common.name
            )));
        }
    }
    // Validate the exported type against the canonical one, tolerating level
    // parameter renaming (the parameter *names* are not semantic).
    let canonical_params = canonical_quot_level_params(qv.kind);
    let aligned = align_level_params(
        &canonical_quot_type(qv.kind),
        &canonical_params,
        &qv.common.level_params,
    )
    .ok_or_else(|| {
        SeamFailure::Reason(format!(
            "quot record '{}' declares universe parameters {:?}, expected {} distinct parameter(s)",
            qv.common.name,
            qv.common.level_params,
            canonical_params.len()
        ))
    })?;
    let mut tc = TypeChecker::new(env);
    if !tc.is_def_eq(&qv.common.ty, &aligned) {
        return Err(SeamFailure::Reason(format!(
            "quot record '{}' does not have the canonical type of its primitive",
            qv.common.name
        )));
    }
    Ok(())
}

/// Instantiate `ty`'s level parameters `from` with the caller-supplied
/// parameter names `to`, or `None` if `to` is not a same-length list of
/// distinct names (level parameter lists with duplicates are ill-formed and
/// must not silently collapse distinct canonical parameters).
fn align_level_params(
    ty: &oxilean_kernel::Expr,
    from: &[Name],
    to: &[Name],
) -> Option<oxilean_kernel::Expr> {
    if to.len() != from.len() {
        return None;
    }
    for (i, n) in to.iter().enumerate() {
        if to[..i].contains(n) {
            return None;
        }
    }
    let levels: Vec<Level> = to.iter().cloned().map(Level::param).collect();
    Some(instantiate_type_lparams(ty, from, levels.as_slice()))
}

/// If `av` declares the quotient soundness axiom `Quot.sound`, validate its
/// type against the kernel's canonical form (see
/// [`oxilean_kernel::env::canonical_quot_sound_type`]):
///
/// ```text
/// ∀ {α : Sort u} {r : α → α → Prop} {a b : α},
///     r a b → Quot.mk r a = Quot.mk r b
/// ```
///
/// Returns `Some(reason)` if the axiom claims the trusted `Quot.sound` name
/// with a non-canonical type (or before the `#QUOT` primitives exist), and
/// `None` for every other axiom (or a canonical `Quot.sound`).
fn validate_quot_sound_axiom(env: &Environment, av: &AxiomVal) -> Option<String> {
    if av.common.name != quot_sound_name() {
        return None;
    }
    if !env.quot_initialized() {
        return Some(
            "axiom 'Quot.sound' requires the #QUOT primitives to be initialized first".into(),
        );
    }
    let canonical_params = canonical_quot_sound_level_params();
    let Some(aligned) = align_level_params(
        &canonical_quot_sound_type(),
        &canonical_params,
        &av.common.level_params,
    ) else {
        return Some(format!(
            "axiom 'Quot.sound' declares universe parameters {:?}, expected {} distinct parameter(s)",
            av.common.level_params,
            canonical_params.len()
        ));
    };
    let mut tc = TypeChecker::new(env);
    if !tc.is_def_eq(&av.common.ty, &aligned) {
        return Some(
            "axiom 'Quot.sound' does not have the canonical quotient soundness type".into(),
        );
    }
    None
}

// ---------------------------------------------------------------------------
// Inductive families.
// ---------------------------------------------------------------------------

/// Replay one `inductive` bundle (a whole mutual group).
///
/// Faithful translation onto the kernel's family API: level parameters and
/// `num_params` are taken from the exported types (and must agree across the
/// mutual group), constructor telescopes are the exported constructor types in
/// the order listed by each `InductiveVal`.
///
/// The bundle is first fully checked *without modifying the environment*
/// (`check_and_derive_family`: signature checks, constructor telescopes and
/// universes, WHNF-hardened strict positivity, recursor derivation), so a
/// failing bundle installs nothing. The **exported** declarations are then
/// installed through `check_constant_info` in dependency order — inductives,
/// constructors, recursors — each re-verified by the kernel; in particular
/// every exported recursor is compared against the kernel's re-derived one
/// (`verify_recursor_val`) and a mismatch is a rejection, never a silent
/// substitution.
///
/// Nested inductives (either flagged by the exporter via `num_nested` or
/// detected by the kernel's positivity check as
/// `KernelError::UnsupportedNestedInductive`) are reported as the named
/// unsupported feature [`NESTED_INDUCTIVES`].
#[allow(clippy::result_large_err)] // KernelError is large; replay is not a hot error path
fn replay_inductive_core(env: &mut Environment, b: &InductiveBundle) -> Result<(), SeamFailure> {
    // Named unsupported: nested inductives. The exporter flag gives the
    // earliest classification; the kernel's positivity check independently
    // catches unflagged nesting below (never a wrong accept).
    if b.types.iter().any(|t| t.num_nested > 0) {
        return Err(SeamFailure::Unsupported(NESTED_INDUCTIVES));
    }
    let first = b
        .types
        .first()
        .ok_or_else(|| SeamFailure::Reason("empty inductive bundle".into()))?;
    let lparams = first.common.level_params.clone();
    let num_params = first.num_params;
    for t in &b.types {
        if t.common.level_params != lparams || t.num_params != num_params {
            return Err(SeamFailure::Reason(format!(
                "mutual inductive '{}' disagrees with '{}' on level params or param count",
                t.common.name, first.common.name
            )));
        }
    }
    // Faithful translation: name + full type + constructor telescopes, in the
    // exported constructor order (each InductiveVal's `ctors` list).
    let mut specs = Vec::with_capacity(b.types.len());
    for t in &b.types {
        let mut ctors = Vec::with_capacity(t.ctors.len());
        for cname in &t.ctors {
            let cv = b
                .ctors
                .iter()
                .find(|c| &c.common.name == cname)
                .ok_or_else(|| {
                    SeamFailure::Reason(format!(
                        "inductive '{}' lists constructor '{}' but the bundle does not contain it",
                        t.common.name, cname
                    ))
                })?;
            ctors.push((cname.clone(), cv.common.ty.clone()));
        }
        specs.push(InductiveSpec::new(
            t.common.name.clone(),
            t.common.ty.clone(),
            ctors,
        ));
    }
    // Atomicity pre-check: fully check + derive the family against the current
    // environment WITHOUT modifying it. A bundle that fails here installs
    // nothing.
    let derived = oxilean_kernel::check_and_derive_family(env, &lparams, num_params, &specs)
        .map_err(map_inductive_kernel_error)?;
    // Install the EXPORTED declarations in dependency order, each re-verified
    // by the kernel (metadata lies — wrong num_indices, cidx, num_fields, ... —
    // are caught here even though the family itself derived cleanly).
    //
    // One field is NOT exported: lean4export does not emit `isProp` (the
    // reader defaults it to `false`), so it is taken from the kernel's own
    // derivation instead of the file — the kernel would otherwise correctly
    // reject every Prop-valued inductive for carrying a wrong flag.
    for (i, t) in b.types.iter().enumerate() {
        let mut iv = t.clone();
        if let Some(ConstantInfo::Inductive(d)) = derived.inductives.get(i) {
            iv.is_prop = d.is_prop;
        }
        check_constant_info(env, ConstantInfo::Inductive(iv))
            .map_err(map_inductive_kernel_error)?;
    }
    for c in &b.ctors {
        check_constant_info(env, ConstantInfo::Constructor(c.clone()))
            .map_err(map_inductive_kernel_error)?;
    }
    for r in &b.recs {
        check_constant_info(env, ConstantInfo::Recursor(r.clone()))
            .map_err(map_inductive_kernel_error)?;
    }
    Ok(())
}

/// Kernel error mapping for inductive bundles (the three-bucket rule):
/// nested inductives are a *named unsupported feature*, everything else stays
/// a kernel error (rejection, or dependency cascade for `UnknownConstant`).
fn map_inductive_kernel_error(e: KernelError) -> SeamFailure {
    match e {
        KernelError::UnsupportedNestedInductive(_) => SeamFailure::Unsupported(NESTED_INDUCTIVES),
        other => SeamFailure::Kernel(other),
    }
}

// ---------------------------------------------------------------------------
// Corpus-scale streaming replay with explicit limits.
// ---------------------------------------------------------------------------

/// Resource limits for a streaming replay.
///
/// The default is the untrusted-input configuration: the reader's default
/// materialization budget, no time budget, and the [`DEFAULT_DECL_FUEL`]
/// per-declaration resource budget.
#[derive(Debug, Clone, Copy)]
pub struct ReplayLimits {
    /// Reader limits (the cumulative expression-materialization node budget).
    pub read: Limits,
    /// Optional **advisory** per-declaration wall-clock budget. Checked after
    /// each declaration completes — it cannot preempt the kernel mid-check
    /// (kernel-side fuel is tracked separately) — and over-budget declarations
    /// are recorded in [`ReplayStats::over_time_budget`] with their true
    /// outcome unchanged.
    pub per_decl_time_budget: Option<Duration>,
    /// Optional **hard, deterministic** per-declaration resource budget, in
    /// kernel `Expr` nodes cloned (see [`Replayer::set_per_decl_fuel`] and
    /// [`RESOURCE_LIMIT`]). `None` = unlimited.
    pub per_decl_fuel: Option<u64>,
}

/// The default per-declaration clone-fuel budget for untrusted input
/// (`ReplayLimits::default()`): 2^24 ≈ 16.8M cloned nodes, empirically
/// ≲ 1 GiB of peak term memory.
pub const DEFAULT_DECL_FUEL: u64 = 1 << 24;

/// The per-declaration clone-fuel budget of the corpus preset
/// ([`ReplayLimits::corpus`]): 2^26 ≈ 67M cloned nodes. Calibrated against
/// the heaviest known Lean-core declaration (`Int.add_mul_ediv_right`,
/// ~26M nodes / ~0.9 GiB peak after the iterative-whnf work) with ~2.5×
/// headroom; a runaway declaration is cut off around a few GiB instead of
/// exhausting the machine.
pub const CORPUS_DECL_FUEL: u64 = 1 << 26;

impl Default for ReplayLimits {
    /// The untrusted-input configuration: default reader budget, no time
    /// budget, [`DEFAULT_DECL_FUEL`] as the per-declaration resource budget.
    fn default() -> Self {
        Self {
            read: Limits::default(),
            per_decl_time_budget: None,
            per_decl_fuel: Some(DEFAULT_DECL_FUEL),
        }
    }
}

impl ReplayLimits {
    /// The documented whole-corpus preset: [`Limits::corpus`] for the node
    /// budget (large trusted exports like `Init.ndjson`), no time budget,
    /// [`CORPUS_DECL_FUEL`] as the per-declaration resource budget.
    #[must_use]
    pub fn corpus() -> Self {
        Self {
            read: Limits::corpus(),
            per_decl_time_budget: None,
            per_decl_fuel: Some(CORPUS_DECL_FUEL),
        }
    }
}

/// Timing statistics from a streaming replay.
#[derive(Debug, Clone, Default)]
pub struct ReplayStats {
    /// Number of declarations replayed.
    pub decls_replayed: usize,
    /// The slowest declaration and its wall-clock time, if any were replayed.
    pub max_decl_time: Option<(Name, Duration)>,
    /// Declarations that exceeded [`ReplayLimits::per_decl_time_budget`]
    /// (empty when no budget was set).
    pub over_time_budget: Vec<(Name, Duration)>,
}

/// The result of a full streaming replay.
#[derive(Debug, Clone)]
pub struct ReplayRun {
    /// The parsed metadata header.
    pub meta: Meta,
    /// Reader statistics (record counts, materialized nodes).
    pub read_stats: ReadStats,
    /// The per-declaration replay report.
    pub report: ReplayReport,
    /// Replay timing statistics.
    pub stats: ReplayStats,
}

/// Replay an export straight from a reader under explicit [`ReplayLimits`],
/// without retaining declarations (resident memory stays proportional to the
/// reader's primitive index tables plus the kernel environment).
///
/// This is the corpus-scale entry point: for whole-library exports pass
/// [`ReplayLimits::corpus`] (see [`Limits::corpus`] for the measured
/// `Init.ndjson` numbers that motivate the preset).
///
/// # Errors
/// Returns the reader's three-bucket [`crate::ExportError`] on malformed
/// input, an unsupported *reader* construct (e.g. the node budget), or I/O
/// failure. Kernel verdicts are never errors — they are the three buckets in
/// [`ReplayRun::report`].
pub fn replay_streaming<R: BufRead>(reader: R, limits: ReplayLimits) -> ExportResult<ReplayRun> {
    let mut report = ReplayReport::default();
    let mut stats = ReplayStats::default();
    let mut replayer = match Replayer::new() {
        Ok(r) => r,
        Err(e) => {
            report.entries.push(ReplayEntry {
                name: Name::str("<init>"),
                kind: "init",
                outcome: ReplayOutcome::Rejected {
                    reason: format!("replayer init failed: {e}"),
                },
            });
            // Still consume the stream so reader errors surface consistently.
            let (meta, read_stats) = read_streaming(reader, limits.read, |_| Ok(()))?;
            return Ok(ReplayRun {
                meta,
                read_stats,
                report,
                stats,
            });
        }
    };

    replayer.set_per_decl_fuel(limits.per_decl_fuel);
    let (meta, read_stats) = read_streaming(reader, limits.read, |decl| {
        let started = Instant::now();
        let entry = replayer.replay(&decl);
        let elapsed = started.elapsed();
        stats.decls_replayed += 1;
        let is_new_max = stats
            .max_decl_time
            .as_ref()
            .map_or(true, |(_, best)| elapsed > *best);
        if is_new_max {
            stats.max_decl_time = Some((entry.name.clone(), elapsed));
        }
        if let Some(budget) = limits.per_decl_time_budget {
            if elapsed > budget {
                stats.over_time_budget.push((entry.name.clone(), elapsed));
            }
        }
        report.entries.push(entry);
        Ok(())
    })?;

    Ok(ReplayRun {
        meta,
        read_stats,
        report,
        stats,
    })
}

/// The (historical) Wave-3b seam.
///
/// These entry points were introduced as `TODO(wave3b)` stubs when only
/// axiom/def/thm/opaque replay existed; the bodies are now the real
/// implementations (the module name is kept for API stability):
///
/// * [`replay_quot`](crate::replay::wave3b::replay_quot) — installs the four `Quot` primitives via the kernel's
///   once-only `add_quot` and validates each exported record against the
///   installed canonical primitive.
/// * [`replay_inductive`](crate::replay::wave3b::replay_inductive) — checks the mutual group as a family and installs
///   the exported declarations, with the kernel **re-deriving** recursors and
///   comparing them against the exported ones.
///
/// Both are *stateless* (no dependency cascade); [`Replayer`] applies the
/// cascade on top of the same logic.
pub mod wave3b {
    use super::{
        replay_inductive_core, replay_quot_core, Environment, ExportDecl, ReplayOutcome,
        SeamFailure,
    };

    /// Map a stateless seam result onto an outcome (no cascade: an
    /// `UnknownConstant` failure is a plain rejection here).
    fn stateless_outcome(res: Result<(), SeamFailure>) -> ReplayOutcome {
        match res {
            Ok(()) => ReplayOutcome::Checked,
            Err(SeamFailure::Unsupported(feature)) => ReplayOutcome::Unsupported { feature },
            Err(SeamFailure::Kernel(e)) => ReplayOutcome::Rejected {
                reason: super::render_kernel_error(&e),
            },
            Err(SeamFailure::Reason(reason)) => ReplayOutcome::Rejected { reason },
        }
    }

    /// Replay a `quot` record: install the four quotient primitives via the
    /// kernel's `add_quot` on first sight, then validate the record against
    /// the installed canonical primitive. See the module docs.
    #[must_use]
    pub fn replay_quot(env: &mut Environment, decl: &ExportDecl) -> ReplayOutcome {
        match decl {
            ExportDecl::Quot(qv) => stateless_outcome(replay_quot_core(env, qv)),
            other => ReplayOutcome::Rejected {
                reason: format!(
                    "replay_quot called on non-quot declaration '{}'",
                    other.primary_name()
                ),
            },
        }
    }

    /// Replay an `inductive` bundle: check the mutual family, install the
    /// exported declarations, and have the kernel re-derive and compare the
    /// recursors. See the module docs.
    #[must_use]
    pub fn replay_inductive(env: &mut Environment, decl: &ExportDecl) -> ReplayOutcome {
        match decl {
            ExportDecl::Inductive(b) => stateless_outcome(replay_inductive_core(env, b)),
            other => ReplayOutcome::Rejected {
                reason: format!(
                    "replay_inductive called on non-inductive declaration '{}'",
                    other.primary_name()
                ),
            },
        }
    }
}
