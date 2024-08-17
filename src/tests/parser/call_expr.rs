use super::*;
use crate::ast::{TokenType::*, *};
use crate::expr::Expression;

#[test]
fn call_struct_1() {
    let left = get_expr(Expression::Call {
        id: 2,
        name: Box::new(Expression::Var {
            id: 3,
            name: Token {
                token: Ident,
                pos: (14, 18),
                lexeme: "Call".to_string(),
                value: None,
                line: 1,
            },
        }),
        args: vec![Expression::Var {
            id: 1,
            name: Token {
                token: Ident,
                pos: (19, 24),
                lexeme: "value".to_string(),
                value: None,
                line: 1,
            },
        }],
        call_type: CallType::Struct,
    });
    let right = get_ast("let x: any = Call.value;");

    assert_eq!(left, right, "testing `let x: any = Call.value;`");
}

#[test]
fn call_enum_1() {
    let left = get_expr(Expression::Call {
        id: 2,
        name: Box::new(Expression::Var {
            id: 3,
            name: Token {
                token: Ident,
                pos: (14, 18),
                lexeme: "call".to_string(),
                value: None,
                line: 1,
            },
        }),
        args: vec![Expression::Var {
            id: 1,
            name: Token {
                token: Ident,
                pos: (20, 24),
                lexeme: "Enum".to_string(),
                value: None,
                line: 1,
            },
        }],
        call_type: CallType::Enum,
    });
    let right = get_ast("let x: any = call::Enum;");

    assert_eq!(left, right, "testing `let x: any = call::Enum;`");
}

#[test]
fn call_func_3() {
    let left = get_expr(Expression::Call {
        id: 3,
        name: Box::new(Expression::Var {
            id: 4,
            name: Token {
                token: Ident,
                pos: (14, 18),
                lexeme: "call".to_string(),
                value: None,
                line: 1,
            },
        }),
        args: vec![
            Expression::Value {
                id: 1,
                value: LiteralType::Boolean(true),
            },
            Expression::Value {
                id: 2,
                value: LiteralType::Boolean(false),
            },
        ],
        call_type: CallType::Func,
    });
    let right = get_ast("let x: any = call(true, false);");

    assert_eq!(left, right, "testing `let x: any = call(true, false);`");
}

#[test]
fn call_func_2() {
    let left = get_expr(Expression::Call {
        id: 2,
        name: Box::new(Expression::Var {
            id: 3,
            name: Token {
                token: Ident,
                pos: (14, 18),
                lexeme: "call".to_string(),
                value: None,
                line: 1,
            },
        }),
        args: vec![Expression::Value {
            id: 1,
            value: LiteralType::Boolean(true),
        }],
        call_type: CallType::Func,
    });
    let right = get_ast("let x: any = call(true);");

    assert_eq!(left, right, "testing `let x: any = call(true);`");
}

#[test]
fn call_func_1() {
    let left = get_expr(Expression::Call {
        id: 1,
        name: Box::new(Expression::Var {
            id: 2,
            name: Token {
                token: Ident,
                pos: (14, 18),
                lexeme: "call".to_string(),
                value: None,
                line: 1,
            },
        }),
        args: vec![],
        call_type: CallType::Func,
    });
    let right = get_ast("let x: any = call();");

    assert_eq!(left, right, "testing `let x: any = call();`");
}
