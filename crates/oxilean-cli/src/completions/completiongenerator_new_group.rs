//! # CompletionGenerator - new_group Methods
//!
//! This module contains method implementations for `CompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::completiongenerator_type::CompletionGenerator;
use super::functions::*;

impl CompletionGenerator {
    /// Create a new completion generator.
    pub fn new() -> Self {
        Self
    }
    pub fn generate_bash(binary_name: &str) -> String {
        let subcommands = get_subcommands();
        let global_flags = get_global_flags();
        let subcmd_names: Vec<&str> = subcommands.iter().map(|(name, _)| *name).collect();
        let flag_names: Vec<&str> = global_flags.iter().map(|(flag, _)| *flag).collect();
        let subcmds_str = subcmd_names.join(" ");
        let flags_str = flag_names.join(" ");
        let fn_name = format!("_{}_completion", binary_name.replace('-', "_"));
        let mut cases = String::new();
        for (name, _desc) in &subcommands {
            cases
                .push_str(
                    &format!(
                        "            {name})\n                COMPREPLY=($(compgen -W \"--help --verbose\" -- \"${{cur}}\"))\n                return 0\n                ;;\n",
                        name = name
                    ),
                );
        }
        format!(
            r#"# Bash completion for {binary_name}
# Source this file or place it in /etc/bash_completion.d/ or ~/.bash_completion.d/

{fn_name}() {{
    local cur prev words cword
    _init_completion || return

    local subcommands="{subcmds_str}"
    local global_flags="{flags_str}"

    if [[ ${{cword}} -eq 1 ]]; then
        COMPREPLY=($(compgen -W "$subcommands $global_flags" -- "${{cur}}"))
        return 0
    fi

    local subcmd="${{words[1]}}"
    case "$subcmd" in
{cases}        *)
            COMPREPLY=($(compgen -W "$global_flags" -- "${{cur}}"))
            ;;
    esac
}}

complete -F {fn_name} {binary_name}
"#,
            binary_name = binary_name,
            fn_name = fn_name,
            subcmds_str = subcmds_str,
            flags_str = flags_str,
            cases = cases,
        )
    }
}
