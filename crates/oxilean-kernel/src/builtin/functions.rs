//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, ConstructorVal, InductiveVal, RecursorRule, RecursorVal,
};
use crate::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BuiltinInfo, BuiltinKind, ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack,
    LabelSet, NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Initialize the environment with built-in types and axioms.
pub fn init_builtin_env(env: &mut Environment) -> Result<(), String> {
    add_legacy_axioms(env)?;
    add_bool_inductive(env)?;
    add_unit_inductive(env)?;
    add_empty_inductive(env)?;
    add_nat_inductive(env)?;
    add_string_type(env)?;
    add_core_axioms(env)?;
    add_decidable_eq(env)?;
    add_eq_inductive(env)?;
    add_prod_inductive(env)?;
    add_list_inductive(env)?;
    Ok(())
}
/// Add legacy axiom declarations (backward compat).
/// Uses the old flat-name convention.
pub(super) fn add_legacy_axioms(env: &mut Environment) -> Result<(), String> {
    let type0 = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("Bool"),
        univ_params: vec![],
        ty: type1.clone(),
    })
    .map_err(|e| e.to_string())?;
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
        name: Name::str("Unit"),
        univ_params: vec![],
        ty: type1,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("unit"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Unit"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Empty"),
        univ_params: vec![],
        ty: type0.clone(),
    })
    .map_err(|e| e.to_string())?;
    let rec_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("C"),
        Box::new(type0),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Const(Name::str("Empty"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Empty.rec"),
        univ_params: vec![],
        ty: rec_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Bool as a proper inductive type (ConstantInfo only, no legacy overlap).
pub(super) fn add_bool_inductive(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let bool_const = Expr::Const(Name::str("Bool"), vec![]);
    let ctor_true = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Bool.true"),
            level_params: vec![],
            ty: bool_const.clone(),
        },
        induct: Name::str("Bool"),
        cidx: 0,
        num_params: 0,
        num_fields: 0,
        is_unsafe: false,
    });
    let ctor_false = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Bool.false"),
            level_params: vec![],
            ty: bool_const,
        },
        induct: Name::str("Bool"),
        cidx: 1,
        num_params: 0,
        num_fields: 0,
        is_unsafe: false,
    });
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Bool.ind"),
            level_params: vec![],
            ty: type1,
        },
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("Bool")],
        ctors: vec![Name::str("Bool.true"), Name::str("Bool.false")],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("Bool.rec"),
            level_params: vec![Name::str("u_1")],
            ty: Expr::Sort(Level::zero()),
        },
        all: vec![Name::str("Bool")],
        num_params: 0,
        num_indices: 0,
        num_motives: 1,
        num_minors: 2,
        rules: vec![
            RecursorRule {
                ctor: Name::str("Bool.true"),
                nfields: 0,
                rhs: Expr::BVar(0),
            },
            RecursorRule {
                ctor: Name::str("Bool.false"),
                nfields: 0,
                rhs: Expr::BVar(0),
            },
        ],
        k: false,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor_true).map_err(|e| e.to_string())?;
    env.add_constant(ctor_false).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Unit as a proper inductive type.
pub(super) fn add_unit_inductive(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let unit_const = Expr::Const(Name::str("Unit"), vec![]);
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Unit.ind"),
            level_params: vec![],
            ty: type1,
        },
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("Unit")],
        ctors: vec![Name::str("Unit.unit")],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let ctor = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Unit.unit"),
            level_params: vec![],
            ty: unit_const,
        },
        induct: Name::str("Unit"),
        cidx: 0,
        num_params: 0,
        num_fields: 0,
        is_unsafe: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("Unit.rec"),
            level_params: vec![Name::str("u_1")],
            ty: Expr::Sort(Level::zero()),
        },
        all: vec![Name::str("Unit")],
        num_params: 0,
        num_indices: 0,
        num_motives: 1,
        num_minors: 1,
        rules: vec![RecursorRule {
            ctor: Name::str("Unit.unit"),
            nfields: 0,
            rhs: Expr::BVar(0),
        }],
        k: false,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Empty as a proper inductive type (no constructors).
pub(super) fn add_empty_inductive(env: &mut Environment) -> Result<(), String> {
    let type0 = Expr::Sort(Level::zero());
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Empty.ind"),
            level_params: vec![],
            ty: type0,
        },
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("Empty")],
        ctors: vec![],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: true,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    Ok(())
}
/// Add Nat as a proper inductive type.
pub(super) fn add_nat_inductive(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let nat_const = Expr::Const(Name::str("Nat"), vec![]);
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Nat"),
            level_params: vec![],
            ty: type1,
        },
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("Nat")],
        ctors: vec![Name::str("Nat.zero"), Name::str("Nat.succ")],
        num_nested: 0,
        is_rec: true,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let ctor_zero = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Nat.zero"),
            level_params: vec![],
            ty: nat_const.clone(),
        },
        induct: Name::str("Nat"),
        cidx: 0,
        num_params: 0,
        num_fields: 0,
        is_unsafe: false,
    });
    let succ_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(nat_const.clone()),
        Box::new(nat_const),
    );
    let ctor_succ = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Nat.succ"),
            level_params: vec![],
            ty: succ_ty,
        },
        induct: Name::str("Nat"),
        cidx: 1,
        num_params: 0,
        num_fields: 1,
        is_unsafe: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("Nat.rec"),
            level_params: vec![Name::str("u_1")],
            ty: Expr::Sort(Level::zero()),
        },
        all: vec![Name::str("Nat")],
        num_params: 0,
        num_indices: 0,
        num_motives: 1,
        num_minors: 2,
        rules: vec![
            RecursorRule {
                ctor: Name::str("Nat.zero"),
                nfields: 0,
                rhs: Expr::BVar(0),
            },
            RecursorRule {
                ctor: Name::str("Nat.succ"),
                nfields: 1,
                rhs: Expr::BVar(0),
            },
        ],
        k: false,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor_zero).map_err(|e| e.to_string())?;
    env.add_constant(ctor_succ).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
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
        Box::new(nat.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(nat.clone()),
            Box::new(nat.clone()),
        )),
    );
    let nat_cmp = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(nat.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(nat),
            Box::new(bool_ty),
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
    Ok(())
}
/// Add the String type.
pub(super) fn add_string_type(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let str_ty = Expr::Const(Name::str("String"), vec![]);
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("String"),
            level_params: vec![],
            ty: type1,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("String.length"),
            level_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(str_ty.clone()),
                Box::new(nat_ty),
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
                Box::new(str_ty.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(str_ty.clone()),
                    Box::new(str_ty.clone()),
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
                Box::new(str_ty.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(str_ty),
                    Box::new(bool_ty),
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
        Box::new(type0.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("b"),
            Box::new(type0.clone()),
            Box::new(type0.clone()),
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
    let quot_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(Expr::Sort(Level::param(Name::str("u")))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("r"),
            Box::new(type0.clone()),
            Box::new(Expr::Sort(Level::param(Name::str("u")))),
        )),
    );
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("Quot"),
            level_params: vec![Name::str("u")],
            ty: quot_ty,
        },
        is_unsafe: false,
    }))
    .map_err(|e| e.to_string())?;
    let choice_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(Expr::Sort(Level::param(Name::str("u")))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type0),
            Box::new(Expr::BVar(1)),
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
        Box::new(type1.clone()),
        Box::new(type0),
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
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Const(Name::str("Bool"), vec![])),
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
/// Add Eq (propositional equality) as a proper inductive type.
///
/// ```text
/// inductive Eq.{u} : {α : Sort u} → α → α → Prop where
///   | refl : ∀ {α : Sort u} (a : α), Eq a a
/// ```
pub(super) fn add_eq_inductive(env: &mut Environment) -> Result<(), String> {
    let u = Level::param(Name::str("u"));
    let prop = Expr::Sort(Level::zero());
    let sort_u = Expr::Sort(u.clone());
    let eq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(sort_u.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(Expr::BVar(1)),
                Box::new(prop.clone()),
            )),
        )),
    );
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Eq"),
            level_params: vec![Name::str("u")],
            ty: eq_ty,
        },
        num_params: 1,
        num_indices: 2,
        all: vec![Name::str("Eq")],
        ctors: vec![Name::str("Eq.refl")],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: true,
    });
    let eq_refl_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(sort_u),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![u.clone()])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::BVar(0)),
            )),
        )),
    );
    let ctor_refl = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Eq.refl"),
            level_params: vec![Name::str("u")],
            ty: eq_refl_ty,
        },
        induct: Name::str("Eq"),
        cidx: 0,
        num_params: 1,
        num_fields: 1,
        is_unsafe: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("Eq.rec"),
            level_params: vec![Name::str("u"), Name::str("v")],
            ty: prop.clone(),
        },
        all: vec![Name::str("Eq")],
        num_params: 1,
        num_indices: 2,
        num_motives: 1,
        num_minors: 1,
        rules: vec![RecursorRule {
            ctor: Name::str("Eq.refl"),
            nfields: 1,
            rhs: Expr::BVar(0),
        }],
        k: true,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor_refl).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
    Ok(())
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
        Box::new(type_u.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("β"),
            Box::new(type_v.clone()),
            Box::new(type_max),
        )),
    );
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("Prod"),
            level_params: vec![Name::str("u"), Name::str("v")],
            ty: prod_ty,
        },
        num_params: 2,
        num_indices: 0,
        all: vec![Name::str("Prod")],
        ctors: vec![Name::str("Prod.mk")],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let prod_mk_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type_u),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type_v),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("fst"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("snd"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Prod"), vec![u.clone(), v.clone()])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    let ctor_mk = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("Prod.mk"),
            level_params: vec![Name::str("u"), Name::str("v")],
            ty: prod_mk_ty,
        },
        induct: Name::str("Prod"),
        cidx: 0,
        num_params: 2,
        num_fields: 2,
        is_unsafe: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("Prod.rec"),
            level_params: vec![Name::str("u"), Name::str("v"), Name::str("w")],
            ty: Expr::Sort(Level::zero()),
        },
        all: vec![Name::str("Prod")],
        num_params: 2,
        num_indices: 0,
        num_motives: 1,
        num_minors: 1,
        rules: vec![RecursorRule {
            ctor: Name::str("Prod.mk"),
            nfields: 2,
            rhs: Expr::BVar(0),
        }],
        k: false,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor_mk).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
    Ok(())
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
        Box::new(Expr::Const(Name::str("List"), vec![u.clone()])),
        Box::new(Expr::BVar(0)),
    );
    let list_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type_u.clone()),
        Box::new(type_u.clone()),
    );
    let ind = ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str("List"),
            level_params: vec![Name::str("u")],
            ty: list_ty,
        },
        num_params: 1,
        num_indices: 0,
        all: vec![Name::str("List")],
        ctors: vec![Name::str("List.nil"), Name::str("List.cons")],
        num_nested: 0,
        is_rec: true,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let nil_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type_u.clone()),
        Box::new(list_bvar0.clone()),
    );
    let ctor_nil = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("List.nil"),
            level_params: vec![Name::str("u")],
            ty: nil_ty,
        },
        induct: Name::str("List"),
        cidx: 0,
        num_params: 1,
        num_fields: 0,
        is_unsafe: false,
    });
    let cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type_u),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("head"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("tail"),
                Box::new(list_bvar0.clone()),
                Box::new(list_bvar0),
            )),
        )),
    );
    let ctor_cons = ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str("List.cons"),
            level_params: vec![Name::str("u")],
            ty: cons_ty,
        },
        induct: Name::str("List"),
        cidx: 1,
        num_params: 1,
        num_fields: 2,
        is_unsafe: false,
    });
    let rec = ConstantInfo::Recursor(RecursorVal {
        common: ConstantVal {
            name: Name::str("List.rec"),
            level_params: vec![Name::str("u"), Name::str("v")],
            ty: Expr::Sort(Level::zero()),
        },
        all: vec![Name::str("List")],
        num_params: 1,
        num_indices: 0,
        num_motives: 1,
        num_minors: 2,
        rules: vec![
            RecursorRule {
                ctor: Name::str("List.nil"),
                nfields: 0,
                rhs: Expr::BVar(0),
            },
            RecursorRule {
                ctor: Name::str("List.cons"),
                nfields: 2,
                rhs: Expr::BVar(0),
            },
        ],
        k: false,
        is_unsafe: false,
    });
    env.add_constant(ind).map_err(|e| e.to_string())?;
    env.add_constant(ctor_nil).map_err(|e| e.to_string())?;
    env.add_constant(ctor_cons).map_err(|e| e.to_string())?;
    env.add_constant(rec).map_err(|e| e.to_string())?;
    Ok(())
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
        assert!(env.get(&Name::str("Bool")).is_some());
        assert!(env.get(&Name::str("true")).is_some());
        assert!(env.get(&Name::str("false")).is_some());
        assert!(env.get(&Name::str("Unit")).is_some());
        assert!(env.get(&Name::str("unit")).is_some());
        assert!(env.get(&Name::str("Empty")).is_some());
    }
    #[test]
    fn test_bool_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        let bool_decl = env
            .get(&Name::str("Bool"))
            .expect("bool_decl should be present");
        assert!(matches!(bool_decl, Declaration::Axiom { .. }));
        let true_decl = env
            .get(&Name::str("true"))
            .expect("true_decl should be present");
        assert!(matches!(true_decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_unit_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        let unit_decl = env
            .get(&Name::str("Unit"))
            .expect("unit_decl should be present");
        assert!(matches!(unit_decl, Declaration::Axiom { .. }));
        let unit_val = env
            .get(&Name::str("unit"))
            .expect("unit_val should be present");
        assert!(matches!(unit_val, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_empty_axioms() {
        let mut env = Environment::new();
        init_builtin_env(&mut env).expect("value should be present");
        let empty_decl = env
            .get(&Name::str("Empty"))
            .expect("empty_decl should be present");
        assert!(matches!(empty_decl, Declaration::Axiom { .. }));
        let rec_decl = env
            .get(&Name::str("Empty.rec"))
            .expect("rec_decl should be present");
        assert!(matches!(rec_decl, Declaration::Axiom { .. }));
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
        "Bool.ind",
        "Bool.true",
        "Bool.false",
        "Bool.rec",
        "true",
        "false",
        "Unit",
        "Unit.ind",
        "Unit.unit",
        "Unit.rec",
        "unit",
        "Empty",
        "Empty.ind",
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
        "String.length",
        "String.append",
        "String.beq",
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
        let mut tb = TokenBucket::new(100, 10);
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
