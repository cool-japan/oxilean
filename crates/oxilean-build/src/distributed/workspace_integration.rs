//! Workspace-aware distributed build planner.
//!
//! Reads `oxilake.toml` workspace manifests and decomposes workspace members
//! into independent `DistributedBuildUnit` tasks, one per member package.
//! Each unit can be dispatched to a separate worker in the distributed build
//! cluster managed by [`WorkerRegistry`].
//!
//! # Design rationale
//!
//! Rather than depending on the `oxilake` crate (which would create a circular
//! dependency — oxilake depends on oxilean-build), this module reads
//! `oxilake.toml` files directly using the `toml` crate.  The parsing is
//! intentionally minimal: only the `[workspace] members = [...]` list is
//! needed to schedule builds.

use super::types::{DistributedTaskState, JobSchedulerKind, WorkerRegistry};
use std::fmt;
use std::path::{Path, PathBuf};

// ─────────────────────────────────────────────────────────────────────────────
// DistributedBuildUnit
// ─────────────────────────────────────────────────────────────────────────────

/// A single workspace member treated as one distributable build unit.
///
/// Each member of an oxilake workspace is compiled independently and can
/// therefore be dispatched to a distinct remote worker.
#[derive(Debug, Clone)]
pub struct DistributedBuildUnit {
    /// Package name derived from the workspace members list in `oxilake.toml`.
    pub package_name: String,
    /// Absolute path to the member's directory.
    pub member_path: PathBuf,
    /// Current lifecycle state of this build unit.
    pub state: DistributedTaskState,
    /// Priority — lower numbers run first; typically set to dependency depth.
    pub priority: u32,
}

impl DistributedBuildUnit {
    /// Create a new unit in the [`DistributedTaskState::Pending`] state.
    pub fn new(package_name: impl Into<String>, member_path: PathBuf) -> Self {
        Self {
            package_name: package_name.into(),
            member_path,
            state: DistributedTaskState::Pending,
            priority: 100,
        }
    }

    /// Override the priority (dependency depth order).
    ///
    /// Lower values are scheduled before higher values.
    pub fn with_priority(mut self, p: u32) -> Self {
        self.priority = p;
        self
    }

    /// Transition this unit to the [`DistributedTaskState::Running`] state.
    pub fn mark_running(&mut self) {
        self.state = DistributedTaskState::Running;
    }

    /// Transition this unit to the [`DistributedTaskState::Done`] state.
    pub fn mark_done(&mut self) {
        self.state = DistributedTaskState::Done;
    }

    /// Transition this unit to the [`DistributedTaskState::Failed`] state.
    pub fn mark_failed(&mut self) {
        self.state = DistributedTaskState::Failed;
    }

    /// Whether this unit is still waiting to be scheduled.
    pub fn is_pending(&self) -> bool {
        self.state == DistributedTaskState::Pending
    }

    /// Whether this unit has completed successfully.
    pub fn is_done(&self) -> bool {
        self.state == DistributedTaskState::Done
    }
}

impl fmt::Display for DistributedBuildUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BuildUnit({}, priority={}, state={})",
            self.package_name, self.priority, self.state
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WorkspaceManifest
// ─────────────────────────────────────────────────────────────────────────────

/// A minimal representation of the `[workspace]` section of an `oxilake.toml`.
///
/// This mirrors the structure written by `oxilake new --workspace` but does
/// not depend on the `oxilake` crate's full manifest types.
#[derive(Debug, Clone)]
pub struct WorkspaceManifest {
    /// Relative member paths declared under `[workspace] members = [...]`.
    pub members: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// WorkspaceError
// ─────────────────────────────────────────────────────────────────────────────

/// Errors that can arise when reading or interpreting an `oxilake.toml`
/// workspace manifest.
#[derive(Debug)]
pub enum WorkspaceError {
    /// The directory has no `oxilake.toml`, or the file lacks a `[workspace]`
    /// section entirely.
    NotAWorkspace,
    /// An I/O error occurred while reading the manifest file.
    IoError(String),
    /// The manifest is present but could not be parsed (e.g. malformed TOML,
    /// or the members value is not a string array).
    MalformedManifest(String),
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceError::NotAWorkspace => write!(
                f,
                "not a workspace (no oxilake.toml with [workspace] section)"
            ),
            WorkspaceError::IoError(e) => write!(f, "I/O error reading workspace manifest: {e}"),
            WorkspaceError::MalformedManifest(e) => {
                write!(f, "malformed workspace manifest: {e}")
            }
        }
    }
}

impl std::error::Error for WorkspaceError {}

// ─────────────────────────────────────────────────────────────────────────────
// Manifest parsing
// ─────────────────────────────────────────────────────────────────────────────

/// Parse the `oxilake.toml` file located at `root_dir/oxilake.toml` and
/// extract the workspace member list.
///
/// # Errors
///
/// - [`WorkspaceError::IoError`] — the file does not exist or cannot be read.
/// - [`WorkspaceError::NotAWorkspace`] — the file lacks a `[workspace]` section.
/// - [`WorkspaceError::MalformedManifest`] — the TOML is invalid or the
///   `members` value is not a string array.
pub fn parse_workspace_manifest(root_dir: &Path) -> Result<WorkspaceManifest, WorkspaceError> {
    let manifest_path = root_dir.join("oxilake.toml");
    let content = std::fs::read_to_string(&manifest_path)
        .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
    parse_manifest_content(&content)
}

/// Parse raw TOML content and extract workspace members.
///
/// Accepts the textual content of an `oxilake.toml` file.  Uses a lightweight
/// line-by-line parser rather than full serde deserialization so that
/// `oxilean-build` does not need to carry the `serde` / `serde_derive` weight.
/// The `toml` crate is still used for value extraction to handle edge cases
/// (quoted strings, multi-line arrays, etc.).
fn parse_manifest_content(content: &str) -> Result<WorkspaceManifest, WorkspaceError> {
    // Fast path: reject files that lack a [workspace] section entirely.
    if !content.contains("[workspace]") {
        return Err(WorkspaceError::NotAWorkspace);
    }

    // Use the `toml` crate to parse the whole document so we handle all valid
    // TOML syntax correctly (multi-line arrays, quoted strings, etc.).
    let doc: toml::Value = content
        .parse::<toml::Value>()
        .map_err(|e| WorkspaceError::MalformedManifest(e.to_string()))?;

    let workspace_table = doc.get("workspace").ok_or(WorkspaceError::NotAWorkspace)?;

    // `members` is optional — an absent key means an empty workspace.
    let members_value = match workspace_table.get("members") {
        Some(v) => v,
        None => {
            return Ok(WorkspaceManifest {
                members: Vec::new(),
            })
        }
    };

    let members_array = members_value.as_array().ok_or_else(|| {
        WorkspaceError::MalformedManifest(
            "workspace.members must be an array of strings".to_string(),
        )
    })?;

    let mut members: Vec<String> = Vec::with_capacity(members_array.len());
    for (idx, item) in members_array.iter().enumerate() {
        match item.as_str() {
            Some(s) if !s.is_empty() => members.push(s.to_string()),
            Some(_) => {
                // Skip empty strings silently — they are no-ops.
            }
            None => {
                return Err(WorkspaceError::MalformedManifest(format!(
                    "workspace.members[{idx}] is not a string"
                )));
            }
        }
    }

    Ok(WorkspaceManifest { members })
}

// ─────────────────────────────────────────────────────────────────────────────
// workspace_to_build_units
// ─────────────────────────────────────────────────────────────────────────────

/// Decompose an oxilake workspace into a list of distributable build units.
///
/// Reads `root_dir/oxilake.toml`, resolves each member path relative to
/// `root_dir`, and returns one [`DistributedBuildUnit`] per member.
///
/// The units are returned sorted by ascending priority (i.e. in the order
/// they appear in the `members` list, which is a reasonable topological
/// approximation when explicit dependency information is not yet available).
///
/// # Errors
///
/// Propagates any [`WorkspaceError`] from [`parse_workspace_manifest`].
pub fn workspace_to_build_units(
    root_dir: &Path,
) -> Result<Vec<DistributedBuildUnit>, WorkspaceError> {
    let manifest = parse_workspace_manifest(root_dir)?;

    let mut units: Vec<DistributedBuildUnit> = manifest
        .members
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let member_path = root_dir.join(name);
            DistributedBuildUnit::new(name.clone(), member_path).with_priority(i as u32)
        })
        .collect();

    // Sort by priority (ascending) so callers get a deterministic schedule.
    units.sort_by_key(|u| u.priority);

    Ok(units)
}

// ─────────────────────────────────────────────────────────────────────────────
// plan_workspace_build
// ─────────────────────────────────────────────────────────────────────────────

/// Plan a workspace build: assign each [`DistributedBuildUnit`] to a worker
/// from `registry` using the given `strategy`.
///
/// Returns a list of `(unit_index, worker_id)` assignments, one per unit.
/// If the registry is empty, returns an empty list (no assignments possible).
///
/// # Strategy behaviour
///
/// - [`JobSchedulerKind::LeastLoaded`] — round-robins across worker IDs,
///   which approximates least-loaded when units are of similar cost and no
///   real-time load information is available at plan time.
/// - [`JobSchedulerKind::RoundRobin`] — pure modular distribution.
/// - [`JobSchedulerKind::PriorityWeighted`] — currently identical to round-robin
///   at plan time; a runtime scheduler would refine this once actual load is
///   known.
pub fn plan_workspace_build(
    units: &[DistributedBuildUnit],
    registry: &WorkerRegistry,
    strategy: JobSchedulerKind,
) -> Vec<(usize, String)> {
    let mut worker_ids: Vec<String> = registry
        .worker_ids()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    if worker_ids.is_empty() {
        return Vec::new();
    }

    // Sort for deterministic assignment order regardless of HashMap iteration order.
    worker_ids.sort();

    units
        .iter()
        .enumerate()
        .map(|(i, _unit)| {
            let worker = match strategy {
                JobSchedulerKind::LeastLoaded
                | JobSchedulerKind::RoundRobin
                | JobSchedulerKind::PriorityWeighted => worker_ids[i % worker_ids.len()].clone(),
            };
            (i, worker)
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::types::{DistributedTaskState, RemoteWorker, WorkerRegistry};
    use std::fs;

    // ── helpers ───────────────────────────────────────────────────────────────

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dirs");
        }
        fs::write(path, content).expect("write file");
    }

    fn registry_with_workers(ids: &[&str]) -> WorkerRegistry {
        let mut reg = WorkerRegistry::new();
        for &id in ids {
            reg.register(RemoteWorker::new(id, &format!("host-{id}:8080"), 4));
        }
        reg
    }

    // ── 1. single member ──────────────────────────────────────────────────────

    #[test]
    fn test_parse_manifest_single_member() {
        let content = "[workspace]\nmembers = [\"pkg_a\"]\n";
        let manifest = parse_manifest_content(content).expect("parse single member");
        assert_eq!(manifest.members.len(), 1);
        assert_eq!(manifest.members[0], "pkg_a");
    }

    // ── 2. multiple members ───────────────────────────────────────────────────

    #[test]
    fn test_parse_manifest_multi_member() {
        let content = "[workspace]\nmembers = [\"a\", \"b\", \"c\"]\n";
        let manifest = parse_manifest_content(content).expect("parse multi member");
        assert_eq!(manifest.members.len(), 3);
        assert_eq!(manifest.members, vec!["a", "b", "c"]);
    }

    // ── 3. not a workspace ────────────────────────────────────────────────────

    #[test]
    fn test_parse_manifest_not_a_workspace() {
        let content = "[package]\nname = \"standalone\"\nversion = \"0.1.0\"\n";
        let result = parse_manifest_content(content);
        assert!(
            matches!(result, Err(WorkspaceError::NotAWorkspace)),
            "expected NotAWorkspace, got: {result:?}"
        );
    }

    // ── 4. empty members array ────────────────────────────────────────────────

    #[test]
    fn test_parse_manifest_empty_members() {
        let content = "[workspace]\nmembers = []\n";
        let manifest = parse_manifest_content(content).expect("parse empty members");
        assert_eq!(manifest.members.len(), 0);
    }

    // ── 5. workspace_to_build_units from real temp dir ────────────────────────

    #[test]
    fn test_workspace_to_build_units_from_dir() {
        let base = std::env::temp_dir().join("oxilean_build_ws_integration_test_005");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create temp dir");

        write_file(
            &base.join("oxilake.toml"),
            "[workspace]\nmembers = [\"alpha\", \"beta\"]\n",
        );

        // Create member directories so the paths resolve.
        fs::create_dir_all(base.join("alpha")).expect("create alpha");
        fs::create_dir_all(base.join("beta")).expect("create beta");

        let units = workspace_to_build_units(&base).expect("build units");
        assert_eq!(units.len(), 2, "should have one unit per member");

        // Units are sorted by priority (insertion order).
        assert_eq!(units[0].package_name, "alpha");
        assert_eq!(units[0].member_path, base.join("alpha"));
        assert_eq!(units[1].package_name, "beta");
        assert_eq!(units[1].member_path, base.join("beta"));

        fs::remove_dir_all(&base).ok();
    }

    // ── 6. new unit starts in Pending state ───────────────────────────────────

    #[test]
    fn test_distributed_build_unit_state() {
        let unit = DistributedBuildUnit::new("my-pkg", PathBuf::from("/tmp/my-pkg"));
        assert_eq!(
            unit.state,
            DistributedTaskState::Pending,
            "new unit must start in Pending state"
        );
        assert!(unit.is_pending());
        assert!(!unit.is_done());
    }

    // ── 7. plan_workspace_build assigns all units ─────────────────────────────

    #[test]
    fn test_plan_workspace_build_assigns_all() {
        let units = vec![
            DistributedBuildUnit::new("pkg-a", PathBuf::from("/tmp/pkg-a")),
            DistributedBuildUnit::new("pkg-b", PathBuf::from("/tmp/pkg-b")),
            DistributedBuildUnit::new("pkg-c", PathBuf::from("/tmp/pkg-c")),
        ];
        let registry = registry_with_workers(&["w1", "w2"]);

        let assignments = plan_workspace_build(&units, &registry, JobSchedulerKind::LeastLoaded);

        assert_eq!(assignments.len(), 3, "all units must receive an assignment");

        // Every unit index 0..2 must appear exactly once.
        let mut unit_indices: Vec<usize> = assignments.iter().map(|(i, _)| *i).collect();
        unit_indices.sort();
        assert_eq!(unit_indices, vec![0, 1, 2]);

        // All assigned worker IDs must be in {w1, w2}.
        for (_, wid) in &assignments {
            assert!(wid == "w1" || wid == "w2", "unexpected worker id: {wid}");
        }
    }

    // ── 8. WorkspaceError Display impls ──────────────────────────────────────

    #[test]
    fn test_workspace_error_display() {
        let not_ws = WorkspaceError::NotAWorkspace;
        let io_err = WorkspaceError::IoError("permission denied".to_string());
        let malformed = WorkspaceError::MalformedManifest("unexpected key".to_string());

        let s_not_ws = format!("{not_ws}");
        let s_io = format!("{io_err}");
        let s_mal = format!("{malformed}");

        assert!(
            !s_not_ws.is_empty(),
            "NotAWorkspace display must not be empty"
        );
        assert!(
            s_io.contains("permission denied"),
            "IoError display must include the inner message"
        );
        assert!(
            s_mal.contains("unexpected key"),
            "MalformedManifest display must include the inner message"
        );

        // Ensure std::error::Error is satisfied (source() should not panic).
        let _: &dyn std::error::Error = &WorkspaceError::NotAWorkspace;
        let _: &dyn std::error::Error = &WorkspaceError::IoError("x".to_string());
        let _: &dyn std::error::Error = &WorkspaceError::MalformedManifest("x".to_string());
    }

    // ── 9. plan with no workers returns empty ────────────────────────────────

    #[test]
    fn test_plan_workspace_build_no_workers() {
        let units = vec![DistributedBuildUnit::new("pkg-a", PathBuf::from("/tmp/a"))];
        let registry = WorkerRegistry::new(); // empty
        let assignments = plan_workspace_build(&units, &registry, JobSchedulerKind::RoundRobin);
        assert!(
            assignments.is_empty(),
            "no workers => no assignments possible"
        );
    }

    // ── 10. state transitions ────────────────────────────────────────────────

    #[test]
    fn test_distributed_build_unit_state_transitions() {
        let mut unit = DistributedBuildUnit::new("pkg-x", PathBuf::from("/tmp/pkg-x"));
        assert!(unit.is_pending());

        unit.mark_running();
        assert_eq!(unit.state, DistributedTaskState::Running);
        assert!(!unit.is_pending());

        unit.mark_done();
        assert!(unit.is_done());

        let mut unit2 = DistributedBuildUnit::new("pkg-y", PathBuf::from("/tmp/pkg-y"));
        unit2.mark_failed();
        assert_eq!(unit2.state, DistributedTaskState::Failed);
    }

    // ── 11. Display for DistributedBuildUnit ─────────────────────────────────

    #[test]
    fn test_distributed_build_unit_display() {
        let unit =
            DistributedBuildUnit::new("my-lib", PathBuf::from("/tmp/my-lib")).with_priority(5);
        let s = format!("{unit}");
        assert!(s.contains("my-lib"), "display must include package name");
        assert!(s.contains("5"), "display must include priority");
        assert!(s.contains("pending"), "display must include state");
    }

    // ── 12. parse_workspace_manifest reads from actual file ──────────────────

    #[test]
    fn test_parse_workspace_manifest_from_file() {
        let dir = std::env::temp_dir().join("oxilean_build_ws_integration_test_012");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");

        write_file(
            &dir.join("oxilake.toml"),
            "[workspace]\nmembers = [\"core\", \"utils\"]\n",
        );

        let manifest = parse_workspace_manifest(&dir).expect("parse from file");
        assert_eq!(manifest.members, vec!["core", "utils"]);

        fs::remove_dir_all(&dir).ok();
    }

    // ── 13. parse_workspace_manifest returns IoError for missing file ────────

    #[test]
    fn test_parse_workspace_manifest_missing_file() {
        let dir = std::env::temp_dir().join("oxilean_build_ws_integration_test_013_nonexistent");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");
        // Do NOT write oxilake.toml — file absent.

        let result = parse_workspace_manifest(&dir);
        assert!(
            matches!(result, Err(WorkspaceError::IoError(_))),
            "missing file must yield IoError, got: {result:?}"
        );

        fs::remove_dir_all(&dir).ok();
    }
}
