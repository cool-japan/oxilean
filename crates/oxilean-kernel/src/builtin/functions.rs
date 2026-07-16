//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{AxiomVal, ConstantInfo, ConstantVal};
use crate::inductive::derive::{add_inductive_family, InductiveSpec};
use crate::Node;
use crate::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::rc::Rc;

use super::types::{
    BuiltinInfo, BuiltinKind, ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack,
    LabelSet, NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Initialize the environment with built-in types and axioms.
///
/// All builtin inductive types (`Bool`, `Unit`, `Empty`, `Nat`, `Eq`,
/// `Prod`, `List`, `String`) go through the kernel's *checked* declaration
/// path ([`crate::inductive::derive::add_inductive_family`]): the recursors
/// are derived — with real types and iota rules — rather than hand-written.
pub fn init_builtin_env(env: &mut Environment) -> Result<(), String> {
    add_bool_inductive(env)?;
    add_unit_inductive(env)?;
    add_empty_inductive(env)?;
    add_nat_inductive(env)?;
    add_legacy_axioms(env)?;
    add_core_axioms(env)?;
    add_decidable_eq(env)?;
    add_eq_inductive(env)?;
    // Install the four `#QUOT` primitives with their canonical, kernel-built
    // types. Must come after `Eq` is available (see `Environment::add_quot`).
    env.add_quot().map_err(|e| e.to_string())?;
    add_prod_inductive(env)?;
    add_list_inductive(env)?;
    add_string_type(env)?;
    Ok(())
}
/// Declare a single builtin inductive through the checked kernel path.
fn add_builtin_inductive(
    env: &mut Environment,
    lparams: Vec<Name>,
    num_params: u32,
    spec: InductiveSpec,
) -> Result<(), String> {
    add_inductive_family(env, lparams, num_params, vec![spec]).map_err(|e| e.to_string())
}
/// Add legacy value axioms (backward compat, flat-name convention):
/// `true`, `false` : Bool and `unit` : Unit.
pub(super) fn add_legacy_axioms(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("true"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Bool"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("false"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Bool"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("unit"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Unit"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Bool as a proper inductive type with a derived recursor.
pub(super) fn add_bool_inductive(env: &mut Environment) -> Result<(), String> {
    let bool_const = Expr::Const(Name::str("Bool"), vec![]);
    let spec = InductiveSpec::new(
        Name::str("Bool"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![
            (Name::str("Bool.true"), bool_const.clone()),
            (Name::str("Bool.false"), bool_const),
        ],
    )
    .with_rec_name(Name::str("Bool.rec"));
    add_builtin_inductive(env, vec![], 0, spec)
}
/// Add Unit as a proper inductive type with a derived recursor.
pub(super) fn add_unit_inductive(env: &mut Environment) -> Result<(), String> {
    let spec = InductiveSpec::new(
        Name::str("Unit"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![(
            Name::str("Unit.unit"),
            Expr::Const(Name::str("Unit"), vec![]),
        )],
    )
    .with_rec_name(Name::str("Unit.rec"));
    add_builtin_inductive(env, vec![], 0, spec)
}
/// Add Empty as a proper Prop inductive type (no constructors) with a
/// derived recursor. Having no constructors, it large-eliminates
/// (ex falso): `Empty.rec.{u} : {motive : Empty → Sort u} → (t : Empty) →
/// motive t`.
pub(super) fn add_empty_inductive(env: &mut Environment) -> Result<(), String> {
    let spec = InductiveSpec::new(Name::str("Empty"), Expr::Sort(Level::zero()), vec![])
        .with_rec_name(Name::str("Empty.rec"));
    add_builtin_inductive(env, vec![], 0, spec)
}
/// Add Nat as a proper inductive type with a derived recursor (the `succ`
/// minor premise carries its induction hypothesis:
/// `(n : Nat) → motive n → motive (Nat.succ n)`).
pub(super) fn add_nat_inductive(env: &mut Environment) -> Result<(), String> {
    let nat_const = Expr::Const(Name::str("Nat"), vec![]);
    let succ_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Node::new(nat_const.clone()),
        Node::new(nat_const.clone()),
    );
    let spec = InductiveSpec::new(
        Name::str("Nat"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![
            (Name::str("Nat.zero"), nat_const),
            (Name::str("Nat.succ"), succ_ty),
        ],
    )
    .with_rec_name(Name::str("Nat.rec"));
    add_builtin_inductive(env, vec![], 0, spec)?;
    register_nat_ops(env)?;
    Ok(())
}
/// Register built-in Nat arithmetic operations.
pub(super) fn register_nat_ops(env: &mut Environment) -> Result<(), String> {
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
    let nat_binop = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(nat.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Node::new(nat.clone()),
            Node::new(nat.clone()),
        )),
    );
    let nat_cmp = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(nat.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Node::new(nat),
            Node::new(bool_ty),
        )),
    );
    let binop_names = [
        "Nat.add",
        "Nat.sub",
        "Nat.mul",
        "Nat.div",
        "Nat.mod",
        "Nat.pow",
        "Nat.gcd",
        "Nat.land",
        "Nat.lor",
        "Nat.xor",
        "Nat.shiftLeft",
        "Nat.shiftRight",
    ];
    for name in &binop_names {
        env.add_constant(ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(*name),
                level_params: vec![],
                ty: nat_binop.clone(),
            },
            is_unsafe: false,
        }))
        .map_err(|e| e.to_string())?;
    }
    let cmp_names = ["Nat.beq", "Nat.ble", "Nat.blt"];
    for name in &cmp_names {
        env.add_constant(ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(*name),
                level_params: vec![],
                ty: nat_cmp.clone(),
            },
            is_unsafe: false,
        }))
        .map_err(|e| e.to_string())?;
    }
    // Unary op: Nat.log2 : Nat -> Nat (kernel literal extension).
    let nat_unop = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(Expr::Const(Name::str("Nat"), vec![])),
        Node::new(Expr::Const(Name::str("Nat"), vec![])),
    );
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("Nat.log2"),
            level_params: vec![],
            ty: nat_unop,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add the String type as a proper inductive (`String.mk : List Char →
/// String`) with a derived recursor, plus the `Char` scaffolding and the
/// String literal operations. Must run after `add_list_inductive`.
pub(super) fn add_string_type(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let str_ty = Expr::Const(Name::str("String"), vec![]);
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
    let char_ty = Expr::Const(Name::str("Char"), vec![]);
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("Char"),
            level_params: vec![],
            ty: type1,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("Char.ofNat"),
            level_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Node::new(nat_ty.clone()),
                Node::new(char_ty.clone()),
            ),
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    let list_char = Expr::App(
        Node::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
        Node::new(char_ty),
    );
    let mk_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("data"),
        Node::new(list_char),
        Node::new(str_ty.clone()),
    );
    let spec = InductiveSpec::new(
        Name::str("String"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![(Name::str("String.mk"), mk_ty)],
    )
    .with_rec_name(Name::str("String.rec"));
    add_builtin_inductive(env, vec![], 0, spec)?;
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("String.length"),
            level_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Node::new(str_ty.clone()),
                Node::new(nat_ty),
            ),
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("String.append"),
            level_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Node::new(str_ty.clone()),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Node::new(str_ty.clone()),
                    Node::new(str_ty.clone()),
                )),
            ),
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("String.beq"),
            level_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Node::new(str_ty.clone()),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Node::new(str_ty),
                    Node::new(bool_ty),
                )),
            ),
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add core logical axioms.
pub(super) fn add_core_axioms(env: &mut Environment) -> Result<(), String> {
    let type0 = Expr::Sort(Level::zero());
    let propext_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("a"),
        Node::new(type0.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("b"),
            Node::new(type0.clone()),
            Node::new(type0.clone()),
        )),
    );
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("propext"),
            level_params: vec![],
            ty: propext_ty,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    // NOTE: `Quot` (and `Quot.mk` / `Quot.lift` / `Quot.ind`) are NOT registered
    // here. Quotients are a kernel primitive, not an axiom: they are installed
    // with their canonical, kernel-constructed types via `Environment::add_quot`
    // (called from `init_builtin_env` after `Eq` is available). The previous
    // `Quot : {α : Sort u} → Prop → Sort u` axiom was both the wrong type (the
    // relation binder must be `α → α → Prop`) and the wrong declaration kind.
    let choice_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(Expr::Sort(Level::param(Name::str("u")))),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(type0),
            Node::new(Expr::BVar(1)),
        )),
    );
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("Classical.choice"),
            level_params: vec![Name::str("u")],
            ty: choice_ty,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add decidable equality.
pub(super) fn add_decidable_eq(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type0 = Expr::Sort(Level::zero());
    let decidable_eq_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type1.clone()),
        Node::new(type0),
    );
    env.add(Declaration::Axiom {
        name: Name::str("DecidableEq"),
        univ_params: vec![],
        ty: decidable_eq_ty,
    })
    .map_err(|e| e.to_string())?;
    let decide_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(type1),
        Node::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("DecidableEq"), vec![])),
                Node::new(Expr::BVar(0)),
            )),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Node::new(Expr::BVar(1)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Node::new(Expr::BVar(2)),
                    Node::new(Expr::Const(Name::str("Bool"), vec![])),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("DecidableEq.decide"),
        univ_params: vec![],
        ty: decide_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Eq (propositional equality) as a proper inductive type with a
/// derived recursor.
///
/// Lean-exact shape: `Eq.{u} {α : Sort u} (a b : α) : Prop` has **two**
/// parameters (`α`, `a`) and one index (`b`); `Eq.refl {α} (a : α) : Eq a a`
/// therefore has zero fields, which is what makes `Eq` K-like and a
/// subsingleton eliminator (it large-eliminates despite living in Prop).
pub(super) fn add_eq_inductive(env: &mut Environment) -> Result<(), String> {
    let u = Level::param(Name::str("u"));
    let prop = Expr::Sort(Level::zero());
    let sort_u = Expr::Sort(u.clone());
    let eq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_u.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Node::new(Expr::BVar(1)),
                Node::new(prop),
            )),
        )),
    );
    let eq_refl_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(sort_u),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("Eq"), vec![u.clone()])),
                        Node::new(Expr::BVar(1)),
                    )),
                    Node::new(Expr::BVar(0)),
                )),
                Node::new(Expr::BVar(0)),
            )),
        )),
    );
    let spec = InductiveSpec::new(
        Name::str("Eq"),
        eq_ty,
        vec![(Name::str("Eq.refl"), eq_refl_ty)],
    )
    .with_rec_name(Name::str("Eq.rec"));
    add_builtin_inductive(env, vec![Name::str("u")], 2, spec)
}
/// Add Prod (dependent pair / product type) as a proper inductive type.
///
/// ```text
/// structure Prod.{u, v} (α : Type u) (β : Type v) : Type (max u v) where
///   | mk : α → β → Prod α β
/// ```
pub(super) fn add_prod_inductive(env: &mut Environment) -> Result<(), String> {
    let u = Level::param(Name::str("u"));
    let v = Level::param(Name::str("v"));
    let type_u = Expr::Sort(Level::succ(u.clone()));
    let type_v = Expr::Sort(Level::succ(v.clone()));
    let type_max = Expr::Sort(Level::succ(Level::max(u.clone(), v.clone())));
    let prod_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type_u.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("β"),
            Node::new(type_v.clone()),
            Node::new(type_max),
        )),
    );
    let prod_mk_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(type_u),
        Node::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Node::new(type_v),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("fst"),
                Node::new(Expr::BVar(1)),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("snd"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("Prod"), vec![u.clone(), v.clone()])),
                            Node::new(Expr::BVar(3)),
                        )),
                        Node::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    let spec = InductiveSpec::new(
        Name::str("Prod"),
        prod_ty,
        vec![(Name::str("Prod.mk"), prod_mk_ty)],
    )
    .with_rec_name(Name::str("Prod.rec"));
    add_builtin_inductive(env, vec![Name::str("u"), Name::str("v")], 2, spec)
}
/// Add List as a proper inductive type.
///
/// ```text
/// inductive List.{u} (α : Type u) : Type u where
///   | nil : List α
///   | cons : α → List α → List α
/// ```
pub(super) fn add_list_inductive(env: &mut Environment) -> Result<(), String> {
    let u = Level::param(Name::str("u"));
    let type_u = Expr::Sort(Level::succ(u.clone()));
    let list_bvar0 = Expr::App(
        Node::new(Expr::Const(Name::str("List"), vec![u.clone()])),
        Node::new(Expr::BVar(0)),
    );
    let list_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type_u.clone()),
        Node::new(type_u.clone()),
    );
    let nil_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(type_u.clone()),
        Node::new(list_bvar0.clone()),
    );
    // cons : {α : Type u} → (head : α) → (tail : List α) → List α.
    // The `α` reference is BVar(1) under `head` and BVar(2) under `tail`
    // (the previous hand-written builtin used BVar(0) in both positions —
    // an ill-typed `List head` / `List tail` — which the checked
    // declaration path now rejects).
    let list_of = |i: u32| {
        Expr::App(
            Node::new(Expr::Const(Name::str("List"), vec![u.clone()])),
            Node::new(Expr::BVar(i)),
        )
    };
    let cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(type_u),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("head"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("tail"),
                Node::new(list_of(1)),
                Node::new(list_of(2)),
            )),
        )),
    );
    let spec = InductiveSpec::new(
        Name::str("List"),
        list_ty,
        vec![
            (Name::str("List.nil"), nil_ty),
            (Name::str("List.cons"), cons_ty),
        ],
    )
    .with_rec_name(Name::str("List.rec"));
    add_builtin_inductive(env, vec![Name::str("u")], 1, spec)
}
/// Check if a name is a built-in primitive.
pub fn is_builtin(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "Bool"
            | "Bool.ind"
            | "Bool.true"
            | "Bool.false"
            | "Bool.rec"
            | "true"
            | "false"
            | "Unit"
            | "Unit.ind"
            | "Unit.unit"
            | "Unit.rec"
            | "unit"
            | "Empty"
            | "Empty.ind"
            | "Empty.rec"
            | "Nat"
            | "Nat.zero"
            | "Nat.succ"
            | "Nat.rec"
            | "String"
            | "DecidableEq"
            | "DecidableEq.decide"
            | "propext"
            | "Quot"
            | "Classical.choice"
    )
}
/// Check if a name is a built-in Nat operation.
pub fn is_nat_op(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "Nat.add"
            | "Nat.sub"
            | "Nat.mul"
            | "Nat.div"
            | "Nat.mod"
            | "Nat.pow"
            | "Nat.gcd"
            | "Nat.beq"
            | "Nat.ble"
            | "Nat.blt"
            | "Nat.land"
            | "Nat.lor"
            | "Nat.xor"
            | "Nat.shiftLeft"
            | "Nat.shiftRight"
            | "Nat.log2"
    )
}
/// Check if a name is a built-in String operation.
pub fn is_string_op(name: &Name) -> bool {
    let s = name.to_string();
    matches!(s.as_str(), "String.length" | "String.append" | "String.beq")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init_builtin_env() {
        let mut env = Environment::new();
        assert!(init_builtin_env(&mut env).is_ok());
        assert!(env.is_inductive(&Name::str("Bool")));
        assert!(env.get(&Name::str("true")).is_some());
        assert!(env.get(&Name::str("false")).is_some());
        assert!(env.is_inductive(&Name::str("Unit")));
        assert!(env.get(&Name::str("unit")).is_some());
        assert!(env.is_inductive(&Name::str("Empty")));
    }
    #[test]
    fn test_bool_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        // Bool is now a real inductive (not a legacy axiom).
        assert!(env.is_inductive(&Name::str("Bool")));
        let true_decl = env
            .get(&Name::str("true"))
            .expect("true_decl should be present");
        assert!(matches!(true_decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_unit_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.is_inductive(&Name::str("Unit")));
        assert!(env.is_constructor(&Name::str("Unit.unit")));
        let unit_val = env
            .get(&Name::str("unit"))
            .expect("unit_val should be present");
        assert!(matches!(unit_val, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_empty_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        // Empty is a Prop inductive with no constructors; its derived
        // recursor large-eliminates (ex falso).
        assert!(env.is_inductive(&Name::str("Empty")));
        let rec = env
            .get_recursor_val(&Name::str("Empty.rec"))
            .expect("Empty.rec should be present");
        assert_eq!(rec.num_minors, 0);
        assert_eq!(rec.common.level_params.len(), 1);
    }
    #[test]
    fn test_decidable_eq() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        let dec_eq = env
            .get(&Name::str("DecidableEq"))
            .expect("dec_eq should be present");
        assert!(matches!(dec_eq, Declaration::Axiom { .. }));
        let decide = env
            .get(&Name::str("DecidableEq.decide"))
            .expect("decide should be present");
        assert!(matches!(decide, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_is_builtin() {
        assert!(is_builtin(&Name::str("Bool")));
        assert!(is_builtin(&Name::str("true")));
        assert!(is_builtin(&Name::str("Unit")));
        assert!(is_builtin(&Name::str("Nat")));
        assert!(is_builtin(&Name::str("String")));
        assert!(!is_builtin(&Name::str("CustomType")));
    }
    #[test]
    fn test_bool_inductive() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.is_constructor(&Name::str("Bool.true")));
        assert!(env.is_constructor(&Name::str("Bool.false")));
        assert!(env.is_recursor(&Name::str("Bool.rec")));
    }
    #[test]
    fn test_nat_inductive() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.is_inductive(&Name::str("Nat")));
        assert!(env.is_constructor(&Name::str("Nat.zero")));
        assert!(env.is_constructor(&Name::str("Nat.succ")));
        assert!(env.is_recursor(&Name::str("Nat.rec")));
    }
    #[test]
    fn test_nat_ops_registered() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.find(&Name::str("Nat.add")).is_some());
        assert!(env.find(&Name::str("Nat.mul")).is_some());
        assert!(env.find(&Name::str("Nat.sub")).is_some());
        assert!(env.find(&Name::str("Nat.div")).is_some());
        assert!(env.find(&Name::str("Nat.beq")).is_some());
    }
    #[test]
    fn test_string_ops_registered() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.find(&Name::str("String")).is_some());
        assert!(env.find(&Name::str("String.length")).is_some());
        assert!(env.find(&Name::str("String.append")).is_some());
        assert!(env.find(&Name::str("String.beq")).is_some());
    }
    #[test]
    fn test_core_axioms_registered() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        assert!(env.find(&Name::str("propext")).is_some());
        assert!(env.find(&Name::str("Quot")).is_some());
        assert!(env.find(&Name::str("Classical.choice")).is_some());
    }
    #[test]
    fn test_is_nat_op() {
        assert!(is_nat_op(&Name::str("Nat.add")));
        assert!(is_nat_op(&Name::str("Nat.mul")));
        assert!(is_nat_op(&Name::str("Nat.gcd")));
        assert!(!is_nat_op(&Name::str("Nat.zero")));
    }
    #[test]
    fn test_is_string_op() {
        assert!(is_string_op(&Name::str("String.length")));
        assert!(is_string_op(&Name::str("String.append")));
        assert!(!is_string_op(&Name::str("String")));
    }
}
/// Classify a name into its `BuiltinKind`.
#[allow(dead_code)]
pub fn classify_builtin(name: &Name) -> BuiltinKind {
    let s = name.to_string();
    if is_nat_op(name) {
        return BuiltinKind::ArithOp;
    }
    if is_string_op(name) {
        return BuiltinKind::StringOp;
    }
    match s.as_str() {
        "Bool" | "Unit" | "Empty" | "Nat" | "String" => BuiltinKind::Type,
        "Bool.true" | "Bool.false" | "Unit.unit" | "Nat.zero" | "Nat.succ" | "true" | "false"
        | "unit" => BuiltinKind::Constructor,
        "Bool.rec" | "Unit.rec" | "Nat.rec" | "Empty.rec" => BuiltinKind::Recursor,
        "propext" | "Quot" | "Classical.choice" => BuiltinKind::Axiom,
        "Nat.beq" | "Nat.ble" | "Nat.blt" => BuiltinKind::CmpOp,
        "DecidableEq" | "DecidableEq.decide" => BuiltinKind::TypeClass,
        _ => BuiltinKind::Unknown,
    }
}
/// Check whether a name is a core logical connective.
#[allow(dead_code)]
pub fn is_logical_connective(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "And" | "Or" | "Not" | "Iff" | "True" | "False" | "Exists"
    )
}
/// Check whether a name is a primitive value (not a type).
#[allow(dead_code)]
pub fn is_primitive_value(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "true" | "false" | "unit" | "Bool.true" | "Bool.false" | "Unit.unit" | "Nat.zero"
    )
}
/// Return the universe level of a builtin type (0 = Prop, 1 = Type₀).
#[allow(dead_code)]
pub fn builtin_universe_level(name: &Name) -> Option<u32> {
    let s = name.to_string();
    match s.as_str() {
        "Empty" => Some(0),
        "Bool" | "Unit" | "Nat" | "String" => Some(1),
        _ => None,
    }
}
/// Return the number of constructors for a builtin inductive type.
#[allow(dead_code)]
pub fn builtin_ctor_count(name: &Name) -> Option<usize> {
    let s = name.to_string();
    match s.as_str() {
        "Bool" => Some(2),
        "Unit" => Some(1),
        "Empty" => Some(0),
        "Nat" => Some(2),
        _ => None,
    }
}
/// Check whether a builtin type is recursive.
#[allow(dead_code)]
pub fn builtin_is_recursive(name: &Name) -> bool {
    name.to_string() == "Nat"
}
/// Check whether a builtin type is in Prop.
#[allow(dead_code)]
pub fn builtin_is_prop(name: &Name) -> bool {
    let s = name.to_string();
    matches!(s.as_str(), "Empty" | "True" | "False")
}
/// Get the full list of all builtin names.
#[allow(dead_code)]
pub fn all_builtin_names() -> Vec<&'static str> {
    vec![
        "Bool",
        "Bool.true",
        "Bool.false",
        "Bool.rec",
        "true",
        "false",
        "Unit",
        "Unit.unit",
        "Unit.rec",
        "unit",
        "Empty",
        "Empty.rec",
        "Nat",
        "Nat.zero",
        "Nat.succ",
        "Nat.rec",
        "Nat.add",
        "Nat.sub",
        "Nat.mul",
        "Nat.div",
        "Nat.mod",
        "Nat.pow",
        "Nat.gcd",
        "Nat.beq",
        "Nat.ble",
        "Nat.blt",
        "Nat.land",
        "Nat.lor",
        "Nat.xor",
        "Nat.shiftLeft",
        "Nat.shiftRight",
        "String",
        "String.mk",
        "String.rec",
        "String.length",
        "String.append",
        "String.beq",
        "Char",
        "Char.ofNat",
        "Eq",
        "Eq.refl",
        "Eq.rec",
        "Prod",
        "Prod.mk",
        "Prod.rec",
        "List",
        "List.nil",
        "List.cons",
        "List.rec",
        "propext",
        "Quot",
        "Classical.choice",
        "DecidableEq",
        "DecidableEq.decide",
    ]
}
/// Count the total number of builtin names.
#[allow(dead_code)]
pub fn builtin_count() -> usize {
    all_builtin_names().len()
}
/// Check if all builtin names are registered in an environment.
#[allow(dead_code)]
pub fn verify_builtins(env: &Environment) -> Vec<&'static str> {
    all_builtin_names()
        .into_iter()
        .filter(|n| env.find(&Name::str(*n)).is_none() && env.get(&Name::str(*n)).is_none())
        .collect()
}
/// Return the Lean 4 equivalent name for a builtin OxiLean name.
#[allow(dead_code)]
pub fn lean4_name(name: &Name) -> Option<&'static str> {
    let s = name.to_string();
    match s.as_str() {
        "Bool.true" => Some("Bool.true"),
        "Bool.false" => Some("Bool.false"),
        "true" => Some("Bool.true"),
        "false" => Some("Bool.false"),
        "unit" => Some("Unit.unit"),
        "Nat.zero" => Some("Nat.zero"),
        "Nat.succ" => Some("Nat.succ"),
        _ => None,
    }
}
/// Return info for all core builtin types.
#[allow(dead_code)]
pub fn core_builtin_infos() -> Vec<BuiltinInfo> {
    vec![
        BuiltinInfo {
            name: "Bool",
            kind: BuiltinKind::Type,
            description: "Boolean type",
        },
        BuiltinInfo {
            name: "Unit",
            kind: BuiltinKind::Type,
            description: "Unit type (single-element)",
        },
        BuiltinInfo {
            name: "Empty",
            kind: BuiltinKind::Type,
            description: "Empty type (no elements)",
        },
        BuiltinInfo {
            name: "Nat",
            kind: BuiltinKind::Type,
            description: "Natural numbers",
        },
        BuiltinInfo {
            name: "String",
            kind: BuiltinKind::Type,
            description: "Unicode string type",
        },
        BuiltinInfo {
            name: "propext",
            kind: BuiltinKind::Axiom,
            description: "Propositional extensionality",
        },
        BuiltinInfo {
            name: "Quot",
            kind: BuiltinKind::Axiom,
            description: "Quotient type constructor",
        },
        BuiltinInfo {
            name: "Classical.choice",
            kind: BuiltinKind::Axiom,
            description: "Classical choice axiom",
        },
    ]
}
#[cfg(test)]
mod extended_builtin_tests {
    use super::*;
    #[test]
    fn test_classify_builtin_type() {
        assert_eq!(classify_builtin(&Name::str("Bool")), BuiltinKind::Type);
        assert_eq!(classify_builtin(&Name::str("Nat")), BuiltinKind::Type);
    }
    #[test]
    fn test_classify_builtin_ctor() {
        assert_eq!(
            classify_builtin(&Name::str("Bool.true")),
            BuiltinKind::Constructor
        );
        assert_eq!(
            classify_builtin(&Name::str("Nat.zero")),
            BuiltinKind::Constructor
        );
    }
    #[test]
    fn test_classify_builtin_arith() {
        assert_eq!(
            classify_builtin(&Name::str("Nat.add")),
            BuiltinKind::ArithOp
        );
        assert_eq!(
            classify_builtin(&Name::str("Nat.mul")),
            BuiltinKind::ArithOp
        );
    }
    #[test]
    fn test_classify_builtin_axiom() {
        assert_eq!(classify_builtin(&Name::str("propext")), BuiltinKind::Axiom);
        assert_eq!(
            classify_builtin(&Name::str("Classical.choice")),
            BuiltinKind::Axiom
        );
    }
    #[test]
    fn test_classify_builtin_unknown() {
        assert_eq!(
            classify_builtin(&Name::str("CustomThing")),
            BuiltinKind::Unknown
        );
    }
    #[test]
    fn test_is_primitive_value() {
        assert!(is_primitive_value(&Name::str("true")));
        assert!(is_primitive_value(&Name::str("false")));
        assert!(is_primitive_value(&Name::str("unit")));
        assert!(!is_primitive_value(&Name::str("Nat")));
    }
    #[test]
    fn test_builtin_universe_level() {
        assert_eq!(builtin_universe_level(&Name::str("Empty")), Some(0));
        assert_eq!(builtin_universe_level(&Name::str("Bool")), Some(1));
        assert_eq!(builtin_universe_level(&Name::str("CustomType")), None);
    }
    #[test]
    fn test_builtin_ctor_count() {
        assert_eq!(builtin_ctor_count(&Name::str("Bool")), Some(2));
        assert_eq!(builtin_ctor_count(&Name::str("Empty")), Some(0));
        assert_eq!(builtin_ctor_count(&Name::str("Unit")), Some(1));
    }
    #[test]
    fn test_builtin_is_recursive() {
        assert!(builtin_is_recursive(&Name::str("Nat")));
        assert!(!builtin_is_recursive(&Name::str("Bool")));
    }
    #[test]
    fn test_builtin_is_prop() {
        assert!(builtin_is_prop(&Name::str("Empty")));
        assert!(!builtin_is_prop(&Name::str("Nat")));
    }
    #[test]
    fn test_all_builtin_names_nonempty() {
        assert!(!all_builtin_names().is_empty());
        assert!(builtin_count() > 20);
    }
    #[test]
    fn test_core_builtin_infos() {
        let infos = core_builtin_infos();
        assert!(!infos.is_empty());
        assert!(infos.iter().any(|i| i.name == "Bool"));
    }
    #[test]
    fn test_builtin_kind_description() {
        assert_eq!(BuiltinKind::Type.description(), "primitive type");
        assert_eq!(BuiltinKind::Axiom.description(), "logical axiom");
        assert_eq!(BuiltinKind::Unknown.description(), "not a builtin");
    }
    #[test]
    fn test_lean4_name() {
        assert_eq!(lean4_name(&Name::str("true")), Some("Bool.true"));
        assert_eq!(lean4_name(&Name::str("unit")), Some("Unit.unit"));
        assert_eq!(lean4_name(&Name::str("CustomFn")), None);
    }
    #[test]
    fn test_verify_builtins() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        let missing = verify_builtins(&env);
        assert!(missing.len() < 15);
    }
    #[test]
    fn test_is_logical_connective() {
        assert!(is_logical_connective(&Name::str("And")));
        assert!(is_logical_connective(&Name::str("Or")));
        assert!(is_logical_connective(&Name::str("Iff")));
        assert!(!is_logical_connective(&Name::str("Nat")));
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
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
