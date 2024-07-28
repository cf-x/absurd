use super::*;

#[test]
fn stmt_9() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
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
fn stmt_8() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: false,
        is_pub: false,
        is_impl: true,
        is_mut: false,
    }];
    let right = get_ast("func name(self) -> void {}");

    assert_eq!(left, right, "testing `func name(self) -> void {{}}`");
}

#[test]
fn stmt_7() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: NumberIdent,
            len: 6,
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
                        len: 1,
                        lexeme: "a".to_string(),
                        value: None,
                        line: 1,
                    },
                }),
                operator: Token {
                    token: Plus,
                    len: 1,
                    lexeme: "+".to_string(),
                    value: None,
                    line: 1,
                },
                right: Box::new(Expression::Var {
                    id: 1,
                    name: Token {
                        token: Ident,
                        len: 1,
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
                    len: 1,
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    len: 6,
                    lexeme: "number".to_string(),
                    value: None,
                    line: 1,
                },
            ),
            (
                Token {
                    token: Ident,
                    len: 1,
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    len: 6,
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
    let right = get_ast("func name(a: number, b: number) -> number {return a + b;}");

    assert_eq!(
        left, right,
        "testing `func name(a: number, b: number) -> number {{return a + b;}}`"
    );
}

#[test]
fn stmt_6() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: NumberIdent,
            len: 6,
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
                        len: 1,
                        lexeme: "a".to_string(),
                        value: None,
                        line: 1,
                    },
                }),
                operator: Token {
                    token: Plus,
                    len: 1,
                    lexeme: "+".to_string(),
                    value: None,
                    line: 1,
                },
                right: Box::new(Expression::Var {
                    id: 1,
                    name: Token {
                        token: Ident,
                        len: 1,
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
                    len: 1,
                    lexeme: "a".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    len: 6,
                    lexeme: "number".to_string(),
                    value: None,
                    line: 1,
                },
            ),
            (
                Token {
                    token: Ident,
                    len: 1,
                    lexeme: "b".to_string(),
                    value: None,
                    line: 1,
                },
                Token {
                    token: NumberIdent,
                    len: 6,
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
fn stmt_5() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: NumberIdent,
            len: 6,
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
fn stmt_4() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: true,
        is_pub: true,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func pub async name() -> void {}");

    assert_eq!(left, right, "testing `func pub async name() -> void {{}}`");
}

#[test]
fn stmt_3() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: false,
        is_pub: true,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func pub name() -> void {}");

    assert_eq!(left, right, "testing `func pub name() -> void {{}}`");
}

#[test]
fn stmt_2() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
            lexeme: "void".to_string(),
            value: None,
            line: 1,
        },
        body: FuncBody::Statements(vec![]),
        params: vec![],
        is_async: true,
        is_pub: false,
        is_impl: false,
        is_mut: false,
    }];
    let right = get_ast("func async name() -> void {}");

    assert_eq!(left, right, "testing `func async name() -> void {{}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Func {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        },
        value_type: Token {
            token: VoidIdent,
            len: 4,
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
