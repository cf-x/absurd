use super::*;
use crate::ast::{TokenType::*, *};

#[test]
fn stmt_4() {
    let left = vec![Statement::Use {
        src: "\"src\"".to_string(),
        names: vec![
            (
                Token {
                    token: Ident,
                    pos: (5, 6),
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                Some(Token {
                    token: Ident,
                    pos: (10, 11),
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                }),
            ),
            (
                Token {
                    token: Ident,
                    pos: (13, 14),
                    lexeme: "c".to_string(),
                    value: None,
                    line: 1,
                },
                None,
            ),
        ],
    }];
    let right = get_ast("use a as b, c from \"src\";");

    assert_eq!(left, right, "testing `use a as b, c from \"src\";`");
}

#[test]
fn stmt_3() {
    let left = vec![Statement::Use {
        src: "\"src\"".to_string(),
        names: vec![(
            Token {
                token: Ident,
                pos: (5, 6),
                lexeme: "a".to_string(),
                value: None,
                line: 1,
            },
            Some(Token {
                token: Ident,
                pos: (10, 11),
                lexeme: "b".to_string(),
                value: None,
                line: 1,
            }),
        )],
    }];
    let right = get_ast("use a as b from \"src\";");

    assert_eq!(left, right, "testing `use a as b from \"src\";`");
}

#[test]
fn stmt_2() {
    let left = vec![Statement::Use {
        src: "\"src\"".to_string(),
        names: vec![
            (
                Token {
                    token: Ident,
                    pos: (5, 6),
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                None,
            ),
            (
                Token {
                    token: Ident,
                    pos: (8, 9),
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                },
                None,
            ),
            (
                Token {
                    token: Ident,
                    pos: (11, 12),
                    lexeme: "c".to_string(),
                    value: None,
                    line: 1,
                },
                None,
            ),
        ],
    }];
    let right = get_ast("use a, b, c from \"src\";");

    assert_eq!(left, right, "testing `use a, b, c from \"src\";`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Use {
        src: "\"src\"".to_string(),
        names: vec![(
            Token {
                token: Ident,
                pos: (5, 6),
                lexeme: "a".to_string(),
                value: None,
                line: 1,
            },
            None,
        )],
    }];
    let right = get_ast("use a from \"src\";");

    assert_eq!(left, right, "testing `use a from \"src\";`");
}
