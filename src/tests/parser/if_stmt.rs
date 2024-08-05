use super::*;
use crate::ast::*;
use crate::expr::Expression;
#[test]
fn stmt_4() {
    let left = vec![Statement::If {
        cond: Expression::Value {
            id: 0,
            value: LiteralType::Boolean(true),
        },
        body: vec![Statement::Block { stmts: vec![] }],
        else_if_branches: vec![
            (
                Expression::Value {
                    id: 1,
                    value: LiteralType::Boolean(false),
                },
                vec![Statement::Block { stmts: vec![] }],
            ),
            (
                Expression::Value {
                    id: 2,
                    value: LiteralType::Number(1.0),
                },
                vec![Statement::Block { stmts: vec![] }],
            ),
        ],
        else_branch: None,
    }];
    let right = get_ast("if true {} elif false {} elif 0b1 {}");

    assert_eq!(
        left, right,
        "testing `if true {{}} elif false {{}} elif 0b1 {{}}`"
    );
}

#[test]
fn stmt_3() {
    let left = vec![Statement::If {
        cond: Expression::Value {
            id: 0,
            value: LiteralType::Boolean(true),
        },
        body: vec![Statement::Block { stmts: vec![] }],
        else_if_branches: vec![(
            Expression::Value {
                id: 1,
                value: LiteralType::Boolean(false),
            },
            vec![Statement::Block { stmts: vec![] }],
        )],
        else_branch: None,
    }];
    let right = get_ast("if true {} elif false {}");

    assert_eq!(left, right, "testing `if true {{}} elif false {{}}`");
}

#[test]
fn stmt_2() {
    let left = vec![Statement::If {
        cond: Expression::Value {
            id: 0,
            value: LiteralType::Boolean(true),
        },
        body: vec![Statement::Block { stmts: vec![] }],
        else_if_branches: vec![],
        else_branch: Some(vec![Statement::Block { stmts: vec![] }]),
    }];
    let right = get_ast("if true {} else {}");

    assert_eq!(left, right, "testing `if true {{}} else {{}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::If {
        cond: Expression::Value {
            id: 0,
            value: LiteralType::Boolean(true),
        },
        body: vec![Statement::Block { stmts: vec![] }],
        else_if_branches: vec![],
        else_branch: None,
    }];
    let right = get_ast("if true {}");

    assert_eq!(left, right, "testing `if true {{}}`");
}
