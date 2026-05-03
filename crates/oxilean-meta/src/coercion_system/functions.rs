//! Functions for the coercion system module.
//!
//! Implements BFS-based coercion path search, cycle detection, standard built-in
//! coercions for Lean4-like numeric and logic hierarchies, and utility helpers.

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    CoercedExpr, CoercionDB, CoercionDef, CoercionError, CoercionPath, CoercionResult,
};

// ─── CoercionDB impl ──────────────────────────────────────────────────────

impl CoercionDB {
    /// Create an empty coercion database.
    pub fn new() -> Self {
        Self {
            coercions: Vec::new(),
        }
    }

    /// Register a new coercion definition.
    ///
    /// The database keeps all coercions sorted by priority (ascending).
    /// Returns `CoercionError::Cycle` if adding the definition would introduce
    /// a cycle reachable within `MAX_PATH_LEN` steps.
    pub fn add(&mut self, def: CoercionDef) -> Result<(), CoercionError> {
        // Temporarily insert so we can run cycle detection.
        let insert_pos = self
            .coercions
            .partition_point(|c| c.priority <= def.priority);
        self.coercions.insert(insert_pos, def);

        // Detect cycles: if any type can reach itself, reject.
        let cycles = detect_cycles(self);
        if !cycles.is_empty() {
            // Roll back.
            self.coercions.remove(insert_pos);
            return Err(CoercionError::Cycle {
                path: cycles.into_iter().next().unwrap_or_default(),
            });
        }
        Ok(())
    }

    /// Find a direct (single-step) coercion from `from` to `to`, if any.
    pub fn find_direct(&self, from: &str, to: &str) -> Option<&CoercionDef> {
        self.coercions
            .iter()
            .find(|c| c.from_type == from && c.to_type == to)
    }

    /// Find the best coercion path from `from` to `to` using BFS.
    ///
    /// BFS explores shortest paths first.  If multiple shortest paths exist at
    /// the same depth, they are all collected and `CoercionResult::Ambiguous`
    /// is returned.  `max_len` limits the chain depth.
    pub fn find_path(&self, from: &str, to: &str, max_len: usize) -> CoercionResult {
        if from == to {
            return CoercionResult::NotFound;
        }

        // Special-case: check for a direct coercion first.
        if let Some(def) = self.find_direct(from, to) {
            return CoercionResult::Direct(def.clone());
        }

        if max_len <= 1 {
            return CoercionResult::NotFound;
        }

        // BFS state: (current_type, path_so_far)
        let mut queue: VecDeque<(String, Vec<CoercionDef>)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(from.to_string());

        // Seed with all one-step neighbours of `from`.
        for def in self.coercions.iter().filter(|c| c.from_type == from) {
            if !visited.contains(&def.to_type) {
                queue.push_back((def.to_type.clone(), vec![def.clone()]));
            }
        }

        let mut found_paths: Vec<CoercionPath> = Vec::new();
        let mut found_depth: Option<usize> = None;

        while let Some((current, path)) = queue.pop_front() {
            let depth = path.len();

            // If we already found paths at a shorter depth, stop.
            if let Some(fd) = found_depth {
                if depth > fd {
                    break;
                }
            }

            if depth > max_len {
                break;
            }

            if current == to {
                found_depth = Some(depth);
                found_paths.push(CoercionPath { steps: path });
                continue;
            }

            // Only continue expanding if we haven't yet reached the target.
            if found_depth.is_none() {
                for def in self.coercions.iter().filter(|c| c.from_type == current) {
                    if !visited.contains(&def.to_type) {
                        let mut new_path = path.clone();
                        new_path.push(def.clone());
                        queue.push_back((def.to_type.clone(), new_path));
                    }
                }
                // Mark as visited only when we won't explore further (prevents revisiting
                // but still allows same-depth alternative routes from the starting set).
                visited.insert(current);
            }
        }

        match found_paths.len() {
            0 => CoercionResult::NotFound,
            1 => {
                let path = found_paths.remove(0);
                if path.len() == 1 {
                    CoercionResult::Direct(
                        path.steps
                            .into_iter()
                            .next()
                            .unwrap_or_else(|| CoercionDef::new("", "", "", 0)),
                    )
                } else {
                    CoercionResult::Chain(path)
                }
            }
            _ => CoercionResult::Ambiguous(found_paths),
        }
    }
}

impl Default for CoercionDB {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Free functions ───────────────────────────────────────────────────────

/// Apply a coercion path to a string expression, producing a `CoercedExpr`.
///
/// The expression is wrapped with coercion function calls from left to right:
/// `step_n(… step_2(step_1(expr)) …)`.
pub fn apply_coercion(expr: &str, path: &CoercionPath) -> CoercedExpr {
    if path.steps.is_empty() {
        return CoercedExpr {
            original: expr.to_string(),
            original_type: String::new(),
            coerced: expr.to_string(),
            target_type: String::new(),
            steps_applied: Vec::new(),
        };
    }

    let original_type = path
        .steps
        .first()
        .map(|s| s.from_type.clone())
        .unwrap_or_default();
    let target_type = path
        .steps
        .last()
        .map(|s| s.to_type.clone())
        .unwrap_or_default();

    let mut current = expr.to_string();
    let mut steps_applied = Vec::with_capacity(path.steps.len());

    for step in &path.steps {
        current = format!("{}({})", step.fn_name, current);
        steps_applied.push(step.fn_name.clone());
    }

    CoercedExpr {
        original: expr.to_string(),
        original_type,
        coerced: current,
        target_type,
        steps_applied,
    }
}

/// Detect all cycles in the coercion graph using DFS.
///
/// Returns a list of cycles, where each cycle is a sequence of type-name
/// strings ending at the type it started from (e.g. `["A", "B", "A"]`).
pub fn detect_cycles(db: &CoercionDB) -> Vec<Vec<String>> {
    // Build adjacency list: from_type → [to_type, …]
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for def in &db.coercions {
        adj.entry(def.from_type.clone())
            .or_default()
            .push(def.to_type.clone());
    }

    let all_types: HashSet<String> = db
        .coercions
        .iter()
        .flat_map(|c| [c.from_type.clone(), c.to_type.clone()])
        .collect();

    let mut visited: HashSet<String> = HashSet::new();
    let mut in_stack: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    let mut cycles: Vec<Vec<String>> = Vec::new();

    for start in &all_types {
        if !visited.contains(start) {
            dfs_cycle(
                start,
                &adj,
                &mut visited,
                &mut in_stack,
                &mut stack,
                &mut cycles,
            );
        }
    }

    cycles
}

/// DFS helper for `detect_cycles`.
fn dfs_cycle(
    node: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    in_stack.insert(node.to_string());
    stack.push(node.to_string());

    if let Some(neighbours) = adj.get(node) {
        for neighbour in neighbours {
            if in_stack.contains(neighbour) {
                // Found a cycle; extract it from the stack.
                let cycle_start = stack.iter().position(|s| s == neighbour).unwrap_or(0);
                let mut cycle: Vec<String> = stack[cycle_start..].to_vec();
                cycle.push(neighbour.clone());
                cycles.push(cycle);
            } else if !visited.contains(neighbour) {
                dfs_cycle(neighbour, adj, visited, in_stack, stack, cycles);
            }
        }
    }

    stack.pop();
    in_stack.remove(node);
}

/// Build a `CoercionDB` pre-populated with standard Lean4 built-in coercions.
///
/// Numeric hierarchy: `Nat → Int → Rat → Real`
/// Logic: `Bool → Prop`, `Fin n → Nat`
pub fn standard_coercions() -> CoercionDB {
    let mut db = CoercionDB::new();

    // Numeric hierarchy
    let _ = db.add(CoercionDef::new("Nat", "Int", "Int.ofNat", 0));
    let _ = db.add(CoercionDef::new("Int", "Rat", "Rat.ofInt", 0));
    let _ = db.add(CoercionDef::new("Rat", "Real", "Real.ofRat", 0));

    // Logic / propositions
    let _ = db.add(CoercionDef::new("Bool", "Prop", "Bool.toProp", 0));

    // Finite types
    let _ = db.add(CoercionDef::new("Fin", "Nat", "Fin.val", 0));

    db
}

/// Render a coercion path as a human-readable string.
///
/// Example: `"Nat → Int → Rat"` via `"Int.ofNat, Rat.ofInt"`.
pub fn coercion_to_string(path: &CoercionPath) -> String {
    if path.steps.is_empty() {
        return String::from("<empty path>");
    }

    let types: Vec<String> = {
        let mut v: Vec<String> = path.steps.iter().map(|s| s.from_type.clone()).collect();
        if let Some(last) = path.steps.last() {
            v.push(last.to_type.clone());
        }
        v
    };

    let fns: Vec<&str> = path.steps.iter().map(|s| s.fn_name.as_str()).collect();

    format!("{} via {}", types.join(" → "), fns.join(", "))
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_db() -> CoercionDB {
        standard_coercions()
    }

    // ── CoercionDB::new ────────────────────────────────────────────────

    #[test]
    fn test_new_db_is_empty() {
        let db = CoercionDB::new();
        assert!(db.coercions.is_empty());
    }

    // ── CoercionDB::add ────────────────────────────────────────────────

    #[test]
    fn test_add_single_coercion() {
        let mut db = CoercionDB::new();
        let def = CoercionDef::new("A", "B", "coeAB", 10);
        assert!(db.add(def).is_ok());
        assert_eq!(db.coercions.len(), 1);
    }

    #[test]
    fn test_add_maintains_priority_sort() {
        let mut db = CoercionDB::new();
        db.add(CoercionDef::new("A", "B", "coeAB_hi", 100)).unwrap();
        db.add(CoercionDef::new("C", "D", "coeCD_lo", 5)).unwrap();
        db.add(CoercionDef::new("E", "F", "coeEF_mid", 50)).unwrap();
        assert!(db.coercions[0].priority <= db.coercions[1].priority);
        assert!(db.coercions[1].priority <= db.coercions[2].priority);
    }

    #[test]
    fn test_add_cycle_rejected() {
        let mut db = CoercionDB::new();
        db.add(CoercionDef::new("A", "B", "coeAB", 0)).unwrap();
        db.add(CoercionDef::new("B", "C", "coeBC", 0)).unwrap();
        // This would create A → B → C → A
        let result = db.add(CoercionDef::new("C", "A", "coeCA", 0));
        assert!(result.is_err());
        if let Err(CoercionError::Cycle { path }) = result {
            assert!(path.len() >= 2);
        }
    }

    #[test]
    fn test_add_self_loop_rejected() {
        let mut db = CoercionDB::new();
        let result = db.add(CoercionDef::new("A", "A", "id_A", 0));
        assert!(result.is_err());
    }

    // ── CoercionDB::find_direct ────────────────────────────────────────

    #[test]
    fn test_find_direct_present() {
        let db = make_db();
        let result = db.find_direct("Nat", "Int");
        assert!(result.is_some());
        assert_eq!(result.unwrap().fn_name, "Int.ofNat");
    }

    #[test]
    fn test_find_direct_absent() {
        let db = make_db();
        let result = db.find_direct("Nat", "Real");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_direct_wrong_direction() {
        let db = make_db();
        assert!(db.find_direct("Int", "Nat").is_none());
    }

    // ── CoercionDB::find_path ──────────────────────────────────────────

    #[test]
    fn test_find_path_direct_returns_direct() {
        let db = make_db();
        match db.find_path("Nat", "Int", 5) {
            CoercionResult::Direct(def) => assert_eq!(def.fn_name, "Int.ofNat"),
            other => panic!("expected Direct, got {:?}", other),
        }
    }

    #[test]
    fn test_find_path_chain_nat_to_rat() {
        let db = make_db();
        match db.find_path("Nat", "Rat", 5) {
            CoercionResult::Chain(path) => {
                assert_eq!(path.len(), 2);
                assert_eq!(path.from_type(), Some("Nat"));
                assert_eq!(path.to_type(), Some("Rat"));
            }
            other => panic!("expected Chain, got {:?}", other),
        }
    }

    #[test]
    fn test_find_path_chain_nat_to_real() {
        let db = make_db();
        match db.find_path("Nat", "Real", 10) {
            CoercionResult::Chain(path) => {
                assert_eq!(path.len(), 3);
                assert_eq!(path.to_type(), Some("Real"));
            }
            other => panic!("expected Chain, got {:?}", other),
        }
    }

    #[test]
    fn test_find_path_not_found() {
        let db = make_db();
        assert_eq!(db.find_path("Real", "Nat", 10), CoercionResult::NotFound);
    }

    #[test]
    fn test_find_path_max_len_exceeded() {
        let db = make_db();
        // Nat → Int → Rat → Real requires len=3; limit to 2.
        match db.find_path("Nat", "Real", 2) {
            CoercionResult::NotFound => {}
            other => panic!("expected NotFound with max_len=2, got {:?}", other),
        }
    }

    #[test]
    fn test_find_path_same_type_not_found() {
        let db = make_db();
        assert_eq!(db.find_path("Nat", "Nat", 10), CoercionResult::NotFound);
    }

    // ── apply_coercion ─────────────────────────────────────────────────

    #[test]
    fn test_apply_coercion_single_step() {
        let path = CoercionPath {
            steps: vec![CoercionDef::new("Nat", "Int", "Int.ofNat", 0)],
        };
        let result = apply_coercion("42", &path);
        assert_eq!(result.original, "42");
        assert_eq!(result.original_type, "Nat");
        assert_eq!(result.coerced, "Int.ofNat(42)");
        assert_eq!(result.target_type, "Int");
        assert_eq!(result.steps_applied, vec!["Int.ofNat"]);
    }

    #[test]
    fn test_apply_coercion_multi_step() {
        let path = CoercionPath {
            steps: vec![
                CoercionDef::new("Nat", "Int", "Int.ofNat", 0),
                CoercionDef::new("Int", "Rat", "Rat.ofInt", 0),
            ],
        };
        let result = apply_coercion("n", &path);
        assert_eq!(result.coerced, "Rat.ofInt(Int.ofNat(n))");
        assert_eq!(result.steps_applied.len(), 2);
    }

    #[test]
    fn test_apply_coercion_empty_path() {
        let path = CoercionPath { steps: vec![] };
        let result = apply_coercion("x", &path);
        assert_eq!(result.coerced, "x");
        assert!(result.steps_applied.is_empty());
    }

    // ── detect_cycles ──────────────────────────────────────────────────

    #[test]
    fn test_detect_cycles_clean_db() {
        let db = make_db();
        let cycles = detect_cycles(&db);
        assert!(cycles.is_empty(), "standard coercions should be acyclic");
    }

    #[test]
    fn test_detect_cycles_finds_cycle() {
        // Build a db with a known cycle without going through add().
        let mut db = CoercionDB::new();
        db.coercions.push(CoercionDef::new("X", "Y", "f", 0));
        db.coercions.push(CoercionDef::new("Y", "X", "g", 0));
        let cycles = detect_cycles(&db);
        assert!(!cycles.is_empty());
    }

    // ── standard_coercions ─────────────────────────────────────────────

    #[test]
    fn test_standard_coercions_has_nat_int() {
        let db = standard_coercions();
        assert!(db.find_direct("Nat", "Int").is_some());
    }

    #[test]
    fn test_standard_coercions_has_bool_prop() {
        let db = standard_coercions();
        assert!(db.find_direct("Bool", "Prop").is_some());
    }

    #[test]
    fn test_standard_coercions_has_fin_nat() {
        let db = standard_coercions();
        assert!(db.find_direct("Fin", "Nat").is_some());
    }

    // ── coercion_to_string ─────────────────────────────────────────────

    #[test]
    fn test_coercion_to_string_single_step() {
        let path = CoercionPath {
            steps: vec![CoercionDef::new("Nat", "Int", "Int.ofNat", 0)],
        };
        let s = coercion_to_string(&path);
        assert!(s.contains("Nat"));
        assert!(s.contains("Int"));
        assert!(s.contains("Int.ofNat"));
    }

    #[test]
    fn test_coercion_to_string_multi_step() {
        let path = CoercionPath {
            steps: vec![
                CoercionDef::new("Nat", "Int", "Int.ofNat", 0),
                CoercionDef::new("Int", "Rat", "Rat.ofInt", 0),
            ],
        };
        let s = coercion_to_string(&path);
        assert!(s.contains("Nat"));
        assert!(s.contains("Rat"));
    }

    #[test]
    fn test_coercion_to_string_empty_path() {
        let path = CoercionPath { steps: vec![] };
        let s = coercion_to_string(&path);
        assert!(s.contains("empty"));
    }

    // ── CoercionPath helpers ───────────────────────────────────────────

    #[test]
    fn test_path_from_to_type() {
        let path = CoercionPath {
            steps: vec![
                CoercionDef::new("A", "B", "f", 0),
                CoercionDef::new("B", "C", "g", 0),
            ],
        };
        assert_eq!(path.from_type(), Some("A"));
        assert_eq!(path.to_type(), Some("C"));
    }

    #[test]
    fn test_path_len() {
        let db = make_db();
        match db.find_path("Int", "Real", 10) {
            CoercionResult::Chain(p) => assert_eq!(p.len(), 2),
            other => panic!("expected chain, got {:?}", other),
        }
    }

    // ── Display impls ──────────────────────────────────────────────────

    #[test]
    fn test_coercion_def_display() {
        let def = CoercionDef::new("Nat", "Int", "Int.ofNat", 0);
        let s = format!("{}", def);
        assert!(s.contains("Nat"));
        assert!(s.contains("Int"));
        assert!(s.contains("Int.ofNat"));
    }

    #[test]
    fn test_coercion_error_display_cycle() {
        let err = CoercionError::Cycle {
            path: vec!["A".to_string(), "B".to_string(), "A".to_string()],
        };
        assert!(format!("{}", err).contains("cycle"));
    }

    #[test]
    fn test_coercion_error_display_too_long() {
        let err = CoercionError::TooLong { length: 10, max: 5 };
        assert!(format!("{}", err).contains("10"));
    }

    #[test]
    fn test_coercion_error_display_ambiguous() {
        let err = CoercionError::Ambiguous;
        assert!(format!("{}", err).contains("ambiguous"));
    }

    #[test]
    fn test_coerced_expr_display() {
        let ce = CoercedExpr {
            original: "42".to_string(),
            original_type: "Nat".to_string(),
            coerced: "Int.ofNat(42)".to_string(),
            target_type: "Int".to_string(),
            steps_applied: vec!["Int.ofNat".to_string()],
        };
        let s = format!("{}", ce);
        assert!(s.contains("42"));
        assert!(s.contains("Int.ofNat"));
    }

    // ── Priority ordering ──────────────────────────────────────────────

    #[test]
    fn test_priority_ordering_selects_lower_priority_first() {
        let mut db = CoercionDB::new();
        db.add(CoercionDef::new("A", "B", "coe_high", 100)).unwrap();
        db.add(CoercionDef::new("A", "B", "coe_low", 1)).unwrap();
        // find_direct returns the first in sorted order → lower priority number first
        let found = db.find_direct("A", "B").unwrap();
        assert_eq!(found.fn_name, "coe_low");
    }
}
