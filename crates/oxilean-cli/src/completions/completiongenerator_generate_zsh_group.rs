//! # CompletionGenerator - generate_zsh_group Methods
//!
//! This module contains method implementations for `CompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::completiongenerator_type::CompletionGenerator;
use super::functions::*;

impl CompletionGenerator {
    pub fn generate_zsh(binary_name: &str) -> String {
        let subcommands = get_subcommands();
        let global_flags = get_global_flags();
        let mut subcmd_defs = String::new();
        for (name, desc) in &subcommands {
            subcmd_defs.push_str(&format!(
                "        '{name}:{desc}'\n",
                name = name,
                desc = desc
            ));
        }
        let mut flag_defs = String::new();
        for (flag, desc) in &global_flags {
            flag_defs.push_str(&format!("    '{flag}[{desc}]'\n", flag = flag, desc = desc));
        }
        let mut subcmd_cases = String::new();
        for (name, _) in &subcommands {
            subcmd_cases
                .push_str(
                    &format!(
                        "        ({name})\n            _arguments \\\n                '--help[Show help]' \\\n                '--verbose[Verbose output]' \\\n                '*:file:_files'\n            ;;\n",
                        name = name
                    ),
                );
        }
        format!(
            r#"#compdef {binary_name}
# Zsh completion for {binary_name}
# Place this file in a directory listed in $fpath

_{binary_name}() {{
    local state

    _arguments -C \
        '--help[Print help information]' \
        '--version[Print version information]' \
        '--verbose[Enable verbose output]' \
        '--color[Control colored output]:color:(auto always never)' \
        '--no-color[Disable colored output]' \
        '--config[Path to configuration file]:file:_files' \
        '--log-level[Set log level]:level:(error warn info debug trace)' \
        '1:command:->subcmds' \
        '*:: :->args'

    case $state in
        subcmds)
            local subcommands
            subcommands=(
{subcmd_defs}            )
            _describe 'subcommand' subcommands
            ;;
        args)
            case $words[1] in
{subcmd_cases}            esac
            ;;
    esac
}}

_{binary_name} "$@"
"#,
            binary_name = binary_name,
            subcmd_defs = subcmd_defs,
            subcmd_cases = subcmd_cases,
        )
    }
}
