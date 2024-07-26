use ape_lexer::Lexer;

use super::*;

/*
test cases:

let name: number = 5;
*/

fn get_ast(source: &str) -> Vec<Statement> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    parser.parse()
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
            id: 1,
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
