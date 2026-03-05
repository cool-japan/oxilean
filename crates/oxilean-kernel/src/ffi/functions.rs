//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::HashMap;

use super::types::{
    BuiltinExterns, CallingConvention, ConfigNode, DecisionNode, Either2, ExternDecl,
    ExternRegistry, FfiError, FfiSafety, FfiSignature, FfiType, FfiValue, Fixture,
    FlatSubstitution, FocusStack, LabelSet, LibraryManifest, LibraryVersion, MinHeap, NonEmptyVec,
    PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, SymbolMetadata, TokenBucket,
    TransformStat, TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ffi_type_display() {
        assert_eq!(FfiType::Bool.to_string(), "bool");
        assert_eq!(FfiType::UInt64.to_string(), "u64");
        assert_eq!(FfiType::Int32.to_string(), "i32");
        assert_eq!(FfiType::Float64.to_string(), "f64");
        assert_eq!(FfiType::String.to_string(), "string");
        assert_eq!(FfiType::Unit.to_string(), "()");
    }
    #[test]
    fn test_ffi_type_is_ffi_safe() {
        assert!(FfiType::Bool.is_ffi_safe());
        assert!(FfiType::UInt64.is_ffi_safe());
        assert!(FfiType::String.is_ffi_safe());
        assert!(FfiType::Ptr(Box::new(FfiType::UInt32)).is_ffi_safe());
        assert!(!FfiType::OxiLean("Expr".to_string()).is_ffi_safe());
    }
    #[test]
    fn test_ffi_type_size_bytes() {
        assert_eq!(FfiType::Bool.size_bytes(), Some(1));
        assert_eq!(FfiType::UInt8.size_bytes(), Some(1));
        assert_eq!(FfiType::UInt16.size_bytes(), Some(2));
        assert_eq!(FfiType::UInt32.size_bytes(), Some(4));
        assert_eq!(FfiType::UInt64.size_bytes(), Some(8));
        assert_eq!(FfiType::Float32.size_bytes(), Some(4));
        assert_eq!(FfiType::Float64.size_bytes(), Some(8));
        assert_eq!(FfiType::Unit.size_bytes(), Some(0));
        assert_eq!(FfiType::String.size_bytes(), None);
    }
    #[test]
    fn test_ffi_safety_display() {
        assert_eq!(FfiSafety::Safe.to_string(), "safe");
        assert_eq!(FfiSafety::Unsafe.to_string(), "unsafe");
        assert_eq!(FfiSafety::System.to_string(), "system");
    }
    #[test]
    fn test_calling_convention_display() {
        assert_eq!(CallingConvention::Rust.to_string(), "rust");
        assert_eq!(CallingConvention::C.to_string(), "c");
        assert_eq!(CallingConvention::System.to_string(), "system");
    }
    #[test]
    fn test_ffi_value_display() {
        assert_eq!(FfiValue::Bool(true).to_string(), "true");
        assert_eq!(FfiValue::UInt(42).to_string(), "42");
        assert_eq!(FfiValue::Int(-42).to_string(), "-42");
        assert_eq!(FfiValue::Str("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(FfiValue::Unit.to_string(), "()");
    }
    #[test]
    fn test_ffi_value_to_expr() {
        let bool_val = FfiValue::Bool(true);
        let _expr = bool_val.to_expr();
        let uint_val = FfiValue::UInt(42);
        let expr = uint_val.to_expr();
        match expr {
            Expr::Lit(crate::Literal::Nat(n)) => assert_eq!(n, 42),
            _ => panic!("Expected Nat literal"),
        }
    }
    #[test]
    fn test_ffi_error_display() {
        let err = FfiError::SymbolNotFound("strlen".to_string());
        assert!(err.to_string().contains("strlen"));
        let err = FfiError::LibraryNotFound("libc".to_string());
        assert!(err.to_string().contains("libc"));
        let err = FfiError::DuplicateSymbol("builtin_print".to_string());
        assert!(err.to_string().contains("builtin_print"));
    }
    #[test]
    fn test_ffi_signature_new() {
        let sig = FfiSignature::new(
            vec![FfiType::String, FfiType::UInt64],
            Box::new(FfiType::Int32),
        );
        assert_eq!(sig.params.len(), 2);
        assert_eq!(*sig.ret_type, FfiType::Int32);
    }
    #[test]
    fn test_ffi_signature_validate_safe() {
        let sig = FfiSignature::new(
            vec![FfiType::UInt64, FfiType::String],
            Box::new(FfiType::Int32),
        );
        assert!(sig.validate().is_ok());
    }
    #[test]
    fn test_ffi_signature_validate_unsafe() {
        let sig = FfiSignature::new(
            vec![FfiType::OxiLean("Expr".to_string())],
            Box::new(FfiType::Int32),
        );
        assert!(sig.validate().is_err());
    }
    #[test]
    fn test_extern_decl_new() {
        let decl = ExternDecl::new(
            Name::str("my_strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        assert_eq!(decl.lib_name, "libc");
        assert_eq!(decl.symbol_name, "strlen");
        assert_eq!(decl.safety, FfiSafety::Unsafe);
    }
    #[test]
    fn test_extern_decl_validate_valid() {
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        assert!(decl.validate().is_ok());
    }
    #[test]
    fn test_extern_decl_validate_invalid_lib() {
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        assert!(decl.validate().is_err());
    }
    #[test]
    fn test_extern_registry_new() {
        let registry = ExternRegistry::new();
        assert_eq!(registry.count(), 0);
    }
    #[test]
    fn test_extern_registry_register() {
        let mut registry = ExternRegistry::new();
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        assert!(registry.register(decl).is_ok());
        assert_eq!(registry.count(), 1);
    }
    #[test]
    fn test_extern_registry_duplicate_symbol() {
        let mut registry = ExternRegistry::new();
        let decl1 = ExternDecl::new(
            Name::str("strlen1"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        let decl2 = ExternDecl::new(
            Name::str("strlen2"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        assert!(registry.register(decl1).is_ok());
        assert!(registry.register(decl2).is_err());
    }
    #[test]
    fn test_extern_registry_lookup() {
        let mut registry = ExternRegistry::new();
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        registry.register(decl).expect("value should be present");
        let found = registry.lookup(&Name::str("strlen"));
        assert!(found.is_ok());
    }
    #[test]
    fn test_extern_registry_lookup_not_found() {
        let registry = ExternRegistry::new();
        let found = registry.lookup(&Name::str("nonexistent"));
        assert!(found.is_err());
    }
    #[test]
    fn test_extern_registry_lookup_by_symbol() {
        let mut registry = ExternRegistry::new();
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        registry.register(decl).expect("value should be present");
        let found = registry.lookup_by_symbol("libc", "strlen");
        assert!(found.is_ok());
    }
    #[test]
    fn test_extern_registry_validate_all() {
        let mut registry = ExternRegistry::new();
        let decl = ExternDecl::new(
            Name::str("strlen"),
            Expr::Const(Name::str("String"), vec![]),
            "libc".to_string(),
            "strlen".to_string(),
            FfiSafety::Unsafe,
            CallingConvention::C,
            FfiSignature::new(vec![FfiType::String], Box::new(FfiType::UInt64)),
        );
        registry.register(decl).expect("value should be present");
        assert!(registry.validate_all().is_ok());
    }
    #[test]
    fn test_builtin_externs_register() {
        let mut registry = ExternRegistry::new();
        assert!(BuiltinExterns::register_builtins(&mut registry).is_ok());
        assert!(registry.count() > 0);
    }
    #[test]
    fn test_builtin_externs_io() {
        let mut registry = ExternRegistry::new();
        BuiltinExterns::register_io(&mut registry).expect("value should be present");
        assert!(registry.lookup(&Name::str("builtin_print")).is_ok());
    }
    #[test]
    fn test_builtin_externs_string() {
        let mut registry = ExternRegistry::new();
        BuiltinExterns::register_string(&mut registry).expect("value should be present");
        assert!(registry.lookup(&Name::str("builtin_strlen")).is_ok());
    }
    #[test]
    fn test_builtin_externs_arithmetic() {
        let mut registry = ExternRegistry::new();
        BuiltinExterns::register_arithmetic(&mut registry).expect("value should be present");
        assert!(registry.lookup(&Name::str("builtin_abs")).is_ok());
    }
    #[test]
    fn test_ffi_type_ptr_display() {
        let ptr_type = FfiType::Ptr(Box::new(FfiType::UInt32));
        assert_eq!(ptr_type.to_string(), "*u32");
    }
    #[test]
    fn test_ffi_type_fn_display() {
        let fn_type = FfiType::Fn(
            vec![FfiType::UInt64, FfiType::String],
            Box::new(FfiType::Int32),
        );
        assert_eq!(fn_type.to_string(), "fn(u64, string) -> i32");
    }
}
#[cfg(test)]
mod extra_ffi_tests {
    use super::*;
    #[test]
    fn test_library_version_new() {
        let v = LibraryVersion::new("libc", 2, 31, 0);
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 31);
        assert_eq!(v.patch, 0);
    }
    #[test]
    fn test_library_version_at_least() {
        let v = LibraryVersion::new("libc", 2, 31, 0);
        assert!(v.at_least(2, 31, 0));
        assert!(v.at_least(2, 30, 0));
        assert!(!v.at_least(2, 32, 0));
    }
    #[test]
    fn test_library_version_display() {
        let v = LibraryVersion::new("libm", 1, 2, 3);
        assert_eq!(format!("{}", v), "libm 1.2.3");
    }
    #[test]
    fn test_library_manifest_new() {
        let m = LibraryManifest::new();
        assert!(m.is_empty());
    }
    #[test]
    fn test_library_manifest_require() {
        let mut m = LibraryManifest::new();
        m.require(LibraryVersion::new("libc", 2, 31, 0));
        assert_eq!(m.len(), 1);
        assert!(m.requires_lib("libc"));
    }
    #[test]
    fn test_library_manifest_not_required() {
        let m = LibraryManifest::new();
        assert!(!m.requires_lib("libz"));
    }
    #[test]
    fn test_symbol_metadata_new() {
        let sm = SymbolMetadata::new("strlen", "libc");
        assert_eq!(sm.symbol, "strlen");
        assert_eq!(sm.library, "libc");
        assert!(!sm.weak);
        assert!(!sm.thread_local);
    }
    #[test]
    fn test_symbol_metadata_weak() {
        let sm = SymbolMetadata::new("optional_fn", "libopt").with_weak();
        assert!(sm.weak);
    }
    #[test]
    fn test_symbol_metadata_thread_local() {
        let sm = SymbolMetadata::new("tls_var", "libx").with_thread_local();
        assert!(sm.thread_local);
    }
    #[test]
    fn test_symbol_metadata_display() {
        let sm = SymbolMetadata::new("puts", "libc").with_weak();
        let s = format!("{}", sm);
        assert!(s.contains("libc::puts"));
        assert!(s.contains("weak"));
    }
    #[test]
    fn test_ffi_value_try_from_expr_nat() {
        let expr = Expr::Lit(crate::Literal::Nat(42));
        let v = FfiValue::try_from_expr(&expr, &FfiType::UInt64);
        assert!(v.is_ok());
        assert_eq!(v.expect("v should be valid"), FfiValue::UInt(42));
    }
    #[test]
    fn test_ffi_value_try_from_expr_str() {
        let expr = Expr::Lit(crate::Literal::Str("hello".into()));
        let v = FfiValue::try_from_expr(&expr, &FfiType::String);
        assert!(v.is_ok());
    }
    #[test]
    fn test_ffi_value_try_from_expr_mismatch() {
        let expr = Expr::Sort(crate::Level::zero());
        let v = FfiValue::try_from_expr(&expr, &FfiType::UInt64);
        assert!(v.is_err());
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
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
