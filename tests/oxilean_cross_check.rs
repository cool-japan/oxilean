//! OX6 step 12 — cross-check `leo4-lean4-parse` against
//! `oxilean-parse` v0.1.2.
//!
//! **Strict-superset invariant**: for every source that
//! `oxilean-parse` accepts, `leo4-lean4-parse` must also
//! accept it, the two parsers must agree on the decl
//! count, and corresponding decls must share a `name` and
//! a compatible `kind` tag.
//!
//! `oxilean-parse` rejecting a source is fine — that is
//! the "strict" half of the invariant; leo4-lean4-parse
//! extends the surface beyond what upstream accepts.
//!
//! AST field-level equivalence is NOT asserted: the two
//! parsers have intentionally divergent internal shapes
//! (e.g. our `Expr::BinOp("+", …)` vs upstream's
//! application-tree `App(App(Plus, lhs), rhs)`), so a
//! structural comparison at the expression level would be
//! a comparison of representation choices, not semantics.
//! Decl-level identity (name + kind tag) is the contract
//! consumers of leo4-oxilean-build actually depend on.

use leo4_lean4_parse::{parse_decls as our_parse, DeclKind as OurKind};
use oxilean_parse::{parse_file as their_parse, Decl as TheirDecl};

/// A single corpus entry.
struct Case {
    /// Short label for diagnostic output.
    label: &'static str,
    /// Lean 4 source.
    src: &'static str,
}

/// The corpus. Each entry MUST be in `oxilean-parse`'s
/// accepted subset of Lean 4 — that is the precondition
/// for cross-checking. Sources designed to exercise the
/// shared surface, not leo4-lean4-parse's extensions.
const CORPUS: &[Case] = &[
    Case {
        label: "single def",
        src: "def x : Nat := 1",
    },
    Case {
        label: "def with binders",
        src: "def add (a : Nat) (b : Nat) : Nat := a",
    },
    Case {
        label: "two defs",
        src: "def x : Nat := 1\ndef y : Nat := 2",
    },
    Case {
        label: "theorem",
        src: "theorem refl : a = a := rfl",
    },
    Case {
        label: "axiom",
        src: "axiom em : p",
    },
    Case {
        label: "namespace with one def",
        src: "namespace Foo\ndef x : Nat := 1\nend Foo",
    },
    Case {
        label: "inductive",
        src: "inductive Color where\n  | red\n  | green\n  | blue",
    },
    Case {
        label: "structure with field",
        src: "structure Point where\n  x : Nat\n  y : Nat",
    },
    Case {
        label: "import",
        src: "import Foo.Bar",
    },
];

#[test]
fn strict_superset_corpus() {
    let mut failures: Vec<String> = Vec::new();

    for case in CORPUS {
        let theirs = match their_parse(case.src) {
            Ok(v) => v,
            Err(e) => {
                // Source is not in oxilean-parse's accepted
                // subset — it does not participate in the
                // cross-check invariant. Skip silently
                // (with a stderr note so corpus drift is
                // visible during dev).
                eprintln!(
                    "[cross-check] SKIP `{}` — oxilean-parse rejected: {e:?}",
                    case.label
                );
                continue;
            }
        };

        let ours = match our_parse(case.src) {
            Ok(v) => v,
            Err(e) => {
                failures.push(format!(
                    "case `{}`: oxilean-parse accepted but leo4-lean4-parse \
                     REJECTED with {e:?} — strict-superset invariant violated",
                    case.label
                ));
                continue;
            }
        };

        if theirs.len() != ours.len() {
            failures.push(format!(
                "case `{}`: decl count mismatch — oxilean-parse {} vs \
                 leo4-lean4-parse {}",
                case.label,
                theirs.len(),
                ours.len()
            ));
            continue;
        }

        for (i, (t, o)) in theirs.iter().zip(ours.iter()).enumerate() {
            let t_name = t.value.name();
            let o_name = our_decl_name(&o.kind);
            if t_name != o_name.as_deref() {
                failures.push(format!(
                    "case `{}` decl[{}]: name mismatch — \
                     oxilean-parse `{:?}` vs leo4-lean4-parse `{:?}`",
                    case.label, i, t_name, o_name
                ));
                continue;
            }
            if !kind_tags_compatible(&t.value, &o.kind) {
                failures.push(format!(
                    "case `{}` decl[{}]: kind tag mismatch — \
                     oxilean-parse `{}` vs leo4-lean4-parse `{}`",
                    case.label,
                    i,
                    their_kind_tag(&t.value),
                    our_kind_tag(&o.kind),
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "OX6 step 12 cross-check failures ({} case(s)):\n\n  - {}\n",
        failures.len(),
        failures.join("\n  - ")
    );
}

/// Project a leo4-lean4-parse `DeclKind` onto its
/// declaration name (the shape that `oxilean-parse`'s
/// `Decl::name()` returns).
fn our_decl_name(k: &OurKind) -> Option<String> {
    match k {
        OurKind::Definition { name, .. }
        | OurKind::DefinitionByArms { name, .. }
        | OurKind::Theorem { name, .. }
        | OurKind::Axiom { name, .. }
        | OurKind::Structure { name, .. }
        | OurKind::Class { name, .. }
        | OurKind::Inductive { name, .. }
        | OurKind::Namespace { name, .. } => Some(name.clone()),
        OurKind::Instance { name, .. } | OurKind::Section { name, .. } => name.clone(),
        // `Import` is intentionally name-less under
        // oxilean-parse's contract (`Decl::name()` returns
        // None for imports — the "name" is just a module
        // path, not an introduced binding). Match that.
        // Similarly for `Example`, `Open`, `Variable`,
        // `Mutual`, `HashCommand`, `Dsl`, `Omit`, `Include`.
        _ => None,
    }
}

/// Compatibility between the two parsers' kind tags.
/// Same conceptual decl maps to the same comparison tag.
fn kind_tags_compatible(t: &TheirDecl, o: &OurKind) -> bool {
    their_kind_tag(t) == our_kind_tag(o)
}

fn their_kind_tag(t: &TheirDecl) -> &'static str {
    match t {
        TheirDecl::Axiom { .. } => "axiom",
        TheirDecl::Definition { .. } => "definition",
        TheirDecl::Theorem { .. } => "theorem",
        TheirDecl::Inductive { .. } => "inductive",
        TheirDecl::Import { .. } => "import",
        TheirDecl::Namespace { .. } => "namespace",
        TheirDecl::Structure { .. } => "structure",
        TheirDecl::ClassDecl { .. } => "class",
        TheirDecl::InstanceDecl { .. } => "instance",
        TheirDecl::SectionDecl { .. } => "section",
        _ => "other",
    }
}

fn our_kind_tag(o: &OurKind) -> &'static str {
    match o {
        OurKind::Axiom { .. } => "axiom",
        OurKind::Definition { .. } | OurKind::DefinitionByArms { .. } => "definition",
        OurKind::Theorem { .. } => "theorem",
        OurKind::Inductive { .. } => "inductive",
        OurKind::Import { .. } => "import",
        OurKind::Namespace { .. } => "namespace",
        OurKind::Structure { .. } => "structure",
        OurKind::Class { .. } => "class",
        OurKind::Instance { .. } => "instance",
        OurKind::Section { .. } => "section",
        _ => "other",
    }
}
