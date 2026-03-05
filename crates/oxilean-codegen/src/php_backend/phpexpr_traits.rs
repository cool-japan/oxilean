//! # PHPExpr - Trait Implementations
//!
//! This module contains trait implementations for `PHPExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_param;
use super::types::PHPExpr;
use std::fmt;

impl fmt::Display for PHPExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PHPExpr::Lit(s) => write!(f, "{}", s),
            PHPExpr::Var(name) => write!(f, "${}", name),
            PHPExpr::BinOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            PHPExpr::UnaryOp(op, operand) => write!(f, "({}{})", op, operand),
            PHPExpr::FuncCall(name, args) => {
                let args_s: Vec<std::string::String> =
                    args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}({})", name, args_s.join(", "))
            }
            PHPExpr::ArrayLit(elems) => {
                let s: Vec<std::string::String> = elems.iter().map(|e| format!("{}", e)).collect();
                write!(f, "[{}]", s.join(", "))
            }
            PHPExpr::ArrayMap(pairs) => {
                let s: Vec<std::string::String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{} => {}", k, v))
                    .collect();
                write!(f, "[{}]", s.join(", "))
            }
            PHPExpr::New(class, args) => {
                let args_s: Vec<std::string::String> =
                    args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "new {}({})", class, args_s.join(", "))
            }
            PHPExpr::Arrow(obj, prop) => {
                write!(
                    f,
                    "{}->{}",
                    if matches!(obj.as_ref(), PHPExpr::Var(_)) {
                        format!("{}", obj)
                    } else {
                        format!("({})", obj)
                    },
                    prop
                )
            }
            PHPExpr::NullSafe(obj, prop) => {
                write!(
                    f,
                    "{}?->{}",
                    if matches!(obj.as_ref(), PHPExpr::Var(_)) {
                        format!("{}", obj)
                    } else {
                        format!("({})", obj)
                    },
                    prop
                )
            }
            PHPExpr::StaticAccess(class, member) => write!(f, "{}::{}", class, member),
            PHPExpr::Index(arr, idx) => write!(f, "{}[{}]", arr, idx),
            PHPExpr::Ternary(cond, then, else_) => {
                write!(f, "({} ? {} : {})", cond, then, else_)
            }
            PHPExpr::NullCoalesce(a, b) => write!(f, "({} ?? {})", a, b),
            PHPExpr::Closure {
                params,
                use_vars,
                return_type,
                body,
            } => {
                let params_s: Vec<std::string::String> = params.iter().map(format_param).collect();
                let use_s = if use_vars.is_empty() {
                    std::string::String::new()
                } else {
                    format!(
                        " use ({})",
                        use_vars
                            .iter()
                            .map(|v| format!("${}", v))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                let ret_s = return_type
                    .as_ref()
                    .map(|t| format!(": {}", t))
                    .unwrap_or_default();
                let body_s = body.join("\n        ");
                write!(
                    f,
                    "function({}){}{} {{\n        {}\n    }}",
                    params_s.join(", "),
                    use_s,
                    ret_s,
                    body_s
                )
            }
            PHPExpr::ArrowFn {
                params,
                return_type,
                body,
            } => {
                let params_s: Vec<std::string::String> = params.iter().map(format_param).collect();
                let ret_s = return_type
                    .as_ref()
                    .map(|t| format!(": {}", t))
                    .unwrap_or_default();
                write!(f, "fn({}){} => {}", params_s.join(", "), ret_s, body)
            }
            PHPExpr::Match {
                subject,
                arms,
                default,
            } => {
                let arms_s: Vec<std::string::String> = arms
                    .iter()
                    .map(|(pat, val)| format!("    {} => {}", pat, val))
                    .collect();
                let default_s = default
                    .as_ref()
                    .map(|d| format!(",\n    default => {}", d))
                    .unwrap_or_default();
                write!(
                    f,
                    "match ({}) {{\n{}{}\n}}",
                    subject,
                    arms_s.join(",\n"),
                    default_s
                )
            }
            PHPExpr::NamedArg(name, val) => write!(f, "{}: {}", name, val),
            PHPExpr::Spread(expr) => write!(f, "...{}", expr),
            PHPExpr::Cast(ty, expr) => write!(f, "({}) {}", ty, expr),
            PHPExpr::Isset(expr) => write!(f, "isset({})", expr),
            PHPExpr::Empty(expr) => write!(f, "empty({})", expr),
        }
    }
}
