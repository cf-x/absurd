mod break_stmt;
mod call_expr;
mod enum_stmt;
mod func_stmt;
mod if_stmt;
mod impl_stmt;
mod loop_stmt;
mod match_stmt;
mod mod_stmt;
mod return_stmt;
mod struct_stmt;
mod type_expr;
mod use_stmt;
mod var_stmt;
mod while_stmt;
use super::*;
use ape_errors::Error;
use ape_lexer::Lexer;

fn get_ast(source: &str) -> Vec<Statement> {
    let err = Error::new(source);
    let mut lexer = Lexer::new(source.to_string(), err.clone());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens, err.clone());
    parser.parse()
}

fn get_expr(expr: Expression) -> Vec<Statement> {
    vec![Statement::Var {
        names: vec![Token {
            token: Ident,
            pos: (5, 6),
            lexeme: "x".to_string(),
            value: None,
            line: 1,
        }],
        value_type: Token {
            token: AnyIdent,
            pos: (8, 11),
            lexeme: "any".to_string(),
            value: None,
            line: 1,
        },
        value: Some(expr),
        is_mut: false,
        is_pub: false,
        pub_names: vec![],
        is_func: false,
    }]
}
