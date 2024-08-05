use super::*;
use crate::ast::{TokenType::*, *};
use crate::expr::Expression;

#[test]
fn type_array_1() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            pos: (5, 6),
            lexeme: "x".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: ArrayIdent,
            pos: (15, 16),
            lexeme: "bool".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Array {
            id: 2,
            items: vec![LiteralType::Boolean(true), LiteralType::Boolean(false)],
        }),
        is_mut: false,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let x: <bool> = [true, false];");

    assert_eq!(left, right, "testing `let x: <bool> = [true, false];`");
}
