//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::types::{
    ArtifactKind, ArtifactPackager, BuildArtifact, CodeGenStep, ConditionalScript,
    CrossCompileScript, ErrorHandlingStrategy, GeneratorKind, HookManager, ParallelScriptExecutor,
    ProtobufCompileOptions, RetryPolicy, ScriptCache, ScriptCacheKey, ScriptCondition, ScriptDef,
    ScriptDependencyTracker, ScriptEnvironment, ScriptError, ScriptErrorHandler, ScriptKind,
    ScriptPipeline, ScriptProfile, ScriptProfiler, ScriptReport, ScriptResult, ScriptRunner,
    ScriptVariables, TargetTriple,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_script_def_creation() {
        let script = ScriptDef::pre_build("gen-version", "echo 1.0.0")
            .set_env("VERSION", "1.0.0")
            .set_fail_on_error(true)
            .set_description("Generate version file");
        assert_eq!(script.name, "gen-version");
        assert_eq!(script.kind, ScriptKind::PreBuild);
        assert!(script.fail_on_error);
        assert!(script.env.contains_key("VERSION"));
    }
    #[test]
    fn test_script_runner() {
        let mut runner = ScriptRunner::new();
        runner.add_script(ScriptDef::pre_build("step1", "echo hello"));
        runner.add_script(ScriptDef::pre_build("step2", "echo world"));
        runner.add_script(ScriptDef::post_build("step3", "echo done"));
        let results = runner
            .run_pre_build()
            .expect("build operation should succeed");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
    }
    #[test]
    fn test_script_runner_post_build() {
        let mut runner = ScriptRunner::new();
        runner.add_script(ScriptDef::post_build("cleanup", "rm -rf tmp"));
        let results = runner
            .run_post_build()
            .expect("build operation should succeed");
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
    }
    #[test]
    fn test_code_gen_step() {
        let step = CodeGenStep::new(
            "ffi-gen",
            Path::new("ffi.toml"),
            Path::new("generated/"),
            GeneratorKind::FfiBindings,
        )
        .set_option("language", "c");
        assert_eq!(step.generator, GeneratorKind::FfiBindings);
        assert_eq!(step.options.get("language"), Some(&"c".to_string()));
        let script_def = step.to_script_def();
        assert_eq!(script_def.kind, ScriptKind::CodeGen);
    }
    #[test]
    fn test_hook_manager() {
        let mut manager = HookManager::new();
        manager.register(ScriptDef::pre_build("hook1", "echo pre"));
        manager.register(ScriptDef::post_build("hook2", "echo post"));
        assert_eq!(manager.hook_count(), 2);
        assert_eq!(manager.hooks_of_kind(&ScriptKind::PreBuild).len(), 1);
        assert_eq!(manager.hooks_of_kind(&ScriptKind::PostBuild).len(), 1);
        manager.set_enabled(false);
        assert!(manager.hooks_of_kind(&ScriptKind::PreBuild).is_empty());
    }
    #[test]
    fn test_hook_manager_clear() {
        let mut manager = HookManager::new();
        manager.register(ScriptDef::pre_build("h1", "a"));
        manager.register(ScriptDef::pre_build("h2", "b"));
        manager.register(ScriptDef::post_build("h3", "c"));
        manager.remove_hooks_of_kind(&ScriptKind::PreBuild);
        assert_eq!(manager.hook_count(), 1);
        manager.clear();
        assert_eq!(manager.hook_count(), 0);
    }
    #[test]
    fn test_script_result() {
        let success = ScriptResult::success("test", Duration::from_millis(100));
        assert!(success.success);
        assert_eq!(success.exit_code, Some(0));
        let failure = ScriptResult::failure("test", 1, "error msg", Duration::from_millis(50));
        assert!(!failure.success);
        assert_eq!(failure.exit_code, Some(1));
    }
    #[test]
    fn test_script_variables() {
        let mut vars = ScriptVariables::new();
        vars.set("FOO", "bar");
        vars.set("NUM", "42");
        assert_eq!(vars.get("FOO"), Some("bar"));
        assert!(vars.contains("FOO"));
        assert!(!vars.contains("MISSING"));
        assert_eq!(vars.count(), 2);
    }
    #[test]
    fn test_script_variables_expand() {
        let mut vars = ScriptVariables::new();
        vars.set("NAME", "oxilean");
        vars.set("VERSION", "0.1.0");
        let expanded = vars.expand("Package: ${NAME} v${VERSION}");
        assert_eq!(expanded, "Package: oxilean v0.1.0");
    }
    #[test]
    fn test_script_variables_build_info() {
        let vars = ScriptVariables::with_build_info("my-pkg", "1.0.0", "release", "/tmp/target");
        assert_eq!(vars.get("OXILEAN_PKG_NAME"), Some("my-pkg"));
        assert_eq!(vars.get("OXILEAN_PROFILE"), Some("release"));
    }
    #[test]
    fn test_script_variables_merge() {
        let mut base = ScriptVariables::new();
        base.set("A", "1");
        base.set("B", "2");
        let mut overlay = ScriptVariables::new();
        overlay.set("B", "overridden");
        overlay.set("C", "3");
        base.merge(&overlay);
        assert_eq!(base.get("A"), Some("1"));
        assert_eq!(base.get("B"), Some("overridden"));
        assert_eq!(base.get("C"), Some("3"));
    }
    #[test]
    fn test_script_condition_always() {
        let vars = ScriptVariables::new();
        assert!(ScriptCondition::Always.evaluate(&vars, "debug"));
    }
    #[test]
    fn test_script_condition_var_set() {
        let mut vars = ScriptVariables::new();
        vars.set("FEATURE", "enabled");
        assert!(ScriptCondition::VarSet("FEATURE".to_string()).evaluate(&vars, "debug"));
        assert!(!ScriptCondition::VarSet("MISSING".to_string()).evaluate(&vars, "debug"));
    }
    #[test]
    fn test_script_condition_profile() {
        let vars = ScriptVariables::new();
        assert!(ScriptCondition::ProfileIs("release".to_string()).evaluate(&vars, "release"));
        assert!(!ScriptCondition::ProfileIs("release".to_string()).evaluate(&vars, "debug"));
    }
    #[test]
    fn test_script_condition_not() {
        let vars = ScriptVariables::new();
        let cond = ScriptCondition::Not(Box::new(ScriptCondition::ProfileIs("debug".to_string())));
        assert!(cond.evaluate(&vars, "release"));
        assert!(!cond.evaluate(&vars, "debug"));
    }
    #[test]
    fn test_script_condition_all_of() {
        let mut vars = ScriptVariables::new();
        vars.set("FOO", "bar");
        let cond = ScriptCondition::AllOf(vec![
            ScriptCondition::VarSet("FOO".to_string()),
            ScriptCondition::ProfileIs("debug".to_string()),
        ]);
        assert!(cond.evaluate(&vars, "debug"));
        assert!(!cond.evaluate(&vars, "release"));
    }
    #[test]
    fn test_script_condition_any_of() {
        let vars = ScriptVariables::new();
        let cond = ScriptCondition::AnyOf(vec![
            ScriptCondition::ProfileIs("release".to_string()),
            ScriptCondition::ProfileIs("debug".to_string()),
        ]);
        assert!(cond.evaluate(&vars, "debug"));
        assert!(cond.evaluate(&vars, "release"));
        assert!(!cond.evaluate(&vars, "bench"));
    }
    #[test]
    fn test_conditional_script() {
        let script = ScriptDef::pre_build("release-only", "echo release");
        let cond_script =
            ConditionalScript::new(script, ScriptCondition::ProfileIs("release".to_string()));
        let vars = ScriptVariables::new();
        assert!(cond_script.should_run(&vars, "release"));
        assert!(!cond_script.should_run(&vars, "debug"));
    }
    #[test]
    fn test_script_pipeline() {
        let mut pipeline = ScriptPipeline::new("build-pipeline");
        pipeline.add_script("step1", ScriptDef::pre_build("s1", "echo 1"));
        pipeline.add_script("step2", ScriptDef::pre_build("s2", "echo 2"));
        assert_eq!(pipeline.step_count(), 2);
    }
    #[test]
    fn test_script_pipeline_execute() {
        let runner = ScriptRunner::new();
        let mut pipeline = ScriptPipeline::new("test-pipeline");
        pipeline.add_script("a", ScriptDef::pre_build("a", "echo a"));
        pipeline.add_script("b", ScriptDef::post_build("b", "echo b"));
        let results = pipeline.execute(&runner);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
    }
    #[test]
    fn test_script_report() {
        let results = vec![
            ScriptResult::success("a", Duration::from_millis(100)),
            ScriptResult::success("b", Duration::from_millis(200)),
            ScriptResult::failure("c", 1, "err", Duration::from_millis(50)),
        ];
        let report = ScriptReport::from_results(results);
        assert_eq!(report.total, 3);
        assert_eq!(report.succeeded, 2);
        assert_eq!(report.failed, 1);
        assert!(!report.all_succeeded());
        assert_eq!(report.failed_names(), vec!["c"]);
    }
    #[test]
    fn test_script_runner_remove() {
        let mut runner = ScriptRunner::new();
        runner.add_script(ScriptDef::pre_build("removeme", "echo bye"));
        runner.add_script(ScriptDef::pre_build("keepme", "echo hi"));
        assert!(runner.remove_script("removeme"));
        assert!(!runner.remove_script("nonexistent"));
        assert_eq!(runner.script_count(), 1);
        assert!(runner.get_script("keepme").is_some());
        assert!(runner.get_script("removeme").is_none());
    }
    #[test]
    fn test_script_error_display() {
        let err = ScriptError::Timeout {
            name: "long-script".to_string(),
            timeout: Duration::from_secs(60),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("long-script"));
        assert!(msg.contains("timed out"));
    }
    #[test]
    fn test_generator_kind_equality() {
        assert_eq!(GeneratorKind::FfiBindings, GeneratorKind::FfiBindings);
        assert_ne!(GeneratorKind::FfiBindings, GeneratorKind::Serialization);
        assert_eq!(
            GeneratorKind::Custom("x".to_string()),
            GeneratorKind::Custom("x".to_string())
        );
    }
}
/// Runs a script with the given retry policy.
#[allow(dead_code)]
pub fn run_with_retry(
    runner: &ScriptRunner,
    script: &ScriptDef,
    policy: &RetryPolicy,
) -> ScriptResult {
    let mut attempts = 0u32;
    loop {
        let result = runner.run_single_public(script).unwrap_or_else(|_| {
            ScriptResult::failure(
                &script.name,
                1,
                "execution failed",
                std::time::Duration::ZERO,
            )
        });
        attempts += 1;
        if result.success || !policy.should_retry(attempts) {
            return result;
        }
        let _delay = policy.delay_for_attempt(attempts);
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_script_environment_minimal() {
        let env = ScriptEnvironment::minimal();
        assert_eq!(env.var_count(), 0);
    }
    #[test]
    fn test_script_environment_set_and_resolve() {
        let mut env = ScriptEnvironment::minimal();
        env.set("FOO", "bar");
        env.set("BAZ", "qux");
        let resolved = env.resolve();
        assert_eq!(resolved.get("FOO"), Some(&"bar".to_string()));
        assert_eq!(resolved.get("BAZ"), Some(&"qux".to_string()));
    }
    #[test]
    fn test_script_environment_unset() {
        let mut env = ScriptEnvironment::minimal();
        env.set("TO_REMOVE", "value");
        env.unset_var("TO_REMOVE");
        let resolved = env.resolve();
        assert!(!resolved.contains_key("TO_REMOVE"));
    }
    #[test]
    fn test_script_environment_path_prepend() {
        let env = ScriptEnvironment::minimal()
            .prepend_path("/usr/local/bin")
            .prepend_path("/opt/bin");
        let resolved = env.resolve();
        let path = resolved.get("PATH").cloned().unwrap_or_default();
        assert!(path.contains("/usr/local/bin"));
        assert!(path.contains("/opt/bin"));
    }
    #[test]
    fn test_script_profile_record() {
        let mut prof = ScriptProfile::new("my-script");
        prof.record(Duration::from_millis(100), false);
        prof.record(Duration::from_millis(200), true);
        assert_eq!(prof.run_count, 2);
        assert_eq!(prof.total_duration, Duration::from_millis(300));
        assert_eq!(prof.min_duration, Duration::from_millis(100));
        assert_eq!(prof.max_duration, Duration::from_millis(200));
        assert!(prof.last_was_cached);
    }
    #[test]
    fn test_script_profiler_hottest() {
        let mut profiler = ScriptProfiler::new();
        profiler.record("fast", Duration::from_millis(10), false);
        profiler.record("slow", Duration::from_millis(500), false);
        profiler.record("medium", Duration::from_millis(100), false);
        let hottest = profiler.hottest_scripts();
        assert_eq!(hottest[0].name, "slow");
    }
    #[test]
    fn test_target_triple_components() {
        let t = TargetTriple::new("aarch64-unknown-linux-gnu");
        assert_eq!(t.arch(), "aarch64");
        assert_eq!(t.vendor(), Some("unknown"));
        assert_eq!(t.os(), Some("linux"));
        assert_eq!(t.env(), Some("gnu"));
        assert!(!t.is_windows());
        assert!(!t.is_wasm());
    }
    #[test]
    fn test_target_triple_windows() {
        let t = TargetTriple::new("x86_64-pc-windows-msvc");
        assert!(t.is_windows());
    }
    #[test]
    fn test_target_triple_wasm() {
        let t = TargetTriple::new("wasm32-unknown-unknown");
        assert!(t.is_wasm());
    }
    #[test]
    fn test_cross_compile_script_env() {
        let base = ScriptDef::pre_build("cross-build", "make");
        let target = TargetTriple::new("aarch64-unknown-linux-gnu");
        let script = CrossCompileScript::new(base, target)
            .with_cc("aarch64-linux-gnu-gcc")
            .with_env("CFLAGS", "-O2");
        let env = script.build_env();
        assert_eq!(env.get("CC"), Some(&"aarch64-linux-gnu-gcc".to_string()));
        assert_eq!(env.get("CFLAGS"), Some(&"-O2".to_string()));
        assert_eq!(
            env.get("TARGET"),
            Some(&"aarch64-unknown-linux-gnu".to_string())
        );
    }
    #[test]
    fn test_artifact_kind_display() {
        assert_eq!(format!("{}", ArtifactKind::Binary), "binary");
        assert_eq!(format!("{}", ArtifactKind::StaticLib), "static-lib");
    }
    #[test]
    fn test_artifact_packager() {
        let mut packager = ArtifactPackager::new("/out", "release-bundle");
        packager.add_artifact(BuildArtifact::new(
            "app",
            "/build/app",
            "bin/app",
            ArtifactKind::Binary,
        ));
        packager.add_artifact(BuildArtifact::new(
            "libfoo.a",
            "/build/libfoo.a",
            "lib/libfoo.a",
            ArtifactKind::StaticLib,
        ));
        assert_eq!(packager.artifact_count(), 2);
        let manifest = packager.generate_manifest();
        assert!(manifest.contains("release-bundle"));
        assert!(manifest.contains("app"));
    }
    #[test]
    fn test_script_cache_store_and_get() {
        let mut cache = ScriptCache::new(10);
        let key = ScriptCacheKey::new("my-script", 0xdeadbeef);
        let result = ScriptResult::success("my-script", Duration::from_millis(50));
        cache.store(key.clone(), result);
        assert!(cache.is_valid(&key));
        assert_eq!(cache.entry_count(), 1);
    }
    #[test]
    fn test_script_cache_invalidate() {
        let mut cache = ScriptCache::new(10);
        let key = ScriptCacheKey::new("s", 1);
        let result = ScriptResult::success("s", Duration::ZERO);
        cache.store(key.clone(), result);
        assert!(cache.invalidate("s"));
        assert!(!cache.is_valid(&key));
    }
    #[test]
    fn test_script_dependency_tracker() {
        let mut tracker = ScriptDependencyTracker::new();
        tracker.register("codegen", "/proto/foo.proto");
        tracker.register("codegen", "/proto/bar.proto");
        tracker.register("compile", "/src/lib.rs");
        assert_eq!(tracker.script_count(), 2);
        assert_eq!(tracker.total_dep_count(), 3);
        let scripts = tracker.scripts_for_path(std::path::Path::new("/proto/foo.proto"));
        assert!(scripts.contains(&"codegen"));
    }
    #[test]
    fn test_protobuf_compile_options() {
        let opts = ProtobufCompileOptions::new("/out")
            .include("/proto")
            .proto("/proto/service.proto")
            .with_grpc();
        assert_eq!(opts.proto_count(), 1);
        assert!(opts.grpc);
        let script = opts.to_script_def("compile-proto");
        assert_eq!(script.kind, ScriptKind::CodeGen);
    }
    #[test]
    fn test_parallel_script_executor() {
        let runner = ScriptRunner::new();
        let mut executor = ParallelScriptExecutor::new(runner, 4);
        let results = executor.run_all(vec![
            ScriptDef::pre_build("s1", "echo 1"),
            ScriptDef::pre_build("s2", "echo 2"),
        ]);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
    }
    #[test]
    fn test_retry_policy_no_retry() {
        let policy = RetryPolicy::no_retry();
        assert!(!policy.should_retry(1));
    }
    #[test]
    fn test_retry_policy_fixed() {
        let policy = RetryPolicy::fixed(3, Duration::from_millis(100));
        assert!(policy.should_retry(0));
        assert!(policy.should_retry(2));
        assert!(!policy.should_retry(3));
        assert_eq!(policy.delay_for_attempt(0), Duration::from_millis(100));
    }
    #[test]
    fn test_retry_policy_exponential() {
        let policy = RetryPolicy::exponential(5, Duration::from_millis(10));
        let d0 = policy.delay_for_attempt(0);
        let d1 = policy.delay_for_attempt(1);
        assert!(d1 >= d0);
    }
    #[test]
    fn test_run_with_retry_success() {
        let runner = ScriptRunner::new();
        let script = ScriptDef::pre_build("ok", "echo ok");
        let policy = RetryPolicy::fixed(3, Duration::ZERO);
        let result = run_with_retry(&runner, &script, &policy);
        assert!(result.success);
    }
    #[test]
    fn test_error_handler_abort() {
        let mut handler = ScriptErrorHandler::new(ErrorHandlingStrategy::AbortBuild);
        let result = ScriptResult::failure("bad", 1, "oops", Duration::ZERO);
        let should_abort = handler.handle(&result);
        assert!(should_abort);
        assert_eq!(handler.error_count(), 1);
    }
    #[test]
    fn test_error_handler_skip() {
        let mut handler = ScriptErrorHandler::new(ErrorHandlingStrategy::Skip);
        let result = ScriptResult::failure("bad", 1, "oops", Duration::ZERO);
        let should_abort = handler.handle(&result);
        assert!(!should_abort);
        assert!(handler.has_errors());
    }
    #[test]
    fn test_error_handler_warn_and_continue() {
        let mut handler = ScriptErrorHandler::new(ErrorHandlingStrategy::WarnAndContinue);
        let result = ScriptResult::failure("warn-me", 2, "warning error", Duration::ZERO);
        assert!(!handler.handle(&result));
        handler.clear();
        assert_eq!(handler.error_count(), 0);
    }
}
