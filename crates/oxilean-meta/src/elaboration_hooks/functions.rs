//! Functions for elaboration hooks — manage and fire hooks during elaboration.

use super::types::{ElabHook, ElabHookKind, HookEvent, HookRegistry, HookResult, HookTrace};

// ─── HookRegistry methods ─────────────────────────────────────────────────────

impl HookRegistry {
    /// Register a new hook and return its assigned id.
    pub fn register(&mut self, kind: ElabHookKind, name: &str, priority: i32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.hooks.push(ElabHook::new(id, kind, priority, name));
        id
    }

    /// Unregister a hook by id.  Returns `true` if the hook existed.
    pub fn unregister(&mut self, id: u64) -> bool {
        let len_before = self.hooks.len();
        self.hooks.retain(|h| h.id != id);
        self.hooks.len() < len_before
    }

    /// Return all hooks matching `kind`, sorted by priority (ascending — lower fires first).
    pub fn hooks_for(&self, kind: &ElabHookKind) -> Vec<&ElabHook> {
        let mut matched: Vec<&ElabHook> = self.hooks.iter().filter(|h| &h.kind == kind).collect();
        matched.sort_by_key(|h| h.priority);
        matched
    }
}

// ─── Firing hooks ─────────────────────────────────────────────────────────────

/// Fire all hooks in `registry` that match `event.kind`, in priority order.
///
/// Returns the merged `HookResult` and a `HookTrace` of every hook that ran.
///
/// Hook simulation: since hooks are purely descriptive in this reflection layer
/// (they carry no Rust closure), we simulate their result deterministically:
/// - Hooks named `"abort_*"` or containing `"abort"` produce `Abort`.
/// - Hooks named `"modify_*"` or containing `"modify"` produce `Modify`.
/// - All other hooks produce `Continue`.
pub fn fire_hooks(registry: &HookRegistry, event: &HookEvent) -> (HookResult, HookTrace) {
    let hooks = registry.hooks_for(&event.kind);
    let mut trace = HookTrace::new();
    let mut results: Vec<HookResult> = Vec::new();

    for hook in hooks {
        let result = simulate_hook_result(hook, event);
        trace.record(hook.clone(), result.clone());
        results.push(result);
    }

    let merged = merge_results(&results);
    (merged, trace)
}

/// Simulate a hook's result based on its name and the event context.
///
/// This is the reflective simulation policy:
/// - Name contains `"abort"` → `Abort` with a message derived from event context.
/// - Name contains `"modify"` → `Modify` with the event expr (or a placeholder).
/// - Otherwise → `Continue`.
fn simulate_hook_result(hook: &ElabHook, event: &HookEvent) -> HookResult {
    let name_lower = hook.name.to_lowercase();
    if name_lower.contains("abort") {
        let msg = event
            .error
            .clone()
            .unwrap_or_else(|| format!("Aborted by hook '{}'", hook.name));
        HookResult::Abort(msg)
    } else if name_lower.contains("modify") {
        let new_expr = event
            .expr
            .clone()
            .map(|e| format!("modified({e})"))
            .unwrap_or_else(|| format!("modified_by_{}", hook.name));
        HookResult::Modify(new_expr)
    } else {
        HookResult::Continue
    }
}

// ─── Merging results ──────────────────────────────────────────────────────────

/// Merge a slice of `HookResult`s into a single result.
///
/// Policy:
/// - The **first** `Abort` wins (returns immediately).
/// - Among non-Abort results, the **last** `Modify` wins.
/// - If no Abort and no Modify, returns `Continue`.
pub fn merge_results(results: &[HookResult]) -> HookResult {
    let mut last_modify: Option<HookResult> = None;

    for result in results {
        match result {
            HookResult::Abort(_) => return result.clone(),
            HookResult::Modify(_) => last_modify = Some(result.clone()),
            HookResult::Continue => {}
        }
    }

    last_modify.unwrap_or(HookResult::Continue)
}

// ─── Trace formatting ─────────────────────────────────────────────────────────

/// Render a `HookTrace` as a human-readable string.
pub fn trace_to_string(trace: &HookTrace) -> String {
    if trace.is_empty() {
        return "(no hooks fired)".to_string();
    }
    trace
        .events
        .iter()
        .enumerate()
        .map(|(i, (hook, result))| format!("[{i}] {} => {result}", hook))
        .collect::<Vec<_>>()
        .join("\n")
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elaboration_hooks::types::{ElabHookKind, HookEvent, HookRegistry, HookResult};

    fn make_registry() -> HookRegistry {
        HookRegistry::new()
    }

    // ── register / unregister ────────────────────────────────────────────────

    #[test]
    fn test_register_returns_unique_ids() {
        let mut reg = make_registry();
        let id1 = reg.register(ElabHookKind::PreElaborate, "hook1", 0);
        let id2 = reg.register(ElabHookKind::PreElaborate, "hook2", 0);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_register_increments_id() {
        let mut reg = make_registry();
        let id1 = reg.register(ElabHookKind::PostElaborate, "a", 0);
        let id2 = reg.register(ElabHookKind::PostElaborate, "b", 0);
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    fn test_unregister_existing_hook() {
        let mut reg = make_registry();
        let id = reg.register(ElabHookKind::OnError, "err_hook", 0);
        assert_eq!(reg.hooks.len(), 1);
        let removed = reg.unregister(id);
        assert!(removed);
        assert!(reg.hooks.is_empty());
    }

    #[test]
    fn test_unregister_nonexistent_hook() {
        let mut reg = make_registry();
        let removed = reg.unregister(999);
        assert!(!removed);
    }

    #[test]
    fn test_hooks_for_filters_by_kind() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::PreElaborate, "pre", 0);
        reg.register(ElabHookKind::PostElaborate, "post", 0);
        reg.register(ElabHookKind::PreElaborate, "pre2", 0);
        let pre_hooks = reg.hooks_for(&ElabHookKind::PreElaborate);
        assert_eq!(pre_hooks.len(), 2);
        let post_hooks = reg.hooks_for(&ElabHookKind::PostElaborate);
        assert_eq!(post_hooks.len(), 1);
    }

    #[test]
    fn test_hooks_for_sorted_by_priority() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::OnDeclaration, "low", 10);
        reg.register(ElabHookKind::OnDeclaration, "high", -5);
        reg.register(ElabHookKind::OnDeclaration, "mid", 0);
        let hooks = reg.hooks_for(&ElabHookKind::OnDeclaration);
        assert_eq!(hooks[0].priority, -5);
        assert_eq!(hooks[1].priority, 0);
        assert_eq!(hooks[2].priority, 10);
    }

    #[test]
    fn test_hooks_for_empty_kind() {
        let reg = make_registry();
        let hooks = reg.hooks_for(&ElabHookKind::OnTacticBegin);
        assert!(hooks.is_empty());
    }

    // ── fire_hooks ───────────────────────────────────────────────────────────

    #[test]
    fn test_fire_hooks_no_hooks_returns_continue() {
        let reg = make_registry();
        let event = HookEvent::new(ElabHookKind::PreElaborate);
        let (result, trace) = fire_hooks(&reg, &event);
        assert_eq!(result, HookResult::Continue);
        assert!(trace.is_empty());
    }

    #[test]
    fn test_fire_hooks_continue_hook() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::PreElaborate, "logger", 0);
        let event = HookEvent::new(ElabHookKind::PreElaborate);
        let (result, trace) = fire_hooks(&reg, &event);
        assert_eq!(result, HookResult::Continue);
        assert_eq!(trace.len(), 1);
    }

    #[test]
    fn test_fire_hooks_abort_hook() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::OnError, "abort_on_error", 0);
        let event = HookEvent::new(ElabHookKind::OnError).with_error("type mismatch");
        let (result, _trace) = fire_hooks(&reg, &event);
        assert!(matches!(result, HookResult::Abort(_)));
    }

    #[test]
    fn test_fire_hooks_modify_hook() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::PostElaborate, "modify_expr", 0);
        let event = HookEvent::new(ElabHookKind::PostElaborate).with_expr("fun x => x");
        let (result, _trace) = fire_hooks(&reg, &event);
        assert!(matches!(result, HookResult::Modify(_)));
    }

    #[test]
    fn test_fire_hooks_trace_records_all() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::PreElaborate, "h1", 0);
        reg.register(ElabHookKind::PreElaborate, "h2", 1);
        let event = HookEvent::new(ElabHookKind::PreElaborate);
        let (_result, trace) = fire_hooks(&reg, &event);
        assert_eq!(trace.len(), 2);
    }

    // ── merge_results ────────────────────────────────────────────────────────

    #[test]
    fn test_merge_empty_is_continue() {
        let merged = merge_results(&[]);
        assert_eq!(merged, HookResult::Continue);
    }

    #[test]
    fn test_merge_all_continue() {
        let results = vec![HookResult::Continue, HookResult::Continue];
        assert_eq!(merge_results(&results), HookResult::Continue);
    }

    #[test]
    fn test_merge_abort_wins() {
        let results = vec![
            HookResult::Continue,
            HookResult::Abort("bad".into()),
            HookResult::Modify("x".into()),
        ];
        assert!(matches!(merge_results(&results), HookResult::Abort(_)));
    }

    #[test]
    fn test_merge_last_modify_wins() {
        let results = vec![
            HookResult::Modify("first".into()),
            HookResult::Continue,
            HookResult::Modify("last".into()),
        ];
        assert_eq!(merge_results(&results), HookResult::Modify("last".into()));
    }

    #[test]
    fn test_merge_first_abort_wins_over_later_abort() {
        let results = vec![
            HookResult::Abort("first".into()),
            HookResult::Abort("second".into()),
        ];
        assert_eq!(merge_results(&results), HookResult::Abort("first".into()));
    }

    // ── trace_to_string ───────────────────────────────────────────────────────

    #[test]
    fn test_trace_to_string_empty() {
        let trace = HookTrace::new();
        let s = trace_to_string(&trace);
        assert!(s.contains("no hooks"));
    }

    #[test]
    fn test_trace_to_string_non_empty() {
        let mut reg = make_registry();
        reg.register(ElabHookKind::OnDeclaration, "decl_hook", 0);
        let event = HookEvent::new(ElabHookKind::OnDeclaration).with_decl("myDef");
        let (_result, trace) = fire_hooks(&reg, &event);
        let s = trace_to_string(&trace);
        assert!(s.contains("decl_hook"));
    }

    #[test]
    fn test_hook_event_builder() {
        let event = HookEvent::new(ElabHookKind::OnError)
            .with_decl("foo")
            .with_expr("bar")
            .with_error("oops");
        assert_eq!(event.decl_name, Some("foo".into()));
        assert_eq!(event.expr, Some("bar".into()));
        assert_eq!(event.error, Some("oops".into()));
    }
}
