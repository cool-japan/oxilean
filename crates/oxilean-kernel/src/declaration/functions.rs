//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::ReducibilityHint;
use crate::{Expr, Level, Name};

use super::types::{
    AnnotationTable, AxiomVal, BeforeAfter, BiMap, ConstantInfo, ConstantKind, ConstantVal,
    ConstructorVal, DeclAttr, DeclDependencies, DeclFilter, DeclIndex, DeclKind, DeclSignature,
    DeclVisibility, DefinitionSafety, DefinitionVal, DiagMeta, EventCounter, FrequencyTable,
    Generation, IdDispenser, InductiveVal, IntervalSet, LoopClock, MemoSlot, QuotKind, QuotVal,
    RecursorVal, RingBuffer, ScopeStack, SeqNum, SimpleLruCache, Slot, SparseBitSet,
    StringInterner, Timestamp, TypedId, WorkQueue, WorkStack,
};

/// Instantiate universe level parameters in an expression.
///
/// Replaces `Level::Param(name)` with the corresponding level from
/// the substitution, based on the parameter name list.
pub fn instantiate_level_params(expr: &Expr, param_names: &[Name], levels: &[Level]) -> Expr {
    if param_names.is_empty() || levels.is_empty() {
        return expr.clone();
    }
    instantiate_level_params_core(expr, param_names, levels)
}
fn instantiate_level_params_core(expr: &Expr, param_names: &[Name], levels: &[Level]) -> Expr {
    match expr {
        Expr::Sort(l) => {
            let new_l = instantiate_level_param(l, param_names, levels);
            Expr::Sort(new_l)
        }
        Expr::Const(name, ls) => {
            let new_ls: Vec<Level> = ls
                .iter()
                .map(|l| instantiate_level_param(l, param_names, levels))
                .collect();
            Expr::Const(name.clone(), new_ls)
        }
        Expr::App(f, a) => {
            let f_new = instantiate_level_params_core(f, param_names, levels);
            let a_new = instantiate_level_params_core(a, param_names, levels);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = instantiate_level_params_core(ty, param_names, levels);
            let body_new = instantiate_level_params_core(body, param_names, levels);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = instantiate_level_params_core(ty, param_names, levels);
            let body_new = instantiate_level_params_core(body, param_names, levels);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = instantiate_level_params_core(ty, param_names, levels);
            let val_new = instantiate_level_params_core(val, param_names, levels);
            let body_new = instantiate_level_params_core(body, param_names, levels);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = instantiate_level_params_core(e, param_names, levels);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
        Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => expr.clone(),
    }
}
/// Instantiate a universe level parameter.
fn instantiate_level_param(level: &Level, param_names: &[Name], levels: &[Level]) -> Level {
    match level {
        Level::Param(name) => {
            for (i, pn) in param_names.iter().enumerate() {
                if pn == name {
                    if let Some(l) = levels.get(i) {
                        return l.clone();
                    }
                }
            }
            level.clone()
        }
        Level::Succ(l) => Level::succ(instantiate_level_param(l, param_names, levels)),
        Level::Max(l1, l2) => Level::max(
            instantiate_level_param(l1, param_names, levels),
            instantiate_level_param(l2, param_names, levels),
        ),
        Level::IMax(l1, l2) => Level::imax(
            instantiate_level_param(l1, param_names, levels),
            instantiate_level_param(l2, param_names, levels),
        ),
        Level::Zero | Level::MVar(_) => level.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinderInfo;
    #[test]
    fn test_constant_info_accessors() {
        let info = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("propext"),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        });
        assert_eq!(info.name(), &Name::str("propext"));
        assert!(info.is_axiom());
        assert!(!info.is_unsafe());
        assert!(!info.has_value(false));
    }
    #[test]
    fn test_definition_val() {
        let info = ConstantInfo::Definition(DefinitionVal {
            common: ConstantVal {
                name: Name::str("id"),
                level_params: vec![Name::str("u")],
                ty: Expr::Sort(Level::param(Name::str("u"))),
            },
            value: Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::BVar(0)),
            ),
            hints: ReducibilityHint::Abbrev,
            safety: DefinitionSafety::Safe,
            all: vec![Name::str("id")],
        });
        assert!(info.is_definition());
        assert!(info.has_value(false));
        assert_eq!(info.reducibility_hint(), ReducibilityHint::Abbrev);
    }
    #[test]
    fn test_recursor_major_idx() {
        let rec = RecursorVal {
            common: ConstantVal {
                name: Name::str("Nat.rec"),
                level_params: vec![Name::str("u")],
                ty: Expr::Sort(Level::zero()),
            },
            all: vec![Name::str("Nat")],
            num_params: 0,
            num_indices: 0,
            num_motives: 1,
            num_minors: 2,
            rules: vec![],
            k: false,
            is_unsafe: false,
        };
        assert_eq!(rec.get_major_idx(), 3);
    }
    #[test]
    fn test_constructor_val() {
        let info = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Nat.succ"),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            induct: Name::str("Nat"),
            cidx: 1,
            num_params: 0,
            num_fields: 1,
            is_unsafe: false,
        });
        assert!(info.is_constructor());
        let ctor = info.to_constructor_val().expect("ctor should be present");
        assert_eq!(ctor.induct, Name::str("Nat"));
        assert_eq!(ctor.cidx, 1);
        assert_eq!(ctor.num_fields, 1);
    }
    #[test]
    fn test_inductive_val() {
        let info = ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Nat"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
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
        assert!(info.is_inductive());
        assert!(!info.is_structure_like());
        let ind = info.to_inductive_val().expect("ind should be present");
        assert_eq!(ind.ctors.len(), 2);
    }
    #[test]
    fn test_structure_like() {
        let info = ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Prod"),
                level_params: vec![Name::str("u"), Name::str("v")],
                ty: Expr::Sort(Level::zero()),
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
        assert!(info.is_structure_like());
    }
    #[test]
    fn test_instantiate_level_params() {
        let expr = Expr::Sort(Level::param(Name::str("u")));
        let result =
            instantiate_level_params(&expr, &[Name::str("u")], &[Level::succ(Level::zero())]);
        assert_eq!(result, Expr::Sort(Level::succ(Level::zero())));
    }
    #[test]
    fn test_instantiate_level_params_const() {
        let expr = Expr::Const(Name::str("List"), vec![Level::param(Name::str("u"))]);
        let result = instantiate_level_params(&expr, &[Name::str("u")], &[Level::zero()]);
        assert_eq!(result, Expr::Const(Name::str("List"), vec![Level::zero()]));
    }
    #[test]
    fn test_quot_val() {
        let info = ConstantInfo::Quotient(QuotVal {
            common: ConstantVal {
                name: Name::str("Quot"),
                level_params: vec![Name::str("u")],
                ty: Expr::Sort(Level::zero()),
            },
            kind: QuotKind::Type,
        });
        assert!(info.is_quotient());
        let qv = info.to_quotient_val().expect("qv should be present");
        assert_eq!(qv.kind, QuotKind::Type);
    }
}
/// Collect all `Level::Param` names referenced in an expression.
pub fn collect_level_params_in_expr(e: &Expr) -> Vec<Name> {
    let mut result = Vec::new();
    collect_level_params_in_expr_impl(e, &mut result);
    result
}
fn collect_level_params_in_expr_impl(e: &Expr, out: &mut Vec<Name>) {
    match e {
        Expr::Sort(level) => collect_level_params_in_level(level, out),
        Expr::Const(_, levels) => {
            for l in levels {
                collect_level_params_in_level(l, out);
            }
        }
        Expr::App(f, a) => {
            collect_level_params_in_expr_impl(f, out);
            collect_level_params_in_expr_impl(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_level_params_in_expr_impl(ty, out);
            collect_level_params_in_expr_impl(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_level_params_in_expr_impl(ty, out);
            collect_level_params_in_expr_impl(val, out);
            collect_level_params_in_expr_impl(body, out);
        }
        _ => {}
    }
}
fn collect_level_params_in_level(l: &Level, out: &mut Vec<Name>) {
    match l {
        Level::Param(n) => {
            if !out.contains(n) {
                out.push(n.clone());
            }
        }
        Level::Succ(inner) => collect_level_params_in_level(inner, out),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_params_in_level(a, out);
            collect_level_params_in_level(b, out);
        }
        _ => {}
    }
}
/// Check that all level params referenced in `e` are in the declared list.
pub fn check_level_params_consistent(e: &Expr, declared: &[Name]) -> Result<(), Name> {
    let found = collect_level_params_in_expr(e);
    for p in found {
        if !declared.contains(&p) {
            return Err(p);
        }
    }
    Ok(())
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    fn mk_axiom(name: &str) -> ConstantInfo {
        ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        })
    }
    fn mk_poly_axiom(name: &str, params: Vec<&str>) -> ConstantInfo {
        ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: params.into_iter().map(Name::str).collect(),
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        })
    }
    #[test]
    fn test_constant_kind_axiom() {
        let info = mk_axiom("Foo");
        assert_eq!(info.kind(), ConstantKind::Axiom);
    }
    #[test]
    fn test_kind_as_str() {
        assert_eq!(ConstantKind::Axiom.as_str(), "axiom");
        assert_eq!(ConstantKind::Definition.as_str(), "definition");
        assert_eq!(ConstantKind::Theorem.as_str(), "theorem");
        assert_eq!(ConstantKind::Inductive.as_str(), "inductive");
        assert_eq!(ConstantKind::Constructor.as_str(), "constructor");
        assert_eq!(ConstantKind::Recursor.as_str(), "recursor");
        assert_eq!(ConstantKind::Quotient.as_str(), "quotient");
    }
    #[test]
    fn test_kind_has_body() {
        assert!(ConstantKind::Definition.has_body());
        assert!(ConstantKind::Theorem.has_body());
        assert!(ConstantKind::Opaque.has_body());
        assert!(!ConstantKind::Axiom.has_body());
        assert!(!ConstantKind::Inductive.has_body());
    }
    #[test]
    fn test_kind_is_inductive_family() {
        assert!(ConstantKind::Inductive.is_inductive_family());
        assert!(ConstantKind::Constructor.is_inductive_family());
        assert!(ConstantKind::Recursor.is_inductive_family());
        assert!(!ConstantKind::Axiom.is_inductive_family());
        assert!(!ConstantKind::Definition.is_inductive_family());
    }
    #[test]
    fn test_num_level_params() {
        let mono = mk_axiom("Foo");
        assert_eq!(mono.num_level_params(), 0);
        assert!(!mono.is_polymorphic());
        let poly = mk_poly_axiom("Bar", vec!["u", "v"]);
        assert_eq!(poly.num_level_params(), 2);
        assert!(poly.is_polymorphic());
    }
    #[test]
    fn test_summarize() {
        let info = mk_poly_axiom("MyAxiom", vec!["u"]);
        let s = info.summarize();
        assert_eq!(s.name, Name::str("MyAxiom"));
        assert_eq!(s.kind, ConstantKind::Axiom);
        assert_eq!(s.num_level_params, 1);
        assert!(s.is_polymorphic);
    }
    #[test]
    fn test_display_string_monomorphic() {
        let info = mk_axiom("Foo");
        let s = info.display_string();
        assert!(s.contains("axiom"));
        assert!(s.contains("Foo"));
    }
    #[test]
    fn test_display_string_polymorphic() {
        let info = mk_poly_axiom("Bar", vec!["u", "v"]);
        let s = info.display_string();
        assert!(s.contains("axiom"));
        assert!(s.contains("Bar"));
        assert!(s.contains("u"));
    }
    #[test]
    fn test_definition_safety() {
        assert!(DefinitionSafety::Safe.is_safe());
        assert!(!DefinitionSafety::Unsafe.is_safe());
        assert!(DefinitionSafety::Partial.is_partial());
        assert!(!DefinitionSafety::Safe.is_partial());
        assert_eq!(DefinitionSafety::Safe.as_str(), "safe");
        assert_eq!(DefinitionSafety::Unsafe.as_str(), "unsafe");
        assert_eq!(DefinitionSafety::Partial.as_str(), "partial");
    }
    #[test]
    fn test_collect_level_params_in_sort() {
        let e = Expr::Sort(Level::Param(Name::str("u")));
        let params = collect_level_params_in_expr(&e);
        assert_eq!(params, vec![Name::str("u")]);
    }
    #[test]
    fn test_collect_level_params_in_const() {
        let e = Expr::Const(
            Name::str("List"),
            vec![Level::Param(Name::str("u")), Level::zero()],
        );
        let params = collect_level_params_in_expr(&e);
        assert!(params.contains(&Name::str("u")));
        assert_eq!(params.len(), 1);
    }
    #[test]
    fn test_check_level_params_consistent_ok() {
        let e = Expr::Sort(Level::Param(Name::str("u")));
        assert!(check_level_params_consistent(&e, &[Name::str("u")]).is_ok());
    }
    #[test]
    fn test_check_level_params_consistent_fail() {
        let e = Expr::Sort(Level::Param(Name::str("v")));
        let result = check_level_params_consistent(&e, &[Name::str("u")]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Name::str("v"));
    }
    #[test]
    fn test_parent_inductive() {
        let info = mk_axiom("Foo");
        assert!(info.parent_inductive().is_none());
    }
}
#[cfg(test)]
mod tests_declaration_extra {
    use super::*;
    #[test]
    fn test_decl_kind() {
        assert!(DeclKind::Theorem.has_body());
        assert!(!DeclKind::Axiom.has_body());
        assert!(DeclKind::Inductive.is_type_decl());
        assert!(!DeclKind::Theorem.is_type_decl());
        assert_eq!(DeclKind::Theorem.label(), "theorem");
    }
    #[test]
    fn test_decl_visibility() {
        assert!(DeclVisibility::Public.is_externally_visible());
        assert!(!DeclVisibility::Private.is_externally_visible());
    }
    #[test]
    fn test_decl_attr() {
        let a = DeclAttr::Simp;
        assert_eq!(a.name(), "simp");
        let u = DeclAttr::Unknown("myattr".into());
        assert_eq!(u.name(), "myattr");
    }
    #[test]
    fn test_decl_signature() {
        let sig = DeclSignature::new("Nat.succ", "Nat → Nat").with_uparam("u");
        assert_eq!(sig.name, "Nat.succ");
        assert!(sig.is_universe_poly());
        assert_eq!(sig.uparams, vec!["u".to_string()]);
    }
    #[test]
    fn test_decl_index() {
        let mut idx = DeclIndex::new();
        idx.insert("Nat.succ", 0);
        idx.insert("Nat.zero", 1);
        assert_eq!(idx.get("Nat.succ"), Some(0));
        assert_eq!(idx.get("missing"), None);
        assert_eq!(idx.len(), 2);
    }
    #[test]
    fn test_decl_dependencies() {
        let mut deps = DeclDependencies::new(3);
        deps.add(1, 0);
        deps.add(2, 0);
        deps.add(2, 1);
        assert!(deps.directly_depends(1, 0));
        assert!(!deps.directly_depends(0, 1));
        assert_eq!(deps.total_edges(), 3);
    }
}
#[cfg(test)]
mod tests_decl_filter {
    use super::*;
    #[test]
    fn test_decl_filter() {
        let filter = DeclFilter::accept_all()
            .with_kinds(vec![DeclKind::Theorem, DeclKind::Axiom])
            .in_namespace("Nat");
        let sig = DeclSignature::new("Nat.zero_add", "∀ n, 0 + n = n");
        assert!(filter.accepts(&sig, DeclKind::Theorem, &[]));
        let other = DeclSignature::new("List.length", "List α → Nat");
        assert!(!filter.accepts(&other, DeclKind::Definition, &[]));
    }
    #[test]
    fn test_decl_filter_attr() {
        let filter = DeclFilter::accept_all().with_attr(DeclAttr::Simp);
        let sig = DeclSignature::new("Foo.bar", "Prop");
        assert!(!filter.accepts(&sig, DeclKind::Theorem, &[DeclAttr::Inline]));
        assert!(filter.accepts(&sig, DeclKind::Theorem, &[DeclAttr::Simp]));
    }
}
#[cfg(test)]
mod tests_common_infra {
    use super::*;
    #[test]
    fn test_event_counter() {
        let mut ec = EventCounter::new();
        ec.inc("hit");
        ec.inc("hit");
        ec.inc("miss");
        assert_eq!(ec.get("hit"), 2);
        assert_eq!(ec.get("miss"), 1);
        assert_eq!(ec.total(), 3);
        ec.reset();
        assert_eq!(ec.total(), 0);
    }
    #[test]
    fn test_diag_meta() {
        let mut m = DiagMeta::new();
        m.add("os", "linux");
        m.add("arch", "x86_64");
        assert_eq!(m.get("os"), Some("linux"));
        assert_eq!(m.len(), 2);
        let s = m.to_string();
        assert!(s.contains("os=linux"));
    }
    #[test]
    fn test_scope_stack() {
        let mut ss = ScopeStack::new();
        ss.push("Nat");
        ss.push("succ");
        assert_eq!(ss.current(), Some("succ"));
        assert_eq!(ss.depth(), 2);
        assert_eq!(ss.path(), "Nat.succ");
        ss.pop();
        assert_eq!(ss.current(), Some("Nat"));
    }
    #[test]
    fn test_annotation_table() {
        let mut tbl = AnnotationTable::new();
        tbl.annotate("doc", "first line");
        tbl.annotate("doc", "second line");
        assert_eq!(tbl.get_all("doc").len(), 2);
        assert!(tbl.has("doc"));
        assert!(!tbl.has("other"));
    }
    #[test]
    fn test_work_stack() {
        let mut ws = WorkStack::new();
        ws.push(1u32);
        ws.push(2u32);
        assert_eq!(ws.pop(), Some(2));
        assert_eq!(ws.len(), 1);
    }
    #[test]
    fn test_work_queue() {
        let mut wq = WorkQueue::new();
        wq.enqueue(1u32);
        wq.enqueue(2u32);
        assert_eq!(wq.dequeue(), Some(1));
        assert_eq!(wq.len(), 1);
    }
    #[test]
    fn test_sparse_bit_set() {
        let mut bs = SparseBitSet::new(128);
        bs.set(5);
        bs.set(63);
        bs.set(64);
        assert!(bs.get(5));
        assert!(bs.get(63));
        assert!(bs.get(64));
        assert!(!bs.get(0));
        assert_eq!(bs.count_ones(), 3);
        bs.clear(5);
        assert!(!bs.get(5));
    }
    #[test]
    fn test_loop_clock() {
        let mut clk = LoopClock::start();
        for _ in 0..10 {
            clk.tick();
        }
        assert_eq!(clk.iters(), 10);
        assert!(clk.elapsed_us() >= 0.0);
    }
}
#[cfg(test)]
mod tests_extra_data_structures {
    use super::*;
    #[test]
    fn test_simple_lru_cache() {
        let mut cache: SimpleLruCache<&str, u32> = SimpleLruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("d", 4);
        assert!(cache.len() <= 3);
    }
    #[test]
    fn test_string_interner() {
        let mut si = StringInterner::new();
        let id1 = si.intern("hello");
        let id2 = si.intern("hello");
        assert_eq!(id1, id2);
        let id3 = si.intern("world");
        assert_ne!(id1, id3);
        assert_eq!(si.get(id1), Some("hello"));
        assert_eq!(si.len(), 2);
    }
    #[test]
    fn test_frequency_table() {
        let mut ft = FrequencyTable::new();
        ft.record("a");
        ft.record("b");
        ft.record("a");
        ft.record("a");
        assert_eq!(ft.freq(&"a"), 3);
        assert_eq!(ft.freq(&"b"), 1);
        assert_eq!(ft.most_frequent(), Some((&"a", 3)));
        assert_eq!(ft.total(), 4);
        assert_eq!(ft.distinct(), 2);
    }
    #[test]
    fn test_bimap() {
        let mut bm: BiMap<u32, &str> = BiMap::new();
        bm.insert(1, "one");
        bm.insert(2, "two");
        assert_eq!(bm.get_b(&1), Some(&"one"));
        assert_eq!(bm.get_a(&"two"), Some(&2));
        assert_eq!(bm.len(), 2);
    }
}
#[cfg(test)]
mod tests_interval_set {
    use super::*;
    #[test]
    fn test_interval_set() {
        let mut s = IntervalSet::new();
        s.add(1, 5);
        s.add(3, 8);
        assert_eq!(s.num_intervals(), 1);
        assert_eq!(s.cardinality(), 8);
        assert!(s.contains(4));
        assert!(!s.contains(9));
        s.add(10, 15);
        assert_eq!(s.num_intervals(), 2);
    }
}
/// Returns the current timestamp.
#[allow(dead_code)]
pub fn now_us() -> Timestamp {
    let us = std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0);
    Timestamp::from_us(us)
}
#[cfg(test)]
mod tests_typed_utilities {
    use super::*;
    #[test]
    fn test_timestamp() {
        let t1 = Timestamp::from_us(1000);
        let t2 = Timestamp::from_us(1500);
        assert_eq!(t2.elapsed_since(t1), 500);
        assert!(t1 < t2);
    }
    #[test]
    fn test_typed_id() {
        struct Foo;
        let id: TypedId<Foo> = TypedId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(format!("{id}"), "#42");
    }
    #[test]
    fn test_id_dispenser() {
        struct Bar;
        let mut disp: IdDispenser<Bar> = IdDispenser::new();
        let a = disp.next();
        let b = disp.next();
        assert_eq!(a.raw(), 0);
        assert_eq!(b.raw(), 1);
        assert_eq!(disp.count(), 2);
    }
    #[test]
    fn test_slot() {
        let mut slot: Slot<u32> = Slot::empty();
        assert!(!slot.is_filled());
        slot.fill(99);
        assert!(slot.is_filled());
        assert_eq!(slot.get(), Some(&99));
        let v = slot.take();
        assert_eq!(v, Some(99));
        assert!(!slot.is_filled());
    }
    #[test]
    #[should_panic]
    fn test_slot_double_fill() {
        let mut slot: Slot<u32> = Slot::empty();
        slot.fill(1);
        slot.fill(2);
    }
    #[test]
    fn test_memo_slot() {
        let mut ms: MemoSlot<u32> = MemoSlot::new();
        assert!(!ms.is_cached());
        let val = ms.get_or_compute(|| 42);
        assert_eq!(*val, 42);
        assert!(ms.is_cached());
        ms.invalidate();
        assert!(!ms.is_cached());
    }
}
#[cfg(test)]
mod tests_ring_buffer {
    use super::*;
    #[test]
    fn test_ring_buffer() {
        let mut rb = RingBuffer::new(3);
        rb.push(1u32);
        rb.push(2u32);
        rb.push(3u32);
        assert!(rb.is_full());
        rb.push(4u32);
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.len(), 2);
    }
    #[test]
    fn test_before_after() {
        let ba = BeforeAfter::new(10u32, 10u32);
        assert!(ba.unchanged());
        let ba2 = BeforeAfter::new(10u32, 20u32);
        assert!(!ba2.unchanged());
    }
    #[test]
    fn test_seq_num() {
        let s = SeqNum::ZERO;
        assert_eq!(s.value(), 0);
        let s2 = s.next();
        assert_eq!(s2.value(), 1);
        assert!(s < s2);
    }
    #[test]
    fn test_generation() {
        let g0 = Generation::INITIAL;
        let g1 = g0.advance();
        assert_eq!(g0.number(), 0);
        assert_eq!(g1.number(), 1);
        assert_ne!(g0, g1);
    }
}
