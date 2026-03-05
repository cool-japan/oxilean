//! # RichCompletionGenerator - new_group Methods
//!
//! This module contains method implementations for `RichCompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AppCompletionSpec;

use super::functions::*;
use super::richcompletiongenerator_type::RichCompletionGenerator;

impl<'a> RichCompletionGenerator<'a> {
    /// Create a new generator for the given spec.
    #[allow(dead_code)]
    pub fn new(spec: &'a AppCompletionSpec) -> Self {
        Self { spec }
    }
    pub fn generate_bash(&self) -> String {
        let binary = &self.spec.binary_name;
        let fn_name = format!("_{}_rich", binary.replace('-', "_"));
        let global_words: Vec<String> = self
            .spec
            .global_flags
            .iter()
            .map(Self::flag_to_bash_word)
            .collect();
        let subcmd_names: Vec<String> = self
            .spec
            .subcommands
            .iter()
            .map(|s| s.name.clone())
            .collect();
        let all_top: Vec<String> = subcmd_names
            .iter()
            .chain(global_words.iter())
            .cloned()
            .collect();
        let mut subcmd_cases = String::new();
        for subcmd in &self.spec.subcommands {
            let flag_words: Vec<String> =
                subcmd.flags.iter().map(Self::flag_to_bash_word).collect();
            let words = flag_words.join(" ");
            subcmd_cases
                .push_str(
                    &format!(
                        "            {name})\n                COMPREPLY=($(compgen -W \"{words}\" -- \"${{cur}}\"))\n                return 0\n                ;;\n",
                        name = subcmd.name, words = words
                    ),
                );
            for alias in &subcmd.aliases {
                subcmd_cases
                    .push_str(
                        &format!(
                            "            {alias})\n                COMPREPLY=($(compgen -W \"{words}\" -- \"${{cur}}\"))\n                return 0\n                ;;\n",
                            alias = alias, words = words
                        ),
                    );
            }
        }
        format!(
            r#"# Rich Bash completion for {binary}

{fn_name}() {{
    local cur prev words cword
    _init_completion || return

    if [[ ${{cword}} -eq 1 ]]; then
        COMPREPLY=($(compgen -W "{all_top}" -- "${{cur}}"))
        return 0
    fi

    local subcmd="${{words[1]}}"
    case "$subcmd" in
{subcmd_cases}        *)
            COMPREPLY=($(compgen -W "{global}" -- "${{cur}}"))
            ;;
    esac
}}

complete -F {fn_name} {binary}
"#,
            binary = binary,
            fn_name = fn_name,
            all_top = all_top.join(" "),
            subcmd_cases = subcmd_cases,
            global = global_words.join(" "),
        )
    }
}
