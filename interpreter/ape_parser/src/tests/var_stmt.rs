use super::*;

#[test]
fn stmt_7() {
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
fn stmt_6() {
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
fn stmt_5() {
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
fn stmt_4() {
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
fn stmt_3() {
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
fn stmt_2() {
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
fn stmt_1() {
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
