//! Tests for `fill_default_methods`.

use super::super::types::{Method, TypeClass};
use super::fill_default_methods;
use oxilean_kernel::{Expr, Name};

fn ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}

#[test]
fn fills_each_unprovided_method_that_has_a_default() {
    let mut class = TypeClass::new(Name::str("Show"), vec![Name::str("α")]);
    class.add_method(Method::new(Name::str("show"), ty()));
    class.add_method(Method::with_default(
        Name::str("showList"),
        ty(),
        Expr::Const(Name::str("defaultShowList"), vec![]),
    ));
    class.add_method(Method::with_default(
        Name::str("showPrec"),
        ty(),
        Expr::Const(Name::str("defaultShowPrec"), vec![]),
    ));
    let fills = fill_default_methods(&class, &[Name::str("showPrec")]);
    assert_eq!(fills.len(), 1);
    assert_eq!(fills[0].method_name, Name::str("showList"));
    assert_eq!(
        fills[0].impl_expr,
        Expr::Const(Name::str("defaultShowList"), vec![])
    );
    let all = fill_default_methods(&class, &[]);
    let names: Vec<Name> = all.iter().map(|f| f.method_name.clone()).collect();
    assert_eq!(names, vec![Name::str("showList"), Name::str("showPrec")]);
    assert!(
        fill_default_methods(&class, &[Name::str("showList"), Name::str("showPrec")]).is_empty()
    );
}
