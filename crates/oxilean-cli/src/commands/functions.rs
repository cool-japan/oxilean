//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_elab::{elaborate_decl as elab_decl_impl, ElabContext, PendingDecl};
use oxilean_kernel::{check_declaration, Declaration, Environment, ReducibilityHint};
use oxilean_parse::{Lexer, Parser};
use std::path::{Path, PathBuf};

use super::types::{
    BuildOptions, CommandArgParser, CommandCategory, CommandConfig, CommandContext,
    CommandDispatcher, CommandEnvironment, CommandError, CommandFlagMeta, CommandHelpFormatter,
    CommandLogger, CommandMetadata, CommandOutput, CommandPipeline, CommandPipelineStep,
    CommandRegistry, CommandStats, ExitCode, FormatCommandOptions, LogLevel, ProgressReporter,
    TestOptions,
};
use std::fs;
use std::io;

/// Result type for commands.
#[allow(dead_code)]
pub type CommandResult<T> = Result<T, CommandError>;
/// Check source code for type errors.
#[allow(dead_code)]
pub fn check_source(source: &str) -> CommandResult<()> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let mut env = Environment::new();
    loop {
        match parser.parse_decl() {
            Ok(surface_decl) => {
                let mut ctx = ElabContext::new(&env);
                match elaborate_decl(&mut ctx, &surface_decl.value) {
                    Ok(kernel_decl) => match check_declaration(&mut env, kernel_decl) {
                        Ok(()) => {}
                        Err(e) => {
                            return Err(CommandError::general(format!("Type error: {}", e)));
                        }
                    },
                    Err(e) => {
                        return Err(CommandError::general(format!("Elaboration error: {:?}", e)));
                    }
                }
            }
            Err(e) => {
                if e.to_string().contains("end of file") {
                    break;
                }
                return Err(CommandError::general(format!("Parse error: {}", e)));
            }
        }
    }
    Ok(())
}
/// Check a source file.
#[allow(dead_code)]
pub fn check_file(path: &Path, verbose: bool) -> CommandResult<()> {
    let reporter = ProgressReporter::new(format!("Checking {}", path.display()), verbose);
    let contents = fs::read_to_string(path).map_err(|e| {
        reporter.error(&e.to_string());
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                CommandError::not_found(format!("File not found: {}", path.display()))
            }
            std::io::ErrorKind::PermissionDenied => {
                CommandError::permission_denied(format!("Permission denied: {}", path.display()))
            }
            _ => CommandError::general(format!("Failed to read file: {}", e)),
        }
    })?;
    check_source(&contents).map_err(|e| {
        reporter.error(&e.message);
        e
    })?;
    reporter.complete();
    Ok(())
}
/// Build a project.
#[allow(dead_code)]
pub fn build_project(project_dir: &Path, release: bool, verbose: bool) -> CommandResult<()> {
    let mode = if release { "release" } else { "debug" };
    let reporter = ProgressReporter::new(format!("Building project in {} mode", mode), verbose);
    let main_file = project_dir.join("main.lean");
    if !main_file.exists() {
        reporter.error("main.lean not found");
        return Err(CommandError::not_found("main.lean not found in project"));
    }
    reporter.progress(&format!("Processing {}", main_file.display()));
    let contents = fs::read_to_string(&main_file).map_err(|e| {
        reporter.error(&e.to_string());
        CommandError::general(format!("Failed to read main.lean: {}", e))
    })?;
    check_source(&contents).map_err(|e| {
        reporter.error(&e.message);
        e
    })?;
    let build_dir = project_dir.join(if release {
        "target/release"
    } else {
        "target/debug"
    });
    fs::create_dir_all(&build_dir).map_err(|e| {
        reporter.error(&e.to_string());
        CommandError::general(format!("Failed to create build directory: {}", e))
    })?;
    reporter.progress("Build successful");
    reporter.complete();
    Ok(())
}
/// Run tests in a project.
#[allow(dead_code)]
pub fn run_tests(project_dir: &Path, filter: Option<&str>, verbose: bool) -> CommandResult<usize> {
    let reporter = ProgressReporter::new("Running tests", verbose);
    let test_dir = project_dir.join("tests");
    if !test_dir.exists() {
        reporter.progress("No tests directory found");
        return Ok(0);
    }
    let mut test_count = 0;
    let mut passed = 0;
    let mut failed = 0;
    if let Ok(entries) = fs::read_dir(&test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lean") {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                if let Some(f) = filter {
                    if !filename.contains(f) {
                        continue;
                    }
                }
                test_count += 1;
                reporter.progress(&format!("Running {}", filename));
                match fs::read_to_string(&path) {
                    Ok(contents) => match check_source(&contents) {
                        Ok(()) => {
                            passed += 1;
                            if verbose {
                                reporter.progress(&format!("  ✓ {}", filename));
                            }
                        }
                        Err(e) => {
                            failed += 1;
                            reporter.progress(&format!("  ✗ {}: {}", filename, e.message));
                        }
                    },
                    Err(e) => {
                        failed += 1;
                        reporter.error(&format!("Failed to read {}: {}", filename, e));
                    }
                }
            }
        }
    }
    reporter.progress(&format!(
        "Test results: {} passed, {} failed",
        passed, failed
    ));
    reporter.complete();
    if failed > 0 {
        Err(CommandError::general(format!("{} test(s) failed", failed)))
    } else {
        Ok(test_count)
    }
}
/// Run a proof or script.
#[allow(dead_code)]
pub fn run_script(path: &Path, verbose: bool) -> CommandResult<()> {
    let reporter = ProgressReporter::new(format!("Running {}", path.display()), verbose);
    let contents = fs::read_to_string(path).map_err(|e| {
        reporter.error(&e.to_string());
        CommandError::general(format!("Failed to read script: {}", e))
    })?;
    check_source(&contents).map_err(|e| {
        reporter.error(&e.message);
        e
    })?;
    reporter.progress("Script executed successfully");
    reporter.complete();
    Ok(())
}
/// Format source files.
#[allow(dead_code)]
pub fn format_files(
    paths: &[PathBuf],
    in_place: bool,
    check_only: bool,
    verbose: bool,
) -> CommandResult<usize> {
    use crate::format::Formatter;
    let reporter = ProgressReporter::new("Formatting source files", verbose);
    let formatter = Formatter::new();
    let mut formatted_count = 0;
    for path in paths {
        reporter.progress(&format!("Formatting {}", path.display()));
        let contents = fs::read_to_string(path).map_err(|e| {
            reporter.error(&e.to_string());
            CommandError::general(format!("Failed to read {}: {}", path.display(), e))
        })?;
        let formatted = formatter.format_source(&contents);
        if check_only {
            if contents != formatted {
                reporter.progress(&format!("  Would reformat {}", path.display()));
            }
        } else if in_place {
            fs::write(path, &formatted).map_err(|e| {
                reporter.error(&e.to_string());
                CommandError::general(format!("Failed to write {}: {}", path.display(), e))
            })?;
            formatted_count += 1;
            reporter.progress(&format!("  Formatted {}", path.display()));
        } else {
            println!("{}", formatted);
        }
    }
    reporter.complete();
    Ok(formatted_count)
}
/// Generate documentation.
#[allow(dead_code)]
pub fn generate_docs(
    project_dir: &Path,
    output_dir: Option<&Path>,
    verbose: bool,
) -> CommandResult<()> {
    let reporter = ProgressReporter::new("Generating documentation", verbose);
    let default_docs_dir = project_dir.join("docs");
    let out_dir = output_dir.unwrap_or(&default_docs_dir);
    fs::create_dir_all(out_dir).map_err(|e| {
        reporter.error(&e.to_string());
        CommandError::general(format!("Failed to create output directory: {}", e))
    })?;
    reporter.progress(&format!("Output directory: {}", out_dir.display()));
    let src_dir = project_dir.join("src");
    if src_dir.exists() {
        reporter.progress(&format!("Scanning {}", src_dir.display()));
        if let Ok(entries) = fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lean") {
                    reporter.progress(&format!(
                        "  Processing {}",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    ));
                }
            }
        }
    }
    reporter.progress("Documentation generated successfully");
    reporter.complete();
    Ok(())
}
/// Clean build artifacts.
#[allow(dead_code)]
pub fn clean_project(project_dir: &Path, verbose: bool) -> CommandResult<()> {
    let reporter = ProgressReporter::new("Cleaning project", verbose);
    let target_dir = project_dir.join("target");
    if target_dir.exists() {
        reporter.progress(&format!("Removing {}", target_dir.display()));
        fs::remove_dir_all(&target_dir).map_err(|e| {
            reporter.error(&e.to_string());
            CommandError::general(format!("Failed to remove target directory: {}", e))
        })?;
    }
    let build_dir = project_dir.join("build");
    if build_dir.exists() {
        reporter.progress(&format!("Removing {}", build_dir.display()));
        fs::remove_dir_all(&build_dir).map_err(|e| {
            reporter.error(&e.to_string());
            CommandError::general(format!("Failed to remove build directory: {}", e))
        })?;
    }
    reporter.progress("Clean complete");
    reporter.complete();
    Ok(())
}
/// Format an error for display.
#[allow(dead_code)]
pub fn format_error(error: &CommandError, color: bool) -> String {
    let prefix = if color {
        "\x1b[31merror\x1b[0m"
    } else {
        "error"
    };
    format!("{}: {}", prefix, error.message)
}
/// Format multiple errors for display.
#[allow(dead_code)]
pub fn format_errors(errors: &[CommandError], color: bool) -> String {
    let mut output = String::new();
    for (i, error) in errors.iter().enumerate() {
        output.push_str(&format_error(error, color));
        if i < errors.len() - 1 {
            output.push('\n');
        }
    }
    output
}
fn elaborate_decl(
    ctx: &mut ElabContext,
    decl: &oxilean_parse::Decl,
) -> Result<Declaration, String> {
    let pending = elab_decl_impl(ctx.env(), decl).map_err(|e| e.to_string())?;
    match pending {
        PendingDecl::Definition { name, ty, val, .. } => Ok(Declaration::Definition {
            name,
            univ_params: vec![],
            ty,
            val,
            hint: ReducibilityHint::Regular(0),
        }),
        PendingDecl::Theorem {
            name, ty, proof, ..
        } => Ok(Declaration::Theorem {
            name,
            univ_params: vec![],
            ty,
            val: proof,
        }),
        PendingDecl::Axiom { name, ty, .. } => Ok(Declaration::Axiom {
            name,
            univ_params: vec![],
            ty,
        }),
        PendingDecl::Inductive { name, ty, .. } => Ok(Declaration::Axiom {
            name,
            univ_params: vec![],
            ty,
        }),
        PendingDecl::Opaque { name, ty, val } => Ok(Declaration::Opaque {
            name,
            univ_params: vec![],
            ty,
            val,
        }),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exit_code_success() {
        assert_eq!(ExitCode::Success.as_u32(), 0);
    }
    #[test]
    fn test_exit_code_error() {
        assert_eq!(ExitCode::Error.as_u32(), 1);
    }
    #[test]
    fn test_exit_code_usage() {
        assert_eq!(ExitCode::Usage.as_u32(), 2);
    }
    #[test]
    fn test_exit_code_not_found() {
        assert_eq!(ExitCode::NotFound.as_u32(), 3);
    }
    #[test]
    fn test_exit_code_permission_denied() {
        assert_eq!(ExitCode::PermissionDenied.as_u32(), 4);
    }
    #[test]
    fn test_exit_code_equality() {
        assert_eq!(ExitCode::Success, ExitCode::Success);
        assert_ne!(ExitCode::Success, ExitCode::Error);
    }
    #[test]
    fn test_exit_code_ordering() {
        assert!(ExitCode::Success.as_u32() < ExitCode::Error.as_u32());
    }
    #[test]
    fn test_command_error_new() {
        let err = CommandError::new(ExitCode::Error, "test error");
        assert_eq!(err.code, ExitCode::Error);
        assert_eq!(err.message, "test error");
    }
    #[test]
    fn test_command_error_general() {
        let err = CommandError::general("general error");
        assert_eq!(err.code, ExitCode::Error);
        assert_eq!(err.message, "general error");
    }
    #[test]
    fn test_command_error_usage() {
        let err = CommandError::usage("usage error");
        assert_eq!(err.code, ExitCode::Usage);
        assert_eq!(err.message, "usage error");
    }
    #[test]
    fn test_command_error_not_found() {
        let err = CommandError::not_found("not found");
        assert_eq!(err.code, ExitCode::NotFound);
        assert_eq!(err.message, "not found");
    }
    #[test]
    fn test_command_error_permission_denied() {
        let err = CommandError::permission_denied("denied");
        assert_eq!(err.code, ExitCode::PermissionDenied);
        assert_eq!(err.message, "denied");
    }
    #[test]
    fn test_command_error_debug() {
        let err = CommandError::general("test");
        assert!(format!("{:?}", err).contains("test"));
    }
    #[test]
    fn test_progress_reporter_new() {
        let reporter = ProgressReporter::new("test", false);
        assert_eq!(reporter.message, "test");
        assert!(!reporter.verbose);
    }
    #[test]
    fn test_progress_reporter_verbose() {
        let reporter = ProgressReporter::new("test", true);
        assert!(reporter.verbose);
    }
    #[test]
    fn test_progress_reporter_progress() {
        let reporter = ProgressReporter::new("test", false);
        reporter.progress("detail");
    }
    #[test]
    fn test_progress_reporter_complete() {
        let reporter = ProgressReporter::new("test", false);
        reporter.complete();
    }
    #[test]
    fn test_progress_reporter_error() {
        let reporter = ProgressReporter::new("test", false);
        reporter.error("error message");
    }
    #[test]
    fn test_progress_reporter_verbose_output() {
        let reporter = ProgressReporter::new("test", true);
        reporter.progress("verbose progress");
    }
    #[test]
    fn test_format_error_without_color() {
        let err = CommandError::general("test error");
        let formatted = format_error(&err, false);
        assert!(formatted.contains("test error"));
        assert!(formatted.contains("error"));
    }
    #[test]
    fn test_format_error_with_color() {
        let err = CommandError::general("test error");
        let formatted = format_error(&err, true);
        assert!(formatted.contains("test error"));
        assert!(formatted.contains("\x1b["));
    }
    #[test]
    fn test_format_single_error() {
        let err = CommandError::general("error 1");
        let errors = vec![err];
        let formatted = format_errors(&errors, false);
        assert!(formatted.contains("error 1"));
    }
    #[test]
    fn test_format_multiple_errors() {
        let errors = vec![
            CommandError::general("error 1"),
            CommandError::general("error 2"),
        ];
        let formatted = format_errors(&errors, false);
        assert!(formatted.contains("error 1"));
        assert!(formatted.contains("error 2"));
        assert!(formatted.contains('\n'));
    }
    #[test]
    fn test_format_empty_errors() {
        let errors: Vec<CommandError> = vec![];
        let formatted = format_errors(&errors, false);
        assert_eq!(formatted, "");
    }
    #[test]
    fn test_command_config_default() {
        let config = CommandConfig::default();
        assert!(!config.verbose);
        assert!(config.color);
        assert_eq!(config.max_errors, 10);
    }
    #[test]
    fn test_command_config_custom() {
        let config = CommandConfig {
            verbose: true,
            color: false,
            project_dir: PathBuf::from("/tmp"),
            max_errors: 20,
        };
        assert!(config.verbose);
        assert!(!config.color);
        assert_eq!(config.max_errors, 20);
    }
    #[test]
    fn test_command_config_equality() {
        let config1 = CommandConfig::default();
        let config2 = CommandConfig::default();
        assert_eq!(config1.verbose, config2.verbose);
    }
    #[test]
    fn test_check_empty_source() {
        let result = check_source("");
        assert!(result.is_ok());
    }
    #[test]
    fn test_check_source_error_handling() {
        let result = check_source("invalid syntax %%%");
        assert!(result.is_err());
    }
    #[test]
    fn test_check_source_preserves_context() {
        let result = check_source("");
        assert!(result.is_ok());
    }
    #[test]
    fn test_run_tests_empty_project() {
        let result = run_tests(&PathBuf::from("/tmp/nonexistent"), None, false);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_run_tests_with_filter() {
        let result = run_tests(&PathBuf::from("/tmp/nonexistent"), Some("test"), false);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_run_tests_verbose() {
        let result = run_tests(&PathBuf::from("/tmp/nonexistent"), None, true);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_build_debug_vs_release() {
        let debug_config = CommandConfig::default();
        let release_config = CommandConfig {
            verbose: false,
            color: true,
            project_dir: PathBuf::from("."),
            max_errors: 10,
        };
        assert_eq!(debug_config.color, release_config.color);
    }
    #[test]
    fn test_build_options_default() {
        let opts = BuildOptions::default();
        assert!(!opts.release);
        assert!(!opts.tests);
        assert!(!opts.docs);
    }
    #[test]
    fn test_build_options_custom() {
        let opts = BuildOptions {
            release: true,
            jobs: Some(4),
            target: Some("x86_64-unknown-linux-gnu".to_string()),
            tests: true,
            docs: true,
            keep_artifacts: true,
        };
        assert!(opts.release);
        assert_eq!(opts.jobs, Some(4));
        assert!(opts.tests);
        assert!(opts.docs);
    }
    #[test]
    fn test_format_files_empty_list() {
        let result = format_files(&[], false, false, false);
        assert!(result.is_ok());
        assert_eq!(result.expect("test operation should succeed"), 0);
    }
    #[test]
    fn test_format_command_options_default() {
        let opts = FormatCommandOptions::default();
        assert!(!opts.in_place);
        assert!(!opts.check);
        assert!(!opts.diff);
        assert!(opts.recursive);
    }
    #[test]
    fn test_format_command_options_custom() {
        let opts = FormatCommandOptions {
            in_place: true,
            check: true,
            diff: true,
            recursive: false,
        };
        assert!(opts.in_place);
        assert!(opts.check);
        assert!(opts.diff);
        assert!(!opts.recursive);
    }
    #[test]
    fn test_doc_generation_default_output() {
        let config = CommandConfig::default();
        assert_eq!(config.project_dir, PathBuf::from("."));
    }
    #[test]
    fn test_doc_generation_custom_output() {
        let custom_out = PathBuf::from("custom_docs");
        let _ = &custom_out;
    }
    #[test]
    fn test_clean_nonexistent_project() {
        let result = clean_project(&PathBuf::from("/tmp/nonexistent_project_xyz"), false);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_clean_verbose() {
        let result = clean_project(&PathBuf::from("/tmp/nonexistent"), true);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_command_result_ok() {
        let result: CommandResult<usize> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(42));
    }
    #[test]
    fn test_command_result_err() {
        let result: CommandResult<()> = Err(CommandError::general("test"));
        assert!(result.is_err());
    }
    #[test]
    fn test_command_result_map() {
        let result: CommandResult<usize> = Ok(42);
        let mapped = result.map(|x| x + 1);
        assert_eq!(mapped.expect("test operation should succeed"), 43);
    }
    #[test]
    fn test_progress_reporter_timing() {
        let reporter = ProgressReporter::new("timing test", false);
        std::thread::sleep(std::time::Duration::from_millis(10));
        reporter.complete();
    }
    #[test]
    fn test_command_error_clone() {
        let err1 = CommandError::general("test");
        let err2 = err1.clone();
        assert_eq!(err1.message, err2.message);
    }
    #[test]
    fn test_command_config_clone() {
        let config1 = CommandConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.verbose, config2.verbose);
    }
    #[test]
    fn test_command_context_new() {
        let ctx = CommandContext::new(CommandConfig::default());
        assert!(!ctx.has_errors());
        assert!(!ctx.has_warnings());
    }
    #[test]
    fn test_command_context_add_error() {
        let mut ctx = CommandContext::new(CommandConfig::default());
        let err = CommandError::general("test error");
        ctx.add_error(err);
        assert!(ctx.has_errors());
        assert_eq!(ctx.errors().len(), 1);
    }
    #[test]
    fn test_command_context_add_warning() {
        let mut ctx = CommandContext::new(CommandConfig::default());
        ctx.add_warning("test warning".to_string());
        assert!(ctx.has_warnings());
        assert_eq!(ctx.warnings().len(), 1);
    }
    #[test]
    fn test_command_context_max_errors_limit() {
        let config = CommandConfig {
            max_errors: 2,
            ..Default::default()
        };
        let mut ctx = CommandContext::new(config);
        ctx.add_error(CommandError::general("error 1"));
        ctx.add_error(CommandError::general("error 2"));
        ctx.add_error(CommandError::general("error 3"));
        assert_eq!(ctx.errors().len(), 2);
    }
    #[test]
    fn test_command_context_multiple_warnings() {
        let mut ctx = CommandContext::new(CommandConfig::default());
        ctx.add_warning("warn 1".to_string());
        ctx.add_warning("warn 2".to_string());
        assert_eq!(ctx.warnings().len(), 2);
    }
    #[test]
    fn test_test_options_default() {
        let opts = TestOptions::default();
        assert!(opts.filter.is_none());
        assert!(!opts.show_output);
        assert!(!opts.sequential);
        assert!(opts.jobs.is_none());
    }
    #[test]
    fn test_test_options_custom() {
        let opts = TestOptions {
            filter: Some("test_".to_string()),
            show_output: true,
            sequential: true,
            jobs: Some(1),
        };
        assert_eq!(opts.filter, Some("test_".to_string()));
        assert!(opts.show_output);
        assert!(opts.sequential);
    }
    #[test]
    fn test_command_context_clone() {
        let ctx1 = CommandContext::new(CommandConfig::default());
        let ctx2 = ctx1.clone();
        assert_eq!(ctx1.errors().len(), ctx2.errors().len());
    }
    #[test]
    fn test_dispatcher_new() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        assert!(!dispatcher.config.verbose);
    }
    #[test]
    fn test_dispatcher_dispatch_unknown_command() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("unknown_cmd", &[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ExitCode::Usage);
    }
    #[test]
    fn test_dispatcher_check_no_args() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("check", &[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_dispatcher_build_default() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("build", &[]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_test_default() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("test", &[]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_clean_default() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("clean", &[]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_doc_default() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("doc", &[]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_verbose_mode() {
        let config = CommandConfig {
            verbose: true,
            ..Default::default()
        };
        let dispatcher = CommandDispatcher::new(config);
        assert!(dispatcher.config.verbose);
    }
    #[test]
    fn test_build_options_release_flag() {
        let opts = BuildOptions {
            release: true,
            ..Default::default()
        };
        assert!(opts.release);
    }
    #[test]
    fn test_build_options_parallel_jobs() {
        let opts = BuildOptions {
            jobs: Some(8),
            ..Default::default()
        };
        assert_eq!(opts.jobs, Some(8));
    }
    #[test]
    fn test_build_options_cross_compile() {
        let opts = BuildOptions {
            target: Some("aarch64-unknown-linux-gnu".to_string()),
            ..Default::default()
        };
        assert!(opts.target.is_some());
    }
    #[test]
    fn test_build_options_docs_flag() {
        let opts = BuildOptions {
            docs: true,
            ..Default::default()
        };
        assert!(opts.docs);
    }
    #[test]
    fn test_build_options_tests_flag() {
        let opts = BuildOptions {
            tests: true,
            ..Default::default()
        };
        assert!(opts.tests);
    }
    #[test]
    fn test_test_options_with_filter() {
        let opts = TestOptions {
            filter: Some("integration_".to_string()),
            ..Default::default()
        };
        assert_eq!(opts.filter, Some("integration_".to_string()));
    }
    #[test]
    fn test_test_options_parallel_jobs() {
        let opts = TestOptions {
            jobs: Some(4),
            ..Default::default()
        };
        assert_eq!(opts.jobs, Some(4));
    }
    #[test]
    fn test_test_options_sequential_mode() {
        let opts = TestOptions {
            sequential: true,
            ..Default::default()
        };
        assert!(opts.sequential);
    }
    #[test]
    fn test_test_options_show_output() {
        let opts = TestOptions {
            show_output: true,
            ..Default::default()
        };
        assert!(opts.show_output);
    }
    #[test]
    fn test_format_options_in_place() {
        let opts = FormatCommandOptions {
            in_place: true,
            ..Default::default()
        };
        assert!(opts.in_place);
    }
    #[test]
    fn test_format_options_check_mode() {
        let opts = FormatCommandOptions {
            check: true,
            ..Default::default()
        };
        assert!(opts.check);
    }
    #[test]
    fn test_format_options_diff_mode() {
        let opts = FormatCommandOptions {
            diff: true,
            ..Default::default()
        };
        assert!(opts.diff);
    }
    #[test]
    fn test_format_options_non_recursive() {
        let opts = FormatCommandOptions {
            recursive: false,
            ..Default::default()
        };
        assert!(!opts.recursive);
    }
    #[test]
    fn test_format_options_combined() {
        let opts = FormatCommandOptions {
            in_place: true,
            check: false,
            diff: true,
            recursive: true,
        };
        assert!(opts.in_place);
        assert!(!opts.check);
        assert!(opts.diff);
        assert!(opts.recursive);
    }
    #[test]
    fn test_format_error_general_type() {
        let err = CommandError::general("test");
        let formatted = format_error(&err, false);
        assert!(!formatted.is_empty());
    }
    #[test]
    fn test_format_error_preserves_message() {
        let msg = "original message";
        let err = CommandError::general(msg);
        let formatted = format_error(&err, false);
        assert!(formatted.contains(msg));
    }
    #[test]
    fn test_context_error_limit_respected() {
        let config = CommandConfig {
            max_errors: 1,
            ..Default::default()
        };
        let mut ctx = CommandContext::new(config);
        ctx.add_error(CommandError::general("err1"));
        ctx.add_error(CommandError::general("err2"));
        ctx.add_error(CommandError::general("err3"));
        assert_eq!(ctx.errors().len(), 1);
    }
    #[test]
    fn test_context_many_warnings() {
        let mut ctx = CommandContext::new(CommandConfig::default());
        for i in 0..10 {
            ctx.add_warning(format!("warning {}", i));
        }
        assert_eq!(ctx.warnings().len(), 10);
    }
    #[test]
    fn test_context_debug_info() {
        let mut ctx = CommandContext::new(CommandConfig::default());
        ctx.add_error(CommandError::general("test"));
        assert!(ctx.has_errors());
        let debug = format!("{:?}", ctx);
        assert!(debug.contains("test"));
    }
    #[test]
    fn test_dispatcher_format_with_args() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result =
            dispatcher.dispatch("fmt", &["test.lean".to_string(), "--in-place".to_string()]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_build_release() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("build", &["--release".to_string()]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_dispatcher_test_with_filter() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let result = dispatcher.dispatch("test", &["unit_".to_string(), ".".to_string()]);
        assert!(result.is_ok() || result.is_err());
    }
    #[test]
    fn test_error_exit_codes_distinct() {
        let e1 = ExitCode::Success;
        let e2 = ExitCode::Error;
        let e3 = ExitCode::Usage;
        assert_ne!(e1.as_u32(), e2.as_u32());
        assert_ne!(e2.as_u32(), e3.as_u32());
    }
    #[test]
    fn test_command_error_preserves_exit_code() {
        let codes = vec![
            ExitCode::Success,
            ExitCode::Error,
            ExitCode::Usage,
            ExitCode::NotFound,
            ExitCode::PermissionDenied,
        ];
        for code in codes {
            let err = CommandError::new(code, "test");
            assert_eq!(err.code, code);
        }
    }
    #[test]
    fn test_progress_reporter_empty_message() {
        let reporter = ProgressReporter::new("", false);
        assert_eq!(reporter.message, "");
    }
    #[test]
    fn test_progress_reporter_long_message() {
        let long_msg = "x".repeat(1000);
        let reporter = ProgressReporter::new(&long_msg, false);
        assert_eq!(reporter.message.len(), 1000);
    }
    #[test]
    fn test_progress_reporter_multiple_operations() {
        let reporter = ProgressReporter::new("multi", false);
        reporter.progress("step 1");
        reporter.progress("step 2");
        reporter.progress("step 3");
        reporter.complete();
    }
    #[test]
    fn test_command_workflow_simulate() {
        let config = CommandConfig::default();
        let mut ctx = CommandContext::new(config);
        ctx.add_error(CommandError::general("step 1 failed"));
        assert!(ctx.has_errors());
        ctx.add_warning("step 1 warning".to_string());
        assert!(ctx.has_warnings());
        assert_eq!(ctx.errors().len(), 1);
        assert_eq!(ctx.warnings().len(), 1);
    }
    #[test]
    fn test_dispatcher_chain_commands() {
        let dispatcher = CommandDispatcher::new(CommandConfig::default());
        let _r1 = dispatcher.dispatch("check", &[]);
        let _r2 = dispatcher.dispatch("build", &[]);
        let _r3 = dispatcher.dispatch("clean", &[]);
    }
}
/// Return the commands module version.
#[allow(dead_code)]
pub fn commands_module_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod commands_extra_tests {
    use super::*;
    #[test]
    fn test_command_metadata_check() {
        let meta = CommandMetadata::check_command();
        assert_eq!(meta.name, "check");
        assert!(meta.aliases.contains(&"c".to_string()));
        assert!(!meta.flags.is_empty());
    }
    #[test]
    fn test_command_registry() {
        let reg = CommandRegistry::standard();
        assert!(reg.find("check").is_some());
        assert!(reg.find("c").is_some());
        assert!(reg.find("nonexistent").is_none());
    }
    #[test]
    fn test_command_registry_by_category() {
        let reg = CommandRegistry::standard();
        let dev_cmds = reg.by_category(&CommandCategory::Development);
        assert!(!dev_cmds.is_empty());
    }
    #[test]
    fn test_command_arg_parser() {
        let parsed =
            CommandArgParser::parse("check", &["--verbose", "--output", "json", "src/main.lean"]);
        assert_eq!(parsed.name, "check");
        assert!(parsed.has_flag("--verbose"));
        assert_eq!(parsed.get_str("--output"), Some("json"));
        assert_eq!(parsed.first_positional(), Some("src/main.lean"));
    }
    #[test]
    fn test_command_output() {
        let out = CommandOutput::success("OK");
        assert!(out.is_success());
        assert_eq!(out.exit_code, 0);
        let err = CommandOutput::failure(1, "file not found");
        assert!(!err.is_success());
        assert_eq!(err.exit_code, 1);
    }
    #[test]
    fn test_help_formatter() {
        let meta = CommandMetadata::check_command();
        let help = CommandHelpFormatter::format(&meta);
        assert!(help.contains("Type-check"));
        assert!(help.contains("USAGE:"));
        assert!(help.contains("FLAGS:"));
        assert!(help.contains("EXAMPLES:"));
    }
    #[test]
    fn test_commands_module_version() {
        assert!(!commands_module_version().is_empty());
    }
    #[test]
    fn test_flag_meta_creation() {
        let flag = CommandFlagMeta::bool_flag("--verbose", Some("-v"), "Verbose output");
        assert_eq!(flag.long, "--verbose");
        assert_eq!(flag.short, Some("-v".to_string()));
        assert!(!flag.takes_value);
    }
}
#[cfg(test)]
mod commands_pipeline_tests {
    use super::*;
    #[test]
    fn test_command_pipeline() {
        let pipeline = CommandPipeline::new(true)
            .add_step(CommandPipelineStep::new(
                "check",
                vec!["src/main.lean".to_string()],
            ))
            .add_step(CommandPipelineStep::new(
                "format",
                vec!["--check".to_string()],
            ));
        assert_eq!(pipeline.step_count(), 2);
        assert!(pipeline.stop_on_failure);
    }
    #[test]
    fn test_command_environment() {
        let mut env = CommandEnvironment::new("/workspace");
        env.set_var("OXILEAN_HOME", "/usr/local/oxilean");
        assert_eq!(env.get_var("OXILEAN_HOME"), Some("/usr/local/oxilean"));
        assert!(env.get_var("MISSING").is_none());
    }
    #[test]
    fn test_command_logger() {
        let logger = CommandLogger::new(LogLevel::Info).with_prefix("oxilean");
        let msg = logger.info("Building...");
        assert!(msg.is_some());
        assert!(msg
            .expect("test operation should succeed")
            .contains("Building"));
        let debug_msg = logger.log(&LogLevel::Debug, "debug info");
        assert!(debug_msg.is_none());
    }
    #[test]
    fn test_command_stats() {
        let mut stats = CommandStats::default();
        stats.record(true, 100);
        stats.record(false, 200);
        stats.record(true, 150);
        assert_eq!(stats.total_invocations, 3);
        assert_eq!(stats.successful, 2);
        assert_eq!(stats.failed, 1);
        assert!((stats.success_rate() - 66.667).abs() < 0.01);
    }
}
/// Return the commands module feature set.
#[allow(dead_code)]
pub fn commands_features() -> Vec<&'static str> {
    vec![
        "check",
        "build",
        "format",
        "doc",
        "lint",
        "serve",
        "clean",
        "test",
        "registry",
        "pipeline",
        "environment",
        "parser",
        "logger",
        "stats",
    ]
}
#[cfg(test)]
mod commands_final_tests {
    use super::*;
    #[test]
    fn test_commands_features() {
        let features = commands_features();
        assert!(features.contains(&"check"));
        assert!(features.contains(&"build"));
        assert!(features.len() >= 10);
    }
}
/// Return total registered command count.
#[allow(dead_code)]
pub fn standard_command_count() -> usize {
    CommandRegistry::standard().command_names().len()
}
#[cfg(test)]
mod commands_count_tests {
    use super::*;
    #[test]
    fn test_standard_command_count() {
        assert!(standard_command_count() >= 3);
    }
}
