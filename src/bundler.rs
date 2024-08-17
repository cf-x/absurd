/*
@todo for bundling different parts with different configurations
*/

use crate::analyzer::Analyzer;
use crate::ast::{Statement, Token};
use crate::errors::Error;
use crate::lexer::Lexer;

pub fn lexer(src: &str) -> Vec<Token> {
    let err = Error::new(src);
    let mut lexer = Lexer::new(src.to_string(), err.clone());
    lexer.lex()
}

pub fn parser(src: &str) -> Vec<Statement> {
    let err = Error::new(src);
    let mut lexer = Lexer::new(src.to_string(), err.clone());
    let tokens = lexer.lex();
    let mut parser = crate::parser::Parser::new(tokens, err.clone());
    parser.parse()
}

pub fn analyzer(src: &str) -> Vec<Statement> {
    let err = Error::new(src);
    let mut lexer = Lexer::new(src.to_string(), err.clone());
    let tokens = lexer.lex();
    let mut parser = crate::parser::Parser::new(tokens, err.clone());
    let stmts = parser.parse();
    let mut analyzer = Analyzer::new(stmts);
    analyzer.analyze()
}
