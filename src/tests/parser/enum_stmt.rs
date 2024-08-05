use super::*;
use crate::ast::{TokenType::*, *};

#[test]
fn stmt_1() {
    let left = vec![Statement::Enum {
        name: Token {
            token: Ident,
            pos: (6, 10),
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        enums: vec![
            Token {
                token: Ident,
                pos: (12, 18),
                lexeme: "Number".to_string(),
                value: None,
                line: 1,
            },
            Token {
                token: Ident,
                pos: (20, 26),
                lexeme: "String".to_string(),
                value: None,
                line: 1,
            },
        ],
        is_pub: false,
    }];
    let right = get_ast("enum Name {Number, String}");

    assert_eq!(left, right, "testing `enum Name {{Number, String}}`");
}
