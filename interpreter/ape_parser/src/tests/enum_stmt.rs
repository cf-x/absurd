use super::*;

#[test]
fn stmt_2() {
    let left = vec![Statement::Enum {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        enums: vec![],
        is_pub: true,
    }];
    let right = get_ast("enum pub Name {}");

    assert_eq!(left, right, "testing `enum pub Name {{}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Enum {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        enums: vec![
            Token {
                token: Ident,
                len: 6,
                lexeme: "Number".to_string(),
                value: None,
                line: 1,
            },
            Token {
                token: Ident,
                len: 6,
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
