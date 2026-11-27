//! Comparator-style checks: does a verified export prove the statements of a
//! *challenge* export, from permitted axioms only?
//!
//! Checking every declaration of an export says that every declaration of the
//! export is well typed. It does not say *which* statements were proved, nor
//! from what. A theorem named `riemannHypothesis` proves the Riemann
//! Hypothesis only if its type is the statement one meant and every
//! definition that statement mentions is the one one meant; and a proof that
//! uses an axiom beyond Lean's three standard ones (`sorryAx`, say) proves
//! nothing. Lean's `lake comparator` answers both questions, and the
//! `--compare` modes of the export checkers `con-leche` and `con-ron` answer
//! the first. This module answers both, over the declarations the kernel
//! checked in the same run:
//!
//! * **The challenge.** A challenge export is the export of a file that states
//!   the target theorems with `sorry` for their proofs. Apart from the targets
//!   and `sorryAx` it contains exactly what the statements depend on —
//!   `riemannZeta`, `Complex`, `Real` and everything below them. Every one of
//!   those declarations must occur in the verified export, under the same
//!   name, structurally identical: same kind, same universe parameters, same
//!   type, same value, same inductive structure, binder names and binder
//!   annotations included (stricter than Lean's `==`, which ignores binder
//!   names). Each target must occur in the verified export as a *theorem*
//!   whose type and universe parameters are identical to the challenge's.
//! * **The axioms.** The dependency closure of the targets in the verified
//!   export — every constant their types and proofs mention, transitively,
//!   through types, values, constructors and recursor rules — is computed,
//!   and the axioms in it must be among the permitted ones (by default
//!   `propext`, `Classical.choice` and `Quot.sound`).
//! * **Nothing unchecked.** Every declaration of that closure must have been
//!   *verified* by the kernel in this run (not `unsupported`, not
//!   `rejected`), every constant it mentions must be declared in the export,
//!   and none may be `unsafe` or `partial`.
//!
//! The comparison is exact: terms are compared with
//! [`oxilean_kernel::terms_structurally_equal`], structural equality that
//! visits each pair of shared subterms once — never by hash alone.
//!
//! What this does not establish: that the statements say what a reader takes
//! them to say. That is a matter of reading the challenge's source.

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use oxilean_export::{read_streaming, ExportDecl, InductiveBundle, Limits, Meta, ReadStats};
use oxilean_kernel::{
    terms_structurally_equal, ConstantVal, ConstructorVal, DefinitionSafety, Expr, InductiveVal,
    Name, RecursorVal,
};

use crate::engine::{DeclEvent, Verdict, VerifyError};
use crate::jsonw::JsonWriter;

/// Lean's three standard axioms: what a proof may use unless told otherwise.
pub const DEFAULT_ALLOWED_AXIOMS: [&str; 3] = ["propext", "Classical.choice", "Quot.sound"];

/// The axiom Lean's `sorry` elaborates to. A challenge states its targets
/// with it; a solution must not mention it.
pub const SORRY_AXIOM: &str = "sorryAx";

/// At most this many names are listed per problem category in the report
/// (the counts are always exact).
pub const LIST_CAP: usize = 32;

/// The options of a comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareConfig {
    /// Path of the challenge export.
    pub challenge_path: String,
    /// The target theorems, by name. `None`: every theorem of the challenge
    /// whose proof mentions `sorryAx`.
    pub theorems: Option<Vec<String>>,
    /// The permitted axioms, by name.
    pub allowed_axioms: Vec<String>,
}

impl CompareConfig {
    /// A configuration with the default permitted axioms and targets found
    /// from the challenge.
    #[must_use]
    pub fn new(challenge_path: impl Into<String>) -> Self {
        Self {
            challenge_path: challenge_path.into(),
            theorems: None,
            allowed_axioms: DEFAULT_ALLOWED_AXIOMS
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        }
    }
}

/// A challenge export, read into memory and indexed by every name it
/// declares.
pub struct ChallengeFile {
    decls: Vec<ExportDecl>,
    by_name: HashMap<Name, usize>,
    targets: Vec<Name>,
    sorry: Name,
    /// The challenge's `meta` record.
    pub meta: Meta,
    /// The challenge's record statistics.
    pub stats: ReadStats,
}

impl ChallengeFile {
    /// Read a challenge export. `theorems` names the targets; `None` takes
    /// every theorem whose proof mentions `sorryAx`.
    ///
    /// # Errors
    /// A [`VerifyError`] when the file cannot be read to its end, when a
    /// named target is not a theorem or axiom of the challenge, when a name
    /// is declared twice, or when no target is found.
    pub fn load<R: BufRead>(
        reader: R,
        limits: Limits,
        theorems: Option<&[String]>,
    ) -> Result<Self, VerifyError> {
        let mut decls: Vec<ExportDecl> = Vec::new();
        let (meta, stats) = read_streaming(reader, limits, |d| {
            decls.push(d);
            Ok(())
        })
        .map_err(VerifyError::from_export)?;

        let mut by_name: HashMap<Name, usize> = HashMap::new();
        for (i, d) in decls.iter().enumerate() {
            for n in decl_names(d) {
                if by_name.insert(n.clone(), i).is_some() {
                    return Err(challenge_error(format!(
                        "the challenge declares '{n}' twice"
                    )));
                }
            }
        }

        let sorry = Name::from_str(SORRY_AXIOM);
        let targets: Vec<Name> = match theorems {
            Some(list) => {
                let mut out = Vec::with_capacity(list.len());
                for t in list {
                    let n = Name::from_str(t);
                    match by_name.get(&n).map(|&i| &decls[i]) {
                        Some(ExportDecl::Theorem(_)) | Some(ExportDecl::Axiom(_)) => out.push(n),
                        Some(other) => {
                            return Err(challenge_error(format!(
                                "target '{t}' is a {} of the challenge, not a theorem",
                                other.kind_label()
                            )))
                        }
                        None => {
                            return Err(challenge_error(format!(
                                "target '{t}' is not declared in the challenge"
                            )))
                        }
                    }
                }
                out
            }
            None => decls
                .iter()
                .filter_map(|d| match d {
                    ExportDecl::Theorem(t) => {
                        let mut consts = HashSet::new();
                        collect_consts(&[&t.value], &mut consts);
                        consts.contains(&sorry).then(|| t.common.name.clone())
                    }
                    _ => None,
                })
                .collect(),
        };
        if targets.is_empty() {
            return Err(challenge_error(
                "no target theorem: name them with --theorems, or give a challenge whose \
                 theorems are proved by sorry"
                    .to_string(),
            ));
        }
        Ok(Self {
            decls,
            by_name,
            targets,
            sorry,
            meta,
            stats,
        })
    }

    /// The target theorems, in the order given or found.
    #[must_use]
    pub fn targets(&self) -> &[Name] {
        &self.targets
    }

    /// The number of declaration records of the challenge.
    #[must_use]
    pub fn declarations(&self) -> usize {
        self.decls.len()
    }
}

fn challenge_error(message: String) -> VerifyError {
    VerifyError::Malformed {
        pos: oxilean_export::Position::line_only(0),
        message: format!("challenge: {message}"),
    }
}

/// What the kernel decided about a declaration of the verified export.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Checked {
    Verified,
    Unsupported,
    Rejected,
}

/// One declaration record of the verified export, as the audit keeps it.
struct AuditRecord {
    /// Interned primary name.
    primary: u32,
    /// The record's kind label.
    kind: &'static str,
    /// Interned names of every constant the record's terms mention.
    deps: Vec<u32>,
    checked: Checked,
    /// `unsafe` or `partial`.
    unsafe_or_partial: bool,
    is_axiom: bool,
}

/// How a target fared, as observed while the verified export streamed by.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetSeen {
    /// Declared as a theorem with the challenge's statement.
    Matches,
    /// Declared, but not as a theorem with that statement.
    Differs(String),
}

/// Collects, while the verified export streams through the kernel, what the
/// comparison and the axiom audit need: the dependency graph of the export,
/// the verdicts, and the outcome of comparing each declaration that the
/// challenge also declares.
pub struct Auditor {
    challenge: ChallengeFile,
    allowed_axioms: Vec<Name>,
    names: HashMap<Name, u32>,
    name_list: Vec<Name>,
    /// By interned name: the record declaring it (`u32::MAX`: none yet).
    owner: Vec<u32>,
    records: Vec<AuditRecord>,
    /// Per challenge declaration: found identical in the verified export.
    matched: Vec<bool>,
    /// Challenge declarations found under their name but not identical.
    differ: Vec<(String, String)>,
    /// Names declared more than once in the verified export.
    duplicates: Vec<String>,
    targets_seen: HashMap<Name, TargetSeen>,
    /// Total number of axiom records in the verified export.
    axioms_in_file: Vec<String>,
}

const NO_OWNER: u32 = u32::MAX;

impl Auditor {
    /// Start an audit against `challenge`, permitting `allowed_axioms`.
    #[must_use]
    pub fn new(challenge: ChallengeFile, allowed_axioms: &[String]) -> Self {
        let matched = vec![false; challenge.decls.len()];
        Self {
            challenge,
            allowed_axioms: allowed_axioms.iter().map(|a| Name::from_str(a)).collect(),
            names: HashMap::new(),
            name_list: Vec::new(),
            owner: Vec::new(),
            records: Vec::new(),
            matched,
            differ: Vec::new(),
            duplicates: Vec::new(),
            targets_seen: HashMap::new(),
            axioms_in_file: Vec::new(),
        }
    }

    fn intern(&mut self, n: &Name) -> u32 {
        if let Some(&id) = self.names.get(n) {
            return id;
        }
        let id = self.name_list.len() as u32;
        self.names.insert(n.clone(), id);
        self.name_list.push(n.clone());
        self.owner.push(NO_OWNER);
        id
    }

    /// Record one declaration of the verified export, with its verdict.
    pub fn observe(&mut self, decl: &ExportDecl, event: &DeclEvent) {
        let record_index = self.records.len() as u32;
        let names = decl_names(decl);
        let mut primary = NO_OWNER;
        for (k, n) in names.iter().enumerate() {
            let id = self.intern(n);
            if k == 0 {
                primary = id;
            }
            if self.owner[id as usize] != NO_OWNER {
                if self.duplicates.len() < LIST_CAP {
                    self.duplicates.push(n.to_string());
                }
            } else {
                self.owner[id as usize] = record_index;
            }
        }

        let mut consts = HashSet::new();
        collect_consts(&decl_exprs(decl), &mut consts);
        let mut deps: Vec<u32> = consts.iter().map(|c| self.intern(c)).collect();
        deps.sort_unstable();
        deps.dedup();

        let checked = match event.verdict {
            Verdict::Verified { .. } => Checked::Verified,
            Verdict::Unsupported { .. } => Checked::Unsupported,
            Verdict::Rejected { .. } => Checked::Rejected,
        };
        let is_axiom = matches!(decl, ExportDecl::Axiom(_));
        if is_axiom {
            self.axioms_in_file.push(decl.primary_name().to_string());
        }
        self.records.push(AuditRecord {
            primary,
            kind: decl.kind_label(),
            deps,
            checked,
            unsafe_or_partial: is_unsafe_or_partial(decl),
            is_axiom,
        });

        self.compare_with_challenge(decl, &names);
    }

    fn compare_with_challenge(&mut self, decl: &ExportDecl, names: &[Name]) {
        // The challenge declaration that declares any name of this record.
        let Some((name, ci)) = names
            .iter()
            .find_map(|n| self.challenge.by_name.get(n).map(|&i| (n.clone(), i)))
        else {
            return;
        };
        let chal = &self.challenge.decls[ci];
        if self.challenge.targets.contains(&name) {
            let seen = match same_statement(chal, decl) {
                Ok(()) => TargetSeen::Matches,
                Err(why) => TargetSeen::Differs(why),
            };
            self.targets_seen.insert(name, seen);
            self.matched[ci] = true;
            return;
        }
        let outcome = same_decl(chal, decl).and_then(|()| {
            // Every name of this record must belong to the same challenge
            // record (the two files group declarations alike).
            match names
                .iter()
                .find(|n| self.challenge.by_name.get(*n).is_some_and(|&j| j != ci))
            {
                Some(n) => Err(format!(
                    "'{n}' is grouped with '{name}' here but not in the challenge"
                )),
                None => Ok(()),
            }
        });
        match outcome {
            Ok(()) => self.matched[ci] = true,
            Err(why) => self.differ.push((name.to_string(), why)),
        }
    }

    /// Finish the audit: compute the closure of the targets and decide.
    #[must_use]
    pub fn finish(self) -> CompareOutcome {
        let mut targets = Vec::with_capacity(self.challenge.targets.len());
        let mut roots: Vec<u32> = Vec::new();
        for t in &self.challenge.targets {
            let declared_as = self
                .names
                .get(t)
                .map(|&id| self.owner[id as usize])
                .filter(|&r| r != NO_OWNER)
                .map(|r| {
                    roots.push(r);
                    &self.records[r as usize]
                });
            let (status, verified) = match (declared_as, self.targets_seen.get(t)) {
                (None, _) => (TargetStatus::Missing, false),
                (Some(rec), Some(TargetSeen::Matches)) => {
                    (TargetStatus::Matches, rec.checked == Checked::Verified)
                }
                (Some(rec), Some(TargetSeen::Differs(why))) => (
                    TargetStatus::Differs(why.clone()),
                    rec.checked == Checked::Verified,
                ),
                (Some(rec), None) => (
                    TargetStatus::Differs("not compared".to_string()),
                    rec.checked == Checked::Verified,
                ),
            };
            targets.push(TargetResult {
                name: t.to_string(),
                status,
                verified,
            });
        }

        // The dependency closure of the targets.
        let mut in_closure = vec![false; self.records.len()];
        let mut stack: Vec<u32> = Vec::new();
        for &r in &roots {
            if !in_closure[r as usize] {
                in_closure[r as usize] = true;
                stack.push(r);
            }
        }
        let mut undeclared: Vec<String> = Vec::new();
        let mut undeclared_count = 0usize;
        let mut seen_undeclared: HashSet<u32> = HashSet::new();
        while let Some(r) = stack.pop() {
            for &d in &self.records[r as usize].deps {
                let o = self.owner[d as usize];
                if o == NO_OWNER {
                    if seen_undeclared.insert(d) {
                        undeclared_count += 1;
                        if undeclared.len() < LIST_CAP {
                            undeclared.push(self.name_list[d as usize].to_string());
                        }
                    }
                    continue;
                }
                if !in_closure[o as usize] {
                    in_closure[o as usize] = true;
                    stack.push(o);
                }
            }
        }

        let mut closure_size = 0usize;
        let mut axioms_used: Vec<String> = Vec::new();
        let mut forbidden_axioms: Vec<String> = Vec::new();
        let mut not_verified = NameList::default();
        let mut unsafe_in_closure = NameList::default();
        for (r, rec) in self.records.iter().enumerate() {
            if !in_closure[r] {
                continue;
            }
            closure_size += 1;
            let name = &self.name_list[rec.primary as usize];
            if rec.is_axiom {
                axioms_used.push(name.to_string());
                if !self.allowed_axioms.contains(name) {
                    forbidden_axioms.push(name.to_string());
                }
            }
            if rec.checked != Checked::Verified {
                not_verified.push(format!("{name} ({})", rec.kind));
            }
            if rec.unsafe_or_partial {
                unsafe_in_closure.push(name.to_string());
            }
        }
        axioms_used.sort();
        forbidden_axioms.sort();

        // Challenge declarations not found identical in the verified export.
        let mut missing = NameList::default();
        let differ_names: HashSet<&str> = self.differ.iter().map(|(n, _)| n.as_str()).collect();
        for (i, d) in self.challenge.decls.iter().enumerate() {
            if self.matched[i] {
                continue;
            }
            let name = d.primary_name();
            if name == self.challenge.sorry {
                continue;
            }
            let shown = name.to_string();
            if differ_names.contains(shown.as_str()) {
                continue;
            }
            missing.push(shown);
        }
        let challenge_compared = self.matched.iter().filter(|m| **m).count();

        CompareOutcome {
            challenge_declarations: self.challenge.decls.len(),
            challenge_compared,
            challenge_sorry_present: self.challenge.by_name.contains_key(&self.challenge.sorry),
            targets,
            differ_count: self.differ.len(),
            differ: self.differ.into_iter().take(LIST_CAP).collect(),
            missing,
            duplicates: self.duplicates,
            closure_size,
            axioms_used,
            forbidden_axioms,
            allowed_axioms: self.allowed_axioms.iter().map(|n| n.to_string()).collect(),
            axioms_in_file: self.axioms_in_file,
            not_verified,
            unsafe_in_closure,
            undeclared_count,
            undeclared,
        }
    }
}

/// A list of names with an exact count and at most [`LIST_CAP`] entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameList {
    /// Exact number of entries.
    pub count: usize,
    /// The first [`LIST_CAP`] entries.
    pub names: Vec<String>,
}

impl NameList {
    fn push(&mut self, s: String) {
        self.count += 1;
        if self.names.len() < LIST_CAP {
            self.names.push(s);
        }
    }
}

/// How a target theorem fared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetStatus {
    /// A theorem of the verified export, with the challenge's statement.
    Matches,
    /// Declared in the verified export, but not as a theorem with the
    /// challenge's statement (the reason says how).
    Differs(String),
    /// Not declared in the verified export.
    Missing,
}

/// One target theorem's result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetResult {
    /// The target's name.
    pub name: String,
    /// Statement comparison.
    pub status: TargetStatus,
    /// Whether the kernel verified the target's declaration in this run.
    pub verified: bool,
}

/// The outcome of a comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareOutcome {
    /// Declaration records of the challenge.
    pub challenge_declarations: usize,
    /// Challenge records found identical (targets: same statement).
    pub challenge_compared: usize,
    /// Whether the challenge declares `sorryAx` (it normally does).
    pub challenge_sorry_present: bool,
    /// The targets.
    pub targets: Vec<TargetResult>,
    /// Exact number of challenge declarations that differ in the export.
    pub differ_count: usize,
    /// The first [`LIST_CAP`] of them, with the reason.
    pub differ: Vec<(String, String)>,
    /// Challenge declarations the verified export does not declare.
    pub missing: NameList,
    /// Names the verified export declares twice (first [`LIST_CAP`]).
    pub duplicates: Vec<String>,
    /// Declaration records in the dependency closure of the targets.
    pub closure_size: usize,
    /// Axioms in that closure.
    pub axioms_used: Vec<String>,
    /// Axioms in that closure that are not permitted.
    pub forbidden_axioms: Vec<String>,
    /// The permitted axioms.
    pub allowed_axioms: Vec<String>,
    /// Every axiom record of the verified export, closure or not.
    pub axioms_in_file: Vec<String>,
    /// Closure records the kernel did not verify.
    pub not_verified: NameList,
    /// Closure records that are `unsafe` or `partial`.
    pub unsafe_in_closure: NameList,
    /// Exact number of constants mentioned in the closure but declared
    /// nowhere in the export.
    pub undeclared_count: usize,
    /// The first [`LIST_CAP`] of them.
    pub undeclared: Vec<String>,
}

impl CompareOutcome {
    /// Whether the verified export proves every target's challenge statement
    /// from the permitted axioms, with everything it relies on verified.
    #[must_use]
    pub fn established(&self) -> bool {
        self.targets
            .iter()
            .all(|t| t.status == TargetStatus::Matches && t.verified)
            && self.differ_count == 0
            && self.missing.count == 0
            && self.duplicates.is_empty()
            && self.forbidden_axioms.is_empty()
            && self.not_verified.count == 0
            && self.unsafe_in_closure.count == 0
            && self.undeclared_count == 0
    }

    /// Render the human-readable comparison section.
    #[must_use]
    pub fn render(&self, challenge_path: &str, challenge_sha256: &str) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "compare: challenge {challenge_path}\n         sha256 {challenge_sha256}, \
             {} declaration records\n",
            self.challenge_declarations
        ));
        for t in &self.targets {
            let (mark, what) = match &t.status {
                TargetStatus::Matches => ("✓", "statement identical to the challenge".to_string()),
                TargetStatus::Differs(why) => ("✗", format!("statement differs: {why}")),
                TargetStatus::Missing => ("✗", "not declared in the solution".to_string()),
            };
            let checked = if t.verified {
                "proof verified"
            } else {
                "proof NOT verified"
            };
            s.push_str(&format!(
                "  {mark}  target {:<40} {what}; {checked}\n",
                t.name
            ));
        }
        s.push_str(&format!(
            "  statement closure: {} of {} challenge records identical in the solution \
             ({} differ, {} missing{})\n",
            self.challenge_compared,
            self.challenge_declarations,
            self.differ_count,
            self.missing.count,
            if self.challenge_sorry_present {
                "; sorryAx excluded"
            } else {
                ""
            }
        ));
        for (n, why) in &self.differ {
            s.push_str(&format!("       differs: {n}: {why}\n"));
        }
        for n in &self.missing.names {
            s.push_str(&format!("       missing: {n}\n"));
        }
        for n in &self.duplicates {
            s.push_str(&format!("       declared twice in the solution: {n}\n"));
        }
        s.push_str(&format!(
            "  target closure: {} declaration records; {} not verified; {} unsafe/partial; \
             {} constants undeclared\n",
            self.closure_size,
            self.not_verified.count,
            self.unsafe_in_closure.count,
            self.undeclared_count
        ));
        for n in &self.not_verified.names {
            s.push_str(&format!("       not verified: {n}\n"));
        }
        for n in &self.unsafe_in_closure.names {
            s.push_str(&format!("       unsafe/partial: {n}\n"));
        }
        for n in &self.undeclared {
            s.push_str(&format!("       undeclared: {n}\n"));
        }
        s.push_str(&format!(
            "  axioms used by the targets: {} (permitted: {})\n",
            list_or_none(&self.axioms_used),
            list_or_none(&self.allowed_axioms)
        ));
        s.push_str(&format!(
            "  axiom records in the solution file: {}\n",
            list_or_none(&self.axioms_in_file)
        ));
        if !self.forbidden_axioms.is_empty() {
            s.push_str(&format!(
                "       NOT PERMITTED: {}\n",
                self.forbidden_axioms.join(", ")
            ));
        }
        if self.established() {
            s.push_str(&format!(
                "compare verdict: ESTABLISHED — the solution proves the {} challenge \
                 statement(s) from the permitted axioms\n",
                self.targets.len()
            ));
        } else {
            s.push_str("compare verdict: NOT ESTABLISHED — see the lines marked above\n");
        }
        s
    }

    /// Write the comparison as a JSON object into `w`.
    pub fn write_json(&self, w: &mut JsonWriter, challenge_path: &str, challenge_sha256: &str) {
        let mut o = w.object();
        o.str_field("challenge_path", challenge_path);
        o.str_field("challenge_sha256", challenge_sha256);
        o.uint_field("challenge_declarations", self.challenge_declarations as u64);
        o.uint_field("challenge_identical", self.challenge_compared as u64);
        o.bool_field("established", self.established());
        {
            let mut arr = o.key("targets").array();
            for t in &self.targets {
                let mut e = arr.element().object();
                e.str_field("name", &t.name);
                match &t.status {
                    TargetStatus::Matches => e.str_field("statement", "identical"),
                    TargetStatus::Differs(why) => {
                        e.str_field("statement", "differs");
                        e.str_field("reason", why);
                    }
                    TargetStatus::Missing => e.str_field("statement", "missing"),
                }
                e.bool_field("verified", t.verified);
            }
        }
        o.uint_field("differ_count", self.differ_count as u64);
        {
            let mut arr = o.key("differ").array();
            for (n, why) in &self.differ {
                let mut e = arr.element().object();
                e.str_field("name", n);
                e.str_field("reason", why);
            }
        }
        write_name_list(&mut o, "missing", &self.missing);
        write_strings(&mut o, "duplicates", &self.duplicates);
        o.uint_field("closure_size", self.closure_size as u64);
        write_strings(&mut o, "axioms_used", &self.axioms_used);
        write_strings(&mut o, "forbidden_axioms", &self.forbidden_axioms);
        write_strings(&mut o, "allowed_axioms", &self.allowed_axioms);
        write_strings(&mut o, "axioms_in_file", &self.axioms_in_file);
        write_name_list(&mut o, "not_verified", &self.not_verified);
        write_name_list(&mut o, "unsafe_in_closure", &self.unsafe_in_closure);
        o.uint_field("undeclared_count", self.undeclared_count as u64);
        write_strings(&mut o, "undeclared", &self.undeclared);
    }
}

fn write_strings(o: &mut crate::jsonw::ObjectScope<'_>, key: &str, items: &[String]) {
    let mut arr = o.key(key).array();
    for s in items {
        arr.element().string_value(s);
    }
}

fn write_name_list(o: &mut crate::jsonw::ObjectScope<'_>, key: &str, list: &NameList) {
    let mut obj = o.key(key).object();
    obj.uint_field("count", list.count as u64);
    let mut arr = obj.key("names").array();
    for s in &list.names {
        arr.element().string_value(s);
    }
}

fn list_or_none(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

// ---------------------------------------------------------------------------
// Declarations: their names, their terms, and exact comparison.
// ---------------------------------------------------------------------------

/// Every name a declaration record introduces.
#[must_use]
pub fn decl_names(d: &ExportDecl) -> Vec<Name> {
    match d {
        ExportDecl::Axiom(v) => vec![v.common.name.clone()],
        ExportDecl::Definition(v) => vec![v.common.name.clone()],
        ExportDecl::Theorem(v) => vec![v.common.name.clone()],
        ExportDecl::Opaque(v) => vec![v.common.name.clone()],
        ExportDecl::Quot(v) => vec![v.common.name.clone()],
        ExportDecl::Inductive(b) => b
            .types
            .iter()
            .map(|t| t.common.name.clone())
            .chain(b.ctors.iter().map(|c| c.common.name.clone()))
            .chain(b.recs.iter().map(|r| r.common.name.clone()))
            .collect(),
        ExportDecl::Oversized(v) => v.names.clone(),
    }
}

/// Every term of a declaration record: types, values, constructor types,
/// recursor types and recursor rules.
fn decl_exprs(d: &ExportDecl) -> Vec<&Expr> {
    match d {
        ExportDecl::Axiom(v) => vec![&v.common.ty],
        ExportDecl::Definition(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Theorem(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Opaque(v) => vec![&v.common.ty, &v.value],
        ExportDecl::Quot(v) => vec![&v.common.ty],
        ExportDecl::Inductive(b) => {
            let mut out: Vec<&Expr> = Vec::new();
            out.extend(b.types.iter().map(|t| &t.common.ty));
            out.extend(b.ctors.iter().map(|c| &c.common.ty));
            for r in &b.recs {
                out.push(&r.common.ty);
                out.extend(r.rules.iter().map(|rule| &rule.rhs));
            }
            out
        }
        ExportDecl::Oversized(_) => Vec::new(),
    }
}

fn is_unsafe_or_partial(d: &ExportDecl) -> bool {
    match d {
        ExportDecl::Axiom(v) => v.is_unsafe,
        ExportDecl::Definition(v) => v.safety != DefinitionSafety::Safe,
        ExportDecl::Theorem(_) | ExportDecl::Quot(_) | ExportDecl::Oversized(_) => false,
        ExportDecl::Opaque(v) => v.is_unsafe,
        ExportDecl::Inductive(b) => {
            b.types.iter().any(|t| t.is_unsafe)
                || b.ctors.iter().any(|c| c.is_unsafe)
                || b.recs.iter().any(|r| r.is_unsafe)
        }
    }
}

/// Insert into `out` every constant the terms mention (including the
/// structure named by a projection), visiting each shared subterm once.
fn collect_consts(roots: &[&Expr], out: &mut HashSet<Name>) {
    let mut seen: HashSet<*const Expr> = HashSet::new();
    let mut stack: Vec<&Expr> = roots.to_vec();
    while let Some(e) = stack.pop() {
        let mut visit = |n: &oxilean_kernel::Node| {
            if seen.insert(n.as_ptr()) {
                Some(())
            } else {
                None
            }
        };
        match e {
            Expr::Const(n, _) => {
                out.insert(n.clone());
            }
            Expr::App(f, a) => {
                if visit(f).is_some() {
                    stack.push(f);
                }
                if visit(a).is_some() {
                    stack.push(a);
                }
            }
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                if visit(ty).is_some() {
                    stack.push(ty);
                }
                if visit(body).is_some() {
                    stack.push(body);
                }
            }
            Expr::Let(_, ty, val, body) => {
                if visit(ty).is_some() {
                    stack.push(ty);
                }
                if visit(val).is_some() {
                    stack.push(val);
                }
                if visit(body).is_some() {
                    stack.push(body);
                }
            }
            Expr::Proj(s, _, inner) => {
                out.insert(s.clone());
                if visit(inner).is_some() {
                    stack.push(inner);
                }
            }
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => {}
        }
    }
}

fn same_term(what: &str, a: &Expr, b: &Expr) -> Result<(), String> {
    if terms_structurally_equal(a, b) {
        Ok(())
    } else {
        Err(format!("{what} differs"))
    }
}

fn same_field<T: PartialEq>(what: &str, a: &T, b: &T) -> Result<(), String> {
    if a == b {
        Ok(())
    } else {
        Err(format!("{what} differs"))
    }
}

fn same_const(a: &ConstantVal, b: &ConstantVal) -> Result<(), String> {
    same_field("name", &a.name, &b.name)?;
    same_field("universe parameters", &a.level_params, &b.level_params)?;
    same_term("type", &a.ty, &b.ty)
}

/// A target's statement: the challenge may state it as a theorem (proved by
/// `sorry`) or as an axiom; the solution must prove it as a theorem.
fn same_statement(chal: &ExportDecl, sol: &ExportDecl) -> Result<(), String> {
    let chal_common = match chal {
        ExportDecl::Theorem(t) => &t.common,
        ExportDecl::Axiom(a) => &a.common,
        other => {
            return Err(format!(
                "the challenge declares it as a {}",
                other.kind_label()
            ))
        }
    };
    match sol {
        ExportDecl::Theorem(t) => same_const(chal_common, &t.common),
        other => Err(format!(
            "the solution declares it as a {}, not a theorem",
            other.kind_label()
        )),
    }
}

/// Exact identity of two declaration records.
fn same_decl(a: &ExportDecl, b: &ExportDecl) -> Result<(), String> {
    match (a, b) {
        (ExportDecl::Axiom(x), ExportDecl::Axiom(y)) => {
            same_const(&x.common, &y.common)?;
            same_field("unsafe flag", &x.is_unsafe, &y.is_unsafe)
        }
        (ExportDecl::Definition(x), ExportDecl::Definition(y)) => {
            same_const(&x.common, &y.common)?;
            same_term("value", &x.value, &y.value)?;
            same_field("reducibility hints", &x.hints, &y.hints)?;
            same_field("safety", &x.safety, &y.safety)?;
            same_field("mutual group", &x.all, &y.all)
        }
        (ExportDecl::Theorem(x), ExportDecl::Theorem(y)) => {
            same_const(&x.common, &y.common)?;
            same_term("proof", &x.value, &y.value)?;
            same_field("mutual group", &x.all, &y.all)
        }
        (ExportDecl::Opaque(x), ExportDecl::Opaque(y)) => {
            same_const(&x.common, &y.common)?;
            same_term("value", &x.value, &y.value)?;
            same_field("unsafe flag", &x.is_unsafe, &y.is_unsafe)?;
            same_field("mutual group", &x.all, &y.all)
        }
        (ExportDecl::Quot(x), ExportDecl::Quot(y)) => {
            same_const(&x.common, &y.common)?;
            same_field("quotient kind", &x.kind, &y.kind)
        }
        (ExportDecl::Inductive(x), ExportDecl::Inductive(y)) => same_bundle(x, y),
        (ExportDecl::Oversized(_), _) | (_, ExportDecl::Oversized(_)) => {
            Err("not read into memory (over the per-declaration budget)".to_string())
        }
        (x, y) => Err(format!(
            "declared as a {} in the challenge, a {} here",
            x.kind_label(),
            y.kind_label()
        )),
    }
}

fn same_bundle(x: &InductiveBundle, y: &InductiveBundle) -> Result<(), String> {
    same_field("number of types", &x.types.len(), &y.types.len())?;
    same_field("number of constructors", &x.ctors.len(), &y.ctors.len())?;
    same_field("number of recursors", &x.recs.len(), &y.recs.len())?;
    for (a, b) in x.types.iter().zip(&y.types) {
        same_inductive(a, b)?;
    }
    for (a, b) in x.ctors.iter().zip(&y.ctors) {
        same_ctor(a, b)?;
    }
    for (a, b) in x.recs.iter().zip(&y.recs) {
        same_rec(a, b)?;
    }
    Ok(())
}

fn same_inductive(a: &InductiveVal, b: &InductiveVal) -> Result<(), String> {
    same_const(&a.common, &b.common)?;
    same_field("parameter count", &a.num_params, &b.num_params)?;
    same_field("index count", &a.num_indices, &b.num_indices)?;
    same_field("mutual group", &a.all, &b.all)?;
    same_field("constructor list", &a.ctors, &b.ctors)?;
    same_field("nesting count", &a.num_nested, &b.num_nested)?;
    same_field("recursive flag", &a.is_rec, &b.is_rec)?;
    same_field("unsafe flag", &a.is_unsafe, &b.is_unsafe)?;
    same_field("reflexive flag", &a.is_reflexive, &b.is_reflexive)
}

fn same_ctor(a: &ConstructorVal, b: &ConstructorVal) -> Result<(), String> {
    same_const(&a.common, &b.common)?;
    same_field("inductive type", &a.induct, &b.induct)?;
    same_field("constructor index", &a.cidx, &b.cidx)?;
    same_field("parameter count", &a.num_params, &b.num_params)?;
    same_field("field count", &a.num_fields, &b.num_fields)?;
    same_field("unsafe flag", &a.is_unsafe, &b.is_unsafe)
}

fn same_rec(a: &RecursorVal, b: &RecursorVal) -> Result<(), String> {
    same_const(&a.common, &b.common)?;
    same_field("mutual group", &a.all, &b.all)?;
    same_field("parameter count", &a.num_params, &b.num_params)?;
    same_field("index count", &a.num_indices, &b.num_indices)?;
    same_field("motive count", &a.num_motives, &b.num_motives)?;
    same_field("minor premise count", &a.num_minors, &b.num_minors)?;
    same_field("K flag", &a.k, &b.k)?;
    same_field("unsafe flag", &a.is_unsafe, &b.is_unsafe)?;
    same_field("rule count", &a.rules.len(), &b.rules.len())?;
    for (ra, rb) in a.rules.iter().zip(&b.rules) {
        same_field("rule constructor", &ra.ctor, &rb.ctor)?;
        same_field("rule field count", &ra.nfields, &rb.nfields)?;
        same_term("rule right-hand side", &ra.rhs, &rb.rhs)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{verify_stream_inspect, VerifyOptions};
    use std::io::Cursor;

    /// The records every test export shares: names, levels, and the terms of
    /// `def D : Sort 1 := Sort 0`, the statement `S := (x : D) → x → x`, its
    /// proof `fun x h => h`, `sorryAx.{u} : (α : Sort u) → α`, and a second
    /// statement `S' := (x : D) → x → x → x` with its proof.
    const COMMON: &str = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"0","version":"4.35.0"}}}
{"in":1,"str":{"pre":0,"str":"D"}}
{"in":2,"str":{"pre":0,"str":"t"}}
{"in":3,"str":{"pre":0,"str":"sorryAx"}}
{"in":4,"str":{"pre":0,"str":"x"}}
{"in":5,"str":{"pre":0,"str":"h"}}
{"in":6,"str":{"pre":0,"str":"u"}}
{"in":7,"str":{"pre":0,"str":"α"}}
{"in":8,"str":{"pre":0,"str":"bad"}}
{"il":1,"succ":0}
{"il":2,"param":6}
{"ie":0,"sort":0}
{"ie":1,"sort":1}
{"const":{"name":1,"us":[]},"ie":2}
{"bvar":0,"ie":3}
{"bvar":1,"ie":4}
{"forallE":{"binderInfo":"default","body":4,"name":5,"type":3},"ie":5}
{"forallE":{"binderInfo":"default","body":5,"name":4,"type":2},"ie":6}
{"ie":7,"lam":{"binderInfo":"default","body":3,"name":5,"type":3}}
{"ie":8,"lam":{"binderInfo":"default","body":7,"name":4,"type":2}}
{"ie":9,"sort":2}
{"forallE":{"binderInfo":"default","body":3,"name":7,"type":9},"ie":10}
{"const":{"name":3,"us":[0]},"ie":11}
{"app":{"arg":6,"fn":11},"ie":12}
{"bvar":2,"ie":13}
{"forallE":{"binderInfo":"default","body":13,"name":5,"type":4},"ie":14}
{"forallE":{"binderInfo":"default","body":14,"name":5,"type":3},"ie":15}
{"forallE":{"binderInfo":"default","body":15,"name":4,"type":2},"ie":16}
{"ie":17,"lam":{"binderInfo":"default","body":4,"name":5,"type":4}}
{"ie":18,"lam":{"binderInfo":"default","body":17,"name":5,"type":3}}
{"ie":19,"lam":{"binderInfo":"default","body":18,"name":4,"type":2}}
{"const":{"name":8,"us":[]},"ie":20}
"#;

    const DEF_D: &str = r#"{"def":{"all":[1],"hints":{"regular":1},"levelParams":[],"name":1,"safety":"safe","type":1,"value":0}}"#;
    const SORRY: &str = r#"{"axiom":{"isUnsafe":false,"levelParams":[6],"name":3,"type":10}}"#;

    fn export(decls: &[&str]) -> String {
        let mut s = COMMON.to_string();
        for d in decls {
            s.push_str(d);
            s.push('\n');
        }
        s
    }

    /// The challenge: `theorem t : S := sorryAx S`.
    fn challenge() -> String {
        export(&[
            DEF_D,
            SORRY,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":12}}"#,
        ])
    }

    fn run(challenge: &str, solution: &str, theorems: Option<&[String]>) -> CompareOutcome {
        let ch = ChallengeFile::load(
            Cursor::new(challenge.as_bytes().to_vec()),
            Limits::default(),
            theorems,
        )
        .expect("the challenge is readable");
        let allowed: Vec<String> = DEFAULT_ALLOWED_AXIOMS
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let mut auditor = Auditor::new(ch, &allowed);
        let report = verify_stream_inspect(
            Cursor::new(solution.as_bytes().to_vec()),
            "test",
            VerifyOptions::default(),
            |d, e| auditor.observe(d, e),
        )
        .expect("the solution is readable");
        assert_eq!(
            report.summary.rejected, 0,
            "the test solutions are well typed"
        );
        auditor.finish()
    }

    #[test]
    fn targets_are_found_from_sorry() {
        let ch = ChallengeFile::load(
            Cursor::new(challenge().into_bytes()),
            Limits::default(),
            None,
        )
        .expect("the challenge is readable");
        assert_eq!(ch.targets(), &[Name::str("t")]);
        assert_eq!(ch.declarations(), 3);
    }

    #[test]
    fn an_honest_solution_is_established() {
        let solution = export(&[
            DEF_D,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":8}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(out.established(), "{out:?}");
        assert_eq!(out.closure_size, 2);
        assert!(out.axioms_used.is_empty());
        assert_eq!(out.challenge_compared, 2, "D and t; sorryAx is excluded");
    }

    #[test]
    fn a_named_target_must_be_a_theorem_of_the_challenge() {
        let r = ChallengeFile::load(
            Cursor::new(challenge().into_bytes()),
            Limits::default(),
            Some(&["D".to_string()]),
        );
        assert!(r.is_err(), "D is a definition");
        let r = ChallengeFile::load(
            Cursor::new(challenge().into_bytes()),
            Limits::default(),
            Some(&["nope".to_string()]),
        );
        assert!(r.is_err(), "nope is not declared");
    }

    #[test]
    fn a_different_statement_is_not_established() {
        let solution = export(&[
            DEF_D,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":16,"value":19}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(!out.established());
        assert_eq!(
            out.targets[0].status,
            TargetStatus::Differs("type differs".to_string())
        );
        assert!(
            out.targets[0].verified,
            "the different theorem is itself fine"
        );
    }

    #[test]
    fn a_target_stated_as_an_axiom_is_not_established() {
        let solution = export(&[
            DEF_D,
            r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":2,"type":6}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(!out.established());
        assert!(matches!(out.targets[0].status, TargetStatus::Differs(_)));
        assert_eq!(out.forbidden_axioms, vec!["t".to_string()]);
    }

    #[test]
    fn a_forbidden_axiom_is_not_established() {
        let solution = export(&[
            DEF_D,
            r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":8,"type":6}}"#,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":20}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(!out.established());
        assert_eq!(out.targets[0].status, TargetStatus::Matches);
        assert_eq!(out.axioms_used, vec!["bad".to_string()]);
        assert_eq!(out.forbidden_axioms, vec!["bad".to_string()]);
    }

    #[test]
    fn sorry_in_the_solution_is_not_established() {
        let solution = export(&[
            DEF_D,
            SORRY,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":12}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(!out.established());
        assert_eq!(out.forbidden_axioms, vec![SORRY_AXIOM.to_string()]);
    }

    #[test]
    fn a_changed_definition_is_not_established() {
        let solution = export(&[
            r#"{"def":{"all":[1],"hints":{"regular":2},"levelParams":[],"name":1,"safety":"safe","type":1,"value":0}}"#,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":8}}"#,
        ]);
        let out = run(&challenge(), &solution, None);
        assert!(!out.established());
        assert_eq!(out.differ_count, 1);
        assert_eq!(out.differ[0].0, "D");
        assert_eq!(out.differ[0].1, "reducibility hints differs");
    }

    #[test]
    fn a_missing_challenge_declaration_is_not_established() {
        // The solution states `t` with a `D` of its own making: an axiom.
        let solution = export(&[
            r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":1}}"#,
            r#"{"thm":{"all":[2],"levelParams":[],"name":2,"type":6,"value":8}}"#,
        ]);
        let ch = ChallengeFile::load(
            Cursor::new(challenge().into_bytes()),
            Limits::default(),
            None,
        )
        .expect("the challenge is readable");
        let allowed = vec!["D".to_string()];
        let mut auditor = Auditor::new(ch, &allowed);
        // `t` does not type-check against an opaque `D`; what matters here is
        // that the comparison flags `D`, whatever the kernel says of `t`.
        let _ = verify_stream_inspect(
            Cursor::new(solution.into_bytes()),
            "test",
            VerifyOptions::default(),
            |d, e| auditor.observe(d, e),
        )
        .expect("the solution is readable");
        let out = auditor.finish();
        assert!(!out.established());
        assert_eq!(out.differ_count, 1);
        assert_eq!(
            out.differ[0].1,
            "declared as a def in the challenge, a axiom here"
        );
    }

    #[test]
    fn collect_consts_sees_every_constant_once() {
        let mut out = HashSet::new();
        let a = Expr::Const(Name::str("A"), vec![]);
        let node = oxilean_kernel::Node::new(a);
        let app = Expr::App(node.clone(), node);
        let proj = Expr::Proj(
            Name::str("S"),
            0,
            oxilean_kernel::Node::new(Expr::Const(Name::str("B"), vec![])),
        );
        collect_consts(&[&app, &proj], &mut out);
        let mut got: Vec<String> = out.iter().map(|n| n.to_string()).collect();
        got.sort();
        assert_eq!(got, vec!["A", "B", "S"]);
    }
}
