use super::*;

#[test]
fn stmt_3() {
    let left = vec![Statement::Struct {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        structs: vec![],
        is_pub: true,
        methods: vec![],
    }];
    let right = get_ast("struct pub Name {}");

    assert_eq!(left, right, "testing `struct pub Name {{}}`");
}

#[test]
fn stmt_2() {
    let left = vec![Statement::Struct {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        structs: vec![
            (
                Token {
                    token: Ident,
                    len: 1,
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
                    len: 1,
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

#[test]
fn stmt_1() {
    let left = vec![Statement::Struct {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        structs: vec![],
        is_pub: false,
        methods: vec![],
    }];
    let right = get_ast("struct Name {}");

    assert_eq!(left, right, "testing `struct Name {{}}`");
}