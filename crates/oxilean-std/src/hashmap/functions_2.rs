//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use oxilean_kernel::{
    BinderInfo as HmBI, Declaration as HmDecl, Expr as HmExpr, Level as HmLevel, Name as HmName,
};

use super::functions::*;
use super::types::*;

/// Register all extended HashMap axioms into the environment.
///
/// Adds 35+ axioms covering:
/// - HashMap as finite partial function
/// - get-after-insert laws
/// - Insert commutativity
/// - Delete correctness
/// - Union laws (left-biased, associativity)
/// - Intersection and difference
/// - Map (fmap over values)
/// - Filter
/// - foldl/foldr
/// - HashMap as functor/monoid
/// - Cardinality
/// - Keys/values as sets
/// - Merge with conflict resolution
/// - Hash collision handling
/// - Load factor / resizing
/// - Association list isomorphism
/// - Extensionality
/// - Ordered vs unordered maps
#[allow(dead_code)]
#[allow(clippy::too_many_lines)]
pub fn register_hashmap_extended_axioms(env: &mut Environment) {
    let mut add = |name: &str, ty: HmExpr| {
        let _ = env.add(HmDecl::Axiom {
            name: HmName::str(name),
            univ_params: vec![],
            ty,
        });
    };
    add(
        "HashMap.get",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_bvar(1), hm_option(hm_bvar(1))),
                ),
            ),
        ),
    );
    add(
        "HashMap.insert",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(
                        hm_bvar(1),
                        hm_arrow(hm_bvar(1), hm_ty(hm_bvar(2), hm_bvar(1))),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.delete",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_bvar(1), hm_ty(hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.get_insert_same",
        hm_ext_forall_map_k_v(hm_eq(
            hm_option(hm_bvar(2)),
            hm_app2(
                hm_cst("HashMap.get"),
                hm_app3(hm_cst("HashMap.insert"), hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                hm_bvar(1),
            ),
            hm_app(hm_cst("Option.some"), hm_bvar(0)),
        )),
    );
    add(
        "HashMap.get_insert_diff",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k1",
                        hm_bvar(2),
                        hm_pi(
                            HmBI::Default,
                            "k2",
                            hm_bvar(3),
                            hm_pi(
                                HmBI::Default,
                                "v",
                                hm_bvar(3),
                                hm_arrow(
                                    hm_arrow(
                                        hm_eq(hm_bvar(5), hm_bvar(2), hm_bvar(1)),
                                        hm_app(hm_cst("False"), hm_prop()),
                                    ),
                                    hm_eq(
                                        hm_option(hm_bvar(3)),
                                        hm_app2(
                                            hm_cst("HashMap.get"),
                                            hm_app3(
                                                hm_cst("HashMap.insert"),
                                                hm_bvar(4),
                                                hm_bvar(2),
                                                hm_bvar(0),
                                            ),
                                            hm_bvar(1),
                                        ),
                                        hm_app2(hm_cst("HashMap.get"), hm_bvar(4), hm_bvar(1)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.insert_comm",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k1",
                        hm_bvar(2),
                        hm_pi(
                            HmBI::Default,
                            "k2",
                            hm_bvar(3),
                            hm_pi(
                                HmBI::Default,
                                "v1",
                                hm_bvar(3),
                                hm_pi(
                                    HmBI::Default,
                                    "v2",
                                    hm_bvar(4),
                                    hm_arrow(
                                        hm_arrow(
                                            hm_eq(hm_bvar(6), hm_bvar(3), hm_bvar(2)),
                                            hm_app(hm_cst("False"), hm_prop()),
                                        ),
                                        hm_ext_map_eq(
                                            hm_bvar(7),
                                            hm_bvar(6),
                                            hm_app3(
                                                hm_cst("HashMap.insert"),
                                                hm_app3(
                                                    hm_cst("HashMap.insert"),
                                                    hm_bvar(5),
                                                    hm_bvar(3),
                                                    hm_bvar(1),
                                                ),
                                                hm_bvar(2),
                                                hm_bvar(0),
                                            ),
                                            hm_app3(
                                                hm_cst("HashMap.insert"),
                                                hm_app3(
                                                    hm_cst("HashMap.insert"),
                                                    hm_bvar(5),
                                                    hm_bvar(2),
                                                    hm_bvar(0),
                                                ),
                                                hm_bvar(3),
                                                hm_bvar(1),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.get_delete_same",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k",
                        hm_bvar(2),
                        hm_eq(
                            hm_option(hm_bvar(1)),
                            hm_app2(
                                hm_cst("HashMap.get"),
                                hm_app2(hm_cst("HashMap.delete"), hm_bvar(1), hm_bvar(0)),
                                hm_bvar(0),
                            ),
                            hm_cst("Option.none"),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.get_delete_diff",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k1",
                        hm_bvar(2),
                        hm_pi(
                            HmBI::Default,
                            "k2",
                            hm_bvar(3),
                            hm_arrow(
                                hm_arrow(
                                    hm_eq(hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                                    hm_app(hm_cst("False"), hm_prop()),
                                ),
                                hm_eq(
                                    hm_option(hm_bvar(3)),
                                    hm_app2(
                                        hm_cst("HashMap.get"),
                                        hm_app2(hm_cst("HashMap.delete"), hm_bvar(3), hm_bvar(1)),
                                        hm_bvar(0),
                                    ),
                                    hm_app2(hm_cst("HashMap.get"), hm_bvar(3), hm_bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.delete_insert_same",
        hm_ext_forall_map_k_v(hm_ext_map_eq(
            hm_bvar(4),
            hm_bvar(3),
            hm_app2(
                hm_cst("HashMap.delete"),
                hm_app3(hm_cst("HashMap.insert"), hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                hm_bvar(1),
            ),
            hm_app2(hm_cst("HashMap.delete"), hm_bvar(4), hm_bvar(1)),
        )),
    );
    add(
        "HashMap.union",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_ty(hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.union_empty_left",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.union"),
                hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
                hm_bvar(0),
            ),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.union_empty_right",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.union"),
                hm_bvar(0),
                hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
            ),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.union_assoc",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m1",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "m2",
                        hm_ty(hm_bvar(2), hm_bvar(1)),
                        hm_pi(
                            HmBI::Default,
                            "m3",
                            hm_ty(hm_bvar(3), hm_bvar(2)),
                            hm_ext_map_eq(
                                hm_bvar(4),
                                hm_bvar(3),
                                hm_app2(
                                    hm_cst("HashMap.union"),
                                    hm_app2(hm_cst("HashMap.union"), hm_bvar(2), hm_bvar(1)),
                                    hm_bvar(0),
                                ),
                                hm_app2(
                                    hm_cst("HashMap.union"),
                                    hm_bvar(2),
                                    hm_app2(hm_cst("HashMap.union"), hm_bvar(1), hm_bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.union_get_left",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m1",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "m2",
                        hm_ty(hm_bvar(2), hm_bvar(1)),
                        hm_pi(
                            HmBI::Default,
                            "k",
                            hm_bvar(3),
                            hm_pi(
                                HmBI::Default,
                                "v",
                                hm_bvar(3),
                                hm_arrow(
                                    hm_eq(
                                        hm_option(hm_bvar(3)),
                                        hm_app2(hm_cst("HashMap.get"), hm_bvar(4), hm_bvar(1)),
                                        hm_app(hm_cst("Option.some"), hm_bvar(0)),
                                    ),
                                    hm_eq(
                                        hm_option(hm_bvar(3)),
                                        hm_app2(
                                            hm_cst("HashMap.get"),
                                            hm_app2(
                                                hm_cst("HashMap.union"),
                                                hm_bvar(4),
                                                hm_bvar(3),
                                            ),
                                            hm_bvar(1),
                                        ),
                                        hm_app(hm_cst("Option.some"), hm_bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.inter",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_ty(hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.diff",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_ty(hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.inter_comm",
        hm_ext_forall_two_maps(hm_ext_map_eq(
            hm_bvar(3),
            hm_bvar(2),
            hm_app2(hm_cst("HashMap.inter"), hm_bvar(1), hm_bvar(0)),
            hm_app2(hm_cst("HashMap.inter"), hm_bvar(0), hm_bvar(1)),
        )),
    );
    add(
        "HashMap.diff_empty",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.diff"),
                hm_bvar(0),
                hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
            ),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.diff_self",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(hm_cst("HashMap.diff"), hm_bvar(0), hm_bvar(0)),
            hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
        )),
    );
    add(
        "HashMap.map",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Implicit,
                    "W",
                    hm_type1(),
                    hm_arrow(
                        hm_arrow(hm_bvar(1), hm_bvar(1)),
                        hm_arrow(hm_ty(hm_bvar(2), hm_bvar(1)), hm_ty(hm_bvar(3), hm_bvar(1))),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.map_id",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(hm_cst("HashMap.map"), hm_cst("id"), hm_bvar(0)),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.map_comp",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Implicit,
                    "W",
                    hm_type1(),
                    hm_pi(
                        HmBI::Implicit,
                        "X",
                        hm_type1(),
                        hm_pi(
                            HmBI::Default,
                            "f",
                            hm_arrow(hm_bvar(2), hm_bvar(2)),
                            hm_pi(
                                HmBI::Default,
                                "g",
                                hm_arrow(hm_bvar(2), hm_bvar(2)),
                                hm_pi(
                                    HmBI::Default,
                                    "m",
                                    hm_ty(hm_bvar(5), hm_bvar(4)),
                                    hm_ext_map_eq(
                                        hm_bvar(6),
                                        hm_bvar(4),
                                        hm_app2(
                                            hm_cst("HashMap.map"),
                                            hm_app2(
                                                hm_cst("Function.comp"),
                                                hm_bvar(1),
                                                hm_bvar(2),
                                            ),
                                            hm_bvar(0),
                                        ),
                                        hm_app2(
                                            hm_cst("HashMap.map"),
                                            hm_bvar(1),
                                            hm_app2(hm_cst("HashMap.map"), hm_bvar(2), hm_bvar(0)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.filter",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_arrow(hm_bvar(1), hm_arrow(hm_bvar(1), hm_bool())),
                    hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_ty(hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.filter_true",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.filter"),
                hm_cst("HashMap.filter_true_fn"),
                hm_bvar(0),
            ),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.filter_false",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.filter"),
                hm_cst("HashMap.filter_false_fn"),
                hm_bvar(0),
            ),
            hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
        )),
    );
    add(
        "HashMap.foldl",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Implicit,
                    "A",
                    hm_type1(),
                    hm_arrow(
                        hm_arrow(
                            hm_bvar(0),
                            hm_arrow(hm_bvar(2), hm_arrow(hm_bvar(2), hm_bvar(2))),
                        ),
                        hm_arrow(
                            hm_bvar(1),
                            hm_arrow(hm_ty(hm_bvar(2), hm_bvar(1)), hm_bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.foldr",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Implicit,
                    "A",
                    hm_type1(),
                    hm_arrow(
                        hm_arrow(
                            hm_bvar(2),
                            hm_arrow(hm_bvar(2), hm_arrow(hm_bvar(1), hm_bvar(1))),
                        ),
                        hm_arrow(
                            hm_bvar(1),
                            hm_arrow(hm_ty(hm_bvar(2), hm_bvar(1)), hm_bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.foldl_empty",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Implicit,
                    "A",
                    hm_type1(),
                    hm_pi(
                        HmBI::Default,
                        "f",
                        hm_arrow(
                            hm_bvar(0),
                            hm_arrow(hm_bvar(2), hm_arrow(hm_bvar(2), hm_bvar(2))),
                        ),
                        hm_pi(
                            HmBI::Default,
                            "init",
                            hm_bvar(1),
                            hm_eq(
                                hm_bvar(2),
                                hm_app3(
                                    hm_cst("HashMap.foldl"),
                                    hm_bvar(1),
                                    hm_bvar(0),
                                    hm_app2(hm_cst("HashMap.empty"), hm_bvar(4), hm_bvar(3)),
                                ),
                                hm_bvar(0),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.union_idem",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(hm_cst("HashMap.union"), hm_bvar(0), hm_bvar(0)),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.size",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_nat()),
            ),
        ),
    );
    add(
        "HashMap.size_empty",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_eq(
                    hm_nat(),
                    hm_app(
                        hm_cst("HashMap.size"),
                        hm_app2(hm_cst("HashMap.empty"), hm_bvar(1), hm_bvar(0)),
                    ),
                    hm_cst("Nat.zero"),
                ),
            ),
        ),
    );
    add(
        "HashMap.size_insert_new",
        hm_ext_forall_map_k_v(hm_arrow(
            hm_eq(
                hm_option(hm_bvar(2)),
                hm_app2(hm_cst("HashMap.get"), hm_bvar(4), hm_bvar(1)),
                hm_cst("Option.none"),
            ),
            hm_eq(
                hm_nat(),
                hm_app(
                    hm_cst("HashMap.size"),
                    hm_app3(hm_cst("HashMap.insert"), hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                ),
                hm_app(
                    hm_cst("Nat.succ"),
                    hm_app(hm_cst("HashMap.size"), hm_bvar(4)),
                ),
            ),
        )),
    );
    add(
        "HashMap.size_delete",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k",
                        hm_bvar(2),
                        hm_arrow(
                            hm_arrow(
                                hm_eq(
                                    hm_option(hm_bvar(1)),
                                    hm_app2(hm_cst("HashMap.get"), hm_bvar(1), hm_bvar(0)),
                                    hm_cst("Option.none"),
                                ),
                                hm_app(hm_cst("False"), hm_prop()),
                            ),
                            hm_eq(
                                hm_nat(),
                                hm_app(
                                    hm_cst("HashMap.size"),
                                    hm_app2(hm_cst("HashMap.delete"), hm_bvar(1), hm_bvar(0)),
                                ),
                                hm_app(
                                    hm_cst("Nat.pred"),
                                    hm_app(hm_cst("HashMap.size"), hm_bvar(1)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.keys",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_list(hm_bvar(1))),
            ),
        ),
    );
    add(
        "HashMap.values",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_list(hm_bvar(0))),
            ),
        ),
    );
    add(
        "HashMap.keys_length_eq_size",
        hm_ext_forall_map(hm_eq(
            hm_nat(),
            hm_app(
                hm_cst("List.length"),
                hm_app(hm_cst("HashMap.keys"), hm_bvar(0)),
            ),
            hm_app(hm_cst("HashMap.size"), hm_bvar(0)),
        )),
    );
    add(
        "HashMap.values_length_eq_size",
        hm_ext_forall_map(hm_eq(
            hm_nat(),
            hm_app(
                hm_cst("List.length"),
                hm_app(hm_cst("HashMap.values"), hm_bvar(0)),
            ),
            hm_app(hm_cst("HashMap.size"), hm_bvar(0)),
        )),
    );
    add(
        "HashMap.mergeWith",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_arrow(hm_bvar(0), hm_arrow(hm_bvar(1), hm_bvar(1))),
                    hm_arrow(
                        hm_ty(hm_bvar(1), hm_bvar(0)),
                        hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_ty(hm_bvar(1), hm_bvar(0))),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.mergeWith_comm_when_f_comm",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "f",
                    hm_arrow(hm_bvar(0), hm_arrow(hm_bvar(1), hm_bvar(1))),
                    hm_pi(
                        HmBI::Default,
                        "m1",
                        hm_ty(hm_bvar(2), hm_bvar(1)),
                        hm_pi(
                            HmBI::Default,
                            "m2",
                            hm_ty(hm_bvar(3), hm_bvar(2)),
                            hm_arrow(
                                hm_pi(
                                    HmBI::Default,
                                    "x",
                                    hm_bvar(3),
                                    hm_pi(
                                        HmBI::Default,
                                        "y",
                                        hm_bvar(4),
                                        hm_eq(
                                            hm_bvar(5),
                                            hm_app2(hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                                            hm_app2(hm_bvar(4), hm_bvar(0), hm_bvar(1)),
                                        ),
                                    ),
                                ),
                                hm_ext_map_eq(
                                    hm_bvar(4),
                                    hm_bvar(3),
                                    hm_app3(
                                        hm_cst("HashMap.mergeWith"),
                                        hm_bvar(2),
                                        hm_bvar(1),
                                        hm_bvar(0),
                                    ),
                                    hm_app3(
                                        hm_cst("HashMap.mergeWith"),
                                        hm_bvar(2),
                                        hm_bvar(0),
                                        hm_bvar(1),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.no_spurious_collision",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k1",
                        hm_bvar(2),
                        hm_pi(
                            HmBI::Default,
                            "k2",
                            hm_bvar(3),
                            hm_pi(
                                HmBI::Default,
                                "v1",
                                hm_bvar(3),
                                hm_pi(
                                    HmBI::Default,
                                    "v2",
                                    hm_bvar(4),
                                    hm_arrow(
                                        hm_arrow(
                                            hm_eq(hm_bvar(6), hm_bvar(3), hm_bvar(2)),
                                            hm_app(hm_cst("False"), hm_prop()),
                                        ),
                                        hm_eq(
                                            hm_option(hm_bvar(4)),
                                            hm_app2(
                                                hm_cst("HashMap.get"),
                                                hm_app3(
                                                    hm_cst("HashMap.insert"),
                                                    hm_app3(
                                                        hm_cst("HashMap.insert"),
                                                        hm_bvar(5),
                                                        hm_bvar(3),
                                                        hm_bvar(1),
                                                    ),
                                                    hm_bvar(2),
                                                    hm_bvar(0),
                                                ),
                                                hm_bvar(3),
                                            ),
                                            hm_app(hm_cst("Option.some"), hm_bvar(1)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.loadFactor",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_nat()),
            ),
        ),
    );
    add(
        "HashMap.capacity",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(hm_ty(hm_bvar(1), hm_bvar(0)), hm_nat()),
            ),
        ),
    );
    add(
        "HashMap.size_le_capacity",
        hm_ext_forall_map(hm_eq(
            hm_bool(),
            hm_app2(
                hm_cst("Nat.ble"),
                hm_app(hm_cst("HashMap.size"), hm_bvar(0)),
                hm_app(hm_cst("HashMap.capacity"), hm_bvar(0)),
            ),
            hm_cst("true"),
        )),
    );
    add(
        "HashMap.ofList",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_list(hm_app2(hm_cst("Prod"), hm_bvar(1), hm_bvar(0))),
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                ),
            ),
        ),
    );
    add(
        "HashMap.toList",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_list(hm_app2(hm_cst("Prod"), hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.toList_length_eq_size",
        hm_ext_forall_map(hm_eq(
            hm_nat(),
            hm_app(
                hm_cst("List.length"),
                hm_app(hm_cst("HashMap.toList"), hm_bvar(0)),
            ),
            hm_app(hm_cst("HashMap.size"), hm_bvar(0)),
        )),
    );
    add(
        "HashMap.ext",
        hm_ext_forall_two_maps(hm_arrow(
            hm_pi(
                HmBI::Default,
                "k",
                hm_bvar(3),
                hm_eq(
                    hm_option(hm_bvar(3)),
                    hm_app2(hm_cst("HashMap.get"), hm_bvar(2), hm_bvar(0)),
                    hm_app2(hm_cst("HashMap.get"), hm_bvar(1), hm_bvar(0)),
                ),
            ),
            hm_ext_map_eq(hm_bvar(3), hm_bvar(2), hm_bvar(1), hm_bvar(0)),
        )),
    );
    add(
        "HashMap.toOrderedList",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_list(hm_app2(hm_cst("Prod"), hm_bvar(1), hm_bvar(0))),
                ),
            ),
        ),
    );
    add(
        "HashMap.ofList_toList_get_eq",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k",
                        hm_bvar(2),
                        hm_eq(
                            hm_option(hm_bvar(1)),
                            hm_app2(
                                hm_cst("HashMap.get"),
                                hm_app(
                                    hm_cst("HashMap.ofList"),
                                    hm_app(hm_cst("HashMap.toList"), hm_bvar(1)),
                                ),
                                hm_bvar(0),
                            ),
                            hm_app2(hm_cst("HashMap.get"), hm_bvar(1), hm_bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.contains",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_arrow(
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_arrow(hm_bvar(1), hm_bool()),
                ),
            ),
        ),
    );
    add(
        "HashMap.contains_insert_same",
        hm_ext_forall_map_k_v(hm_eq(
            hm_bool(),
            hm_app2(
                hm_cst("HashMap.contains"),
                hm_app3(hm_cst("HashMap.insert"), hm_bvar(4), hm_bvar(1), hm_bvar(0)),
                hm_bvar(1),
            ),
            hm_cst("true"),
        )),
    );
    add(
        "HashMap.contains_delete_same",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "k",
                        hm_bvar(2),
                        hm_eq(
                            hm_bool(),
                            hm_app2(
                                hm_cst("HashMap.contains"),
                                hm_app2(hm_cst("HashMap.delete"), hm_bvar(1), hm_bvar(0)),
                                hm_bvar(0),
                            ),
                            hm_cst("false"),
                        ),
                    ),
                ),
            ),
        ),
    );
    add(
        "HashMap.inter_empty_left",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(
                hm_cst("HashMap.inter"),
                hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
                hm_bvar(0),
            ),
            hm_app2(hm_cst("HashMap.empty"), hm_bvar(2), hm_bvar(1)),
        )),
    );
    add(
        "HashMap.inter_self",
        hm_ext_forall_map(hm_ext_map_eq(
            hm_bvar(2),
            hm_bvar(1),
            hm_app2(hm_cst("HashMap.inter"), hm_bvar(0), hm_bvar(0)),
            hm_bvar(0),
        )),
    );
    add(
        "HashMap.union_inter_distrib",
        hm_pi(
            HmBI::Implicit,
            "K",
            hm_type1(),
            hm_pi(
                HmBI::Implicit,
                "V",
                hm_type1(),
                hm_pi(
                    HmBI::Default,
                    "m1",
                    hm_ty(hm_bvar(1), hm_bvar(0)),
                    hm_pi(
                        HmBI::Default,
                        "m2",
                        hm_ty(hm_bvar(2), hm_bvar(1)),
                        hm_pi(
                            HmBI::Default,
                            "m3",
                            hm_ty(hm_bvar(3), hm_bvar(2)),
                            hm_ext_map_eq(
                                hm_bvar(4),
                                hm_bvar(3),
                                hm_app2(
                                    hm_cst("HashMap.union"),
                                    hm_app2(hm_cst("HashMap.inter"), hm_bvar(2), hm_bvar(1)),
                                    hm_app2(hm_cst("HashMap.inter"), hm_bvar(2), hm_bvar(0)),
                                ),
                                hm_app2(
                                    hm_cst("HashMap.inter"),
                                    hm_bvar(2),
                                    hm_app2(hm_cst("HashMap.union"), hm_bvar(1), hm_bvar(0)),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
}
#[cfg(test)]
mod tests_hm_ext {
    use super::*;
    fn setup_hm_ext_env() -> Environment {
        let mut env = Environment::new();
        for (nm, ty) in &[
            ("Nat", hm_type1()),
            ("Bool", hm_type1()),
            ("List", hm_type1()),
            ("Option", hm_type1()),
            ("Prod", hm_type1()),
            ("Eq", hm_prop()),
            ("Iff", hm_prop()),
            ("False", hm_prop()),
            ("id", hm_type1()),
            ("Function.comp", hm_type1()),
        ] {
            env.add(HmDecl::Axiom {
                name: HmName::str(*nm),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .expect("operation should succeed");
        }
        for nm in &[
            "Nat.zero", "Nat.succ", "Nat.add", "Nat.mul", "Nat.sub", "Nat.mod", "Nat.pow",
            "Nat.pred", "Nat.ble", "Nat.lt",
        ] {
            env.add(HmDecl::Axiom {
                name: HmName::str(*nm),
                univ_params: vec![],
                ty: hm_nat(),
            })
            .expect("operation should succeed");
        }
        for nm in &["true", "false"] {
            env.add(HmDecl::Axiom {
                name: HmName::str(*nm),
                univ_params: vec![],
                ty: hm_bool(),
            })
            .expect("operation should succeed");
        }
        for nm in &["List.length", "Option.some", "Option.none"] {
            env.add(HmDecl::Axiom {
                name: HmName::str(*nm),
                univ_params: vec![],
                ty: hm_type1(),
            })
            .expect("operation should succeed");
        }
        for nm in &["HashMap.filter_true_fn", "HashMap.filter_false_fn"] {
            env.add(HmDecl::Axiom {
                name: HmName::str(*nm),
                univ_params: vec![],
                ty: hm_type1(),
            })
            .expect("operation should succeed");
        }
        build_hashmap_env(&mut env).expect("build_hashmap_env should succeed");
        register_hashmap_extended_axioms(&mut env);
        env
    }
    #[test]
    fn test_hm_ext_get_insert_same() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.get_insert_same")));
    }
    #[test]
    fn test_hm_ext_get_insert_diff() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.get_insert_diff")));
    }
    #[test]
    fn test_hm_ext_insert_comm() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.insert_comm")));
    }
    #[test]
    fn test_hm_ext_delete_laws() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.get_delete_same")));
        assert!(env.contains(&HmName::str("HashMap.get_delete_diff")));
        assert!(env.contains(&HmName::str("HashMap.delete_insert_same")));
    }
    #[test]
    fn test_hm_ext_union_laws() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.union_empty_left")));
        assert!(env.contains(&HmName::str("HashMap.union_empty_right")));
        assert!(env.contains(&HmName::str("HashMap.union_assoc")));
        assert!(env.contains(&HmName::str("HashMap.union_get_left")));
        assert!(env.contains(&HmName::str("HashMap.union_idem")));
    }
    #[test]
    fn test_hm_ext_inter_diff() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.inter_comm")));
        assert!(env.contains(&HmName::str("HashMap.diff_empty")));
        assert!(env.contains(&HmName::str("HashMap.diff_self")));
        assert!(env.contains(&HmName::str("HashMap.inter_empty_left")));
        assert!(env.contains(&HmName::str("HashMap.inter_self")));
        assert!(env.contains(&HmName::str("HashMap.union_inter_distrib")));
    }
    #[test]
    fn test_hm_ext_map_fmap() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.map")));
        assert!(env.contains(&HmName::str("HashMap.map_id")));
        assert!(env.contains(&HmName::str("HashMap.map_comp")));
    }
    #[test]
    fn test_hm_ext_filter() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.filter")));
        assert!(env.contains(&HmName::str("HashMap.filter_true")));
        assert!(env.contains(&HmName::str("HashMap.filter_false")));
    }
    #[test]
    fn test_hm_ext_foldl_foldr() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.foldl")));
        assert!(env.contains(&HmName::str("HashMap.foldr")));
        assert!(env.contains(&HmName::str("HashMap.foldl_empty")));
    }
    #[test]
    fn test_hm_ext_cardinality() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.size")));
        assert!(env.contains(&HmName::str("HashMap.size_empty")));
        assert!(env.contains(&HmName::str("HashMap.size_insert_new")));
        assert!(env.contains(&HmName::str("HashMap.size_delete")));
    }
    #[test]
    fn test_hm_ext_keys_values() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.keys")));
        assert!(env.contains(&HmName::str("HashMap.values")));
        assert!(env.contains(&HmName::str("HashMap.keys_length_eq_size")));
        assert!(env.contains(&HmName::str("HashMap.values_length_eq_size")));
    }
    #[test]
    fn test_hm_ext_merge_with() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.mergeWith")));
        assert!(env.contains(&HmName::str("HashMap.mergeWith_comm_when_f_comm")));
    }
    #[test]
    fn test_hm_ext_collision_handling() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.no_spurious_collision")));
    }
    #[test]
    fn test_hm_ext_load_factor() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.loadFactor")));
        assert!(env.contains(&HmName::str("HashMap.capacity")));
        assert!(env.contains(&HmName::str("HashMap.size_le_capacity")));
    }
    #[test]
    fn test_hm_ext_assoclist_iso() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.ofList")));
        assert!(env.contains(&HmName::str("HashMap.toList")));
        assert!(env.contains(&HmName::str("HashMap.toList_length_eq_size")));
        assert!(env.contains(&HmName::str("HashMap.ofList_toList_get_eq")));
    }
    #[test]
    fn test_hm_ext_extensionality() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.ext")));
    }
    #[test]
    fn test_hm_ext_ordered_map() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.toOrderedList")));
    }
    #[test]
    fn test_hm_ext_contains() {
        let env = setup_hm_ext_env();
        assert!(env.contains(&HmName::str("HashMap.contains")));
        assert!(env.contains(&HmName::str("HashMap.contains_insert_same")));
        assert!(env.contains(&HmName::str("HashMap.contains_delete_same")));
    }
    #[test]
    fn test_hm_ext_struct_sizes() {
        let _m: HashMapMonoid<u32, u32> = HashMapMonoid {
            inner: std::collections::HashMap::new(),
        };
        let _d: HashMapDiff<u32, u32> = HashMapDiff {
            added: std::collections::HashMap::new(),
            removed: std::collections::HashSet::new(),
        };
        let _lru: LRUCacheHm<u32, u32> = LRUCacheHm {
            capacity: 16,
            store: std::collections::HashMap::new(),
            order: std::collections::VecDeque::new(),
        };
        let _mm: MultiMapHm<u32, u32> = MultiMapHm {
            inner: std::collections::HashMap::new(),
        };
        let _frozen: FrozenHashMap<u32, u32> = FrozenHashMap {
            inner: std::collections::HashMap::new(),
            frozen: true,
        };
    }
}
