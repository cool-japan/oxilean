# oxilean-elab — TODO

> Task list for the elaborator crate.
> Last updated: 2026-05-03

## ✅ Completed

- [x] Module structure (466 files, ~92,415 SLOC total)
- [x] `MetaVarId` type (u64 wrapper with derive traits)
- [x] Re-exports in `lib.rs`

---

## ✅ Completed (Phase 3): Elaborator Core

### Meta-variable Infrastructure (`metavar.rs` — 166 lines)
- [x] `MetaContext` struct — map from `MetaVarId` to assignment status
- [x] `create_mvar(type)` — create a fresh metavariable with expected type
- [x] `assign(mvar, expr)` — assign a value to a metavariable
- [x] `is_assigned(mvar)` / `is_resolved(mvar)` — check assignment status
- [x] `get_unresolved()` — list unresolved metavariables
- [x] `count()` — total count
- [x] `zonk(expr)` — replace all assigned metavariables (recursive traversal)
- [x] Occurs check
- [x] Scope management for metavariables

### Unification (`unify.rs` — 148 lines + `solver.rs` — 181 lines)
- [x] `unify(lhs, rhs)` → `Result<(), UnifyError>` — structural equality
- [x] Structural comparison: Sort, BVar, FVar, Const, App, Lam, Pi, Let, Lit, Proj
- [x] First-order unification: `?m =? t` (metavar-aware) — assignment with occurs check
- [x] Priority-based constraint scheduler with postponement and retry
- [x] Definitional equality integration via kernel `is_def_eq`

### Expression Elaboration (`elaborate.rs` — 2,237 lines)
- [x] `ElabContext` struct with env, local context, meta context
- [x] `elab_expr(surface_expr)` → `Result<Expr, ElabError>`
- [x] Name resolution: local → global → overload resolution
- [x] Application elaboration with implicit argument insertion
- [x] Lambda elaboration (infer binder types from expected type)
- [x] Pi / Arrow elaboration
- [x] Let elaboration
- [x] Literal elaboration (Nat/String)
- [x] Hole `_` → create metavariable
- [x] Projection elaboration (`e.field`)
- [x] Match expression elaboration
- [x] `by` block → invoke tactic engine
- [x] If/then/else elaboration
- [x] Do-notation elaboration
- [x] Have / Suffices / Show expressions
- [x] Named arguments
- [x] Anonymous constructors, list literals, tuples
- [x] String interpolation, range expressions
- [x] Calc blocks
- [x] Type-directed elaboration with expected type propagation
- [x] Overload resolution

### Pattern Match Compilation (`pattern_match.rs` — 1,819 lines + `equation.rs` — 240 lines)
- [x] Surface patterns → decision tree
- [x] Scrutinee elaboration → pattern elaboration → exhaustiveness check → compile
- [x] Exhaustiveness checking (constructor set validation)
- [x] Redundancy checking (catch-all detection, subsumption)
- [x] Pattern elaboration: Wild, Var, Ctor, Lit, Or

### Declaration Elaboration (`elab_decl.rs` — 1,567 lines)
- [x] `elab_def(name, params, ret_ty, body)` — definition elaboration
- [x] `elab_theorem(name, params, ty, proof)` — theorem elaboration
- [x] `elab_axiom(name, params, ty)` — axiom elaboration
- [x] `elab_inductive(name, params, ty, ctors)` — inductive type elaboration
- [x] Universe parameter inference / checking
- [x] Mutual recursion support (forward declare → elaborate → assign)
- [x] Where clause elaboration (let-binding generation)
- [x] Opaque declarations
- [x] Structure / Class / Instance declarations
- [x] Namespace / Section / Variable / Open / Attribute / HashCmd
- [x] Attribute processing (simp/ext/instance/reducible/irreducible/inline etc.)

### Namespace & Import Resolution (`module_import.rs` — 1,983 lines)
- [x] Hierarchical namespace management
- [x] Module definition (name, imports, exports, visibility: Public/Protected/Private)
- [x] Import resolution with selective/hiding/renamed imports
- [x] Multi-module management (`ModuleManager`)
- [x] Dependency graph with cycle detection (DFS) and topological sort (Kahn)

### Command Elaboration (`command_elab.rs` — 1,850 lines)
- [x] Section / End section (with variable abstraction)
- [x] Namespace / End namespace
- [x] Variable / Universe declarations
- [x] Open / Set option
- [x] `#check`, `#eval`, `#print` commands

### Termination Checking (`mutual.rs` — 1,575 lines + `equation.rs`)
- [x] Structural recursion detection and checking
- [x] Recursive call collection and decreasing argument analysis
- [x] Well-founded recursion (WellFounded.fix term construction)
- [x] Mutual recursion support

---

## ✅ Completed (Phase 4, partial): Tactics

### Tactic Infrastructure (`tactic.rs` — 1,604 lines)
- [x] `TacticState` struct (goals, solved)
- [x] `Goal` struct (mvar_id, hypotheses, local_ctx, target, tag)
- [x] Tactic combinator framework (sequence execution)
- [x] Goal focusing
- [x] `TacticRegistry` — registration, lookup, execution (18 tactics registered)
- [x] Undo / backtrack support (snapshot/restore)

### Core Tactics (implemented)
- [x] `intro` / `intros` — introduce Pi binder as hypothesis, create new goal
- [x] `apply` — apply function/lemma (simplified)
- [x] `exact` — provide exact proof term
- [x] `assumption` — search local context for matching hypothesis
- [x] `rfl` — prove `a = a` by reflexivity (Eq pattern detection)
- [x] `trivial` — try refl → assumption → True.intro
- [x] `constructor` — apply constructor (True, And patterns)
- [x] `left` / `right` — for disjunction goals (Or)
- [x] `exists` — provide witness for existential (Exists pattern)
- [x] `exfalso` — change goal to False
- [x] `clear` / `rename` / `revert` — hypothesis management
- [x] `have` / `suffices` — introduce intermediate goals
- [x] `sorry` — admit proof

### Additional Tactics (IMPLEMENTED)
- [x] `cases` — And/Or/False/Nat/Exists case split
- [x] `induction` — Nat structural induction with IH
- [x] `rw [h]` / `rw [← h]` — rewrite goal; chain rewrites supported
- [x] `rw [h] at hyp` — rewrite inside a hypothesis
- [x] `simp` / `simp only [h1, h2]` — beta-reduce + built-in rules + rewrites
- [x] `push_neg`, `by_contra`, `contrapose`, `split` — logic tactics
- [x] `omega`, `ring`, `linarith`, `field_simp` — arithmetic tactics
- [x] `norm_cast`, `exact_mod_cast`, `push_cast` — coercion tactics

---

## ✅ Additional Features (beyond original TODO)

### Attribute System (`attribute.rs` — 1,348 lines)
- [x] 10+ attribute kinds: simp, ext, instance, reducible, irreducible, inline, noinline, specialize, priority, custom
- [x] Duplicate/incompatible attribute checking
- [x] `AttributeRegistry` with handler registration

### Binder Elaboration (`binder.rs` — 1,167 lines)
- [x] Binder type annotation elaboration + metavar generation
- [x] Auto-bound implicit variables
- [x] Instance synthesis (simplified environment-based)

### Coercion System (`coercion.rs` — 965 lines)
- [x] Coercion registration and chaining (BFS shortest chain search)
- [x] Built-in coercions: Nat→Int, Bool→Prop, Int→Rat, generic coe
- [x] Apply coercion chains

### Derive System (`derive.rs` — 1,672 lines + `derive_adv.rs` — 2,543 lines)
- [x] Derive handlers: BEq, DecidableEq, Hashable, Ord, Repr, Inhabited, Nonempty, ToString, Show, Default
- [x] Field comparison + AND chain generation
- [x] Constructor repr, tag hashing

### Structure Elaboration (`structure.rs` — 2,186 lines)
- [x] Structure/Class elaboration with parent field inheritance
- [x] Projection function generation
- [x] Constructor and recursor type generation
- [x] Circular inheritance detection

### Do-Notation Elaboration (`elaborate.rs` — includes Do desugaring)
- [x] DoBlock → bind/pure/map chain transformation
- [x] Bind, LetBind, Action, Return, For, If, Match, TryCatch, Unless elements

### Info Tree (`info_tree.rs` — 2,263 lines)
- [x] TermInfo, FieldInfo, TacticInfo, MacroExpansion, CommandInfo, CompletionInfo
- [x] Tree construction and query
- [x] Hover info, documentation display

### Macro Expansion (`macro_expand.rs` — 1,361 lines)
- [x] SyntaxMacro, CommandMacro, TacticMacro, TermMacro, NotationMacro
- [x] Depth-limited recursive expansion
- [x] Hygienic renaming

### Notation System (`notation.rs` — 1,351 lines)
- [x] Prefix/Infixl/Infixr/Postfix/Notation/Macro kinds
- [x] Scope management, registration, lookup
- [x] Do-notation and list literal expansion

### Parallel Elaboration (`parallel.rs` — 1,605 lines)
- [x] Task scheduling with dependency graph
- [x] Cycle detection (DFS) and topological sort (Kahn)
- [x] Level-parallel execution with max_parallelism
- [x] Progress tracking

### Error Messages (`error_msg.rs` — 877 lines)
- [x] 50+ error codes (E1000-E5010): syntax, type, name, universe, pattern errors
- [x] Levenshtein distance for "did you mean?" suggestions
- [x] Code snippet highlighting

### Implicit Resolution (`implicit.rs` — 106 lines)
- [x] Implicit argument insertion (Pi traversal + metavar generation)
- [x] `infer_implicit` — single-match local hypothesis lookup
- [x] `resolve_instance` — local + global environment search by class head

### Type Class System (`typeclass.rs` — 160 lines + `instance.rs` — 132 lines)
- [x] Class/Instance registration and lookup
- [x] `resolve_constraint` — uses `find_best_instance` from registry
- [x] Priority-based instance scoring

### Inference (`infer.rs` — 223 lines)
- [x] Type inference for basic Expr variants (Sort, BVar, FVar, Const, Lam, Pi, App, Let, Lit)
- [x] Proj inference (delegates to kernel `infer_proj`)
- [x] Universe level instantiation

### Quote/Unquote (`quote.rs` — 189 lines)
- [x] `quote_expr` / `unquote_expr` — full constructor-by-constructor transformation
- [x] Nested Name, Level, BinderInfo, Literal unquoting

### Context (`context.rs` — 174 lines)
- [x] `ElabLocalContext` with push/pop, lookup, metavar support

### Trace (`trace.rs` — 1,041 lines)
- [x] Log levels (Off/Error/Warn/Info/Debug/Trace)
- [x] Categories: Elaboration, TypeInference, Unification, InstanceSynthesis, etc.
- [x] Event recording, filtering, file output

---

## 🐛 Known Issues

None. All previously tracked issues have been resolved as of 2026-03-09.

---

## ⚪ Future Enhancements

- [x] Auto tactic (simple proof search) — `tactic_auto.rs` (AutoTactic, TautoTactic, eval_auto, 12 tests)
- [x] Metaprogramming (user-defined elaborators / tactics) — `metaprog.rs` (UserTactic, UserElab, MacroEngine registries, 7 tests)
- [x] Omega tactic (linear arithmetic) — stub in meta/tactic/omega.rs
- [x] Ring normalization tactic — partial in meta/tactic/ring.rs
- [x] Decision procedures (SAT/SMT via OxiZ integration) — stub in meta/tactic/smt.rs
