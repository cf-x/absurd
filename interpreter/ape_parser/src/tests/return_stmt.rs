use super::*;

#[test]
fn stmt_2() {
    let left = vec![Statement::Return {
        expr: Expression::Value {
            id: 0,
            value: LiteralType::Null,
        },
    }];
    let right = get_ast("return;");

    assert_eq!(left, right, "testing `return;`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Return {
        expr: Expression::Value {
            id: 0,
            value: LiteralType::Number(15.0),
        },
    }];
    let right = get_ast("return 0xf;");

    assert_eq!(left, right, "testing `return 0xf;`");
}
