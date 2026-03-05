//! # RichCompletionGenerator - generate_zsh_group Methods
//!
//! This module contains method implementations for `RichCompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::richcompletiongenerator_type::RichCompletionGenerator;

impl<'a> RichCompletionGenerator<'a> {
    pub fn generate_zsh(&self) -> String {
        let binary = &self.spec.binary_name;
        let mut subcmd_defs = String::new();
        for s in &self.spec.subcommands {
            subcmd_defs.push_str(&format!(
                "        '{name}:{desc}'\n",
                name = s.name,
                desc = s.description
            ));
        }
        let mut flag_args = String::new();
        for f in &self.spec.global_flags {
            let valspec = if f.takes_value {
                if !f.possible_values.is_empty() {
                    format!(":value:({})", f.possible_values.join(" "))
                } else if f.is_file_path {
                    ":file:_files".to_string()
                } else {
                    ":value: ".to_string()
                }
            } else {
                String::new()
            };
            flag_args.push_str(&format!(
                "        '{long}[{desc}]{valspec}' \\\n",
                long = f.long,
                desc = f.description,
                valspec = valspec
            ));
        }
        format!(
            r#"#compdef {binary}

_{binary}_rich() {{
    local state
    _arguments -C \
{flag_args}        '1:command:->subcmds' \
        '*:: :->args'

    case $state in
        subcmds)
            local subcommands
            subcommands=(
{subcmd_defs}            )
            _describe 'subcommand' subcommands
            ;;
        args)
            ;;
    esac
}}

_{binary}_rich "$@"
"#,
            binary = binary,
            flag_args = flag_args,
            subcmd_defs = subcmd_defs,
        )
    }
    pub fn generate_powershell(&self) -> String {
        let binary = &self.spec.binary_name;
        let mut completions = String::new();
        for s in &self.spec.subcommands {
            completions
                .push_str(
                    &format!(
                        "    [CompletionResult]::new('{name}', '{name}', [CompletionResultType]::ParameterValue, '{desc}')\n",
                        name = s.name, desc = s.description
                    ),
                );
        }
        for f in &self.spec.global_flags {
            completions
                .push_str(
                    &format!(
                        "    [CompletionResult]::new('{flag}', '{flag}', [CompletionResultType]::ParameterName, '{desc}')\n",
                        flag = f.long, desc = f.description
                    ),
                );
        }
        format!(
            r#"# Rich PowerShell completion for {binary}
Register-ArgumentCompleter -Native -CommandName '{binary}' -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)
    $completions = @(
{completions}    )
    $completions | Where-Object {{ $_.CompletionText -like "$wordToComplete*" }}
}}
"#,
            binary = binary,
            completions = completions,
        )
    }
}
