use super::*;
use crate::ast::*;
use crate::expr::Expression;

#[test]
fn stmt_1() {
    let left = vec![Statement::While {
        cond: Expression::Value {
            id: 0,
            value: LiteralType::Boolean(true),
        },
        body: vec![Statement::Block { stmts: vec![] }],
    }];
    let right = get_ast("while true {}");

    assert_eq!(left, right, "testing `while true {{}}`");
}
