//! `RubyModule::emit` against the module printer written with `write!`.

use super::*;
use std::fmt::Write as FmtWrite;

fn reference_emit(module: &RubyModule) -> std::string::String {
    let mut out = std::string::String::new();
    writeln!(out, "# frozen_string_literal: true").expect("writing to String never fails");
    writeln!(out).expect("writing to String never fails");
    reference_body(module, &mut out, "");
    out
}
fn reference_body(module: &RubyModule, out: &mut std::string::String, indent: &str) {
    let inner = format!("{}  ", indent);
    writeln!(out, "{}module {}", indent, module.name).expect("writing to String never fails");
    for (name, expr) in &module.constants {
        writeln!(out, "{}{} = {}", inner, name, expr).expect("writing to String never fails");
    }
    if !module.constants.is_empty() {
        writeln!(out).expect("writing to String never fails");
    }
    for submod in &module.submodules {
        reference_body(submod, out, &inner);
        writeln!(out).expect("writing to String never fails");
    }
    for class in &module.classes {
        let mut fmt_buf = std::string::String::new();
        struct Wrapper<'a>(&'a RubyClass, &'a str);
        impl fmt::Display for Wrapper<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt_ruby_class(self.0, self.1, f)
            }
        }
        write!(fmt_buf, "{}", Wrapper(class, &inner)).expect("writing to String never fails");
        out.push_str(&fmt_buf);
        writeln!(out).expect("writing to String never fails");
    }
    if !module.functions.is_empty() {
        if module.module_function {
            writeln!(out, "{}module_function", inner).expect("writing to String never fails");
            writeln!(out).expect("writing to String never fails");
        }
        for method in &module.functions {
            let mut fmt_buf = std::string::String::new();
            struct Wrapper<'a>(&'a RubyMethod, &'a str);
            impl fmt::Display for Wrapper<'_> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt_ruby_method(self.0, self.1, f)
                }
            }
            write!(fmt_buf, "{}", Wrapper(method, &inner)).expect("writing to String never fails");
            out.push_str(&fmt_buf);
        }
    }
    writeln!(out, "{}end", indent).expect("writing to String never fails");
}

fn lit(n: i64) -> RubyExpr {
    RubyExpr::Lit(RubyLit::Int(n))
}

fn method(name: &str) -> RubyMethod {
    RubyMethod::new(name, vec!["a", "b"], vec![RubyStmt::Expr(lit(1))])
}

fn class(name: &str) -> RubyClass {
    let mut class = RubyClass::new(name);
    class.methods.push(method("run"));
    class.class_methods.push(method("build"));
    class.attr_readers.push("x".to_string());
    class
}

fn modules() -> Vec<RubyModule> {
    let empty = RubyModule::new("Empty");
    let mut constants_only = RubyModule::new("Consts");
    constants_only.constants.push(("ONE".to_string(), lit(1)));
    constants_only.constants.push(("TWO".to_string(), lit(2)));
    let mut functions = RubyModule::new("Funcs");
    functions.functions.push(method("f"));
    functions.functions.push(method("g"));
    let mut functions_plain = functions.clone();
    functions_plain.module_function = false;
    let mut classes = RubyModule::new("Classes");
    classes.classes.push(class("A"));
    classes.classes.push(class("B"));
    let mut everything = RubyModule::new("Outer");
    everything.constants.push(("K".to_string(), lit(7)));
    let mut inner = RubyModule::new("Inner");
    inner.constants.push(("J".to_string(), lit(8)));
    inner.functions.push(method("h"));
    let mut innermost = RubyModule::new("Innermost");
    innermost.classes.push(class("C"));
    inner.submodules.push(innermost);
    everything.submodules.push(inner);
    everything.submodules.push(RubyModule::new("Sibling"));
    everything.classes.push(class("D"));
    everything.functions.push(method("top"));
    vec![
        empty,
        constants_only,
        functions,
        functions_plain,
        classes,
        everything,
    ]
}

#[test]
fn emit_agrees_with_the_write_based_printer() {
    for module in modules() {
        assert_eq!(
            module.emit(),
            reference_emit(&module),
            "module {}",
            module.name
        );
    }
}

#[test]
fn an_empty_module_is_the_header_and_the_module_frame() {
    assert_eq!(
        RubyModule::new("M").emit(),
        "# frozen_string_literal: true\n\nmodule M\nend\n"
    );
}

#[test]
fn constants_are_indented_and_followed_by_a_blank_line() {
    let mut module = RubyModule::new("M");
    module.constants.push(("ONE".to_string(), lit(1)));
    assert_eq!(
        module.emit(),
        "# frozen_string_literal: true\n\nmodule M\n  ONE = 1\n\nend\n"
    );
}

#[test]
fn module_function_marker_follows_the_flag() {
    let mut module = RubyModule::new("M");
    module.functions.push(method("f"));
    assert!(module.emit().contains("  module_function\n\n"));
    module.module_function = false;
    assert!(!module.emit().contains("module_function"));
}

#[test]
fn submodules_and_classes_are_nested_inside_the_frame() {
    let mut module = RubyModule::new("Outer");
    module.submodules.push(RubyModule::new("Inner"));
    module.classes.push(class("K"));
    let out = module.emit();
    assert!(
        out.starts_with("# frozen_string_literal: true\n\nmodule Outer\n  module Inner\n  end\n\n")
    );
    assert!(out.contains("  class K\n"));
    assert!(out.ends_with("end\n"));
}
