//! # RichCompletionGenerator - generate_group Methods
//!
//! This module contains method implementations for `RichCompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ShellKind;

use super::functions::*;
use super::richcompletiongenerator_type::RichCompletionGenerator;

impl<'a> RichCompletionGenerator<'a> {
    /// Generate completion for a given shell.
    #[allow(dead_code)]
    pub fn generate(&self, shell: ShellKind) -> String {
        match shell {
            ShellKind::Bash => self.generate_bash(),
            ShellKind::Zsh => self.generate_zsh(),
            ShellKind::Fish => self.generate_fish(),
            ShellKind::PowerShell => self.generate_powershell(),
            ShellKind::Elvish => self.generate_elvish(),
        }
    }
    fn generate_fish(&self) -> String {
        let binary = &self.spec.binary_name;
        let mut out = format!(
            "# Rich Fish completion for {binary}\n\ncomplete -c {binary} -f\n\n",
            binary = binary
        );
        for f in &self.spec.global_flags {
            let long = f.long.trim_start_matches('-');
            if !f.possible_values.is_empty() {
                for val in &f.possible_values {
                    out.push_str(&format!(
                        "complete -c {binary} -l {long} -a '{val}' -d '{desc}'\n",
                        binary = binary,
                        long = long,
                        val = val,
                        desc = f.description
                    ));
                }
            } else {
                out.push_str(&format!(
                    "complete -c {binary} -l {long} -d '{desc}'\n",
                    binary = binary,
                    long = long,
                    desc = f.description
                ));
            }
        }
        out.push('\n');
        for s in &self.spec.subcommands {
            out.push_str(&format!(
                "complete -c {binary} -n '__fish_use_subcommand' -a {name} -d '{desc}'\n",
                binary = binary,
                name = s.name,
                desc = s.description
            ));
        }
        out
    }
    fn generate_elvish(&self) -> String {
        let binary = &self.spec.binary_name;
        let subcmds: Vec<String> = self
            .spec
            .subcommands
            .iter()
            .map(|s| format!("'{}'", s.name))
            .collect();
        let flags: Vec<String> = self
            .spec
            .global_flags
            .iter()
            .map(|f| format!("'{}'", f.long))
            .collect();
        format!(
            r#"# Rich Elvish completion for {binary}
set edit:completion:arg-completer[{binary}] = {{|@args|
    var subcommands = [{subcmds}]
    var global-flags = [{flags}]
    var nargs = (count $args)
    if (== $nargs 2) {{
        for cmd $subcommands {{ edit:complex-candidate $cmd }}
        for flag $global-flags {{ edit:complex-candidate $flag }}
    }}
}}
"#,
            binary = binary,
            subcmds = subcmds.join(" "),
            flags = flags.join(" "),
        )
    }
}
