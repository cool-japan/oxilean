//! # BashExpr - Trait Implementations
//!
//! This module contains trait implementations for `BashExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BashExpr;
use std::fmt;

impl fmt::Display for BashExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BashExpr::Var(v) => write!(f, "{}", v),
            BashExpr::Lit(s) => write!(f, "'{}'", s.replace('\'', "'\\''")),
            BashExpr::DQuoted(s) => write!(f, "\"{}\"", s),
            BashExpr::CmdSubst(cmd) => write!(f, "$({})", cmd),
            BashExpr::ArithExpr(expr) => write!(f, "$(({}))", expr),
            BashExpr::ProcSubst { is_input, cmd } => {
                if *is_input {
                    write!(f, "<({})", cmd)
                } else {
                    write!(f, ">({})", cmd)
                }
            }
            BashExpr::ArrayElem(name, idx) => write!(f, "${{{}[{}]}}", name, idx),
            BashExpr::ArrayLen(name) => write!(f, "${{#{}[@]}}", name),
            BashExpr::ArrayAll(name) => write!(f, "\"${{{}[@]}}\"", name),
            BashExpr::AssocElem(name, key) => write!(f, "${{{}[{}]}}", name, key),
            BashExpr::Default(var, default) => write!(f, "${{{}:-{}}}", var, default),
            BashExpr::AssignDefault(var, default) => {
                write!(f, "${{{}:={}}}", var, default)
            }
            BashExpr::Substring(var, off, None) => write!(f, "${{{}:{}}}", var, off),
            BashExpr::Substring(var, off, Some(len)) => {
                write!(f, "${{{}:{}:{}}}", var, off, len)
            }
            BashExpr::StringLen(var) => write!(f, "${{#{}}}", var),
            BashExpr::StripPrefix(var, pat) => write!(f, "${{{}#{}}}", var, pat),
            BashExpr::StripSuffix(var, pat) => write!(f, "${{{}%{}}}", var, pat),
            BashExpr::UpperCase(var) => write!(f, "${{{}^^}}", var),
            BashExpr::LowerCase(var) => write!(f, "${{{},,}}", var),
            BashExpr::LastStatus => write!(f, "$?"),
            BashExpr::ShellPid => write!(f, "$$"),
            BashExpr::ScriptName => write!(f, "$0"),
            BashExpr::Positional(n) => write!(f, "${}", n),
            BashExpr::AllArgs => write!(f, "\"$@\""),
            BashExpr::ArgCount => write!(f, "$#"),
            BashExpr::Concat(a, b) => write!(f, "{}{}", a, b),
        }
    }
}
