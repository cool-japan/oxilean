//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    FutharkAttr, FutharkBackend, FutharkExpr, FutharkFeatureFlags, FutharkFun, FutharkModule,
    FutharkStmt, FutharkType, FutharkTypeAlias,
};

/// Build a 1-D array type with a named size parameter.
pub fn array1(elem: FutharkType, size: impl Into<String>) -> FutharkType {
    FutharkType::Array(Box::new(elem), vec![Some(size.into())])
}
/// Build a 1-D array type with an anonymous size.
pub fn array1_dyn(elem: FutharkType) -> FutharkType {
    FutharkType::Array(Box::new(elem), vec![None])
}
/// Build a 2-D array type.
pub fn array2(elem: FutharkType, rows: impl Into<String>, cols: impl Into<String>) -> FutharkType {
    FutharkType::Array(Box::new(elem), vec![Some(rows.into()), Some(cols.into())])
}
/// Build a binary lambda: `\\ (x: t) (y: t) -> body`.
pub fn bin_lambda(
    x: impl Into<String>,
    y: impl Into<String>,
    ty: FutharkType,
    body: FutharkExpr,
) -> FutharkExpr {
    FutharkExpr::Lambda(vec![(x.into(), ty.clone()), (y.into(), ty)], Box::new(body))
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn var(s: &str) -> FutharkExpr {
        FutharkExpr::Var(s.to_string())
    }
    #[test]
    pub(super) fn test_type_display() {
        assert_eq!(FutharkType::I32.to_string(), "i32");
        assert_eq!(FutharkType::F64.to_string(), "f64");
        assert_eq!(FutharkType::Bool.to_string(), "bool");
        let arr = array1(FutharkType::F32, "n");
        assert_eq!(arr.to_string(), "[n]f32");
        let arr2 = array2(FutharkType::I64, "n", "m");
        assert_eq!(arr2.to_string(), "[n][m]i64");
        let tup = FutharkType::Tuple(vec![FutharkType::I32, FutharkType::F32]);
        assert_eq!(tup.to_string(), "(i32, f32)");
        let rec = FutharkType::Record(vec![
            ("x".to_string(), FutharkType::F64),
            ("y".to_string(), FutharkType::F64),
        ]);
        assert_eq!(rec.to_string(), "{x: f64, y: f64}");
    }
    #[test]
    pub(super) fn test_emit_iota_replicate() {
        let mut be = FutharkBackend::new();
        be.emit_expr(&FutharkExpr::Iota(Box::new(FutharkExpr::IntLit(
            10,
            FutharkType::I64,
        ))));
        assert_eq!(be.finish(), "iota 10i64");
        let mut be = FutharkBackend::new();
        be.emit_expr(&FutharkExpr::Replicate(
            Box::new(FutharkExpr::IntLit(5, FutharkType::I64)),
            Box::new(FutharkExpr::FloatLit(0.0, FutharkType::F32)),
        ));
        assert_eq!(be.finish(), "replicate 5i64 0f32");
    }
    #[test]
    pub(super) fn test_emit_map_reduce() {
        let mut be = FutharkBackend::new();
        let f = bin_lambda(
            "a",
            "b",
            FutharkType::I32,
            FutharkExpr::BinOp("+".to_string(), Box::new(var("a")), Box::new(var("b"))),
        );
        be.emit_expr(&FutharkExpr::Reduce(
            Box::new(f),
            Box::new(FutharkExpr::IntLit(0, FutharkType::I32)),
            Box::new(var("xs")),
        ));
        let out = be.finish();
        assert!(out.starts_with("reduce"), "got: {out}");
        assert!(out.contains("->"), "no arrow in lambda: {out}");
        assert!(out.contains("xs"), "no array var: {out}");
    }
    #[test]
    pub(super) fn test_emit_let_in() {
        let mut be = FutharkBackend::new();
        be.emit_expr(&FutharkExpr::LetIn(
            "tmp".to_string(),
            Some(FutharkType::I32),
            Box::new(FutharkExpr::IntLit(42, FutharkType::I32)),
            Box::new(var("tmp")),
        ));
        let out = be.finish();
        assert!(out.contains("let tmp"), "missing let: {out}");
        assert!(out.contains("42i32"), "missing value: {out}");
        assert!(out.contains("in tmp"), "missing in: {out}");
    }
    #[test]
    pub(super) fn test_emit_function() {
        let fun = FutharkFun::new(
            "dot_product",
            vec![
                ("xs".to_string(), array1_dyn(FutharkType::F32)),
                ("ys".to_string(), array1_dyn(FutharkType::F32)),
            ],
            FutharkType::F32,
            vec![FutharkStmt::ReturnExpr(FutharkExpr::Reduce(
                Box::new(bin_lambda(
                    "a",
                    "b",
                    FutharkType::F32,
                    FutharkExpr::BinOp("+".to_string(), Box::new(var("a")), Box::new(var("b"))),
                )),
                Box::new(FutharkExpr::FloatLit(0.0, FutharkType::F32)),
                Box::new(FutharkExpr::Map2(
                    Box::new(bin_lambda(
                        "x",
                        "y",
                        FutharkType::F32,
                        FutharkExpr::BinOp("*".to_string(), Box::new(var("x")), Box::new(var("y"))),
                    )),
                    Box::new(var("xs")),
                    Box::new(var("ys")),
                )),
            ))],
        );
        let mut be = FutharkBackend::new();
        be.emit_fun(&fun);
        let out = be.finish();
        assert!(out.contains("let dot_product"), "fn name: {out}");
        assert!(out.contains("f32"), "return type: {out}");
        assert!(out.contains("reduce"), "body reduce: {out}");
        assert!(out.contains("map2"), "body map2: {out}");
    }
    #[test]
    pub(super) fn test_emit_entry_point() {
        let fun = FutharkFun::entry(
            "main",
            vec![("input".to_string(), array1_dyn(FutharkType::F64))],
            FutharkType::F64,
            vec![FutharkStmt::ReturnExpr(FutharkExpr::Reduce(
                Box::new(bin_lambda(
                    "a",
                    "b",
                    FutharkType::F64,
                    FutharkExpr::BinOp("+".to_string(), Box::new(var("a")), Box::new(var("b"))),
                )),
                Box::new(FutharkExpr::FloatLit(0.0, FutharkType::F64)),
                Box::new(var("input")),
            ))],
        );
        let mut be = FutharkBackend::new();
        be.emit_fun(&fun);
        let out = be.finish();
        assert!(out.starts_with("entry main"), "entry keyword: {out}");
    }
    #[test]
    pub(super) fn test_emit_full_module() {
        let mut module = FutharkModule::new();
        module.set_doc("Matrix operations");
        module.add_open("import \"futlib/math\"");
        module.add_type(FutharkTypeAlias {
            name: "Matrix".to_string(),
            params: vec!["t".to_string()],
            ty: FutharkType::Array(
                Box::new(FutharkType::Array(
                    Box::new(FutharkType::Named("t".to_string())),
                    vec![Some("n".to_string())],
                )),
                vec![Some("m".to_string())],
            ),
            is_opaque: false,
        });
        module.add_fun(FutharkFun::entry(
            "matmul",
            vec![
                ("a".to_string(), array2(FutharkType::F32, "n", "k")),
                ("b".to_string(), array2(FutharkType::F32, "k", "m")),
            ],
            array2(FutharkType::F32, "n", "m"),
            vec![FutharkStmt::ReturnExpr(var("a"))],
        ));
        let src = FutharkBackend::generate(&module);
        assert!(src.contains("-- | Matrix operations"), "doc: {src}");
        assert!(src.contains("open import"), "open: {src}");
        assert!(src.contains("type Matrix"), "type alias: {src}");
        assert!(src.contains("entry matmul"), "entry: {src}");
    }
    #[test]
    pub(super) fn test_attrs_and_scan() {
        let fun = FutharkFun::new(
            "prefix_sum",
            vec![("xs".to_string(), array1_dyn(FutharkType::I32))],
            array1_dyn(FutharkType::I32),
            vec![FutharkStmt::ReturnExpr(FutharkExpr::Scan(
                Box::new(bin_lambda(
                    "a",
                    "b",
                    FutharkType::I32,
                    FutharkExpr::BinOp("+".to_string(), Box::new(var("a")), Box::new(var("b"))),
                )),
                Box::new(FutharkExpr::IntLit(0, FutharkType::I32)),
                Box::new(var("xs")),
            ))],
        )
        .with_attr(FutharkAttr::Inline);
        let mut be = FutharkBackend::new();
        be.emit_fun(&fun);
        let out = be.finish();
        assert!(out.contains("#[inline]"), "attr: {out}");
        assert!(out.contains("scan"), "scan: {out}");
        assert!(out.contains("prefix_sum"), "name: {out}");
        assert_eq!(FutharkAttr::Inline.to_string(), "#[inline]");
        assert_eq!(FutharkAttr::NoInline.to_string(), "#[noinline]");
        assert_eq!(FutharkAttr::NoMap.to_string(), "#[nomap]");
        assert_eq!(FutharkAttr::Sequential.to_string(), "#[sequential]");
        assert_eq!(
            FutharkAttr::Custom("fusable".to_string()).to_string(),
            "#[fusable]"
        );
    }
}
/// Futhark operator table
#[allow(dead_code)]
pub fn futhark_binop_str(op: &str) -> &'static str {
    match op {
        "add" | "+" => "+",
        "sub" | "-" => "-",
        "mul" | "*" => "*",
        "div" | "/" => "/",
        "rem" | "%" => "%",
        "and" | "&&" => "&&",
        "or" | "||" => "||",
        "eq" | "==" => "==",
        "ne" | "!=" => "!=",
        "lt" | "<" => "<",
        "le" | "<=" => "<=",
        "gt" | ">" => ">",
        "ge" | ">=" => ">=",
        "band" | "&" => "&",
        "bor" | "|" => "|",
        "bxor" | "^" => "^",
        "shl" | "<<" => "<<",
        "shr" | ">>" => ">>",
        _ => "+",
    }
}
#[allow(dead_code)]
pub fn futhark_unop_str(op: &str) -> &'static str {
    match op {
        "neg" | "-" => "-",
        "not" | "!" => "!",
        "bnot" | "~" => "~",
        _ => "-",
    }
}
/// Futhark version string
#[allow(dead_code)]
pub const FUTHARK_PASS_VERSION: &str = "0.25.0";
/// Futhark default backend
#[allow(dead_code)]
pub const FUTHARK_DEFAULT_BACKEND: &str = "opencl";
/// Futhark max inline depth
#[allow(dead_code)]
pub const FUTHARK_MAX_INLINE: usize = 20;
/// Futhark map1 helper name
#[allow(dead_code)]
pub const FUTHARK_MAP1: &str = "map";
/// Futhark reduce helper name
#[allow(dead_code)]
pub const FUTHARK_REDUCE: &str = "reduce";
/// Futhark default feature flags
#[allow(dead_code)]
pub fn futhark_default_features() -> FutharkFeatureFlags {
    FutharkFeatureFlags {
        enable_unsafe: false,
        enable_in_place_updates: true,
        enable_streaming: false,
        enable_loop_fusion: true,
        enable_double_buffering: false,
    }
}
/// Futhark entry version marker
#[allow(dead_code)]
pub const FUTHARK_ENTRY_VERSION: u32 = 1;
/// Futhark backend helper: emit a comment block
#[allow(dead_code)]
pub fn futhark_emit_comment_block(title: &str, body: &str) -> String {
    format!("-- {}\n-- {}\n", title, body)
}
