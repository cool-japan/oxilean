//! # CompletionGenerator - generate_powershell_group Methods
//!
//! This module contains method implementations for `CompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::completiongenerator_type::CompletionGenerator;
use super::functions::*;

impl CompletionGenerator {
    pub fn generate_powershell(binary_name: &str) -> String {
        let subcommands = get_subcommands();
        let global_flags = get_global_flags();
        let mut completions = String::new();
        for (name, desc) in &subcommands {
            completions
                .push_str(
                    &format!(
                        "                [CompletionResult]::new('{name}', '{name}', [CompletionResultType]::ParameterValue, '{desc}')\n",
                        name = name, desc = desc
                    ),
                );
        }
        for (flag, desc) in &global_flags {
            completions
                .push_str(
                    &format!(
                        "                [CompletionResult]::new('{flag}', '{flag}', [CompletionResultType]::ParameterName, '{desc}')\n",
                        flag = flag, desc = desc
                    ),
                );
        }
        format!(
            r#"# PowerShell completion for {binary_name}
# Add this to your PowerShell profile ($PROFILE)

Register-ArgumentCompleter -Native -CommandName '{binary_name}' -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $completions = @(
{completions}    )

    $completions | Where-Object {{ $_.CompletionText -like "$wordToComplete*" }}
}}
"#,
            binary_name = binary_name,
            completions = completions,
        )
    }
}
