mod break_stmt;
mod enum_stmt;
mod func_stmt;
mod if_stmt;
mod impl_stmt;
mod loop_stmt;
mod match_stmt;
mod mod_stmt;
mod return_stmt;
mod struct_stmt;
mod use_stmt;
mod var_stmt;
mod while_stmt;
use super::*;
use ape_lexer::Lexer;

pub fn get_ast(source: &str) -> Vec<Statement> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    parser.parse()
}
