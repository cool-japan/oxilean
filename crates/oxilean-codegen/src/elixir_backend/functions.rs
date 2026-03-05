//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    ElixirBackend, ElixirExpr, ElixirExtCache, ElixirExtConstFolder, ElixirExtDepGraph,
    ElixirExtDomTree, ElixirExtLiveness, ElixirExtPassConfig, ElixirExtPassPhase,
    ElixirExtPassRegistry, ElixirExtPassStats, ElixirExtWorklist, ElixirFunction, ElixirModule,
    ElixirX2Cache, ElixirX2ConstFolder, ElixirX2DepGraph, ElixirX2DomTree, ElixirX2Liveness,
    ElixirX2PassConfig, ElixirX2PassPhase, ElixirX2PassRegistry, ElixirX2PassStats,
    ElixirX2Worklist, ElxAnalysisCache, ElxConstantFoldingHelper, ElxDepGraph, ElxDominatorTree,
    ElxLivenessInfo, ElxPassConfig, ElxPassPhase, ElxPassRegistry, ElxPassStats, ElxWorklist,
};

/// Convert a `CamelCase` or `mixedCase` identifier to `snake_case`.
pub(super) fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_upper = false;
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper {
                out.push('_');
            }
            out.extend(c.to_lowercase());
            prev_upper = true;
        } else {
            out.push(c);
            prev_upper = false;
        }
    }
    out
}
/// Return `true` if `name` is an Elixir reserved word.
pub(super) fn is_elixir_reserved(name: &str) -> bool {
    matches!(
        name,
        "do" | "end"
            | "fn"
            | "in"
            | "nil"
            | "true"
            | "false"
            | "when"
            | "and"
            | "or"
            | "not"
            | "if"
            | "else"
            | "cond"
            | "case"
            | "receive"
            | "after"
            | "for"
            | "try"
            | "catch"
            | "rescue"
            | "with"
            | "import"
            | "use"
            | "require"
            | "alias"
            | "defmodule"
            | "def"
            | "defp"
            | "defmacro"
            | "defmacrop"
            | "defstruct"
            | "defprotocol"
            | "defimpl"
            | "defdelegate"
    )
}
/// Escape special characters inside an Elixir double-quoted string.
pub(super) fn escape_elixir_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
pub const ELIXIR_RUNTIME: &str = r#"defmodule OxiLean.Runtime do
  @moduledoc """
  OxiLean runtime support for Elixir-compiled code.

  Provides:
  - Algebraic-data-type helpers (tagged tuples)
  - Basic numeric utilities
  - Functional combinators
  """

  # ---- ADT constructors -------------------------------------------------

  @doc "Wrap a value in a tagged tuple for algebraic data types."
  def adt(tag, fields) when is_atom(tag) do
    List.to_tuple([tag | fields])
  end

  @doc "Extract the tag from an ADT tagged tuple."
  def adt_tag(t) when is_tuple(t), do: elem(t, 0)

  # ---- Numeric utilities ------------------------------------------------

  @doc "Integer power: base^exp (exp >= 0)."
  def ipow(_base, 0), do: 1
  def ipow(base, exp) when rem(exp, 2) == 0 do
    half = ipow(base, div(exp, 2))
    half * half
  end
  def ipow(base, exp), do: base * ipow(base, exp - 1)

  @doc "Clamp a value to [lo, hi]."
  def clamp(v, lo, hi), do: max(lo, min(hi, v))

  # ---- Functional combinators -------------------------------------------

  @doc "Function identity."
  def id(x), do: x

  @doc "Constant function: always returns `a`."
  def const(a, _b), do: a

  @doc "Flip argument order of a two-argument function."
  def flip(f, a, b), do: f.(b, a)

  @doc "Compose two functions: `compose(f, g).(x) == f.(g.(x))`."
  def compose(f, g), do: fn x -> f.(g.(x)) end

  # ---- Option/Maybe helpers ---------------------------------------------

  @doc "Wrap in `{:some, v}` or return `:none`."
  def some(v), do: {:some, v}
  def none, do: :none

  @doc "Unwrap an option, returning `default` on `:none`."
  def option_get({:some, v}, _default), do: v
  def option_get(:none, default), do: default

  # ---- List helpers -----------------------------------------------------

  @doc "Safe head: returns `{:some, head}` or `:none`."
  def list_head([h | _]), do: {:some, h}
  def list_head([]), do: :none

  @doc "Safe tail: returns `{:some, tail}` or `:none`."
  def list_tail([_ | t]), do: {:some, t}
  def list_tail([]), do: :none

  @doc "Zip two lists into a list of 2-tuples."
  def zip([], _), do: []
  def zip(_, []), do: []
  def zip([h1 | t1], [h2 | t2]), do: [{h1, h2} | zip(t1, t2)]
end
"#;
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend() -> ElixirBackend {
        ElixirBackend::new()
    }
    #[test]
    pub(super) fn test_emit_atom() {
        assert_eq!(
            backend().emit_expr(&ElixirExpr::Atom("ok".to_string())),
            ":ok"
        );
    }
    #[test]
    pub(super) fn test_emit_integer() {
        assert_eq!(backend().emit_expr(&ElixirExpr::Integer(42)), "42");
        assert_eq!(backend().emit_expr(&ElixirExpr::Integer(-7)), "-7");
    }
    #[test]
    pub(super) fn test_emit_float() {
        let s = backend().emit_expr(&ElixirExpr::Float(3.14));
        assert!(s.contains("3.14"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_binary() {
        assert_eq!(
            backend().emit_expr(&ElixirExpr::Binary("hello".to_string())),
            "\"hello\""
        );
    }
    #[test]
    pub(super) fn test_emit_bool() {
        assert_eq!(backend().emit_expr(&ElixirExpr::Bool(true)), "true");
        assert_eq!(backend().emit_expr(&ElixirExpr::Bool(false)), "false");
    }
    #[test]
    pub(super) fn test_emit_nil() {
        assert_eq!(backend().emit_expr(&ElixirExpr::Nil), "nil");
    }
    #[test]
    pub(super) fn test_emit_list() {
        let e = ElixirExpr::List(vec![
            ElixirExpr::Integer(1),
            ElixirExpr::Integer(2),
            ElixirExpr::Integer(3),
        ]);
        assert_eq!(backend().emit_expr(&e), "[1, 2, 3]");
        let empty = ElixirExpr::List(vec![]);
        assert_eq!(backend().emit_expr(&empty), "[]");
    }
    #[test]
    pub(super) fn test_emit_tuple() {
        let e = ElixirExpr::Tuple(vec![
            ElixirExpr::Atom("ok".to_string()),
            ElixirExpr::Integer(0),
        ]);
        assert_eq!(backend().emit_expr(&e), "{:ok, 0}");
    }
    #[test]
    pub(super) fn test_emit_map() {
        let e = ElixirExpr::Map(vec![(
            ElixirExpr::Atom("key".to_string()),
            ElixirExpr::Integer(1),
        )]);
        assert_eq!(backend().emit_expr(&e), "%{:key => 1}");
        let empty = ElixirExpr::Map(vec![]);
        assert_eq!(backend().emit_expr(&empty), "%{}");
    }
    #[test]
    pub(super) fn test_emit_func_call() {
        let e = ElixirExpr::FuncCall(
            "Enum.map".to_string(),
            vec![
                ElixirExpr::Var("list".to_string()),
                ElixirExpr::Var("f".to_string()),
            ],
        );
        assert_eq!(backend().emit_expr(&e), "Enum.map(list, f)");
    }
    #[test]
    pub(super) fn test_emit_binop() {
        let e = ElixirExpr::BinOp(
            "+".to_string(),
            Box::new(ElixirExpr::Integer(1)),
            Box::new(ElixirExpr::Integer(2)),
        );
        assert_eq!(backend().emit_expr(&e), "1 + 2");
    }
    #[test]
    pub(super) fn test_emit_pipe() {
        let e = ElixirExpr::Pipe(
            Box::new(ElixirExpr::Var("list".to_string())),
            Box::new(ElixirExpr::FuncCall("Enum.sort".to_string(), vec![])),
        );
        let s = backend().emit_expr(&e);
        assert!(s.contains("|>"), "pipe missing: {}", s);
        assert!(s.contains("list"), "lhs missing: {}", s);
    }
    #[test]
    pub(super) fn test_mangle_name() {
        let b = backend();
        assert_eq!(b.mangle_name("fooBar"), "foo_bar");
        assert_eq!(b.mangle_name("do"), "do_");
        assert_eq!(b.mangle_name("myFunc"), "my_func");
    }
    #[test]
    pub(super) fn test_emit_function() {
        let func = ElixirFunction {
            name: "add".to_string(),
            arity: 2,
            clauses: vec![(
                vec![
                    ElixirExpr::Var("a".to_string()),
                    ElixirExpr::Var("b".to_string()),
                ],
                None,
                ElixirExpr::BinOp(
                    "+".to_string(),
                    Box::new(ElixirExpr::Var("a".to_string())),
                    Box::new(ElixirExpr::Var("b".to_string())),
                ),
            )],
            is_private: false,
            doc: None,
        };
        let s = backend().emit_function(&func);
        assert!(s.contains("def add(a, b) do"), "got: {}", s);
        assert!(s.contains("a + b"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_module() {
        let module = ElixirModule {
            name: "MyApp.Math".to_string(),
            functions: vec![ElixirFunction {
                name: "square".to_string(),
                arity: 1,
                clauses: vec![(
                    vec![ElixirExpr::Var("x".to_string())],
                    None,
                    ElixirExpr::BinOp(
                        "*".to_string(),
                        Box::new(ElixirExpr::Var("x".to_string())),
                        Box::new(ElixirExpr::Var("x".to_string())),
                    ),
                )],
                is_private: false,
                doc: None,
            }],
            use_modules: vec![],
            imports: vec![],
            attributes: HashMap::new(),
        };
        let s = backend().emit_module(&module);
        assert!(s.contains("defmodule MyApp.Math do"), "got: {}", s);
        assert!(s.contains("def square(x) do"), "got: {}", s);
        assert!(s.contains("x * x"), "got: {}", s);
        assert!(s.ends_with("end\n"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_runtime_content() {
        let rt = backend().emit_runtime();
        assert!(rt.contains("OxiLean.Runtime"));
        assert!(rt.contains("defmodule"));
        assert!(rt.contains("def adt(tag, fields)"));
        assert!(rt.contains("def ipow("));
        assert!(rt.contains("def compose(f, g)"));
    }
    #[test]
    pub(super) fn test_emit_case() {
        let e = ElixirExpr::Case(
            Box::new(ElixirExpr::Var("x".to_string())),
            vec![
                (ElixirExpr::Integer(0), ElixirExpr::Atom("zero".to_string())),
                (
                    ElixirExpr::Var("_".to_string()),
                    ElixirExpr::Atom("nonzero".to_string()),
                ),
            ],
        );
        let s = backend().emit_expr(&e);
        assert!(s.contains("case x do"), "got: {}", s);
        assert!(s.contains(":zero"), "got: {}", s);
        assert!(s.contains(":nonzero"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_lambda() {
        let e = ElixirExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(ElixirExpr::BinOp(
                "+".to_string(),
                Box::new(ElixirExpr::Var("x".to_string())),
                Box::new(ElixirExpr::Var("y".to_string())),
            )),
        );
        let s = backend().emit_expr(&e);
        assert!(s.contains("fn x, y ->"), "got: {}", s);
        assert!(s.contains("x + y"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_escape_string() {
        assert_eq!(
            backend().emit_expr(&ElixirExpr::Binary("say \"hi\"".to_string())),
            r#""say \"hi\"""#
        );
    }
    #[test]
    pub(super) fn test_private_function() {
        let func = ElixirFunction {
            name: "helper".to_string(),
            arity: 0,
            clauses: vec![(vec![], None, ElixirExpr::Nil)],
            is_private: true,
            doc: None,
        };
        let s = backend().emit_function(&func);
        assert!(s.contains("defp helper()"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_emit_if() {
        let e = ElixirExpr::If(
            Box::new(ElixirExpr::Bool(true)),
            Box::new(ElixirExpr::Integer(1)),
            Box::new(ElixirExpr::Integer(0)),
        );
        let s = backend().emit_expr(&e);
        assert!(s.contains("if true do"), "got: {}", s);
        assert!(s.contains("else"), "got: {}", s);
    }
}
#[cfg(test)]
mod Elx_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ElxPassConfig::new("test_pass", ElxPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ElxPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = ElxPassRegistry::new();
        reg.register(ElxPassConfig::new("pass_a", ElxPassPhase::Analysis));
        reg.register(ElxPassConfig::new("pass_b", ElxPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ElxAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = ElxWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = ElxDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = ElxLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ElxConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ElxConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ElxConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ElxConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ElxConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ElxDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
#[cfg(test)]
mod elixirext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_elixirext_phase_order() {
        assert_eq!(ElixirExtPassPhase::Early.order(), 0);
        assert_eq!(ElixirExtPassPhase::Middle.order(), 1);
        assert_eq!(ElixirExtPassPhase::Late.order(), 2);
        assert_eq!(ElixirExtPassPhase::Finalize.order(), 3);
        assert!(ElixirExtPassPhase::Early.is_early());
        assert!(!ElixirExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_elixirext_config_builder() {
        let c = ElixirExtPassConfig::new("p")
            .with_phase(ElixirExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_elixirext_stats() {
        let mut s = ElixirExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_elixirext_registry() {
        let mut r = ElixirExtPassRegistry::new();
        r.register(ElixirExtPassConfig::new("a").with_phase(ElixirExtPassPhase::Early));
        r.register(ElixirExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&ElixirExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_elixirext_cache() {
        let mut c = ElixirExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_elixirext_worklist() {
        let mut w = ElixirExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_elixirext_dom_tree() {
        let mut dt = ElixirExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_elixirext_liveness() {
        let mut lv = ElixirExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_elixirext_const_folder() {
        let mut cf = ElixirExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_elixirext_dep_graph() {
        let mut g = ElixirExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
#[cfg(test)]
mod elixirx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_elixirx2_phase_order() {
        assert_eq!(ElixirX2PassPhase::Early.order(), 0);
        assert_eq!(ElixirX2PassPhase::Middle.order(), 1);
        assert_eq!(ElixirX2PassPhase::Late.order(), 2);
        assert_eq!(ElixirX2PassPhase::Finalize.order(), 3);
        assert!(ElixirX2PassPhase::Early.is_early());
        assert!(!ElixirX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_elixirx2_config_builder() {
        let c = ElixirX2PassConfig::new("p")
            .with_phase(ElixirX2PassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_elixirx2_stats() {
        let mut s = ElixirX2PassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_elixirx2_registry() {
        let mut r = ElixirX2PassRegistry::new();
        r.register(ElixirX2PassConfig::new("a").with_phase(ElixirX2PassPhase::Early));
        r.register(ElixirX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&ElixirX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_elixirx2_cache() {
        let mut c = ElixirX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_elixirx2_worklist() {
        let mut w = ElixirX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_elixirx2_dom_tree() {
        let mut dt = ElixirX2DomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_elixirx2_liveness() {
        let mut lv = ElixirX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_elixirx2_const_folder() {
        let mut cf = ElixirX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_elixirx2_dep_graph() {
        let mut g = ElixirX2DepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
