use std::cell::RefCell;
use std::rc::Rc;

use crate::analyzer::Analyzer;
use crate::ast::{Statement, Token};
use crate::env::Env;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::resolver::Resolver;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn analyzer(src: &str) -> Vec<Statement> {
    let err = Error::new(src);
    let mut lexer = Lexer::new(src.to_string(), err.clone());
    let tokens = lexer.lex();
    let mut parser = crate::parser::Parser::new(tokens, err.clone());
    let stmts = parser.parse();
    let mut analyzer = Analyzer::new(stmts);
    analyzer.analyze()
}

pub fn interpreter_raw(src: &str) {
    let mut int = Interpreter::new(src);
    let stmts = parser(src);
    let mut resolver = Resolver::new(src);
    let locals = resolver.resolve(&stmts.iter().collect(), &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect());
}

pub fn interpreter_mod(
    src: &str,
    mod_src: Option<String>,
    env: Rc<RefCell<Env>>,
) -> Rc<RefCell<Env>> {
    let mut int = Interpreter::new_with_env(env, true, src, mod_src);
    let stmts = parser(src);
    let mut resolver = Resolver::new(src);
    let locals = resolver.resolve(&stmts.iter().collect(), &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect())
}
