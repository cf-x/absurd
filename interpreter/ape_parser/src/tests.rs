use ape_lexer::Lexer;

use super::*;

fn get_ast(source: &str) -> Vec<Statement> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_var_stmt_7() {
    let left = vec![Statement::Var {
        names: vec![
            Token {
                token: Ident,
                len: 5,
                lexeme: "name1".to_string(),
                value: None,
                line: 1,
            },
            Token {
                token: Ident,
                len: 5,
                lexeme: "name2".to_string(),
                value: None,
                line: 1,
            },
        ],
        value_type: Token {
            token: CharIdent,
            len: 4,
            lexeme: "char".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Char('c'),
        }),
        is_mut: false,
        is_pub: true,
        pub_names: vec![
            Token {
                token: Ident,
                len: 1,
                lexeme: "c".to_string(),
                value: None,
                line: 1,
            },
            Token {
                token: Ident,
                len: 2,
                lexeme: "cc".to_string(),
                value: None,
                line: 1,
            },
        ],
        is_func: false,
    }];
    let right = get_ast("let pub(c, cc) name1, name2: char = 'c';");

    assert_eq!(
        left, right,
        "testing `let pub(c, cc) name1, name2: char = 'c';`"
    );
}

#[test]
fn test_var_stmt_6() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: CharIdent,
            len: 4,
            lexeme: "char".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Char('c'),
        }),
        is_mut: false,
        is_pub: true,
        pub_names: vec![Token {
            token: Ident,
            len: 1,
            lexeme: "c".to_string(),
            value: None,
            line: 1,
        }],
        is_func: false,
    }];
    let right = get_ast("let pub(c) name: char = 'c';");

    assert_eq!(left, right, "testing `let pub(c) name: char = 'c';`");
}

#[test]
fn test_var_stmt_5() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: CharIdent,
            len: 4,
            lexeme: "char".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Char('c'),
        }),
        is_mut: false,
        is_pub: true,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let pub name: char = 'c';");

    assert_eq!(left, right, "testing `let pub name: char = 'c';`");
}

#[test]
fn test_var_stmt_4() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: CharIdent,
            len: 4,
            lexeme: "char".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Char('c'),
        }),
        is_mut: true,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let mut name: char = 'c';");

    assert_eq!(left, right, "testing `let mut name: char = 'c';`");
}

#[test]
fn test_var_stmt_3() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: NullIdent,
            len: 4,
            lexeme: "null".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Null,
        }),
        is_mut: false,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let name;");

    assert_eq!(left, right, "testing `let name;`");
}

#[test]
fn test_var_stmt_2() {
    let left = vec![Statement::Var {
        names: vec![
            Token {
                token: Ident,
                len: 5,
                lexeme: "name1".to_string(),
                value: None,
                line: 1,
            },
            Token {
                token: Ident,
                len: 5,
                lexeme: "name2".to_string(),
                value: None,
                line: 1,
            },
        ],
        value_type: Token {
            token: NumberIdent,
            len: 6,
            lexeme: "number".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Number(5.0),
        }),
        is_mut: false,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let name1, name2: number = 5;");

    assert_eq!(left, right, "testing `let name1, name2: number = 5;`");
}

#[test]
fn test_var_stmt_1() {
    let left = vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            len: 4,
            lexeme: "name".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: NumberIdent,
            len: 6,
            lexeme: "number".to_string(),
            value: None,
            line: 1,
        },
        value: Some(Expression::Value {
            id: 0,
            value: LiteralType::Number(5.0),
        }),
        is_mut: false,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }];
    let right = get_ast("let name: number = 5;");

    assert_eq!(left, right, "testing `let name: number = 5;`");
}

#[test]
fn test_func_stmt_9() {
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
    let right = get_ast("func name(mut impl) -> void {}");

    assert_eq!(left, right, "testing `func name(mut impl) -> void {{}}`");
}

#[test]
fn test_func_stmt_8() {
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
    let right = get_ast("func name(impl) -> void {}");

    assert_eq!(left, right, "testing `func name(impl) -> void {{}}`");
}

#[test]
fn test_func_stmt_7() {
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
fn test_func_stmt_6() {
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
fn test_func_stmt_5() {
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
fn test_func_stmt_4() {
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
fn test_func_stmt_3() {
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
fn test_func_stmt_2() {
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
fn test_func_stmt_1() {
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

/*
    if statement test cases:

    if is_true {}
    if is_true {} else {}
    if is_true {} else if 1 {}
    if is_true {} else if 1 {} else {}
    if is_true {} else if 1 {} else if 2 {} else {}

    return statement test cases:

    return;
    return 1;

    while statement test cases:

    while 1 {}

    loop statement test cases:

    loop {}
    loop 2 {}

    break statement test cases:

    break;

    match statement test cases:

    match fruit {
        _ -> 5
    }
    match fruit {
        _ -> {}
    }
    match fruit {
        "banana" -> 1,
        "apple" -> {}
        "grape" -> 2,
        _ -> {}
    }

    mod statement test cases:

    mod "./file.ape";

    use statement test cases:

    use a from "src";
    use a as b from "src";
    use a, b, c from "src";
    use a as b, c as d, e from "src";

    struct statement test cases:

    struct Name {}
    struct Name {
        a: number,
        b: string,
        c: char
    }
    struct pub Name {}

    impl statement test cases:

    impl Name {}
    impl Name {
        fn new(src: string) {
            return src;
        }
    }

    enum statement test cases:

    enum name {
        Number,
        String,
        Char,
    }
    enum pub name {}

    expr test cases:
    2 + 2
    5 * 5
    2 + 2 / 4
    5!
    3!!
    4++
    Struct.call;
    Enum::call;
    call();
    ident;
    [1, 2, 3];
    "foo"
    'b'
*/
