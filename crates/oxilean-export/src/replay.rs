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
//!   positivity). The types and constructors are installed as the file
//!   declares them, each re-verified by the kernel. An exported **recursor
//!   is never installed**: Lean's kernel takes no recursor from its input,
//!   and neither does this one — the recursor in the environment is the one
//!   the kernel derives (its type, its rules, its counts, its `k` flag),
//!   under the record's name and universe parameter names. The record is
//!   only held against it, each record with the derived recursor of **its
//!   own name** (`T.rec`; on the nested path `<first type>.rec_<k>` for the
//!   `k`-th auxiliary type): its type and each rule's right-hand side must
//!   be the derived ones as written (up to the names and the binder
//!   information of binders) or, failing that, be well typed and only then
//!   definitionally equal to them. A comparison by definitional equality
//!   alone is not a check — it takes an ill-typed proof term for a proof.
//!   What the file says about the family beyond its declaration
//!   (`numIndices`, `isRec`, `isReflexive`, `numNested`, the mutual group,
//!   `cidx`, `numFields`, ...) is what Lean's kernel computes when it adds
//!   the declaration; it is compared with this kernel's own derivation and
//!   a difference rejects the bundle. A bundle is installed **whole or not
//!   at all**.
//! * A `thm` record must state a proposition: the kernel refuses a theorem
//!   whose type is not in `Prop`, as Lean's does.
//! * `quot` records install the four `#QUOT` primitives via the kernel's
//!   once-only `Environment::add_quot` (kernel-constructed canonical types;
//!   `Eq` must be the inductive type they assume, type and constructor
//!   type compared as Lean compares them); each exported record is then
//!   held against the installed primitive, once per primitive — its type
//!   compared as a recursor's is, and never installed.
//! * The `Quot.sound` **axiom** (a plain axiom record in exports) is validated
//!   against the kernel's canonical soundness type — an export cannot smuggle
//!   an arbitrary assertion under that trusted name — and then checked and
//!   added like any other axiom.
//!
//! ## What is installed
//!
//! A term of the file is in the environment — and reaches definitional
//! equality, weak-head reduction or iota — only after the kernel
//! type-checked it: the type of an axiom; the type and the value of a
//! definition, a theorem and an opaque constant; an inductive type; a
//! constructor's type. Where the kernel derives a term itself — a recursor
//! (its type and its rules, on the ordinary and on the nested path) and the
//! quotient primitives — its own term is installed, and the file's record is
//! compared with it and dropped.
//!
//! A derived term names other constants, and the installed rules of a
//! mutual or nested family call the sibling recursors **by name**. Two
//! things keep those names to the constants installed. A recursor record is
//! accepted only under the name Lean's kernel gives the recursor
//! (`T.rec`, the component `rec` under the type's name, or
//! `<first type>.rec_<k>`), so the derivation names every recursor as Lean
//! does; and the kernel's guard (`oxilean_kernel::inductive::installed`)
//! holds every constant a derived term names to the environment as the
//! install leaves it — it must resolve, and a name the derivation gives a
//! recursor or a type of the family must be that constant — and refuses the
//! bundle otherwise. (Before both, a record under the one flat component
//! `"A.rec"` made the derivation name the family in that style: the rules
//! called recursors that were never installed, and a later record could
//! define one and prove `False` through iota.)
//!
//! ## Three buckets
//!
//! Every declaration lands in exactly one bucket ([`ReplayOutcome`]):
//!
//! * `Checked` — the kernel accepted it.
//! * `Unsupported { feature }` — no verdict, with a named feature: valid
//!   Lean the kernel cannot replay *yet* ([`NESTED_INDUCTIVES`]: a nested
//!   inductive it does not un-nest, such as one with parameters or one
//!   nested through a mutual container), or a declaration that ran into a
//!   resource limit ([`RESOURCE_LIMIT`], [`REDUCTION_LIMIT`],
//!   [`TERM_DEPTH_LIMIT`], or [`DECL_BUDGET_FEATURE`](crate::DECL_BUDGET_FEATURE)
//!   when the reader would not materialize it) — a limit is never a verdict
//!   on the proof; plus [`DEFERRED_DEPENDENCY`] for declarations that only
//!   fail because they depend on an unsupported one.
//! * `Rejected { reason }` — the kernel judged it ill-typed (or it depends on
//!   a rejected declaration; the reason names the root cause).
//!
//! ## What a failed record leaves behind
//!
//! Nothing of its own. A record that is not `Checked` adds none of its
//! declarations to the environment — an inductive bundle refused at its
//! last recursor adds neither its types nor its constructors — and every
//! name it declares joins the replayer's set of rejected (or deferred)
//! names. (The first `quot` record makes the kernel install its four
//! canonical quotient primitives, which are the kernel's own and stay
//! whatever becomes of the record; a `quot` record that is then refused
//! still has its name in the rejected set.) A later record that mentions
//! such a name, as a constant or as the structure of a projection, is not
//! handed to the kernel at all: it "depends on rejected declaration" (or on
//! an unsupported one) whether or not a constant of that name is present.
//! A later record that declares such a name again is rejected: a name is
//! declared once in a Lean environment. The same holds for a name a record
//! declared and was accepted for — the kernel refuses a second constant of
//! a name, and a second `quot` record for a primitive an earlier `quot`
//! record declared is refused by the replayer (the kernel has installed all
//! four primitives by then and could not tell) — and that name then joins
//! the rejected set, with the same consequence for the records after it.
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
use oxilean_kernel::expr_util::find_mentioned_name;
use oxilean_kernel::instantiate::instantiate_type_lparams;
use oxilean_kernel::{
    check_constant_info, check_constant_infos_atomic, escaped_name, supplied_type_matches,
    AxiomVal, ConstantInfo, DerivedFamily, Environment, Expr, InductiveSpec, KernelError, Level,
    Name, NameView, QuotVal,
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
/// a non-family type constructor in a constructor field) that the kernel does
/// not un-nest. Since 0.1.4 the kernel elaborates a nested inductive to a
/// mutual family when the family has no parameters and the container is not
/// part of a mutual block; for any other nesting it reports
/// `KernelError::UnsupportedNestedInductive`, and replay surfaces that as this
/// named unsupported feature (three-bucket rule: valid Lean, not replayable
/// yet, never a rejection).
pub const NESTED_INDUCTIVES: &str = "nested inductives";

/// The named feature reported when a declaration exceeded the deterministic
/// per-declaration resource budget ([`Replayer::set_per_decl_fuel`], measured
/// in units of kernel work — the subterms its term builders reach; see
/// `oxilean_kernel::fuel`). The string is part of the report format and
/// keeps the name the unit had when it counted cloned nodes.
///
/// This is a **named unsupported** verdict, never a rejection: the kernel
/// degrades conservatively when fuel runs out (stuck reduction, syntactic
/// def-eq only), so the failure means "this build refused to spend more
/// resources", not "this is not a proof". Deterministic: the same
/// declaration with the same budget always lands in the same bucket.
pub const RESOURCE_LIMIT: &str = "resource limit exceeded: per-declaration clone-fuel budget";

/// Named unsupported feature: the kernel's reducer stopped a reduction at one
/// of its own limits and the declaration then failed to check.
///
/// The reducer caps the NESTING depth of reduction (a stack guard) and the
/// LENGTH of one head-reduction chain. A reduction cut short by either cap
/// returns a term that is not a normal form, so a comparison that fails
/// afterwards says nothing about the proof. A [`Replayer`] with a
/// per-declaration budget repeats such a declaration with the chain cap
/// lifted — the budget bounds the chain instead (see `oxilean_kernel::fuel`) —
/// so under a budget only the nesting-depth guard leads here; with no budget
/// the chain cap does too. Like [`RESOURCE_LIMIT`] this is a **named
/// unsupported** verdict, never a rejection, and it is deterministic.
pub const REDUCTION_LIMIT: &str = "resource limit exceeded: reduction depth or step limit";

/// Named unsupported feature: a term of the declaration, or one built while
/// checking it, is deeper than the checking thread allows
/// (`oxilean_kernel::fuel::set_term_depth_limit`).
///
/// The kernel walks terms by recursion, so the deepest term it holds decides
/// how much native stack it needs. Where that stack is small and cannot be
/// enlarged — a WebAssembly build runs on its host's, about a megabyte in a
/// browser — the kernel is given a limit on the depth of its terms (the
/// WebAssembly build of `oxilean-verify` sets 512; the command-line tool
/// sets none), and a declaration over it is not checked further. Like
/// [`RESOURCE_LIMIT`] this is a **named unsupported** verdict, never a
/// rejection, and it is deterministic: it depends on the terms, not on how
/// much stack happened to be free.
pub const TERM_DEPTH_LIMIT: &str = "resource limit exceeded: term depth";

/// A stateful replayer: an initially **empty** kernel [`Environment`] plus the
/// bookkeeping needed for honest three-bucket reporting:
///
/// * `deferred` — names whose declarations were *unsupported* (e.g. nested
///   inductives). A later declaration that mentions a deferred name is
///   reported as `Unsupported` ([`DEFERRED_DEPENDENCY`]) rather than as a
///   rejection, and its own name joins the set (the cascade).
/// * `rejected` — names whose declarations the kernel *rejected*. A later
///   declaration that mentions a rejected name is reported as `Rejected`
///   with a reason naming the root cause, and its own name joins the set.
///
/// A declaration is looked at for such a name **before** the kernel sees
/// it (`Replayer::blocked_by_an_earlier_failure`), so the cascade does not
/// depend on the name being absent from the environment; a failure with
/// `KernelError::UnknownConstant` on a name of either set is mapped the
/// same way.
///
/// This keeps `Rejected` meaning exactly "the kernel could not verify this"
/// and `Unsupported` meaning "valid Lean this build cannot replay yet".
pub struct Replayer {
    env: Environment,
    deferred: HashSet<Name>,
    rejected: HashSet<Name>,
    /// The quotient primitives a `quot` record of this file has declared.
    /// The kernel installs all four at the first record, so the environment
    /// cannot say which of them the file has already declared; a second
    /// record for one of these names is rejected like a second record of
    /// any other name.
    quot_records: HashSet<Name>,
    /// Deterministic per-declaration resource budget, in units of kernel work
    /// (`None` = unlimited). See [`RESOURCE_LIMIT`].
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
            quot_records: HashSet::new(),
            per_decl_fuel: None,
            per_decl_time_budget: None,
        })
    }

    /// Set the deterministic per-declaration resource budget, in units of
    /// kernel work — the subterms the kernel's term builders reach, see
    /// `oxilean_kernel::fuel` (`None` = unlimited, the default).
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

    /// Arm the per-declaration budgets for one attempt at a declaration:
    /// fuel, deadline, cleared latches, the reducer's chain cap on.
    fn arm_budgets(&self) {
        oxilean_kernel::fuel::set_budget(self.per_decl_fuel);
        oxilean_kernel::deadline::set_deadline(self.per_decl_time_budget);
    }

    /// Replay one declaration.
    pub fn replay(&mut self, decl: &ExportDecl) -> ReplayEntry {
        self.arm_budgets();
        let name = decl.primary_name();
        let kind = decl.kind_label();
        let outcome = match self.blocked_by_an_earlier_failure(decl) {
            Some(outcome) => outcome,
            None => self.replay_unblocked(decl, &name),
        };
        ReplayEntry {
            name,
            kind,
            outcome,
        }
    }

    /// The outcome of a record that names a declaration this replay has
    /// already failed on, or `None` when it names none — the record then
    /// goes to the kernel.
    ///
    /// Two ways of naming one:
    ///
    /// * **Declaring it again.** A name of the rejected or of the deferred
    ///   set that the record itself declares: rejected. A failed record
    ///   leaves nothing in the environment, so the kernel would not see the
    ///   clash; and a file in which a name is declared twice is not the
    ///   export of a Lean environment.
    /// * **Mentioning it** in a type, a value or a recursor rule — as a
    ///   constant, or as the structure a projection reads. A rejected name
    ///   makes the record "depends on rejected declaration" (rejected); a
    ///   deferred one, with no rejected name beside it, makes it a
    ///   [`DEFERRED_DEPENDENCY`] (unsupported). This holds **whether or not
    ///   a constant of that name is in the environment**: the verdict on a
    ///   dependent must not rest on every failing path having cleaned up
    ///   after itself.
    ///
    /// Every name the record declares joins the set its outcome belongs to.
    /// With both sets empty — every record so far was checked — no term is
    /// walked.
    fn blocked_by_an_earlier_failure(&mut self, decl: &ExportDecl) -> Option<ReplayOutcome> {
        if self.rejected.is_empty() && self.deferred.is_empty() {
            return None;
        }
        let own = declared_names(decl);
        let again = own
            .iter()
            .find(|n| self.rejected.contains(*n) || self.deferred.contains(*n));
        if let Some(n) = again {
            let reason = format!(
                "declares '{n}' again: an earlier record of this file declared it and was not accepted"
            );
            self.reject_all(&own);
            return Some(ReplayOutcome::Rejected { reason });
        }
        let mut rejected_dep: Option<&Name> = None;
        let mut deferred_dep = false;
        for term in declared_terms(decl) {
            rejected_dep = find_mentioned_name(term, |n| {
                if self.rejected.contains(n) {
                    return true;
                }
                deferred_dep = deferred_dep || self.deferred.contains(n);
                false
            });
            if rejected_dep.is_some() {
                break;
            }
        }
        if let Some(n) = rejected_dep {
            let reason = format!("depends on rejected declaration '{n}'");
            self.reject_all(&own);
            return Some(ReplayOutcome::Rejected { reason });
        }
        if deferred_dep {
            self.defer_all(&own);
            return Some(ReplayOutcome::Unsupported {
                feature: DEFERRED_DEPENDENCY,
            });
        }
        None
    }

    /// Hand a record that names no failed declaration to the kernel.
    fn replay_unblocked(&mut self, decl: &ExportDecl, name: &Name) -> ReplayOutcome {
        match decl {
            ExportDecl::Axiom(v) => self.check(|| ConstantInfo::Axiom(v.clone()), name),
            ExportDecl::Definition(v) => self.check(|| ConstantInfo::Definition(v.clone()), name),
            ExportDecl::Theorem(v) => self.check(|| ConstantInfo::Theorem(v.clone()), name),
            ExportDecl::Opaque(v) => self.check(|| ConstantInfo::Opaque(v.clone()), name),
            ExportDecl::Quot(v) => {
                // The first record of a primitive's name is held against the
                // installed primitive; a second one declares the name again.
                let res = if self.quot_records.contains(name) {
                    Err(SeamFailure::Reason(format!(
                        "declares '{name}' again: an earlier quot record of this file declared it"
                    )))
                } else {
                    replay_quot_core(&mut self.env, v)
                };
                if res.is_ok() {
                    self.quot_records.insert(name.clone());
                }
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
        }
    }

    /// Run `check_constant_info` and map its result to a [`ReplayOutcome`],
    /// cascading deferrals/rejections for unknown constants whose declarations
    /// were themselves unsupported/rejected.
    ///
    /// `make` builds the declaration to check; it is called once per attempt.
    ///
    /// A second attempt is made in exactly one situation: the first one
    /// FAILED, the reducer's chain cap had fired during it, and the budget is
    /// not spent. The cap cuts a head-reduction chain off after a fixed
    /// number of steps, which is what lets definitional equality give up on a
    /// reduction that is going nowhere — but the chain it cut may instead be
    /// a long, cheap computation the proof depends on (a `decide` proof
    /// iterating `Nat.rec` tens of thousands of times), and then the failure
    /// says nothing about the proof. The second attempt runs under a fresh
    /// budget with the cap lifted, so the budget alone bounds the chain. Its
    /// result is the declaration's result: checked, a genuine rejection, or a
    /// named resource limit. Nothing was added to the environment by the
    /// failed first attempt, so the second starts from the same state.
    fn check<F: Fn() -> ConstantInfo>(&mut self, make: F, own_name: &Name) -> ReplayOutcome {
        let first_ci = make();
        if let ConstantInfo::Axiom(av) = &first_ci {
            if let Some(reason) = validate_quot_sound_axiom(&self.env, av) {
                self.rejected.insert(own_name.clone());
                return ReplayOutcome::Rejected { reason };
            }
        }
        let mut result = check_constant_info(&mut self.env, first_ci);
        if let Err(e) = &result {
            let cap_cut_a_chain = oxilean_kernel::fuel::reduction_limit_hit()
                && !oxilean_kernel::fuel::is_exhausted()
                && oxilean_kernel::fuel::is_metered();
            if cap_cut_a_chain && !self.has_known_cause(e) {
                self.arm_budgets();
                oxilean_kernel::fuel::set_reduction_cap(false);
                result = check_constant_info(&mut self.env, make());
                oxilean_kernel::fuel::set_reduction_cap(true);
            }
        }
        match result {
            Ok(()) => ReplayOutcome::Checked,
            Err(e) => self.map_failure(std::slice::from_ref(own_name), SeamFailure::Kernel(e)),
        }
    }

    /// Whether a kernel error is already accounted for by an earlier
    /// declaration: an unknown constant that was itself deferred or rejected.
    fn has_known_cause(&self, e: &KernelError) -> bool {
        match e {
            KernelError::UnknownConstant(n) => {
                self.deferred.contains(n) || self.rejected.contains(n)
            }
            _ => false,
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
        // A failure at a resource limit is a NAMED unsupported verdict, never
        // a rejection. With the fuel budget or the deadline spent the kernel
        // degraded conservatively (stuck reduction, syntactic def-eq only),
        // so the error only means "this build refused to spend more resources
        // on this declaration". The same holds when the reducer cut a
        // reduction short at one of its own limits — the nesting depth, or
        // the chain length where the check could not be repeated without
        // that cap (no budget armed; see `Self::check`): the term it
        // returned is not a normal form, so the comparison that failed
        // afterwards is not a verdict either. Genuine kernel verdicts
        // reached *before* a limit are unaffected (the latches only set when
        // a limit is actually crossed), and the known-cause outcomes — a
        // named unsupported feature, a dependency that was itself deferred
        // or rejected — take precedence.
        let known_cause = match &failure {
            SeamFailure::Unsupported(_) => true,
            SeamFailure::Kernel(e) => self.has_known_cause(e),
            SeamFailure::Reason(_) => false,
        };
        if !known_cause {
            if let Some(feature) = resource_limit_feature() {
                self.defer_all(own_names);
                return ReplayOutcome::Unsupported { feature };
            }
        }
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

/// Every name a record declares.
fn declared_names(decl: &ExportDecl) -> Vec<Name> {
    match decl {
        ExportDecl::Axiom(v) => vec![v.common.name.clone()],
        ExportDecl::Definition(v) => vec![v.common.name.clone()],
        ExportDecl::Theorem(v) => vec![v.common.name.clone()],
        ExportDecl::Opaque(v) => vec![v.common.name.clone()],
        ExportDecl::Quot(v) => vec![v.common.name.clone()],
        ExportDecl::Inductive(bundle) => bundle_names(bundle),
        ExportDecl::Oversized(v) => v.names.clone(),
    }
}

/// Every term a record carries: its types, its values, and the right-hand
/// sides of its recursor rules. (An oversized record carries none: its terms
/// were never built.)
fn declared_terms(decl: &ExportDecl) -> Vec<&Expr> {
    match decl {
        ExportDecl::Axiom(v) => vec![&v.common.ty],
        ExportDecl::Definition(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Theorem(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Opaque(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Quot(v) => vec![&v.common.ty],
        ExportDecl::Inductive(bundle) => {
            let mut terms: Vec<&Expr> = Vec::new();
            terms.extend(bundle.types.iter().map(|t| &t.common.ty));
            terms.extend(bundle.ctors.iter().map(|c| &c.common.ty));
            for r in &bundle.recs {
                terms.push(&r.common.ty);
                terms.extend(r.rules.iter().map(|rule| &rule.rhs));
            }
            terms
        }
        ExportDecl::Oversized(_) => Vec::new(),
    }
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
    // This entry point arms no budget of its own (the caller may have armed
    // one), but a reduction-limit hit left over from an earlier declaration
    // must not be read as this declaration's.
    oxilean_kernel::fuel::clear_reduction_limit();
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

/// The named resource-limit feature that accounts for a failure, if the
/// current declaration ran into one: [`TERM_DEPTH_LIMIT`] when a term was
/// deeper than the thread allows (which also spends the budget, so it is
/// asked first), [`RESOURCE_LIMIT`] when the fuel budget or the deadline is
/// spent, [`REDUCTION_LIMIT`] when the reducer cut a reduction short at one
/// of its own limits. A failure reached in any of these states is not a
/// verdict on the proof.
fn resource_limit_feature() -> Option<&'static str> {
    if oxilean_kernel::fuel::term_depth_limit_hit() {
        Some(TERM_DEPTH_LIMIT)
    } else if oxilean_kernel::fuel::is_exhausted() {
        Some(RESOURCE_LIMIT)
    } else if oxilean_kernel::fuel::reduction_limit_hit() {
        Some(REDUCTION_LIMIT)
    } else {
        None
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
        Err(e) => match resource_limit_feature() {
            Some(feature) => ReplayOutcome::Unsupported { feature },
            None => ReplayOutcome::Rejected {
                reason: render_kernel_error(&e),
            },
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
/// a type that is the canonical one (after aligning level parameter names) —
/// the same as written, or type-checked first and then definitionally equal
/// (`oxilean_kernel::supplied_type_matches`). Records that validate are
/// `Checked` (the primitive is already installed; the record's type is
/// dropped); mismatches are rejections.
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
    // The record's type is only held against the canonical one (which is
    // what is installed): the same as written, or a type the kernel checked
    // and then found definitionally equal — never compared unchecked.
    if !supplied_type_matches(env, &aligned, &qv.common.ty) {
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
    // As written, or type-checked first and then definitionally equal: the
    // axiom's type is not handed to definitional equality unchecked. (The
    // kernel checks it once more when the axiom is added.)
    if !supplied_type_matches(env, &aligned, &av.common.ty) {
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
/// **Whole or not at all.** The bundle is first fully checked *without
/// modifying the environment* (`check_and_derive_family`: signature checks,
/// constructor telescopes and universes, WHNF-hardened strict positivity,
/// recursor derivation). The **exported** declarations are then handed to
/// the kernel as one group (`check_constant_infos_atomic`) in dependency
/// order — inductives, constructors, recursors. The types and constructors
/// are re-verified and added as exported. A recursor record is compared with
/// the kernel's re-derived recursor of the same name (`verify_recursor_val`:
/// as written, or type-checked and then by definitional equality), and a
/// record that is not it is a rejection; what the kernel then adds is its
/// OWN recursor, under the record's name and universe parameter names — the
/// record's type and rules are never installed. A failure at any member
/// takes the earlier ones out again: a bundle that is not accepted leaves no
/// constant behind, neither its types nor its first constructors.
///
/// **Derived, and compared.** A Lean declaration of an inductive family is
/// its universe parameters, its number of parameters, and each type's name,
/// type and constructors (name and type), with one `isUnsafe` flag. Every
/// other field of the bundle is what Lean's kernel computes from that when
/// it adds the declaration, so a file whose value differs from the
/// computation is not the export of a Lean environment. This kernel makes
/// the same computation and the bundle is held to it
/// ([`compare_with_derivation`], and the kernel's own checks of the group):
/// a difference is a rejection. One field is not exported at all:
/// lean4export does not emit `isProp` (the reader defaults it to `false`),
/// so it is taken from the derivation.
///
/// Nested inductives (exporter-flagged `num_nested > 0`) are specialized into
/// an auxiliary mutual family, derived, and their recursors restored over the
/// real container; the family that comes back must be the declared one, and
/// every exported recursor is then held, in the same way, against the
/// derived recursor that carries Lean's name for it
/// (`verify_nested_bundle`); the kernel-derived family is added as one
/// group. Shapes this stage cannot un-nest (and any nesting the kernel's
/// positivity check surfaces as `KernelError::UnsupportedNestedInductive`)
/// are reported as the named unsupported feature [`NESTED_INDUCTIVES`] —
/// unless the kernel can tell that the file's `numNested` is wrong (a family
/// flagged nested that checks as an ordinary one; a family flagged ordinary
/// that verifies as a nested one), which is a rejection like any other
/// field that differs from the derivation.
#[allow(clippy::result_large_err)] // KernelError is large; replay is not a hot error path
fn replay_inductive_core(env: &mut Environment, b: &InductiveBundle) -> Result<(), SeamFailure> {
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
    // Every recursor record under the name Lean's kernel gives a recursor of
    // this declaration, before anything is derived.
    check_recursor_record_names(b)?;
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
    // Nested inductives (exporter-flagged `num_nested > 0`): specialize into an
    // auxiliary mutual family, derive, restore the recursors over the real
    // container, and require every EXPORTED recursor to be the kernel's
    // re-derivation (`verify_nested_bundle`). A shape this stage cannot un-nest
    // surfaces as `UnsupportedNestedInductive` → the named unsupported feature.
    if b.types.iter().any(|t| t.num_nested > 0) {
        let derived = match oxilean_kernel::verify_nested_bundle(
            env, &lparams, num_params, &specs, &b.recs,
        ) {
            Ok(derived) => derived,
            Err(KernelError::UnsupportedNestedInductive(_)) => {
                // No verdict from the nested stage. If the family checks as
                // an ordinary one, it has no nested occurrence at all and
                // the file's count is not what the kernel computes.
                let ordinary =
                    oxilean_kernel::check_and_derive_family(env, &lparams, num_params, &specs);
                return Err(match ordinary {
                    Ok(_) => differs(&first.common.name, "numNested", &first.num_nested, &0u32),
                    Err(_) => SeamFailure::Unsupported(NESTED_INDUCTIVES),
                });
            }
            Err(e) => return Err(SeamFailure::Kernel(e)),
        };
        // Nothing of the file's bundle is installed on this path but its
        // declaration; what it says beyond it is compared in full.
        compare_with_derivation(b, &derived, true)?;
        let is_unsafe = first.is_unsafe;
        let group = derived
            .into_constant_infos()
            .into_iter()
            .map(|ci| with_safety_flag(ci, is_unsafe))
            .collect();
        // One group, after the guard (every constant a term of the derived
        // family names resolves once the group is in, and to the constant
        // the derivation meant): no constant of it is added unless every
        // name is free.
        return oxilean_kernel::add_derived_family(env, group).map_err(|e| match e {
            // The group's own names are not free: the environment's message.
            KernelError::Other(msg) => SeamFailure::Reason(msg),
            other => SeamFailure::Kernel(other),
        });
    }
    // Fully check + derive the family against the current environment WITHOUT
    // modifying it. A bundle that fails here installs nothing.
    let derived = match oxilean_kernel::check_and_derive_family(env, &lparams, num_params, &specs) {
        Ok(derived) => derived,
        Err(KernelError::UnsupportedNestedInductive(_)) => {
            // The file says the family is not nested and the kernel finds
            // a nested occurrence. If the family verifies as a nested one,
            // the file's count is not what the kernel computes; if not,
            // there is no verdict (the named unsupported feature).
            let nested =
                oxilean_kernel::verify_nested_bundle(env, &lparams, num_params, &specs, &b.recs);
            return Err(match nested {
                Ok(family) => differs(
                    &first.common.name,
                    "numNested",
                    &first.num_nested,
                    &derived_num_nested(&family),
                ),
                Err(_) => SeamFailure::Unsupported(NESTED_INDUCTIVES),
            });
        }
        Err(e) => return Err(SeamFailure::Kernel(e)),
    };
    // What the file says beyond the declaration and the kernel does not
    // compare when it checks the members one by one: before anything is added.
    compare_with_derivation(b, &derived, false)?;
    // The EXPORTED declarations, in dependency order, each re-verified by the
    // kernel (wrong num_indices, cidx, num_fields, a constructor its type does
    // not list, a recursor that is not the derived one, ... are caught here),
    // as ONE group: a failure at any member takes the earlier ones out again.
    // For a recursor record the kernel adds its own derivation, not the record.
    //
    // One field is NOT exported: lean4export does not emit `isProp` (the
    // reader defaults it to `false`), so it is taken from the kernel's own
    // derivation instead of the file — the kernel would otherwise correctly
    // reject every Prop-valued inductive for carrying a wrong flag.
    let mut group = Vec::with_capacity(b.types.len() + b.ctors.len() + b.recs.len());
    for (i, t) in b.types.iter().enumerate() {
        let mut iv = t.clone();
        if let Some(ConstantInfo::Inductive(d)) = derived.inductives.get(i) {
            iv.is_prop = d.is_prop;
        }
        group.push(ConstantInfo::Inductive(iv));
    }
    group.extend(b.ctors.iter().cloned().map(ConstantInfo::Constructor));
    group.extend(b.recs.iter().cloned().map(ConstantInfo::Recursor));
    check_constant_infos_atomic(env, group).map_err(map_inductive_kernel_error)
}

/// The rejection of a bundle whose file says `said` for a field the kernel
/// derives as `derived`.
fn differs(
    name: &Name,
    field: &str,
    said: &dyn std::fmt::Display,
    derived: &dyn std::fmt::Display,
) -> SeamFailure {
    SeamFailure::Reason(format!(
        "'{name}': the file says {field} = {said} and the kernel derives {derived} from the \
         declaration"
    ))
}

/// Every recursor record of a bundle is named as Lean's kernel names a
/// recursor of the declaration: `T.rec` — the component `rec` under the
/// name of a type `T` of the bundle — or `<first type>.rec_<k>` (`k` ≥ 1),
/// the name of the recursor of the `k`-th auxiliary type of a nested
/// declaration. A record under any other name is refused, with the name
/// expected. (Whether the declaration has a `k`-th auxiliary type is the
/// derivation's to say, as is the file's `numNested`: a record of that
/// shape is paired, or refused, by the derivation.)
///
/// The name is not a label. The kernel derives the family's recursors under
/// the names of the records it is handed, and in a mutual or nested family
/// a derived rule calls the sibling recursors by name; Lean's kernel never
/// gives a recursor the one flat component `"T.rec"` (which prints as
/// `T.rec` too), and a bundle whose records mixed the two installed rules
/// that called a recursor of the other spelling, never installed — a later
/// record could then define it and prove `False` through iota. The kernel's
/// guard refuses such a group as well (`oxilean_kernel::inductive::installed`);
/// this is the rule that keeps the replay to Lean's names in the first
/// place. (Which recursor a record is paired with, and that every derived
/// one has its record, is decided later, by the derivation.)
fn check_recursor_record_names(b: &InductiveBundle) -> Result<(), SeamFailure> {
    let Some(first) = b.types.first() else {
        return Err(SeamFailure::Reason("empty inductive bundle".into()));
    };
    let of_a_type = |name: &Name| {
        b.types
            .iter()
            .any(|t| *name == Name::mk_str(t.common.name.clone(), "rec"))
    };
    let auxiliary = |name: &Name| match name.view() {
        NameView::Str(parent, s) if *parent == first.common.name => {
            s.strip_prefix("rec_").is_some_and(|k| {
                !k.is_empty() && !k.starts_with('0') && k.bytes().all(|d| d.is_ascii_digit())
            })
        }
        _ => false,
    };
    for r in &b.recs {
        let name = &r.common.name;
        if of_a_type(name) || auxiliary(name) {
            continue;
        }
        let spelled = escaped_name(name);
        // The same spelling under another structure — `«T.rec»` for
        // `T.rec` — names the one expected; otherwise list them.
        let expected = match b
            .types
            .iter()
            .find(|t| format!("{}.rec", t.common.name) == name.to_string())
        {
            Some(t) => format!(
                "the recursor of '{}' is '{}', the component 'rec' under '{}'",
                t.common.name,
                escaped_name(&Name::mk_str(t.common.name.clone(), "rec")),
                t.common.name
            ),
            None => {
                let names: Vec<String> = b
                    .types
                    .iter()
                    .map(|t| escaped_name(&Name::mk_str(t.common.name.clone(), "rec")))
                    .collect();
                format!(
                    "expected {} (or {}.rec_<k> for the k-th auxiliary type of a nested \
                     declaration)",
                    names.join(" or "),
                    escaped_name(&first.common.name)
                )
            }
        };
        return Err(SeamFailure::Reason(format!(
            "recursor record '{spelled}' of inductive bundle '{}' is not named as Lean's kernel \
             names a recursor of this declaration: {expected}",
            first.common.name
        )));
    }
    Ok(())
}

/// A list of names as it is written: `[A, A.B]`.
struct Names<'a>(&'a [Name]);

impl std::fmt::Display for Names<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, name) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{name}")?;
        }
        f.write_str("]")
    }
}

/// The number of nested occurrences the kernel found for a derived family.
fn derived_num_nested(derived: &DerivedFamily) -> u32 {
    derived
        .inductives
        .iter()
        .find_map(|ci| ci.to_inductive_val().map(|iv| iv.num_nested))
        .unwrap_or(0)
}

/// A kernel-derived constant of an inductive family with the family's
/// declared `isUnsafe` flag (the derivation itself produces safe constants).
fn with_safety_flag(mut ci: ConstantInfo, is_unsafe: bool) -> ConstantInfo {
    match &mut ci {
        ConstantInfo::Inductive(v) => v.is_unsafe = is_unsafe,
        ConstantInfo::Constructor(v) => v.is_unsafe = is_unsafe,
        ConstantInfo::Recursor(v) => v.is_unsafe = is_unsafe,
        _ => {}
    }
    ci
}

/// Hold what a bundle says beyond its declaration against the kernel's
/// derivation of the family; the first difference is a rejection.
///
/// On both paths:
///
/// * one `isUnsafe` flag for the whole bundle (a Lean declaration has one);
/// * for every type: `all` (the family, in the bundle's order), `numNested`,
///   `isRec` and `isReflexive` — the two flags are Lean's, computed for the
///   family as a whole from the constructor types as written (see
///   `oxilean_kernel::inductive`'s derivation). `isRec` is what
///   `is_structure_like` reads, and with it structure eta, unit-like
///   equality and eta inside iota: a recursive type that claims
///   `isRec: false` would get all three.
///
/// On the ordinary path the kernel then checks the exported members one by
/// one and compares the rest itself (`numIndices` against the type's
/// telescope; a constructor's `induct`, `cidx`, `numParams`, `numFields` and
/// universe parameters against its type and the inductive's list; each
/// recursor against the derived recursor of ITS OWN NAME, rule by rule in
/// the constructors' order — and it is the derived recursor the kernel then
/// adds, not the record), and one more thing is asked here: one recursor
/// record per type. On the nested path (`full`) nothing of the file is
/// handed to the kernel member by member — the derived family is what gets
/// installed, after the kernel has held the declared types, the declared
/// constructor types and the recursor records against it
/// (`verify_nested_bundle`) — so those fields are compared here as well, and
/// a constructor record that no type of the bundle lists is refused.
///
/// **How a term of a recursor record is compared with the derived term**
/// (the recursor's type, a rule's right-hand side; the kernel does it,
/// `oxilean_kernel::inductive::supplied`): as written, up to the names and
/// the binder information of binders; failing that, the record's term must
/// be closed and well typed — a type must be a type, a right-hand side must
/// have the type of the derived one — and only then is it compared by
/// definitional equality. Either way the record's term is dropped: the
/// derived one is what is installed.
///
/// **How a record is paired with a derived item.** By its name, on both
/// paths, and one to one:
///
/// * a type record is the family member of its name — the family handed to
///   the derivation IS the type records, in their order, and a name that
///   occurs twice is refused by the derivation;
/// * a constructor record is found by its name in the `ctors` list of a
///   type (the derivation takes each listed constructor's type from the
///   record of that name; a listed name without a record, a record no list
///   names, a second record of a name are each refused), and its `cidx` is
///   its place in the list of the type it names as `induct`;
/// * a recursor record is the recursor Lean's kernel gives that name:
///   `T.rec` for a type `T` of the declaration, and on the nested path
///   `<first type>.rec_<k>` for the `k`-th auxiliary type. A record under
///   any other name (`check_recursor_record_names`, before anything is
///   derived — the one flat component `"T.rec"` among them), a second
///   record of a name (the kernel refuses a second constant of a name) and
///   a missing record (the count) are refused, so every derived recursor is
///   matched by exactly one record and every record matches exactly one.
///   The derived rules call the sibling recursors by these names, and the
///   kernel's guard holds them to the recursors the bundle installs;
/// * rule `i` of a recursor record is the rule of the type's `i`-th
///   constructor.
#[allow(clippy::result_large_err)] // KernelError is large; replay is not a hot error path
fn compare_with_derivation(
    b: &InductiveBundle,
    derived: &DerivedFamily,
    full: bool,
) -> Result<(), SeamFailure> {
    let internal = |what: &str| SeamFailure::Reason(format!("internal: derivation without {what}"));
    let Some(first) = b.types.first() else {
        return Err(SeamFailure::Reason("empty inductive bundle".into()));
    };
    let is_unsafe = first.is_unsafe;
    let mixed = b.types.iter().any(|t| t.is_unsafe != is_unsafe)
        || b.ctors.iter().any(|c| c.is_unsafe != is_unsafe)
        || b.recs.iter().any(|r| r.is_unsafe != is_unsafe);
    if mixed {
        return Err(SeamFailure::Reason(format!(
            "inductive bundle '{}' mixes safe and unsafe records: a declaration has one isUnsafe \
             flag",
            first.common.name
        )));
    }
    if derived.inductives.len() != b.types.len() {
        return Err(internal("one inductive per type"));
    }
    for (t, d) in b.types.iter().zip(&derived.inductives) {
        let d = d
            .to_inductive_val()
            .ok_or_else(|| internal("an inductive"))?;
        let name = &t.common.name;
        if t.all != d.all {
            return Err(differs(name, "all", &Names(&t.all), &Names(&d.all)));
        }
        if t.num_nested != d.num_nested {
            return Err(differs(name, "numNested", &t.num_nested, &d.num_nested));
        }
        if t.is_rec != d.is_rec {
            return Err(differs(name, "isRec", &t.is_rec, &d.is_rec));
        }
        if t.is_reflexive != d.is_reflexive {
            return Err(differs(
                name,
                "isReflexive",
                &t.is_reflexive,
                &d.is_reflexive,
            ));
        }
        if full && t.num_indices != d.num_indices {
            return Err(differs(name, "numIndices", &t.num_indices, &d.num_indices));
        }
    }
    if !full {
        if b.recs.len() != b.types.len() {
            return Err(SeamFailure::Reason(format!(
                "inductive bundle '{}' has {} recursor record(s) for {} type(s): the kernel \
                 derives one recursor per type",
                first.common.name,
                b.recs.len(),
                b.types.len()
            )));
        }
        return Ok(());
    }
    if derived.constructors.len() != b.ctors.len() {
        return Err(SeamFailure::Reason(format!(
            "inductive bundle '{}' has {} constructor record(s) and its types list {}",
            first.common.name,
            b.ctors.len(),
            derived.constructors.len()
        )));
    }
    for c in &b.ctors {
        let name = &c.common.name;
        let d = derived
            .constructors
            .iter()
            .filter_map(ConstantInfo::to_constructor_val)
            .find(|d| &d.common.name == name)
            .ok_or_else(|| {
                SeamFailure::Reason(format!(
                    "constructor record '{name}' is not listed by a type of its bundle"
                ))
            })?;
        if c.induct != d.induct {
            return Err(differs(name, "induct", &c.induct, &d.induct));
        }
        if c.common.level_params != d.common.level_params {
            return Err(differs(
                name,
                "levelParams",
                &Names(&c.common.level_params),
                &Names(&d.common.level_params),
            ));
        }
        if c.cidx != d.cidx {
            return Err(differs(name, "cidx", &c.cidx, &d.cidx));
        }
        if c.num_params != d.num_params {
            return Err(differs(name, "numParams", &c.num_params, &d.num_params));
        }
        if c.num_fields != d.num_fields {
            return Err(differs(name, "numFields", &c.num_fields, &d.num_fields));
        }
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
    /// units of kernel work (see [`Replayer::set_per_decl_fuel`] and
    /// [`RESOURCE_LIMIT`]). `None` = unlimited.
    pub per_decl_fuel: Option<u64>,
}

/// The default per-declaration budget for untrusted input
/// (`ReplayLimits::default()`): 2^24 ≈ 16.8M units of kernel work (see
/// `oxilean_kernel::fuel`) — about half a second on a 2023 laptop core.
///
/// A unit is a subterm one of the kernel's term builders reaches; every
/// node the kernel allocates is charged one, so a declaration can have at
/// most this many nodes of its own live at once (72 bytes each: 1.2 GiB,
/// and what the caches' tables add), and in practice a small part of that —
/// most units are subterms shared, not allocated.
///
/// Calibration (2026-11-11): the heaviest declaration of Lean core `Init`,
/// the `omega` certificate `Array.extract_append._proof_1_1`, takes 6.7M
/// units, and no declaration of `Init` or of the Mathlib slice takes more
/// than half this budget.
pub const DEFAULT_DECL_FUEL: u64 = 1 << 24;

/// The per-declaration budget of the corpus preset
/// ([`ReplayLimits::corpus`]): 2^26 ≈ 67M units of kernel work — about two
/// seconds on a 2023 laptop core, ten times the heaviest declaration of
/// Lean core `Init` (see [`DEFAULT_DECL_FUEL`]). A declaration that runs
/// through all of it is cut off with at most 67M nodes of its own
/// (4.8 GiB at 72 bytes each); the ones observed doing so
/// (a `List.foldl` over 200,000 elements evaluated by the kernel) peaked
/// at 0.5 GiB.
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
            .is_none_or(|(_, best)| elapsed > *best);
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
///   the exported types and constructors and the recursors the kernel
///   **re-derives**; the exported recursors are compared with those and
///   never installed.
///
/// Both are *stateless* (no dependency cascade); [`Replayer`] applies the
/// cascade on top of the same logic.
pub mod wave3b {
    use super::{
        replay_inductive_core, replay_quot_core, Environment, ExportDecl, ReplayOutcome,
        SeamFailure,
    };

    /// Map a stateless seam result onto an outcome (no cascade: an
    /// `UnknownConstant` failure is a plain rejection here). A failure
    /// reached at a resource limit is a named unsupported verdict, as in
    /// the stateful replayer.
    fn stateless_outcome(res: Result<(), SeamFailure>) -> ReplayOutcome {
        let failure = match res {
            Ok(()) => return ReplayOutcome::Checked,
            Err(SeamFailure::Unsupported(feature)) => {
                return ReplayOutcome::Unsupported { feature }
            }
            Err(failure) => failure,
        };
        if let Some(feature) = super::resource_limit_feature() {
            return ReplayOutcome::Unsupported { feature };
        }
        match failure {
            SeamFailure::Unsupported(feature) => ReplayOutcome::Unsupported { feature },
            SeamFailure::Kernel(e) => ReplayOutcome::Rejected {
                reason: super::render_kernel_error(&e),
            },
            SeamFailure::Reason(reason) => ReplayOutcome::Rejected { reason },
        }
    }

    /// Replay a `quot` record: install the four quotient primitives via the
    /// kernel's `add_quot` on first sight, then validate the record against
    /// the installed canonical primitive. See the module docs. (Stateless:
    /// it keeps no memory of the records it has seen, so a second record
    /// for a primitive is validated like the first; [`super::Replayer`]
    /// rejects it.)
    #[must_use]
    pub fn replay_quot(env: &mut Environment, decl: &ExportDecl) -> ReplayOutcome {
        oxilean_kernel::fuel::clear_reduction_limit();
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
    /// exported types and constructors and the recursors the kernel
    /// re-derives (the exported ones are compared with them). See the module
    /// docs.
    #[must_use]
    pub fn replay_inductive(env: &mut Environment, decl: &ExportDecl) -> ReplayOutcome {
        oxilean_kernel::fuel::clear_reduction_limit();
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
