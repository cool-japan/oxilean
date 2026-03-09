//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::io::Write;

use super::completiongenerator_type::CompletionGenerator;
use super::richcompletiongenerator_type::RichCompletionGenerator;
use super::types::{
    AppCompletionSpec, CompletionCache, CompletionCandidate, CompletionConfig, CompletionContext,
    CompletionEngine, CompletionHistory, CompletionInstallTarget, CompletionItemKind,
    CompletionOutputFormat, CompletionSpec, CompletionStats, DynamicCompletionRegistry,
    FileSystemCompletionProvider, LspCompletionItem, ShellEnvironment, ShellKind, SubcommandSpec,
};

/// Return the list of subcommands with their descriptions.
pub fn get_subcommands() -> Vec<(&'static str, &'static str)> {
    vec![
        ("check", "Type-check and verify an OxiLean source file"),
        ("repl", "Start the interactive proof REPL"),
        ("build", "Compile and build the current project"),
        ("format", "Format OxiLean source files"),
        ("doc", "Generate documentation from definitions"),
        ("lint", "Run static analysis lint rules"),
        ("serve", "Start the Language Server Protocol server"),
        ("clean", "Remove build artifacts and caches"),
        ("test", "Run the project test suite"),
    ]
}
/// Return the list of global flags with their descriptions.
pub fn get_global_flags() -> Vec<(&'static str, &'static str)> {
    vec![
        ("--verbose", "Enable verbose output"),
        ("--help", "Print help information"),
        ("--version", "Print version information"),
        ("--color", "Control colored output (auto|always|never)"),
        ("--no-color", "Disable colored output"),
        ("--config", "Path to configuration file"),
        ("--log-level", "Set log level (error|warn|info|debug|trace)"),
    ]
}
/// Write a completion script to the specified output path.
pub fn write_completion_file(
    shell: ShellKind,
    binary_name: &str,
    output_path: &str,
) -> std::io::Result<()> {
    let script = CompletionGenerator::generate(shell, binary_name);
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(script.as_bytes())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bash_completion() {
        let script = CompletionGenerator::generate(ShellKind::Bash, "oxilean");
        assert!(script.contains("complete -F _oxilean_completion oxilean"));
        assert!(script.contains("check"));
        assert!(script.contains("repl"));
        assert!(script.contains("build"));
        assert!(script.contains("_init_completion"));
    }
    #[test]
    fn test_zsh_completion() {
        let script = CompletionGenerator::generate(ShellKind::Zsh, "oxilean");
        assert!(script.contains("#compdef oxilean"));
        assert!(script.contains("_oxilean"));
        assert!(script.contains("check:"));
        assert!(script.contains("_describe"));
    }
    #[test]
    fn test_fish_completion() {
        let script = CompletionGenerator::generate(ShellKind::Fish, "oxilean");
        assert!(script.contains("complete -c oxilean"));
        assert!(script.contains("__fish_use_subcommand"));
        assert!(script.contains("check"));
        assert!(script.contains("repl"));
    }
    #[test]
    fn test_powershell_completion() {
        let script = CompletionGenerator::generate(ShellKind::PowerShell, "oxilean");
        assert!(script.contains("Register-ArgumentCompleter"));
        assert!(script.contains("oxilean"));
        assert!(script.contains("CompletionResult"));
        assert!(script.contains("check"));
    }
    #[test]
    fn test_elvish_completion() {
        let script = CompletionGenerator::generate(ShellKind::Elvish, "oxilean");
        assert!(script.contains("set edit:completion:arg-completer[oxilean]"));
        assert!(script.contains("subcommands"));
        assert!(script.contains("'check'"));
    }
    #[test]
    fn test_subcommands_list() {
        let cmds = get_subcommands();
        assert_eq!(cmds.len(), 9);
        let names: Vec<&str> = cmds.iter().map(|(n, _)| *n).collect();
        assert!(names.contains(&"check"));
        assert!(names.contains(&"repl"));
        assert!(names.contains(&"build"));
        assert!(names.contains(&"format"));
        assert!(names.contains(&"doc"));
        assert!(names.contains(&"lint"));
        assert!(names.contains(&"serve"));
        assert!(names.contains(&"clean"));
        assert!(names.contains(&"test"));
        let flags = get_global_flags();
        assert!(!flags.is_empty());
        let flag_names: Vec<&str> = flags.iter().map(|(f, _)| *f).collect();
        assert!(flag_names.contains(&"--verbose"));
        assert!(flag_names.contains(&"--help"));
        assert!(flag_names.contains(&"--version"));
    }
}
/// Install a completion script to the given target.
#[allow(dead_code)]
pub fn install_completion(
    shell: ShellKind,
    binary_name: &str,
    target: &CompletionInstallTarget,
) -> std::io::Result<()> {
    let script = CompletionGenerator::generate(shell.clone(), binary_name);
    match target {
        CompletionInstallTarget::Stdout => {
            print!("{}", script);
            Ok(())
        }
        CompletionInstallTarget::CustomPath(path) => {
            let mut f = std::fs::File::create(path)?;
            f.write_all(script.as_bytes())
        }
        CompletionInstallTarget::SystemDir(dir) | CompletionInstallTarget::UserDir(dir) => {
            let ext = shell.file_extension();
            let file_name = format!("{}.{}", binary_name, ext);
            let full_path = format!("{}/{}", dir.trim_end_matches('/'), file_name);
            std::fs::create_dir_all(dir)?;
            let mut f = std::fs::File::create(&full_path)?;
            f.write_all(script.as_bytes())
        }
    }
}
/// Trait for providing dynamic completions.
#[allow(dead_code)]
pub trait DynamicCompletionProvider: Send + Sync {
    /// Return the argument this provider handles.
    fn handles_argument(&self) -> &str;
    /// Return completion candidates for a partial input.
    fn candidates(&self, partial: &str) -> Vec<CompletionCandidate>;
}
/// Format a list of candidates as newline-separated values for shell consumption.
#[allow(dead_code)]
pub fn format_candidates_plain(candidates: &[CompletionCandidate]) -> String {
    candidates
        .iter()
        .map(|c| c.text.clone())
        .collect::<Vec<_>>()
        .join("\n")
}
/// Format candidates as tab-separated text\tdescription pairs (e.g. for Zsh).
#[allow(dead_code)]
pub fn format_candidates_with_descriptions(candidates: &[CompletionCandidate]) -> String {
    candidates
        .iter()
        .map(|c| format!("{}\t{}", c.text, c.description))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Filter candidates by a prefix.
#[allow(dead_code)]
pub fn filter_candidates<'c>(
    candidates: &'c [CompletionCandidate],
    prefix: &str,
) -> Vec<&'c CompletionCandidate> {
    candidates
        .iter()
        .filter(|c| c.text.starts_with(prefix))
        .collect()
}
/// Deduplicate candidates by text, keeping the highest-priority one.
#[allow(dead_code)]
pub fn dedup_candidates(mut candidates: Vec<CompletionCandidate>) -> Vec<CompletionCandidate> {
    candidates.sort_by(|a, b| a.text.cmp(&b.text).then(a.priority.cmp(&b.priority)));
    candidates.dedup_by(|a, b| a.text == b.text);
    candidates
}
/// Attempt to detect the current shell from environment variables.
#[allow(dead_code)]
pub fn detect_shell() -> Option<ShellKind> {
    if let Ok(shell_path) = std::env::var("SHELL") {
        let lower = shell_path.to_lowercase();
        if lower.contains("bash") {
            return Some(ShellKind::Bash);
        }
        if lower.contains("zsh") {
            return Some(ShellKind::Zsh);
        }
        if lower.contains("fish") {
            return Some(ShellKind::Fish);
        }
        if lower.contains("elvish") || lower.contains("elv") {
            return Some(ShellKind::Elvish);
        }
    }
    if std::env::var("PSModulePath").is_ok() {
        return Some(ShellKind::PowerShell);
    }
    None
}
/// Parse a shell name string into a ShellKind.
#[allow(dead_code)]
pub fn parse_shell_kind(s: &str) -> Option<ShellKind> {
    match s.to_lowercase().as_str() {
        "bash" => Some(ShellKind::Bash),
        "zsh" => Some(ShellKind::Zsh),
        "fish" => Some(ShellKind::Fish),
        "powershell" | "pwsh" | "ps1" => Some(ShellKind::PowerShell),
        "elvish" | "elv" => Some(ShellKind::Elvish),
        _ => None,
    }
}
/// Return the completions module version string.
#[allow(dead_code)]
pub fn completions_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod rich_tests {
    use super::*;
    #[test]
    fn test_completion_spec_builder() {
        let spec = CompletionSpec::new("--output", "Output format")
            .with_short("-o")
            .with_possible_values(vec!["text", "json"]);
        assert_eq!(spec.long, "--output");
        assert_eq!(spec.short, Some("-o".to_string()));
        assert!(spec.takes_value);
        assert_eq!(spec.possible_values, vec!["text", "json"]);
    }
    #[test]
    fn test_subcommand_spec_builder() {
        let spec = SubcommandSpec::new("check", "Check a file")
            .with_alias("c")
            .accepts_files();
        assert_eq!(spec.name, "check");
        assert!(spec.aliases.contains(&"c".to_string()));
        assert!(spec.accepts_files);
    }
    #[test]
    fn test_app_spec_find_subcommand() {
        let app = AppCompletionSpec::oxilean_spec();
        let found = app.find_subcommand("check");
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").name, "check");
        let alias = app.find_subcommand("fmt");
        assert!(alias.is_some());
    }
    #[test]
    fn test_rich_bash_generation() {
        let spec = AppCompletionSpec::oxilean_spec();
        let gen = RichCompletionGenerator::new(&spec);
        let script = gen.generate(ShellKind::Bash);
        assert!(script.contains("oxilean"));
        assert!(script.contains("check"));
        assert!(script.contains("build"));
    }
    #[test]
    fn test_rich_zsh_generation() {
        let spec = AppCompletionSpec::oxilean_spec();
        let gen = RichCompletionGenerator::new(&spec);
        let script = gen.generate(ShellKind::Zsh);
        assert!(script.contains("#compdef oxilean"));
        assert!(script.contains("--verbose"));
    }
    #[test]
    fn test_rich_fish_generation() {
        let spec = AppCompletionSpec::oxilean_spec();
        let gen = RichCompletionGenerator::new(&spec);
        let script = gen.generate(ShellKind::Fish);
        assert!(script.contains("complete -c oxilean"));
    }
    #[test]
    fn test_completion_context_parse() {
        let ctx = CompletionContext::from_cmdline("oxilean check src/", 18);
        assert_eq!(ctx.active_subcommand, Some("check".to_string()));
    }
    #[test]
    fn test_detect_shell() {
        let _ = detect_shell();
    }
    #[test]
    fn test_parse_shell_kind() {
        assert_eq!(parse_shell_kind("bash"), Some(ShellKind::Bash));
        assert_eq!(parse_shell_kind("ZSH"), Some(ShellKind::Zsh));
        assert_eq!(parse_shell_kind("fish"), Some(ShellKind::Fish));
        assert_eq!(parse_shell_kind("pwsh"), Some(ShellKind::PowerShell));
        assert_eq!(parse_shell_kind("elvish"), Some(ShellKind::Elvish));
        assert_eq!(parse_shell_kind("unknown"), None);
    }
    #[test]
    fn test_format_candidates() {
        let cands = vec![
            CompletionCandidate::new("--verbose", "Verbose"),
            CompletionCandidate::new("--help", "Help"),
        ];
        let plain = format_candidates_plain(&cands);
        assert!(plain.contains("--verbose"));
        assert!(plain.contains("--help"));
        let with_desc = format_candidates_with_descriptions(&cands);
        assert!(with_desc.contains('\t'));
    }
    #[test]
    fn test_dedup_candidates() {
        let cands = vec![
            CompletionCandidate::new("--verbose", "A"),
            CompletionCandidate::new("--verbose", "B"),
            CompletionCandidate::new("--help", "Help"),
        ];
        let deduped = dedup_candidates(cands);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_dynamic_registry() {
        let mut registry = DynamicCompletionRegistry::new();
        let provider = FileSystemCompletionProvider::new("--file");
        registry.register(Box::new(provider));
        let _cands = registry.complete("--file", "");
    }
    #[test]
    fn test_engine_top_level() {
        let spec = AppCompletionSpec::oxilean_spec();
        let registry = DynamicCompletionRegistry::new();
        let engine = CompletionEngine::new(&spec, &registry);
        let ctx = CompletionContext {
            cmdline: "oxilean ".to_string(),
            current_word: String::new(),
            cursor_position: 1,
            active_subcommand: None,
        };
        let cands = engine.complete(&ctx);
        assert!(!cands.is_empty());
    }
    #[test]
    fn test_completion_install_target() {
        let target = CompletionInstallTarget::Stdout;
        assert!(target.path().is_none());
        let target = CompletionInstallTarget::CustomPath("/tmp/test_completion.bash".to_string());
        assert_eq!(target.path(), Some("/tmp/test_completion.bash"));
    }
    #[test]
    fn test_completions_version() {
        assert!(!completions_version().is_empty());
    }
}
/// Compute a simple fuzzy match score between `pattern` and `text`.
/// Returns Some(score) if all pattern chars appear as a subsequence in text, else None.
#[allow(dead_code)]
pub fn fuzzy_match_score(pattern: &str, text: &str) -> Option<i64> {
    if pattern.is_empty() {
        return Some(0);
    }
    let pat_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let mut pi = 0;
    let mut ti = 0;
    let mut score: i64 = 0;
    let mut last_match: Option<usize> = None;
    while pi < pat_chars.len() && ti < text_chars.len() {
        if pat_chars[pi].to_lowercase().next() == text_chars[ti].to_lowercase().next() {
            if let Some(last) = last_match {
                if last + 1 == ti {
                    score += 2;
                }
            }
            if ti == 0 || text_chars[ti - 1] == '-' || text_chars[ti - 1] == '_' {
                score += 3;
            }
            score += 1;
            last_match = Some(ti);
            pi += 1;
        }
        ti += 1;
    }
    if pi == pat_chars.len() {
        Some(score)
    } else {
        None
    }
}
/// Sort candidates by fuzzy match score against the given pattern.
#[allow(dead_code)]
pub fn fuzzy_sort_candidates(
    candidates: Vec<CompletionCandidate>,
    pattern: &str,
) -> Vec<CompletionCandidate> {
    let mut scored: Vec<(i64, CompletionCandidate)> = candidates
        .into_iter()
        .filter_map(|c| fuzzy_match_score(pattern, &c.text).map(|score| (score, c)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, c)| c).collect()
}
/// Render completion candidates in a given output format.
#[allow(dead_code)]
pub fn render_completions(
    candidates: &[CompletionCandidate],
    format: CompletionOutputFormat,
) -> String {
    match format {
        CompletionOutputFormat::Lines => format_candidates_plain(candidates),
        CompletionOutputFormat::Json => {
            let items: Vec<String> = candidates
                .iter()
                .map(|c| {
                    format!(
                        "{{\"text\":\"{}\",\"description\":\"{}\"}}",
                        c.text.replace('"', "\\\""),
                        c.description.replace('"', "\\\"")
                    )
                })
                .collect();
            format!("[{}]", items.join(","))
        }
        CompletionOutputFormat::ZshArray => {
            let items: Vec<String> = candidates
                .iter()
                .map(|c| format!("'{}:{}'", c.text, c.description))
                .collect();
            format!("({})", items.join(" "))
        }
        CompletionOutputFormat::Fish => format_candidates_with_descriptions(candidates),
    }
}
/// Convert a list of candidates to LSP completion items.
#[allow(dead_code)]
pub fn candidates_to_lsp_items(candidates: &[CompletionCandidate]) -> Vec<LspCompletionItem> {
    candidates
        .iter()
        .map(LspCompletionItem::from_candidate)
        .collect()
}
/// Generate the header comment block for a completion script.
#[allow(dead_code)]
pub fn completion_script_header(binary_name: &str, shell: &ShellKind, version: &str) -> String {
    format!(
        "# {shell_name} completion script for {binary}\n# Generated by oxilean-cli v{version}\n# DO NOT EDIT - regenerate with: {binary} completions {shell_name}\n\n",
        shell_name = shell.name(), binary = binary_name, version = version,
    )
}
/// Generate instructions for loading a completion script.
#[allow(dead_code)]
pub fn loading_instructions(binary_name: &str, shell: &ShellKind) -> String {
    match shell {
        ShellKind::Bash => {
            format!(
                "# Add to ~/.bashrc:\n# source <({} completions bash)\n",
                binary_name
            )
        }
        ShellKind::Zsh => {
            format!(
                "# Add to ~/.zshrc:\n# autoload -U compinit && compinit\n# source <({} completions zsh)\n",
                binary_name
            )
        }
        ShellKind::Fish => {
            format!(
                "# Save to ~/.config/fish/completions/{}.fish:\n# {} completions fish > ~/.config/fish/completions/{}.fish\n",
                binary_name, binary_name, binary_name
            )
        }
        ShellKind::PowerShell => {
            format!(
                "# Add to $PROFILE:\n# {} completions powershell | Invoke-Expression\n",
                binary_name
            )
        }
        ShellKind::Elvish => {
            format!(
                "# Add to ~/.elvish/rc.elv:\n# eval ({} completions elvish)\n",
                binary_name
            )
        }
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_completion_cache_basic() {
        let mut cache = CompletionCache::new(10);
        assert!(cache.is_empty());
        cache.insert(
            "key1".to_string(),
            vec![CompletionCandidate::new("--help", "Help")],
            60000,
        );
        assert_eq!(cache.len(), 1);
        let found = cache.get("key1");
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").len(), 1);
    }
    #[test]
    fn test_completion_cache_expired() {
        let mut cache = CompletionCache::new(10);
        cache.insert(
            "key1".to_string(),
            vec![CompletionCandidate::new("--help", "Help")],
            0,
        );
        let _ = cache.get("key1");
    }
    #[test]
    fn test_completion_history() {
        let mut history = CompletionHistory::new(100);
        history.record("check".to_string(), "check".to_string(), None);
        history.record("check".to_string(), "check".to_string(), None);
        let top = history.top_completions("check", 5);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].count, 2);
    }
    #[test]
    fn test_fuzzy_match_score() {
        let score = fuzzy_match_score("chk", "--check");
        assert!(score.is_some());
        let no_match = fuzzy_match_score("xyz", "--check");
        assert!(no_match.is_none());
    }
    #[test]
    fn test_fuzzy_sort_candidates() {
        let cands = vec![
            CompletionCandidate::new("--check", "Check"),
            CompletionCandidate::new("--color", "Color"),
            CompletionCandidate::new("--config", "Config"),
        ];
        let sorted = fuzzy_sort_candidates(cands, "ch");
        assert!(!sorted.is_empty());
        assert!(sorted[0].text == "--check" || sorted[0].text == "--color");
    }
    #[test]
    fn test_render_completions_json() {
        let cands = vec![CompletionCandidate::new("--help", "Help")];
        let json = render_completions(&cands, CompletionOutputFormat::Json);
        assert!(json.starts_with('['));
        assert!(json.contains("\"text\""));
    }
    #[test]
    fn test_render_completions_zsh() {
        let cands = vec![CompletionCandidate::new("--help", "Help")];
        let zsh = render_completions(&cands, CompletionOutputFormat::ZshArray);
        assert!(zsh.starts_with('('));
        assert!(zsh.ends_with(')'));
    }
    #[test]
    fn test_completion_stats() {
        let mut stats = CompletionStats::new();
        stats.record_hit();
        stats.record_miss(5, 100);
        assert_eq!(stats.total_requests, 2);
        assert!((stats.hit_rate() - 50.0).abs() < 1.0);
    }
    #[test]
    fn test_lsp_completion_item() {
        let item = LspCompletionItem::new("--help", CompletionItemKind::Keyword)
            .with_detail("Print help")
            .with_documentation("Prints the help message");
        assert_eq!(item.label, "--help");
        assert_eq!(item.kind, CompletionItemKind::Keyword);
        assert!(item.detail.is_some());
        assert!(item.documentation.is_some());
        assert!(!item.deprecated);
    }
    #[test]
    fn test_candidates_to_lsp_items() {
        let cands = vec![
            CompletionCandidate::new("--help", "Help"),
            CompletionCandidate::new("--verbose", "Verbose"),
        ];
        let items = candidates_to_lsp_items(&cands);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].label, "--help");
    }
    #[test]
    fn test_completion_script_header() {
        let header = completion_script_header("oxilean", &ShellKind::Bash, "1.0.0");
        assert!(header.contains("oxilean"));
        assert!(header.contains("bash"));
        assert!(header.contains("1.0.0"));
    }
    #[test]
    fn test_loading_instructions() {
        for shell in &[
            ShellKind::Bash,
            ShellKind::Zsh,
            ShellKind::Fish,
            ShellKind::PowerShell,
            ShellKind::Elvish,
        ] {
            let instr = loading_instructions("oxilean", shell);
            assert!(!instr.is_empty());
            assert!(instr.contains("oxilean"));
        }
    }
    #[test]
    fn test_shell_environment_detect() {
        let env = ShellEnvironment::detect();
        let _ = env.kind;
    }
    #[test]
    fn test_completion_output_format_fish() {
        let cands = vec![
            CompletionCandidate::new("check", "Check"),
            CompletionCandidate::new("build", "Build"),
        ];
        let fish = render_completions(&cands, CompletionOutputFormat::Fish);
        assert!(fish.contains('\t'));
    }
    #[test]
    fn test_filter_candidates() {
        let cands = vec![
            CompletionCandidate::new("--help", "Help"),
            CompletionCandidate::new("--verbose", "Verbose"),
            CompletionCandidate::new("--version", "Version"),
        ];
        let filtered = filter_candidates(&cands, "--ver");
        assert_eq!(filtered.len(), 2);
    }
}
/// Return the module's feature set description.
#[allow(dead_code)]
pub fn completion_features() -> Vec<&'static str> {
    vec![
        "bash",
        "zsh",
        "fish",
        "powershell",
        "elvish",
        "rich-specs",
        "dynamic-providers",
        "caching",
        "history",
        "fuzzy-matching",
        "lsp-items",
        "middleware",
        "stats",
    ]
}
#[cfg(test)]
mod config_tests {
    use super::*;
    #[test]
    fn test_completion_config_default() {
        let cfg = CompletionConfig::default();
        assert!(cfg.enabled);
        assert!(cfg.fuzzy_matching);
        assert!(cfg.history_enabled);
        assert_eq!(cfg.max_candidates, 50);
    }
    #[test]
    fn test_completion_config_disabled() {
        let cfg = CompletionConfig::disabled();
        assert!(!cfg.enabled);
    }
    #[test]
    fn test_completion_features() {
        let features = completion_features();
        assert!(features.contains(&"bash"));
        assert!(features.contains(&"fuzzy-matching"));
    }
}
