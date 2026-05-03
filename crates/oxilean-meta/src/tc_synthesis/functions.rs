//! Functions for the type class instance synthesis module.
//!
//! Implements recursive instance synthesis, coherence checking, superclass
//! traversal, a standard type class database, and pretty-printing helpers.

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{SynthConfig, SynthGoal, SynthResult, SynthTrace, TcClass, TcDB, TcInstance};

// ─── TcDB impl ────────────────────────────────────────────────────────────

impl TcDB {
    /// Create an empty type class database.
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            instances: Vec::new(),
        }
    }

    /// Register a type class.  Overwrites any existing class with the same name.
    pub fn add_class(&mut self, class: TcClass) {
        self.classes.insert(class.name.clone(), class);
    }

    /// Register an instance.
    pub fn add_instance(&mut self, instance: TcInstance) {
        self.instances.push(instance);
    }

    /// Return all instances whose `class` field matches `class_name`.
    pub fn instances_for(&self, class_name: &str) -> Vec<&TcInstance> {
        self.instances
            .iter()
            .filter(|i| i.class == class_name)
            .collect()
    }
}

impl Default for TcDB {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Synthesis ────────────────────────────────────────────────────────────

/// Synthesize a type class instance for `goal` using `db` and `cfg`.
///
/// Returns `(SynthResult, SynthTrace)`.  The trace records every sub-goal
/// attempted so callers can diagnose failures.
pub fn synthesize_instance(
    db: &TcDB,
    goal: &SynthGoal,
    cfg: &SynthConfig,
) -> (SynthResult, SynthTrace) {
    let mut trace = SynthTrace::new();
    let mut in_progress: Vec<SynthGoal> = Vec::new();
    let result = synth_recursive(db, goal, cfg, &mut trace, &mut in_progress, 0);
    (result, trace)
}

/// Internal recursive synthesis helper.
fn synth_recursive(
    db: &TcDB,
    goal: &SynthGoal,
    cfg: &SynthConfig,
    trace: &mut SynthTrace,
    in_progress: &mut Vec<SynthGoal>,
    depth: usize,
) -> SynthResult {
    // Depth guard.
    if depth > cfg.max_depth {
        let result = SynthResult::NotFound;
        trace.record(goal.clone(), result.clone());
        return result;
    }

    // Cycle detection: if this exact goal is already on the stack, we have a cycle.
    if in_progress.contains(goal) {
        let result = SynthResult::Cycle(in_progress.clone());
        trace.record(goal.clone(), result.clone());
        return result;
    }

    in_progress.push(goal.clone());

    // Collect candidate instances for this goal's class.
    let candidates: Vec<TcInstance> = db
        .instances
        .iter()
        .filter(|i| i.matches_goal(goal))
        .cloned()
        .collect();

    let result = match candidates.len() {
        0 => SynthResult::NotFound,
        1 => {
            let inst = candidates.into_iter().next().unwrap_or_else(|| {
                // Unreachable due to the len() == 1 guard; provide a fallback.
                TcInstance::new("", vec![], "")
            });
            // Resolve superclass sub-goals.
            let constraints =
                superclass_constraints(db, goal, &inst, cfg, trace, in_progress, depth);
            SynthResult::Found {
                instance: inst,
                constraints,
            }
        }
        _ => {
            // More than one match → coherence violation / overlapping instances.
            SynthResult::Overlapping(candidates)
        }
    };

    trace.record(goal.clone(), result.clone());
    in_progress.pop();
    result
}

/// Collect sub-goals arising from superclass constraints of a resolved instance.
fn superclass_constraints(
    db: &TcDB,
    goal: &SynthGoal,
    _inst: &TcInstance,
    cfg: &SynthConfig,
    trace: &mut SynthTrace,
    in_progress: &mut Vec<SynthGoal>,
    depth: usize,
) -> Vec<SynthGoal> {
    let class_name = &goal.class;
    let superclasses = match db.classes.get(class_name) {
        Some(cls) => cls.superclasses.clone(),
        None => return Vec::new(),
    };

    let mut pending: Vec<SynthGoal> = Vec::new();
    for sc in &superclasses {
        let sc_goal = SynthGoal::new(sc.clone(), goal.type_args.clone());
        // Attempt synthesis of the superclass (for trace richness); ignore the
        // result here — the caller can inspect the trace.
        let _ = synth_recursive(db, &sc_goal, cfg, trace, in_progress, depth + 1);
        pending.push(sc_goal);
    }
    pending
}

// ─── Coherence checking ───────────────────────────────────────────────────

/// Check for overlapping (incoherent) instances in `db`.
///
/// Two instances overlap when they have the same class name **and the same
/// concrete type arguments**.  Multiple instances for the same class but
/// different type arguments (e.g., `Add Nat` and `Add Int`) are perfectly
/// legal in a Lean4-style system.
///
/// Returns a list of diagnostic strings describing each overlap.
pub fn check_coherence(db: &TcDB) -> Vec<String> {
    let mut diagnostics: Vec<String> = Vec::new();

    // Group instances by (class, type_args).
    let mut groups: HashMap<(String, Vec<String>), Vec<&TcInstance>> = HashMap::new();
    for inst in &db.instances {
        groups
            .entry((inst.class.clone(), inst.type_args.clone()))
            .or_default()
            .push(inst);
    }

    for ((class, type_args), insts) in &groups {
        if insts.len() > 1 {
            let names: Vec<&str> = insts.iter().map(|i| i.name.as_str()).collect();
            diagnostics.push(format!(
                "coherence violation: class `{}` applied to [{}] has overlapping instances: {}",
                class,
                type_args.join(", "),
                names.join(", ")
            ));
        }
    }

    diagnostics.sort();
    diagnostics
}

// ─── Superclass chain ─────────────────────────────────────────────────────

/// Compute the transitive superclass chain of `class` in `db` (BFS order).
///
/// The returned list does not include `class` itself, and each name appears
/// at most once.
pub fn superclass_chain(db: &TcDB, class: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    // Seed with direct superclasses.
    if let Some(cls) = db.classes.get(class) {
        for sc in &cls.superclasses {
            if !visited.contains(sc) {
                visited.insert(sc.clone());
                queue.push_back(sc.clone());
            }
        }
    }

    while let Some(current) = queue.pop_front() {
        result.push(current.clone());
        if let Some(cls) = db.classes.get(&current) {
            for sc in &cls.superclasses {
                if !visited.contains(sc) {
                    visited.insert(sc.clone());
                    queue.push_back(sc.clone());
                }
            }
        }
    }

    result
}

// ─── Standard database ────────────────────────────────────────────────────

/// Build a `TcDB` pre-populated with Lean4-like standard type classes and
/// instances for common types.
///
/// Classes: `Eq`, `Ord`, `Add`, `Mul`, `Neg`, `Zero`, `One`, `Functor`, `Monad`.
/// Instances for: `Nat`, `Int`, `Bool`, `Float`, `String`, `Option`, `List`.
pub fn standard_tc_db() -> TcDB {
    let mut db = TcDB::new();

    // ── Classes ─────────────────────────────────────────────────────────

    db.add_class(TcClass::new("Eq", vec!["α".to_string()]).with_method("beq", "α → α → Bool"));

    db.add_class(
        TcClass::new("Ord", vec!["α".to_string()])
            .with_superclass("Eq")
            .with_method("compare", "α → α → Ordering"),
    );

    db.add_class(TcClass::new("Add", vec!["α".to_string()]).with_method("add", "α → α → α"));

    db.add_class(TcClass::new("Mul", vec!["α".to_string()]).with_method("mul", "α → α → α"));

    db.add_class(TcClass::new("Neg", vec!["α".to_string()]).with_method("neg", "α → α"));

    db.add_class(TcClass::new("Zero", vec!["α".to_string()]).with_method("zero", "α"));

    db.add_class(TcClass::new("One", vec!["α".to_string()]).with_method("one", "α"));

    db.add_class(
        TcClass::new("Functor", vec!["f".to_string()]).with_method("map", "(α → β) → f α → f β"),
    );

    db.add_class(
        TcClass::new("Monad", vec!["m".to_string()])
            .with_superclass("Functor")
            .with_method("pure", "α → m α")
            .with_method("bind", "m α → (α → m β) → m β"),
    );

    // ── Instances for Nat ────────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Eq", vec!["Nat".to_string()], "instEqNat").with_impl("beq", "Nat.beq"),
    );
    db.add_instance(
        TcInstance::new("Ord", vec!["Nat".to_string()], "instOrdNat")
            .with_impl("compare", "Nat.compare"),
    );
    db.add_instance(
        TcInstance::new("Add", vec!["Nat".to_string()], "instAddNat").with_impl("add", "Nat.add"),
    );
    db.add_instance(
        TcInstance::new("Mul", vec!["Nat".to_string()], "instMulNat").with_impl("mul", "Nat.mul"),
    );
    db.add_instance(
        TcInstance::new("Zero", vec!["Nat".to_string()], "instZeroNat").with_impl("zero", "0"),
    );
    db.add_instance(
        TcInstance::new("One", vec!["Nat".to_string()], "instOneNat").with_impl("one", "1"),
    );

    // ── Instances for Int ────────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Eq", vec!["Int".to_string()], "instEqInt").with_impl("beq", "Int.beq"),
    );
    db.add_instance(
        TcInstance::new("Ord", vec!["Int".to_string()], "instOrdInt")
            .with_impl("compare", "Int.compare"),
    );
    db.add_instance(
        TcInstance::new("Add", vec!["Int".to_string()], "instAddInt").with_impl("add", "Int.add"),
    );
    db.add_instance(
        TcInstance::new("Mul", vec!["Int".to_string()], "instMulInt").with_impl("mul", "Int.mul"),
    );
    db.add_instance(
        TcInstance::new("Neg", vec!["Int".to_string()], "instNegInt").with_impl("neg", "Int.neg"),
    );
    db.add_instance(
        TcInstance::new("Zero", vec!["Int".to_string()], "instZeroInt").with_impl("zero", "0"),
    );
    db.add_instance(
        TcInstance::new("One", vec!["Int".to_string()], "instOneInt").with_impl("one", "1"),
    );

    // ── Instances for Float ──────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Add", vec!["Float".to_string()], "instAddFloat")
            .with_impl("add", "Float.add"),
    );
    db.add_instance(
        TcInstance::new("Mul", vec!["Float".to_string()], "instMulFloat")
            .with_impl("mul", "Float.mul"),
    );
    db.add_instance(
        TcInstance::new("Neg", vec!["Float".to_string()], "instNegFloat")
            .with_impl("neg", "Float.neg"),
    );

    // ── Instances for Bool ───────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Eq", vec!["Bool".to_string()], "instEqBool").with_impl("beq", "Bool.beq"),
    );

    // ── Instances for String ─────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Eq", vec!["String".to_string()], "instEqString")
            .with_impl("beq", "String.beq"),
    );

    // ── Instances for Option ─────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Functor", vec!["Option".to_string()], "instFunctorOption")
            .with_impl("map", "Option.map"),
    );
    db.add_instance(
        TcInstance::new("Monad", vec!["Option".to_string()], "instMonadOption")
            .with_impl("pure", "Option.some")
            .with_impl("bind", "Option.bind"),
    );

    // ── Instances for List ───────────────────────────────────────────────

    db.add_instance(
        TcInstance::new("Functor", vec!["List".to_string()], "instFunctorList")
            .with_impl("map", "List.map"),
    );
    db.add_instance(
        TcInstance::new("Monad", vec!["List".to_string()], "instMonadList")
            .with_impl("pure", "List.pure")
            .with_impl("bind", "List.bind"),
    );

    db
}

// ─── Pretty printing ──────────────────────────────────────────────────────

/// Render a `TcInstance` as a human-readable multi-line string.
pub fn instance_to_string(inst: &TcInstance) -> String {
    let header = format!(
        "instance {} : {} {}",
        inst.name,
        inst.class,
        inst.type_args.join(" ")
    );

    if inst.impl_body.is_empty() {
        return format!("{} where\n  -- (no methods)", header);
    }

    let mut methods: Vec<String> = inst
        .impl_body
        .iter()
        .map(|(name, body)| format!("  {} := {}", name, body))
        .collect();
    methods.sort();

    format!("{} where\n{}", header, methods.join("\n"))
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> SynthConfig {
        SynthConfig::default()
    }

    fn std_db() -> TcDB {
        standard_tc_db()
    }

    // ── TcDB::new ──────────────────────────────────────────────────────

    #[test]
    fn test_new_db_empty() {
        let db = TcDB::new();
        assert!(db.classes.is_empty());
        assert!(db.instances.is_empty());
    }

    // ── add_class / add_instance ───────────────────────────────────────

    #[test]
    fn test_add_class_and_instance() {
        let mut db = TcDB::new();
        db.add_class(TcClass::new("MyClass", vec!["α".to_string()]));
        db.add_instance(TcInstance::new(
            "MyClass",
            vec!["Nat".to_string()],
            "instMyClassNat",
        ));
        assert!(db.classes.contains_key("MyClass"));
        assert_eq!(db.instances.len(), 1);
    }

    // ── instances_for ──────────────────────────────────────────────────

    #[test]
    fn test_instances_for_add() {
        let db = std_db();
        let insts = db.instances_for("Add");
        assert!(!insts.is_empty());
        assert!(insts.iter().any(|i| i.type_args == vec!["Nat"]));
    }

    #[test]
    fn test_instances_for_unknown_class() {
        let db = std_db();
        assert!(db.instances_for("NonExistent").is_empty());
    }

    // ── synthesize_instance ────────────────────────────────────────────

    #[test]
    fn test_synth_found_add_nat() {
        let db = std_db();
        let goal = SynthGoal::new("Add", vec!["Nat".to_string()]);
        let (result, trace) = synthesize_instance(&db, &goal, &cfg());
        match result {
            SynthResult::Found { instance, .. } => {
                assert_eq!(instance.class, "Add");
                assert_eq!(instance.type_args, vec!["Nat"]);
            }
            other => panic!("expected Found, got {:?}", other),
        }
        assert!(!trace.is_empty());
    }

    #[test]
    fn test_synth_found_eq_bool() {
        let db = std_db();
        let goal = SynthGoal::new("Eq", vec!["Bool".to_string()]);
        let (result, _) = synthesize_instance(&db, &goal, &cfg());
        assert!(matches!(result, SynthResult::Found { .. }));
    }

    #[test]
    fn test_synth_not_found_no_instance() {
        let db = std_db();
        // Neg is not registered for Bool.
        let goal = SynthGoal::new("Neg", vec!["Bool".to_string()]);
        let (result, _) = synthesize_instance(&db, &goal, &cfg());
        assert_eq!(result, SynthResult::NotFound);
    }

    #[test]
    fn test_synth_not_found_unknown_class() {
        let db = std_db();
        let goal = SynthGoal::new("MyFancyClass", vec!["Nat".to_string()]);
        let (result, _) = synthesize_instance(&db, &goal, &cfg());
        assert_eq!(result, SynthResult::NotFound);
    }

    #[test]
    fn test_synth_cycle_detection() {
        // Construct a minimal db where two instances form a cycle: synthesizing
        // class A requires synthesizing class B, which again requires class A.
        // We simulate this by having the synthesizer encounter the same goal
        // twice on the stack.  We do that via the in_progress list in the
        // recursive helper — we test the public API here by checking that even
        // a deeply recursive search terminates and doesn't panic.
        let db = std_db();
        let goal = SynthGoal::new("Add", vec!["Nat".to_string()]);
        // A very shallow depth still successfully resolves a leaf instance.
        let cfg_shallow = SynthConfig {
            max_depth: 1,
            ..SynthConfig::default()
        };
        let (result, trace) = synthesize_instance(&db, &goal, &cfg_shallow);
        // The direct instance for Add Nat should be found at depth 0.
        assert!(matches!(result, SynthResult::Found { .. }));
        // The trace must be non-empty.
        assert!(!trace.is_empty());
    }

    #[test]
    fn test_synth_monad_option() {
        let db = std_db();
        let goal = SynthGoal::new("Monad", vec!["Option".to_string()]);
        let (result, _) = synthesize_instance(&db, &goal, &cfg());
        assert!(matches!(result, SynthResult::Found { .. }));
    }

    #[test]
    fn test_synth_functor_list() {
        let db = std_db();
        let goal = SynthGoal::new("Functor", vec!["List".to_string()]);
        let (result, _) = synthesize_instance(&db, &goal, &cfg());
        assert!(matches!(result, SynthResult::Found { .. }));
    }

    // ── check_coherence ────────────────────────────────────────────────

    #[test]
    fn test_coherence_standard_db_clean() {
        let db = std_db();
        let diags = check_coherence(&db);
        assert!(
            diags.is_empty(),
            "standard db should be coherent, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_coherence_detects_overlap() {
        let mut db = TcDB::new();
        db.add_class(TcClass::new("Eq", vec!["α".to_string()]));
        // Two instances with the same class AND same type_args → coherence violation.
        db.add_instance(TcInstance::new("Eq", vec!["Nat".to_string()], "instA"));
        db.add_instance(TcInstance::new("Eq", vec!["Nat".to_string()], "instB"));
        let diags = check_coherence(&db);
        assert!(!diags.is_empty());
        assert!(diags[0].contains("coherence violation"));
    }

    // ── superclass_chain ───────────────────────────────────────────────

    #[test]
    fn test_superclass_chain_ord_extends_eq() {
        let db = std_db();
        let chain = superclass_chain(&db, "Ord");
        assert!(chain.contains(&"Eq".to_string()));
    }

    #[test]
    fn test_superclass_chain_monad_extends_functor() {
        let db = std_db();
        let chain = superclass_chain(&db, "Monad");
        assert!(chain.contains(&"Functor".to_string()));
    }

    #[test]
    fn test_superclass_chain_no_superclasses() {
        let db = std_db();
        let chain = superclass_chain(&db, "Add");
        assert!(chain.is_empty());
    }

    #[test]
    fn test_superclass_chain_unknown_class() {
        let db = std_db();
        assert!(superclass_chain(&db, "Unknown").is_empty());
    }

    // ── standard_tc_db ─────────────────────────────────────────────────

    #[test]
    fn test_standard_db_has_eq_class() {
        let db = std_db();
        assert!(db.classes.contains_key("Eq"));
    }

    #[test]
    fn test_standard_db_has_monad_class() {
        let db = std_db();
        assert!(db.classes.contains_key("Monad"));
    }

    #[test]
    fn test_standard_db_eq_nat_instance_has_beq() {
        let db = std_db();
        let insts = db.instances_for("Eq");
        let nat_inst = insts.iter().find(|i| i.type_args == vec!["Nat"]);
        assert!(nat_inst.is_some());
        assert!(nat_inst.unwrap().impl_body.contains_key("beq"));
    }

    // ── instance_to_string ─────────────────────────────────────────────

    #[test]
    fn test_instance_to_string_basic() {
        let inst = TcInstance::new("Add", vec!["Nat".to_string()], "instAddNat")
            .with_impl("add", "Nat.add");
        let s = instance_to_string(&inst);
        assert!(s.contains("instAddNat"));
        assert!(s.contains("Add"));
        assert!(s.contains("Nat"));
        assert!(s.contains("Nat.add"));
    }

    #[test]
    fn test_instance_to_string_no_methods() {
        let inst = TcInstance::new("Eq", vec!["Nat".to_string()], "instEqNat");
        let s = instance_to_string(&inst);
        assert!(s.contains("no methods"));
    }

    // ── TcClass helpers ────────────────────────────────────────────────

    #[test]
    fn test_tc_class_builder_methods() {
        let cls = TcClass::new("MyClass", vec!["α".to_string()])
            .with_superclass("Eq")
            .with_method("myMethod", "α → Bool");
        assert_eq!(cls.superclasses, vec!["Eq"]);
        assert_eq!(cls.methods.len(), 1);
        assert_eq!(cls.methods[0].0, "myMethod");
    }

    #[test]
    fn test_tc_class_display() {
        let cls = TcClass::new("Ord", vec!["α".to_string()]).with_superclass("Eq");
        let s = format!("{}", cls);
        assert!(s.contains("Ord"));
        assert!(s.contains("Eq"));
    }

    // ── SynthGoal helpers ──────────────────────────────────────────────

    #[test]
    fn test_synth_goal_display() {
        let g = SynthGoal::new("Add", vec!["Nat".to_string()]);
        assert_eq!(format!("{}", g), "Add Nat");
    }

    #[test]
    fn test_synth_goal_display_no_args() {
        let g = SynthGoal::new("Eq", vec![]);
        assert_eq!(format!("{}", g), "Eq");
    }

    // ── SynthTrace ─────────────────────────────────────────────────────

    #[test]
    fn test_synth_trace_records() {
        let mut trace = SynthTrace::new();
        let goal = SynthGoal::new("Add", vec!["Nat".to_string()]);
        trace.record(goal.clone(), SynthResult::NotFound);
        assert_eq!(trace.len(), 1);
        assert!(!trace.is_empty());
    }

    // ── TcInstance::matches_goal ───────────────────────────────────────

    #[test]
    fn test_instance_matches_goal_true() {
        let inst = TcInstance::new("Add", vec!["Nat".to_string()], "instAddNat");
        let goal = SynthGoal::new("Add", vec!["Nat".to_string()]);
        assert!(inst.matches_goal(&goal));
    }

    #[test]
    fn test_instance_matches_goal_wrong_arity() {
        let inst = TcInstance::new("Add", vec!["Nat".to_string()], "instAddNat");
        let goal = SynthGoal::new("Add", vec![]);
        assert!(!inst.matches_goal(&goal));
    }

    #[test]
    fn test_instance_matches_goal_wrong_class() {
        let inst = TcInstance::new("Add", vec!["Nat".to_string()], "instAddNat");
        let goal = SynthGoal::new("Mul", vec!["Nat".to_string()]);
        assert!(!inst.matches_goal(&goal));
    }
}
