use super::*;

#[test]
fn stmt_4() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            pos: (6, 10),
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            pos: (24, 28),
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: false,
        is_pub: false,
        is_impl: true,
        is_mut: true,
    }];
    let right = get_ast("func name(mut self) -> void {}");

    assert_eq!(left, right, "testing `func name(mut self) -> void {{}}`");
}

#[test]
fn stmt_3() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            pos: (6, 10),
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: NumberIdent,
            pos: (36, 42),
            lexeme: "number".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![Statement::Return {
            expr: Expression::Binary {
                id: 2,
                left: Box::new(Expression::Var {
                    id: 0,
                    name: Token {
                        token: Ident,
                        pos: (45, 46),
                        lexeme: "a".to_string(),
                        value: None,
                        line: 1,
                    },
                }),
                operator: Token {
                    token: Plus,
                    pos: (47, 48),
                    lexeme: "+".to_string(),
                    value: None,
                    line: 1,
                },
                right: Box::new(Expression::Var {
                    id: 1,
                    name: Token {
                        token: Ident,
                        pos: (49, 50),
                        lexeme: "b".to_string(),
                        value: None,
                        line: 1,
                    },
                }),
            },
        }]),
        params: vec![
            (
                Token {
                    token: Ident,
                    pos: (11, 12),
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    pos: (14, 20),
                    lexeme: "number".to_string(),
                    value: None,
                    line: 1,
                },
            ),
            (
                Token {
                    token: Ident,
                    pos: (22, 23),
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    pos: (25, 31),
                    lexeme: "number".to_string(),
                    value: None,
                    line: 1,
                },
            ),
        ],
        is_async: false,
        is_pub: false,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func name(a: number, b: number) -> number = a + b;");

    assert_eq!(
        left, right,
        "testing `func name(a: number, b: number) -> number = a + b;`"
    );
}

#[test]
fn stmt_2() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            pos: (6, 10),
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: NumberIdent,
            pos: (16, 22),
            lexeme: "number".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![Statement::Return {
            expr: Expression::Value {
                id: 0,
                value: LiteralType::Number(5.0),
            },
        }]),
        params: vec![],
        is_async: false,
        is_pub: false,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func name() -> number = 5;");

    assert_eq!(left, right, "testing `func name() -> number = 5;`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            pos: (6, 10),
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            pos: (16, 20),
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: false,
        is_pub: false,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func name() -> void {}");

    assert_eq!(left, right, "testing `func name() -> void {{}}`");
}
