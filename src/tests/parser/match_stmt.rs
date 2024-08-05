use super::*;
use crate::ast::{TokenType::*, *};
use crate::expr::Expression;

#[test]
fn stmt_3() {
    let left = vec![Statement::Match {
        cond: Expression::Var {
            id: 0,
            name: Token {
                token: Ident,
                pos: (7, 12),
                lexeme: "fruit".to_string(),
                value: None,
                line: 1,
            },
        },
        cases: vec![
            (
                Expression::Value {
                    id: 1,
                    value: LiteralType::String("banana".to_string()),
                },
                FuncBody::Expression(Box::new(Expression::Value {
                    id: 2,
                    value: LiteralType::Number(5.0),
                })),
            ),
            (
                Expression::Value {
                    id: 3,
                    value: LiteralType::String("apple".to_string()),
                },
                FuncBody::Statements(vec![]),
            ),
        ],
        def_case: FuncBody::Statements(vec![]),
    }];
    let right = get_ast("match fruit {\"banana\" => 5, \"apple\" => {} _ => {}}");

    assert_eq!(
        left, right,
        "testing `match fruit {{\"banana\" => 5, \"apple\" => {{}} _ => {{}}}}`"
    );
}

#[test]
fn stmt_2() {
    let left = vec![Statement::Match {
        cond: Expression::Var {
            id: 0,
            name: Token {
                token: Ident,
                pos: (7, 12),
                lexeme: "fruit".to_string(),
                value: None,
                line: 1,
            },
        },
        cases: vec![],
        def_case: FuncBody::Expression(Box::new(Expression::Value {
            id: 1,
            value: LiteralType::Number(5.0),
        })),
    }];
    let right = get_ast("match fruit {_ => 5}");

    assert_eq!(left, right, "testing `match fruit {{_ => 5}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Match {
        cond: Expression::Var {
            id: 0,
            name: Token {
                token: Ident,
                pos: (7, 12),
                lexeme: "fruit".to_string(),
                value: None,
                line: 1,
            },
        },
        cases: vec![],
        def_case: FuncBody::Statements(vec![]),
    }];
    let right = get_ast("match fruit {_ => {}}");

    assert_eq!(left, right, "testing `match fruit {{_ => {{}}}}`");
}
