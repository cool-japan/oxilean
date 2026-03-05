//! # CompletionGenerator - generate_group Methods
//!
//! This module contains method implementations for `CompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ShellKind;

use super::completiongenerator_type::CompletionGenerator;
use super::functions::*;

impl CompletionGenerator {
    /// Generate a completion script for the given shell and binary name.
    pub fn generate(shell: ShellKind, binary_name: &str) -> String {
        match shell {
            ShellKind::Bash => Self::generate_bash(binary_name),
            ShellKind::Zsh => Self::generate_zsh(binary_name),
            ShellKind::Fish => Self::generate_fish(binary_name),
            ShellKind::PowerShell => Self::generate_powershell(binary_name),
            ShellKind::Elvish => Self::generate_elvish(binary_name),
        }
    }
    fn generate_fish(binary_name: &str) -> String {
        let subcommands = get_subcommands();
        let global_flags = get_global_flags();
        let mut lines = format!(
            "# Fish completion for {binary_name}\n# Place this file in ~/.config/fish/completions/\n\n",
            binary_name = binary_name
        );
        lines.push_str(&format!(
            "complete -c {binary_name} -f\n\n",
            binary_name = binary_name
        ));
        lines.push_str("# Global flags\n");
        for (flag, desc) in &global_flags {
            let flag_clean = flag.trim_start_matches('-');
            lines.push_str(&format!(
                "complete -c {binary_name} -l {flag} -d '{desc}'\n",
                binary_name = binary_name,
                flag = flag_clean,
                desc = desc
            ));
        }
        lines.push('\n');
        lines.push_str("# Subcommands\n");
        for (name, desc) in &subcommands {
            lines.push_str(&format!(
                "complete -c {binary_name} -n '__fish_use_subcommand' -a {name} -d '{desc}'\n",
                binary_name = binary_name,
                name = name,
                desc = desc
            ));
        }
        lines.push('\n');
        for (name, _) in &subcommands {
            lines.push_str(&format!(
                "# Completions for '{name}' subcommand\n",
                name = name
            ));
            lines
                .push_str(
                    &format!(
                        "complete -c {binary_name} -n '__fish_seen_subcommand_from {name}' -l help -d 'Show help'\n",
                        binary_name = binary_name, name = name
                    ),
                );
            lines
                .push_str(
                    &format!(
                        "complete -c {binary_name} -n '__fish_seen_subcommand_from {name}' -l verbose -d 'Verbose output'\n",
                        binary_name = binary_name, name = name
                    ),
                );
        }
        lines
    }
    fn generate_elvish(binary_name: &str) -> String {
        let subcommands = get_subcommands();
        let global_flags = get_global_flags();
        let subcmd_list: Vec<String> = subcommands
            .iter()
            .map(|(name, _)| format!("'{}'", name))
            .collect();
        let subcmds_elvish = subcmd_list.join(" ");
        let flag_list: Vec<String> = global_flags
            .iter()
            .map(|(flag, _)| format!("'{}'", flag))
            .collect();
        let flags_elvish = flag_list.join(" ");
        format!(
            r#"# Elvish completion for {binary_name}
# Place this in your ~/.elvish/rc.elv

set edit:completion:arg-completer[{binary_name}] = {{|@args|
    var subcommands = [{subcmds_elvish}]
    var global-flags = [{flags_elvish}]

    var nargs = (count $args)
    if (== $nargs 2) {{
        # Complete top-level subcommand or flag
        for cmd $subcommands {{
            edit:complex-candidate $cmd
        }}
        for flag $global-flags {{
            edit:complex-candidate $flag
        }}
    }} elif (>= $nargs 3) {{
        # Subcommand-specific completions
        var subcmd = $args[1]
        if (has-value $subcommands $subcmd) {{
            edit:complex-candidate '--help'
            edit:complex-candidate '--verbose'
        }}
    }}
}}
"#,
            binary_name = binary_name,
            subcmds_elvish = subcmds_elvish,
            flags_elvish = flags_elvish,
        )
    }
}
