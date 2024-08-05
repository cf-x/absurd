use super::*;

use crate::ast::{TokenType::*, *};
#[test]
fn stmt_1() {
    let left = vec![Statement::Struct {
        name: Token {
            token: Ident,
            pos: (8, 12),
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        structs: vec![
            (
                Token {
                    token: Ident,
                    pos: (14, 15),
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                NumberIdent,
                false,
            ),
            (
                Token {
                    token: Ident,
                    pos: (29, 30),
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                },
                StringIdent,
                true,
            ),
        ],
        is_pub: false,
        methods: vec![],
    }];
    let right = get_ast("struct Name {a: number, pub b: string}");

    assert_eq!(
        left, right,
        "testing `struct Name {{a: number, pub b: string}}`"
    );
}
